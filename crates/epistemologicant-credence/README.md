# epistemologicant-credence

An uncertainty value with an honest `Unknown`, combination rules and scoring rules.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-credence = "0.0"
```

```rust
use epistemologicant_credence::*;

let v = combine_independent(&[Credence::point(0.5)?, Credence::point(0.5)?])?;
assert_eq!(v.value, Credence::Point(0.75)); // noisy-OR
assert_eq!(v.reasons[0].rule, "credence.independent.noisy-or");
// "No evidence" is not "evidence it is false".
assert_ne!(Credence::Unknown, Credence::point(0.0)?);
let scale = LevelScale::new(vec!["low".into(), "medium".into(), "high".into()])?;
assert_eq!(scale.to_level(&Credence::point(0.9)?)?, 2);
# Ok::<(), EpError>(())
```

Independence is the caller's claim in `combine_independent`; `epistemologicant-evidence` checks it. `log_score` uses `ln` and is not bit-stable across platforms.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
