---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-34-beech-ash-and-birch-as-real-species.1 Implement Beech, ash and birch as real species

## Description
Onboard European beech, silver birch and European ash as fn-9-style packets and value tables. Beech and birch are catalogue species. Ash waits for fn-33 compound leaves.

## Acceptance
- [x] R1 packets under `.flow/evidence/fn34/<id>/` with gating/contextual ranges and labelled estimates
- [x] R2 beech and birch presets as value tables; every listed identity site updated; no generator/renderer change
- [x] R3 numeric protocol passed (48/48); visual verdicts left unassessed for the owner
- [x] R4 ash profile/references complete; no ash preset until fn-33 (the worker's simple-blade fixture was removed at host review as dead code)
- [x] R5 identity pins, sweep bands, mature height-by-age on the ready profiles

## Progress
- Packets: european-beech (ready), silver-birch (ready), european-ash (unready; waits for fn-33).
- Catalogue ids `european-beech`, `silver-birch`. `european-ash` is not in `from_id` or `CATALOGUE`.
- Fresh seeds drawn once to `.flow/evidence/fn34/seeds.json`. 48/48 numeric cases passed. No retained failures. `species:qa --capture-only` wrote every required still and exited 1 with visual unassessed.
- Judging stills: `.flow/evidence/fn34/stills/`, hashes in `stills.json`. Visual unassessed.
- Gaps: owner visual verdicts (R3). Ash template after fn-33. Growth-trait calibration after fn-30. `crown_reference.rs` FN6 list not extended (oak/spruce are not on it either).

## NEEDS_HUMAN

Round 1 verdict (2026-09-14): rejecting, "To me it's clear that it's not there yet." The owner asked for a shot that closely imitates each reference image, then a QA pass. Recorded in the spec under Owner verdicts. fn-36 shipped the matched-shot rig and round 2 is rendered: six pairs (beech and birch, whole, bare and base at seed 1) with the comparison numbers in `.flow/evidence/fn34/round2/` and the round-2 section of REPORT.md. The owner's verdict on the pairs is the open item.

The host reviewed the worker's range from 3468ef7, ran the core, render, typecheck, clippy and browser gates (all pass), removed the dead ash fixture and rebased the branch onto master. R3 closes only on the owner's visual verdicts: twelve stills under `.flow/evidence/fn34/stills/`, three fixed seeds per species in whole and bare views, judged beside the references in `.refs/fn34/`. The owner records a verdict per species in the spec; on accepting verdicts the host runs `flowctl done`. Ash's template and the growth-trait rewrite wait for fn-33 and fn-30 and are outside this task.

## Done summary
TBD — host completes the task after the owner's verdicts.

## Evidence
- Commits:
- Tests:
- PRs:
