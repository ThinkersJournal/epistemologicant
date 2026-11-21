# HANDOFF: epistemologicant (updated 2026-11-21)

Lane for `C:\Projects\epistemologicant`. Read `C:\Projects\CLAUDE.md`, then `docs/SPEC-STEP1.md`, `ROADMAP.md`. Sonnet only. Primary stays on `main`; work in worktrees (`..\epistemologicant-wt-<topic>`).
Reporting: [ASK]/[STATUS]/[FINDING] to pm-ants; [READY] to pm-assist cc pm-ants. No push to a branch under a gate without telling pm-assist.

## State (2026-11-21)
Merged #1-#5; `main` = 0.0.4 (a3c9f53): core, credence, evidence (+remove_item), justify, record, umbrella `epistemologicant`, property tests, SageAnt-shaped fixtures, canonical LICENSE-APACHE, CI incl. advisory portfolio job. #5 used 11-21 UTC: next PR not before 2026-11-22 UTC. Version lane-carried (workspace + path pins + lock + CHANGELOG).
Rulings: f64, no exp/ln in rules; fifth tool (defeaters first, then belief revision unscheduled) HELD by a PM ruling "not now" (inherited line in HANDOFF at 894abc7; NOT CireSnave's words, I mis-attributed it and told pm-ants 2026-11-21). Board 479 asks CireSnave anyway; lift only on word via pm-ants.
LICENSE-MIT holder notice checked: "Copyright (c) 2026 Eric Evans" is in LICENSE-MIT, one blob in all 7 copies. Closed.
Lesson: proptest `prop_assume!` can abort on CI seeds ("Too many global rejects"); build inputs instead of rejecting; test with PROPTEST_CASES=512 over many runs.

## Next
Branch `next` (worktree `..epistemologicant-wt-next`, off a3c9f53): HARDENING approved by pm-ants: rustdoc with doctests that run, examples, one-page SageAnt usage guide; ONE PR on 2026-11-22 UTC. Fifth tool only after the hold is lifted (via pm-ants).

## Holds / WAITs
(no WAITs: pm-ants answered 2026-11-21: hold stays until CireSnave rules, via PM; hardening on `next` approved, one PR 11-22 UTC)
