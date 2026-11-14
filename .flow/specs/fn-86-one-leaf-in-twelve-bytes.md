# One leaf in twelve bytes

## Goal
A leaf transform was stored in 64 bytes but carries 12 bytes of information (a rotation, a uniform scale and a translation); on the spruce at seed 1 that is 470.6 MB of a 639 MB specimen. The forest target (a thousand trees, vegetation inside 4 ms on an RTX 3080, about 1 MB of leaf data per hero tree) needs the factor. After fn-85, peak RSS was spruces 3,244 MB, beeches 3,945, oaks 1,469, birches 778. This spec stores a leaf as three `u32` words on CPU and GPU.

## Decisions
- Owner, 2026-09-19: go straight to the 12-byte quantised form rather than a 32-byte similarity first, so the reading sites and three shaders are rewritten once.
- Owner, 2026-09-19: byte identity is not a requirement while the generator is in development; the committed digests are a change detector and a deliberate change recommits them.
- Owner, 2026-09-19: the rotation bound moved from 0.002 rad when thirty bits proved unable to reach it; it then moved to the analytic 0.003 (`3 * sqrt(2) * 6.91e-4`) after the preset round trip found the 4,096-orientation sweep undersampled (0.00250 sweep vs 0.00271 on 13 million real placements).
- Owner's host, 2026-09-19: R4 and R8 cover the four species the generator can express; the European ash has no preset and belongs to fn-56.
- Layout: word 0 a smallest-three quaternion (2-bit index, three 10-bit components); word 1 x,y as `u16` unorm; word 2 z as `u16` unorm plus scale as `f16`. The reference box is derived from parameters (envelope, curtain band, largest radius), so cached growth placements decode at any age. A `mat4x3` was rejected because WGSL pads it back to 64 bytes.
- The parameter-derived box and the third shader (`shadow.wgsl`) were found in fn-85 and escalated rather than guessed. Spec sized to one session on purpose.

## Acceptance Criteria
- **R1:** One leaf is 12 bytes on CPU and GPU — met.
- **R2:** Round trip within half a code step plus one ulp on position (worst 0.2527 mm, beech), 0.003 rad rotation, 1e-3 relative scale, over the shipped presets' own placements — met.
- **R3:** The reference box comes from parameters alone and is identical at every age — met.
- **R4:** Oak, spruce, beech and birch pass `species:qa` unchanged; retained count moves under 0.1% — met (96/96 cases; worst seed beech +0.00170%, birch -0.00177%; oak and spruce unmoved).
- **R5:** `digests.json` recommitted with the encoding; old/new digest named per species — met (49 of 49 moved; seed 1 beech `f160e7b5d26e8b5f` -> `20f49a7c4e806880`, spruce `ff7036aa4bcde448` -> `91d4655c94fa66ac`, oak `648a3aaf8592e555` -> `d2e207bdf8d77c11`, birch `e20e3d6368f80ac2` -> `cac3ef4d4fd5ca39`).
- **R6:** All three shaders read the new layout; cached growth placements decode at a later age — met (birch 16 -> 18: 407 of 744 leaves served from cache, standoff exact; an age-dependent box drifts up to 2.85 m).
- **R7:** `fixed_spruces` peak RSS below 1,800 MB — met.
- **R8:** One headless still per expressible species after the numeric gates pass — met.

## Result
Encoding landed in 5b49227c. Peak RSS (ci, VmHWM, one test per process): spruces 3,244 -> 1,763 MB (-46%), beeches 3,945 -> 2,589 (-34%), oaks 1,469 -> 1,271 (-13%), birches 778 -> 704 (-10%). Stills (960x720, RTX 3080, seed 1, captured at a5c16243) draw exactly the retained counts `species_measure` reports (oak 715,065, spruce 7,353,754, beech 4,928,812, birch 226,057 leaves), each with foliage shadows from the same words. `catalogue:check` had gone red because pins moved without regenerated pages; regenerated in 50f7e9d5.

## Open
Cluster cards, detail levels, LOD tiers, impostors and streaming placement, each its own spec; the wood mesh (168.3 MB of the spruce).

## Friction
- Fresh worktree had no build cache -> pointed `CARGO_TARGET_DIR` at the main target; proposed the worktree kit set it (later superseded by the rule that checkouts never share a target).
- Stated rotation bound and bit budget disagreed (~25 min) -> bound derived from the worst case.
- Eight criteria in one box -> spec sized to one session.
- No before-count recorded for R4; a second worktree and run at base (~25 min) -> proposed `species:qa --measure-only` writing a count summary and diff.
- Catalogue pages did not follow `pins.json` -> proposed regenerating pages where pins are written.
- `/usr/bin/time` missing -> local setup.

## Evidence
- `.flow/evidence/fn-86-one-leaf-in-twelve-bytes/stills/oregon-white-oak-1-whole.jpg`
- `.flow/evidence/fn-86-one-leaf-in-twelve-bytes/stills/norway-spruce-1-whole.jpg`
- `.flow/evidence/fn-86-one-leaf-in-twelve-bytes/stills/european-beech-1-whole.jpg`
- `.flow/evidence/fn-86-one-leaf-in-twelve-bytes/stills/silver-birch-1-whole.jpg`
