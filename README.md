# EpistemologicAnt

Comprehensive tool libraries covering the whole field of epistemology and epistemological analysis.

**Status: early. Four crates exist and are tested; nothing is published beyond the 0.0.0 placeholder.** The spec is `docs/SPEC-STEP1.md`.

| Crate | What is built |
|---|---|
| `epistemologicant-core` | `EpError`; re-exports `Reason`, `Verdict`, `Provenance` unchanged from `semanticant-core` 0.1.0 |
| `epistemologicant-credence` | `Credence` (`Unknown` is not zero; point; interval; ordinal through a caller-supplied `LevelScale`), `combine_independent` (noisy-OR, exact on intervals), `combine_conflicting` (hull or mean), `brier`, `log_score` |
| `epistemologicant-evidence` | `EvidenceGraph`: items with a source and derivation chain; `independent` (same item, mutual citation, ancestor, shared ancestor, shared source; reasons on every verdict); `corroborate` (clusters of dependent items); `combine_corroborated` (never counts the same evidence twice) |
| `epistemologicant-justify` | `Justifications`: premise and inference grounds, `retract` returns the exact set that lost support, `status` (`Held`/`Unsupported`/`Contradicted`) with reasons; cycles cannot support themselves, a second ground survives a retraction |

Determinism: the combination rules and `brier` use only `+ - * /` and `min`/`max`, and the evidence graph uses ordered sets and no floating point of its own; a golden test per crate compares output with ONE committed file, bit for bit, and CI runs it on ubuntu, windows and macos. `log_score` uses `ln` and is excluded from that claim. Independence of evidence is the caller's claim
in `combine_independent`; `epistemologicant-evidence` is what checks it.

Part of the field-library layer of the AnalysAnt family: these libraries are toolsets that AnalysAnt's lenses (and other projects, such as
SageAnt) use as dependencies. The standing rule (CireSnave, 2026-11-18): if a project needs a tool that falls in this field, the tool is
built here and used as a dependency by the project that needs it.

Licence: MIT OR Apache-2.0.
