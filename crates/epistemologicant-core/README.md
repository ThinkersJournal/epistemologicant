# epistemologicant-core

Shared types for EpistemologicAnt: `EpError`, plus `Reason`, `Verdict` and `Provenance` re-exported unchanged from `semanticant-core`.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-core = "0.0"
```

```rust
use epistemologicant_core::{EpError, Reason, Verdict};

fn main() {
    let e = EpError::OutOfRange(1.5);
    println!("{e}"); // every error names the offending value
    let _unused: Option<(Verdict<bool>, Reason)> = None; // the types every tool's verdicts use
}
```

Most users want the `epistemologicant` umbrella crate instead; this crate exists so every tool shares one error and verdict type.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
