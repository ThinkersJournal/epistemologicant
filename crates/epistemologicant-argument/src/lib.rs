//! Abstract argumentation in the style of Dung: a set of arguments and an attack relation, with the
//! usual semantics and a reasoned grounded verdict.
//!
//! Generic over the argument id `A`: this crate knows nothing of what arguments say. Deterministic
//! (ordered sets, no clock, no RNG, no floating point). Arguments are ordered by `A: Ord`; every
//! list of extensions is in a fixed order (see [`ArgumentationFramework::extensions`]).
//!
//! # Definitions (as read; see "Credit")
//! For a set `E` of arguments: `E` is **conflict-free** if no member attacks another; `E` **defends**
//! `a` if every attacker of `a` is attacked by a member of `E` (`a` is *acceptable* w.r.t. `E`); `E` is
//! **admissible** if conflict-free and it defends each member; **complete** if admissible and it
//! contains every argument it defends; **preferred** if a maximal (by inclusion) admissible set;
//! **stable** if conflict-free and it attacks every argument outside it; **grounded** if the
//! smallest complete extension, equivalently the least fixed point of `E -> {a : E defends a}`.
//! The grounded extension is unique and exists for every finite framework; a stable one may not.
//!
//! # Grounded verdict
//! [`ArgumentationFramework::grounded`] labels each argument [`Label::In`] (all its attackers are
//! out), [`Label::Out`] (some attacker is in) or [`Label::Undecided`], and
//! [`ArgumentationFramework::explain`] says why. Rule ids: `argument.in`, `argument.out`,
//! `argument.undecided`.
//!
//! # Limits and cost
//! Enumerating complete, preferred and stable extensions is exhaustive: it visits all `2^n` subsets of
//! the `n` arguments, each at a cost of roughly `n^2` bit operations (preferred also compares admissible
//! sets pairwise). At the limit of [`ENUMERATION_LIMIT`] = 20 that is about a million subsets, which takes
//! a moment in a release build and about two seconds in a debug one (measured); every extra argument doubles it. Above the
//! limit these calls return `Err(`[`EpError::TooLarge`]`)` immediately, before any work, and never
//! panic. [`ArgumentationFramework::grounded`] and [`ArgumentationFramework::explain`] have no such
//! limit (a few passes over the attack list). *Skeptical* acceptance under a semantics with **no** extension (stable can have none)
//! is vacuously true by definition; the verdict says so (`argument.skeptical.no-extensions`).
//!
//! # Credit and what was read
//! The definitions are Dung's (Dung, P. M. (1995), "On the acceptability of arguments and its
//! fundamental role in nonmonotonic reasoning, logic programming and n-person games", *Artificial
//! Intelligence* 77(2): 321-357). That paper was **not** read: the definitions were read second
//! hand in Wikipedia "Argumentation framework" (revision of 19 Nov 2026) and the Stanford
//! Encyclopedia of Philosophy entry "Defeasible Reasoning" (§4.4, partly via a summary). The
//! least-fixed-point statement comes from the SEP summary and from arXiv survey text surfaced in the
//! patent sweep, not from Dung. Skepticism of the grounded extension is an inference from those
//! sources, not a quotation.
//!
//! # Example
//!
//! ```
//! use epistemologicant_argument::*;
//!
//! let mut f = ArgumentationFramework::new();
//! for a in ["a", "b", "c"] {
//!     f.add_argument(a);
//! }
//! f.add_attack("b", "a")?; // b attacks a
//! f.add_attack("c", "b")?; // c attacks b, so c defends a
//!
//! assert_eq!(f.grounded()["a"], Label::In); // reinstated
//! assert_eq!(f.explain(&"b")?.reasons[0].rule, "argument.out");
//! assert_eq!(f.extensions(Semantics::Preferred)?.len(), 1);
//! # Ok::<(), EpError>(())
//! ```
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub use epistemologicant_core::{EpError, Reason, Verdict};

/// Exhaustive enumeration is refused above this many arguments.
pub const ENUMERATION_LIMIT: usize = 20;

/// The standing of an argument in the grounded labelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label {
    /// Every attacker is out.
    In,
    /// Some attacker is in.
    Out,
    /// Neither is forced.
    Undecided,
}

/// Which extensions to speak about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semantics {
    /// The one smallest complete extension.
    Grounded,
    /// Every complete extension.
    Complete,
    /// The maximal admissible sets.
    Preferred,
    /// Conflict-free sets that attack everything outside them (may be none).
    Stable,
}

/// Arguments and who attacks whom.
pub struct ArgumentationFramework<A> {
    args: BTreeSet<A>,
    attacks: BTreeSet<(A, A)>,
}

impl<A> Default for ArgumentationFramework<A> {
    fn default() -> Self {
        Self {
            args: BTreeSet::new(),
            attacks: BTreeSet::new(),
        }
    }
}

/// Index form for fast set algebra on bitmasks.
struct Compiled<A> {
    order: Vec<A>,
    /// `attackers[i]`: mask of arguments attacking `i`.
    attackers: Vec<u32>,
    /// `targets[i]`: mask of arguments attacked by `i`.
    targets: Vec<u32>,
}

impl<A: Ord + Clone + Debug> ArgumentationFramework<A> {
    /// An empty framework.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an argument; returns `false` if it was already there.
    pub fn add_argument(&mut self, a: A) -> bool {
        self.args.insert(a)
    }

    /// Record that `attacker` attacks `target`. Both must exist (self-attack is allowed).
    pub fn add_attack(&mut self, attacker: A, target: A) -> Result<(), EpError> {
        for a in [&attacker, &target] {
            if !self.args.contains(a) {
                return Err(EpError::UnknownItem(format!("{a:?}")));
            }
        }
        self.attacks.insert((attacker, target));
        Ok(())
    }

    /// Number of arguments.
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// True when there are no arguments.
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    fn attackers_of(&self, a: &A) -> Vec<&A> {
        self.attacks
            .iter()
            .filter(|(_, t)| t == a)
            .map(|(s, _)| s)
            .collect()
    }

    fn compile(&self) -> Option<Compiled<A>> {
        if self.args.len() > 31 {
            return None;
        }
        let order: Vec<A> = self.args.iter().cloned().collect();
        let idx: BTreeMap<&A, usize> = order.iter().enumerate().map(|(i, a)| (a, i)).collect();
        let n = order.len();
        let (mut attackers, mut targets) = (vec![0u32; n], vec![0u32; n]);
        for (s, t) in &self.attacks {
            attackers[idx[t]] |= 1 << idx[s];
            targets[idx[s]] |= 1 << idx[t];
        }
        Some(Compiled {
            order,
            attackers,
            targets,
        })
    }

    /// The grounded labelling, as the least fixed point: repeat "in if all attackers are out; out
    /// if some attacker is in" until nothing changes. No size limit.
    pub fn grounded(&self) -> BTreeMap<A, Label> {
        let mut lab: BTreeMap<&A, Label> =
            self.args.iter().map(|a| (a, Label::Undecided)).collect();
        loop {
            let mut changed = false;
            for a in &self.args {
                if lab[a] != Label::Undecided {
                    continue;
                }
                let att = self.attackers_of(a);
                if att.iter().all(|x| lab[*x] == Label::Out) {
                    lab.insert(a, Label::In);
                    changed = true;
                } else if att.iter().any(|x| lab[*x] == Label::In) {
                    lab.insert(a, Label::Out);
                    changed = true;
                }
            }
            if !changed {
                return lab.into_iter().map(|(a, l)| (a.clone(), l)).collect();
            }
        }
    }

    /// The grounded label of one argument and why. An unknown argument is an error.
    pub fn explain(&self, a: &A) -> Result<Verdict<Label>, EpError> {
        if !self.args.contains(a) {
            return Err(EpError::UnknownItem(format!("{a:?}")));
        }
        let lab = self.grounded();
        let att = self.attackers_of(a);
        let v = match lab[a] {
            Label::In if att.is_empty() => Verdict::new(Label::In)
                .because(Reason::new("argument.in", format!("{a:?} is unattacked"))),
            Label::In => Verdict::new(Label::In).because(Reason::new(
                "argument.in",
                format!("{a:?} is in: every attacker is out ({att:?})"),
            )),
            Label::Out => {
                let by = att.iter().find(|x| lab[**x] == Label::In).copied();
                Verdict::new(Label::Out).because(Reason::new(
                    "argument.out",
                    format!("{a:?} is out: attacked by {by:?}, which is in"),
                ))
            }
            Label::Undecided => {
                let open: Vec<&&A> = att
                    .iter()
                    .filter(|x| lab[**x] == Label::Undecided)
                    .collect();
                Verdict::new(Label::Undecided).because(Reason::new(
                    "argument.undecided",
                    format!("{a:?} is undecided: attacker(s) {open:?} are undecided"),
                ))
            }
        };
        Ok(v)
    }

    /// The extensions under `semantics`, each a set of arguments. Order: grounded is one set;
    /// the others are in increasing order of their membership bitmask over the arguments sorted by
    /// `A: Ord` (so deterministic). Refuses more than [`ENUMERATION_LIMIT`] arguments for the
    /// non-grounded semantics.
    pub fn extensions(&self, semantics: Semantics) -> Result<Vec<BTreeSet<A>>, EpError> {
        if semantics == Semantics::Grounded {
            let g = self
                .grounded()
                .into_iter()
                .filter(|(_, l)| *l == Label::In)
                .map(|(a, _)| a)
                .collect();
            return Ok(vec![g]);
        }
        if self.args.len() > ENUMERATION_LIMIT {
            return Err(EpError::TooLarge {
                arguments: self.args.len(),
                limit: ENUMERATION_LIMIT,
            });
        }
        let c = self.compile().expect("limit checked above");
        let n = c.order.len();
        let full: u32 = if n == 0 { 0 } else { (1u32 << n) - 1 };
        let attacked = |s: u32| -> u32 {
            (0..n)
                .filter(|i| s >> i & 1 == 1)
                .fold(0, |m, i| m | c.targets[i])
        };
        let defends = |s: u32| -> u32 {
            let hit = attacked(s);
            (0..n)
                .filter(|&i| c.attackers[i] & !hit == 0)
                .fold(0u32, |m, i| m | 1 << i)
        };
        let conflict_free = |s: u32| s & attacked(s) == 0;
        let admissible = |s: u32| conflict_free(s) && s & !defends(s) == 0;
        let masks: Vec<u32> = match semantics {
            Semantics::Complete => (0..=full)
                .filter(|&s| conflict_free(s) && defends(s) == s)
                .collect(),
            Semantics::Stable => (0..=full)
                .filter(|&s| conflict_free(s) && attacked(s) == full & !s)
                .collect(),
            Semantics::Preferred => {
                let adm: Vec<u32> = (0..=full).filter(|&s| admissible(s)).collect();
                adm.iter()
                    .copied()
                    .filter(|&s| !adm.iter().any(|&t| t != s && s & !t == 0))
                    .collect()
            }
            Semantics::Grounded => unreachable!(),
        };
        Ok(masks
            .into_iter()
            .map(|m| {
                (0..n)
                    .filter(|i| m >> i & 1 == 1)
                    .map(|i| c.order[i].clone())
                    .collect()
            })
            .collect())
    }

    /// Is `a` in at least one extension of `semantics`?
    pub fn credulously_accepted(
        &self,
        semantics: Semantics,
        a: &A,
    ) -> Result<Verdict<bool>, EpError> {
        if !self.args.contains(a) {
            return Err(EpError::UnknownItem(format!("{a:?}")));
        }
        let ex = self.extensions(semantics)?;
        let n = ex.iter().filter(|e| e.contains(a)).count();
        Ok(Verdict::new(n > 0).because(Reason::new(
            "argument.credulous",
            format!("{a:?} is in {n} of {} {semantics:?} extension(s)", ex.len()),
        )))
    }

    /// Is `a` in every extension of `semantics`? With no extension at all this is vacuously true
    /// (reason `argument.skeptical.no-extensions`).
    pub fn skeptically_accepted(
        &self,
        semantics: Semantics,
        a: &A,
    ) -> Result<Verdict<bool>, EpError> {
        if !self.args.contains(a) {
            return Err(EpError::UnknownItem(format!("{a:?}")));
        }
        let ex = self.extensions(semantics)?;
        if ex.is_empty() {
            return Ok(Verdict::new(true).because(Reason::new(
                "argument.skeptical.no-extensions",
                format!("{semantics:?} has no extension, so every argument is vacuously accepted"),
            )));
        }
        let n = ex.iter().filter(|e| e.contains(a)).count();
        Ok(Verdict::new(n == ex.len()).because(Reason::new(
            "argument.skeptical",
            format!("{a:?} is in {n} of {} {semantics:?} extension(s)", ex.len()),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type F = ArgumentationFramework<&'static str>;

    fn af(args: &[&'static str], attacks: &[(&'static str, &'static str)]) -> F {
        let mut f = F::new();
        for a in args {
            f.add_argument(*a);
        }
        for (s, t) in attacks {
            f.add_attack(*s, *t).unwrap();
        }
        f
    }

    fn set(xs: &[&'static str]) -> BTreeSet<&'static str> {
        xs.iter().copied().collect()
    }

    /// The example in the Wikipedia article read: A={a,b,c,d}, R={(a,b),(b,c),(d,c)}: the single
    /// extension is {a,d}.
    #[test]
    fn the_published_four_argument_example_has_one_extension_a_d() {
        let f = af(&["a", "b", "c", "d"], &[("a", "b"), ("b", "c"), ("d", "c")]);
        for sem in [
            Semantics::Grounded,
            Semantics::Complete,
            Semantics::Preferred,
            Semantics::Stable,
        ] {
            assert_eq!(
                f.extensions(sem).unwrap(),
                vec![set(&["a", "d"])],
                "{sem:?}"
            );
        }
        assert_eq!(f.grounded()["c"], Label::Out);
    }

    #[test]
    fn a_two_cycle_has_two_preferred_and_stable_extensions_and_an_empty_grounded_one() {
        let f = af(&["a", "b"], &[("a", "b"), ("b", "a")]);
        assert_eq!(f.extensions(Semantics::Grounded).unwrap(), vec![set(&[])]);
        assert_eq!(
            f.extensions(Semantics::Preferred).unwrap(),
            vec![set(&["a"]), set(&["b"])]
        );
        assert_eq!(f.extensions(Semantics::Stable).unwrap().len(), 2);
        assert_eq!(f.extensions(Semantics::Complete).unwrap().len(), 3);
        assert_eq!(f.grounded()["a"], Label::Undecided);
        let v = f.explain(&"a").unwrap();
        assert_eq!(v.reasons[0].rule, "argument.undecided");
    }

    #[test]
    fn an_odd_cycle_has_no_stable_extension_and_skeptical_acceptance_is_vacuous_there() {
        let f = af(&["a", "b", "c"], &[("a", "b"), ("b", "c"), ("c", "a")]);
        assert!(f.extensions(Semantics::Stable).unwrap().is_empty());
        let v = f.skeptically_accepted(Semantics::Stable, &"a").unwrap();
        assert!(v.value);
        assert_eq!(v.reasons[0].rule, "argument.skeptical.no-extensions");
        assert_eq!(f.extensions(Semantics::Preferred).unwrap(), vec![set(&[])]);
    }

    #[test]
    fn a_self_attacker_is_never_in_any_extension() {
        let f = af(&["x", "y"], &[("x", "x")]);
        for sem in [Semantics::Complete, Semantics::Preferred, Semantics::Stable] {
            assert!(
                f.extensions(sem).unwrap().iter().all(|e| !e.contains("x")),
                "{sem:?}"
            );
        }
        assert_eq!(f.grounded()["x"], Label::Undecided);
        assert_eq!(f.grounded()["y"], Label::In);
    }

    #[test]
    fn reinstatement_the_defended_argument_comes_back() {
        // c attacks b, b attacks a: a is defended by c
        let f = af(&["a", "b", "c"], &[("b", "a"), ("c", "b")]);
        let g = f.grounded();
        assert_eq!((g["a"], g["b"], g["c"]), (Label::In, Label::Out, Label::In));
        let v = f.explain(&"b").unwrap();
        assert_eq!(v.reasons[0].rule, "argument.out");
        assert!(
            v.reasons[0].note.contains("attacked by Some(\"c\")"),
            "{:?}",
            v.reasons[0]
        );
    }

    #[test]
    fn credulous_and_skeptical_differ_on_a_choice() {
        let f = af(&["a", "b"], &[("a", "b"), ("b", "a")]);
        assert!(
            f.credulously_accepted(Semantics::Preferred, &"a")
                .unwrap()
                .value
        );
        assert!(
            !f.skeptically_accepted(Semantics::Preferred, &"a")
                .unwrap()
                .value
        );
    }

    #[test]
    fn unknown_arguments_are_errors() {
        let mut f = af(&["a"], &[]);
        assert!(matches!(
            f.add_attack("a", "ghost"),
            Err(EpError::UnknownItem(_))
        ));
        assert!(matches!(f.explain(&"ghost"), Err(EpError::UnknownItem(_))));
    }

    /// Run `f` on a worker thread and fail (not hang) if it has not answered within `secs`. A worker that is
    /// still spinning is abandoned; the test process exits when the harness finishes.
    fn within<T: Send + 'static>(secs: u64, f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(f());
        });
        rx.recv_timeout(std::time::Duration::from_secs(secs))
            .expect("did not answer in time: the size limit must refuse BEFORE enumerating")
    }

    #[test]
    fn too_many_arguments_is_a_typed_error_returned_at_once_never_a_panic_or_a_stall() {
        for n in [ENUMERATION_LIMIT + 1, 31, 32, 100] {
            for sem in [Semantics::Complete, Semantics::Preferred, Semantics::Stable] {
                let r = within(5, move || {
                    let mut big = ArgumentationFramework::<u8>::new();
                    for i in 0..n as u8 {
                        big.add_argument(i);
                    }
                    big.extensions(sem)
                });
                assert_eq!(
                    r,
                    Err(EpError::TooLarge {
                        arguments: n,
                        limit: ENUMERATION_LIMIT
                    }),
                    "{n} arguments, {sem:?}"
                );
            }
        }
        // credulous and skeptical acceptance go through the same check
        let r = within(5, || {
            let mut big = ArgumentationFramework::<u8>::new();
            for i in 0..25u8 {
                big.add_argument(i);
            }
            big.credulously_accepted(Semantics::Preferred, &0)
        });
        assert!(matches!(r, Err(EpError::TooLarge { .. })));
        // grounded has no limit
        let mut big = ArgumentationFramework::<u8>::new();
        for i in 0..100u8 {
            big.add_argument(i);
        }
        assert_eq!(big.grounded().len(), 100);
    }

    #[test]
    fn exactly_the_limit_is_allowed() {
        let mut f = ArgumentationFramework::<u8>::new();
        for i in 0..ENUMERATION_LIMIT as u8 {
            f.add_argument(i);
        }
        let t = std::time::Instant::now();
        let ex = within(120, move || f.extensions(Semantics::Stable)).unwrap();
        assert_eq!(
            ex.len(),
            1,
            "no attacks: the only stable extension is everything"
        );
        eprintln!(
            "enumerating {ENUMERATION_LIMIT} arguments took {:?}",
            t.elapsed()
        );
    }

    #[test]
    fn the_empty_framework_has_the_empty_extension() {
        let f = F::new();
        assert_eq!(f.extensions(Semantics::Preferred).unwrap(), vec![set(&[])]);
        assert_eq!(f.extensions(Semantics::Grounded).unwrap(), vec![set(&[])]);
    }
}
