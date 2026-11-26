//! Fixtures built from SageAnt's proposed inputs (SageAnt repo, branch `work/phase1-design` @ f2ff0d6,
//! design note 4.5, a PROPOSAL). Its item 1 speaks of a claim later "supported or contradicted by an
//! independent source". SageAnt's note states NO requirement for defeaters; these cases are this lane's
//! reading of how a retraction notice or a contradiction would look as reasons, and shape tests only,
//! never the API.
use epistemologicant_defeat::*;

#[test]
fn a_retraction_notice_undercuts_the_report_it_retracts() {
    let mut s = DefeasibleSystem::new();
    let report = s.assert("claim-x", &[], 2, "paper-a says x");
    s.assert("paper-a-retracted", &[], 2, "retraction notice");
    s.undercut(report, "paper-a-retracted").unwrap();
    let v = s.status(&"claim-x");
    assert_eq!(v.value, Standing::Out);
    assert_eq!(v.reasons[0].rule, "defeat.out");
    // another source's reason for the same claim is untouched
    s.assert("claim-x", &[], 2, "survey says x");
    assert_eq!(s.status(&"claim-x").value, Standing::In);
}

#[test]
fn an_independent_contradiction_of_equal_standing_leaves_the_claim_undecided_not_false() {
    let mut s = DefeasibleSystem::new();
    s.assert("claim-x", &[], 2, "source one");
    s.assert("not-x", &[], 2, "independent source two");
    s.contradict("claim-x", "not-x");
    assert_eq!(s.status(&"claim-x").value, Standing::Undecided);
    assert_eq!(s.status(&"not-x").value, Standing::Undecided);
}

#[test]
fn a_better_ranked_contradiction_wins_and_the_loser_says_why() {
    let mut s = DefeasibleSystem::new();
    s.assert("claim-x", &[], 1, "blog says x");
    s.assert("not-x", &[], 4, "lab measurement says not x");
    s.contradict("claim-x", "not-x");
    assert_eq!(s.status(&"not-x").value, Standing::In);
    let v = s.status(&"claim-x");
    assert_eq!(v.value, Standing::Out);
    assert!(
        v.reasons[0].note.contains("lab measurement"),
        "{:?}",
        v.reasons[0]
    );
}

#[test]
fn discrediting_the_better_source_reinstates_the_claim() {
    let mut s = DefeasibleSystem::new();
    s.assert("claim-x", &[], 1, "blog says x");
    let lab = s.assert("not-x", &[], 4, "lab says not x");
    s.contradict("claim-x", "not-x");
    s.assert("lab-miscalibrated", &[], 1, "audit");
    s.undercut(lab, "lab-miscalibrated").unwrap();
    assert_eq!(s.status(&"claim-x").value, Standing::In);
    assert_eq!(s.status(&"not-x").value, Standing::Out);
}
