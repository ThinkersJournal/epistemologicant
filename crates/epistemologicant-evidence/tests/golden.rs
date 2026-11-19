//! Determinism evidence: the independence verdicts (value, rule ids and notes), the clusters and the
//! combined credences (as raw IEEE-754 bits) over a fixed graph must match `golden/evidence.txt` byte
//! for byte. That ONE committed file is the reference every OS in CI compares against.
//! To regenerate after an intended change: `BLESS=1 cargo test -p epistemologicant-evidence --test golden`.
use epistemologicant_credence::Credence;
use epistemologicant_evidence::*;

fn graph() -> EvidenceGraph<&'static str, &'static str> {
    let mut g = EvidenceGraph::new();
    g.add_item("agency-report", "agency", &[]).unwrap();
    g.add_item("paper-a", "paper-1", &["agency-report"])
        .unwrap();
    g.add_item("paper-b", "paper-2", &["agency-report"])
        .unwrap();
    g.add_item("blog", "blog-1", &["paper-b"]).unwrap();
    g.add_item("ring-x", "s-x", &["ring-y"]).unwrap();
    g.add_item("ring-y", "s-y", &["ring-x"]).unwrap();
    g.add_item("lab", "lab-1", &["field-notes"]).unwrap();
    g.add_item("survey", "survey-1", &[]).unwrap();
    g.add_item("survey-2", "survey-1", &[]).unwrap();
    g
}

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
    let g = graph();
    let ids = [
        "agency-report",
        "paper-a",
        "paper-b",
        "blog",
        "ring-x",
        "ring-y",
        "lab",
        "survey",
        "survey-2",
    ];
    let mut out = String::new();
    for (i, a) in ids.iter().enumerate() {
        for b in &ids[i..] {
            let v = g.independent(a, b).unwrap();
            out += &format!("{a} {b} -> {}\n", v.value);
            for r in v.reasons {
                out += &format!("  {} | {}\n", r.rule, r.note);
            }
        }
    }
    let c = g.corroborate(&ids).unwrap();
    out += &format!("clusters {:?}\n", c.value.clusters);
    for r in c.reasons {
        out += &format!("  {} | {}\n", r.rule, r.note);
    }
    let p = |x: f64| Credence::point(x).unwrap();
    let items = [
        ("paper-a", p(0.3)),
        ("paper-b", p(0.55)),
        ("blog", p(0.61)),
        ("ring-x", p(0.37)),
        ("ring-y", Credence::interval(0.2, 0.9).unwrap()),
        ("lab", p(0.1)),
        ("survey", p(0.999)),
        ("survey-2", Credence::Unknown),
    ];
    let v = g.combine_corroborated(&items).unwrap();
    out += &format!("combined {}\n", bits(&v.value));
    out
}

#[test]
fn output_is_identical_to_golden() {
    let got = render();
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let path = format!("{dir}/evidence.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
