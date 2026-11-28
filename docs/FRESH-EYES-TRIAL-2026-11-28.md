# Fresh-eyes README trial, 2026-11-28 (EXPECTATIONS 6.24)

Method: a Sonnet subagent (`model: sonnet`) was given ONLY `README.md` at the PR head's parent state (Quick start, before the three
clarifying lines added after the trial) and told not to read other repo files or search the web. One permitted deviation: a path
dependency in place of `epistemologicant = "0.0"`, because nothing is published yet. Worked in an empty `cargo new` project outside the repo.

Transcript (verbatim from the agent's report):

    $ cargo new demo
    $ Cargo.toml [dependencies]: epistemologicant = { path = ".../crates/epistemologicant" }
    $ src/main.rs = README Quick start verbatim; cargo run
       Compiling semanticant-core v0.1.2 ... epistemologicant v0.0.7, demo v0.1.0
        Finished `dev` profile in 8.29s
    independent pieces of evidence: 2
    Point(0.75)
    $ extension from README alone: three 0.5 credences through combine_independent
    Point(0.875)          (= 1 - 0.5^3, noisy-OR)

Verdict: reached the result from the README alone, yes. No compile errors, no edits needed.

Gaps the agent named, and what was done:
- the wrapper type with `.value` and its `.reasons` was never named: one sentence added under the Quick start;
- meaning of `add_item`'s second and third arguments was only inferable: stated in the same sentence;
- no edition or MSRV in the Quick start: stated (edition 2024, Rust 1.88 or newer);
- `= "0.0"` does not resolve until the first publish (the README already says nothing is published beyond the placeholder).

Limits: one trial, one agent, the root README only; the seven per-crate READMEs are compiled as doctests, not read cold.
