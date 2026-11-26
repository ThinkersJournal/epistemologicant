//! Property tests for defeasible reasoning. Inputs are BUILT (no `prop_assume!`): a reason spec is a
//! claim, antecedents, a rank, and the claims that undercut it; contradictions are pairs of claims.
use epistemologicant_defeat::*;
use epistemologicant_justify as justify;
use proptest::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct R {
    claim: u8,
    ante: Vec<u8>,
    rank: u32,
    cut_by: Vec<u8>,
}

fn reasons() -> impl Strategy<Value = Vec<R>> {
    prop::collection::vec(
        (
            0u8..7,
            prop::collection::vec(0u8..7, 0..3),
            0u32..4,
            prop::collection::vec(0u8..7, 0..2),
        )
            .prop_map(|(claim, ante, rank, cut_by)| R {
                claim,
                ante,
                rank,
                cut_by,
            }),
        1..12,
    )
}

fn pairs() -> impl Strategy<Value = Vec<(u8, u8)>> {
    prop::collection::vec((0u8..7, 0u8..7), 0..4)
}

/// Build in the given order; returns the system.
fn build(rs: &[R], order: &[usize], conflicts: &[(u8, u8)]) -> DefeasibleSystem<u8> {
    let mut s = DefeasibleSystem::new();
    let mut ids = BTreeMap::new();
    for &i in order {
        ids.insert(i, s.assert(rs[i].claim, &rs[i].ante, rs[i].rank, "r"));
    }
    for &i in order {
        for c in &rs[i].cut_by {
            s.undercut(ids[&i], *c).unwrap();
        }
    }
    for &(a, b) in conflicts {
        s.contradict(a, b);
    }
    s
}

fn identity(n: usize) -> Vec<usize> {
    (0..n).collect()
}

proptest! {
    #[test]
    fn contradictory_claims_are_never_both_in(rs in reasons(), cs in pairs()) {
        let s = build(&rs, &identity(rs.len()), &cs);
        let st = s.standings();
        for (a, b) in &cs {
            prop_assert!(!(st[a] == Standing::In && st[b] == Standing::In), "{a} and {b} both in");
        }
    }

    #[test]
    fn the_verdict_does_not_depend_on_the_order_reasons_were_added(rs in reasons(), cs in pairs(), seed in any::<u64>()) {
        let mut order = identity(rs.len());
        // deterministic shuffle from the seed
        let mut x = seed | 1;
        for i in (1..order.len()).rev() {
            x ^= x << 13; x ^= x >> 7; x ^= x << 17;
            order.swap(i, (x % (i as u64 + 1)) as usize);
        }
        let a = build(&rs, &identity(rs.len()), &cs).standings();
        let b = build(&rs, &order, &cs).standings();
        prop_assert_eq!(a, b);
    }

    #[test]
    fn in_and_out_are_forced_by_the_reasons(rs in reasons(), cs in pairs()) {
        let s = build(&rs, &identity(rs.len()), &cs);
        let st = s.standings();
        for (claim, standing) in &st {
            let own: Vec<&R> = rs.iter().filter(|r| r.claim == *claim).collect();
            match standing {
                // In: some reason has every antecedent In and every undercutter Out
                Standing::In => prop_assert!(own.iter().any(|r|
                    r.ante.iter().all(|a| st[a] == Standing::In) && r.cut_by.iter().all(|c| st[c] == Standing::Out))),
                // Out: no reason is applicable (an antecedent is Out, an undercutter is In, or a stronger live rebutter exists,
                // or there is no reason): in every case it is not the case that all antecedents are In and all undercutters Out
                // unless a rebutter beat it, so check the weaker, always-true consequence: no reason is both applicable and unrebutted
                Standing::Out => prop_assert!(own.iter().all(|r|
                    r.ante.iter().any(|a| st[a] == Standing::Out)
                    || r.cut_by.iter().any(|c| st[c] == Standing::In)
                    || cs.iter().any(|&(x, y)| (x == *claim && st[&y] == Standing::In) || (y == *claim && st[&x] == Standing::In)))),
                Standing::Undecided => {}
            }
        }
    }

    #[test]
    fn with_no_defeaters_and_equal_ranks_it_agrees_with_justify(rs in reasons()) {
        // the same reasons, no undercuts, no contradictions: In == justify's well-founded support
        let mut d = DefeasibleSystem::new();
        let mut j = justify::Justifications::new();
        for r in &rs {
            d.assert(r.claim, &r.ante, 1, "r");
            let g: justify::Ground<u8> = if r.ante.is_empty() {
                justify::Ground::Premise { label: "p".into() }
            } else {
                justify::Ground::Inference { antecedents: r.ante.clone() }
            };
            j.assert(r.claim, g);
        }
        let held: Vec<u8> = d.standings().into_iter().filter(|(_, s)| *s == Standing::In).map(|(c, _)| c).collect();
        prop_assert_eq!(held, j.supported_claims());
        // and nothing is undecided without a defeater
        prop_assert!(d.standings().values().all(|s| *s != Standing::Undecided));
    }

    #[test]
    fn retract_reports_exactly_the_standing_difference(rs in reasons(), cs in pairs(), pick in any::<prop::sample::Index>()) {
        let mut s = build(&rs, &identity(rs.len()), &cs);
        let before = s.standings();
        let ch = s.retract(ReasonId(pick.index(rs.len()) as u64)).unwrap();
        let after = s.standings();
        let diff: Vec<_> = before.iter().filter(|(c, b)| after[*c] != **b).map(|(c, b)| (*c, *b, after[c])).collect();
        let got: Vec<_> = ch.iter().map(|c| (c.claim, c.before, c.after)).collect();
        prop_assert_eq!(got, diff);
    }

    #[test]
    fn a_raised_rank_never_helps_the_opponent(rs in reasons(), cs in pairs(), pick in any::<prop::sample::Index>()) {
        // raising one reason's rank above every other can only make ITS claim stand better or equal
        // against its rebutters; checked as: if it was In it stays In
        let i = pick.index(rs.len());
        let before = build(&rs, &identity(rs.len()), &cs).standings();
        let mut hi = rs.clone();
        hi[i].rank = 100;
        let after = build(&hi, &identity(rs.len()), &cs).standings();
        let c = rs[i].claim;
        // only claim the property where nothing else about the claim's support changes: skip claims with several reasons
        if rs.iter().filter(|r| r.claim == c).count() == 1 && before[&c] == Standing::In {
            prop_assert_eq!(after[&c], Standing::In);
        }
    }
}
