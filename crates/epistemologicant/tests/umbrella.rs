//! The umbrella re-exports the real tools, and the shared types are one type across all of them.
#![cfg(all(
    feature = "credence",
    feature = "defeat",
    feature = "evidence",
    feature = "justify",
    feature = "record"
))]
use epistemologicant::{EpError, Verdict, credence, defeat, evidence, justify, record};

#[test]
fn every_tool_is_reachable_and_shares_one_verdict_type() {
    let c = credence::Credence::point(0.4).unwrap();
    let combined: Verdict<credence::Credence> = credence::combine_independent(&[c, c]).unwrap();

    let mut g = evidence::EvidenceGraph::new();
    g.add_item("a", "s", &[]).unwrap();
    let ind: Verdict<bool> = g.independent(&"a", &"a").unwrap();

    let mut j = justify::Justifications::new();
    j.assert(
        "x",
        justify::Ground::Premise {
            label: "obs".into(),
        },
    );
    let st: Verdict<justify::Status> = j.status(&"x");

    let r = record::SourceRecord::<&str, &str>::new(record::Params::new(0.5, 2.0, 10).unwrap());
    let rel: Verdict<credence::Credence> = r.reliability(&"s", &"t", 0).unwrap();

    let mut d = defeat::DefeasibleSystem::new();
    d.assert("x", &[], 1, "obs");
    let standing: Verdict<defeat::Standing> = d.status(&"x");
    assert!(!standing.reasons.is_empty());

    assert!(!combined.reasons.is_empty() && !ind.reasons.is_empty());
    assert!(!st.reasons.is_empty() && !rel.reasons.is_empty());
    let e: EpError = credence::Credence::point(2.0).unwrap_err();
    assert!(matches!(e, EpError::OutOfRange(_)));
}
