# fn-274 measurements

All measured on the desk, 2026-10-14, `ci` profile, raw logs in `raw/` (gitignored).

## Per test, alone (R2)

Each of telperion-core's 451 unit tests run alone (`--exact`, one test thread, rayon at 4), its peak RSS from `wait4` (`raw/peaks-base.tsv`). Tests at 0.4 GB or more:

| Test | Peak (MiB) | Wall (s) |
|---|---:|---:|
| suite::packed_leaf::the_position_step_is_under_a_quarter_millimetre_for_every_species | 6,186 | 39 |
| suite::packed_leaf::no_station_of_any_shipped_species_falls_outside_its_box | 6,185 | 45 |
| suite::mesh::every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers | 6,138 | 40 |
| pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline | 5,753 | 42 |
| pipeline::space::repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores, before | 5,036 | 103 |
| the same, after (one tree at a time) | 3,752 | 145 |
| pipeline::space::ceiling_tests::every_shipped_preset_keeps_under_the_wasm_ceiling | 2,636 | 29 |
| pipeline::space::relay_tests::a_stem_never_thickens_on_its_way_up | 2,049 | 15 |
| pipeline::surface::attachment::tests::contacts_read_from_the_wood_are_the_swept_ones | 919 | 6 |
| pipeline::foliage::packed::tests::every_preset_placement_round_trips_inside_r2 | 846 | 8 |
| pipeline::surface::curve::tests::every_presets_rings_are_drawn_from_its_curve_to_the_bit | 823 | 4 |
| pipeline::surface::curve::tests::packed_points_stand_within_their_stated_error | 428 | 4 |

Every other test peaks under 330 MiB. Apart from the repeat test, each of these already held one preset's build at a time; their peak is one Telperion build, so it is unchanged. All eleven now take one process-wide turn (`crates/telperion-core/src/one_tree.rs`).

## telperion-core's unit tests at 4 test threads (R3)

Anonymous memory of the scope, sampled every 0.5 s (page cache excluded):

| Build | Anon peak (MiB) | Result |
|---|---:|---|
| before (b03523aba) | over 11,651 | killed at the 13 GiB cap after 81 s |
| repeat test only, no turn | not sampled | killed at the 9 GiB cap |
| the seven tests over 2 GB in the turn (5bb776dd5) | 8,412 | passed, 360 s |
| all eleven in the turn (6f9a8d5) | 7,541 | passed, 380 s |

## The workspace gate at 4 test threads (R3)

`cargo test --profile ci --workspace --no-fail-fast`, `telperion-heavy <cap> --threads 4`:

| Run | Cap | Anon peak (MiB) | Result | Wall (s) |
|---|---:|---:|---|---:|
| before, fn-251 | 13 GiB | - | OOM-killed in telperion-core's unit tests | 190 |
| 5bb776dd5 | 9 GiB | not sampled | passed, 1,349 + 87 ignored | 1,696 |
| 6f9a8d5 | 8 GiB | 7,847 at the kill | OOM-killed in the suite's whole-preset tests | 320 |
| final (1a8c26f29) | 9 GiB | 8,860 | passed, 1,348 + 87 ignored | 1,551 |

The gate's peak sits in telperion-core's unit tests, while one 6 GB whole-preset test runs beside the light ones. Within the gate it reached 8,860 MiB, against 7,541 MiB for the same binary alone, so R3's under-8-GB target holds for the binary alone, not reliably for the gate.

The test count is unchanged: 451 in telperion-core before and after (415 run and 36 ignored in the final gate), 1,348 run and 87 ignored across the workspace at the final commit.
