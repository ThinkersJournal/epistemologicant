//! Determinism evidence: grounded labels with reasons and every extension list over fixed frameworks
//! must match `golden/argument.txt` byte for byte. That ONE committed file is the reference every OS in
//! CI compares against. To regenerate after an intended change:
//! `BLESS=1 cargo test -p epistemologicant-argument --test golden`.
use epistemologicant_argument::*;

/// A named framework: its arguments and its attacks.
type Case = (
    &'static str,
    &'static [&'static str],
    &'static [(&'static str, &'static str)],
);

fn render() -> String {
    let frameworks: [Case; 4] = [
        (
            "published",
            &["a", "b", "c", "d"],
            &[("a", "b"), ("b", "c"), ("d", "c")],
        ),
        ("two-cycle", &["a", "b"], &[("a", "b"), ("b", "a")]),
        (
            "odd-cycle",
            &["a", "b", "c"],
            &[("a", "b"), ("b", "c"), ("c", "a")],
        ),
        (
            "mixed",
            &["a", "b", "c", "d", "e"],
            &[
                ("a", "b"),
                ("b", "a"),
                ("b", "c"),
                ("c", "d"),
                ("d", "d"),
                ("e", "c"),
            ],
        ),
    ];
    let mut out = String::new();
    for (name, args, atts) in frameworks {
        let mut f = ArgumentationFramework::new();
        for a in args {
            f.add_argument(*a);
        }
        for (s, t) in atts {
            f.add_attack(*s, *t).unwrap();
        }
        out += &format!("== {name}\n");
        for a in args {
            let v = f.explain(a).unwrap();
            out += &format!(
                "{a} -> {:?} | {} | {}\n",
                v.value, v.reasons[0].rule, v.reasons[0].note
            );
        }
        for sem in [
            Semantics::Grounded,
            Semantics::Complete,
            Semantics::Preferred,
            Semantics::Stable,
        ] {
            out += &format!("{sem:?}: {:?}\n", f.extensions(sem).unwrap());
        }
        let sk = f.skeptically_accepted(Semantics::Stable, &args[0]).unwrap();
        out += &format!(
            "stable-skeptical({}): {} ({})\n",
            args[0], sk.value, sk.reasons[0].rule
        );
    }
    out
}

#[test]
fn output_is_identical_to_golden() {
    let got = render();
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/golden");
    let path = format!("{dir}/argument.txt");
    if std::env::var_os("BLESS").is_some() {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, &got).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("golden file missing; run with BLESS=1 once");
    // line endings are not part of the claim; git may convert them on checkout
    assert_eq!(got.replace("\r\n", "\n"), want.replace("\r\n", "\n"));
}
