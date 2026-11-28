# epistemologicant-evidence

An evidence graph: sources, derivation chains, an independence test, and corroboration that never counts the same origin twice.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-evidence = "0.0"
```

```rust
use epistemologicant_evidence::*;

let mut g = EvidenceGraph::new();
g.add_item("agency-report", "agency", &[])?;
g.add_item("paper-a", "paper-1", &["agency-report"])?; // both papers repeat the report
g.add_item("paper-b", "paper-2", &["agency-report"])?;
g.add_item("survey", "survey-1", &[])?;
let v = g.independent(&"paper-a", &"paper-b")?;
assert!(!v.value);
assert_eq!(v.reasons[0].rule, "evidence.dependent.shared-ancestor");
assert_eq!(g.corroborate(&["paper-a", "paper-b", "survey"])?.value.independent_count(), 2);
# Ok::<(), EpError>(())
```

Independence here is a structural test on the derivation chains you record; it cannot see dependence you did not record.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
