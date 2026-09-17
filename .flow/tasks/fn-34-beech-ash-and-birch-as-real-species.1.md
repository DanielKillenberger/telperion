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

Round 26 settings round (2026-09-17, commits 0f2c2f6a and 471e1385 on `fn-34-integration`): the whorls, the twig thicket and the central leaf mass are answered in the value table; the fork ring and the flat-ended limbs are untouched, being fn-48.3's and fn-4's. Four rows of `silver_birch` move and no generator or renderer code does: `laterals_per_station` 4 to 2, `lateral_pitch` 62 to 45 degrees, `twigs.laterals` 8 to 7, `twigs.length_ratio` 0.6 to 0.55. `rise_secondary` stays at -0.85, which is already a full droop and already the row the shoots under a limb read; `lateral_spacing` stays at 1.2, because the trial that shortened it to 1.0 put the wood back in the interior the round was clearing.

What the instruments say at the first fixed seed, round 25 then round 26. Primary limbs 174 to 82, of which the lower crown from 2 m to 10 m holds 27 then 13; branches 54,512 to 33,526, the orders 4 and 5 that made the thicket 36,081 to 22,279; twigs 53,043 to 32,611; placed leaves 353,885 to 226,057; leaf area 583.1 to 372.6 square metres; wood triangles 6.65 M to 4.08 M. Against the photographs, S-WHOLE's centre 70.8 to 82.2 where the photograph reads 83.0, its occupied share 0.4253 to 0.4508 against 0.4426, its width over height 0.9049 to 0.9139 against 0.8461; S-BARE's occupied 0.4878 to 0.4974 against 0.4587 and its centre 86.9 to 102.9 against 91.3. The leaf spacing was re-read against S-WHOLE's centre and left at a leaf every 60 mm: the centre already lands on the photograph's own value. Every candidate tried is in the round record; the thinner ones (five and six twig laterals) overshot the photograph's centre to 96.4 and 88.8 and took the bare tree well under its occupied share.

Gates: 48/48 numeric cases pass at `.flow/evidence/fn34/measure/protocol-round26/`, none capped, visual unassessed by design; `cargo test --release --workspace`, `cargo clippy --release -p telperion-core --all-targets -- -D warnings`, `npm test`, `npm run typecheck` and `npm run test:render` all pass. The birch's identity, drop, sag and strand pins are re-recorded once each with the reason stated in their files; the drop test's cooked curtain now holds the four moved rows at the values its own counts were read under, so what its share-below assertion measures is still the drop law.

What the owner is asked to judge, at `.flow/evidence/fn34/measure/protocol-round26/` (stills stay on the ignored measure/ directory, hashes in `.flow/evidence/fn34/rounds.tsv` under `round26-birch-crown`):
- `silver-birch-1-S-WHOLE.png` sha256 2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e
- `silver-birch-1-S-BARE.png` sha256 246aa0c58eae50300ec5be6520aca8a854cf97506eb5c86f581c7ede7c29be68
- `silver-birch-1-S-BARK.png` sha256 67d8000c31f2272ef242b3f78f622213522fabf6020834f6ad6ddc614248edbb
- `silver-birch-1-leaf.png` (S-LEAF and S-SHOOT) sha256 7ccca41a4664bb715ec5066f1ba94d7de40ad74059d8ec02e0b7176080fc1a14, byte-identical to round 25's
- `silver-birch-1-S-FORK.png` sha256 cee99ba273553073b0efe0817800c01a904af4eda14c9d74023b6e81ecf1883b, the new close-up of the base from two metres with the leaves hidden and the fork in frame
- `silver-birch-1-S-UNDER.png` sha256 ec6b32a66eb82c58f1b861fe52d8877c5bb9f04246f31dde355a4636d2795f1d, the new view from inside the lower crown at human eye height, looking up
Pairs for the three matched shots are in `.flow/evidence/fn34/measure/pairs-round26/`.

The verdict is the owner's and no agent takes it: whether the lower crown now reads as a birch rather than rings of spokes, whether the interior is open enough from underneath, and whether the whole tree has not been thinned too far. On an accepting verdict the fork-ring bug spec and fn-4 are next, then the verdict is retaken and PR #29 goes back to ready. `flowctl done` was deliberately not run.

Round 26 (recorded 2026-09-17): the round-25 acceptance is withdrawn. In the live renderer the owner found a fork ring at the birch's base, whorls of near-horizontal limbs and a twig thicket in the lower crown, and flat-ended limbs; all reproduce in the headless renderer. See the spec's Round 26 verdict for the rows and the path. The task is reopened for the settings round; PR #29 is a draft.


Round 1 verdict (2026-09-14): rejecting, "To me it's clear that it's not there yet." The owner asked for a shot that closely imitates each reference image, then a QA pass. Recorded in the spec under Owner verdicts. fn-36 shipped the matched-shot rig and round 2 is rendered: six pairs (beech and birch, whole, bare and base at seed 1) with the comparison numbers in `.flow/evidence/fn34/round2/` and the round-2 section of REPORT.md. Round 3 (2026-09-14): the owner asked for the easy flaws fixed; the host tuned both value tables in two passes against the pairs (REPORT.md, round 3), re-pinned identity once, and re-rendered the six pairs into `.flow/evidence/fn34/round3/`. The owner's verdict on the round-3 pairs is the open item; the birch's second stem and weeping curtain are gaps, not values.

The host reviewed the worker's range from 3468ef7, ran the core, render, typecheck, clippy and browser gates (all pass), removed the dead ash fixture and rebased the branch onto master. R3 closes only on the owner's visual verdicts: twelve stills under `.flow/evidence/fn34/stills/`, three fixed seeds per species in whole and bare views, judged beside the references in `.refs/fn34/`. The owner records a verdict per species in the spec; on accepting verdicts the host runs `flowctl done`. Ash's template and the growth-trait rewrite wait for fn-33 and fn-30 and are outside this task.

## Done summary
The silver birch ships as a catalogue species, accepted by the owner at round 25 after matched-shot rounds 2 to 25; the veins' tone and relief are fn-60's. The European beech is held out of the public catalogue (`params::IN_WORK`, ABI id 5 reserved) and continues in fn-62 with fn-61; the European ash is fn-56's. All forty-eight protocol cases pass at round 25, none capped.
## Evidence
- Commits:
- Tests:
- PRs:
