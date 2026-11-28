# epistemologicant

One dependency for the EpistemologicAnt tool libraries: credence, evidence, justify, record, defeat and argument, each re-exported under its own module behind a feature of the same name (all on by default).

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant). **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant = "0.0"   # or default-features = false, features = ["record"]
```

```rust
use epistemologicant::{credence::*, evidence::EvidenceGraph, EpError};

let mut g = EvidenceGraph::new();
g.add_item("agency-report", "agency", &[])?;
g.add_item("paper-a", "paper-1", &["agency-report"])?; // repeats the report
g.add_item("survey", "survey-1", &[])?;
// Two items, but are they independent? paper-a derives from the report, the survey does not.
let c = g.corroborate(&["agency-report", "paper-a", "survey"])?.value;
assert_eq!(c.independent_count(), 2); // the report and paper-a count once
let both = combine_independent(&[Credence::point(0.5)?, Credence::point(0.5)?])?;
assert_eq!(both.value, Credence::Point(0.75));
# Ok::<(), EpError>(())
```

The umbrella has no code of its own. Everything is generic over your ids, deterministic, and carries `reasons` (a stable rule id and a note) on every verdict. A usage page is `docs/USAGE.md` in the repository.

**Honest limits.** Independence is a structural test on the derivation you record. The defeat crate's rank rule is this project's own design choice, not Pollock's; Pollock (1987) and Dung (1995) were not read, their definitions were read second hand. A patent sweep (`docs/PATENT-SWEEP.md`) is a model reading, not legal advice.

Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
