//! Justification tracking (a truth-maintenance core): claims, the grounds that support them,
//! retraction that reports exactly what lost support, and a status that says why.
//!
//! Generic over the claim id `C`: this crate knows nothing of what claims say. Deterministic
//! (ordered maps, no clock, no RNG, no floating point).
//!
//! A **ground** is either a premise (supported from outside, e.g. an observation) or an inference
//! (supports its claim when every antecedent claim is itself supported). A claim is **supported**
//! when some active ground supports it *well-foundedly*: support is computed as a least fixpoint
//! from the premises, so a cycle of claims that only support each other is NOT supported
//! (`justify.unsupported`), and a claim with several grounds survives the loss of one.
//!
//! Claims may be declared **contradictory** in pairs. A supported claim whose partner is also
//! supported is `Contradicted` (both are); that is reported, never resolved by picking a side.
//!
//! Rule ids in verdict reasons: `justify.held.premise`, `justify.held.inference`,
//! `justify.contradicted`, `justify.unsupported`.
//!
//! Credit: the design follows the justification-based truth maintenance of Doyle (1979), restricted
//! to monotone grounds (no "out" antecedents); well-founded support is the standard device.
//!
//! # Example
//!
//! ```
//! use epistemologicant_justify::*;
//!
//! let mut j = Justifications::new();
//! let obs = j.assert("wet-grass", Ground::Premise { label: "saw it".into() });
//! j.assert("it-rained", Ground::Inference { antecedents: vec!["wet-grass"] });
//! assert_eq!(j.status(&"it-rained").value, Status::Held);
//!
//! // Withdrawing the observation takes the conclusion with it, and says exactly what was lost.
//! let fallout = j.retract(obs)?;
//! assert_eq!(fallout.lost, vec!["it-rained", "wet-grass"]);
//! assert_eq!(j.status(&"it-rained").value, Status::Unsupported);
//! # Ok::<(), EpError>(())
//! ```
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub use epistemologicant_core::{EpError, Reason, Verdict};

/// Identifies one ground, issued by [`Justifications::assert`]. Ids are sequential from 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundId(pub u64);

/// What supports a claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ground<C> {
    /// Supported from outside the system; `label` says where from (kept for reasons).
    Premise {
        /// Free-text origin, shown in reasons.
        label: String,
    },
    /// Supports the claim when every antecedent is supported. An empty list is a valid
    /// inference and supports unconditionally (it behaves like a premise, reported as inference).
    Inference {
        /// Claims that must all be supported.
        antecedents: Vec<C>,
    },
}

/// The standing of a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Supported, and no contradictory partner is supported.
    Held,
    /// No well-founded active support (never asserted, all grounds retracted, or only circular).
    Unsupported,
    /// Supported, but a declared contradictory claim is also supported.
    Contradicted,
}

/// What a retraction took away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fallout<C> {
    /// Exactly the claims that were supported before the retraction and are not after, in claim
    /// order. A claim that kept another ground is NOT here.
    pub lost: Vec<C>,
}

struct Entry<C> {
    claim: C,
    ground: Ground<C>,
    active: bool,
}

/// Claims, their grounds and contradictions.
pub struct Justifications<C> {
    grounds: Vec<Entry<C>>,
    contradictions: BTreeMap<C, BTreeSet<C>>,
}

impl<C> Default for Justifications<C> {
    fn default() -> Self {
        Self {
            grounds: Vec::new(),
            contradictions: BTreeMap::new(),
        }
    }
}

impl<C: Ord + Clone + Debug> Justifications<C> {
    /// An empty system.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a ground for `claim` and return its id. Antecedents need not be asserted yet.
    pub fn assert(&mut self, claim: C, ground: Ground<C>) -> GroundId {
        self.grounds.push(Entry {
            claim,
            ground,
            active: true,
        });
        GroundId(self.grounds.len() as u64 - 1)
    }

    /// Declare two claims contradictory (symmetric). A claim contradicting itself is allowed and
    /// makes it `Contradicted` whenever it is supported.
    pub fn contradict(&mut self, a: C, b: C) {
        self.contradictions
            .entry(a.clone())
            .or_default()
            .insert(b.clone());
        self.contradictions.entry(b).or_default().insert(a);
    }

    /// Withdraw a ground and report exactly which claims lost all support because of it.
    pub fn retract(&mut self, ground: GroundId) -> Result<Fallout<C>, EpError> {
        let idx = ground.0 as usize;
        let Some(e) = self.grounds.get(idx) else {
            return Err(EpError::UnknownGround(ground.0));
        };
        if !e.active {
            return Err(EpError::AlreadyRetracted(ground.0));
        }
        let before = self.supported();
        self.grounds[idx].active = false;
        let after = self.supported();
        let lost = before
            .keys()
            .filter(|c| !after.contains_key(*c))
            .cloned()
            .collect();
        Ok(Fallout { lost })
    }

    /// Well-founded support: least fixpoint from the premises. Maps each supported claim to the
    /// ground that first supported it (so the reported ground is never a circular one).
    fn supported(&self) -> BTreeMap<C, GroundId> {
        let mut held: BTreeMap<C, GroundId> = BTreeMap::new();
        loop {
            let mut grew = false;
            for (i, e) in self.grounds.iter().enumerate() {
                if !e.active || held.contains_key(&e.claim) {
                    continue;
                }
                let ok = match &e.ground {
                    Ground::Premise { .. } => true,
                    Ground::Inference { antecedents } => {
                        antecedents.iter().all(|a| held.contains_key(a))
                    }
                };
                if ok {
                    held.insert(e.claim.clone(), GroundId(i as u64));
                    grew = true;
                }
            }
            if !grew {
                return held;
            }
        }
    }

    /// The claim's status and why. A claim never mentioned is `Unsupported`.
    pub fn status(&self, claim: &C) -> Verdict<Status> {
        let held = self.supported();
        let Some(gid) = held.get(claim) else {
            let total = self.grounds.iter().filter(|e| &e.claim == claim).count();
            let active = self
                .grounds
                .iter()
                .filter(|e| &e.claim == claim && e.active)
                .count();
            let note = if total == 0 {
                format!("{claim:?} was never asserted")
            } else if active == 0 {
                format!("every ground of {claim:?} was retracted")
            } else {
                format!(
                    "{claim:?} has {active} active ground(s) but none is well-founded (missing or circular support)"
                )
            };
            return Verdict::new(Status::Unsupported)
                .because(Reason::new("justify.unsupported", note));
        };
        let partners: Vec<&C> = self
            .contradictions
            .get(claim)
            .map(|s| s.iter().filter(|p| held.contains_key(*p)).collect())
            .unwrap_or_default();
        if !partners.is_empty() {
            return Verdict::new(Status::Contradicted).because(Reason::new(
                "justify.contradicted",
                format!("{claim:?} is supported but contradicts supported {partners:?}"),
            ));
        }
        let reason = match &self.grounds[gid.0 as usize].ground {
            Ground::Premise { label } => Reason::new(
                "justify.held.premise",
                format!("{claim:?} held by ground {} (premise: {label})", gid.0),
            ),
            Ground::Inference { antecedents } => Reason::new(
                "justify.held.inference",
                format!("{claim:?} held by ground {} from {antecedents:?}", gid.0),
            ),
        };
        Verdict::new(Status::Held).because(reason)
    }

    /// Every claim that is currently supported (contradicted ones included), in claim order.
    pub fn supported_claims(&self) -> Vec<C> {
        self.supported().into_keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prem(s: &str) -> Ground<&'static str> {
        Ground::Premise {
            label: s.to_string(),
        }
    }
    fn inf(a: &[&'static str]) -> Ground<&'static str> {
        Ground::Inference {
            antecedents: a.to_vec(),
        }
    }

    #[test]
    fn premise_holds_and_unknown_is_unsupported() {
        let mut j = Justifications::new();
        j.assert("a", prem("obs"));
        let v = j.status(&"a");
        assert_eq!(v.value, Status::Held);
        assert_eq!(v.reasons[0].rule, "justify.held.premise");
        let u = j.status(&"zzz");
        assert_eq!(u.value, Status::Unsupported);
        assert!(u.reasons[0].note.contains("never asserted"));
    }

    #[test]
    fn inference_needs_every_antecedent() {
        let mut j = Justifications::new();
        j.assert("a", prem("x"));
        j.assert("c", inf(&["a", "b"]));
        assert_eq!(j.status(&"c").value, Status::Unsupported);
        j.assert("b", prem("y"));
        let v = j.status(&"c");
        assert_eq!(v.value, Status::Held);
        assert_eq!(v.reasons[0].rule, "justify.held.inference");
    }

    #[test]
    fn retraction_reports_exact_fallout_through_a_chain() {
        let mut j = Justifications::new();
        let g = j.assert("a", prem("x"));
        j.assert("b", inf(&["a"]));
        j.assert("c", inf(&["b"]));
        j.assert("d", prem("independent"));
        let f = j.retract(g).unwrap();
        assert_eq!(f.lost, vec!["a", "b", "c"]);
        assert_eq!(j.status(&"d").value, Status::Held);
    }

    #[test]
    fn second_support_survives_retraction() {
        let mut j = Justifications::new();
        let g1 = j.assert("a", prem("first"));
        j.assert("a", prem("second"));
        j.assert("b", inf(&["a"]));
        let f = j.retract(g1).unwrap();
        assert!(f.lost.is_empty(), "a and b keep support, got {:?}", f.lost);
        assert_eq!(j.status(&"b").value, Status::Held);
    }

    #[test]
    fn a_cycle_does_not_support_itself() {
        let mut j = Justifications::new();
        j.assert("p", inf(&["q"]));
        j.assert("q", inf(&["p"]));
        let v = j.status(&"p");
        assert_eq!(v.value, Status::Unsupported);
        assert!(v.reasons[0].note.contains("circular"));
    }

    #[test]
    fn a_cycle_with_outside_support_loses_everything_when_that_is_retracted() {
        let mut j = Justifications::new();
        let g = j.assert("p", prem("seed"));
        j.assert("q", inf(&["p"]));
        j.assert("p", inf(&["q"])); // p now also "supported" by q
        assert_eq!(j.status(&"p").value, Status::Held);
        let f = j.retract(g).unwrap();
        assert_eq!(f.lost, vec!["p", "q"], "the loop must not hold itself up");
        assert_eq!(j.status(&"p").value, Status::Unsupported);
    }

    #[test]
    fn contradiction_marks_both_and_clears_on_retraction() {
        let mut j = Justifications::new();
        j.assert("x", prem("a"));
        let g = j.assert("not-x", prem("b"));
        j.contradict("x", "not-x");
        assert_eq!(j.status(&"x").value, Status::Contradicted);
        assert_eq!(j.status(&"not-x").value, Status::Contradicted);
        j.retract(g).unwrap();
        assert_eq!(j.status(&"x").value, Status::Held);
        assert_eq!(j.status(&"not-x").value, Status::Unsupported);
    }

    #[test]
    fn unsupported_after_retraction_names_the_cause() {
        let mut j = Justifications::new();
        let g = j.assert("a", prem("x"));
        j.retract(g).unwrap();
        assert!(j.status(&"a").reasons[0].note.contains("retracted"));
    }

    #[test]
    fn bad_retractions_are_errors() {
        let mut j: Justifications<&str> = Justifications::new();
        assert_eq!(j.retract(GroundId(7)), Err(EpError::UnknownGround(7)));
        let g = j.assert("a", prem("x"));
        j.retract(g).unwrap();
        assert_eq!(j.retract(g), Err(EpError::AlreadyRetracted(0)));
    }

    #[test]
    fn empty_inference_supports_unconditionally() {
        let mut j = Justifications::new();
        j.assert("t", inf(&[]));
        assert_eq!(j.status(&"t").value, Status::Held);
    }
}
