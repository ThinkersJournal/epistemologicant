# HANDOFF: epistemologicant (updated 2026-11-18)

Lane for `C:\Projects\epistemologicant`. Read `C:\Projects\CLAUDE.md`, then `docs/SPEC-STEP1.md`. Sonnet only. Primary stays on `main`; work in worktrees (`..\epistemologicant-wt-<topic>`).

## State
Spec PR #1 merged (d5807e4). PM rulings (2026-11-18): `Reason`/`Verdict`/`Provenance` live in `semanticant-core` (crates.io once published, after 11-19);
until then a crate-private shim in `-core`. `f64`, no exp/ln in combination rules (`log_score` flagged), golden-output test BYTE-IDENTICAL on ubuntu/windows/macos CI.
Fifth tool: not now.

## Next (unblocked)
One PR per day, red-first tests plus sabotage: core+credence PR opened 11-18 (second of the bootstrap pair; version 0.0.1), then evidence 11-19, justify 11-20, record 11-21 (one PR per UTC day, counted by day opened).
SageAnt needs `-evidence` + `-record` by ~11-22; its design note (general capabilities) comes via the PM ~11-20. CI + branch protection before the first code PR.

## Holds / WAITs
None.
