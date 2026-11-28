//! Defeasible reasoning: reasons that support claims, two kinds of defeater, ranked conflict, and a
//! skeptical verdict that says why.
//!
//! Generic over the claim id `C`: this crate knows nothing of what claims say. Deterministic
//! (ordered maps, no clock, no RNG, no floating point).
//!
//! # Model
//! * A **reason** supports one claim from zero or more antecedent claims (zero = a premise) and has a
//!   caller-supplied `rank` (higher is stronger; what the numbers mean is the caller's data).
//! * A **rebutting defeater**: two claims declared contradictory with [`DefeasibleSystem::contradict`].
//!   A reason for one rebuts every reason for the other. A rebutter *defeats* a reason only if it is
//!   live and **strictly** stronger; a live rebutter of equal or greater rank *blocks* it. Equal ranks
//!   therefore leave both sides undecided (collective defeat).
//! * An **undercutting defeater**: [`DefeasibleSystem::undercut`] names a claim that, when accepted,
//!   cuts a *reason* (the link from its antecedents to its claim) without saying the claim is false.
//!
//! # Verdict
//! Each claim ends [`Standing::In`], [`Standing::Out`] or [`Standing::Undecided`], computed as the
//! least fixpoint of the labelling rules below (the grounded labelling): a label is set only when the
//! facts already decided force it, so the result does not depend on the order reasons were added, and
//! it is skeptical. Cycles of defeat, and equal-rank conflicts, stay `Undecided`.
//! * a reason is **dead** if an antecedent is `Out`, an undercutter is `In`, or a *live* rebutter is
//!   strictly stronger; it is **live** if every antecedent is `In`, every undercutter is `Out`, and
//!   every rebutter of equal or greater rank is dead; otherwise it is open;
//! * a claim is `In` if some reason is live, `Out` if every reason is dead (or it has none).
//! * a claim that no well-founded chain of reasons reaches (for instance one supported only by a cycle
//!   of claims that cite each other) is `Out` from the start, as in `epistemologicant-justify`.
//!
//! Rule ids in reasons: `defeat.in`, `defeat.out`, `defeat.out.no-reasons`, `defeat.undecided`.
//! A claim declared contradictory with itself can never be `In`.
//!
//! # Credit and what was read
//! The two kinds of defeater are Pollock's (rebutting: a reason for the denial of the conclusion;
//! undercutting: a reason to doubt that the premises support the conclusion here), read in the Stanford
//! Encyclopedia of Philosophy entry "Defeasible Reasoning" (§1.1, §4.1, revision of 13 Jun 2025), which
//! cites Pollock, J. L. (1987), "Defeasible Reasoning", *Cognitive Science* 11: 481-518; that paper
//! itself was **not** read. The skeptical, least-fixpoint verdict is in the spirit of Dung's grounded
//! extension (Dung 1995, *Artificial Intelligence* 77(2): 321-357, not read; definitions read at
//! second hand in Wikipedia "Argumentation framework" and the SEP entry §4.4). **Rank decides a
//! rebuttal is this crate's own design choice**: the SEP entry gives no such rule; the nearest
//! published idea read is the value-based argumentation framework (an attack succeeds unless the
//! target's value is preferred), in the same Wikipedia article.
//!
//! # Example
//!
//! ```
//! use epistemologicant_defeat::*;
//!
//! let mut s = DefeasibleSystem::new();
//! s.assert("looks-red", &[], 1, "perception");
//! let red = s.assert("red", &["looks-red"], 1, "it looks red");
//! assert_eq!(s.status(&"red").value, Standing::In);
//!
//! // A report of red lighting does not say "it is not red"; it cuts the reason (an undercutter).
//! s.assert("red-light", &[], 1, "lighting report");
//! s.undercut(red, "red-light")?;
//! assert_eq!(s.status(&"red").value, Standing::Out);
//!
//! // A reason for the opposite claim is a rebuttal: the stronger one wins, equal strengths leave both undecided.
//! s.assert("not-red", &[], 5, "spectrometer");
//! s.contradict("red", "not-red");
//! assert_eq!(s.status(&"not-red").value, Standing::In);
//! # Ok::<(), EpError>(())
//! ```
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub use epistemologicant_core::{EpError, Reason, Verdict};

/// Identifies one reason, issued by [`DefeasibleSystem::assert`]. Ids are sequential from 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReasonId(pub u64);

/// The skeptical standing of a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Some reason for it is live.
    In,
    /// Every reason for it is dead, or it has none.
    Out,
    /// Neither is forced: a cycle of defeat, or a conflict between reasons of equal rank.
    Undecided,
}

/// One claim whose standing changed because of a [`DefeasibleSystem::retract`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change<C> {
    /// The claim.
    pub claim: C,
    /// Its standing before.
    pub before: Standing,
    /// Its standing after.
    pub after: Standing,
}

struct Entry<C> {
    claim: C,
    antecedents: Vec<C>,
    rank: u32,
    label: String,
    active: bool,
}

/// Reasons, contradictions and undercuts.
pub struct DefeasibleSystem<C> {
    reasons: Vec<Entry<C>>,
    contradictions: BTreeMap<C, BTreeSet<C>>,
    undercuts: Vec<(ReasonId, C)>,
}

impl<C> Default for DefeasibleSystem<C> {
    fn default() -> Self {
        Self {
            reasons: Vec::new(),
            contradictions: BTreeMap::new(),
            undercuts: Vec::new(),
        }
    }
}

/// Labels at the fixpoint. `None` = open.
struct Labels<C> {
    claims: BTreeMap<C, Option<bool>>, // Some(true) In, Some(false) Out
    live: Vec<Option<bool>>,           // Some(true) live, Some(false) dead
}

impl<C: Ord + Clone + Debug> DefeasibleSystem<C> {
    /// An empty system.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a reason for `claim` from `antecedents` (empty = a premise), with a `rank` and a `label`
    /// (free text naming where it comes from, kept for reasons). Antecedents need not exist yet.
    pub fn assert(&mut self, claim: C, antecedents: &[C], rank: u32, label: &str) -> ReasonId {
        self.reasons.push(Entry {
            claim,
            antecedents: antecedents.to_vec(),
            rank,
            label: label.to_string(),
            active: true,
        });
        ReasonId(self.reasons.len() as u64 - 1)
    }

    /// Declare two claims contradictory (symmetric): reasons for one rebut reasons for the other.
    pub fn contradict(&mut self, a: C, b: C) {
        self.contradictions
            .entry(a.clone())
            .or_default()
            .insert(b.clone());
        self.contradictions.entry(b).or_default().insert(a);
    }

    /// Make `by` an undercutter of `reason`: while `by` is `In`, the reason is dead; while `by` is
    /// not yet `Out`, the reason cannot be live.
    pub fn undercut(&mut self, reason: ReasonId, by: C) -> Result<(), EpError> {
        if reason.0 as usize >= self.reasons.len() {
            return Err(EpError::UnknownReason(reason.0));
        }
        self.undercuts.push((reason, by));
        Ok(())
    }

    /// Withdraw a reason and report every claim whose standing changed, in claim order.
    pub fn retract(&mut self, reason: ReasonId) -> Result<Vec<Change<C>>, EpError> {
        let idx = reason.0 as usize;
        let Some(e) = self.reasons.get(idx) else {
            return Err(EpError::UnknownReason(reason.0));
        };
        if !e.active {
            return Err(EpError::AlreadyRetracted(reason.0));
        }
        let before = self.standings();
        self.reasons[idx].active = false;
        let after = self.standings();
        Ok(before
            .into_iter()
            .filter_map(|(claim, b)| {
                let a = after[&claim];
                (a != b).then_some(Change {
                    claim,
                    before: b,
                    after: a,
                })
            })
            .collect())
    }

    fn all_claims(&self) -> BTreeSet<C> {
        let mut s = BTreeSet::new();
        for e in &self.reasons {
            s.insert(e.claim.clone());
            s.extend(e.antecedents.iter().cloned());
        }
        for (a, bs) in &self.contradictions {
            s.insert(a.clone());
            s.extend(bs.iter().cloned());
        }
        s.extend(self.undercuts.iter().map(|(_, c)| c.clone()));
        s
    }

    fn rebutters(&self, i: usize) -> Vec<usize> {
        let claim = &self.reasons[i].claim;
        let Some(enemies) = self.contradictions.get(claim) else {
            return Vec::new();
        };
        self.reasons
            .iter()
            .enumerate()
            .filter(|(_, e)| enemies.contains(&e.claim))
            .map(|(j, _)| j)
            .collect()
    }

    fn undercutters(&self, i: usize) -> Vec<&C> {
        self.undercuts
            .iter()
            .filter(|(r, _)| r.0 as usize == i)
            .map(|(_, c)| c)
            .collect()
    }

    /// Least fixpoint of the labelling rules.
    fn label(&self) -> Labels<C> {
        let mut l = Labels {
            claims: self.all_claims().into_iter().map(|c| (c, None)).collect(),
            live: self
                .reasons
                .iter()
                .map(|e| if e.active { None } else { Some(false) })
                .collect(),
        };
        let rebut: Vec<Vec<usize>> = (0..self.reasons.len()).map(|i| self.rebutters(i)).collect();
        // Unfounded support can never make a claim `In`: a claim that no well-founded chain of active
        // reasons reaches (ignoring all defeat) is `Out` (a reason leaning on one is then dead through its antecedent). This is
        // what keeps a cycle of claims that only support each other from staying open.
        let mut reachable: BTreeSet<&C> = BTreeSet::new();
        loop {
            let before = reachable.len();
            for e in self.reasons.iter().filter(|e| e.active) {
                if e.antecedents.iter().all(|a| reachable.contains(a)) {
                    reachable.insert(&e.claim);
                }
            }
            if reachable.len() == before {
                break;
            }
        }
        for (c, v) in l.claims.iter_mut() {
            if !reachable.contains(c) {
                *v = Some(false);
            }
        }
        loop {
            let mut changed = false;
            for i in 0..self.reasons.len() {
                if l.live[i].is_some() {
                    continue;
                }
                let e = &self.reasons[i];
                let ante_out = e.antecedents.iter().any(|a| l.claims[a] == Some(false));
                let ante_in = e.antecedents.iter().all(|a| l.claims[a] == Some(true));
                let uc = self.undercutters(i);
                let cut = uc.iter().any(|c| l.claims[*c] == Some(true));
                let uncut = uc.iter().all(|c| l.claims[*c] == Some(false));
                let beaten = rebut[i]
                    .iter()
                    .any(|&j| l.live[j] == Some(true) && self.reasons[j].rank > e.rank);
                let unblocked = rebut[i]
                    .iter()
                    .all(|&j| self.reasons[j].rank < e.rank || l.live[j] == Some(false));
                if ante_out || cut || beaten {
                    l.live[i] = Some(false);
                    changed = true;
                } else if ante_in && uncut && unblocked {
                    l.live[i] = Some(true);
                    changed = true;
                }
            }
            let claims: Vec<C> = l.claims.keys().cloned().collect();
            for c in claims {
                if l.claims[&c].is_some() {
                    continue;
                }
                let own: Vec<usize> = (0..self.reasons.len())
                    .filter(|&i| self.reasons[i].claim == c)
                    .collect();
                if own.iter().any(|&i| l.live[i] == Some(true)) {
                    l.claims.insert(c, Some(true));
                    changed = true;
                } else if own.iter().all(|&i| l.live[i] == Some(false)) {
                    l.claims.insert(c, Some(false));
                    changed = true;
                }
            }
            if !changed {
                return l;
            }
        }
    }

    fn standing_of(v: Option<bool>) -> Standing {
        match v {
            Some(true) => Standing::In,
            Some(false) => Standing::Out,
            None => Standing::Undecided,
        }
    }

    /// The standing of every claim the system has heard of, in claim order.
    pub fn standings(&self) -> BTreeMap<C, Standing> {
        self.label()
            .claims
            .into_iter()
            .map(|(c, v)| (c, Self::standing_of(v)))
            .collect()
    }

    /// Why reason `i` is dead (first cause found), for reasons.
    fn why_dead(&self, l: &Labels<C>, i: usize) -> String {
        let e = &self.reasons[i];
        let name = format!("reason {} ({})", i, e.label);
        if !e.active {
            return format!("{name} was retracted");
        }
        if let Some(a) = e.antecedents.iter().find(|a| l.claims[*a] == Some(false)) {
            return format!("{name}: antecedent {a:?} is out");
        }
        if let Some(c) = self
            .undercutters(i)
            .into_iter()
            .find(|c| l.claims[*c] == Some(true))
        {
            return format!("{name} is undercut by {c:?}, which is in");
        }
        if let Some(j) = self
            .rebutters(i)
            .into_iter()
            .find(|&j| l.live[j] == Some(true) && self.reasons[j].rank > e.rank)
        {
            let r = &self.reasons[j];
            return format!(
                "{name} (rank {}) is rebutted by live reason {j} ({}) for {:?} at rank {}",
                e.rank, r.label, r.claim, r.rank
            );
        }
        format!("{name} is dead")
    }

    /// What keeps open reason `i` open, for reasons.
    fn why_open(&self, l: &Labels<C>, i: usize) -> String {
        let e = &self.reasons[i];
        let name = format!("reason {} ({})", i, e.label);
        if let Some(a) = e.antecedents.iter().find(|a| l.claims[*a].is_none()) {
            return format!("{name}: antecedent {a:?} is undecided");
        }
        if let Some(c) = self
            .undercutters(i)
            .into_iter()
            .find(|c| l.claims[*c].is_none())
        {
            return format!("{name}: undercutter {c:?} is undecided");
        }
        if let Some(j) = self
            .rebutters(i)
            .into_iter()
            .find(|&j| self.reasons[j].rank >= e.rank && l.live[j].is_none())
        {
            let r = &self.reasons[j];
            return format!(
                "{name} (rank {}) is contested by open reason {j} ({}) for {:?} at rank {}",
                e.rank, r.label, r.claim, r.rank
            );
        }
        format!("{name} is open")
    }

    /// The claim's standing and why. A claim never mentioned is `Out` (`defeat.out.no-reasons`).
    pub fn status(&self, claim: &C) -> Verdict<Standing> {
        let l = self.label();
        let own: Vec<usize> = (0..self.reasons.len())
            .filter(|&i| &self.reasons[i].claim == claim)
            .collect();
        match l.claims.get(claim).copied().flatten() {
            Some(true) => {
                let i = own
                    .iter()
                    .copied()
                    .find(|&i| l.live[i] == Some(true))
                    .unwrap_or(0);
                let e = &self.reasons[i];
                Verdict::new(Standing::In).because(Reason::new(
                    "defeat.in",
                    format!(
                        "{claim:?} is in by live reason {i} ({}), rank {}, from {:?}",
                        e.label, e.rank, e.antecedents
                    ),
                ))
            }
            Some(false) if own.is_empty() => Verdict::new(Standing::Out).because(Reason::new(
                "defeat.out.no-reasons",
                format!("{claim:?} has no reason"),
            )),
            Some(false) => {
                let mut v = Verdict::new(Standing::Out);
                for &i in &own {
                    v = v.because(Reason::new("defeat.out", self.why_dead(&l, i)));
                }
                v
            }
            None if !l.claims.contains_key(claim) => {
                Verdict::new(Standing::Out).because(Reason::new(
                    "defeat.out.no-reasons",
                    format!("{claim:?} was never mentioned"),
                ))
            }
            None => {
                let mut v = Verdict::new(Standing::Undecided);
                let mut any = false;
                for &i in own.iter().filter(|&&i| l.live[i].is_none()) {
                    any = true;
                    v = v.because(Reason::new("defeat.undecided", self.why_open(&l, i)));
                }
                if !any {
                    v = v.because(Reason::new(
                        "defeat.undecided",
                        format!("{claim:?} is undecided"),
                    ));
                }
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type S = DefeasibleSystem<&'static str>;

    fn st(s: &S, c: &'static str) -> Standing {
        s.status(&c).value
    }

    #[test]
    fn a_premise_is_in_and_an_unknown_claim_is_out() {
        let mut s = S::new();
        s.assert("a", &[], 1, "obs");
        assert_eq!(st(&s, "a"), Standing::In);
        let v = s.status(&"a");
        assert_eq!(v.reasons[0].rule, "defeat.in");
        let u = s.status(&"zzz");
        assert_eq!(u.value, Standing::Out);
        assert_eq!(u.reasons[0].rule, "defeat.out.no-reasons");
    }

    #[test]
    fn a_cycle_of_claims_that_only_support_each_other_is_out() {
        let mut s = S::new();
        s.assert("p", &["q"], 1, "p from q");
        s.assert("q", &["p"], 1, "q from p");
        assert_eq!(st(&s, "p"), Standing::Out);
        assert_eq!(st(&s, "q"), Standing::Out);
        // with an outside premise the loop is held up by it, not by itself
        let g = s.assert("p", &[], 1, "seed");
        assert_eq!((st(&s, "p"), st(&s, "q")), (Standing::In, Standing::In));
        s.retract(g).unwrap();
        assert_eq!((st(&s, "p"), st(&s, "q")), (Standing::Out, Standing::Out));
    }

    #[test]
    fn a_reason_needs_every_antecedent_in() {
        let mut s = S::new();
        s.assert("a", &[], 1, "x");
        s.assert("c", &["a", "b"], 1, "inf");
        assert_eq!(st(&s, "c"), Standing::Out, "b has no reason: out");
        s.assert("b", &[], 1, "y");
        assert_eq!(st(&s, "c"), Standing::In);
    }

    #[test]
    fn undercutting_cuts_the_reason_not_the_claim_and_another_reason_survives() {
        // looks-red supports red; "red light" undercuts that reason
        let mut s = S::new();
        s.assert("looks-red", &[], 1, "perception");
        let r = s.assert("red", &["looks-red"], 1, "looks red");
        s.assert("red-light", &[], 1, "lighting report");
        s.undercut(r, "red-light").unwrap();
        let v = s.status(&"red");
        assert_eq!(v.value, Standing::Out);
        assert!(v.reasons[0].note.contains("undercut by \"red-light\""));
        // the claim "not red" was never asserted: undercutting is not rebutting
        assert_eq!(st(&s, "not-red"), Standing::Out);
        // a second, independent reason for red keeps it in
        s.assert("red", &[], 1, "spectrometer");
        assert_eq!(st(&s, "red"), Standing::In);
    }

    #[test]
    fn a_stronger_rebutter_wins_and_a_weaker_one_loses() {
        let mut s = S::new();
        s.assert("flies", &[], 1, "default");
        s.assert("grounded", &[], 5, "veterinary report");
        s.contradict("flies", "grounded");
        assert_eq!(st(&s, "grounded"), Standing::In);
        let v = s.status(&"flies");
        assert_eq!(v.value, Standing::Out);
        assert!(v.reasons[0].note.contains("rebutted by live reason 1"));
    }

    #[test]
    fn equal_rank_rebuttal_is_collective_defeat_both_undecided() {
        let mut s = S::new();
        s.assert("p", &[], 3, "source a");
        s.assert("not-p", &[], 3, "source b");
        s.contradict("p", "not-p");
        assert_eq!(st(&s, "p"), Standing::Undecided);
        assert_eq!(st(&s, "not-p"), Standing::Undecided);
        let v = s.status(&"p");
        assert_eq!(v.reasons[0].rule, "defeat.undecided");
        assert!(v.reasons[0].note.contains("contested"));
    }

    #[test]
    fn a_mutual_undercut_cycle_is_undecided_and_never_picks_a_side() {
        let mut s = S::new();
        let ra = s.assert("a", &[], 1, "ra");
        let rb = s.assert("b", &[], 1, "rb");
        s.undercut(ra, "b").unwrap();
        s.undercut(rb, "a").unwrap();
        assert_eq!(st(&s, "a"), Standing::Undecided);
        assert_eq!(st(&s, "b"), Standing::Undecided);
    }

    #[test]
    fn reinstatement_defeating_the_defeater_restores_the_claim() {
        // c undercuts b's reason, b undercuts a's reason: a is back in
        let mut s = S::new();
        let ra = s.assert("a", &[], 1, "ra");
        let rb = s.assert("b", &[], 1, "rb");
        s.assert("c", &[], 1, "rc");
        s.undercut(ra, "b").unwrap();
        s.undercut(rb, "c").unwrap();
        assert_eq!(st(&s, "c"), Standing::In);
        assert_eq!(st(&s, "b"), Standing::Out);
        assert_eq!(st(&s, "a"), Standing::In);
    }

    #[test]
    fn a_rebuttal_can_be_beaten_by_undercutting_the_stronger_reason() {
        let mut s = S::new();
        s.assert("x", &[], 1, "weak");
        let strong = s.assert("not-x", &[], 9, "strong");
        s.contradict("x", "not-x");
        assert_eq!(st(&s, "x"), Standing::Out);
        s.assert("strong-is-unreliable", &[], 1, "audit");
        s.undercut(strong, "strong-is-unreliable").unwrap();
        assert_eq!(st(&s, "not-x"), Standing::Out);
        assert_eq!(st(&s, "x"), Standing::In, "its only rebutter is dead");
    }

    #[test]
    fn defeat_propagates_through_inference() {
        let mut s = S::new();
        let obs = s.assert("a", &[], 1, "obs");
        s.assert("b", &["a"], 1, "inf");
        s.assert("c", &["b"], 1, "inf2");
        s.assert("defeater", &[], 1, "d");
        s.undercut(obs, "defeater").unwrap();
        assert_eq!(st(&s, "a"), Standing::Out);
        assert_eq!(st(&s, "b"), Standing::Out);
        assert_eq!(st(&s, "c"), Standing::Out);
    }

    #[test]
    fn retract_reports_exactly_the_claims_that_changed() {
        let mut s = S::new();
        let ga = s.assert("a", &[], 1, "obs");
        s.assert("b", &["a"], 1, "inf");
        s.assert("d", &[], 1, "other");
        let ch = s.retract(ga).unwrap();
        let moved: Vec<_> = ch.iter().map(|c| (c.claim, c.before, c.after)).collect();
        assert_eq!(
            moved,
            vec![
                ("a", Standing::In, Standing::Out),
                ("b", Standing::In, Standing::Out)
            ]
        );
        assert_eq!(s.retract(ga), Err(EpError::AlreadyRetracted(0)));
        assert_eq!(s.retract(ReasonId(99)), Err(EpError::UnknownReason(99)));
    }

    #[test]
    fn retracting_a_defeater_can_bring_a_claim_back() {
        let mut s = S::new();
        let r = s.assert("x", &[], 1, "x");
        let d = s.assert("d", &[], 1, "d");
        s.undercut(r, "d").unwrap();
        assert_eq!(st(&s, "x"), Standing::Out);
        let ch = s.retract(d).unwrap();
        assert!(ch.iter().any(|c| c.claim == "x" && c.after == Standing::In));
    }

    #[test]
    fn a_self_contradictory_claim_is_never_in() {
        let mut s = S::new();
        s.assert("liar", &[], 1, "r");
        s.contradict("liar", "liar");
        assert_ne!(st(&s, "liar"), Standing::In);
    }

    #[test]
    fn undercut_of_an_unknown_reason_is_an_error() {
        let mut s = S::new();
        assert_eq!(s.undercut(ReasonId(3), "x"), Err(EpError::UnknownReason(3)));
    }
}

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
