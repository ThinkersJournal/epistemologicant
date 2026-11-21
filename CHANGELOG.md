# Changelog

One version number for the whole project. Pre-1.0, the second number is the breaking-change number.

## Unreleased (version allocated by pm-assist at gate, per pm-ants 2026-11-21: no lane bump)
- Added `epistemologicant-defeat` (the fifth tool, defeaters first; hold lifted by pm-ants 2026-11-21): `DefeasibleSystem`
  with ranked reasons, rebutting defeaters (`contradict`) and undercutting defeaters (`undercut`), and a skeptical grounded
  verdict `In`/`Out`/`Undecided` (least fixpoint, independent of insertion order) whose reasons are `defeat.in|out|out.no-reasons|undecided`.
  Equal-rank conflict and cycles of defeat stay undecided; a cycle that only supports itself is out; `retract` reports the claims
  whose standing changed. Rank-decides-rebuttal is this crate's own design choice, not Pollock's (the SEP entry gives no such rule).
  Credit: Pollock 1987 via the SEP entry (paper not read), Dung 1995 via second-hand definitions. Golden `golden/defeat.txt`, property tests
  (never both of a contradictory pair `In`; order independence; In/Out forced by the reasons; equals `-justify` when no defeater exists;
  retraction diff; rank), SageAnt-proposal-shaped fixtures (SageAnt states no defeater requirement). The property test caught a real
  bug (a self-supporting cycle stayed undecided); mutation runs: eight rule mutations; five caught directly, the retired-reason and unreachable-claim rules shield each other (only the combined mutation is caught), and `>` vs `>=` on live rebutters is an equivalent mutant (two equal-rank rebutters are never both live).
- `docs/PATENT-SWEEP.md`: short "outside as read" patent sweep of the defeater / argument-structure family (5 hits read, none needing review; the other tools are unswept).
- Stated plainly in README too: higher-rank-wins is this project's design choice, and Pollock 1987 / Dung 1995 were read second hand only.
- Added `epistemologicant-argument` (the fifth tool's second half, in pm-ants' order): Dung-style `ArgumentationFramework` with a grounded labelling
  (least fixpoint, reasons `argument.in|out|undecided`), complete / preferred / stable extensions by exhaustive enumeration (refused above 20
  arguments with `EpError::TooLarge`), credulous and skeptical acceptance (skeptical under a semantics with no extension is vacuously true and says
  so). Tested against an independent set-based oracle, against the published four-argument example, and against `-defeat` (an attack as an undercut
  gives the same grounded verdict). Golden `golden/argument.txt`, doctest. Dung 1995 was NOT read; definitions are second hand (see crate docs).
  Mutation runs: 6 of 7 caught by named tests; the seventh (enumeration limit removed) makes the oversize test enumerate 2^21 subsets and hang.
- Patent sweep verdict wording is now `outside as read` (a model reading, not legal advice), per the PM (board 480). `-credence`, `-evidence`,
  `-justify`, `-record` remain unswept.
- `epistemologicant` umbrella gained the `defeat` and `argument` features (default on). `EpError::UnknownReason`, `EpError::TooLarge`.
- Docs only; no library code changed. Runnable doctests on the front page of every tool crate (run by `cargo test --doc`), a runnable
  example `epistemologicant --example sageant_shape` (SageAnt-proposal-shaped flow, cited to its branch ref), and `docs/USAGE.md`
  (one-page guide: which function answers which question, and which decisions are the caller's).
- ROADMAP/HANDOFF: the fifth-tool hold is a PM ruling relayed in HANDOFF, not CireSnave's words (earlier text mis-attributed it).

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
