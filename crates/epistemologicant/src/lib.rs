//! EpistemologicAnt umbrella crate: one dependency for the foundation tools.
//!
//! Nothing is implemented here; each tool lives in its own crate and is re-exported under a
//! module of the same short name, behind a feature of that name (all on by default):
//! [`credence`], [`evidence`], [`justify`], [`record`]. The shared types ([`Reason`], [`Verdict`],
//! [`Provenance`], [`EpError`]) are always available at the top level and are the same types every
//! tool uses. `evidence` and `record` imply `credence`.
#![forbid(unsafe_code)]

pub use epistemologicant_core::{EpError, Provenance, Reason, Verdict};

#[cfg(feature = "credence")]
pub use epistemologicant_credence as credence;
#[cfg(feature = "evidence")]
pub use epistemologicant_evidence as evidence;
#[cfg(feature = "justify")]
pub use epistemologicant_justify as justify;
#[cfg(feature = "record")]
pub use epistemologicant_record as record;
