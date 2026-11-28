# epistemologicant-defeat

Defeasible reasoning: reasons, rebutting and undercutting defeaters, ranked conflict, and a skeptical verdict with reasons.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-defeat = "0.0"
```

```rust
use epistemologicant_defeat::*;

let mut s = DefeasibleSystem::new();
s.assert("looks-red", &[], 1, "perception");
let red = s.assert("red", &["looks-red"], 1, "it looks red");
assert_eq!(s.status(&"red").value, Standing::In);
s.assert("red-light", &[], 1, "lighting report");
s.undercut(red, "red-light")?; // cuts the reason, does not say "not red"
assert_eq!(s.status(&"red").value, Standing::Out);
s.assert("not-red", &[], 5, "spectrometer");
s.contradict("red", "not-red"); // a rebuttal: the stronger rank wins
assert_eq!(s.status(&"not-red").value, Standing::In);
# Ok::<(), EpError>(())
```

**Honest limits.** "A higher rank beats a lower one in a rebuttal" is this project's own design choice, not Pollock's. Pollock (1987) was not read; its definitions were read second hand. See the repository's `docs/PATENT-SWEEP.md`.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
