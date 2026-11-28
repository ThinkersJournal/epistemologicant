# Patent sweep: epistemologicant (argument-structure and defeater family) @ branch `next`

Date: 2026-11-21; Run by: epistemologicant lane, Sonnet 5.5; Method: AI-assisted web search (US-only `WebSearch`, restricted to
patents.google.com / patents.justia.com), then the independent claims of the closest hits read with `tools/patent-claims.py`
(Google Patents pages); NOT legal advice; NOT a freedom-to-operate opinion. There is no attorney. Process:
`C:\Projects\PATENT-SWEEP-PROCESS.md`; format follows redactant's `PATENT-SWEEP.md`.

**Status: first pass COMPLETE for the routes run; no hit read overlaps an independent claim as read, so none is `needs-review`.** Every verdict below, `outside as read`, is a model reading of machine-converted claim text, not legal advice and not a clearance; the PM read this file and filed board 480 (2026-11-21); the publish hold was lifted by CireSnave 2026-11-27 (board 522 A) on these limits and the ROADMAP watch list. Routes NOT run: USPTO full-text, Espacenet, WIPO, assignee
searches, non-US grants. Requested by pm-ants (2026-11-21) as a short "outside as read" sweep before argument-structure code.

**Scope, and what is NOT covered.** This sweep covers only the family now being built or just built: ranked reasons with rebutting
and undercutting defeaters (`epistemologicant-defeat`) and a Dung-style argument structure (next, unbuilt). The earlier tools
(`-credence`, `-evidence`, `-justify`, `-record`) are covered by the second-pass section at the end of this file.

## Ideas swept
1. A computer stores arguments and an attack relation between them and labels each argument in, out or undecided as the least
   fixed point of a defence (characteristic) function (the grounded labelling).
2. Reasons for claims carry a rank; a rebuttal succeeds only if the rebutter is live and strictly stronger; equal ranks leave both
   undecided.
3. Two kinds of defeater: a rebutter (a reason for a contradictory claim) and an undercutter (a claim that cuts a reason's link to its
   claim); a claim is out when every reason is dead, in when one is live, with a stated reason for each verdict.
4. Retracting a reason returns exactly the claims whose standing changed.
5. A claim supported only by a cycle of claims that cite each other is out (well-founded support).
6. Enumerating preferred, stable and complete extensions of an abstract framework (planned).

## Queries (source: WebSearch limited to patents.google.com unless noted; US-only; date 2026-11-21)
| # | Idea | Query |
|---|---|---|
| Q1 | 1 | `patent computing system argumentation framework attack relation grounded extension least fixed point label arguments in out undecided` |
| Q2 | 3 | `patent defeasible reasoning rebutting defeater undercutting defeater argument status determined computer-implemented decision support` |
| Q3 | 2 | `patent preference-based argumentation attack succeeds unless target argument preferred strength of argument conflict resolution automated` |
| Q4 | 4, 5 | `patent truth maintenance system justification retract assumption propagate set of beliefs that lost support` |
| Q5 | 3 | `patent argument graph supporting and attacking evidence nodes computing acceptability of claim explanation of why claim accepted or rejected` |
| Q6 | 4 | `patent incremental update of argumentation framework when argument removed report arguments whose acceptance status changed` |
| Q7 | 1, 6 | `patent method computer system arguments attack relation determine accepted arguments extension semantics preferred stable grounded claim` (domains: patents.google.com, patents.justia.com) |
| Q8 | 2 | `patent source reliability evidence conflict resolution defeat strength-ranked contradictory claims knowledge base accept claim with higher rank explain decision` |

Positive control: Q2 returned US5706406 (a real defeasible-reasoning patent whose background names both defeat types), and Q6
returned US6098062 and US10817670B2, so the search reaches the field. Q1, Q3, Q5 and Q7 returned **no patent** at all (only papers):
that is "no hit surfaced", not "none exists". Q1's papers (Dung-style grounded semantics, Knaster-Tarski least fixed point) and Q3's
(preference-based argumentation frameworks, attack reversal; Amgoud and Cayrol 1998, named in the results, not retrieved) show ideas 1
and 2 are long-published academic work, so no novelty is claimed in either.

## Hits
Verdict vocabulary: `outside as read` = on the independent claims I read, at least one limitation we lack is required (the model reading, not a clearance). Claims read: every independent claim from Google Patents text (machine-converted; check the official record before quoting). Status
is Google Patents' field, not the official record.

| # | patent | title | assignee | status | claims read | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US5706406A | Architecture for an artificial agent that reasons defeasibly | individual (Pollock; assignee field "Individual") | Expired - Fee Related; priority 1995-05-22 | 1, 19, 20, 21, 27, 28, 29 | defeasible reasoning with defeat status computed for nodes of an inference graph | claims 1 and 21 need generating conclusions by monotonic reasoning into an inference-graph, **computing a list of status-assignments**, and judging a node undefeated by every assignment; claims 19, 20 and 27 are plan adoption and execution; 28 and 29 are first-order reasoning with unification and reductio. We have no inference queue, no status-assignment enumeration, no plans, no unification | outside as read (expired; and different on every independent claim) |
| 2 | US6098062A | Argument structure hierarchy system and method for facilitating analysis and decision-making | individual | Expired - Fee Related; priority 1997-01-17 | 1 | a data structure of arguments for and against a hypothesis | needs linking argument-structure units of hypothesis/counter-hypothesis into a hierarchy, accepting inputs from remote contributors, degrees of support and a visual display. We take no contributor input, hold no hypothesis pairs and display nothing | outside as read (expired) |
| 3 | US10740682B2 | Sensor based truth maintenance | IBM, now Kyndryl | Active; est. 2033-12-15 | 1, 8, 14 | a truth-maintenance database and non-monotonic logic that retracts assumption data | every independent claim needs RFID tags in sensors supplying event data, a cloud mediation system, Dempster-Shafer theory over sets of assumptions with plausibility percentages, and generating recommendations. We have no sensors, no plausibility percentages and no Dempster-Shafer | outside as read (watch: do not add sensor-event ingestion with Dempster-Shafer plausibility recommendations) |
| 4 | US10817670B2 | Enabling chatbots by validating argumentation | Oracle | Active; est. 2039-04-26 | 1 | extracts defeasible rules and checks an argument's consistency | every independent claim starts from text: build a discourse tree, a communicative discourse tree with verb signatures, a trained classifier to find an argument, then a logic system from it. We process no text | outside as read (watch: no discourse-tree or text-extraction front end) |
| 5 | US11036935B2 | Argument structure extension device, method, program, and data structure | NTT | Active | 1, 8, 15 | an argumentation structure of nodes and links | claims generate new propositions from a template tied to an argumentation scheme by rewriting words. We generate no propositions and use no schemes or templates | outside as read (watch: no text-generation of arguments) |

Seen in results and NOT read (unread, no verdict): US4918621 (ATMS graph of worlds), EP0346128A2 (ATMS on parallel processors),
US20120075067A1 (sibling of hit 3), US7552104B2 and US6151565A (decision support; excerpts showed no defeasible reasoning),
US9715488B2, US9665564B2, US9904668B2, US9588961B2 (natural-language processing patents that adjust a source's relevance on
conflicting facts), US5694590A (plausibility rules over reliability-tagged sources in a secure database), US20090282019A1,
US20110010344A1, US20080114795A1, US8275745B2.

## Limitations
AI-assisted search over a US-only web index; coverage is partial and the queries are in this file. Four of eight queries returned no
patents. Hits read from machine-converted claim text. Grants outside the US, USPTO full text, Espacenet and WIPO were not searched.
The unread list above has no verdict. Ideas 1 and 2 rest on public academic literature, so the sweep looked for claims that
*apply* them, not for novelty. A lawyer's freedom-to-operate review is a separate purchase.

## Watch list (do not add without a re-sweep)
Status-assignment enumeration over an inference graph with plan reasoning (US5706406 family, expired but cite); sensor/RFID event
ingestion with Dempster-Shafer; text-to-argument extraction or discourse trees; generating propositions from argumentation schemes.

## Second pass: `-credence`, `-evidence`, `-justify`, `-record` (2026-11-21, @ `next`)
Run by the epistemologicant lane, Sonnet 5.5; same method, same limits as above (US-only `WebSearch` limited to patents.google.com /
patents.justia.com, claims read with `tools/patent-claims.py`; NOT legal advice; NOT a freedom-to-operate opinion; no USPTO full text,
Espacenet or WIPO). Verdicts are `outside as read`, never `clear`. Board 480 (CireSnave: "Go with your recommendation", relayed by pm-ants).
**Reading limit (closed 2026-11-27):** the first reading cut the claim reader's output at 3,500 characters per patent, so only claim 1 (and the
start of 7 for US8682913B1) was read. On 2026-11-27 a Sonnet subagent re-fetched each patent's Google Patents page and read ALL claims as page
text (machine-converted, not the USPTO wrapper): every independent claim of the five hits is now read and carries the verdict below. The verdicts
rest on that Google Patents text only.

### Ideas swept
7. Credence that distinguishes unknown from zero and carries a point, an interval or an ordinal level; independent claims combine by
   noisy-OR (exact on intervals) and conflicting claims by hull or mean (`-credence`).
8. Evidence items carry a source and derivation chain; two items are judged independent unless one is the other, cites the other, shares an
   ancestor or shares a source, with a reason on every verdict; corroboration counts each dependent cluster once (`-evidence`).
9. A claim's status is held, unsupported or contradicted from premise and inference grounds; a cycle cannot support itself; retracting a
   ground returns exactly the claims that lost support (`-justify`; overlaps idea 4 and 5 above).
10. Per-source, per-topic reliability from observed outcomes, shrunk toward a baseline while data is thin, with a recency half-life
    weight and sub-topic pooling (`-record`).

### Queries (WebSearch, patents.google.com / patents.justia.com, US-only, 2026-11-21)
| # | Idea | Query |
|---|---|---|
| Q9 | 7 | `patent combining independent evidence probabilities noisy-OR interval credence uncertain claim confidence computer system` |
| Q10 | 8 | `patent determine independence of evidence sources shared source common ancestor citation provenance avoid double counting corroboration` |
| Q11 | 9 | `patent justification tracking retract premise returns set of conclusions that lost support cycle cannot support itself` |
| Q12 | 10 | `patent source reliability estimate per topic from observed outcomes shrinkage toward prior recency half-life weighting` |

Positive control: each query returned real patents (Q9 US8041632, US10430722; Q10 US8682913B1, US7421419B2, US10318870B2; Q11 US5276775A,
US10740682B2; Q12 US20240248963A1, US9342790). Q12 surfaced no patent on shrinkage of per-topic reliability toward a prior: that is "no hit
surfaced" from one query, not "none exists". Q11 surfaced no hit on cycle handling.

### Hits (claim 1 read unless stated)
| # | patent | title | assignee | status | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|
| 6 | US8682913B1 | Corroborating facts extracted from multiple sources | Google | Active; est. 2033-11-08 | corroborating a fact across multiple sources | claim 1 needs source documents, attribute-value pairs for a common subject, predefined corroboration requirements and updating a status in a fact repository. We take no documents or attribute-value pairs and keep no fact repository; we group dependent evidence rather than count documents | outside as read (all independent claims 1, 7, 8, 14, 15, 21 read; each needs source documents / attribute-value pairs and a fact-repository status update) |
| 7 | US7421419B2 | Evidence accumulation and hypothesis generation | Viziant | Expired - Fee Related | accumulating evidence against a threshold | claim 1 needs an evidence processor matching an extracted entity against reference entities, a total evidence function and a context-dependent threshold. We match no entities and use no threshold | outside as read (expired; all 9 independent claims 1, 12, 20, 27, 32, 35-38 read; each needs a threshold processor generating a context-dependent threshold and a validation/refutation degree over matched entities / feature vectors) |
| 8 | US10318870B2 | Grading sources and managing evidence for intelligence analysis | IBM | Active; est. 2037-12-30 | grading a source's reliability | claim 1 needs a corpora management engine, a source profile directory, a grading measurement, thresholds and removing a source's evidence from a corpus. We have no corpus, no directory and no threshold removal; ours is per-topic from outcomes with shrinkage | outside as read (watch: no corpus management or threshold-based evidence removal; independent claims 1, 10, 18 read: all require the corpora management engine, source profile directory and below-threshold removal) |
| 9 | US10430722B2 | Learning parameters in a feed forward probabilistic graphical model | IBM | Expired - Fee Related | evidence graph with node confidence from parents | claim 1 needs natural-language training scenarios, factors extracted into evidence graphs, learned weights and recursive confidence formulas. We learn nothing and parse no language | outside as read (sole independent claim 1 read; needs training scenarios, evidence-graph construction and learned weights) |
| 10 | US5276775A | System and method for building knowledge-based applications | Texas Instruments | Expired - Lifetime | truth maintenance of logical dependencies | claim 1 needs a TMS containing both a deductive reasoning system and a constraint satisfaction system over declarative statements. We have neither | outside as read (expired; independent claims 1, 17, 33 read: each needs a TMS containing a deductive reasoning system and a constraint satisfaction system) |

Seen-and-not-read list, worked 2026-11-27 (Google Patents page text, every independent claim read; same method and limits as the five hits above):

| patent | assignee; status on page | independent claims read | our difference | verdict |
|---|---|---|---|---|
| US8041632B1 | Citibank; Active, exp. 2027-09-13 | 1 | claim 1 needs a Bayesian network of implications over variables with initial distribution hypotheses, probabilistic induction of a cause of a changed value, and a plausibility-of-data-error determination. We build no Bayesian network and diagnose no data errors | outside as read |
| US20120004949A1 | Citibank; family record shows lapse for non-payment 2025-01-27 | 10, 19, 21, 24 (1-9 cancelled) | each needs a financial-risk-assessment system with a Bayesian network (plus factoids / exposure profile / virtual assistant in 19, 21, 24). We have none | outside as read |
| EP1788461A1 | Multitel; Expired, 2026-04-21 | 1, 21, 22, 23 | each needs measurement specifications with confidence regions and levels, and a configuration that guarantees a target confidence level for a sensor arrangement. We design no sensor arrangements | outside as read |
| US7318808B2 | Oxford Univ. Innovation; Expired | 1, 18, 20 | each needs two independent breathing-rate measurement channels with predicted-vs-measured differences setting combination weights (Kalman in 20). We take no measurements | outside as read |
| US7949581B2 | PatentRatings; Expired (family litigation flagged on page) | 1, 3, 5 | each needs patent forward-citation data fitted by a lognormal ramp and an exponential decay to build a depreciation schedule. Unrelated to our tools | outside as read |

Seen and still NOT read (no verdict; the reader did not reach the claims, or only a tool summary exists): US9342790B1 (IBM/Kyndryl, half-life forecast
combination), US4918621A (IntelliCorp ATMS graph of worlds), EP0346128A2 (Xerox, parallel propositional reasoning), US11204929B2, US11238351B2
and US9727642B2 (IBM evidence-aggregation family; US10318870B2 is a sibling, and its independent claims were read above).

### Limits specific to this pass
One query per idea; independent claims of the five hits all read (Google Patents text, 2026-11-27), and five of the eleven seen-and-not-read patents are now read too; six stay unread (listed above); US-only index; no assignee, citation-chain or
classification search. `-credence`'s noisy-OR and interval arithmetic are textbook mathematics; the sweep looked for claims that apply
them, not for novelty. Watch list additions: no document or attribute-value fact extraction front end; no corpus management that removes a
source's evidence on a threshold; no learned evidence-graph weights; no entity-matching threshold decision.
