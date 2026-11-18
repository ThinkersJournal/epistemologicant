//! Determinism evidence: every number from the combination and Brier rules, as raw IEEE-754 bits, must
//! match `golden/credence.txt` byte for byte. CI runs this on ubuntu, windows and macos. `log_score`
//! is deliberately absent (it uses the platform's `ln`).
//! To regenerate after an intended change: `BLESS=1 cargo test -p epistemologicant-credence --test golden`.
use epistemologicant_credence::*;

fn bits(c: &Credence) -> String {
    match *c {
        Credence::Unknown => "unknown".into(),
        Credence::Ordinal(l) => format!("ordinal:{l}"),
        Credence::Point(p) => format!("point:{:016x}", p.to_bits()),
        Credence::Interval { lo, hi } => {
            format!("interval:{:016x}:{:016x}", lo.to_bits(), hi.to_bits())
        }
    }
}

fn render() -> String {
    let ps = [0.0, 0.1, 0.2, 0.3, 0.37, 0.5, 0.61, 0.9, 0.999, 1.0];
    let mut out = String::new();
    for (i, &a) in ps.iter().enumerate() {
        for &b in &ps[i..] {
            let (ca, cb) = (Credence::point(a).unwrap(), Credence::point(b).unwrap());
            let v = combine_independent(&[
                ca,
                cb,
                Credence::Unknown,
                Credence::interval(a.min(b), a.max(b)).unwrap(),
            ])
            .unwrap();
            out += &format!("or {a} {b} {}\n", bits(&v.value));
            for p in [ConflictPolicy::Hull, ConflictPolicy::Mean] {
                out += &format!(
                    "conflict {p:?} {a} {b} {}\n",
                    bits(&combine_conflicting(&ca, &cb, p).unwrap().value)
                );
            }
        }
    }
    let pairs: Vec<(Credence, bool)> = ps
        .iter()
        .enumerate()
        .map(|(i, &p)| (Credence::point(p).unwrap(), i % 3 != 0))
        .collect();
    out += &format!("brier {:016x}\n", brier(&pairs).unwrap().to_bits());
    out
}

#[test]
fn output_is_bit_identical_to_golden() {
    let got = render();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/golden/credence.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/golden")).unwrap();
        std::fs::write(path, &got).unwrap();
    }
    let want = std::fs::read_to_string(path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
