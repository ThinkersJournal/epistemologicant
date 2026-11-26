kind: the argument tool, patent sweep
---
- `docs/PATENT-SWEEP.md`: short "outside as read" patent sweep of the defeater / argument-structure family (5 hits read, none needing review; the other tools are unswept).
- Added `epistemologicant-argument` (the fifth tool's second half, in pm-ants' order): Dung-style `ArgumentationFramework` with a grounded labelling
  (least fixpoint, reasons `argument.in|out|undecided`), complete / preferred / stable extensions by exhaustive enumeration (refused above 20
  arguments with `EpError::TooLarge`), credulous and skeptical acceptance (skeptical under a semantics with no extension is vacuously true and says
  so). Tested against an independent set-based oracle, against the published four-argument example, and against `-defeat` (an attack as an undercut
  gives the same grounded verdict). Golden `golden/argument.txt`, doctest. Dung 1995 was NOT read; definitions are second hand (see crate docs).
  Mutation runs: all 7 caught by named tests; the size-limit tests run the call on a worker thread with a 5 s bound, so removing or shifting the limit FAILS the test (in under 10 s) instead of stalling CI. Over the limit the call returns `EpError::TooLarge` at once (tested for 21, 31, 32 and 100 arguments: never a panic); the exponential cost (2^n subsets) is documented, and 20 arguments measured 1.9 s in a debug build.
- Patent sweep verdict wording is now `outside as read` (a model reading, not legal advice), per the PM (board 480). `-credence`, `-evidence`,
  `-justify`, `-record` remain unswept.
- `epistemologicant` umbrella gained the `argument` feature (default on). `EpError::TooLarge`.
