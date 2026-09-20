# Tuning loop: measured candidates and automated visual readiness

## Conversation Evidence

> user: "i want you to review all specs that reload to the add-species pipeline. Find ways to simplify and make it as efficient to run as possible. Make sure we"
> user: "make sure we're aligned on the goals with this pipeline. See if you can understand my goals/targets clearly and make sure we're on the same page"
> user: "ideally i'd like the whole loop to be basically autonomous with a lot of judgement being done quickly and implementation to close gaps being done and I can just come back to review a new species template that looks awesome and what I would expect and i can just tick it off."
> user: "i also want you to evaluate how well jev fits into this pipeline. I feel a lot of judgement calls should be doable by it quickly and cheaply such that even cheap models can make the right decisions. Like escalating to a frontier model when a gap requires a complex new feature to be planned and implemented."
> user: "ok $flow-next-flow towards this pipeline I think you understood my intention well."

> user: "1" — selected the proposed allocation: two new specs and amendments to fn-68 and fn-80.

> user: "efficiency is mostly about choosing the right tool for the job. judgements/decisions/routing should be done by jev wherever possible to keep llm token costs low. Then the agent should be a cheap effective model. But when a gap turn up and something is judged complex (by jev i imagine) then we escalate to a high reasoning model with medium/high effort to design the spec. Then we judge who can implement the design. If the design has done most of the complex work ahead of time we can probably use a cheap model. If the implementation is complex then we should probably use a high reasoning model on low effort."
> user: "That's mostly what i meant with efficiency. Makes sense? do we need to adapt the specs? can you review them over? i really don't think we need to save on jev calls with fn-88"
> user: "ok" (accepted the recommendation to retire fn-88 and amend fn-89, fn-68 and fn-80).

> user: "the proposed flow is a straight line but it should contain a loop with automated visual readiness"
> user: "there should also be a path to escalate to human. We don't want to burn tokens on dead ends etc. So we should have judgement calls evaluating certainty of success within reasonable token amounts. If there's high uncertainty or unusually high complexity or other implementation risk loop should escalate."
> user: "ok then pls $flow-next-capture --rewrite them"

> user: "alright a sweep sounds very inefficient though? doing renders and assessing them for tiny steps?"
> user: "do we need the sweep fallback? if jev gets us there much faster. Can we have jev say how much a parameter should change?"
> user: "ok" (accepted removing automatic sweeping, testing bounded direction-and-magnitude proposals, and retaining historical sweep results only as a comparison).

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

The owner tunes a species by rounds: a value change, matched stills, a verdict, another change. fn-34 ran 26 rounds on the birch and 23 on the beech that way, with the host model reading the verdict and guessing the rows. The owner asked on 2026-09-18 whether Jev could take the parameters and the research and return the set that gets closer, with a model judging each render, and asked for a small prototype before deciding. [user]

The prototype ran that day on the beech as shipped at fn-62's round 22 (`experiments/fn58-tuning-loop/`). Code evaluated a candidate by measuring it through the species example, rendering its two matched stills through the headless renderer and reading the compare script's five numbers, width over height, crown base, occupied share, outline deviation and centre brightness, against the photograph's own. A sweep of one step on each of twelve dials took the distance from 0.245 to 0.145 in three rounds, 74 evaluations and 18 minutes, with the leaf-on still landing on the photograph's density, outline and brightness within two percent. Jev asked one direction per dial, up, down or hold, over the owner's written verdict, reached 0.154 in 13 evaluations and three minutes, and proposed the same first move as the sweep, one limb per station instead of two. Two other framings, a yes-or-no per move and a choice over the measured candidates, did not move the score. [paraphrase]

This spec turns the probe into the loop a species round runs: code steps the dials, Jev routes sourced observations to directions, measured scores choose numerical candidates, automated visual review checks readiness, and the owner's eye accepts the finalists. It runs on the beech today from the reference records fn-36 wrote, and takes its targets from fn-58's packet once that packet exists. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **A candidate is a partial wire object over the shipped preset.** `params::overlay` lays it over the preset's wire and reads it back through `parse`, so an unknown row or a value off its range is refused by name. The species example and the headless renderer both take it as `--family FILE`. An empty overlay reproduces the shipped metrics and still byte for byte, which is the golden check. [paraphrase]
- **The dial table is authored.** Each allowed dial has a plain-language meaning, valid range, numeric type and code-defined bounded adjustments for small and substantial changes in each direction. Jev selects a named adjustment; code computes the numeric candidate. Step sizes are parameter-specific, never derived from Jev confidence. A dial outside the table never moves. [inferred]
- **Evaluation is code.** Measure through the species example for the gates and the node cap; render still and twin per reference record that carries a shot block, at the matched height; read the compare script's still-side numbers off the pair; score the mean relative distance from the photograph's numbers, which come from the reference record's compare output. A failed gate, a capped tree or a still with no tree makes the candidate infeasible. The trial record identifies the effective parameters, seed, generator, renderer, references, camera and measurement definition so results remain attributable and stale results cannot advance the run. This requires no cross-run cache or guarantee that identical work is never repeated. [paraphrase]
- **The notes have two sources, and both are only proposers.** The owner's verdict on the previous round is the first, verbatim, as the probe used it. A vision-capable model demonstrated adequate on the species checklist reads the current matched pair as the second source: it writes what is better and what is missing in prose, per still, and that prose enters the direction question's state beside the owner's, tagged with its source and the model's name. Neither note changes a measured score. A visible defect that the score cannot measure remains unresolved at the separate visual-readiness gate and triggers reassessment; a numerical win cannot erase it. The owner's note outranks the model's when they disagree, by ordering in the state. The model's notes are not deterministic, so a run records them and the swap test compares accepted moves, never note text. [paraphrase]
- **Targeted proposals, measured selection.** Jev chooses direction and magnitude from named options using sourced visual observations, reference measurements, current values and prior outcomes. Code evaluates at most four targeted candidates per round, rejects invalid candidates before rendering, and selects the lowest feasible measured score among improving candidates. Each selection still passes visual assessment. No automatic full sweep follows stalled progress, failed calls or low-confidence proposals; R11 routes to justified diagnosis or human escalation. [paraphrase]
- **A round feeds automated refinement.** Matched rendering produces a visual assessment; Jev routes remaining defects to another tuning round or a capability fix; verified changes rebuild and render for reassessment. Before further work, the shared continuation policy checks evidence, progress, reasonable token allowance and risk. The loop ends at machine readiness or a resumable human escalation, not only at a depleted budget. Pending final owner acceptance does not stop justified intermediate refinement. [paraphrase]
- **The trial table is the record.** One row per evaluation: arm, round, label, key, overrides, feasibility, gates, counts, every still number, score, seconds, still hashes. The rounds table fn-34 kept by hand is what it replaces. Ledger references for every Jev call sit in the run record. [paraphrase]
- **The adjustment question is versioned and evaluated.** Historical rounds supply direction labels; magnitude choices need their own labeled cases and bounded pilot. Existing up/down/hold results do not establish magnitude accuracy. Validate the action, including hold and insufficient evidence, separately from distribution confidence. [inferred]

## API Contracts
<!-- scope: technical -->

- Each dial defines its meaning, numeric type, bounds and deterministic mappings from named direction-and-magnitude actions to valid candidate values. Options distinguish small/substantial decrease, hold, small/substantial increase and insufficient evidence; unavailable actions are explicitly excluded. Discrete dials may have fewer distinct valid adjustments. [inferred]
- The question state carries sourced observations, current/reference measurements, the allowed action meanings and recent attempts with outcomes. Hold means no change is indicated; insufficient evidence means the model cannot support an adjustment. Jev selects an action, never invents a numeric delta or a preset value. [inferred]
- A trial records the question/table versions, selected dial and action, code-computed change, input identity, feasibility, measured score, visual result when assessed, time and judgment ledger reference. Original fixed-step trial records remain historical evidence. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Dials interact.** In the probe the compound of every improving move scored worse than the best single move in rounds one and two. The loop accepts one move per round unless the compound measures better. [paraphrase]
- **The score is a proxy.** Five numbers per still miss what the owner's notes name, fine twigs at the edge and density by height. A candidate can score well and read wrong; one limb per station did. The owner's checklist, not the score, closes a species round. New measurements are their own spec. [paraphrase]
- **The mean can trade one still for another.** The probe's sweep matched the leaf-on still while the bare still grew brighter, 116 to 136 against the photograph's 100. Weights per number and per reference are a table a person edits, and the trial table shows every number so the trade is visible. [paraphrase]
- **A value the generator refuses is infeasible, never clamped.** Ten leaves per short shoot was refused by the placement builder in half a second; the loop records the refusal and moves on. [paraphrase]
- **Missing observations can hide useful moves.** The original direction probe missed crown-base and spread changes absent from its notes. Updated visual observations and bounded diagnosis address such blind spots; missing signal does not trigger exhaustive rendering. Failed or unsupported proposals go through R11. [inferred]
- **Stills are assessed by the configured vision model, not Jev, and never committed.** They go to a scratch directory; the trial table carries their hashes and view/seed identities. Matched assessments, fixed/fresh seed checks and the final review packet remain inside the standing total capture budget. [inferred]
- **Jev runs only here.** The loop is evidence tooling; the species example, the renderer and the presets stay free of the caller. The workspace tests keep the key unset and use the mock transport. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A loop command takes a preset id, a seed, the species' reference records, an authored dial table and the owner's notes, and per round proposes candidates as partial wire objects, evaluates each by measurement, matched render and the compare script's numbers, and records every evaluation in the trial table with still hashes and ledger references. [paraphrase] Errors: a candidate the generator refuses or that fails a gate is recorded infeasible with the reason and never rendered; a still with no tree is infeasible.
- **R2:** Jev proposes a direction and bounded magnitude from the authored action set, using sourced observations, current/reference measurements and prior outcomes. Code computes the numeric values, rejects invalid candidates before rendering and evaluates at most four targeted candidates per round; measured selection and visual readiness remain separate. Errors: stalled progress, failed Jev calls, low-confidence answers or insufficient evidence never launch a full sweep; they route through R11. Confidence must not scale the numeric adjustment. [paraphrase]
- **R3:** The score is the weighted mean relative distance from the photograph's numbers over both references, with weights in an authored table; the table's default is equal weights. [paraphrase] Errors: a number the compare script cannot read, such as an outline that does not close, counts as the full distance.
- **R4:** A round stops on its evaluation budget, no improvement or a stalled score, and records up to the best three candidates as matched pairs at fixed seeds with their numbers. Automated visual refinement then continues under R8-R10; only the owner supplies final acceptance. Errors: no improvement leaves shipped rows untouched and triggers bounded reassessment, never false readiness. [inferred]
- **R5:** Version and evaluate the adjustment question on labeled direction-and-magnitude cases, including hold, insufficient evidence, discrete bounds and overshoot. Retain the existing direction agreement target of at least 0.8 on held-out historical cases; define and document magnitude acceptance rules on labeled examples before the pilot, using held-out cases separately from tuning. Errors: absent magnitude labels, failed validation or unsupported actions prevent unattended magnitude proposals and route to diagnosis or human escalation; no sweep-only fallback. [inferred]
- **R6:** The overlay, both `--family` flags and `jev ask` are tested: the overlay unit test, a golden test that an empty overlay reproduces a preset's metrics, and an `ask` test on the mock transport that writes a ledger entry. The loop itself is excluded from the workspace test commands. [paraphrase] Errors: the isolation guard fails if the core, render or Wasm crates import the caller.
- **R7:** Run a bounded direction-and-magnitude pilot on the beech from the recorded round-22 rows at seed 1, comparing against the recorded fixed-step direction baseline (13 evaluations, three minutes, final score about 0.154) and historical sweep baseline (74 evaluations, 18 minutes, about 0.145). Declare its evaluation, capture and token limits before execution; report actual cost, visual defects improved, numerical trajectory, rejected/overshooting moves and stop reason. Errors: unavailable comparable evidence is explicit, a numerical win alone cannot prove visual success, and a failed pilot leaves magnitude routing unvalidated. The sweep is historical comparison only, not an implementation or rerun requirement; no exact first move is mandated for the new strategy. [inferred]

- **R8:** A configured vision-capable model demonstrated adequate on representative checklist cases evaluates reference-matched mature-tree views, records concrete defects with view/seed identity, and emits machine readiness separately from owner acceptance. Use the cheapest demonstrated-capable visual role rather than requiring frontier reasoning for every pass. Jev routes the textual observations and structured evidence; it does not inspect images. Errors: missing references, unassessable views or failed visual assessment cannot produce readiness; escalation follows the recorded policy. [inferred]
- **R9:** Jev routes those defects to existing dials, an existing gap spec, a new capability investigation or an appearance issue. Code evaluates candidates; a numeric improvement cannot erase a remaining visual defect. A stalled numeric objective with an unresolved visible requirement triggers reassessment rather than falsely proving a generator gap or requiring the owner to tune the next round. [inferred]
- **R10:** Repeat matched render, automated visual assessment, defect routing, justified tuning or verified capability repair, rebuild and reassessment until machine readiness or a resumable human escalation. Recheck readiness after relevant changes. Preserve prior owner-rejected numeric-good candidates in the replay set and report false-ready outcomes separately from false rejections. Errors: unresolved visible defects never become readiness solely because numerical scores improve; R11 decides whether further work is justified. The owner retains final acceptance. [inferred]

- **R11:** At each round boundary and before a proposed fix dispatch, apply the shared fn-89 continuation policy to observed defects, progress, failed attempts, expected next-attempt token spend, remaining budget and risk. Pause with a resumable human handoff when uncertainty, unusual complexity, implementation risk or no-progress makes another attempt unjustified. Errors: missing assessment or exhausted hard limits cannot start another round; resuming preserves spent budgets and requires the scoped human decision. [inferred]

## Boundaries
<!-- scope: business -->

- Jev never invents a numeric preset value or chooses between measured candidates. It selects a named, code-defined direction-and-magnitude adjustment; code calculates the value and enforces its range. No full-sweep fallback is shipped. [paraphrase]
- The owner's eye retains final acceptance. A vision-capable model may judge checklist readiness and provide sourced observations for refinement, but cannot manufacture a numeric score or impersonate an owner verdict. This replaces the earlier prohibition on model visual judgments, following the owner's autonomous-loop allocation on 2026-09-19. [paraphrase]
- The loop ships no species. The beech's rows change only through fn-62, which may adopt a candidate the loop hands it. [paraphrase]
- No new measurements. Fine twigs at the edge and density by height are a spec of their own that this loop consumes when it lands. [paraphrase]
- No generator or renderer behaviour beyond the `--family` flag and the overlay. [inferred]
- Growth stays hidden. The loop tunes the direct build the harness shows. [paraphrase]

## Decision Context

### Joint visual diagnosis and independent evaluation, 2026-09-20

- The owner asks to build the pipeline so the reviewer identifies important gaps from its evidence and instructions without being given the owner's assessment as the answer. This clarifies R8-R10. [user]
- The visual packet groups available matched references and renders for joint assessment. It identifies image roles, views, seeds, candidate identity and known condition relationships. Reference photographs are not assumed to depict the same specimen. A render with foliage hidden is identified as a visibility change on the same geometry, not evidence of an unloaded or seasonal tree. Unknown relationships remain explicit. Packet metadata participates in evidence identity. [inferred]
- The reviewer ranks the largest observed gaps in believable reference character, preserves cross-view constraints, and separates observations, acceptable variation, uncertain conclusions and causal hypotheses. The live loop retains those findings and their image attribution for Jev routing and diagnosis. Per-view passes cannot override an unresolved cross-view blocker. Clipped or missing evidence cannot establish the affected readiness criterion. [paraphrase]
- Independent evaluation supplies the reviewer an allowlisted factual packet and generic joint-review instructions. Owner assessments, previous judge verdicts, proposed fixes and expected findings stay in a separate evaluation record. Changing expected labels cannot change the dispatched request. Production owner requirements remain authoritative; the blind evaluation measures whether the reviewer discovers gaps independently. Beech is a known development case, not a fresh held-out generalization test. [inferred]
- Offline contract tests establish isolation, attribution and propagation, not model adequacy. A changed visual protocol requires fresh bounded calibration before unattended use. No prompt iteration silently replenishes existing token or capture budgets. Leaf-load deformation and seasons remain generator work outside this spec. [inferred]

### Autonomous-loop amendment, 2026-09-19

- The owner rejected automatic sweeping and accepted testing Jev-selected adjustment size. This supersedes the earlier full-sweep fallback, sweep-only uncalibrated mode and mandatory reproduction of the sweep arm. Historical results remain evidence; variable magnitude remains a hypothesis until the bounded pilot and held-out validation support it. [paraphrase]

- Visual refinement explicitly loops through reassessment. fn-89 owns the shared early-escalation policy and conductor integration; this loop exposes the observations, continuation decision and paused/resume contract without a second policy or a cyclic implementation dependency. Hard budgets are a backstop, not a reason to spend the remaining allowance on a dead end. [inferred]

- The owner's efficiency correction assigns numeric evaluation to code, semantic routing to Jev and image inspection to a demonstrated-capable vision model. Complex design and implementation use fn-89's separate routing decisions; no expensive model remains active just because an earlier stage escalated. [paraphrase]

- The owner selected two new pipeline specs and amendments here and in fn-80. The previous per-round owner stop and model-judgment prohibition are superseded by machine readiness followed by final owner acceptance. Numeric selection, authored dials and generator isolation remain intact. R7 now uses the original probe as recorded comparison for the bounded magnitude pilot. [paraphrase]
- fn-72 supplies catalogue/profile resolution. fn-89 consumes this loop's readiness contract and owns model/effort routing. Fixed/fresh seed checks remain part of the evidence protocol. Retiring fn-88 removes caching as a prerequisite; result identity and stale-result rejection remain correctness requirements. [paraphrase]

<!-- scope: both — conditionally substructured -->

### Motivation
<!-- scope: business -->

The owner's hope was a tuning loop much faster than hand rounds with a language model. The probe put the speed in code: measure, render, read the numbers, step. One sweep round found the move the birch needed twenty-five hand rounds to reach. The owner's own framing for Jev, a direction per dial that code applies, was the one framing that helped, and it helped because it reads the verdict, which is prose, and hands code a direction, which is a number's job. The owner chose a parallel spec so fn-58 keeps its finish line and the loop can run on the beech now. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

Three Jev framings ran on 2026-09-18 with the same state. A Noul per move put both directions of one dial above one half and the move that worked below it. A Choice over the sweep's measured candidates was flat between 0.15 and 0.28 over 22 options. A Choice per dial over up, down and hold ranked fewer leaves per cluster, wider spacing, fewer limbs and fewer twigs for "too dense everywhere", all directions the sweep measured as improving. These observations motivated the original direction-only proposal and sweep fallback. The owner's later correction removes the fallback and extends the proposal to named magnitudes. The original evidence still cautions against treating Jev as a numerical optimizer or assuming combined parameter changes improve together. [paraphrase]

## Strategy Alignment

- Serves "Growth and botanical fidelity": a species judged against real photographs by measured rounds, with every accepted move traceable to a trial row and a ledger reference. [strategy:Growth and botanical fidelity]

## Resolved via Codebase

- `params::overlay` in `crates/telperion-core/src/params.rs` with its test in `params/tests.rs`; `--family` and `--print-family` in `crates/telperion-core/examples/species_measure.rs`; `--family` in `crates/telperion-render/examples/headless.rs` and `headless/walk.rs`; `jev ask` in `crates/telperion-jev/src/bin/jev.rs`. All uncommitted on 2026-09-18.
- The probe: `experiments/fn58-tuning-loop/loop.py` (evaluation, score, the three arms), `jev-choice.py`, `trials.tsv`, `arms.json`, `arms-direction.json`, `README.md`. Ledger entries under `.flow/ledger/jev-tune-probe/`.
- The still-side numbers: `scripts/compare-references.py` (`tree_mask`, `box_of`, `crown_base`, `measure`); the photograph's numbers per reference in `.flow/evidence/fn34/rounds.tsv`.
- Render cost on the RTX 3080: 3.8 seconds a still at 1440 high, 10 seconds a measurement, so a candidate costs 25 seconds and a lighter tree half that.

## Parked unknowns

- Whether a multimodal model's notes steer as well as the owner's; the probe ran on the owner's verdict only, and one round with the model's notes beside it, compared on accepted moves, answers it.
- Whether the direction question holds its accuracy once the labelled set exists; the probe's answers were uncalibrated and the bare-still numbers were weaker signal than the leaf-on ones.
- Whether the loop's candidates read right to the owner's eye; the first fn-62 round that adopts one answers it.
- Which weights per number and per reference the owner wants as the default; the first round the owner reads the trial table answers it.
