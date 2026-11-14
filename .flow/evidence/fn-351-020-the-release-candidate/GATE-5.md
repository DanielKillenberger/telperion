# GATE-5: the five failures of the phase-top gate (fn-351-phase-top at d9228bdd8), 13 Nov 2026

## Still failing, and why

Three are still failing. None of them is a code defect I could fix without changing a tree or a test's promise, so I fixed nothing.

- **`bark_occlusion` (the furrow follows the sun):** fn-361's oak row broke it. The trunk at 2 m got thinner, and on it the occlusion term takes much less light. Reframing the camera does not bring it back. This needs a host decision.
- **`bark_distance`:** it already failed on the base, fn-360, at 6.00 against a 4x limit of 5.8. fn-361 raised it to 6.64. This is the "Product, as measured" item fn-354's GATE-FAILURES.md returned.
- **`curve` (the device matches the CPU reference):** it fails the same way on all four branches. It is the after-hero budget item that GATE-FAILURES.md returned.

The two time-outs were not regressions. A desk freeze landed inside the gate's run of both tests. Alone, both pass well inside their limits on every branch.

One correction to the brief: the failing `bark_distance` row is the 4x one, whose limits are a 5.8 mean and a 15.25 p95. The 5.2 limit belongs to the 2x row.

Commit `a56842ee7` on `fn-351-phase-top` holds only a FRICTION.md entry about the freeze. There are no code or test changes, and the tree is clean.

## Branches

- p3top is `d9228bdd8`, the merge.
- fn-360 is `97aa5bc76`, the first parent.
- fn-362 is `810e90bf3`.
- fn-361 is `949b59a13`.

Every branch was run from its own worktree and its own `target/`.

## 4. `telperion-space::walks a_walk_between_species_changes_the_tree_by_degree`: time-out, the gate's freeze

**What it checks:** three species blends (beech to spruce, spruce to oak, oak to palm). Each must change the tree by degree, with no jump.

**Run alone** through the queue at `--threads 2`, from the `cargo -p` binary `walks-789157725613f891`, with no PAUSE inside:

| Branch | Wall | CPU | Peak |
|---|---|---|---|
| p3top | 26 s | 345 s | 770 MiB |
| fn-360 | 25 s | 357 s | 771 MiB |
| fn-361 | 25 s | 351 s | 768 MiB |
| fn-362 | 26 s | 353 s | 743 MiB |

The gate's own binary (`walks-466a8fe2be3c2929`, a workspace build) also took 25 s wall and 347 s CPU on p3top. All of these pass.

**Cause, from the gate's junit** (`target/nextest/gate/junit.xml`):

- The test ran from 12:15:47 to 12:17:47.
- yield.log has `12:15:53 PAUSE` and `12:16:59 RESUME`, a 66 s freeze inside that window.
- The scope is frozen through `cgroup.freeze`. I saw `cgroup.freeze=1` on my own job during a later PAUSE. nextest's clock keeps counting through a freeze.
- Beside it in the gate ran `walks a_sample_of_settings_in_leaf` (82 s), `walks light_changes` (61 s), `colour_cavity cavity_cuts...` (85 s) and packed_leaf.

**Changed:** nothing.

**Side observation:** CPU is about 14 times wall at `RAYON_NUM_THREADS=2`. Some part of this test sizes its threads from `available_parallelism` instead of the queue's thread count. That is the same on every branch, so it is not new. Which pool does it is unknown.

## 5. `telperion-core suite::packed_leaf::every_station_stands_inside_its_box_at_a_quarter_millimetre_step`: time-out, the same freeze

**What it checks:** across the six presets (ordinary, oak, spruce, beech, birch, palm), every leaf station stands inside its packed box at a quarter-millimetre step.

**Run alone** at `--threads 2` (`telperion_core-74dc39aa8e476c54`). The host's oak census (`census-old heldrun`, 4 threads) was running beside the fn-361 and fn-362 runs and the first fn-362 run, so those are "alone from my side" only.

| Branch | Wall | CPU | Peak |
|---|---|---|---|
| p3top | 48 s | 78 s | 3,247 MiB |
| fn-360 | 51 s | 88 s | 4,415 MiB |
| fn-361 | 60 s | 101 s | 4,377 MiB |
| fn-362 | 56 s and 57 s | 89 s and 91 s | 3,186 and 3,284 MiB |

- fn-361's first run is not timed: 276 s wall with the 13:10:04 to 13:13:42 PAUSE inside.
- All runs pass, against a budget of 139 s.

**Cause:** the gate ran this test from 12:15:25 to 12:17:45, so the same 66 s freeze fell inside it. In GATE-3, with no freeze, it took 116.9 s in the gate.

**Changed:** nothing.

**Side observation:** fn-362 lowers this test's peak by about 1.1 GiB.

## 3. `telperion-render::bark_occlusion the_darkened_side_of_a_furrow_follows_the_sun`: fn-361 moved it; stopped on a question

**What it checks:** the oak at 80, seed 1, with the bare view on the trunk at 2 m. At sun azimuth 45 and 225, the directional-occlusion term must take more than 0.05 luminance from the two strips together. The side it darkens must also swap with the sun.

| Branch | Sun 45, left / right | Sun 225, left / right | Result |
|---|---|---|---|
| fn-360 | 0.0691 / 0 | 0 / 0.0946 | pass |
| fn-362 | 0.0691 / 0 | 0 / 0.0946 | pass |
| fn-361 | 0.0105 / 0 | not reached | FAIL |
| p3top | 0.0105 / 0 | not reached | FAIL |

fn-362 leaves every value unchanged to the digit. fn-361, the oak's `/space/ages/6/form/dominance = 0.5`, moves it.

**Cause, from a probe** (a temporary test file, run and removed, never committed):

- On fn-360 the trunk is 215 px wide on the frame's middle row (columns 136 to 350), and both strips are 100% wood.
- On fn-361 it is 179 px wide (columns 154 to 332). The left strip is 94% wood and the right strip 100%.
- The oak at 80 has 34.1 M wood vertices on fn-361, against 37.7 M on fn-360.
- Losing 6% of the strip's wood cannot explain a drop from 0.069 to 0.0105.

**Reframing test, reverted:** I moved the camera in by the width ratio, from 1.42 m to 1.18 m, which is the move fn-354's comment made. It still failed, at 0.0286 / 0 at sun 45. On fn-361's thinner trunk the term takes much less light, and framing does not restore it. The edit is reverted and the worktree is clean.

Frames: `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/p3top/occ-fn-360-bare45.png` and `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/p3top/occ-fn-361-bare45.png`. The clay frames sit beside them.

**Question for the host:** the term is not broken, but on this tree it is weak. Should the test pick another trunk, height or age, or does the term's strength on a thinner trunk need a look? Is the 0.05 floor still the promise for the new oak? Each of these changes a tree or a promise, so I made none of them.

**Unknown:** why the term is weaker. It could be the bark relief, which may scale with girth, or the viewing angle across a narrower stem. I did not measure which.

## 1. `telperion-render::bark_distance resolved_scales_survive_until_the_two_pixel_boundary`: already failing on the base; fn-361 made it worse

**What it checks:** the shipped oak at seed 7, on a trunk strip at 2 m. The 1600x1000 and 800x500 renders must agree, in mean code values on the masked wood, within 5.2 (p95 14) at 2x distance and 5.8 (p95 15.25) at 4x.

| Branch | 2x mean / p95 / px | 4x mean / p95 / px | Result |
|---|---|---|---|
| fn-360 | 5.171 / 13.00 / 4,150 | 6.004 / 16.25 / 1,563 | FAIL on the base |
| fn-362 | 5.171 / 13.00 / 4,150 | 6.004 / 16.25 / 1,563 | FAIL, identical |
| fn-361 | 5.099 / 12.75 / 3,453 | 6.636 / 17.00 / 1,243 | FAIL |
| p3top | 5.099 / 12.75 / 3,453 | 6.636 / 17.00 / 1,243 | FAIL |

**Cause:**

- On the base, this is the failure GATE-3.md lists (6.0038 / 16.25). It is item 2 of fn-354's GATE-FAILURES.md, returned there as "Product, as measured": the trunk is thinner at the same distance, and moving the camera would change the footprint series the test exists for.
- fn-361 thins the oak's trunk further (the 4x mask goes from 1,563 to 1,243 px), and the 4x mean rises 0.63.
- fn-362's ring sides do not reach this frame. Every value is identical.

**Changed:** nothing. The question is the one already returned to the host.

## 2. `telperion-render::curve the_device_surfaces_the_wood_the_cpu_reference_does`: identical on all four branches; a known product item

**What it checks:** the ordinary tree (the young beech dressing) and the palm at 480x360, at the hero view and then at a close view (0.15 of the hero distance). Each view must be drawn within budget and match the CPU tessellation.

All four branches give the same result. The hero view is at scale 1, with demand [rings, vertices, tube indices, ribbon indices] = [169,134, 510,116, 23,328, 1,285,320]. Then the "ordinary close: over budget" assertion fails, 3 against 0.

**Cause, from a probe on p3top** (temporary test file, removed):

- The close frame's budget is [211,418, 637,645, **29,160**, 1,606,650]. The tube-index budget is ceil(23,328 × 1.25), the hero frame's tube demand plus `HEADROOM` in `crates/telperion-render/src/curve/feedback.rs`.
- The close frame's tube demand is **255,492**. So no scale fits: it draws at scale 8 and leaves a cluster out, which is overrun 3 = 1 | 2.
- Drawing the same close view a second time fits at scale 1 with overrun 0, because its tube budget is now 319,365 = 255,492 × 1.25.

This is item 1 of fn-354's GATE-FAILURES.md, returned there as "Product". It predates all three merges.

**Changed:** nothing. The question for the host is how a view's first frame should be budgeted when its demand jumps from the previous frame's. That is a design matter.

## Unknowns

- Why the occlusion term is weaker on fn-361's trunk (relief or viewing angle).
- Which code makes `walks` use about 14 cores under `RAYON_NUM_THREADS=2`.
- The packed_leaf times on fn-361 and fn-362 were taken beside the host's oak census job, so their absolute seconds may read high. All of them are still at or under 60 s against 139 s.

# Host decisions on GATE-5 (13 Nov 2026): curve test change and the bark_distance split

## What did not come out as expected

- **bark_distance, fn-360, 4x:** the excess is not only in the trunk's outline. The interior alone reads 5.875, which is still over the 5.8 bound. The edge band (6.33) does pull the total up to 6.004.
- **curve:** nothing unexpected. The feedback arrives in one further frame, and the date-palm's close view never passes its capacity.

## A. `curve the_device_surfaces_the_wood_the_cpu_reference_does`: changed, commit `f17affbb3`

The change is to `crates/telperion-render/tests/curve.rs` only. feedback.rs, HEADROOM and every budget are untouched.

- **Hero:** unchanged. Overrun 0, and the frame matches the reference.
- **Close, first frame:** the test asserts that a report exists and that its scale is one `choose` returns: 1, 2, 4 or 8, since `reported` decodes `1 << min(chosen, 3)`. It prints the scale, overrun, budget and demand.
- **Close, same view again:** overrun 0 and `compare` at the reported scale, as the test asserted before.
- **Doc comment:** two lines added, pointing to `src/curve/feedback.rs` (fn-258).

Measured on p3top, where all four curve tests pass:

| Tree | Frame | Scale | Overrun | Notes |
|---|---|---|---|---|
| ordinary | close, first | 8 | 3 | tube-index budget 29,160 against demand 255,492; 9,660 + 208 triangles drawn |
| ordinary | close, next | 1 | 0 | 175,698 vertices, 85,164 + 138,216 triangles; gap 9.54e-7 m, cosine 1.000000 |
| date-palm | close, first and next | 1 | 0 | |

The feedback took one further frame to arrive on this device.

**For the owner's list:** on the first frame after the camera jumps to the close view, wood is left out of the picture. Overrun bit 2 is "a cluster past the budget was left out", and the frame draws 9,660 + 208 triangles at scale 8 against 85,164 + 138,216 the next frame. This lasts one frame: a one-frame pop.

## B. `bark_distance` at 4x: edge band against interior (measurement only, probe removed, trees clean)

### How the test builds its mask

`common/resolution.rs::wood_mask` takes the 800x500 clay frame and counts a pixel as wood when red is greater than blue. It keeps a pixel only if every pixel in the 9x9 square around it (±4 px) is warm. So the mask is the silhouette eroded by 4 px. The test then keeps rows 200 to 299 and the columns within 400 ± 100/distance: ±50 at 2x, ±25 at 4x.

### How the probe split it

The edge band is the mask pixels with any pixel outside the full eroded mask within ±2 px (Chebyshev), at the low resolution. Cutting to the strip does not count as an edge. The interior is the rest. The figures are per-channel absolute differences of a 2x2 box of the 1600x1000 frame against the 800x500 frame, as in `masked_agreement`. The probe's totals reproduce the test's own figures exactly on both commits.

### Results

Trunk width is the unmasked warm run through x = 400 on rows 210, 250 and 290 of the low-resolution frame. Each cell gives pixels, then mean / p95 in code values.

| Commit | Distance | Trunk width (px) | Mask | Edge band | Interior |
|---|---|---|---|---|---|
| fn-360 `97aa5bc76` | 2x | 67 / 50 / 53 | 4,150: 5.171 / 13.00 | 482: 6.067 / 15.75 | 3,668: 5.053 / 12.50 |
| fn-360 | 4x | 37 / 24 / 24 | 1,563: **6.004** / 16.25 | 439: 6.334 / 17.25 | 1,124: **5.875** / 15.75 |
| gate-base `5ebab2de0` | 2x | 141 / 138 / 109 | 9,972: 4.420 / 12.75 | 186: 7.168 / 18.25 | 9,786: 4.368 / 12.50 |
| gate-base | 4x | 79 / 68 / 56 | 4,625: 5.521 / 14.50 | 367: 4.397 / 11.75 | 4,258: 5.617 / 14.50 |

The bounds are 5.2 mean and 14 p95 at 2x, and 5.8 mean and 15.25 p95 at 4x.

At 4x on gate-base the trunk (68 px) is wider than the 50 px strip, so most of the mask is interior. On fn-360 the 24 px trunk sits wholly inside the strip, and after erosion about 28% of the mask is edge band.

### The bark parameter that scales feature size with girth

The oak's material rows are identical on both commits: `ridgeScale` 0.032, `plateScale` 0.055, `plateCellScale` 0.055. The shader scales plate size by girth:

- `bark.wgsl` line 158 and `plates.wgsl` line 301: `girth = 0.3 + 0.7 * smoothstep(2.5, 20.0, radius / ridge_scale)`.
- `plates.wgsl` line 149: `bark_plate_size = cell_scale * girth`.
- Plate depth (`bark_plate_depth`, `plates.wgsl` line 316) and the colour range (`bark.wgsl` line 171) scale with girth too.
- `maturity = smoothstep(2, 5, 2r / ridge_scale)` is 1 on both trunks.

I estimated each trunk's radius at 2 m from its pixel width at row 250. One pixel at the low resolution is 6.8 mm at 2x and 13.6 mm at 4x (vertical field of view 38° over 500 px). These are pixel estimates, about ±1 px; I did not read them off the mesh.

| Commit | Distance | Radius | r / ridge | Girth | Plate width | Plate at 800x500 |
|---|---|---|---|---|---|---|
| fn-360 | 2x | 0.170 m | 5.3 | 0.348 | 19.2 mm | 2.8 px |
| fn-360 | 4x | 0.163 m | 5.1 | 0.342 | 18.8 mm | **1.4 px** |
| gate-base | 2x | 0.469 m | 14.6 | 0.843 | 46.4 mm | 6.8 px |
| gate-base | 4x | 0.462 m | 14.4 | 0.833 | 45.8 mm | 3.4 px |

At 4x on fn-360's trunk, a plate spans about 1.4 low-resolution pixels. That is under the two-pixel boundary the test is named for. On the base trunk it spanned 3.4 px.

**Inference, not measured:** this would explain why the interior's error rises even with the outline excluded. I did not test it, for example by holding `girth` fixed in a probe.
