# HANDOFF: epistemologicant (updated 2026-11-18)

Lane for `C:\Projects\epistemologicant`. Read `C:\Projects\CLAUDE.md`, then `docs/SPEC-STEP1.md`. Sonnet only. Primary stays on `main`; work in worktrees (`..\epistemologicant-wt-<topic>`).

## State (2026-11-19)
Merged: spec #1, core+credence #2 (0.0.1; all 4 CI checks green, golden compared to one committed file on 3 OSs). Open: `-evidence` PR from branch `evidence` (worktree `..\epistemologicant-wt-evidence`, 0.0.2).
PM rulings: depend on `semanticant-core = "0.1.0"` (done in #2); f64, no exp/ln in rules; one PR per UTC day counted by day opened; version lane-carried (workspace + path pins + lock + CHANGELOG); fifth tool not now.

## Next (unblocked)
justify 11-20, record 11-21 (SageAnt needs `-evidence` + `-record` by ~11-22; its design note of general capabilities comes via the PM). Each: golden file + sabotage run, CHANGELOG, README row.

## Holds / WAITs
WAIT: evidence PR merge | check: gh pr view --repo ThinkersJournal/epistemologicant evidence --json state --jq .state | wake: 2026-11-19T02:00:00Z
