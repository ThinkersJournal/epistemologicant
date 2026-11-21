# HANDOFF: epistemologicant (updated 2026-11-21)

Lane for `C:\Projects\epistemologicant`. Read `C:\Projects\CLAUDE.md`, then `docs/SPEC-STEP1.md`, `ROADMAP.md`. Sonnet only. Primary stays on `main`; work in worktrees (`..\epistemologicant-wt-<topic>`).
Reporting: [ASK]/[STATUS]/[FINDING] to pm-ants; [READY] to pm-assist cc pm-ants. No push to a branch under a gate without telling pm-assist.

## State (2026-11-21)
Merged #1-#5; `main` = 0.0.4. pm-ants lifted the fifth-tool hold 2026-11-21 (a PM sequencing ruling, NOT CireSnave's words; board 479 withdrawn) and TASKed: defeaters first, then argument structure; generic (AnalysAnt is a test case); ONE PR per UTC day; version via pm-assist, NO lane bump (the workspace stays 0.0.4; CHANGELOG has an "Unreleased" heading). Belief revision stays unscheduled.
Reporting: [ASK]/[STATUS]/[FINDING] to pm-ants; [READY] to pm-assist cc pm-ants. No push to a branch under a gate without telling pm-assist.

## Branch `next` (worktree `..\epistemologicant-wt-next`, off a3c9f53), pushed, NO PR yet
Hardening (doctests, `--example sageant_shape`, docs/USAGE.md, hold re-attribution) + NEW `epistemologicant-defeat` (defeaters: rebut/undercut, ranks, grounded verdict; golden, property tests incl. cross-check with -justify, SageAnt-shaped fixtures, doctest, umbrella `defeat` feature). All gates clean locally. Open ONE PR on 2026-11-22 UTC or later (first `git fetch`; rebase if main moved).
Rulings: f64, no exp/ln in rules; Lesson: proptest `prop_assume!` can abort on CI seeds: build inputs, run with PROPTEST_CASES=512+ over many runs.

## Next
1. 11-22 UTC: open the PR, [READY] to pm-assist cc pm-ants (say the version is pm-assist's to allocate), watch CI.
2. After merge: argument structure (Dung-style grounded/preferred over an attack relation) as the next row, [STATUS] to pm-ants at row start. Read Dung 1995 if reachable; credit it either way.

## Holds / WAITs
WAIT: UTC-day gate for the PR | check: gh pr list --repo ThinkersJournal/epistemologicant --state all --json number,createdAt | wake: 2026-11-22T00:05:00Z
