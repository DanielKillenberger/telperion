---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-34-beech-ash-and-birch-as-real-species.1 Implement Beech, ash and birch as real species

## Description
Onboard European beech, silver birch and European ash as fn-9-style packets and value tables. Beech and birch are catalogue species. Ash waits for fn-33 compound leaves.

## Acceptance
- [x] R1 packets under `.flow/evidence/fn34/<id>/` with gating/contextual ranges and labelled estimates
- [x] R2 beech and birch presets as value tables; every listed identity site updated; no generator/renderer change
- [ ] R3 numeric protocol passed; visual verdicts left unassessed for the owner
- [x] R4 ash profile/references complete; simple-blade fixture not in the catalogue
- [x] R5 identity pins, sweep bands, mature height-by-age on the ready profiles

## Progress
- Packets: european-beech (ready), silver-birch (ready), european-ash (unready; waits for fn-33).
- Catalogue ids `european-beech`, `silver-birch`. `european-ash` is not in `from_id` or `CATALOGUE`.
- Fresh seeds drawn once to `.flow/evidence/fn34/seeds.json`. 48/48 numeric cases passed. No retained failures.
- Judging stills: `.flow/evidence/fn34/stills/`, hashes in `stills.json`. Visual unassessed.
- Gaps: owner visual verdicts (R3). Ash template after fn-33. Growth-trait calibration after fn-30. `crown_reference.rs` FN6 list not extended (oak/spruce are not on it either).

## Done summary
TBD — host completes the task.

## Evidence
- Commits:
- Tests:
- PRs:
