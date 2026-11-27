//! How the four tools fit together, in the shape SageAnt proposed (SageAnt repo, branch
//! `work/phase1-design` @ f2ff0d6, design note 4.5: a PROPOSAL, not approved defaults).
//! Run it: `cargo run -p epistemologicant --example sageant_shape`.
//!
//! Flow: source track records -> a reliability per source -> evidence items for one claim ->
//! corroboration that never counts the same origin twice -> an ordinal level with the reasons.
use epistemologicant::EpError;
use epistemologicant::credence::{Credence, LevelScale};
use epistemologicant::evidence::EvidenceGraph;
use epistemologicant::record::{Params, SourceRecord};

fn main() -> Result<(), EpError> {
    // 1. Track records. Baseline 50% worth 4 observations; weight halves every 100 time units.
    //    `true` = a claim this source made was later supported by an independent source.
    let mut record = SourceRecord::new(Params::new(0.5, 4.0, 100)?);
    for t in 0..30 {
        record.observe("agency", "physics", t % 10 != 0, t);
    }
    for t in 0..3 {
        record.observe("blog", "physics", false, t);
    }

    // 2. What the claim rests on. Both papers repeat the agency's report, so they are one origin.
    let mut graph = EvidenceGraph::new();
    graph.add_item("agency-report", "agency", &[])?;
    graph.add_item("paper-a", "journal-a", &["agency-report"])?;
    graph.add_item("paper-b", "journal-b", &["agency-report"])?;
    graph.add_item("blog-post", "blog", &["paper-b"])?;
    graph.add_item("own-measurement", "lab", &[])?;

    // 3. Each item's credence is its source's reliability on the topic (lab: no record yet).
    let now = 30;
    let items = [
        (
            "paper-a",
            record.reliability(&"journal-a", &"physics", now)?,
        ),
        (
            "paper-b",
            record.reliability(&"journal-b", &"physics", now)?,
        ),
        (
            "agency-report",
            record.reliability(&"agency", &"physics", now)?,
        ),
        ("blog-post", record.reliability(&"blog", &"physics", now)?),
        (
            "own-measurement",
            record.reliability(&"lab", &"physics", now)?,
        ),
    ];
    for (id, v) in &items {
        println!("{id:16} {:?}  ({})", v.value, v.reasons[0].rule);
    }

    // 4. Corroborate: the report, both papers and the blog are ONE cluster; the measurement is another.
    let ids: Vec<&str> = items.iter().map(|(i, _)| *i).collect();
    let clusters = graph.corroborate(&ids)?.value;
    println!(
        "independent pieces of evidence: {}",
        clusters.independent_count()
    );

    let pairs: Vec<(&str, Credence)> = items.iter().map(|(i, v)| (*i, v.value)).collect();
    let combined = graph.combine_corroborated(&pairs)?;
    println!("combined: {:?}", combined.value);
    for r in &combined.reasons {
        println!("  {} | {}", r.rule, r.note);
    }

    // 5. A name for the caller's display. The level names are the caller's data.
    let scale = LevelScale::new(
        [
            "low",
            "doubtful",
            "unproven",
            "corroborated",
            "well-corroborated",
        ]
        .map(String::from)
        .to_vec(),
    )?;
    match scale.to_level(&combined.value) {
        Ok(l) => println!("level: {}", scale.name(l).unwrap_or("?")),
        Err(e) => println!("no single level: {e}"), // an honest refusal, e.g. an interval across levels
    }
    Ok(())
}
