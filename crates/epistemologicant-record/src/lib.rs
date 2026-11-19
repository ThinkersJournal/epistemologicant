//! Source record: how reliable has a source been on a topic?
//!
//! Generic over the source `S` and topic `T`; this crate knows nothing of what they mean. What
//! counts as a good outcome, the baseline, the prior strength and the half-life are all data the
//! caller supplies in [`Params`]. Deterministic: no clock (callers pass `now`), no RNG, ordered maps,
//! and only `+ - * /` on floats (no `exp`/`ln`).
//!
//! **Estimate.** Each observation is a good (`true`) or bad (`false`) outcome at time `at`, carrying
//! weight `w(now - at)`. Reliability is
//! `(baseline * prior_strength + sum(w * outcome)) / (prior_strength + sum(w))`:
//! with little evidence the answer stays near the baseline (shrinkage); as evidence accumulates
//! it moves to the source's observed rate. Rule ids: `record.baseline-only` (no observations)
//! and `record.shrunk` (the estimate, with the effective evidence weight in the note).
//!
//! **Recency.** `w(age)` is 1 at age 0 and exactly halves at every `half_life`; between those
//! points it falls linearly. That is exp-free and monotone, and it means old failures fade, so a
//! source that stopped failing can recover (tested).
//!
//! **Topics.** A topic hierarchy is data: [`SourceRecord::set_parent`] links a topic under a
//! broader one. Reliability on a topic pools its own observations with those of every topic below
//! it, so a broad topic reflects its sub-topics and a narrow one only itself.
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub use epistemologicant_core::{EpError, Reason, Verdict};
use epistemologicant_credence::Credence;

/// Caller-set parameters of the estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// Reliability assumed with no evidence, in `[0, 1]`.
    pub baseline: f64,
    /// How many full-weight observations the baseline is worth, `> 0`.
    pub prior_strength: f64,
    /// Age (in the caller's time unit) at which an observation's weight has halved, `> 0`.
    pub half_life: i64,
}

impl Params {
    /// Validate and build.
    pub fn new(baseline: f64, prior_strength: f64, half_life: i64) -> Result<Self, EpError> {
        if !(0.0..=1.0).contains(&baseline) {
            return Err(EpError::BadParam(format!(
                "baseline {baseline} not in [0,1]"
            )));
        }
        if prior_strength.is_nan() || prior_strength <= 0.0 || prior_strength.is_infinite() {
            return Err(EpError::BadParam(format!(
                "prior_strength {prior_strength} must be finite and > 0"
            )));
        }
        if half_life <= 0 {
            return Err(EpError::BadParam(format!(
                "half_life {half_life} must be > 0"
            )));
        }
        Ok(Self {
            baseline,
            prior_strength,
            half_life,
        })
    }

    /// Weight of an observation of the given age (`>= 0`): 1 at 0, halves each half-life, linear between.
    fn weight(&self, age: i64) -> f64 {
        let k = age / self.half_life;
        let rem = age % self.half_life;
        let mut base = 1.0_f64;
        for _ in 0..k.min(1100) {
            base *= 0.5; // exact: powers of two
        }
        base * (1.0 - 0.5 * (rem as f64 / self.half_life as f64))
    }
}

/// Observations of sources on topics.
pub struct SourceRecord<S, T> {
    params: Params,
    obs: BTreeMap<(S, T), Vec<(i64, bool)>>,
    parent: BTreeMap<T, T>,
}

impl<S: Ord + Clone + Debug, T: Ord + Clone + Debug> SourceRecord<S, T> {
    /// An empty record with the given parameters.
    pub fn new(params: Params) -> Self {
        Self {
            params,
            obs: BTreeMap::new(),
            parent: BTreeMap::new(),
        }
    }

    /// Put `child` under the broader `parent`. Refuses a link that would make a cycle.
    pub fn set_parent(&mut self, child: T, parent: T) -> Result<(), EpError> {
        let mut cur = Some(&parent);
        while let Some(t) = cur {
            if *t == child {
                return Err(EpError::TopicCycle(format!("{child:?}")));
            }
            cur = self.parent.get(t);
        }
        self.parent.insert(child, parent);
        Ok(())
    }

    /// Record that `source` was right (`true`) or wrong (`false`) on `topic` at time `at`.
    pub fn observe(&mut self, source: S, topic: T, outcome: bool, at: i64) {
        self.obs
            .entry((source, topic))
            .or_default()
            .push((at, outcome));
    }

    /// `topic` and every topic below it.
    fn subtree(&self, topic: &T) -> BTreeSet<T> {
        let mut set = BTreeSet::new();
        set.insert(topic.clone());
        loop {
            let before = set.len();
            for (c, p) in &self.parent {
                if set.contains(p) {
                    set.insert(c.clone());
                }
            }
            if set.len() == before {
                return set;
            }
        }
    }

    /// The source's reliability on the topic as of `now`, with the reason it is what it is.
    /// Errors if any pooled observation is dated after `now`.
    pub fn reliability(
        &self,
        source: &S,
        topic: &T,
        now: i64,
    ) -> Result<Verdict<Credence>, EpError> {
        let topics = self.subtree(topic);
        let (mut wsum, mut good, mut n) = (0.0_f64, 0.0_f64, 0usize);
        for ((s, t), list) in &self.obs {
            if s != source || !topics.contains(t) {
                continue;
            }
            for &(at, ok) in list {
                if at > now {
                    return Err(EpError::FutureObservation { at, now });
                }
                let w = self.params.weight(now - at);
                wsum += w;
                if ok {
                    good += w;
                }
                n += 1;
            }
        }
        let p = &self.params;
        if n == 0 {
            return Ok(
                Verdict::new(Credence::point(p.baseline)?).because(Reason::new(
                    "record.baseline-only",
                    format!(
                        "no observations of {source:?} on {topic:?}; baseline {}",
                        p.baseline
                    ),
                )),
            );
        }
        let est = (p.baseline * p.prior_strength + good) / (p.prior_strength + wsum);
        Ok(Verdict::new(Credence::point(est)?).because(Reason::new(
            "record.shrunk",
            format!(
                "{source:?} on {topic:?}: {n} observation(s), effective weight {wsum}, prior {} at baseline {}",
                p.prior_strength, p.baseline
            ),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec() -> SourceRecord<&'static str, &'static str> {
        SourceRecord::new(Params::new(0.5, 2.0, 10).unwrap())
    }
    fn pt(v: &Verdict<Credence>) -> f64 {
        match v.value {
            Credence::Point(p) => p,
            ref o => panic!("{o:?}"),
        }
    }

    #[test]
    fn params_reject_bad_values() {
        assert!(Params::new(1.5, 1.0, 1).is_err());
        assert!(Params::new(f64::NAN, 1.0, 1).is_err());
        assert!(Params::new(0.5, 0.0, 1).is_err());
        assert!(Params::new(0.5, 1.0, 0).is_err());
    }

    #[test]
    fn no_data_is_the_baseline_and_says_so() {
        let v = rec().reliability(&"s", &"t", 0).unwrap();
        assert_eq!(pt(&v), 0.5);
        assert_eq!(v.reasons[0].rule, "record.baseline-only");
    }

    #[test]
    fn one_observation_barely_moves_it_many_move_it_a_lot() {
        let mut r = rec();
        r.observe("s", "t", true, 0);
        let one = pt(&r.reliability(&"s", &"t", 0).unwrap());
        assert!((one - (0.5 * 2.0 + 1.0) / 3.0).abs() < 1e-12);
        for _ in 0..98 {
            r.observe("s", "t", true, 0);
        }
        let many = pt(&r.reliability(&"s", &"t", 0).unwrap());
        assert!(many > 0.98 && one < 0.7, "one {one}, many {many}");
    }

    #[test]
    fn weight_halves_at_each_half_life() {
        let p = Params::new(0.5, 1.0, 10).unwrap();
        assert_eq!(p.weight(0), 1.0);
        assert_eq!(p.weight(10), 0.5);
        assert_eq!(p.weight(20), 0.25);
        assert!(p.weight(5) < 1.0 && p.weight(5) > 0.5);
    }

    #[test]
    fn a_source_recovers_after_it_stops_failing() {
        let mut r = rec();
        for _ in 0..10 {
            r.observe("s", "t", false, 0);
        }
        let bad = pt(&r.reliability(&"s", &"t", 0).unwrap());
        for _ in 0..10 {
            r.observe("s", "t", true, 100);
        }
        let later = pt(&r.reliability(&"s", &"t", 100).unwrap());
        assert!(bad < 0.2 && later > 0.8, "bad {bad}, later {later}");
    }

    #[test]
    fn topics_are_separate_and_the_hierarchy_pools_downward() {
        let mut r = rec();
        r.set_parent("physics", "science").unwrap();
        r.set_parent("biology", "science").unwrap();
        for _ in 0..20 {
            r.observe("s", "physics", true, 0);
        }
        for _ in 0..8 {
            r.observe("s", "biology", false, 0);
        }
        assert!(pt(&r.reliability(&"s", &"physics", 0).unwrap()) > 0.9);
        assert!(pt(&r.reliability(&"s", &"biology", 0).unwrap()) < 0.15);
        let sci = pt(&r.reliability(&"s", &"science", 0).unwrap());
        // pooled: (0.5*2 + 20) / (2 + 28)
        assert!(
            (sci - 21.0 / 30.0).abs() < 1e-12,
            "science pools both: {sci}"
        );
        // a narrow topic does not see its parent's data
        assert_eq!(
            r.reliability(&"s", &"chemistry", 0).unwrap().reasons[0].rule,
            "record.baseline-only"
        );
    }

    #[test]
    fn sources_do_not_leak_into_each_other() {
        let mut r = rec();
        for _ in 0..20 {
            r.observe("a", "t", true, 0);
        }
        assert_eq!(
            r.reliability(&"b", &"t", 0).unwrap().reasons[0].rule,
            "record.baseline-only"
        );
    }

    #[test]
    fn parent_cycle_and_future_observation_are_errors() {
        let mut r = rec();
        r.set_parent("a", "b").unwrap();
        r.set_parent("b", "c").unwrap();
        assert!(matches!(
            r.set_parent("c", "a"),
            Err(EpError::TopicCycle(_))
        ));
        assert!(matches!(
            r.set_parent("a", "a"),
            Err(EpError::TopicCycle(_))
        ));
        r.observe("s", "a", true, 50);
        assert_eq!(
            r.reliability(&"s", &"a", 10),
            Err(EpError::FutureObservation { at: 50, now: 10 })
        );
    }

    #[test]
    fn estimate_stays_within_unit_interval() {
        let mut r = rec();
        for i in 0..50 {
            r.observe("s", "t", i % 3 == 0, i);
            let v = pt(&r.reliability(&"s", &"t", i).unwrap());
            assert!((0.0..=1.0).contains(&v));
        }
    }
}
