# Changelog

One version number for the whole project. Pre-1.0, the second number is the breaking-change number.

## 0.0.3 - 2026-11-19
- Added `epistemologicant-justify`: `Justifications` (premise and inference grounds, `assert`, `contradict`, `retract` returning the
  exact `Fallout`, `status` with reasons `justify.held.premise|held.inference|contradicted|unsupported`). Support is a least
  fixpoint from the premises, so cycles cannot hold themselves up and a claim with a second ground survives losing one.
- `EpError` gained `UnknownGround` and `AlreadyRetracted`.
- Golden test `golden/justify.txt`; four rule mutations (any-for-all, retired grounds counted, contradiction ignored, fallout
  narrowed to the retracted claim) each fail tests.

## 0.0.2 - 2026-11-19
- Added `epistemologicant-evidence`: `EvidenceGraph` (items, sources, derivation chains, cycles allowed), `independent`
  (same item, mutual citation, ancestor, shared ancestor, shared source; every verdict carries rule-id reasons),
  `corroborate` (clusters of dependent items, transitive and cautious), `combine_corroborated` (strongest per cluster,
  noisy-OR across clusters, so the same evidence is never counted twice).
- `EpError` gained `UnknownItem` and `DuplicateItem`.
- Golden test `golden/evidence.txt`, compared byte for byte on ubuntu, windows and macos.

## 0.0.1 - 2026-11-18
- Added `epistemologicant-core` (`EpError`; re-exports `Reason`, `Verdict`, `Provenance` from `semanticant-core` 0.1.0) and
  `epistemologicant-credence` (`Credence`, `LevelScale`, `combine_independent`, `combine_conflicting`, `brier`, `log_score`).
- CI (fmt, clippy, tests on three OSs, cargo-deny).
