# A run's rings have the sides its widest ring can show

Child of fn-351 (P1, P3; the gate's memory items). Host-written, 13 Nov 2026. Builds on the phase's top branch (`fn-360-a-runs-girth-eases-wherever-it-drops`).

## Problem

Measured (`.flow/evidence/fn-351-020-the-release-candidate/BEECH-VERTICES.md`, the shipped European beech at 80, seed 1, the CPU mesh): 101.8 M wood vertices where the base drew 35.2 M; 73.5 M of them in rings of 1 to 2 mm drawn radius and 18.3 M under 1 mm; 74.8 M on short-shoot nodes. Every ring has 20.1 vertices whatever its radius: a twig 1.5 mm in radius is cut with the trunk's 20 sides. The build peaks at 5,875 MiB against 2,146, and the gate's memory items follow from it (`GATE-3.md`: `smooth_bark` 5,817 against 4,096 MiB, `form the_beech` 8,631 MiB, `ground_reach` 6,035 against 5,184).

The twigs are right: the plan's P15 records that the base's beech carried a fifth of a real beech's leaf, and the wood that costs is what bears the added leaf. What is wrong is cutting 2 mm wood like 1 m wood. The pipeline already holds that wood under `FINEST` = 0.2 mm "and all it bears" is not drawn (`pipeline/space.rs`): a deviation the drawing cannot show.

Not checked by the host, for the builder's reading (R1): where the CPU mesh takes its sides (`radialSegments`, `pipeline/surface/`, `chunks.rs`), whether the GPU's curve path reads the same count or its own, every consumer of the ring's vertex count (the bark's coordinates round the stem, caps, the collar and socket joins, the field, the wire and distant levels, any test that counts sides), and how a child run joins a bearer whose ring has a different number of sides.

## The design (host)

A run is cut with as many sides as its widest drawn ring needs for its polygon to stay within `FINEST` of the circle, and never more than the preset's `radialSegments`:

- For a ring of drawn radius r cut with n sides, the polygon's flat lies `r (1 - cos(pi / n))` inside the circle. The sides a radius needs are the least n of at least 3 with `r (1 - cos(pi / n)) <= FINEST`.
- A run takes one count for all its rings: the count its widest drawn ring needs, capped at `radialSegments`. No ring-to-ring change of count, so no stitching between unlike rings inside a run.
- No new value: `FINEST` is the drawing's existing floor and `radialSegments` the preset's existing row, now the cap.

On paper (FINEST 0.2 mm, cap 20): r = 0.25 mm needs 3; 0.5 mm 4; 1 mm 5; 1.5 mm 7; 2 mm 7; 5 mm 12; 1 cm 16; 1.5 cm 20; anything wider is capped at 20, so trunks, limbs and every run wider than 1.5 cm are cut exactly as today. Expected on the beech at 80, if runs of 1 to 2 mm take 6 to 7 sides and runs under 1 mm 3 to 5: the 73.5 M vertices of the first class fall to about 25 M and the 18.3 M of the second to about 4 M; the total from 101.8 M to roughly 40 M. The builder holds the build against these numbers and returns if they are off by more than a quarter.

Drawing only. The grown tree, its nodes, its radii, the leaves and every digest upstream of the surface stage do not change.

## Acceptance Criteria

- **R1.** The reading above, in the evidence, before any product change; with the answer to whether the GPU path is touched at all.
- **R2.** Tests, red before and green after: a hand-built run of radius 1.5 mm is cut with 7 sides and one of 1 m with the preset's 20; every ring of a run has the same number of sides; the flat of every ring of a mixed tree lies within `FINEST` of its circle or the ring has the cap's sides.
- **R3.** The existing surface, bark, curve and mesh tests pass, or a test that changes is named with what it checked. A test that compares the GPU's wood with the CPU reference still compares the same wood.
- **R4.** Measured before and after, the CPU mesh: the beech at 80 and at 40 on seed 1, the oak at 120 on seed 1001, the spruce at 40, Telperion and Laurelin at their preset ages: wood vertices in the radius classes of `BEECH-VERTICES.md`, peak memory of the build, one-thread seconds of the surface stage. Then the gate's memory items alone, each against its budget: `smooth_bark`, `form the_beech_grows_at_every_age_of_its_sheet`, `ground_reach`, the two Wasm entry tests named in `GATE-FAILURES.md`.
- **R5.** Stills on one sheet, before beside after, close enough to resolve a twig: a beech short shoot with its leaves, an oak twig, a spruce shoot, and the oak's first fork (which must not change).
- **R6.** Every digest pin that moves is listed with the tree it pins. Pins upstream of the surface stage do not move, and that is checked.
- **R7.** `docs/pipeline.md` states the rule.

## Stops

- A join between a child run and its bearer that opens a visible gap or needs a new kind of stitching: return with the site and a still.
- The GPU path's drawn wood changing where the reading said it would not.
- A count for a run that the rule above does not decide (a run with no drawn ring; a degenerate radius): return with the case.
- The measured vertices off the paper numbers by more than a quarter.

## Boundaries

The surface stage's ring count only. No change to which wood is drawn, to the rings along a run, to the grower, to any preset, to the GPU's level of detail. No new constant.

## Rigor

Opus builds; narrow tests while building; heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, one GPU job at a time, under the yield watcher (jobs in the background, waited for; seconds only with no PAUSE inside). The host views the sheet; the gate and review at the phase's landing.
