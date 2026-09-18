# Tuning loop: code steps the dials, Jev routes the notes, the number decides

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

The owner tunes a species by rounds: a value change, matched stills, a verdict, another change. fn-34 ran 26 rounds on the birch and 23 on the beech that way, with the host model reading the verdict and guessing the rows. The owner asked on 2026-09-18 whether Jev could take the parameters and the research and return the set that gets closer, with a model judging each render, and asked for a small prototype before deciding. [user]

The prototype ran that day on the beech as shipped at fn-62's round 22 (`experiments/fn58-tuning-loop/`). Code evaluated a candidate by measuring it through the species example, rendering its two matched stills through the headless renderer and reading the compare script's five numbers, width over height, crown base, occupied share, outline deviation and centre brightness, against the photograph's own. A sweep of one step on each of twelve dials took the distance from 0.245 to 0.145 in three rounds, 74 evaluations and 18 minutes, with the leaf-on still landing on the photograph's density, outline and brightness within two percent. Jev asked one direction per dial, up, down or hold, over the owner's written verdict, reached 0.154 in 13 evaluations and three minutes, and proposed the same first move as the sweep, one limb per station instead of two. Two other framings, a yes-or-no per move and a choice over the measured candidates, did not move the score. [paraphrase]

This spec turns the probe into the loop a species round runs: code steps the dials, Jev routes the owner's notes to directions, the number decides which candidate stands, and the owner's eye judges the finalists. It runs on the beech today from the reference records fn-36 wrote, and takes its targets from fn-58's packet once that packet exists. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **A candidate is a partial wire object over the shipped preset.** `params::overlay` lays it over the preset's wire and reads it back through `parse`, so an unknown row or a value off its range is refused by name. The species example and the headless renderer both take it as `--family FILE`. An empty overlay reproduces the shipped metrics and still byte for byte, which is the golden check. [paraphrase]
- **The dial table is authored.** One row per dial the loop may move: wire path, a plain reading for the question, the step, the bounds. The probe's twelve rows are the first table; a dial the table does not name never moves. The table lives beside the species' evidence, is versioned, and a person edits it. [paraphrase]
- **Evaluation is code.** Measure through the species example for the gates and the node cap; render still and twin per reference record that carries a shot block, at the matched height; read the compare script's still-side numbers off the pair; score the mean relative distance from the photograph's numbers, which come from the reference record's compare output. A failed gate, a capped tree or a still with no tree makes the candidate infeasible. Every evaluation is keyed by the sha256 of its canonical overrides and never repeated. [paraphrase]
- **The notes have two sources, and both are only proposers.** The owner's verdict on the previous round is the first, verbatim, as the probe used it. A strong multimodal model reading the current matched pair is the second: it writes what is better and what is missing in prose, per still, and that prose enters the direction question's state beside the owner's, tagged with its source and the model's name. Neither note touches the score, so a note can only steer toward moves the numbers confirm; a note that names what no number measures proposes moves the score rejects, which the trial table shows. The owner's note outranks the model's when they disagree, by ordering in the state. The model's notes are not deterministic, so a run records them and the swap test compares accepted moves, never note text. [paraphrase]
- **Two proposers, one decider.** Each round the loop asks Jev one Choice per dial over up, down and hold, with the owner's notes, the photograph's numbers, the current still's numbers and the dial table as state, and evaluates the four dials Jev moves most confidently. When none of them improves the score by one percent, code sweeps every single-step move and the compound of the improving ones. The candidate with the lowest feasible score stands. Jev's answer never sets a value and never decides between measured candidates; the probe showed it cannot separate twenty tables of numbers. [paraphrase]
- **The round stops at the owner.** A round ends on a budget of evaluations, on no improvement, or when the accepted candidate's score stops falling. The loop writes the accepted candidate and the two next best as matched pairs at the fixed seeds, with their numbers beside the photograph's, and files a visual-unassessed decision. A person judges the stills; no model does. [paraphrase]
- **The trial table is the record.** One row per evaluation: arm, round, label, key, overrides, feasibility, gates, counts, every still number, score, seconds, still hashes. The rounds table fn-34 kept by hand is what it replaces. Ledger references for every Jev call sit in the run record. [paraphrase]
- **The direction question is versioned data with a labelled set.** fn-34's and fn-62's rounds record which dial moved in answer to which note and whether the owner accepted; those are the labels. The probe ran the question uncalibrated. [inferred]

## API Contracts
<!-- scope: technical -->

The dial table row and the trial row are the two shapes.

```json
{"dial": "laterals_per_station", "path": "skeleton/habit/lateralsPerStation", "what": "limbs born at each station up the trunk", "step": 1, "bounds": [1, 4]}
```

```json
{"arm": "direction", "round": 1, "label": "direction:laterals_per_station:down", "key": "1abe9433ef89", "overrides": {"skeleton": {"habit": {"lateralsPerStation": 1}}}, "feasible": true, "reason": "", "height_m": 31.99, "dbh_m": 1.016, "wood_triangles": 5928360, "B-WHOLE.occupied": 0.4783, "B-BARE.occupied": 0.3353, "score": 0.187, "seconds": 13.1, "ledger": "tune-probe-direction:18d65e26b4da2ef6-e09a-0"}
```

The direction question, per dial: a Choice with `up`, `down` and `hold`, instructions naming the dial's path and reading, state carrying `notes`, `photograph`, `still`, `numbers` (a plain reading per number), `dials` (value, step, reading) and `accepted_so_far`. The command surface is the plan's to fix; the spec fixes the shapes and the question. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Dials interact.** In the probe the compound of every improving move scored worse than the best single move in rounds one and two. The loop accepts one move per round unless the compound measures better. [paraphrase]
- **The score is a proxy.** Five numbers per still miss what the owner's notes name, fine twigs at the edge and density by height. A candidate can score well and read wrong; one limb per station did. The owner's checklist, not the score, closes a species round. New measurements are their own spec. [paraphrase]
- **The mean can trade one still for another.** The probe's sweep matched the leaf-on still while the bare still grew brighter, 116 to 136 against the photograph's 100. Weights per number and per reference are a table a person edits, and the trial table shows every number so the trade is visible. [paraphrase]
- **A value the generator refuses is infeasible, never clamped.** Ten leaves per short shoot was refused by the placement builder in half a second; the loop records the refusal and moves on. [paraphrase]
- **Jev proposes only what the notes name.** The sweep's best later moves, crown base and spread, appear in no note. The sweep fallback is what reaches them. [paraphrase]
- **Stills are never looked at by the loop and never committed.** They go to a scratch directory; the trial table carries their hashes. The loop reads at most one still pair per candidate and the owner sees at most three pairs per round, inside the project's capture budget. [paraphrase]
- **Jev runs only here.** The loop is evidence tooling; the species example, the renderer and the presets stay free of the caller. The workspace tests keep the key unset and use the mock transport. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A loop command takes a preset id, a seed, the species' reference records, an authored dial table and the owner's notes, and per round proposes candidates as partial wire objects, evaluates each by measurement, matched render and the compare script's numbers, and records every evaluation in the trial table with still hashes and ledger references. [paraphrase] Errors: a candidate the generator refuses or that fails a gate is recorded infeasible with the reason and never rendered; a still with no tree is infeasible.
- **R2:** Candidates come first from Jev's per-dial direction question over the notes, which are the owner's last verdict and, when the run enables it, a named multimodal model's prose read of the current pair, each tagged with its source in the run record, at most four evaluated per round, and from a full single-step sweep with the compound of improving moves when no proposed move improves the score by one percent. The candidate with the lowest feasible score stands. A Jev answer never sets a value and never chooses between measured candidates. [user] Errors: a Jev call that fails after retries falls through to the sweep and is recorded.
- **R3:** The score is the weighted mean relative distance from the photograph's numbers over both references, with weights in an authored table; the table's default is equal weights. [paraphrase] Errors: a number the compare script cannot read, such as an outline that does not close, counts as the full distance.
- **R4:** A round stops on its evaluation budget, on no improvement, or when the accepted score stops falling, writes the best three candidates as matched pairs at the fixed seeds with their numbers beside the photograph's, and files a visual-unassessed decision. No still is judged by a model. [paraphrase] Errors: a round that hits its budget with no improvement says so and leaves the shipped rows untouched.
- **R5:** The direction question is versioned data with a labelled set drawn from fn-34's and fn-62's rounds, and top-one agreement with the dial the owner's accepted round moved is at least 0.8 on held-out cases before the question is trusted in R2; until then the loop runs sweep-only and says so. [inferred] Errors: agreement below the bound lists the missed cases.
- **R6:** The overlay, both `--family` flags and `jev ask` are tested: the overlay unit test, a golden test that an empty overlay reproduces a preset's metrics, and an `ask` test on the mock transport that writes a ledger entry. The loop itself is excluded from the workspace test commands. [paraphrase] Errors: the isolation guard fails if the core, render or Wasm crates import the caller.
- **R7:** The loop reproduces the probe on the beech at seed 1 from fn-62's round-22 rows: the sweep reaches at most 0.15 within three rounds and the direction arm at most 0.16, from the probe's recorded trial table, with the same first accepted move. [inferred] Errors: a different first move or a higher score fails with the trial rows listed.

## Boundaries
<!-- scope: business -->

- Jev never writes a number and never picks between measured candidates. It answers up, down or hold per dial from the owner's notes. [user]
- Every still stays with the owner's eye for the verdict. A multimodal model may read a pair to write notes that steer the next round's proposals; its notes never score a candidate, never close a checklist line and never stand in a report as a judgment. [user]
- The loop ships no species. The beech's rows change only through fn-62, which may adopt a candidate the loop hands it. [paraphrase]
- No new measurements. Fine twigs at the edge and density by height are a spec of their own that this loop consumes when it lands. [paraphrase]
- No generator or renderer behaviour beyond the `--family` flag and the overlay. [inferred]
- Growth stays hidden. The loop tunes the direct build the harness shows. [paraphrase]

## Decision Context
<!-- scope: both — conditionally substructured -->

### Motivation
<!-- scope: business -->

The owner's hope was a tuning loop much faster than hand rounds with a language model. The probe put the speed in code: measure, render, read the numbers, step. One sweep round found the move the birch needed twenty-five hand rounds to reach. The owner's own framing for Jev, a direction per dial that code applies, was the one framing that helped, and it helped because it reads the verdict, which is prose, and hands code a direction, which is a number's job. The owner chose a parallel spec so fn-58 keeps its finish line and the loop can run on the beech now. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

Three Jev framings ran on 2026-09-18 with the same state. A Noul per move put both directions of one dial above one half and the move that worked below it. A Choice over the sweep's measured candidates was flat between 0.15 and 0.28 over 22 options. A Choice per dial over up, down and hold ranked fewer leaves per cluster, wider spacing, fewer limbs and fewer twigs for "too dense everywhere", all directions the sweep measured as improving. The loop therefore asks the third and never the first two. The sweep stays as the fallback because it found crown base and spread, which no note names. Coordinate steps rather than compound moves because the compound lost twice. [paraphrase]

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
