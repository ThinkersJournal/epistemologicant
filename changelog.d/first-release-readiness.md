kind: per-crate READMEs, metadata, MSRV job, fresh-eyes trial
---
Added `README.md` to all eight crates, each with its own short example compiled as a doctest by `cargo test --workspace`; added `readme`, `keywords` and `categories` to every manifest; added a CI job `msrv (1.88)`; added a Quick start with an example to the root `README.md`; recorded the fresh-eyes trial in `docs/FRESH-EYES-TRIAL-2026-11-28.md` and the checklist in `docs/RELEASE-READINESS.md`. No code change. Verified: `cargo test --workspace` on stable and on 1.88.
