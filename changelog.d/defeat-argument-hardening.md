kind: the defeat and argument tools, a patent sweep, hardening
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
- `docs/PATENT-SWEEP.md`: short "outside as read" patent sweep of the defeater / argument-structure family (5 hits read, none needing review; the other tools are unswept).
- Stated plainly in README too: higher-rank-wins is this project's design choice, and Pollock 1987 / Dung 1995 were read second hand only.
- Added `epistemologicant-argument` (the fifth tool's second half, in pm-ants' order): Dung-style `ArgumentationFramework` with a grounded labelling
  (least fixpoint, reasons `argument.in|out|undecided`), complete / preferred / stable extensions by exhaustive enumeration (refused above 20
  arguments with `EpError::TooLarge`), credulous and skeptical acceptance (skeptical under a semantics with no extension is vacuously true and says
  so). Tested against an independent set-based oracle, against the published four-argument example, and against `-defeat` (an attack as an undercut
  gives the same grounded verdict). Golden `golden/argument.txt`, doctest. Dung 1995 was NOT read; definitions are second hand (see crate docs).
  Mutation runs: all 7 caught by named tests; the size-limit tests run the call on a worker thread with a 5 s bound, so removing or shifting the limit FAILS the test (in under 10 s) instead of stalling CI. Over the limit the call returns `EpError::TooLarge` at once (tested for 21, 31, 32 and 100 arguments: never a panic); the exponential cost (2^n subsets) is documented, and 20 arguments measured 1.9 s in a debug build.
- Patent sweep verdict wording is now `outside as read` (a model reading, not legal advice), per the PM (board 480). `-credence`, `-evidence`,
  `-justify`, `-record` remain unswept.
- `epistemologicant` umbrella gained the `defeat` and `argument` features (default on). `EpError::UnknownReason`, `EpError::TooLarge`.
- Docs only; no library code changed. Runnable doctests on the front page of every tool crate (run by `cargo test --doc`), a runnable
  example `epistemologicant --example sageant_shape` (SageAnt-proposal-shaped flow, cited to its branch ref), and `docs/USAGE.md`
  (one-page guide: which function answers which question, and which decisions are the caller's).
- ROADMAP/HANDOFF: the fifth-tool hold is a PM ruling relayed in HANDOFF, not CireSnave's words (earlier text mis-attributed it).
