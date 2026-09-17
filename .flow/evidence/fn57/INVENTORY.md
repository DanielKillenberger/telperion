# fn-57: the fragile-code sweep, 2026-09-16

A host sweep over tooling, QA scripts, evidence scripts and tests, excluding the generator, the renderer and every preset-reaching path. Ranked by how much a typed judgment would change the outcome. Dispositions: `jev` (a judgment earns its place), `code` (a plain code fix, no model), `leave` (sound as is, or the interpretation is the owner's).

| # | Site | What it does today | Why it is fragile | Disposition | Judgment shape |
|---|---|---|---|---|---|
| 1 | Species QA runner, visual slot | Captures every still, writes `visual_status: unassessed`, exits 1 until a human writes the report | The acceptance gate is an empty slot that no rerun can fill | leave | Stills need the owner's eye; Jev judges text and state, never an image |
| 2 | QA "viewer sees" notes and QA receipts | Owner feedback becomes a titled finding with severity, introduced-or-pre-existing and a constant confidence, by agent narration | No code judges routing, duplication or severity | jev | Choice over open specs; Noul same-defect against prior findings; Score severity against the owner's recorded standard. Pilot: 8/8 routing, true duplicate pairs above every negative, 4/4 severity |
| 3 | Species QA runner, headless stdout | Regex over the renderer's last stdout line for triangles, instances and draw calls | One word changed in a println turns every capture into a failure | code | Renderer emits a JSON line; the runner parses it |
| 4 | Foliage reference test | Finds a JSON key by string split and scans characters for a number; provenance by substring | Hand-rolled JSON parser inside a correctness test | code | Use the workspace's JSON crate |
| 5 | Growth report script | Slices the older report between two literal substrings to lift its reference table | A sentence edit in the old report breaks the new one | jev | Choice over section headings with the section bodies as state. Pilot: 3/3 at 0.99 or higher |
| 6 | Growth report and convergence scripts | A 15 percent cliff becomes a tolerance miss; two presets get a bounds verdict, three get a literal string | A hardcoded cliff plus a species carve-out | leave | The tolerance is the spec's rule and the miss is the owner's decision; the carve-out is a code cleanup outside this spec |
| 7 | Species metrics compare | Pass, fail or unassessed from hand-authored gating or contextual classification and ranges | A range authored from a description decides a numeric verdict | jev, later | Score agreement between a measurement and the cited source text; needs the literature screen first |
| 8 | Geometry benchmark admission | Admitted, missing-evidence or unsupported-anatomy from hash equality, name normalisation and three non-empty fields | "A usable attributed real reference" reduced to field presence | jev, later | Noul: does this reference record support the species' claimed scales |
| 9 | Onboarding examples verify script | Coordination state by kind checks; any non-empty string counts as expert feedback | Hand-written classification of workflow state | leave | Fixture code for a frozen protocol |
| 10 | Flow config clean-review pattern | Regex over reviewer prose decides whether a PR review is clean | Any phrasing change flips the landing gate | leave | Plugin-owned setting; review backend is off in this repo |
| 11 | fn9 scaffold-reach and articulation audits | Reachability sums and shortfall sorting with caveats stored as a prose blob | Geometry is sound; the interpretation is the heuristic | leave | Historical experiment |
| 12 | Species occupancy script | Ad-hoc fractions over 50 mm bands; the method string disclaims being a gate | A human reads the numbers | leave | Historical experiment |
| 13 | fn9 allocation audit | Downward-cosine and group-size literals define a drooping secondary | Two literals define a botanical class | leave | Historical experiment |
| 14 | Scenic cut script | Regex frame discovery, a 24 second limit and a grade string as literals; probe returns null on failure | Taste encoded as constants; a failed probe reports success | code | Fail loud on a failed probe; the constants are the owner's |
| 15 | Generation benchmark and migration compare | Hash and count equality; the report has no notion of acceptable drift | Correctly deterministic; only the report is heuristic | leave | Drift acceptance is the owner's |
| 16 | Species onboarding docs and profile template | Evidence obligations as prose with no executable check | No check that a filled packet meets its obligation | jev | The literature screen and the citation check (R3, R4) are the executable check for the numeric obligations. Pilot: 12/12 sentence kinds, O1 listed for the owner |

## How the QA pipeline records a verdict today

Two artefacts. The QA receipt is the machine verdict with open findings, each carrying severity, classification, confidence and a status. The outcome file is the human slot with owner verdict slots that stay null until the owner writes. Owner complaints are transcribed by hand into bug memory notes with YAML front matter and Problem, Expected, Actual and Evidence prose. Every severity, classification and confidence field is filled by a person or an agent's narration.

## How species numbers are sourced today

Yield tables and papers are transcribed by hand into the growth curve script, composed into age curves, fitted, then frozen into the profiles file with hand-authored confidence and a gating-or-contextual classification per dimension. The reference inventories record the reading, including corrections of OCR unit errors. Checking is a 15 percent tolerance at three ages plus a human eye. No step checks a cited sentence against the claim it backs.
