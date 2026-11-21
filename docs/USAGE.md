# Using EpistemologicAnt: one page

Everything is generic over your own ids (`claim`, `source`, `topic` are any `Ord + Clone + Debug` type) and takes the time as an
integer you pass in. Nothing reads a clock, a network or a file. Every verdict carries `reasons` (a stable `rule` id and a `note`)
so you can show why. Pre-1.0: the second version number is the breaking-change number.

```toml
epistemologicant = "0.0"   # all four tools; or default-features = false, features = ["record"]
```

| You want to know | Use | Returns |
|---|---|---|
| how sure, with an honest "unknown" | `credence::Credence` (`Unknown`, `Point`, `Interval`, `Ordinal`) | value; `Unknown` is **not** zero |
| two independent pieces of evidence, combined | `credence::combine_independent` | noisy-OR `1 - prod(1 - p)`; the independence is **your** claim |
| two credences that disagree | `credence::combine_conflicting(a, b, Hull or Mean)` | hull (widens) or mean |
| a name for a number | `credence::LevelScale` (your level names, lowest first) | a level; an interval spanning levels is refused |
| how good were my forecasts | `credence::brier` (`log_score` uses `ln`, not bit-stable across platforms) | score; intervals and unknowns are refused |
| are these two items really independent | `evidence::EvidenceGraph::independent` | `Verdict<bool>`; reasons name the rule: same item, mutual citation, ancestor, shared ancestor, shared source |
| how many independent pieces of evidence | `EvidenceGraph::corroborate` | clusters; each cluster counts once |
| combined credence without double counting | `EvidenceGraph::combine_corroborated` | strongest per cluster, noisy-OR across clusters |
| forget a claim | `remove_item` (dependents stay; the id becomes an opaque origin) or `remove_item_and_dependents` | removed/affected ids |
| what holds up a conclusion; what falls if I withdraw X | `justify::Justifications` (`assert`, `retract` -> `Fallout`, `status`) | `Held` / `Unsupported` / `Contradicted` with the ground |
| a claim with reasons for and against it, some cut or outranked | `defeat::DefeasibleSystem` (`assert` with a rank, `contradict` = rebut, `undercut` a reason, `status`, `retract`) | `In` / `Out` / `Undecided` with the live or dead reasons; skeptical: equal-rank conflict and defeat cycles stay `Undecided` |
| how reliable has this source been on this topic | `record::SourceRecord` (`observe`, `reliability`) | a `Point` pulled toward your baseline while evidence is thin; old outcomes fade so a source can recover |

## Decisions that are yours, not the library's
- **What counts as a good outcome**, the baseline, how many observations it is worth (`prior_strength`) and the half-life: all `record::Params`.
- **Level names** and where a claim sits on them.
- **What a rank means.** `defeat` only compares them: higher beats lower, equal leaves both undecided.
- **Whether a same-source contradiction is a revision or a conflict.** `justify::Justifications::contradict` marks any two supported
  claims contradictory; it does not look at their sources.
- **What "no longer exists" means for a purged claim.** Call `remove_item` or `remove_item_and_dependents`; the graph does not guess.

## Run the whole flow
`cargo run -p epistemologicant --example sageant_shape` walks track record -> reliability -> evidence -> corroboration -> level, in
the shape SageAnt proposed (SageAnt repo, branch `work/phase1-design` @ f2ff0d6, design note 4.5: a proposal, not approved defaults).
Each crate's front page has a short example that `cargo test --doc` runs.

## Guarantees and their limits
Determinism: the combination rules, `brier`, the record and the graph use only `+ - * /`, `min`/`max` and ordered collections, and a
golden test per crate compares output with one committed file on ubuntu, windows and macos. `log_score` is the exception.
Property tests check the laws (bounds, order-independence, monotonicity, exact retraction fallout). They are tests, not proofs.
