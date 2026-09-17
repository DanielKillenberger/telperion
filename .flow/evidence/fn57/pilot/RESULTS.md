# TypeSafe (Jev) pilot on Telperion evidence, 2026-09-16

Model `jev-latest`, `POST /v1/systemone`, key from `TYPESAFE_API_KEY` through `bash -ic`.
Every request answered in 0.56 to 0.78 s. Input 500 to 5,500 tokens; output 17 to 816.
Scripts: `jev.py` (caller), `build_exp.py` (exp 1 to 3), `build_exp2.py` (exp 4 and 5).
Sources fetched raw with curl into `refs/` (OWIC oak page, Iowa State spruce page, NCTA spruce page).

## Exp 1: screen candidate sentences from a source (12 sentences, 3 questions each)

Questions over `{source, candidate.sentence, candidate.context}`:
`kind` Choice (measured_size_at_age | site_quality_criterion | mature_size_range | typical_growth_rate | sprout_cultivar_or_nursery | not_about_tree_size),
`condition` Choice (open_grown | stand_grown | plantation_or_nursery | unstated),
`anchor_usable` Noul.

Result: 12 of 12 on `kind`, all at confidence 0.94 or higher.
The O1 sentence "Sustained height growth of 1 to 2 ft per year for trees 10 to 30 years old" scored
site_quality_criterion 0.99, stand_grown 0.87, anchor_usable 0.05.
The two true anchors (Iowa State "75 feet in 50 years", NCTA "8 to 11 years ... 6-7 foot tree") were the
only sentences with anchor_usable above 0.5 (0.80 and 0.87). NCTA condition: plantation_or_nursery 0.99.
`condition` was the weakest question: 0.54 confidence on the stump-sprout sentence and 0.39 on the
elevation sentence, both cases where condition is not applicable.

## Exp 2: citation check of research-section claims (9 claims)

Question over `{source, claim, section}`: `relation` Choice (supports | contradicts | says_nothing), the
docs' citation_check pattern.

Result: 6 true claims all `supports` at 0.98 to 1.00; the planted false claim (6-7 ft within five years)
`contradicts` 0.84.
The O1 misuse claim ("open-grown Oregon white oak reaches 2.4 m at ten years" against the site-criterion
sentence) answered `supports` 0.50 with confidence 0.26: under any auto-accept threshold it routes to a
person, which is the right outcome.
Miss: "Oregon white oak typically grows 1 to 2 ft per year between ages 10 and 30" against the same
sentence answered `supports` at 1.00. A citation check alone cannot see that the source frames the
number as a site test. The `kind` screen of exp 1 catches it. Composition rule: a number passes only
when the screen says it is a measurement and the citation check says the claim is supported.

## Exp 3: select a pre-parsed number (2 documents, 5 questions)

Code regex finds numeric spans with a unit; each is a Choice option plus `none`.

Result after a coverage fix: 5 of 5 (50 to 90 ft; none for height at ten years, 0.64; 500 years;
6-7 foot 0.93; 8 to 11 years 1.00).
First run missed because the recall regex lacked the word "foot", so "6-7 foot" was not an option; Jev
answered none 0.49 against "8 to 11 years" 0.46, a visible coverage failure rather than a wrong copy.
Rule: test candidate coverage on the recall regex before trusting a `none`.

## Exp 4: owner-feedback triage (8 routing, 7 duplicate, 4 severity)

Routing: Choice over the 12 open specs plus new_spec, state `{owner_observation, open_specs}`.
8 of 8 on the top answer. Two were low confidence and correctly so: the 20-year crown hollow
(fn-31 0.48, fn-29 0.26) and the owner's spruce quote "the branching looks weird and too straight"
(fn-31 0.65, fn-21 0.28), which QA itself filed under fn-31 with fn-21 as the neighbour.

Duplicate detection: Noul `same_defect` over `{new_observation, prior_finding}`.
The two true pairs (leafy bush vs round-5 shrub 0.79; bottle flare vs round-6 bell 0.82) were the only
pairs above 0.5. Clear negatives 0.10 to 0.13. The two "related but separately filed" pairs sat in the
middle (tuft vs umbrella 0.29; gangly spruce vs straight rays 0.26).

Severity: Score with three described levels, state `{observation, standard}`.
Hollow 0.01 (cosmetic; the note itself says minor), leafy bush 2.00, bottle flare 1.99, spruce rays
1.79 at 0.68 confidence. All four as expected.

## Exp 5: report section selection (fn30 REPORT.md, 13 sections, 3 questions)

Choice over section headings with the first 700 characters of each as state.
3 of 3 at 0.99 or higher (The references; Owner verdict; The curves and their composition).
This is the shape that replaces `report.py`'s `old.index('| ID |'):old.index('U1 lists')` slice.

## Not Jev cases found by the sweep

- `crates/telperion-core/tests/foliage_reference.rs:20-30` hand-rolled JSON number scan: use serde_json.
- `tests/species.mjs:86` regex over the headless renderer's last stdout line: make the renderer emit a
  JSON line and parse it.
