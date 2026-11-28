# epistemologicant-argument

Abstract argumentation in the style of Dung: arguments and an attack relation, grounded, complete, preferred and stable semantics, and reasons for the grounded verdict.

Part of [EpistemologicAnt](https://github.com/ThinkersJournal/epistemologicant), the epistemology tool libraries. **Status: early (0.0.x); pre-1.0 the second version number is the breaking-change number.**

```toml
epistemologicant-argument = "0.0"
```

```rust
use epistemologicant_argument::*;

let mut f = ArgumentationFramework::new();
for a in ["a", "b", "c"] {
    f.add_argument(a);
}
f.add_attack("b", "a")?; // b attacks a
f.add_attack("c", "b")?; // c attacks b, so c defends a
assert_eq!(f.grounded()["a"], Label::In); // reinstated
assert_eq!(f.explain(&"b")?.reasons[0].rule, "argument.out");
assert_eq!(f.extensions(Semantics::Preferred)?.len(), 1);
# Ok::<(), EpError>(())
```

**Honest limits.** Dung (1995) was not read; the definitions were read second hand. Complete, preferred and stable extensions are exhaustive and refused above 20 arguments.

Deterministic: ordered collections, no clock, no RNG. Rust 1.88 or newer. Yank policy: a release with a wrong result is yanked and replaced by a patch version; a release that merely lacks something is not.

Licence: MIT OR Apache-2.0.
