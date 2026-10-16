# fn-257 R7: the field's resident data written compact (worker, 2026-10-16)

The field's primitives are written in their kept layout where they are made (`field_build.wgsl`'s `wood`, `leaves`, `leaflets`), never converted afterwards. No f32 copy of them is made. The kernels run twice in one submission. The first pass grows the field's box. The second writes each primitive as fractions of that box, which it reads from the work words on the device. `field.wgsl`'s `endpoints`, `reach`, `attributes` and `dead` are the one decoder, read by `span`, `count`, `fill` and `query`.

## The layout: 16 bytes a primitive (was 32), systems for the leaves alone

| Bits | Wood (a drawn node) | Leaf segment | Frond leaflet |
|---|---|---|---|
| `x`, low 28 of `y` | its node, 3 × 20-bit fractions of the box | its first end, the same | the same |
| `y`, high 4 | unused | the shift of its second end's counts | the same |
| `z`, low bit of `w` | its parent's primitive (`NONE`: a point); the sweep runs from that parent's position | second end minus first in 3 × 11-bit signed counts of 2^shift steps | the same |
| `w`, 30 bits | start and end radius, each a positive half (15 bits) | reach (positive half), leaves (15 bits) | reach (positive half), flank 2 × 5-bit octahedral, thickness share 5 bits |
| `w`, high bit | stands for nothing (undrawn); the node's position stays for its children | a frond's segment, which stands for none | |

The owner (limb system) array now holds the leaf primitives only, indexed from the first: 4 B × leaves, where it was 4 B × every primitive.

## Precision

- **Positions:** a step is the box's extent / 1,048,575 on each axis, about 0.03 mm on a 30 m box; error at most half a step. The wood takes both ends at this step (the far end is its parent's own position).
- **A leaf's or leaflet's second end:** exact to the step for differences under 1,023 steps (about 3 cm on a 30 m box); past that, at most half of 2^shift steps (a 0.5 m segment on a 30 m box: shift 5, 0.46 mm).
- **Radii and reach:** half precision, relative 2^-11 (0.05 %).
- **A leaflet's flank:** 5-bit octahedral, a few degrees. Its share is in 31ths. The palm's leaflets are held to its own looser tolerance (their draws differ from placement's anyway, `field_tests::within`).

**A first cut, measured and replaced:** 16-bit fractions for both ends of every primitive, attributes as full halves. The spruce came to 57.1 MB and seed 1 held, but Telperion seed 5678 at 36 cells read 0.0051 cells off in radius against a tolerance of 0.005. A run with exact f32 positions beside the 16-bit attributes brought it back to 0.0018. That put the error in the positions, not the halves, and the 20-bit layout above was built to remove it (`raw/t5/after.log`, `after-seeds.log`).

## Bytes and tolerances, seed 1 (`the_device_field_answers_as_the_cpu_field`, `raw/t5/before.log`, `raw/t5/after20.log`)

| Preset | Field MB before | after | Worst radius off at 24/36/48, before → after | Worst foliage J, before → after |
|---|--:|--:|---|---|
| ordinary | 5.0 | 2.6 | 0.0006 → 0.0006 | 1.0000 → 0.9999 |
| oak | 18.0 | 9.4 | 0.0007 → 0.0008 | 0.9999 → 0.9999 |
| spruce | 100.2 | **57.1** | 0.0006 → 0.0009 | 0.9998 → 0.9997 |
| beech | 6.6 | 3.4 | 0.0004 → 0.0003 | 0.9999 → 0.9999 |
| birch | 15.4 | 8.3 | 0.0016 → 0.0015 | 0.9999 → 0.9999 |
| date palm | 0.4 | 0.2 | 0 → 0 | 0.7714 → 0.7720 |
| Telperion | 101.8 | 53.0 | 0.0016 → 0.0014 | 1.0000 → 0.9997 |
| Laurelin | 95.6 | 49.8 | 0.0021 → 0.0022 | 0.9999 → 0.9999 |

Every preset is within R2's tolerances at 24, 36 and 48 cells, at seeds 1 and 5678 (`after20.log`). The worst radius reading is Telperion 5678 at 24 cells, 0.0026 against the 0.005 tolerance. The leaves' error and the limbs are unchanged to the fourth place. The spruce's field is 57.1 MB at seed 1 and 56.5 MB at 5678, under the owner's 60 MB.

The gate's test (`every_preset_s_device_field_answers_within_its_tolerance`) now asks all three grids. The run-to-run, crowded-cell and lost-device tests are green. The harness build is untouched: the field is the field entry's alone.
