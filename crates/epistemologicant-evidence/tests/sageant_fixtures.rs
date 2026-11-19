//! Fixtures shaped by SageAnt's proposed inputs: SageAnt repo, branch `work/phase1-design` at f2ff0d6,
//! `docs/DESIGN-WORTH-AND-TRUST.md` section 4.5 (a PROPOSAL on a design branch, not a merged or approved
//! document). These shape test cases only, never this crate's API. Each case in 4.5 item 3 is named by
//! its letter. Cases that are the consumer's business or that this crate cannot yet express are listed
//! at the bottom as `NOT COVERED` so the gap is visible rather than silent.
use epistemologicant_evidence::*;

fn dependent(
    g: &EvidenceGraph<&'static str, &'static str>,
    a: &'static str,
    b: &'static str,
    rule: &str,
) {
    let v = g.independent(&a, &b).unwrap();
    assert!(!v.value, "{a} vs {b} must be dependent");
    assert!(
        v.reasons.iter().any(|r| r.rule == rule),
        "{a} vs {b}: want {rule}, got {:?}",
        v.reasons
            .iter()
            .map(|r| r.rule.to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn case_a_a_source_cannot_corroborate_its_own_claim_even_through_another_claim_it_made() {
    let mut g = EvidenceGraph::new();
    g.add_item("claim", "s1", &[]).unwrap();
    g.add_item("other-claim-by-s1", "s1", &[]).unwrap();
    dependent(
        &g,
        "claim",
        "other-claim-by-s1",
        "evidence.dependent.shared-source",
    );
    let c = g.corroborate(&["claim", "other-claim-by-s1"]).unwrap();
    assert_eq!(c.value.independent_count(), 1);
}

#[test]
fn case_b_two_supporters_sharing_a_source_count_once() {
    let mut g = EvidenceGraph::new();
    g.add_item("origin", "s0", &[]).unwrap();
    g.add_item("sup-1", "s1", &["origin"]).unwrap();
    g.add_item("sup-2", "s2", &["origin"]).unwrap();
    dependent(&g, "sup-1", "sup-2", "evidence.dependent.shared-ancestor");
    assert_eq!(
        g.corroborate(&["sup-1", "sup-2"])
            .unwrap()
            .value
            .independent_count(),
        1
    );
}

#[test]
fn case_c_supporters_citing_each_other_in_a_cycle_count_once() {
    let mut g = EvidenceGraph::new();
    g.add_item("x", "s1", &["y"]).unwrap();
    g.add_item("y", "s2", &["x"]).unwrap();
    dependent(&g, "x", "y", "evidence.dependent.mutual-citation");
    assert_eq!(
        g.corroborate(&["x", "y"])
            .unwrap()
            .value
            .independent_count(),
        1
    );
}

#[test]
fn case_d_a_later_version_of_a_claim_does_not_support_the_earlier_one() {
    // the consumer records the later version as derived from the earlier
    let mut g = EvidenceGraph::new();
    g.add_item("v1", "s1", &[]).unwrap();
    g.add_item("v2", "s2", &["v1"]).unwrap();
    dependent(&g, "v1", "v2", "evidence.dependent.ancestor");
    assert_eq!(
        g.corroborate(&["v1", "v2"])
            .unwrap()
            .value
            .independent_count(),
        1
    );
}

#[test]
fn control_two_truly_independent_sources_do_count_twice() {
    let mut g = EvidenceGraph::new();
    g.add_item("a", "s1", &[]).unwrap();
    g.add_item("b", "s2", &[]).unwrap();
    assert!(g.independent(&"a", &"b").unwrap().value);
    assert_eq!(
        g.corroborate(&["a", "b"])
            .unwrap()
            .value
            .independent_count(),
        2
    );
}

// NOT COVERED (4.5 item 3), stated so nobody assumes they are:
// (e) evidence missing its source or time: `add_item` requires a source, so the type cannot express it; the time is
//     not an input to the graph at all. The consumer must not call `add_item` without them.
// (f) evidence from a claim that "no longer exists": covered by `remove_item` / `remove_item_and_dependents` (unit
//     tests in src/lib.rs); not repeated here.
// (g) an imported rating offered as a level: consumer-side; here a rating can only enter as an item with its own source.
// (h) same-source contradiction: `-justify`'s `contradict` does not distinguish it; the consumer decides (SageAnt confirmed
//     it handles this in its mapper, 2026-11-19).
