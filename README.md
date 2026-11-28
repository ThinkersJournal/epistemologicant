# EpistemologicAnt

Comprehensive tool libraries covering the whole field of epistemology and epistemological analysis.

**Status: early. Eight crates exist and are tested; nothing is published beyond the 0.0.0 placeholder.** The spec is `docs/SPEC-STEP1.md`.

## Quick start

```toml
[dependencies]
epistemologicant = "0.0"
```

```rust
use epistemologicant::{credence::*, evidence::EvidenceGraph, EpError};

fn main() -> Result<(), EpError> {
    let mut g = EvidenceGraph::new();
    g.add_item("agency-report", "agency", &[])?;
    g.add_item("paper-a", "paper-1", &["agency-report"])?; // repeats the report
    g.add_item("survey", "survey-1", &[])?;
    let c = g.corroborate(&["agency-report", "paper-a", "survey"])?.value;
    println!("independent pieces of evidence: {}", c.independent_count()); // prints 2, not 3
    let both = combine_independent(&[Credence::point(0.5)?, Credence::point(0.5)?])?;
    println!("{:?}", both.value); // Point(0.75)
    Ok(())
}
```

Every verdict is a `Verdict`: `.value` is the answer and `.reasons` says why (a stable `rule` id and a `note`). In `add_item(id, source, cited_ids)` the second argument names who said it and the third the items it derives from. Edition 2024, Rust 1.88 or newer.

| Crate | What is built |
|---|---|
| `epistemologicant-core` | `EpError`; re-exports `Reason`, `Verdict`, `Provenance` unchanged from `semanticant-core` 0.1.0 |
| `epistemologicant-credence` | `Credence` (`Unknown` is not zero; point; interval; ordinal through a caller-supplied `LevelScale`), `combine_independent` (noisy-OR, exact on intervals), `combine_conflicting` (hull or mean), `brier`, `log_score` |
| `epistemologicant-evidence` | `EvidenceGraph`: items with a source and derivation chain; `independent` (same item, mutual citation, ancestor, shared ancestor, shared source; reasons on every verdict); `corroborate` (clusters of dependent items); `remove_item` / `remove_item_and_dependents` (forget a claim; a plain remove leaves its dependents citing an opaque origin); `combine_corroborated` (never counts the same evidence twice) |
| `epistemologicant-justify` | `Justifications`: premise and inference grounds, `retract` returns the exact set that lost support, `status` (`Held`/`Unsupported`/`Contradicted`) with reasons; cycles cannot support themselves, a second ground survives a retraction |
| `epistemologicant-record` | `SourceRecord`: per-source, per-topic reliability from observed outcomes; shrunk toward a caller-set baseline while evidence is thin; recency weight (half-life, no `exp`) so a source can recover; a topic pools its sub-topics; reasons on every estimate |
| `epistemologicant-defeat` | `DefeasibleSystem`: reasons with ranks, rebutting defeaters (`contradict`) and undercutting defeaters (`undercut`), a skeptical grounded verdict (`In`/`Out`/`Undecided`) with reasons; equal-rank conflict and defeat cycles stay undecided; `retract` reports the standings that changed |
| `epistemologicant-argument` | `ArgumentationFramework` (Dung-style): arguments and an attack relation; grounded labelling with a reason per argument; complete, preferred and stable extensions (exhaustive, refused above 20 arguments); credulous and skeptical acceptance; checked against a set-based oracle and against `-defeat` |
| `epistemologicant` | umbrella: one dependency; re-exports the four tools behind features of the same name; no code of its own |

**Honest limits of `-defeat`.** "A higher rank beats a lower one in a rebuttal" is this project's own design choice, not Pollock's: the
Stanford Encyclopedia entry read gives no such rule (the nearest idea read is value-based argumentation). Pollock 1987 and Dung 1995 were
**not** read; their definitions were read second hand (SEP "Defeasible Reasoning", Wikipedia "Argumentation framework"). A patent sweep
of the argument-structure and defeater family is in `docs/PATENT-SWEEP.md` (verdicts are `outside as read`, a model reading, not legal advice); `-credence`, `-evidence`, `-justify` and `-record` have a second-pass sweep in the same file (one query each, claim 1 read; board 480; publish hold lifted by CireSnave 2026-11-27, board 522 A, with a watch list in `ROADMAP.md`). `-argument` follows Dung (1995), read second hand only.

Start with `docs/USAGE.md` (one page) and `cargo run -p epistemologicant --example sageant_shape`.

Determinism: the combination rules and `brier` use only `+ - * /` and `min`/`max`, and the evidence graph uses ordered sets and no floating point of its own; a golden test per crate compares output with ONE committed file, bit for bit, and CI runs it on ubuntu, windows and macos. `log_score` uses `ln` and is excluded from that claim. `-justify` marks any two supported claims contradictory when told to; same-source contradiction is not distinguished, and the consumer decides whether it is a revision or a conflict. Independence of evidence is the caller's claim
in `combine_independent`; `epistemologicant-evidence` is what checks it.

Part of the field-library layer of the AnalysAnt family: these libraries are toolsets that AnalysAnt's lenses (and other projects, such as
SageAnt) use as dependencies. The standing rule (CireSnave, 2026-11-18): if a project needs a tool that falls in this field, the tool is
built here and used as a dependency by the project that needs it.

Licence: MIT OR Apache-2.0.
