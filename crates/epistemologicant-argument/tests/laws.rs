//! Property tests for abstract argumentation. The ORACLE below restates the definitions with
//! plain sets (no bitmasks, no shared code with the crate), so the two can disagree.
use epistemologicant_argument::*;
use epistemologicant_defeat as defeat;
use proptest::prelude::*;
use std::collections::BTreeSet;

type Set = BTreeSet<u8>;

/// (argument count, attacks); every edge endpoint is taken modulo the count (no rejection).
fn framework() -> impl Strategy<Value = (u8, Vec<(u8, u8)>)> {
    (1u8..7, prop::collection::vec((0u8..255, 0u8..255), 0..14))
        .prop_map(|(n, e)| (n, e.into_iter().map(|(a, b)| (a % n, b % n)).collect()))
}

fn build(n: u8, e: &[(u8, u8)]) -> ArgumentationFramework<u8> {
    let mut f = ArgumentationFramework::new();
    for i in 0..n {
        f.add_argument(i);
    }
    for (a, b) in e {
        f.add_attack(*a, *b).unwrap();
    }
    f
}

fn attacks(e: &[(u8, u8)], s: &Set, t: u8) -> bool {
    e.iter().any(|(a, b)| *b == t && s.contains(a))
}
fn conflict_free(e: &[(u8, u8)], s: &Set) -> bool {
    s.iter().all(|t| !attacks(e, s, *t))
}
fn defends(e: &[(u8, u8)], s: &Set, a: u8) -> bool {
    e.iter()
        .filter(|(_, t)| *t == a)
        .all(|(x, _)| attacks(e, s, *x))
}
fn admissible(e: &[(u8, u8)], s: &Set) -> bool {
    conflict_free(e, s) && s.iter().all(|a| defends(e, s, *a))
}
fn subsets(n: u8) -> Vec<Set> {
    (0u32..1 << n)
        .map(|m| (0..n).filter(|i| m >> i & 1 == 1).collect())
        .collect()
}
fn complete(n: u8, e: &[(u8, u8)]) -> Vec<Set> {
    subsets(n)
        .into_iter()
        .filter(|s| admissible(e, s) && (0..n).all(|a| !defends(e, s, a) || s.contains(&a)))
        .collect()
}
fn preferred(n: u8, e: &[(u8, u8)]) -> Vec<Set> {
    let adm: Vec<Set> = subsets(n)
        .into_iter()
        .filter(|s| admissible(e, s))
        .collect();
    adm.iter()
        .filter(|s| !adm.iter().any(|t| t != *s && t.is_superset(s)))
        .cloned()
        .collect()
}
fn stable(n: u8, e: &[(u8, u8)]) -> Vec<Set> {
    subsets(n)
        .into_iter()
        .filter(|s| {
            conflict_free(e, s) && (0..n).filter(|a| !s.contains(a)).all(|a| attacks(e, s, a))
        })
        .collect()
}
fn sorted(mut v: Vec<Set>) -> Vec<Set> {
    v.sort();
    v
}

proptest! {
    #[test]
    fn the_crate_agrees_with_the_set_oracle((n, e) in framework()) {
        let f = build(n, &e);
        prop_assert_eq!(sorted(f.extensions(Semantics::Complete).unwrap()), sorted(complete(n, &e)));
        prop_assert_eq!(sorted(f.extensions(Semantics::Preferred).unwrap()), sorted(preferred(n, &e)));
        prop_assert_eq!(sorted(f.extensions(Semantics::Stable).unwrap()), sorted(stable(n, &e)));
    }

    #[test]
    fn grounded_is_the_smallest_complete_extension_and_their_intersection((n, e) in framework()) {
        let f = build(n, &e);
        let g = f.extensions(Semantics::Grounded).unwrap().remove(0);
        let comp = complete(n, &e);
        prop_assert!(comp.contains(&g), "grounded must be complete");
        prop_assert!(comp.iter().all(|c| c.is_superset(&g)));
        let inter = comp.iter().fold((0..n).collect::<Set>(), |acc, c| acc.intersection(c).copied().collect());
        prop_assert_eq!(inter, g);
    }

    #[test]
    fn stable_implies_preferred_implies_complete((n, e) in framework()) {
        let f = build(n, &e);
        let (c, p, s) = (
            f.extensions(Semantics::Complete).unwrap(),
            f.extensions(Semantics::Preferred).unwrap(),
            f.extensions(Semantics::Stable).unwrap(),
        );
        prop_assert!(s.iter().all(|x| p.contains(x)));
        prop_assert!(p.iter().all(|x| c.contains(x)));
        prop_assert!(!p.is_empty(), "a preferred extension always exists");
    }

    #[test]
    fn the_grounded_labelling_matches_the_grounded_extension((n, e) in framework()) {
        let f = build(n, &e);
        let g: Set = f.extensions(Semantics::Grounded).unwrap().remove(0);
        let lab = f.grounded();
        let ins: Set = lab.iter().filter(|(_, l)| **l == Label::In).map(|(a, _)| *a).collect();
        prop_assert_eq!(&ins, &g);
        for (a, l) in &lab {
            // out exactly when an in-argument attacks it
            prop_assert_eq!(*l == Label::Out, attacks(&e, &g, *a), "argument {}", a);
        }
    }

    #[test]
    fn an_acyclic_framework_has_one_extension_under_every_semantics(n in 1u8..8, e in prop::collection::vec((0u8..255, 0u8..255), 0..12)) {
        // attacks only from a lower to a higher index
        let edges: Vec<(u8, u8)> = e.into_iter().map(|(a, b)| (a % n, b % n)).filter(|(a, b)| a < b).collect();
        let f = build(n, &edges);
        let g = f.extensions(Semantics::Grounded).unwrap();
        for sem in [Semantics::Complete, Semantics::Preferred, Semantics::Stable] {
            prop_assert_eq!(&f.extensions(sem).unwrap(), &g, "{:?}", sem);
        }
    }

    #[test]
    fn explanations_are_consistent_with_the_labels((n, e) in framework()) {
        let f = build(n, &e);
        let lab = f.grounded();
        for (a, l) in &lab {
            let v = f.explain(a).unwrap();
            prop_assert_eq!(v.value, *l);
            let rule = match l { Label::In => "argument.in", Label::Out => "argument.out", Label::Undecided => "argument.undecided" };
            prop_assert_eq!(v.reasons[0].rule, rule);
        }
    }

    /// -defeat encodes a framework: each argument is a premise reason for its own claim, and an attack
    /// a->b is an undercut of b's reason by claim a. The two crates then must give the same verdict.
    #[test]
    fn defeat_and_argument_agree_on_the_grounded_verdict((n, e) in framework()) {
        let f = build(n, &e);
        let mut d = defeat::DefeasibleSystem::new();
        let ids: Vec<_> = (0..n).map(|i| d.assert(i, &[], 1, "arg")).collect();
        for (a, b) in &e {
            d.undercut(ids[*b as usize], *a).unwrap();
        }
        for (a, l) in f.grounded() {
            let want = match l { Label::In => defeat::Standing::In, Label::Out => defeat::Standing::Out, Label::Undecided => defeat::Standing::Undecided };
            prop_assert_eq!(d.status(&a).value, want, "argument {}", a);
        }
    }
}
