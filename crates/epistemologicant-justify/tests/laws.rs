//! Property tests for justification laws: retraction order never changes the final support, the
//! reported fallout is exactly the support difference, and a retraction never adds support.
use epistemologicant_justify::*;
use proptest::prelude::*;

type G = Ground<u8>;

fn build(spec: &[(u8, Option<Vec<u8>>)]) -> (Justifications<u8>, Vec<GroundId>) {
    let mut j = Justifications::new();
    let ids = spec
        .iter()
        .map(|(claim, ante)| {
            let g: G = match ante {
                None => Ground::Premise { label: "p".into() },
                Some(a) => Ground::Inference {
                    antecedents: a.clone(),
                },
            };
            j.assert(*claim, g)
        })
        .collect();
    (j, ids)
}

fn spec() -> impl Strategy<Value = Vec<(u8, Option<Vec<u8>>)>> {
    prop::collection::vec(
        (
            0u8..8,
            prop::option::of(prop::collection::vec(0u8..8, 0..3)),
        ),
        1..14,
    )
}

proptest! {
    #[test]
    fn fallout_is_exactly_the_support_difference_and_never_adds(s in spec(), pick in any::<prop::sample::Index>()) {
        let (mut j, ids) = build(&s);
        let before = j.supported_claims();
        let f = j.retract(ids[pick.index(ids.len())]).unwrap();
        let after = j.supported_claims();
        prop_assert!(after.iter().all(|c| before.contains(c)), "retraction added support");
        let diff: Vec<u8> = before.iter().filter(|c| !after.contains(c)).copied().collect();
        prop_assert_eq!(f.lost, diff);
    }

    #[test]
    fn retracting_a_ground_equals_never_having_asserted_it(s in spec(), pick in any::<prop::sample::Index>()) {
        let (mut j, ids) = build(&s);
        let i = pick.index(ids.len());
        j.retract(ids[i]).unwrap();
        let mut without = s.clone();
        without.remove(i);
        let (fresh, _) = build(&without);
        prop_assert_eq!(j.supported_claims(), fresh.supported_claims());
    }

    #[test]
    fn retraction_order_does_not_matter(s in spec(), a in any::<prop::sample::Index>(), b in any::<prop::sample::Index>()) {
        let (mut j1, ids) = build(&s);
        let (mut j2, _) = build(&s);
        let (x, y) = (ids[a.index(ids.len())], ids[b.index(ids.len())]);
        if x == y {
            return Ok(());
        }
        j1.retract(x).unwrap(); j1.retract(y).unwrap();
        j2.retract(y).unwrap(); j2.retract(x).unwrap();
        prop_assert_eq!(j1.supported_claims(), j2.supported_claims());
    }

    #[test]
    fn a_supported_claim_has_a_wellfounded_chain_to_a_premise(s in spec()) {
        // every supported claim is a premise claim or has an inference whose antecedents are supported
        let (j, _) = build(&s);
        let sup = j.supported_claims();
        for c in &sup {
            let ok = s.iter().any(|(cl, g)| cl == c && match g {
                None => true,
                Some(a) => a.iter().all(|x| sup.contains(x)),
            });
            prop_assert!(ok, "{c} supported without a ground");
        }
        // and nothing unsupported has a premise
        for (cl, g) in &s {
            if g.is_none() { prop_assert!(sup.contains(cl)); }
        }
    }
}
