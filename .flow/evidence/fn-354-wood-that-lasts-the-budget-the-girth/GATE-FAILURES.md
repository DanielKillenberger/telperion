# The full gate's failures on fn-357's head, 12 Nov 2026

The gate ran at 487266f8f (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py --full`) and failed: two assertions, seven tests cut off at 120 s, and a kill at the 8 GB scope with 293 tests unrun. Each failure was reproduced alone below. The base is the integration branch's head, 5ebab2de0. Its code equals the merge base 5d13c5a1e, and it was built in its own worktree and `target/` at `.worktrees/gate-base`.

## 1. The palm's skeleton pin (`runner_accept the_palm_s_pins_are_the_ones_its_catalogue_records`)

**Reading the assertion.** Its left side is the computed value and its right side the recorded one. `catalogue/date-palm/pins.json` records `skeleton` 3095343102689997409, unchanged since e74aee988. The branch computes 2169287505064991713. The base computes the recorded value: the test is green at 5ebab2de0.

**What moved.** The palm's skeleton was dumped at seed 7 on both commits through a scratch example that is not committed. Each node was dumped as its f64 position and its parent.

- Both trees have 7,605 nodes, and every parent is identical.
- 1,469 of the 22,815 coordinates differ, all on 491 alternate nodes between index 482 and 992, at heights from 0.24 to 14.9 m.
- The largest difference is 7.3 nm, a relative difference of 6.3e-8.
- Every other pin is equal on both commits: wood counts, instances, bounds, placement and element.

The palm's shape is therefore unchanged.

**Cause, bisected.** The dump's checksum was compared commit by commit:

| Commit | What it is | The dump against the base |
|---|---|---|
| 933a318c6 | | Equal |
| b9d337a63 | Deletes `girth/disused.rs` while the code still uses it | Does not build |
| a3e16468e | fn-354 R6 step 1, the girth rows | Differs, by up to 5.6 m |
| 3c9b20ad6 | fn-354 R6 step 2, wood laid by the leaf above it | Within 7.3 nm |
| 487266f8f | The branch's head | Same dump as 3c9b20ad6 |

The two girth steps compute the stem's values by a different floating-point path. The later fn-357 and fn-358 commits, a retired root's stand and the phytomers held at f32, do not move the palm.

**Fixed** in a849a825c: the `skeleton` pin was changed from 3095343102689997409 to 2169287505064991713. `runner_accept`'s 6 tests are green.

## 2. `bark_depth the_depth_row_moves_no_wood_and_changes_the_picture`

**Reproduced.** Alone on the branch, the depth row's mean change over the fixture frame is 0.0835/255, the same as in the gate. The base gives 0.4794/255 at the same camera. The render crate's code is the same on both commits; only an example differs.

**Cause.** The oak is seed 7 at its default age. Its trunk at 2 m was about 170 px wide in the frame on the base and is about 75 px on the branch: the bounds show fn-354's wood laid the trunk thinner. Less bark fills the frame, so the depth row changes less of the picture. The stills compared are `probe-base/fixture-plain.png` and `probe-branch/fixture-plain.png` in the dispatching session's scratchpad.

**Fixed** in 5033ed05a. The camera comes in along the same line, from 2.47 m to 1.09 m, at the same target (0, 2, 0) and field of view. The trunk again fills about 170 px of the frame's width: on the branch at 1.09 m, the mean change is 0.3665/255 and the worst 41/255. The promise and its assertion are unchanged: with the row on, the picture of the trunk changes by a mean of more than 0.2/255. The checks that the row moves no wood and that the row at zero returns the starting frame also stand as they were.

## 3. Seven tests cut off at 120 s: waits for the host

Each test ran alone through `telperion-heavy 8 --threads 4`, under nextest's default profile so nothing was cut off. The runs were back to back on each commit. The load average stood at 20 to 65 throughout, from other programs. A run's load was taken at its start; the raw rows are in the dispatching session's scratchpad, `measure.tsv` and `measure2.tsv`. Two more commits were timed in `.worktrees/gate-bisect`: b2b45f3c6, the parent of fn-355's 4ca638dfd, and 4ca638dfd itself.

| Test (wall s) | Base 5ebab2de0 | b2b45f3c6 | 4ca638dfd (fn-355) | Head 487266f8f |
|---|---:|---:|---:|---:|
| `suite::sweep::every_pair_of_presets_grows_a_tree_at_every_step` | 60.9 | 43.9 | 227.4 | 407.8 |
| `suite::sweep::every_wire_parameter_a_preset_moves_is_walked` | 34.6 | 32.2 | 185.4 | 244.6 |
| `catalogue_identity every_walk_is_the_walk_it_was` | 25.7 | 19.3 | 181.1 | 209.3 |
| `space_walks a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80` | 50.5 | 47.9 | 441.3 | 504.0 |
| `space_walks the_engines_walk_converges_on_either_end_at_40_and_80` | 24.0 | 19.7 | 201.2 | 296.8 |
| `space_walks the_grown_count_binds_where_a_blend_sheds_more_than_its_ends` | 25.6 | 24.0 | 122.7 | 172.0 |
| `walk_leaves birch_to_oak_changes_its_leaves_by_degree` | 40.9 | 44.4 | 123.8 | 126.4 |

All seven pass when they are left to finish.

**The cause is fn-355's 4ca638dfd, "a year is a state, the grower reads no horizon": the walk's blend became about 15 times slower.** Before that commit every test is at the base's time. At it, each takes 3 to 9 times as long. On that commit the windows look 40 cycles ahead, and the expected-length table runs to the year plus 40.

- **The blend alone.** Three blends of the oak to the beech, at 0.13, 0.5 and 0.731 at the presets' ages, took 1.9 s on the base and 34.3 s on the head, back to back at load 60.
- **Bisect.** This timing was bisected with good meaning under 6 s:

  | Commit | Blend time (s) |
  |---|---:|
  | b2b45f3c6 | 1.8 |
  | 4ca638dfd | 30.1 |

  Every commit tested that contains 4ca638dfd took 30 to 33 s. Every commit without it took 1.8 to 2.0 s.
- **`catalogue_identity`'s walk test grows no tree.** It only formats `blend::families` at five points per pair, and it went from 26 s to 209 s. Its digest is unchanged and the test passes.
- **The sweep's time split, blend against build.** The sweep's trees were built one at a time, the first 8 of its 21 pairs, at seed 7 under the 8,000-node cap at age 12 and below:

  | Commit | Blend (s) | Build (s) | Wood vertices, sum |
  |---|---:|---:|---:|
  | Base | 121.8 | 4.6 | 16.1 M |
  | Head | 1,238.7 | 53.9 | 20.9 M |

  The blend is 96% of the added time. The trees are 1.30 times the wood but take 11.6 times as long to build, so the build's slowdown is per tree, not by size.

  The worst single pair is the ordinary tree to the spruce: blend 12.9 s on the base and 401.4 s on the head. The ordinary tree to Telperion and to Laurelin go from 40 s to 213–267 s.
- **What is left between 4ca638dfd and the head.** This adds 3 to 96 s per test, for example the sweep from 227 to 408 s. These commits include fn-354 task 7 (the beech's 12-year short shoot, the ordinary tree's leaf), fn-357 and fn-358. They were not split further.

**Not changed.** No test was edited and no budget was raised. The choice between smaller trees in the tests, a cheaper blend or grower, and a budget for the owner is the host's.

## 4. The kill at the 8 GB scope: waits for the host

The four tests that ran when the scope was killed were each run alone, and their peak memory was taken by the gate's own runner (`scripts/gate_test.py`):

| Test | Base peak (MiB) | Head peak (MiB) | Head wall (s) | Gate group |
|---|---:|---:|---:|---|
| `bark_detail relief_moves_the_light_while_every_wood_mesh_byte_holds` | 957 | 2,957 | 14.1 (base 7.0) | device |
| `paused::tests::a_retired_root_let_go_is_read_as_it_was` | not on base | 2,025 | 27.7 | none |
| `paused::tests::a_tips_path_survives_letting_go_and_a_pause` | not on base | 1,442 | 19.5 | none |
| `suite::packed_leaf::every_station_stands_inside_its_box_at_a_quarter_millimetre_step` | 4,395 | 4,536 | 76.2 (base 84.7) | one-at-a-time; listed at 4,800 MiB |

The four together come to 10.9 GiB in an 8 GB scope. Each test is under its own budget. packed_leaf is unchanged.

**Causes:**

- **The two `paused` tests are new on this branch**, from fn-355 and fn-357. They are at 2.0 and 1.4 GiB. The first is over the 1.5 GiB that `.config/nextest.toml` sends to the one-at-a-time turn, and it is not listed there. Neither is in any group, so both ran beside packed_leaf.
- **bark_detail's fixture tree is larger.** That fixture is the ordinary preset on the beech's engine species at 40, with a scaffold cap of 400. It was built once per commit, alone:

  | Commit | Wood vertices | Leaf instances | Peak (MiB) |
  |---|---:|---:|---:|
  | Base | 5.50 M | 69,745 | 360 |
  | b2b45f3c6 | 11.44 M | 150,683 | 721 |
  | 4ca638dfd | 11.20 M | 148,018 | 716 |
  | f8fe68633, after fn-354 task 7 | 19.16 M | 185,308 | 1,126 |
  | Head | 19.16 M | 185,308 | 1,142 |

  The tree grew in fn-354: first by the commits before b2b45f3c6, among them R7's lateral kind by light, then by task 7's 12-year short shoot. fn-355 did not change it. The test builds this tree twice and submits it to the device, which comes to 2.96 GiB.

**Not changed.** Whether a new test or a test that grew joins the one-at-a-time group, and whether bark_detail's fixture should be a smaller tree, are for the host.

## 5. The gate run again, at 86d273e59

The run was `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, with receipts, which reran every suite. Another program's heavy job ran beside it, and the load average was 15 to 60.

**Summary.** `Summary [2836.013s] 1394 tests run: 1369 passed, 7 failed, 18 timed out, 87 skipped`. The test run took 2,840 s against its 1,560 s budget. The suites that failed were the test run's budget, rust-jev, rust-wasm, rust-render, rust-core and node.

**Not passing, and what was checked:**

- **Cut off at its time budget**, all at about 120 s unless shown:
  - the seven of item 3;
  - `relay_tests::a_stem_never_thickens_on_its_way_up`, 33 s in the first gate;
  - `repeat_tests::the_largest_tree_and_the_palm...`, cut at 181 s; 144 s in the first gate;
  - `suite::mesh::every_preset_builds_a_mesh...`;
  - `grow::cycle::tests::woken_buds_that_stop_at_once_retire...`;
  - `boundary a_stage_outside_the_executor_interface_does_not_compile`, a trybuild compile;
  - `suite::packed_leaf...`, cut at 139 s; 76 s alone;
  - `ground_reach nothing_drawn_passes_below_the_ground`, cut at 124 s;
  - `light_rows each_populated_response...`;
  - `walks a_walk_between_species_changes_the_tree_by_degree`;
  - `walk_leaves spruce_to_birch...`, cut at 186 s;
  - `form the_beech_grows_at_every_age_of_its_sheet`;
  - `telperion_wasm flowers_cross_the_binding...`, cut at 239 s.

  None of the eleven beyond item 3's seven was timed alone, so load and growth are not told apart for them.
- **Over the memory budget:**
  - `smooth_bark the_smooth_barks_hold_the_resolution_and_redraw_contracts` peaked at 6,001 MiB against 4,096. Its assertions pass. Alone it passes in 72 s on the branch, against 23 s on the base.
  - `telperion_wasm every_shipped_preset_builds_through_the_main_entry` peaked at 4,250 MiB against 4,096; its recorded figure is 3,977.
- **Assertions that fail on the branch, alone, and pass on the base, alone.** These five never ran in the first gate. Their causes were not diagnosed:
  - `bark_distance resolved_scales_survive_until_the_two_pixel_boundary`: "distance series aliases on the OregonWhiteOak 2x: mean 5.5257, p95 13.50".
  - `bark_occlusion the_darkened_side_of_a_furrow_follows_the_sun`: "the term did nothing at sun 45: 0, 0".
  - `bark_structure_colour each_structure_colour_row_changes_the_picture_and_moves_no_wood`: "the shipped oak barely moved: 0.5518/255".
  - `curve the_device_surfaces_the_wood_the_cpu_reference_does`: "ordinary close: over budget", 3 against 0.
  - `curve the_second_slices_hold_what_the_first_could_not`: "no view reached the second slices at scale 1".
- **node.** `catalogue/date-palm/README.md` differed from its regeneration. The page prints the skeleton digest repinned in item 1. **Fixed** in 801c4e74e by `node scripts/catalogue-pages.mjs`; `catalogue-check` now passes.

# The render, memory and timing items, 12 to 13 Nov 2026

Worked in `.worktrees/gate2` (branch `gate/render-and-memory`, head 34b28aa43 at the start) against the base 5ebab2de0 in `.worktrees/gate-base`, each with its own `target/`. Every test ran alone through `telperion-heavy 8 --threads 4` (`--gpu 2` for device tests), under nextest's default profile with the gate's runner (`scripts/gate_test.py`) taking the peak. Times are nextest's test durations, without the build. Tree sizes come from a scratch probe, `mesh::build` alone in its own process with peak `VmHWM`. Scratch probes were not committed.

## 6. Five assertions that failed here and passed on the base

| Test | What it promises | Base | Branch | Cause | Outcome |
|---|---|---|---|---|---|
| `bark_occlusion the_darkened_side_of_a_furrow_follows_the_sun` | The occlusion term darkens the side away from the sun, and that side swaps when the sun crosses | took 0.0917 / 0 at sun 45 | took 0 / 0 | Framing. The oak at 80, seed 1, is thinner: the wood across the frame's middle row went from 222 to 127 px, and the strips 20 to 90 px either side of centre held little trunk | **Fixed** in 086dfedc5. The camera comes in from 2.47 m to 1.42 m along the same line. Takes 0.0703 at sun 45 and 0.1028 at sun 225, and the swing changes sides |
| `bark_structure_colour each_structure_colour_row_changes_the_picture_and_moves_no_wood` | Each structure row changes the frame, and the shipped row moves it by more than 1/255 | shipped 1.8014/255 | 0.5518/255 | Framing, on the same tree and pose as `bark_depth` (oak seed 7). The middle row went from 236 to 123 px of wood | **Fixed** in 086dfedc5, with `bark_depth`'s 1.09 m pose (200 px). Shipped 1.6223/255; the rows 0.938, 0.772 and 1.374 (base 1.118, 0.751 and 1.474) |
| `curve the_second_slices_hold_what_the_first_could_not` | Some view from 40 to 96 px wide is drawn at scale 1 past half its budget, so the second slices are written | found at 84 px | none | The fixture tree grew. The ordinary tree on the beech at 40 went from 5.50 M to 19.16 M wood vertices. Its close view asks about 2.6 times the wood a pixel, so no small view fits the finest scale: scale 8 up to 92 px, and 2 at 96 px | **Fixed** in 8eeac71af. This test now uses that tree at 30, its own year 30, with 4.91 M wood vertices. The 76 px view draws at scale 1, 33,363 vertices against a budget of 36,525, and matches the reference |
| `curve the_device_surfaces_the_wood_the_cpu_reference_does` | At 480x360, the hero view and then the close view are drawn within budget and match the CPU reference | close: scale 8, overrun 0 | close: overrun 3 | **Product.** The device sizes the close frame's budget from the hero frame's demand. Tube indices: the budget was 100,350 on the base and is 27,990 here, against a close demand of 254,556. No scale fits, and a cluster is left out. The shipped `european-beech` at its default age gives the same: overrun 3 here, 0 on the base | **Returned** |
| `bark_distance resolved_scales_survive_until_the_two_pixel_boundary` | The shipped oak's trunk at 2x and 4x distance agrees across resolution within 5.2 and 5.8 mean, and 14 and 15.25 p95 | 2x: 4.42 / 12.75; 4x: 5.52 / 14.50 | 2x: 5.53 / 13.50; 4x: 6.03 / 16.25 | **Product, as measured.** The trunk is thinner at the same distance. The mask fell from 9,972 to 4,123 px at 2x. The central half of each mask row still reads 4.96 against 3.72 on the base at 2x, so the rise is not only edge pixels. Moving the camera in would change the footprint series the test exists for | **Returned** |

On the curve overrun: the base shows the same mechanism on younger ordinary trees. At 25, 20 and 15 the close frame after the hero frame overruns (3) on both commits. The branch's tree at 40 and the shipped beech at 40 now land in it.

## 7. Memory

| Test | Base peak (MiB) | Branch peak (MiB) | What holds it | Outcome |
|---|---:|---:|---|---|
| `paused a_retired_root_let_go_is_read_as_it_was` | not on base | 2,025 → **954** | The spruce's two growers, 2,008 alone at 40 | **Fixed** in d6bbf6f4a: grown to 30 in steps 10, 20, 30. It still lets go of 13,062, 110,705 and 3,657 retired roots (oak, spruce, beech; the test asks for over 100), and every closed subtree's living is equal. Not listed |
| `paused a_tips_path_survives_letting_go_and_a_pause` | not on base | 1,442 → **737** | The spruce, 1,458 alone at 40 | **Fixed** in d6bbf6f4a, the same way. It compares 19,203, 67,273 and 16,435 living tips, all equal. Not listed |
| `bark_detail relief_moves_the_light_while_every_wood_mesh_byte_holds` | 976 | 2,929 | Its fixture, the ordinary tree on the beech at 40, built twice: 19.16 M wood vertices (base 5.50 M), 1,143 MiB a build | **Listed** one-at-a-time in `.config/nextest.toml` (d6bbf6f4a). A younger tree does not prove it: the relief changes 0 channels at 20, 30 and 35, and 311 at 40 (the bar is 100). The test needs wood in view thicker than the 0.16 m ridge scale; this was read from the shader's guard and not measured |
| `smooth_bark the_smooth_barks_hold_the_resolution_and_redraw_contracts` | 2,254 | 5,883 (fails its 4,096 budget; assertions pass) | The build of the shipped **european-beech at 80, seed 1**: 101.7 M wood vertices and 5,826 MiB, against 35.2 M and 2,094 MiB on the base, in 54 s against 8 s. The birch at 80 is unchanged: 1,684 against 1,674 | **Returned**: a shipped preset at 80 needs 2.8 times the memory |
| `telperion_wasm every_shipped_preset_builds_through_the_main_entry` | 3,981 | 4,198 alone (4,250 in the gate), against 4,096 | The largest preset, **telperion** at its default age: 73.3 M wood vertices and 4,432 MiB a build, against 72.6 M and 4,286 on the base. laurelin is at 4,204 against 4,114. The beech grew most, 360 to 1,138, but is not the largest | **Returned**: +146 MiB on the largest shipped preset |

Builds of every shipped preset at its default age (seed 42), base → branch, in wood vertices and MiB:

- oak: 6.30 M / 445 → 7.14 M / 460
- spruce: 24.5 M / 1,590 → 26.9 M / 1,737
- beech: 5.50 M / 360 → 19.16 M / 1,138
- birch: 6.71 M / 449 → 7.05 M / 456
- palm: equal
- telperion: 72.6 M / 4,286 → 73.3 M / 4,432
- laurelin: 68.0 M / 4,114 → 68.9 M / 4,204

Three more tests are now over their memory budgets alone. Their assertions pass. All three were found while timing item 8, and each is returned:

| Test | Base peak (MiB) | Branch peak (MiB) | Budget | What holds it |
|---|---:|---:|---:|---|
| `form the_beech_grows_at_every_age_of_its_sheet` | 1,809 | **8,631** | 4,096 | The beech grown to 10, 20, 40 and 80 at seeds 1 and 7; its largest are the trees at 80. The beech at 80, seed 1, builds 101.7 M wood vertices against 35.2 M. Under the 8 GB scope the test was killed twice (SIGTERM at 82 to 90 s), so it now needs more than the gate's whole scope |
| `ground_reach nothing_drawn_passes_below_the_ground` | 4,660 | **5,984** | 5,184 | The largest row, **telperion at its own age 82, seed 1**: 56.7 M wood vertices and 3,369 MiB a build, against 42.9 M and 2,519 MiB. laurelin at 80, seed 1: 53.6 M and 3,207 MiB, against 40.2 M and 2,424 MiB |
| `telperion_wasm flowers_cross_the_binding_where_the_pipeline_grows_them` | 3,903 | **4,173** and 4,178 (4,007 once) | 4,096 | Telperion and laurelin at their default ages, as in the main-entry test above |

No budget in `.config/gate-budgets.toml` was raised.

## 8. The eleven tests cut off in the loaded gate, timed alone

Each test ran alone, branch then base, back to back where the queue allowed. The times are nextest's test durations. A run with a watcher `PAUSE` inside it was discarded and run again. The runs on 12 Nov came before the watcher existed and are taken at the load shown. Peaks come from the gate's runner.

| Test | Base s / MiB (load) | Branch s / MiB (load) | Reading |
|---|---|---|---|
| `relay_tests a_stem_never_thickens_on_its_way_up` | 35.5 / 1,511 (42) | 58.6 / 3,269 (15) | **A larger tree**: 2.2 times the memory, 1.65 times the time at lower load |
| `repeat_tests the_largest_tree_and_the_palm...` | 121.9 / 2,686 (46) | 185.9 / 2,144 (72) | **Load**, as far as can be told: less memory on the branch, and the run sat at load 72 against 46 |
| `suite::mesh every_preset_builds_a_mesh...` | 44.5 / 4,380 (48) | 60.8 / 4,386 (40) | **Slower code or load**: the same memory, 1.37 times the time. The beech's build alone is 0.97 s on the base and 2.82 s on the branch at seed 42 |
| `grow::cycle::tests::woken_buds_that_stop_at_once_retire...` | 0.71 / 24 (45) | 225.8 / 246 (29) | **Slower code, about 300 times.** The same oak, 2,207 axes against 2,208. One seed's `grow_census` takes 0.11 s on the base and 35.4 s on the branch, and the reference grower 0.09 s and 34.2 s. No blend is involved. The commit was not bisected |
| `suite::packed_leaf every_station...` | 83.2 / 4,352 (55) | 115.3 / 4,416 (45) | **Slower code or load**: the same memory, 1.39 times the time |
| `ground_reach nothing_drawn_passes_below_the_ground` | 88.4 / 4,660 (56) | 114.2 / 5,984 (57) | **A larger tree**: telperion and laurelin at seed 1, item 7. Over its memory budget |
| `light_rows each_populated_response...` | 45.5 / 1,622 (8) | 68.1 / 1,233 (36) | **Load** most likely: less memory, and the branch's run sat at load 36 against 8 |
| `form the_beech_grows_at_every_age_of_its_sheet` | 18.5 / 1,809 (10) | 71.2 / 8,631 (7) | **A larger tree**: the beech at 80, 4.8 times the memory. Over its memory budget and killed under the 8 GB scope |
| `telperion_wasm flowers_cross_the_binding...` | 91.8 / 3,903 (9) | 93.4 / 4,173 (4) | **Not slower alone.** Its 239 s cut came from the loaded gate. Over its 4,096 MiB budget, item 7 |
| `boundary a_stage_outside_the_executor_interface_does_not_compile` | not cleanly timed (8.0 s with a pause inside; 107.9 s on a cold trybuild cache) | 46.9 / 925 (11) | **A compile, not a tree**: its time follows trybuild's build cache |
| `walks a_walk_between_species_changes_the_tree_by_degree` | — | — | **Blends species**: the cause another agent is fixing (item 3). Not timed |
| `walk_leaves spruce_to_birch_changes_its_leaves_by_degree` | — | — | **Blends species**: as above. Not timed |

## What is returned to the host

1. **The device budget after a hero frame.** `curve the_device_surfaces...`: at 480x360 a close frame after the hero frame overruns (3) for the ordinary tree on the beech at 40, and for the shipped beech at its default age. The tube-index budget is 27,990 against a demand of 254,556; the base had a budget of 100,350, with overrun 0. The same mechanism overruns on the base for younger trees (15 to 25).
2. **The shipped oak's bark aliasing at distance.** `bark_distance` reads mean 5.53 at 2x against a bound of 5.2 (base 4.42). On the central half of the trunk it reads 4.96 against 3.72 on the base.
3. **Shipped presets that need more memory**:
   - the beech at 80, seed 1: 101.7 M wood vertices and 5,826 MiB, against 35.2 M and 2,094 (`smooth_bark`, `form`);
   - telperion at 82, seed 1: 3,369 against 2,519 MiB, and laurelin at 80, seed 1: 3,207 against 2,424 MiB (`ground_reach`);
   - telperion at its default age: 4,432 against 4,286 MiB (`every_shipped_preset...` at 4,198, and `flowers_cross...` at 4,173, both against 4,096).
4. **`bark_detail` needs the tree at 40.** It is listed one-at-a-time at 2,929 MiB; younger trees change 0 channels.
5. **The grower is about 300 times slower** on `woken_buds` (0.7 s to 226 s), on an unchanged tree, with no blend.
