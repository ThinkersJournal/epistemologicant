//! Determinism evidence: reliabilities (as raw IEEE-754 bits) and reasons over a fixed history must
//! match `golden/record.txt` byte for byte. That ONE committed file is the reference every OS in CI
//! compares against. To regenerate after an intended change:
//! `BLESS=1 cargo test -p epistemologicant-record --test golden`.
use epistemologicant_credence::Credence;
use epistemologicant_record::*;

fn render() -> String {
    let mut r = SourceRecord::new(Params::new(0.55, 3.0, 30).unwrap());
    r.set_parent("physics", "science").unwrap();
    r.set_parent("biology", "science").unwrap();
    r.set_parent("science", "all").unwrap();
    for i in 0..40i64 {
        r.observe("wire", "physics", i % 4 != 0, i * 3);
        r.observe("wire", "biology", i % 5 == 0, i * 3 + 1);
        r.observe("blog", "physics", i < 10, i * 7);
    }
    r.observe("lab", "biology", true, 5);
    let mut out = String::new();
    for now in [280i64, 400, 800, 2000] {
        for s in ["wire", "blog", "lab", "nobody"] {
            for t in ["physics", "biology", "science", "all", "chemistry"] {
                let v = r.reliability(&s, &t, now).unwrap();
                let bits = match v.value {
                    Credence::Point(p) => format!("{:016x}", p.to_bits()),
                    ref o => panic!("{o:?}"),
                };
                out += &format!("now={now} {s} {t} -> {bits}\n");
                for x in v.reasons {
                    out += &format!("  {} | {}\n", x.rule, x.note);
                }
            }
        }
    }
    out
}

#[test]
fn output_is_identical_to_golden() {
    let got = render();
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let path = format!("{dir}/record.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
