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
