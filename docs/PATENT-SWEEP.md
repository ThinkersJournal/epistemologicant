# Patent sweep: epistemologicant (argument-structure and defeater family) @ branch `next`

Date: 2026-11-21; Run by: epistemologicant lane, Sonnet 5.5; Method: AI-assisted web search (US-only `WebSearch`, restricted to
patents.google.com / patents.justia.com), then the independent claims of the closest hits read with `tools/patent-claims.py`
(Google Patents pages); NOT legal advice; NOT a freedom-to-operate opinion. There is no attorney. Process:
`C:\Projects\PATENT-SWEEP-PROCESS.md`; format follows redactant's `PATENT-SWEEP.md`.

**Status: first pass COMPLETE for the routes run; no `needs-review`.** Routes NOT run: USPTO full-text, Espacenet, WIPO, assignee
searches, non-US grants. Requested by pm-ants (2026-11-21) as a short "outside as read" sweep before argument-structure code.

**Scope, and what is NOT covered.** This sweep covers only the family now being built or just built: ranked reasons with rebutting
and undercutting defeaters (`epistemologicant-defeat`) and a Dung-style argument structure (next, unbuilt). The earlier tools
(`-credence`, `-evidence`, `-justify`, `-record`) have **not** been swept; that is owed before any publish under the portfolio rule.

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
Claims read: every independent claim from Google Patents text (machine-converted; check the official record before quoting). Status
is Google Patents' field, not the official record.

| # | patent | title | assignee | status | claims read | overlap | our difference | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | US5706406A | Architecture for an artificial agent that reasons defeasibly | individual (Pollock; assignee field "Individual") | Expired - Fee Related; priority 1995-05-22 | 1, 19, 20, 21, 27, 28, 29 | defeasible reasoning with defeat status computed for nodes of an inference graph | claims 1 and 21 need generating conclusions by monotonic reasoning into an inference-graph, **computing a list of status-assignments**, and judging a node undefeated by every assignment; claims 19, 20 and 27 are plan adoption and execution; 28 and 29 are first-order reasoning with unification and reductio. We have no inference queue, no status-assignment enumeration, no plans, no unification | clear (expired; and different on every independent claim) |
| 2 | US6098062A | Argument structure hierarchy system and method for facilitating analysis and decision-making | individual | Expired - Fee Related; priority 1997-01-17 | 1 | a data structure of arguments for and against a hypothesis | needs linking argument-structure units of hypothesis/counter-hypothesis into a hierarchy, accepting inputs from remote contributors, degrees of support and a visual display. We take no contributor input, hold no hypothesis pairs and display nothing | clear (expired) |
| 3 | US10740682B2 | Sensor based truth maintenance | IBM, now Kyndryl | Active; est. 2033-12-15 | 1, 8, 14 | a truth-maintenance database and non-monotonic logic that retracts assumption data | every independent claim needs RFID tags in sensors supplying event data, a cloud mediation system, Dempster-Shafer theory over sets of assumptions with plausibility percentages, and generating recommendations. We have no sensors, no plausibility percentages and no Dempster-Shafer | clear (watch: do not add sensor-event ingestion with Dempster-Shafer plausibility recommendations) |
| 4 | US10817670B2 | Enabling chatbots by validating argumentation | Oracle | Active; est. 2039-04-26 | 1 | extracts defeasible rules and checks an argument's consistency | every independent claim starts from text: build a discourse tree, a communicative discourse tree with verb signatures, a trained classifier to find an argument, then a logic system from it. We process no text | clear (watch: no discourse-tree or text-extraction front end) |
| 5 | US11036935B2 | Argument structure extension device, method, program, and data structure | NTT | Active | 1, 8, 15 | an argumentation structure of nodes and links | claims generate new propositions from a template tied to an argumentation scheme by rewriting words. We generate no propositions and use no schemes or templates | clear (watch: no text-generation of arguments) |

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
