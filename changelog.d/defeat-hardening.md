kind: the defeat tool, hardening
---
- Added `epistemologicant-defeat` (the fifth tool, defeaters first; hold lifted by pm-ants 2026-11-21): `DefeasibleSystem`
  with ranked reasons, rebutting defeaters (`contradict`) and undercutting defeaters (`undercut`), and a skeptical grounded
  verdict `In`/`Out`/`Undecided` (least fixpoint, independent of insertion order) whose reasons are `defeat.in|out|out.no-reasons|undecided`.
  Equal-rank conflict and cycles of defeat stay undecided; a cycle that only supports itself is out; `retract` reports the claims
  whose standing changed. Rank-decides-rebuttal is this crate's own design choice, not Pollock's (the SEP entry gives no such rule).
  Credit: Pollock 1987 via the SEP entry (paper not read), Dung 1995 via second-hand definitions. Golden `golden/defeat.txt`, property tests
  (never both of a contradictory pair `In`; order independence; In/Out forced by the reasons; equals `-justify` when no defeater exists;
  retraction diff; rank), SageAnt-proposal-shaped fixtures (SageAnt states no defeater requirement). The property test caught a real
  bug (a self-supporting cycle stayed undecided); mutation runs: eight rule mutations; five caught directly, the retired-reason and unreachable-claim rules shield each other (only the combined mutation is caught), and `>` vs `>=` on live rebutters is an equivalent mutant (two equal-rank rebutters are never both live).
- Stated plainly in README too: higher-rank-wins is this project's design choice, and Pollock 1987 / Dung 1995 were read second hand only.
- `epistemologicant` umbrella gained the `defeat` feature (default on). `EpError::UnknownReason`, `EpError::TooLarge`.
- Docs only; no library code changed. Runnable doctests on the front page of every tool crate (run by `cargo test --doc`), a runnable
  example `epistemologicant --example sageant_shape` (SageAnt-proposal-shaped flow, cited to its branch ref), and `docs/USAGE.md`
  (one-page guide: which function answers which question, and which decisions are the caller's).
- ROADMAP/HANDOFF: the fifth-tool hold is a PM ruling relayed in HANDOFF, not CireSnave's words (earlier text mis-attributed it).
