# HANDOFF: epistemologicant (updated 2026-11-21)

Lane for `C:\Projects\epistemologicant`. Read `C:\Projects\CLAUDE.md`, then `docs/SPEC-STEP1.md`, `ROADMAP.md`. Sonnet only. Primary stays on `main`; work in worktrees (`..\epistemologicant-wt-<topic>`).
Reporting: [ASK]/[STATUS]/[FINDING] to pm-ants; [READY] to pm-assist cc pm-ants. No push to a branch under a gate without telling pm-assist.

## State (2026-11-21)
Merged #1-#5; `main` = 0.0.4. pm-ants lifted the fifth-tool hold 2026-11-21 (a PM sequencing ruling, NOT CireSnave's words; board 479 withdrawn) and TASKed: defeaters first, then argument structure; generic (AnalysAnt is a test case); ONE PR per UTC day; version via pm-assist, NO lane bump (the workspace stays 0.0.4; CHANGELOG has an "Unreleased" heading). Belief revision stays unscheduled.
Reporting: [ASK]/[STATUS]/[FINDING] to pm-ants; [READY] to pm-assist cc pm-ants. No push to a branch under a gate without telling pm-assist.

## Branch `next` (worktree `..\epistemologicant-wt-next`, off a3c9f53), pushed, NO PR yet
Hardening (doctests, `--example sageant_shape`, docs/USAGE.md, hold re-attribution) + `epistemologicant-defeat` + `epistemologicant-argument` (Dung-style; set-oracle + -defeat cross-check; golden; doctest) + umbrella features + docs/PATENT-SWEEP.md (verdicts reworded to `outside as read` per PM, board 480). The portfolio fragment kit is installed (.github/workflows/changelog-fragment.yml, tools/check-fragment.py, tools/stamp-version.py, changelog.d/README.md): a PR touches NO CHANGELOG.md / version / Cargo.lock edits by hand; its change is ONE fragment, changelog.d/defeat-argument-hardening.md (CHANGELOG.md here equals main's). pm-assist stamps the number. crates.io name check 2026-11-21: all 7 new/other epistemologicant-* names 404 (free), `epistemologicant` is our 0.0.0 placeholder; positive control serde 200. All gates clean locally. Open ONE PR on 2026-11-22 UTC or later (first `git fetch`; rebase if main moved).
PR BODY MUST SAY: `-credence`, `-evidence`, `-justify`, `-record` are UNSWEPT (owed before any publish); no publish until CireSnave has read board 480; version is pm-assist's (no lane bump); higher-rank-wins is a design choice; Pollock 1987 and Dung 1995 read second hand only.
Rulings: f64, no exp/ln in rules; no publish (board 480). Lesson: proptest `prop_assume!` can abort on CI seeds: build inputs, run with PROPTEST_CASES=512+ over many runs. Windows: a stopped mutation run can leave a test .exe running and lock the linker (LNK1104): kill it (Get-Process epistemologicant_*).

## Next
1. 11-22 UTC: open the PR (body per above), [READY] to pm-assist cc pm-ants, watch CI.
2. Dated row (owed before any publish): patent sweep of credence, evidence, justify, record (process: C:\Projects\PATENT-SWEEP-PROCESS.md; template: docs/PATENT-SWEEP.md here; tools/patent-claims.py), [FINDING] to pm-ants with the path.
3. Belief revision stays unscheduled; no row without a consumer.

## Holds / WAITs
WAIT: UTC-day gate for the PR | check: gh pr list --repo ThinkersJournal/epistemologicant --state all --json number,createdAt | wake: 2026-11-22T00:05:00Z
