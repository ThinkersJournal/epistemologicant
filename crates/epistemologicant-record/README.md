# epistemologicant-record

Per-source, per-topic reliability from observed outcomes, shrunk toward a baseline while evidence is thin and recency-weighted so a source can recover.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-record = "0.0"
```

```rust
use epistemologicant_record::*;

// Baseline 50%, worth 4 observations; observations halve in weight every 100 time units.
let mut r = SourceRecord::new(Params::new(0.5, 4.0, 100)?);
assert_eq!(r.reliability(&"wire", &"physics", 0)?.reasons[0].rule, "record.baseline-only");
for t in 0..8 {
    r.observe("wire", "physics", true, t); // a good outcome at time t
}
let v = r.reliability(&"wire", &"physics", 8)?;
assert_eq!(v.reasons[0].rule, "record.shrunk");
# Ok::<(), EpError>(())
```

What counts as a good outcome, the baseline, the prior strength and the half-life are your data. No clock is read: you pass `now`.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
