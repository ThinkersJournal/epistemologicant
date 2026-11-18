//! Shared types for EpistemologicAnt.
//!
//! [`Reason`], [`Verdict`] and [`Provenance`] come from `semanticant-core` (the portfolio's shared
//! substrate) and are re-exported unchanged; nothing is copied. This crate adds only [`EpError`].
#![forbid(unsafe_code)]

use std::fmt;

pub use semanticant_core::{Provenance, Reason, Verdict};

/// Why an epistemic operation refused its input. Every variant names the offending value.
#[derive(Debug, Clone, PartialEq)]
pub enum EpError {
    /// A probability outside `[0, 1]`, or NaN.
    OutOfRange(f64),
    /// An interval with `lo > hi`.
    BadInterval {
        /// Lower bound given.
        lo: f64,
        /// Upper bound given.
        hi: f64,
    },
    /// An ordinal credence was given where a number is needed; resolve it with a scale first.
    OrdinalNeedsScale,
    /// A scale with fewer than two levels.
    ScaleTooSmall(usize),
    /// An ordinal level index past the end of its scale.
    LevelOutOfScale {
        /// Level given.
        level: usize,
        /// Number of levels in the scale.
        levels: usize,
    },
    /// An interval that spans more than one level of the scale, so it has no single level.
    StraddlesLevels,
    /// An unknown credence where a value is needed.
    UnknownValue,
    /// A scoring rule given an interval, which it will not silently collapse to a point.
    ScoringNeedsPoint,
    /// No inputs.
    Empty,
}

impl fmt::Display for EpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange(p) => write!(f, "probability out of [0,1] or NaN: {p}"),
            Self::BadInterval { lo, hi } => write!(f, "interval has lo {lo} > hi {hi}"),
            Self::OrdinalNeedsScale => write!(f, "ordinal credence needs a LevelScale to resolve"),
            Self::ScaleTooSmall(n) => write!(f, "a level scale needs at least 2 levels, got {n}"),
            Self::LevelOutOfScale { level, levels } => {
                write!(f, "level {level} is not in a scale of {levels} levels")
            }
            Self::StraddlesLevels => write!(f, "interval spans more than one level"),
            Self::UnknownValue => write!(f, "credence is unknown"),
            Self::ScoringNeedsPoint => write!(f, "scoring needs a point credence, not an interval"),
            Self::Empty => write!(f, "no inputs"),
        }
    }
}

impl std::error::Error for EpError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_name_the_input() {
        assert!(EpError::OutOfRange(1.5).to_string().contains("1.5"));
        assert!(
            EpError::BadInterval { lo: 0.9, hi: 0.1 }
                .to_string()
                .contains("0.9")
        );
    }

    #[test]
    fn verdict_is_the_shared_one() {
        let v = Verdict::new(1).because(Reason::new("t.rule", "why"));
        assert_eq!(v.reasons[0].rule, "t.rule");
    }
}
