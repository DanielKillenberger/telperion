# fn-277 R1 and R2: the jumps and the rows behind them

Measured 2026-10-14 on 850eb039a with `crates/telperion-core/examples/walk_leaves.rs` (`--profile ci`), each frame `blend::families(a, b, t)` built by `mesh::build`, as the film builds it. `t` is the share the walk is asked at (the film's `at` column, already eased). `WALK_ISOLATE=1` builds the frame after a jump again with each whole-number row that stepped put back, one at a time.

## R1: the jumps, standalone

`crates/telperion-core/tests/walk_leaves.rs`, red on 850eb039a (101 s, three tests). Every jump falls at the same `t` at seeds 1, 7 and 42, so each is a row crossing a threshold, not a seed's noise. Off the jumps, consecutive frames in these windows differ by at most 10 %.

| walk at 40 | between t | leaves (seed 1, 7, 42) | row that steps |
|---|---|---|---|
| oak to palm | 0.004, 0.005 | 314,715 to 610,192; 453,338 to 923,410; 331,153 to 658,552 | `/canopy/leafletCount` 1 to 2 |
| oak to palm | 0.011, 0.012 | 537,216 / 864,892 / 570,808 to 4 at every seed | `/canopy/rosetteFronds` 0 to 1 |
| spruce to birch | 0.166, 0.168 | 672,715 to 504,844; 689,442 to 533,043; 694,161 to 538,531 | engine `cluster_leaves` 0 to 1 (PAs 12-14) |
| birch to oak | 0.832, 0.834 | 96,271 to 167,616; 163,763 to 265,617; 118,700 to 206,235 | engine `cluster_leaves` 1 to 0 (PAs 12-14) |

Isolation (seed 7): putting `leafletCount` back to 1 gives 461,705 (from 923,410); `rosetteFronds` back to 0 gives 862,122 (from 4); `cluster_leaves` back gives 685,530 (from 533,043) and 160,478 (from 265,617). The other whole rows that step at the same frames (`lateralsPerStation`, `twigs/laterals`, `clump`) leave the count unchanged: the engine path does not read them.

## R2: the rules, checked against code

1. **`rosetteFronds` is a switch at zero.** The row walks as `Blend::Count` (`catalogue/walk.rs:17`, linear then rounded): 0 to 42 reaches 1 at t = 0.5 / 42 = 0.0119. `rosette::bearing` (`pipeline/foliage/rosette.rs:60`) is `rosette_fronds > 0`, and while it holds `plan::bearing_runs` returns no run for the whole tree (`pipeline/foliage/plan.rs:446`, "a stem that bears a frond crown bears nothing along its length") and `plan` places fronds alone (`plan.rs:377`). One frond at each of the frame's two stem apices, of two leaflets, is the 4 leaves. The row's own note states the switch: "any rise makes the rosette the tree's only foliage" (`canopy/params.rs:174`).
2. **`leafletCount` is a whole multiplier gated at one.** `Blend::Count`, 1 to 113, reaches 2 at t = 0.5 / 112 = 0.0045. `rosette::leaflets` (`rosette.rs:93`) returns `leaflet_count` once it is above 1 and `rachisLength` is above 0 (the palm's 7 m walks in from the oak's 0 continuously), and every clothed placement is multiplied by it (`plan.rs:365`). The step from 1 to 2 doubles the oak's whole crown; had the rosette not cut it at 0.012, 3 leaflets at t = 0.0134 would triple it.
3. **Engine `cluster_leaves` is a switch at zero.** It walks on the `Whole` scale (`telperion-space/src/species/scale.rs:71, 135`, value then rounded, fn-223's fix for validation), birch 3 to spruce and oak 0, so it crosses 0.5 at t = 1/6 and 5/6 and steps again at 0.5. Above zero an axis of that PA bears its leaves as a cluster at its tip and none along its length (`species.rs:149`, `unfold.rs:342`, `pipeline/space.rs:201`). The film's other steps on these walks are the same row: 1 to 2 at t = 0.5 (spruce to birch frame 344, +23 %; birch to oak frame 435, -13 %), 2 to 3 at 5/6 (frame 366, +17 %), 3 to 2 at 1/6 (frame 413, -17 %).

Not a jump: birch to oak frames 462 to 463 (-19 % at t 0.906 to 0.916) is a steady slope at 0.002 steps (290,623 to 233,089 in five steps of 4 to 6 %), and no whole row steps there.

Same pattern, not yet seen to jump: `/canopy/skirtFronds` (`Blend::Count`, 0 to 16) adds dead fronds from t = 0.031 under the oak's own `skirtLength` 0.8; `leafBases` and `acanthophylls` are other `Blend::Count` canopy rows a frond crown reads.
