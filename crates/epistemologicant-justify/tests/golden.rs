//! Determinism evidence: statuses, reasons and retraction fallout over a fixed scenario (chains,
//! a cycle, a double support, a contradiction) must match `golden/justify.txt` byte for byte. That ONE
//! committed file is the reference every OS in CI compares against.
//! To regenerate after an intended change: `BLESS=1 cargo test -p epistemologicant-justify --test golden`.
use epistemologicant_justify::*;

fn prem(s: &str) -> Ground<&'static str> {
    Ground::Premise {
        label: s.to_string(),
    }
}
fn inf(a: &[&'static str]) -> Ground<&'static str> {
    Ground::Inference {
        antecedents: a.to_vec(),
    }
}

fn dump(j: &Justifications<&'static str>, claims: &[&'static str], out: &mut String) {
    for c in claims {
        let v = j.status(c);
        *out += &format!("{c} -> {:?}\n", v.value);
        for r in v.reasons {
            *out += &format!("  {} | {}\n", r.rule, r.note);
        }
    }
}

fn render() -> String {
    let claims = ["a", "b", "c", "d", "e", "p", "q", "x", "not-x", "ghost"];
    let mut j = Justifications::new();
    let ga = j.assert("a", prem("observation 1"));
    j.assert("b", inf(&["a"]));
    j.assert("c", inf(&["b", "d"]));
    let gd = j.assert("d", prem("observation 2"));
    j.assert("d", prem("observation 3"));
    j.assert("e", inf(&["c"]));
    let gp = j.assert("p", prem("seed"));
    j.assert("q", inf(&["p"]));
    j.assert("p", inf(&["q"]));
    j.assert("x", prem("claim"));
    j.assert("not-x", prem("counter-claim"));
    j.contradict("x", "not-x");
    let mut out = String::from("== initial\n");
    dump(&j, &claims, &mut out);
    for (name, g) in [("d first support", gd), ("p seed", gp), ("a", ga)] {
        let f = j.retract(g).unwrap();
        out += &format!("== retract {name}: lost {:?}\n", f.lost);
        dump(&j, &claims, &mut out);
    }
    out
}

#[test]
fn output_is_identical_to_golden() {
    let got = render();
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let path = format!("{dir}/justify.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
