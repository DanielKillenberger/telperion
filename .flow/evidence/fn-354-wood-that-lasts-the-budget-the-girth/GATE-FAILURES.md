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
