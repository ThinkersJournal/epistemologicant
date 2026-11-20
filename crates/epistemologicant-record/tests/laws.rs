//! Property tests for the source record: the estimate stays in [0,1], more good outcomes never
//! lower it, ageing a bad history never lowers it, and sources never leak into each other.
use epistemologicant_record::*;
use proptest::prelude::*;

fn est(r: &SourceRecord<u8, u8>, s: u8, now: i64) -> f64 {
    match r.reliability(&s, &0, now).unwrap().value {
        epistemologicant_credence::Credence::Point(p) => p,
        o => panic!("{o:?}"),
    }
}

fn params() -> impl Strategy<Value = Params> {
    (0.0f64..=1.0, 0.1f64..20.0, 1i64..200).prop_map(|(b, k, h)| Params::new(b, k, h).unwrap())
}

proptest! {
    #[test]
    fn estimate_is_in_unit_interval(p in params(), obs in prop::collection::vec((any::<bool>(), 0i64..500), 0..30)) {
        let mut r = SourceRecord::new(p);
        for (ok, at) in &obs { r.observe(1u8, 0u8, *ok, *at); }
        let e = est(&r, 1, 500);
        prop_assert!((0.0..=1.0).contains(&e));
    }

    #[test]
    fn one_more_good_outcome_never_lowers_it_and_one_more_bad_never_raises_it(
        p in params(), obs in prop::collection::vec((any::<bool>(), 0i64..500), 0..20), at in 0i64..=500
    ) {
        let mut base = SourceRecord::new(p);
        for (ok, t) in &obs { base.observe(1u8, 0u8, *ok, *t); }
        let before = est(&base, 1, 500);
        let mut good = SourceRecord::new(p);
        let mut bad = SourceRecord::new(p);
        for (ok, t) in &obs { good.observe(1u8, 0u8, *ok, *t); bad.observe(1u8, 0u8, *ok, *t); }
        good.observe(1, 0, true, at);
        bad.observe(1, 0, false, at);
        prop_assert!(est(&good, 1, 500) >= before - 1e-12);
        prop_assert!(est(&bad, 1, 500) <= before + 1e-12);
    }

    #[test]
    fn a_purely_bad_history_recovers_toward_baseline_as_it_ages(p in params(), n in 1usize..20, t1 in 0i64..300, dt in 0i64..300) {
        let mut r = SourceRecord::new(p);
        for _ in 0..n { r.observe(1u8, 0u8, false, 0); }
        let (early, late) = (est(&r, 1, t1), est(&r, 1, t1 + dt));
        if p.baseline >= early { prop_assert!(late >= early - 1e-12); }
        prop_assert!(late <= p.baseline + 1e-12, "bad history cannot beat the baseline");
    }

    #[test]
    fn sources_do_not_leak(p in params(), obs in prop::collection::vec((any::<bool>(), 0i64..500), 1..20)) {
        let mut r = SourceRecord::new(p);
        for (ok, t) in &obs { r.observe(1u8, 0u8, *ok, *t); }
        prop_assert_eq!(est(&r, 2, 500), p.baseline);
    }
}
