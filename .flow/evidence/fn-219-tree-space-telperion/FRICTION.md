# fn-219 friction

## 2026-10-09: four rounds tuned a face the cameras barely see

- **Doing:** task 1, setting Telperion's leaf front and underside colours on stills.
- **Hindrance:** rounds 2 to 4 and 8 to 9 moved the front and underside rows with little visible change at the whole-tree and mid views. A diagnostic round with the underside tinted red showed that those cameras, standing below the crown, see mostly undersides, so the crown's colour at distance is the underside's. Nothing in the example or the sheet says which face a view shows.
- **Cost:** about 5 render rounds, roughly 10 minutes of GPU-lock time and the tokens of 7 sheets.
- **Remedy:** a face mask beside the organ mask (the share of a still's leaf pixels drawn as fronts), printed by the sheet example per view, would have shown it in round 1.

## 2026-10-09: the age ceiling is found by failing renders

- **Doing:** checking whether an age past 120 gives the "very old" beech Astra asked for.
- **Hindrance:** age 160 is refused at grow (`engine phytomer budget`) and age 140 grows its off tree in 18 s and then is refused at draw on the on tree (`wood points buffer needs 330311680 bytes, above the device's limit of 268435456`). Nothing states the largest age a preset with a scale can reach before the build is tried.
- **Cost:** about 3 minutes and two failed render runs.
- **Remedy:** a cheap size estimate (phytomers and wood points at a given age and scale, from the grown tree's counts before meshing) that the sheet example prints, or a family check that refuses an age and scale over the device's limit before the build.

## 2026-10-09: the device route's capacity is learnt after the look is settled

- **Doing:** R4, timing Telperion on the harness's route (`prepare_timing`, resident delivery) after eleven rounds of stills at age 120.
- **Hindrance:** at the preset's default seed 42 the route refuses him (`DeviceRefused ... Out of Memory`, RTX 3080, 10 GB), and refuses the plain beech at 120 the same way; the headless stills draw through the CPU mesh and never meet that limit. Age 110 prepares at seeds 1, 7 and 42 in 5.5 to 6.9 s. The stills had to be re-rendered and re-judged at 110.
- **Cost:** about 10 minutes and one more judging round.
- **Remedy:** the sheet example could prepare each tree on the resident route as well and print its time, so a sheet that the harness cannot show fails in round 1.

## 2026-10-09: the browser's age ceiling is learnt from npm test, after review-ready values

- **Doing:** the gate's `npm test` after values settled at age 110 on stills, Astra and the device route.
- **Hindrance:** the Wasm route refuses him (`the tree keeps more than 4,000,000 phytomers, more than a browser prepares in its 4 GB Wasm memory`, `WASM_KEPT`), so the parity and field tests fail. Probing by rebuilding the Wasm binding per age found the ceiling at 92 (93 refused). Three ceilings (Wasm 92, device memory between 110 and 120, engine budget under 160) are each met by a different failing run, the tightest one last.
- **Cost:** about 25 minutes: four Wasm probes and a second set of value rounds at the new age.
- **Remedy:** a native check that reports a family's foreseen kept phytomers against `WASM_KEPT` (the predictor already exists, `pipeline/space.rs::foreseen`), run by the sheet example, would have bounded his age before round 1. fn-209's fine path is noted in the code as lifting the bound.

## 2026-10-09: an all-SHIP review's finalize refused its own merge plan

- **Doing:** finalizing the codex impl-review fan-out, three draws all SHIP with no findings.
- **Hindrance:** `impl-review-fanout-finalize --merge-plan` with an empty keep list failed: the correctness draw ended without a findings container (`no-findings-container`), so the plan "cannot parse draw correctness". The finalize took a hand-written `--merged-file` instead.
- **Cost:** about 3 minutes and two extra finalize calls.
- **Remedy:** a draw whose verdict is SHIP and whose text reports no findings could count as an empty findings list for the merge plan.
