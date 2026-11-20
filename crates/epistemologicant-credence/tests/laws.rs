//! Property tests for the laws the combination rules and scores promise (SPEC-STEP1 section 7).
//! Exact equality is asserted only where the arithmetic guarantees it (min/max, commutative
//! addition, skipped inputs); everything else is within 1e-12.
use epistemologicant_credence::*;
use proptest::prelude::*;

const EPS: f64 = 1e-12;

fn prob() -> impl Strategy<Value = f64> {
    prop_oneof![Just(0.0), Just(1.0), 0.0f64..=1.0]
}

fn credence() -> impl Strategy<Value = Credence> {
    prop_oneof![
        Just(Credence::Unknown),
        prob().prop_map(|p| Credence::point(p).unwrap()),
        (prob(), prob()).prop_map(|(a, b)| Credence::interval(a.min(b), a.max(b)).unwrap()),
    ]
}

fn bounds(c: &Credence) -> Option<(f64, f64)> {
    match *c {
        Credence::Unknown => None,
        Credence::Point(p) => Some((p, p)),
        Credence::Interval { lo, hi } => Some((lo, hi)),
        Credence::Ordinal(_) => unreachable!(),
    }
}

proptest! {
    #[test]
    fn noisy_or_stays_in_unit_interval_and_keeps_lo_le_hi(xs in prop::collection::vec(credence(), 1..8)) {
        let v = combine_independent(&xs).unwrap().value;
        if let Some((lo, hi)) = bounds(&v) {
            prop_assert!((0.0..=1.0).contains(&lo) && (0.0..=1.0).contains(&hi));
            prop_assert!(lo <= hi);
        } else {
            prop_assert!(xs.iter().all(|c| *c == Credence::Unknown), "unknown only from all-unknown");
        }
    }

    #[test]
    fn noisy_or_never_lowers_what_it_was_given(xs in prop::collection::vec(credence(), 1..8)) {
        let v = combine_independent(&xs).unwrap().value;
        if let Some((lo, hi)) = bounds(&v) {
            for (l, h) in xs.iter().filter_map(bounds) {
                prop_assert!(lo >= l - EPS && hi >= h - EPS, "combined ({lo},{hi}) below input ({l},{h})");
            }
        }
    }

    #[test]
    fn noisy_or_ignores_order(xs in prop::collection::vec(credence(), 1..8)) {
        let mut ys = xs.clone();
        ys.reverse();
        let (a, b) = (combine_independent(&xs).unwrap().value, combine_independent(&ys).unwrap().value);
        match (bounds(&a), bounds(&b)) {
            (None, None) => {}
            (Some((al, ah)), Some((bl, bh))) => prop_assert!((al - bl).abs() < EPS && (ah - bh).abs() < EPS),
            other => prop_assert!(false, "{other:?}"),
        }
    }

    #[test]
    fn unknown_adds_nothing_bit_for_bit(xs in prop::collection::vec(credence(), 1..6)) {
        let mut with = xs.clone();
        with.push(Credence::Unknown);
        let (a, b) = (combine_independent(&xs).unwrap().value, combine_independent(&with).unwrap().value);
        prop_assert_eq!(a, b);
    }

    #[test]
    fn certainty_absorbs_and_zero_is_the_identity(p in prob()) {
        let one = Credence::point(1.0).unwrap();
        let got = combine_independent(&[Credence::point(p).unwrap(), one]).unwrap().value;
        prop_assert_eq!(got, one);
        let got = combine_independent(&[Credence::point(p).unwrap(), Credence::point(0.0).unwrap()]).unwrap().value;
        let (lo, _) = bounds(&got).unwrap();
        prop_assert!((lo - p).abs() < EPS);
    }

    #[test]
    fn hull_contains_both_is_exactly_commutative_and_idempotent(a in credence(), b in credence()) {
        let ab = combine_conflicting(&a, &b, ConflictPolicy::Hull).unwrap().value;
        let ba = combine_conflicting(&b, &a, ConflictPolicy::Hull).unwrap().value;
        prop_assert_eq!(ab, ba);
        if let (Some((lo, hi)), Some((al, ah)), Some((bl, bh))) = (bounds(&ab), bounds(&a), bounds(&b)) {
            prop_assert!(lo <= al && lo <= bl && hi >= ah && hi >= bh);
        }
        let aa = combine_conflicting(&a, &a, ConflictPolicy::Hull).unwrap().value;
        // compare bounds: a degenerate interval is normalised to a point by the combinators
        prop_assert_eq!(bounds(&aa), bounds(&a));
    }

    #[test]
    fn mean_is_commutative_and_between_its_inputs(a in credence(), b in credence()) {
        let ab = combine_conflicting(&a, &b, ConflictPolicy::Mean).unwrap().value;
        let ba = combine_conflicting(&b, &a, ConflictPolicy::Mean).unwrap().value;
        prop_assert_eq!(ab, ba);
        if let (Some((lo, hi)), Some((al, ah)), Some((bl, bh))) = (bounds(&ab), bounds(&a), bounds(&b)) {
            prop_assert!(lo >= al.min(bl) - EPS && hi <= ah.max(bh) + EPS);
            prop_assert!(hi >= ah.min(bh) - EPS && lo <= al.max(bl) + EPS);
            prop_assert!((lo - (al + bl) / 2.0).abs() < EPS && (hi - (ah + bh) / 2.0).abs() < EPS);
        }
    }

    #[test]
    fn brier_is_in_unit_interval_and_zero_only_when_perfect(ps in prop::collection::vec((prob(), any::<bool>()), 1..10)) {
        let pairs: Vec<_> = ps.iter().map(|&(p, o)| (Credence::point(p).unwrap(), o)).collect();
        let b = brier(&pairs).unwrap();
        prop_assert!((0.0..=1.0).contains(&b));
        let perfect = ps.iter().all(|&(p, o)| p == if o { 1.0 } else { 0.0 });
        prop_assert_eq!(b == 0.0, perfect);
    }

    #[test]
    fn brier_is_the_mean_of_squared_errors(ps in prop::collection::vec((prob(), any::<bool>()), 1..10)) {
        let pairs: Vec<_> = ps.iter().map(|&(p, o)| (Credence::point(p).unwrap(), o)).collect();
        let want = ps.iter().map(|&(p, o)| (p - if o { 1.0 } else { 0.0 }).powi(2)).sum::<f64>() / ps.len() as f64;
        prop_assert!((brier(&pairs).unwrap() - want).abs() < EPS);
        // repeating a whole sample does not change the mean
        let doubled: Vec<_> = pairs.iter().chain(pairs.iter()).cloned().collect();
        prop_assert!((brier(&doubled).unwrap() - brier(&pairs).unwrap()).abs() < EPS);
    }

    #[test]
    fn brier_prefers_the_truthful_forecast(o in any::<bool>(), p in 0.0f64..=1.0) {
        // a point given to the outcome that happened scores no worse than the mirror image
        let truth = if o { p.max(1.0 - p) } else { p.min(1.0 - p) };
        let mirror = 1.0 - truth;
        let s = |x: f64| brier(&[(Credence::point(x).unwrap(), o)]).unwrap();
        prop_assert!(s(truth) <= s(mirror) + EPS);
    }

    #[test]
    fn scoring_refuses_intervals_and_unknowns(lo in 0.0f64..0.5, hi in 0.5f64..=1.0) {
        let iv = Credence::interval(lo, hi).unwrap();
        prop_assert!(brier(&[(iv, true)]).is_err());
        prop_assert!(brier(&[(Credence::Unknown, true)]).is_err());
    }

    #[test]
    fn level_scale_round_trips(n in 2usize..9, level in 0usize..9) {
        let scale = LevelScale::new((0..n).map(|i| format!("l{i}")).collect()).unwrap();
        let level = level % n;
        let c = scale.from_level(level).unwrap();
        prop_assert_eq!(scale.to_level(&c).unwrap(), level);
    }
}
