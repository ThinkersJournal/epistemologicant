# First-release readiness checklist (PM task 2026-11-28; EXPECTATIONS 6.24)

| Item | State at this PR |
|---|---|
| Fresh-eyes trial, transcript kept | `docs/FRESH-EYES-TRIAL-2026-11-28.md`: passed |
| `README.md` in each of the 7 sub-crates and the umbrella, each with its own short example | done; `readme = "README.md"` in every manifest |
| README examples compiled in CI | each README is included as a doctest (`#[cfg(doctest)]`), run by `cargo test --workspace` in the `test` job |
| `keywords`, `categories` | set in all 8 manifests (5 keywords, `science` and `algorithms`) |
| Licence files in each crate | `LICENSE-MIT` and `LICENSE-APACHE` already present in every crate directory |
| MSRV | `rust-version = 1.88`; new CI job `msrv (1.88)` runs the tests on 1.88 (also run locally on 1.88: all pass) |
| Yank policy | stated in each README: a release with a wrong result is yanked and replaced by a patch; one that merely lacks something is not |
| Honest-limits section | kept in the root README and repeated, short, in the defeat, argument and umbrella READMEs |
| Publish | NOT done; the PM publishes (8 crates, `--allow-new` for 7 first publishes) after this merges and he has read the trial |

Not checked here: `cargo publish --dry-run` after the README change (the PM's dry run at 0.0.7 predates it); `cargo package --list` shows
a README and both licences for all eight crates.
