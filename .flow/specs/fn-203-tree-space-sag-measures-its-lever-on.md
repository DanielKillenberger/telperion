# Tree space: sag measures its lever on the bent branch

## Goal

fn-200's sag takes each phytomer's bending moment on the unbent tree (small-deflection beam theory). The spruce's round 9 (fn-194) gave its old lower boughs the heavy sag S1's sweep needs, and there it failed: a bough hanging near vertical kept its full horizontal lever, bent past vertical and, with tip tropism, looped wood round the trunk base. In a real beam the lever shrinks as it droops, so a heavy branch hangs and stops (host, 2026-10-05). Geometry only, in the sag pass; no reaction wood.

## Decisions

- Design (host, 2026-10-05): one march base to tip; the load beyond each phytomer is rotated rigidly by the upstream bends and the lever is its horizontal distance in that pose. No iteration; the 0.15 rad stop stays as backstop only; tip tropism unchanged; sag 0 neutral; heavy-sag trees change by design, the beech (sag 0) byte-identical.
- Decision 1 (host, 2026-10-05): the lever only ever shrinks — the moment is the smaller of the turned and unbent levers, in the turned direction, so upright wood does not buckle.
- Decision 2: the ground carries the load — wood past where it rests adds no lever, and its direction stays on the ground (extends fn-200).
- Decision 3: R1 reworded into (i) free-air convergence and (ii) round 9's ground-coil as the red-first test.
- Decision 4: one landing bound, 0.8 rad, for every sag (0.72 measured at the spruce's sag, 0.52 at sag 5.0); the bound guards regressions, the stills judge the look.

## Acceptance Criteria

- **R1:** (i) a heavy free-air branch converges toward hanging; (ii) a heavy low bough on the ground does not coil, red on the small-deflection sag — met ((ii) winds 1.61 rad on the old sag, 0.02 to 0.17 now, bound 1.2).
- **R2:** A light branch bends within a small tolerance of the small-deflection result — met (within 1 percent of fn-200's bends from `0706c79c`).
- **R3:** fn-200's walks and every crate test green, landing corner within 0.8 rad, walk bound 30 on `form.sag` up to the spruce's heaviest — met (worst landing 0.717 at 6e-4).
- **R4:** Spruce round-9 values at seeds 1 and 4, close-ups and whole tree, viewed: no loops — met.
- **R5:** Workspace gate; Codex review — met (1,038 passed; Codex SHIP round 2).

## Result

`sag::levers` keeps each phytomer's moment and chord on the unbent tree and turns both rigidly when re-laid; `sag::turn` integrates each turn exactly so it cannot overshoot hanging; `sag::held` lets the ground take its share. Codex round 1 fixes: the ground share now falls by degree as a last phytomer's end reaches the ground (chord through continuations; new test `the_ground_takes_a_carried_load_by_degree`), and R2 compares against recorded fn-200 bends. R4 on the final engine: no loops, nothing toppled (21.9 m tall; crowns 17.2 × 19.1 m and 16.7 × 14.3 m), boughs land in smooth arcs and run out radially; the hook is lost, as expected — round 10 retunes. Beech byte-identical. Sheets were in ignored `raw/`.

## Open

The spruce's hook retune is fn-194's round 10.

## Friction

- R1 as written could not be red on the old code (the backstop held free-air branches); ~60 min of probes -> run a spec's repro before marking ready.
- Landing corner under heavy sag unnamed in the design; ~40 min, four approaches -> large-deflection landing belongs in the design (now decision 4).
- telperion-core's fixed-beech memory ceiling (4.41 vs 4.25 GB) failed under the parallel gate, passes alone -> ceiling that holds under gate load, or run serially.
- Render script lost its panic text -> keep the panic in the log.
