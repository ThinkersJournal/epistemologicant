# epistemologicant-justify

Justification tracking: claims with grounds, retraction that reports exactly what lost support, and a status with reasons.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-justify = "0.0"
```

```rust
use epistemologicant_justify::*;

let mut j = Justifications::new();
let obs = j.assert("wet-grass", Ground::Premise { label: "saw it".into() });
j.assert("it-rained", Ground::Inference { antecedents: vec!["wet-grass"] });
assert_eq!(j.status(&"it-rained").value, Status::Held);
let fallout = j.retract(obs)?; // withdraw the observation
assert_eq!(fallout.lost, vec!["it-rained", "wet-grass"]);
assert_eq!(j.status(&"it-rained").value, Status::Unsupported);
# Ok::<(), EpError>(())
```

Grounds are monotone (no "out" antecedents). Contradiction is reported, never resolved by picking a side; same-source contradiction is not distinguished.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
