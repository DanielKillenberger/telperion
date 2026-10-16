# fn-281 R3 and R4: fix A, measured

The fix is in `pipeline/space.rs`, and `generation/cycle/dress.wgsl` does the same. Above a relay's inside join, the conversion compares the radius it draws, `min(p.radius, above)`, against `FINEST`. Wood thinner than that ends the axis at the join, the rule the module already states. The engine's f32 girth sum is unchanged (host ruling, 2026-10-14). Raw logs are in `raw/` (ignored).

## Red to green

- **CPU, the oak at 80, seed 7** (`radii_tests::the_oak_at_80_seed_7_solves_every_radius`): red on the base (`radii_tests-1.log`), green on the fix (`radii_tests-fix.log`).
- **Device, the oak at 80, seeds 3, 7, 30, 39** (`thin_tests::the_oak_at_80_draws_no_radius_zero_node_on_the_device`):
  - On the base shader, seed 3 draws 2 nodes at radius zero, as the CPU did (`device-thin-base.log`).
  - On the fix, none of the four seeds draws one (`device-thin-fix.log`).
  - The test is held to the device budget, as a build is: without the cap, `tree::drawn` was refused for memory.
- **Device and CPU node counts at 80 differ by a few nodes in 2.3 million, before the fix as after it.**
  - Seed 3: the device drew 2,329,637 nodes on the base against the CPU's 2,329,623 (+14), and 2,328,959 on the fix against 2,328,944 (+15).
  - Seeds 7, 30 and 39 on the fix: +19, +35 and -69.
  - The young agreement test (age 12, `every_preset_is_drawn_as_the_cpu_converts_it`) still holds counts equal (`agree-fix.log`).
  - Where the counts part at 80 was not traced. It predates fn-281.
- **Focused suites:** the core's `pipeline::space` tests pass, 24 of 24 (`core-space-tests.log`).

## The sweep: no unsolved radii

The digest sweep covered the same 20 shards at seeds 1 to 50 (`sweep.sh`), from the base (`raw/base/`) and from the fix (`raw/fix/`).

- **Unsolved radii:** there are none on the fix, so the oak at 60, 70 and 80 builds wherever the base refused it.
- **The engine's phytomer budget:** the refusals are unchanged. The oak is refused at 80 (6 seeds), 90 (39) and 100 (46). The host left this to fn-236's new values.

## Bytes

`compare.py` holds each tree's digest on the fix against the base's: 487 trees are unchanged and 463 change. The per-tree list of changed trees and nodes removed is archived. Every change is the expected one. In every tree that changed, the fall in the count of nodes thinner than `FINEST` equals the nodes removed, with no exception. The palm and the Norway spruce do not change at all.

| Preset and age | Trees changed of 50 | Nodes removed | Share of the tree, most |
|---|---|---|---|
| Oregon white oak 40 (own) | 50 | 13 to 122 | 0.027% |
| Oregon white oak 60 | 50 | 58 to 504 | 0.033% |
| Oregon white oak 70 | 50 | 87 to 665 | 0.029% |
| Oregon white oak 80 | 44 | 126 to 679 | 0.029% |
| Oregon white oak 90 | 11 | 168 to 421 | 0.022% |
| Oregon white oak 100 | 4 | 171 to 389 | 0.017% |
| Ordinary 40 (own) | 48 | 1 to 11 | 0.008% |
| Ordinary 80 | 50 | 28 to 73 | 0.006% |
| European beech 40 (own) | 14 | 1 to 4 | 0.004% |
| European beech 80 | 38 | 1 to 16 | 0.001% |
| Silver birch 40 (own) | 22 | 1 to 4 | 0.001% |
| Silver birch 80 | 50 | 1 to 14 | 0.001% |
| Telperion 82 (own) | 14 | 1 to 10 | under 0.001% |
| Telperion 80 | 10 | 1 to 10 | under 0.001% |
| Laurelin 80 (own) | 8 | 1 to 9 | under 0.001% |

At 90 and 100 only the oaks that grew are compared; the others are refused by the budget on both sides.

## Shipped presets at their own age

I dressed the most-changed tree of each preset at its own age on the CPU, base against fix (`mesh-own.log`, `radii_sweep ... mesh`):

| Tree | Wood triangles | Leaves | Bounds |
|---|---|---|---|
| Oak 40, seed 34 | 19,761,320 to 19,751,560 (-0.05%) | 773,037 to 772,852 (-185, 0.024%) | identical to 0.1 mm |
| Ordinary 40, seed 3 | -880 | -11 of 14,384 | identical |
| Beech 40, seed 37 | -320 | none | identical |
| Birch 40, seed 2 | -320 | -8 of 179,786 | identical |
| Telperion 82, seed 49 | -760 | none | identical |
| Laurelin 80, seed 25 | -720 | none | identical |

The wood removed is under 0.2 mm in radius. At most it takes 0.024% of a crown's leaves, and the tree's bounds do not move. On these numbers no shipped preset changes visibly at its own age, so no stills were rendered. That judgment rests on counts, not on a viewed still.

## R4: the 486 nodes thinner than FINEST in the oak at 80, seed 7

The fix removes all 486, along with the one at radius zero: 487 nodes, from 2,193,278 to 2,192,791. The conversion's tree on the fix draws no wood thinner than `FINEST` (`radii_tests::where_the_oak_at_80_seed_7_draws_wood_thinner_than_finest`, `radii_tests-fix.log`). There is no remainder.

Across the sweep, one tree still draws such wood after the fix: the ordinary tree at 80, seed 48, keeps one node thinner than `FINEST`, down from 43. Its source was not classified. The oak's classification test reads one tree.
