# EpistemologicAnt: SPEC-STEP1 (lane draft, 2026-11-18)

Expands `SPEC-SEED-2026-11-18.md`. Markers: **[said]** CireSnave, **[PM]** PM proposal, **[lane]** this lane's judgement.
Nothing is built. Section 9 lists what is unverified.

## 1. Scope and non-goals
In scope **[said/PM]**: tool libraries for the theory of knowledge and its analysis: degrees of belief, evidence and independence,
justification and retraction, source reliability and calibration; later: defeaters, belief revision, coherence.
Non-goals **[lane]**: no model or text understanding in the core (that is SemanticAnt's side); no lenses (AnalysAnt); no
storage, network or UI; no domain policy ("what counts as a good source" is data a caller supplies); no ontology (OntologicAnt).

## 2. Survey (crates.io API, 2026-11-18; docs.rs front pages for two crates)
**Read** (metadata and docs.rs page only, not source): `statskit` 0.4.0 (MIT/Apache-2.0), `furtif-core` 0.1.2 (**LGPL-3.0**, fails our
permissive-only rule). **Seen only in search results or crates.io metadata**: `eigentrust` 0.2.0 (MIT/Apache-2.0, 29 downloads),
`kpopper` 0.18.0 (MIT), `eidos-claims` 0.1.0 (Apache-2.0), `plato-unified-belief` 0.2.0 (MIT), `propagators-chirho` 0.3.0 (MIT),
`bulwark-decision` 0.6.0, `shadow-reputation`, `reputation-core`. Queries run: belief revision, truth maintenance, dempster shafer,
subjective logic, argumentation framework, brier score calibration, bayesian network, imprecise probability, evidence theory,
reputation trust, credence, provenance.
**Finding:** no mature, general, permissively licensed crate exists for any of the four foundation tools. Hits are young
(tens to hundreds of downloads), tied to one product (agent memory, a security engine), or unrelated. Dempster-Shafer's only hit is LGPL.
**Consequence [lane]:** build the four from the literature; the stdlib-only core is feasible. Re-check at each tool's build time
(a crate may appear). `statskit` is a possible dev-dependency cross-check for Brier/log score (verify before use).
**Literature** (named from knowledge, **not read in this survey**; each must be read before its tool ships, and credited in its docs):
Dempster/Shafer (belief functions), Jøsang (subjective logic), Doyle and de Kleer (truth maintenance), Alchourrón-Gärdenfors-Makinson
(AGM revision), Dung (argumentation), Pollock (rebutting/undercutting defeaters), Brier 1950 and Gneiting-Raftery 2007 (scoring rules),
Walley (imprecise probability), Kamvar et al. (EigenTrust). Reuse of any code or table from them keeps licence and credit.

## 3. Confirming the seed's section 7
Four foundation tools confirmed, with corrections **[lane]**:
- Merge the seed's tool 4 split: **calibration scoring** is stateless functions (belongs with Credence); **source record** is stateful.
  Crates below follow that.
- Add to the core a **reason trace**: every verdict returns the facts and rule that produced it. Needed by all four.
- No fifth tool is proposed before the spec is read. Candidates parked: defeater/argument structure, belief revision, coherence.

## 4. Crate layout (one version for the project **[said]**)
`epistemologicant-core` (ids, reason trace, errors; stdlib only) · `-credence` · `-evidence` (graph + independence) ·
`-justify` · `-record`. The `epistemologicant` crate re-exports behind features. All `#![forbid(unsafe_code)]`, `no_std` not a goal.
Core types shared with siblings are the open substrate question (section 8, ASK 1); until settled, core types are crate-private to avoid a second copy.

## 5. API sketches (shape only; every type generic over what the claim, source and topic are)
**Credence.** `enum Credence { Unknown, Point(P), Interval{lo,hi}, Ordinal(Level) }`. `Unknown` is not `Point(0)`.
`combine_independent(&[Credence]) -> Verdict<Credence>`, `combine_conflicting(a,b,policy) -> Verdict<..>`, `to_level`/`from_level` through a
caller-supplied `LevelScale` (data, not hard-coded names). Scoring: `brier(&[(Credence, bool)])`, `log_score(..)`; `Interval` inputs are rejected or
scored by explicit policy, never silently collapsed.
**Evidence graph.** `EvidenceGraph<S,E>`: `add_item(id, source: S, derived_from: &[Id])`, `independent(a,b) -> Verdict<bool>` (shared ancestry, mutual
citation, one origin twice), `corroborate(&[Id]) -> Corroboration` where the result lists the independent clusters actually counted and the
reasons a pair was merged. Never counts the same origin twice; a source cannot corroborate itself through sources that only cite it.
**Justification.** `Justifications<C>`: `assert(claim, Ground)`, `retract(ground) -> Fallout` (exact set that lost support), `status(claim) -> Status{Held, Unsupported, Contradicted}`
with the grounds; handles cycles and multiple supports (the classic TMS cases, tested explicitly).
**Source record.** `SourceRecord<S,T>`: `observe(source, topic, outcome, at)`, `reliability(source, topic, now) -> Verdict<Credence>`, shrunk toward a
caller-set baseline while evidence is thin, recency-weighted by a caller-set half-life (so recovery is possible). Topic hierarchy is data.
`Verdict<T> = { value: T, reasons: Vec<Reason> }` everywhere.
Determinism **[lane]**: no clock reads (callers pass `now`), no RNG, iteration order fixed, so equal input gives byte-equal output.

## 6. Consumers and timing
**SageAnt (first consumer, X3)** needs: per-source, per-topic reliability with shrinkage and recovery; corroboration that refuses self-citing
clusters; an ordinal confidence level. Needs it when SageAnt reaches X3 **[lane: date unknown; the PM holds the roadmap, so [ASK] 3]**. These are test cases here (the
SageAnt scenarios become fixtures), never API shape **[said, 2026-11-18]**.
**AnalysAnt (`analysant-epistemic`)** later needs defeaters and argument structure: parked, after the four.

## 7. Build plan
Spec PR (this) → foundation tools, **one PR per day** **[said]**, order: core+credence, evidence, justify, record. Each: red-first tests, a
sabotage check per rule (mutate the rule, watch a test fail), property tests for combination laws, a README claiming only what is tested.
Version is allocated by the PM at gate time. CI and branch protection per the portfolio rule before the first code PR.

## 8. Decisions that need CireSnave or the PM
- **[ASK 1, PM]** Shared substrate (ids, provenance, reason trace, data packs). Recommend: this project ships `Reason`/`Verdict` in core now, and moves them to the
  shared crate if the PM settles one; reason: blocking on three specs stalls the first PR.
- **[ASK 2, CireSnave]** Numeric representation for `Point`: `f64` (simple, not bit-stable across platforms for transcendental ops) vs exact rational (slower, reproducible). Recommend `f64` with documented rounding and no `exp/ln` in combination rules; `log_score` is the exception and is flagged.
- **[ASK 3, PM]** When does SageAnt need X3? Sets the order if it is earlier than one PR per day allows.
- **[ASK 4, CireSnave]** Fifth tool after the four: defeaters/argument structure (for AnalysAnt) vs belief revision. Recommend defeaters.

## 9. Unverified
Everything in section 2 beyond metadata (no crate source read; no paper read); that the stdlib-only core suffices; that `statskit`'s scores match
the literature; all API shapes (sketches, not compiled); SageAnt's actual X3 needs (taken from the seed, not from SageAnt's design note).
