# Changelog

One version number for the whole project. Pre-1.0, the second number is the breaking-change number.

## 0.0.4 - 2026-11-21
- Property tests (`proptest`, dev-dependency only; no library code changed) for the laws each tool promises, in `tests/laws.rs` of
  `-credence` (noisy-OR bounded, never lowers an input, order-free, unknown adds nothing bit for bit, certainty absorbs; hull exactly
  commutative and idempotent; mean exact; Brier is the mean of squared errors, bounded, zero only when perfect; level round trip),
  `-justify` (fallout is exactly the support difference, retraction never adds support, retracting equals never asserting, order of
  retractions irrelevant, support is well-founded), `-record` (estimate in [0,1], monotone in good/bad outcomes, bad history recovers,
  sources do not leak) and `-evidence` (independence symmetric and irreflexive, same-source items never independent, corroboration
  partitions its input, removal consistent). Ten rule mutations were each caught by a property; four of them were first missed (Brier divisor, mean upper bound,
  retired grounds counted, shared source ignored) and the properties were strengthened until they were caught.
- `LICENSE-APACHE` (the root file and the six crate copies) replaced with the canonical Apache-2.0 text (blob d645695, as in PersistAnt); ours had altered section 6 ("reasonable and customary use" dropped), a reworded section 9 and a filled-in appendix line. Task from pm-ants.
- Noted by the properties: `Credence::interval(x, x)` stays an `Interval`, while the combinators return a `Point`; bounds are equal.

## 0.0.3 - 2026-11-20
- Added `epistemologicant-justify`: `Justifications` (premise and inference grounds, `assert`, `contradict`, `retract` returning the
  exact `Fallout`, `status` with reasons `justify.held.premise|held.inference|contradicted|unsupported`). Support is a least
  fixpoint from the premises, so cycles cannot hold themselves up and a claim with a second ground survives losing one.
- Added `epistemologicant-record` (same PR, one version): `SourceRecord` / `Params` (baseline, prior strength, half-life, all caller data),
  `observe`, `set_parent` (topic hierarchy; cycles refused), `reliability(source, topic, now)` returning a `Credence::Point` with
  reasons `record.baseline-only|record.shrunk`. Shrunk toward the baseline while evidence is thin; recency weight halves at each
  half-life using only `+ - * /` (no `exp`), so failures fade and a source can recover; a topic pools its sub-topics; a future-dated
  observation is an error. Golden `golden/record.txt`; six rule mutations each fail tests.
- Fixtures shaped by SageAnt's proposed inputs (SageAnt branch `work/phase1-design` @ f2ff0d6, design note 4.5; a proposal): corroboration cases (a)-(d) plus a control in `-evidence` tests, a shrinkage/long-record case in `-record`; (e)-(h) listed as NOT COVERED in the test file.
- `-evidence`: `remove_item` (forget one item; dependents stay and the id becomes an opaque origin, so shared-ancestor dependence is never lost but the forgotten source stops counting), `remove_item_and_dependents` (cycle-safe cascade), `dependents_of`. Decided with pm-ants for SageAnt's forget/purge.
- Added the `epistemologicant` umbrella crate: re-exports `credence`, `evidence`, `justify`, `record` behind same-named features (all default; `evidence` and `record` imply `credence`) and the shared types at the top level. A test checks every tool is reachable and uses one `Verdict`; the no-default-features build is checked, and un-gating a re-export breaks it.
- Added `ROADMAP.md` (rows incl. the held fifth tool).
- CI: advisory `portfolio` job calling CodeRipper's portfolio workflow (same SHA pin as AlarmAnt).
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
