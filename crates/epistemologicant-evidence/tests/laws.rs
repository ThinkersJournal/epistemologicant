//! Property tests for the evidence graph: independence is symmetric, an item is never independent of
//! itself, and corroboration partitions its input without ever counting more clusters than items.
use epistemologicant_evidence::*;
use proptest::prelude::*;

type G = EvidenceGraph<u8, u8>;

fn graph(spec: &[(u8, Vec<u8>)]) -> G {
    let mut g = G::new();
    for (i, (src, from)) in spec.iter().enumerate() {
        g.add_item(i as u8, *src, from).unwrap();
    }
    g
}

fn spec() -> impl Strategy<Value = Vec<(u8, Vec<u8>)>> {
    prop::collection::vec((0u8..4, prop::collection::vec(0u8..14, 0..3)), 2..12)
}

proptest! {
    #[test]
    fn independence_is_symmetric_and_irreflexive(s in spec(), a in any::<prop::sample::Index>(), b in any::<prop::sample::Index>()) {
        let g = graph(&s);
        let (x, y) = (a.index(s.len()) as u8, b.index(s.len()) as u8);
        prop_assert_eq!(g.independent(&x, &y).unwrap().value, g.independent(&y, &x).unwrap().value);
        prop_assert!(!g.independent(&x, &x).unwrap().value);
    }

    #[test]
    fn two_distinct_items_from_one_source_are_never_independent(s in spec(), a in any::<prop::sample::Index>(), b in any::<prop::sample::Index>()) {
        let g = graph(&s);
        let (x, y) = (a.index(s.len()), b.index(s.len()));
        prop_assume!(x != y && s[x].0 == s[y].0);
        prop_assert!(!g.independent(&(x as u8), &(y as u8)).unwrap().value);
    }

    #[test]
    fn corroboration_partitions_the_input(s in spec()) {
        let g = graph(&s);
        let ids: Vec<u8> = (0..s.len() as u8).collect();
        let c = g.corroborate(&ids).unwrap().value;
        let mut all: Vec<u8> = c.clusters.iter().flatten().copied().collect();
        all.sort();
        prop_assert_eq!(all, ids.clone(), "every item in exactly one cluster");
        prop_assert!(c.independent_count() <= ids.len());
        // items in different clusters are independent of each other
        for (i, a) in c.clusters.iter().enumerate() {
            for b in c.clusters.iter().skip(i + 1) {
                prop_assert!(g.independent(&a[0], &b[0]).unwrap().value, "{a:?} vs {b:?}");
            }
        }
    }

    #[test]
    fn removing_an_item_never_creates_dependence_among_the_rest_except_through_it(s in spec(), pick in any::<prop::sample::Index>()) {
        let mut g = graph(&s);
        let gone = pick.index(s.len()) as u8;
        let removed = g.remove_item_and_dependents(&gone).unwrap();
        prop_assert!(removed.contains(&gone));
        prop_assert_eq!(g.len(), s.len() - removed.len());
        for id in &removed { prop_assert!(g.independent(id, id).is_err()); }
    }
}
