//! Evidence graph: items with a source and a derivation chain, an independence test, and
//! corroboration that never counts the same origin twice.
//!
//! Generic over the item id `I` and the source identity `S`: this crate knows nothing of what the
//! items claim or who the sources are. Everything is deterministic (ordered maps, no clock, no RNG).
//!
//! Two items are **dependent** (not independent evidence) when any of these holds, each reported as a
//! stable rule id in the verdict's reasons:
//! * `evidence.dependent.same-item`: the same item named twice (one origin seen twice);
//! * `evidence.dependent.mutual-citation`: each is in the other's ancestry;
//! * `evidence.dependent.ancestor`: one is in the other's ancestry (one derives from the other);
//! * `evidence.dependent.shared-ancestor`: their ancestries share an item, recorded or not;
//! * `evidence.dependent.shared-source`: some source appears in both ancestries, so a source cannot
//!   corroborate itself, nor lift its standing through items that merely pass its claim along.
//!
//! Ancestry of an item = itself plus everything reachable through `derived_from`, cycles allowed.
//! A `derived_from` id with no recorded item is an opaque origin: it still counts for shared ancestry.
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

pub use epistemologicant_core::{EpError, Reason, Verdict};
use epistemologicant_credence::{Credence, combine_independent};

struct Item<I, S> {
    source: S,
    derived_from: Vec<I>,
}

/// A set of evidence items with their sources and derivations.
pub struct EvidenceGraph<I, S> {
    items: BTreeMap<I, Item<I, S>>,
}

impl<I, S> Default for EvidenceGraph<I, S> {
    fn default() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }
}

/// The result of [`EvidenceGraph::corroborate`]: groups of mutually dependent items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Corroboration<I> {
    /// Clusters of dependent items. Each cluster counts as ONE piece of evidence. Clusters are
    /// ordered by their first member's position in the input; members keep input order.
    pub clusters: Vec<Vec<I>>,
}

impl<I> Corroboration<I> {
    /// How many independent pieces of evidence there really are.
    pub fn independent_count(&self) -> usize {
        self.clusters.len()
    }
}

impl<I: Ord + Clone + Debug, S: Ord + Clone + Debug> EvidenceGraph<I, S> {
    /// An empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an item, its source, and what it was derived from or cites. `derived_from` may name
    /// items not (yet) recorded.
    pub fn add_item(&mut self, id: I, source: S, derived_from: &[I]) -> Result<(), EpError> {
        if self.items.contains_key(&id) {
            return Err(EpError::DuplicateItem(format!("{id:?}")));
        }
        self.items.insert(
            id,
            Item {
                source,
                derived_from: derived_from.to_vec(),
            },
        );
        Ok(())
    }

    /// Number of recorded items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// True when no item is recorded.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn require(&self, id: &I) -> Result<(), EpError> {
        if self.items.contains_key(id) {
            Ok(())
        } else {
            Err(EpError::UnknownItem(format!("{id:?}")))
        }
    }

    /// The item and everything it derives from, transitively (cycle-safe), recorded or not.
    fn ancestry(&self, id: &I) -> BTreeSet<I> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![id.clone()];
        while let Some(cur) = stack.pop() {
            if !seen.insert(cur.clone()) {
                continue;
            }
            if let Some(item) = self.items.get(&cur) {
                stack.extend(item.derived_from.iter().cloned());
            }
        }
        seen
    }

    fn sources_of(&self, ancestry: &BTreeSet<I>) -> BTreeSet<S> {
        ancestry
            .iter()
            .filter_map(|i| self.items.get(i))
            .map(|it| it.source.clone())
            .collect()
    }

    /// Are two recorded items independent evidence? `true` only when none of the dependence rules
    /// fires; otherwise the reasons name every rule that did.
    pub fn independent(&self, a: &I, b: &I) -> Result<Verdict<bool>, EpError> {
        self.require(a)?;
        self.require(b)?;
        let mut reasons = Vec::new();
        if a == b {
            reasons.push(Reason::new(
                "evidence.dependent.same-item",
                format!("{a:?} named twice: one origin seen twice"),
            ));
            return Ok(Verdict {
                value: false,
                reasons,
            });
        }
        let (anc_a, anc_b) = (self.ancestry(a), self.ancestry(b));
        let (a_in_b, b_in_a) = (anc_b.contains(a), anc_a.contains(b));
        if a_in_b && b_in_a {
            reasons.push(Reason::new(
                "evidence.dependent.mutual-citation",
                format!("{a:?} and {b:?} each derive from the other"),
            ));
        } else if b_in_a {
            reasons.push(Reason::new(
                "evidence.dependent.ancestor",
                format!("{a:?} derives from {b:?}"),
            ));
        } else if a_in_b {
            reasons.push(Reason::new(
                "evidence.dependent.ancestor",
                format!("{b:?} derives from {a:?}"),
            ));
        }
        if let Some(x) = anc_a.intersection(&anc_b).find(|x| *x != a && *x != b) {
            reasons.push(Reason::new(
                "evidence.dependent.shared-ancestor",
                format!("{a:?} and {b:?} both derive from {x:?}"),
            ));
        }
        let (src_a, src_b) = (self.sources_of(&anc_a), self.sources_of(&anc_b));
        if let Some(s) = src_a.intersection(&src_b).next() {
            reasons.push(Reason::new(
                "evidence.dependent.shared-source",
                format!("source {s:?} appears in the ancestry of both {a:?} and {b:?}"),
            ));
        }
        if reasons.is_empty() {
            reasons.push(Reason::new(
                "evidence.independent",
                format!("{a:?} and {b:?} share no item, ancestor or source"),
            ));
        }
        Ok(Verdict {
            value: reasons.iter().all(|r| r.rule == "evidence.independent"),
            reasons,
        })
    }

    /// Group items into clusters of dependent evidence. Dependence is merged transitively (if a~b
    /// and b~c then a, b, c are one cluster): the cautious direction, it can undercount independent
    /// evidence but never overcount it. A repeated id is counted once (reason recorded).
    pub fn corroborate(&self, ids: &[I]) -> Result<Verdict<Corroboration<I>>, EpError> {
        if ids.is_empty() {
            return Err(EpError::Empty);
        }
        let mut reasons = Vec::new();
        let mut uniq: Vec<I> = Vec::new();
        for id in ids {
            self.require(id)?;
            if uniq.contains(id) {
                reasons.push(Reason::new(
                    "evidence.corroborate.duplicate-input",
                    format!("{id:?} given more than once; counted once"),
                ));
            } else {
                uniq.push(id.clone());
            }
        }
        let n = uniq.len();
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(p: &mut [usize], x: usize) -> usize {
            let mut r = x;
            while p[r] != r {
                r = p[r];
            }
            let mut c = x;
            while p[c] != r {
                let next = p[c];
                p[c] = r;
                c = next;
            }
            r
        }
        for i in 0..n {
            for j in (i + 1)..n {
                let v = self.independent(&uniq[i], &uniq[j])?;
                if !v.value {
                    let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                    if ri != rj {
                        // keep the lower index as root so cluster order follows input order
                        let (lo, hi) = (ri.min(rj), ri.max(rj));
                        parent[hi] = lo;
                    }
                    for r in v.reasons {
                        reasons.push(Reason::new(
                            "evidence.corroborate.merged",
                            format!("{}: {}", r.rule, r.note),
                        ));
                    }
                }
            }
        }
        let mut clusters: Vec<Vec<I>> = Vec::new();
        let mut slot: BTreeMap<usize, usize> = BTreeMap::new();
        for (i, id) in uniq.iter().enumerate() {
            let root = find(&mut parent, i);
            let k = *slot.entry(root).or_insert_with(|| {
                clusters.push(Vec::new());
                clusters.len() - 1
            });
            clusters[k].push(id.clone());
        }
        reasons.push(Reason::new(
            "evidence.corroborate.count",
            format!(
                "{} item(s) form {} independent cluster(s)",
                n,
                clusters.len()
            ),
        ));
        Ok(Verdict {
            value: Corroboration { clusters },
            reasons,
        })
    }

    /// Combine the credences of evidence items for one claim without counting the same evidence
    /// twice: inside a cluster of dependent items only the strongest counts (the bounds' maximum);
    /// across clusters the results combine as independent evidence (`combine_independent`).
    /// `Unknown` items add nothing; ordinal credences must be resolved first.
    pub fn combine_corroborated(
        &self,
        items: &[(I, Credence)],
    ) -> Result<Verdict<Credence>, EpError> {
        if items.is_empty() {
            return Err(EpError::Empty);
        }
        let ids: Vec<I> = items.iter().map(|(i, _)| i.clone()).collect();
        let cor = self.corroborate(&ids)?;
        let mut reasons = cor.reasons;
        let mut per_cluster = Vec::new();
        for cluster in &cor.value.clusters {
            let mut best: Option<(f64, f64)> = None;
            for id in cluster {
                // every item of the input that has this id (duplicates were collapsed by corroborate)
                for (_, c) in items.iter().filter(|(i, _)| i == id) {
                    c.validate()?;
                    let b = match *c {
                        Credence::Unknown => continue,
                        Credence::Point(p) => (p, p),
                        Credence::Interval { lo, hi } => (lo, hi),
                        Credence::Ordinal(_) => return Err(EpError::OrdinalNeedsScale),
                    };
                    best = Some(match best {
                        None => b,
                        Some((l, h)) => (l.max(b.0), h.max(b.1)),
                    });
                }
            }
            per_cluster.push(match best {
                None => Credence::Unknown,
                Some((l, h)) if l == h => Credence::Point(l),
                Some((lo, hi)) => Credence::Interval { lo, hi },
            });
        }
        let combined = combine_independent(&per_cluster)?;
        reasons.extend(combined.reasons);
        Ok(Verdict {
            value: combined.value,
            reasons,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type G = EvidenceGraph<&'static str, &'static str>;

    fn rules(v: &Verdict<bool>) -> Vec<&'static str> {
        v.reasons.iter().map(|r| r.rule).collect()
    }

    #[test]
    fn independent_items() {
        let mut g = G::new();
        g.add_item("a", "s1", &[]).unwrap();
        g.add_item("b", "s2", &[]).unwrap();
        let v = g.independent(&"a", &"b").unwrap();
        assert!(v.value);
        assert_eq!(rules(&v), ["evidence.independent"]);
    }

    #[test]
    fn same_item_is_one_origin() {
        let mut g = G::new();
        g.add_item("a", "s1", &[]).unwrap();
        let v = g.independent(&"a", &"a").unwrap();
        assert!(!v.value);
        assert_eq!(rules(&v), ["evidence.dependent.same-item"]);
    }

    #[test]
    fn a_source_cannot_corroborate_itself() {
        let mut g = G::new();
        g.add_item("a", "s1", &[]).unwrap();
        g.add_item("b", "s1", &[]).unwrap();
        let v = g.independent(&"a", &"b").unwrap();
        assert!(!v.value);
        assert_eq!(rules(&v), ["evidence.dependent.shared-source"]);
    }

    #[test]
    fn ancestor_either_direction() {
        let mut g = G::new();
        g.add_item("orig", "s1", &[]).unwrap();
        g.add_item("copy", "s2", &["orig"]).unwrap();
        for (x, y) in [("orig", "copy"), ("copy", "orig")] {
            let v = g.independent(&x, &y).unwrap();
            assert!(!v.value);
            assert!(
                rules(&v).contains(&"evidence.dependent.ancestor"),
                "{:?}",
                rules(&v)
            );
            assert!(!rules(&v).contains(&"evidence.dependent.shared-ancestor"));
        }
    }

    #[test]
    fn shared_ancestor_including_an_unrecorded_origin() {
        let mut g = G::new();
        g.add_item("a", "s1", &["press-release"]).unwrap();
        g.add_item("b", "s2", &["press-release"]).unwrap();
        let v = g.independent(&"a", &"b").unwrap();
        assert!(!v.value);
        assert_eq!(rules(&v), ["evidence.dependent.shared-ancestor"]);
    }

    #[test]
    fn shared_source_deep_in_the_chain() {
        let mut g = G::new();
        g.add_item("x", "agency", &[]).unwrap();
        g.add_item("a", "s1", &["x"]).unwrap();
        g.add_item("b", "s2", &["x"]).unwrap();
        g.add_item("c", "s3", &["b"]).unwrap();
        let v = g.independent(&"a", &"c").unwrap();
        assert!(!v.value);
        assert!(rules(&v).contains(&"evidence.dependent.shared-source"));
    }

    #[test]
    fn mutual_citation_and_cycles_terminate() {
        let mut g = G::new();
        g.add_item("a", "s1", &["b"]).unwrap(); // forward reference
        g.add_item("b", "s2", &["a"]).unwrap();
        let v = g.independent(&"a", &"b").unwrap();
        assert!(!v.value);
        assert!(rules(&v).contains(&"evidence.dependent.mutual-citation"));
        // a longer ring: x -> y -> z -> x
        g.add_item("x", "t1", &["y"]).unwrap();
        g.add_item("y", "t2", &["z"]).unwrap();
        g.add_item("z", "t3", &["x"]).unwrap();
        assert!(!g.independent(&"x", &"z").unwrap().value);
    }

    #[test]
    fn unknown_and_duplicate_items() {
        let mut g = G::new();
        g.add_item("a", "s", &[]).unwrap();
        assert_eq!(
            g.add_item("a", "s", &[]),
            Err(EpError::DuplicateItem("\"a\"".into()))
        );
        assert_eq!(
            g.independent(&"a", &"nope"),
            Err(EpError::UnknownItem("\"nope\"".into()))
        );
        assert_eq!(g.corroborate(&[]), Err(EpError::Empty));
    }

    #[test]
    fn sources_that_only_cite_each_other_count_once() {
        let mut g = G::new();
        g.add_item("a", "s1", &["b"]).unwrap();
        g.add_item("b", "s2", &["a"]).unwrap();
        g.add_item("c", "s3", &[]).unwrap();
        let v = g.corroborate(&["a", "b", "c"]).unwrap();
        assert_eq!(v.value.clusters, vec![vec!["a", "b"], vec!["c"]]);
        assert_eq!(v.value.independent_count(), 2);
    }

    #[test]
    fn clustering_is_transitive_and_ordered() {
        let mut g = G::new();
        g.add_item("p", "s1", &["u"]).unwrap();
        g.add_item("q", "s2", &["u", "w"]).unwrap();
        g.add_item("r", "s3", &["w"]).unwrap();
        g.add_item("z", "s9", &[]).unwrap();
        // p~q (u), q~r (w); p and r are not directly dependent but share a cluster
        assert!(g.independent(&"p", &"r").unwrap().value);
        let v = g.corroborate(&["z", "r", "p", "q"]).unwrap();
        assert_eq!(v.value.clusters, vec![vec!["z"], vec!["r", "p", "q"]]);
    }

    #[test]
    fn repeated_ids_count_once() {
        let mut g = G::new();
        g.add_item("a", "s1", &[]).unwrap();
        let v = g.corroborate(&["a", "a", "a"]).unwrap();
        assert_eq!(v.value.independent_count(), 1);
        assert!(
            v.reasons
                .iter()
                .any(|r| r.rule == "evidence.corroborate.duplicate-input")
        );
    }

    fn pt(p: f64) -> Credence {
        Credence::point(p).unwrap()
    }

    #[test]
    fn the_same_evidence_is_never_counted_twice() {
        let mut g = G::new();
        g.add_item("orig", "s1", &[]).unwrap();
        g.add_item("copy", "s2", &["orig"]).unwrap();
        g.add_item("other", "s3", &[]).unwrap();
        // original and its copy: one cluster, strongest only
        let v = g
            .combine_corroborated(&[("orig", pt(0.5)), ("copy", pt(0.5))])
            .unwrap();
        assert_eq!(v.value, pt(0.5));
        // truly independent items corroborate: noisy-OR
        let v = g
            .combine_corroborated(&[("orig", pt(0.5)), ("other", pt(0.5))])
            .unwrap();
        assert_eq!(v.value, pt(0.75));
        // a cluster contributes its maximum
        let v = g
            .combine_corroborated(&[("orig", pt(0.2)), ("copy", pt(0.6)), ("other", pt(0.5))])
            .unwrap();
        assert_eq!(v.value, pt(0.8));
    }

    #[test]
    fn unknown_adds_nothing_and_ordinal_is_refused() {
        let mut g = G::new();
        g.add_item("a", "s1", &[]).unwrap();
        g.add_item("b", "s2", &[]).unwrap();
        let v = g
            .combine_corroborated(&[("a", Credence::Unknown), ("b", Credence::Unknown)])
            .unwrap();
        assert_eq!(v.value, Credence::Unknown);
        let v = g
            .combine_corroborated(&[("a", Credence::Unknown), ("b", pt(0.4))])
            .unwrap();
        assert_eq!(v.value, pt(0.4));
        assert_eq!(
            g.combine_corroborated(&[("a", Credence::Ordinal(1))]),
            Err(EpError::OrdinalNeedsScale)
        );
        assert_eq!(
            g.combine_corroborated(&[("a", Credence::Point(3.0))]),
            Err(EpError::OutOfRange(3.0))
        );
    }

    #[test]
    fn works_with_other_id_and_source_types() {
        let mut g: EvidenceGraph<u32, (u8, String)> = EvidenceGraph::new();
        g.add_item(1, (0, "x".into()), &[]).unwrap();
        g.add_item(2, (0, "y".into()), &[1]).unwrap();
        assert!(!g.independent(&1, &2).unwrap().value);
    }
}
