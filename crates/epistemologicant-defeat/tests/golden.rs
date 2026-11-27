//! Determinism evidence: standings and reasons over a fixed scenario (rebuttal, collective defeat,
//! undercutting, reinstatement, a defeat cycle, retractions) must match `golden/defeat.txt` byte for
//! byte. That ONE committed file is the reference every OS in CI compares against.
//! To regenerate after an intended change: `BLESS=1 cargo test -p epistemologicant-defeat --test golden`.
use epistemologicant_defeat::*;

fn dump(s: &DefeasibleSystem<&'static str>, claims: &[&'static str], out: &mut String) {
    for c in claims {
        let v = s.status(c);
        *out += &format!("{c} -> {:?}\n", v.value);
        for r in v.reasons {
            *out += &format!("  {} | {}\n", r.rule, r.note);
        }
    }
}

fn render() -> String {
    let claims = [
        "flies",
        "grounded",
        "p",
        "not-p",
        "red",
        "looks-red",
        "red-light",
        "a",
        "b",
        "c",
        "ghost",
    ];
    let mut s = DefeasibleSystem::new();
    s.assert("flies", &[], 1, "default");
    s.assert("grounded", &[], 5, "vet report");
    s.contradict("flies", "grounded");
    s.assert("p", &[], 3, "source a");
    s.assert("not-p", &[], 3, "source b");
    s.contradict("p", "not-p");
    s.assert("looks-red", &[], 1, "perception");
    let red = s.assert("red", &["looks-red"], 1, "looks red");
    let light = s.assert("red-light", &[], 1, "lighting report");
    s.undercut(red, "red-light").unwrap();
    let ra = s.assert("a", &[], 1, "ra");
    let rb = s.assert("b", &[], 1, "rb");
    let rc = s.assert("c", &[], 1, "rc");
    s.undercut(ra, "b").unwrap();
    s.undercut(rb, "c").unwrap();
    let mut out = String::from("== initial\n");
    dump(&s, &claims, &mut out);
    for (name, r) in [("lighting report", light), ("rc", rc)] {
        let ch = s.retract(r).unwrap();
        out += &format!(
            "== retract {name}: {:?}\n",
            ch.iter()
                .map(|c| (c.claim, c.before, c.after))
                .collect::<Vec<_>>()
        );
        dump(&s, &claims, &mut out);
    }
    out
}

#[test]
fn output_is_identical_to_golden() {
    let got = render();
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let path = format!("{dir}/defeat.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
