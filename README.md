# EpistemologicAnt

Comprehensive tool libraries covering the whole field of epistemology and epistemological analysis.

**Status: early. Two crates exist and are tested; nothing is published beyond the 0.0.0 placeholder.** The spec is `docs/SPEC-STEP1.md`.

| Crate | What is built |
|---|---|
| `epistemologicant-core` | `EpError`; re-exports `Reason`, `Verdict`, `Provenance` unchanged from `semanticant-core` 0.1.0 |
| `epistemologicant-credence` | `Credence` (`Unknown` is not zero; point; interval; ordinal through a caller-supplied `LevelScale`), `combine_independent` (noisy-OR, exact on intervals), `combine_conflicting` (hull or mean), `brier`, `log_score` |

Determinism: the combination rules and `brier` use only `+ - * /` and `min`/`max`; a golden test compares their output bit for bit and CI
runs it on ubuntu, windows and macos. `log_score` uses `ln` and is excluded from that claim. Independence of evidence is the caller's claim
in `combine_independent`; checking it is the evidence-graph crate's job (not built yet).

Part of the field-library layer of the AnalysAnt family: these libraries are toolsets that AnalysAnt's lenses (and other projects, such as
SageAnt) use as dependencies. The standing rule (CireSnave, 2026-11-18): if a project needs a tool that falls in this field, the tool is
built here and used as a dependency by the project that needs it.

Licence: MIT OR Apache-2.0.
