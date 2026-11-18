//! Credence: an uncertainty value with an honest "unknown", combination rules and scoring rules.
//!
//! Generic over what the credence is *about*: this crate knows nothing of claims, sources or topics.
//!
//! Numeric policy: combination rules use only `+ - * /` and `min`/`max`, which IEEE 754 fixes
//! bit-for-bit on every platform. [`log_score`] needs `ln`, which depends on the platform's libm, so it
//! is the one function NOT covered by the byte-identical golden test.
#![forbid(unsafe_code)]

pub use epistemologicant_core::{EpError, Reason, Verdict};

/// How sure something is. `Unknown` means "no evidence", which is not the same as `Point(0.0)`
/// ("evidence that it is false").
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Credence {
    /// No information.
    Unknown,
    /// A probability in `[0, 1]`.
    Point(f64),
    /// Imprecise bounds, `0 <= lo <= hi <= 1`.
    Interval {
        /// Lower bound.
        lo: f64,
        /// Upper bound.
        hi: f64,
    },
    /// A level index into a caller-supplied [`LevelScale`].
    Ordinal(usize),
}

fn check_p(p: f64) -> Result<f64, EpError> {
    if p.is_nan() || !(0.0..=1.0).contains(&p) {
        Err(EpError::OutOfRange(p))
    } else {
        Ok(p)
    }
}

impl Credence {
    /// A validated point credence.
    pub fn point(p: f64) -> Result<Self, EpError> {
        check_p(p).map(Self::Point)
    }

    /// A validated interval credence.
    pub fn interval(lo: f64, hi: f64) -> Result<Self, EpError> {
        check_p(lo)?;
        check_p(hi)?;
        if lo > hi {
            return Err(EpError::BadInterval { lo, hi });
        }
        Ok(Self::Interval { lo, hi })
    }

    /// Check the invariants of a value built directly from the public variants.
    pub fn validate(&self) -> Result<(), EpError> {
        match *self {
            Self::Point(p) => check_p(p).map(|_| ()),
            Self::Interval { lo, hi } => Self::interval(lo, hi).map(|_| ()),
            Self::Unknown | Self::Ordinal(_) => Ok(()),
        }
    }

    /// `(lo, hi)` bounds, or `None` for `Unknown`. Ordinals are refused: they need a scale.
    fn bounds(&self) -> Result<Option<(f64, f64)>, EpError> {
        self.validate()?;
        match *self {
            Self::Unknown => Ok(None),
            Self::Point(p) => Ok(Some((p, p))),
            Self::Interval { lo, hi } => Ok(Some((lo, hi))),
            Self::Ordinal(_) => Err(EpError::OrdinalNeedsScale),
        }
    }
}

fn from_bounds(lo: f64, hi: f64) -> Credence {
    if lo == hi {
        Credence::Point(lo)
    } else {
        Credence::Interval { lo, hi }
    }
}

/// A named ordinal scale of `n >= 2` equal-width levels over `[0, 1]`. The names are data the caller
/// supplies; nothing here is English. Level `i` covers `[i/n, (i+1)/n]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelScale {
    names: Vec<String>,
}

impl LevelScale {
    /// Build a scale from level names, lowest first.
    pub fn new(names: Vec<String>) -> Result<Self, EpError> {
        if names.len() < 2 {
            return Err(EpError::ScaleTooSmall(names.len()));
        }
        Ok(Self { names })
    }

    /// Number of levels.
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Always false: a scale has at least two levels.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Name of a level.
    pub fn name(&self, level: usize) -> Option<&str> {
        self.names.get(level).map(String::as_str)
    }

    /// The interval a level stands for. Honest: a level is a range, never a point.
    pub fn from_level(&self, level: usize) -> Result<Credence, EpError> {
        let n = self.names.len();
        if level >= n {
            return Err(EpError::LevelOutOfScale { level, levels: n });
        }
        Ok(Credence::Interval {
            lo: level as f64 / n as f64,
            hi: (level + 1) as f64 / n as f64,
        })
    }

    /// The level a credence falls in. A point on a boundary belongs to the higher level (the top
    /// level includes 1.0). An interval is accepted only if it lies inside one level.
    pub fn to_level(&self, c: &Credence) -> Result<usize, EpError> {
        let n = self.names.len();
        let bin = |p: f64| -> usize { ((p * n as f64) as usize).min(n - 1) };
        match *c {
            Credence::Unknown => Err(EpError::UnknownValue),
            Credence::Ordinal(l) if l < n => Ok(l),
            Credence::Ordinal(l) => Err(EpError::LevelOutOfScale {
                level: l,
                levels: n,
            }),
            _ => {
                let (lo, hi) = c.bounds()?.ok_or(EpError::UnknownValue)?;
                let (a, b) = (bin(lo), bin(hi));
                // an upper bound exactly on a boundary still belongs to the level below it
                let b = if hi > lo && hi * n as f64 == b as f64 && b > 0 {
                    b - 1
                } else {
                    b
                };
                if a == b {
                    Ok(a)
                } else {
                    Err(EpError::StraddlesLevels)
                }
            }
        }
    }

    /// Resolve an ordinal to an interval through this scale; other credences pass through unchanged.
    pub fn resolve(&self, c: &Credence) -> Result<Credence, EpError> {
        match *c {
            Credence::Ordinal(l) => self.from_level(l),
            other => Ok(other),
        }
    }
}

/// What to do when two credences disagree about the same claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictPolicy {
    /// Keep both: the interval spanning the two (cautious; widens under disagreement).
    Hull,
    /// Split the difference: the arithmetic mean of the bounds.
    Mean,
}

/// Combine credences from INDEPENDENT pieces of evidence that each support the same claim, by
/// noisy-OR: `1 - prod(1 - p_i)`. It is monotone, so intervals combine exactly by combining the
/// lower bounds and the upper bounds. `Unknown` inputs add nothing (recorded in the reasons); all-unknown
/// stays `Unknown`. Independence is the caller's claim: this function cannot check it (see the
/// evidence-graph crate for that).
pub fn combine_independent(inputs: &[Credence]) -> Result<Verdict<Credence>, EpError> {
    if inputs.is_empty() {
        return Err(EpError::Empty);
    }
    let (mut keep_lo, mut keep_hi) = (1.0_f64, 1.0_f64);
    let mut used = 0usize;
    for c in inputs {
        if let Some((lo, hi)) = c.bounds()? {
            keep_lo *= 1.0 - lo;
            keep_hi *= 1.0 - hi;
            used += 1;
        }
    }
    let unknowns = inputs.len() - used;
    if used == 0 {
        return Ok(Verdict::new(Credence::Unknown).because(Reason::new(
            "credence.independent.all-unknown",
            "every input was unknown, so the result is unknown (not zero)",
        )));
    }
    let mut v = Verdict::new(from_bounds(1.0 - keep_lo, 1.0 - keep_hi)).because(Reason::new(
        "credence.independent.noisy-or",
        format!("combined {used} independent input(s) by noisy-OR"),
    ));
    if unknowns > 0 {
        v = v.because(Reason::new(
            "credence.independent.skipped-unknown",
            format!("{unknowns} unknown input(s) contributed nothing"),
        ));
    }
    Ok(v)
}

/// Combine two credences that disagree about the same claim, under an explicit policy. If either is
/// `Unknown` the other is returned unchanged (and the reason says so).
pub fn combine_conflicting(
    a: &Credence,
    b: &Credence,
    policy: ConflictPolicy,
) -> Result<Verdict<Credence>, EpError> {
    match (a.bounds()?, b.bounds()?) {
        (None, None) => Ok(Verdict::new(Credence::Unknown).because(Reason::new(
            "credence.conflict.both-unknown",
            "both unknown: nothing to reconcile",
        ))),
        (Some(_), None) => Ok(Verdict::new(*a).because(Reason::new(
            "credence.conflict.one-unknown",
            "the other side is unknown, so this one stands",
        ))),
        (None, Some(_)) => Ok(Verdict::new(*b).because(Reason::new(
            "credence.conflict.one-unknown",
            "the other side is unknown, so this one stands",
        ))),
        (Some((al, ah)), Some((bl, bh))) => {
            let (value, rule, note) = match policy {
                ConflictPolicy::Hull => (
                    from_bounds(al.min(bl), ah.max(bh)),
                    "credence.conflict.hull",
                    "kept both: the interval spanning the two",
                ),
                ConflictPolicy::Mean => (
                    from_bounds((al + bl) / 2.0, (ah + bh) / 2.0),
                    "credence.conflict.mean",
                    "averaged the bounds",
                ),
            };
            Ok(Verdict::new(value).because(Reason::new(rule, note)))
        }
    }
}

fn point_outcomes(pairs: &[(Credence, bool)]) -> Result<Vec<(f64, bool)>, EpError> {
    if pairs.is_empty() {
        return Err(EpError::Empty);
    }
    pairs
        .iter()
        .map(|(c, o)| match c.bounds()? {
            None => Err(EpError::UnknownValue),
            Some((lo, hi)) if lo == hi => Ok((lo, *o)),
            Some(_) => Err(EpError::ScoringNeedsPoint),
        })
        .collect()
}

/// Mean Brier score, `mean((p - outcome)^2)`; 0 is perfect, lower is better. Intervals and unknowns
/// are refused rather than collapsed to a point.
pub fn brier(pairs: &[(Credence, bool)]) -> Result<f64, EpError> {
    let v = point_outcomes(pairs)?;
    let sum: f64 = v
        .iter()
        .map(|&(p, o)| {
            let d = p - if o { 1.0 } else { 0.0 };
            d * d
        })
        .sum();
    Ok(sum / v.len() as f64)
}

/// Mean logarithmic score, `mean(-ln(probability given to what happened))`; lower is better, and
/// infinite if something that happened was given probability 0. Uses `ln`: platform-libm dependent,
/// so NOT bit-identical across platforms (unlike every other function here).
pub fn log_score(pairs: &[(Credence, bool)]) -> Result<f64, EpError> {
    let v = point_outcomes(pairs)?;
    let sum: f64 = v
        .iter()
        .map(|&(p, o)| -(if o { p } else { 1.0 - p }).ln())
        .sum();
    Ok(sum / v.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(p: f64) -> Credence {
        Credence::point(p).unwrap()
    }

    #[test]
    fn rejects_bad_values() {
        assert_eq!(Credence::point(1.5), Err(EpError::OutOfRange(1.5)));
        assert!(Credence::point(f64::NAN).is_err());
        assert_eq!(
            Credence::interval(0.8, 0.2),
            Err(EpError::BadInterval { lo: 0.8, hi: 0.2 })
        );
        // a value built from the public variants is still checked by the operations
        assert!(combine_independent(&[Credence::Point(2.0)]).is_err());
    }

    #[test]
    fn unknown_is_not_zero() {
        assert_ne!(Credence::Unknown, pt(0.0));
        let v = combine_independent(&[Credence::Unknown, Credence::Unknown]).unwrap();
        assert_eq!(v.value, Credence::Unknown);
        let v = combine_independent(&[Credence::Unknown, pt(0.5)]).unwrap();
        assert_eq!(v.value, pt(0.5));
        assert!(
            v.reasons
                .iter()
                .any(|r| r.rule == "credence.independent.skipped-unknown")
        );
    }

    #[test]
    fn noisy_or_values() {
        let v = combine_independent(&[pt(0.5), pt(0.5)]).unwrap();
        assert_eq!(v.value, pt(0.75));
        let v = combine_independent(&[pt(0.5), pt(1.0)]).unwrap();
        assert_eq!(v.value, pt(1.0));
        let v = combine_independent(&[pt(0.0), pt(0.0)]).unwrap();
        assert_eq!(v.value, pt(0.0));
    }

    #[test]
    fn combination_is_order_independent_and_monotone() {
        let a = combine_independent(&[pt(0.2), pt(0.3), pt(0.6)])
            .unwrap()
            .value;
        let b = combine_independent(&[pt(0.6), pt(0.2), pt(0.3)])
            .unwrap()
            .value;
        assert_eq!(a, b);
        // adding evidence never lowers credence
        let base = combine_independent(&[pt(0.2), pt(0.3)]).unwrap().value;
        let (Credence::Point(x), Credence::Point(y)) = (base, a) else {
            panic!()
        };
        assert!(y >= x);
    }

    #[test]
    fn intervals_combine_by_bounds() {
        let i = Credence::interval(0.2, 0.4).unwrap();
        let v = combine_independent(&[i, pt(0.5)]).unwrap();
        let Credence::Interval { lo, hi } = v.value else {
            panic!("{:?}", v.value)
        };
        assert!((lo - 0.6).abs() < 1e-12 && (hi - 0.7).abs() < 1e-12);
    }

    #[test]
    fn ordinal_must_be_resolved() {
        assert_eq!(
            combine_independent(&[Credence::Ordinal(1)]),
            Err(EpError::OrdinalNeedsScale)
        );
        assert_eq!(combine_independent(&[]), Err(EpError::Empty));
    }

    #[test]
    fn conflict_policies() {
        let (a, b) = (pt(0.9), pt(0.3));
        let h = combine_conflicting(&a, &b, ConflictPolicy::Hull).unwrap();
        assert_eq!(h.value, Credence::Interval { lo: 0.3, hi: 0.9 });
        let m = combine_conflicting(&a, &b, ConflictPolicy::Mean).unwrap();
        let Credence::Point(p) = m.value else {
            panic!()
        };
        assert!((p - 0.6).abs() < 1e-12);
        let u = combine_conflicting(&a, &Credence::Unknown, ConflictPolicy::Mean).unwrap();
        assert_eq!(u.value, a);
        assert_eq!(u.reasons[0].rule, "credence.conflict.one-unknown");
        let uu = combine_conflicting(&Credence::Unknown, &Credence::Unknown, ConflictPolicy::Hull)
            .unwrap();
        assert_eq!(uu.value, Credence::Unknown);
    }

    fn scale() -> LevelScale {
        LevelScale::new(["low", "mid", "high", "certain"].map(String::from).to_vec()).unwrap()
    }

    #[test]
    fn scale_round_trips_through_intervals() {
        let s = scale();
        assert_eq!(
            LevelScale::new(vec!["x".into()]),
            Err(EpError::ScaleTooSmall(1))
        );
        for l in 0..s.len() {
            let c = s.from_level(l).unwrap();
            assert_eq!(s.to_level(&c).unwrap(), l, "level {l} -> {c:?}");
        }
        assert_eq!(s.to_level(&pt(0.0)).unwrap(), 0);
        assert_eq!(s.to_level(&pt(0.25)).unwrap(), 1); // boundary goes up
        assert_eq!(s.to_level(&pt(1.0)).unwrap(), 3); // top includes 1.0
        assert_eq!(
            s.to_level(&Credence::interval(0.1, 0.9).unwrap()),
            Err(EpError::StraddlesLevels)
        );
        assert_eq!(s.to_level(&Credence::Unknown), Err(EpError::UnknownValue));
        assert_eq!(
            s.from_level(4),
            Err(EpError::LevelOutOfScale {
                level: 4,
                levels: 4
            })
        );
        assert_eq!(s.name(2), Some("high"));
    }

    #[test]
    fn resolved_ordinals_combine() {
        let s = scale();
        let r = s.resolve(&Credence::Ordinal(1)).unwrap();
        let v = combine_independent(&[r, pt(0.5)]).unwrap();
        assert!(matches!(v.value, Credence::Interval { .. }));
    }

    #[test]
    fn brier_values() {
        let perfect = [(pt(1.0), true), (pt(0.0), false)];
        assert_eq!(brier(&perfect).unwrap(), 0.0);
        let worst = [(pt(0.0), true), (pt(1.0), false)];
        assert_eq!(brier(&worst).unwrap(), 1.0);
        let hedge = [(pt(0.5), true), (pt(0.5), false)];
        assert_eq!(brier(&hedge).unwrap(), 0.25);
        assert_eq!(
            brier(&[(Credence::Unknown, true)]),
            Err(EpError::UnknownValue)
        );
        assert_eq!(
            brier(&[(Credence::interval(0.4, 0.6).unwrap(), true)]),
            Err(EpError::ScoringNeedsPoint)
        );
        assert_eq!(brier(&[]), Err(EpError::Empty));
    }

    #[test]
    fn log_score_values() {
        let s = log_score(&[(pt(0.5), true), (pt(0.5), false)]).unwrap();
        assert!((s - std::f64::consts::LN_2).abs() < 1e-12);
        assert_eq!(log_score(&[(pt(0.0), true)]).unwrap(), f64::INFINITY);
        assert_eq!(log_score(&[(pt(1.0), true)]).unwrap(), 0.0);
    }
}
