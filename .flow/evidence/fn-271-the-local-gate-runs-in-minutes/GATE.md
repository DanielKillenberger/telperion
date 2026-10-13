# The local gate, measured and shortened (fn-271 task 1)

All runs are on the owner's desk (32 threads, 31 GB, RTX 3080). Each gate ran once, under the GPU lock, as `cargo test --profile ci --workspace --no-fail-fast`. Per-test times come from libtest's `--report-time`, switched on through a cargo runner (`CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER`) so that cargo's fingerprints, and so the build, stayed unchanged. The desk was shared with another project's test runs throughout, at load 7 to 47, so every wall time here carries that noise. Raw logs are in `raw/` (ignored).

## Result

| | Before (`f03c124a7`) | After (`4b39c53d0`) |
|---|---|---|
| Gate wall time | 1,525 s (25:25) | 1,128 s (18:48) |
| Tests run / ignored | 1,357 / 89 | 1,357 / 89 |
| Exit | 101, see note | 0 |

No test count moved and no test went behind `--ignored`. No assertion, age, seed count, step count or preset changed. Every gain comes from growing at once trees that a test grew one after another.

The before run's one failure, `telperion-render --test boundary`, came from the timing runner. trybuild compiles under the test's environment, and `RUSTC_BOOTSTRAP=1` makes rustc add a `help:` line to the diagnostic. The same binary run plain at the same commit passed (`raw/before-render-boundary-plain.log`). The after run's runner exempts the boundary binaries.

## Per binary (gate wall, s; every binary over 30 s)

| Binary | Before | After | Tests | What changed |
|---|---|---|---|---|
| space `tests/walks.rs` | 481.6 | 310.2 | 12 (+1 ignored) | walk pieces, refinements and crossings spread over the cores |
| core lib | 217.4 | 190.4 | 417 (+36) | the sweep's steps, the wire walk's points |
| render lib (GPU) | 101.8 | 98.4 | 177 (+31) | none |
| field lib | 100.9 | 54.0 | 7 | each preset on its own thread |
| core `tests/catalogue_identity.rs` | 62.8 | 10.1 | 4 | each pair's walk on its own thread |
| wasm lib | 49.7 | 42.7 | 5 | none |
| render `tests/conformance.rs` | 38.2 | 37.2 | 5 (+1) | none |
| core `tests/light_rows.rs` | 35.2 | 37.0 | 3 | none |
| space lib | 33.1 | 33.9 | 96 (+5) | none |

The rest of the binaries take under 30 s each and together about 330 s in both runs.

## R1: the slowest twenty tests, before

Times are gate wall times under the binary's own concurrency, where 32 test threads share the desk.

| s | Test | What it grows |
|---|---|---|
| 481.6 | space `every_setting_changes_the_tree_by_degree` | the three-PA walk tree at age 12: about 110 settings × 3 seeds × 201 steps, then each setting's 3 steepest steps refined 6 levels × 9 trees, and a noise floor of up to 34 trees where a refinement does not shrink |
| 320.6 | space `branches_are_made_and_unmade_at_vanishing_size` | the same walks (shared), then each walk's most-made step bisected 48 rounds, one tree a round |
| 217.3 | core `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` | each of the 8 shipped presets at its own age and seed, three times: twice on 4 cores, once on 1 |
| 187.7 | space `light_changes_the_tree_by_degree` | 16 light settings on the leafy walk tree, age 12, 3 seeds × 201 steps |
| 186.6 | space `a_walk_between_species_changes_the_tree_by_degree` | beech to spruce, spruce to oak, oak to palm, the blend at age 12, 3 seeds × 201 steps |
| 175.9 | core `suite::sweep::every_pair_of_presets_grows_a_tree_at_every_step` | every preset pair's blend at 10 steps, a mesh at age ≤ 12 and an 8,000-node cap |
| 157.6 | space `a_sample_of_settings_in_leaf_changes_the_tree_by_degree` | 3 settings in leaf, age 12, 3 seeds × 201 steps |
| 149.0 | space `the_release_law_changes_the_tree_by_degree` | 6 sleeping-bud settings, age 12, 3 seeds × 201 steps |
| 148.0 | space `the_shedding_delay_changes_the_tree_by_degree_in_leaf` | 1 setting in leaf, age 12, 3 seeds × 201 steps |
| 144.6 | space `a_midpoint_between_two_undying_leaders_keeps_its_leader` | one beech-spruce blend at age 60, seed 1 |
| 140.5 | core `pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline` | each shipped preset's skeleton at its own age, then 4 artifact requests |
| 139.1 | core `suite::packed_leaf::no_station_of_any_shipped_species_falls_outside_its_box` | each shipped preset's mesh |
| 132.3 | core `suite::mesh::every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers` | 7 presets' meshes |
| 128.2 | core `suite::packed_leaf::the_position_step_is_under_a_quarter_millimetre_for_every_species` | each shipped preset's mesh, again |
| 120.5 | core `suite::sweep::every_wire_parameter_a_preset_moves_is_walked` | 3 blends per preset pair, each with a size solve |
| 111.3 | space `a_stop_near_its_bound_fades_nothing_its_relay_carries` | the walk tree at age 12, 4 seeds × 201 abortion values |
| 108.4 | core `ceiling_tests::every_shipped_preset_keeps_under_the_wasm_ceiling` | each shipped preset grown as the browser grows it |
| 100.9 | field `every_shipped_preset_answers_as_the_main_pipeline_does` | each preset at seeds 1 and 4242, through the slim binding and the core pipeline's field build |
| 96.6 | render `resident::repeat_tests::every_device_preset_builds_the_same_leaves_each_time` | GPU: each device preset's leaves, twice |
| 88.9 | render `vm_tests::the_sizing_programs_compute_what_the_host_does` | GPU: sizing programs against the host |

Then `family::tests::every_row_past_its_bound_is_refused_on_every_family` (70.6 s), `relay_tests::a_stem_never_thickens_on_its_way_up` (64.2 s: the beech and the ordinary tree at 80 and the ordinary tree at 40, seed 42, all at full size) and `catalogue_identity::every_walk_is_the_walk_it_was` (62.8 s: 11 families, 55 pairs × 5 points, each interior point a size solve).

A binary's tests run at once on 32 threads, so these times include waiting for the desk. Alone, the stop walk takes 0.3 s and the filtered insertion walk 3 s.

## Where the walks' time goes

One setting is most of `walks.rs`: `states[0].zones[1].lateral[0]`, a trunk lateral of the trunk's own PA, walked up to p = 0.295. Its tree grows from 700 branches at the low end to 365,000 to 617,000 at the top (seeds 1 to 3) and takes 1.2 to 2.4 s single-threaded there (`raw/` probe, not kept). Alone, its walk takes 104 to 125 s. Refining its three steepest steps near the top end is about 75 s of that: 6 levels of 9 trees of that size, one level after another. Spreading each level's 9 trees as well took the binary's peak memory from 6.9 GB to 14 to 17 GB, because 27 trees of that size were then held at once, and it did not shorten the wall time. That change was reverted. Even so, the kept change raised the walks binary's peak from 2.7 GB (sampled during the before gate) to 6.9 GB (alone, `raw/walks-pieces.rc`), because the costly top pieces of that walk now grow at once. The walks binary is now bound by CPU (5,400 CPU-seconds over 322 s alone). Shortening it further means growing fewer or smaller trees, which is a coverage decision (below).

## Red and green, for each changed test

Each red run applied a temporary local patch to the generator, ran the test and reverted the patch. No patch was committed. Logs are `raw/rg-*.log`.

| Test | Broken generator (temporary patch) | Red | Green on the engine |
|---|---|---|---|
| walks `every_setting_changes_the_tree_by_degree` (and the six `by_degree` walks through the same `walks`/`judge`) | insertion made a switch at 0.55 (`lineage.rs`) | `states[1].insertion: 35.2 per unit` and a jump at 0.5500 on every seed and PA | `WALK=insertion`, ok |
| walks `every_setting_changes_the_tree_by_degree` | presence window × 1e-6, so a branch is made at full size (`contract/presence.rs`) | fails (`WALK=zones[0].lateral`) | `WALK=zones[0].lateral`, ok |
| walks `branches_are_made_and_unmade_at_vanishing_size` | the same window patch | `states[2].zones[0].lateral[2] seed 1 at -0.3025: 0.00145 of the tree` | ok |
| walks `a_stop_near_its_bound_fades_nothing_its_relay_carries` | abortion less 0.1 above 0.25 (`contract/decide.rs`) | `seed 1: the stem dips to 25.6126 between 25.6209s` | ok |
| core `every_pair_of_presets_grows_a_tree_at_every_step` | the mesh's first wood position NaN (`mesh.rs`) | `ordinary->oregon-white-oak at 0: a position is not finite` | ok |
| core `every_wire_parameter_a_preset_moves_is_walked` | a blend's interior points are its first end (`blend.rs`) | `the walk never moves {"/age (european-beech->laurelin)", ...}` | ok |
| core `catalogue_identity::every_walk_is_the_walk_it_was` | the same blend patch | digest 950059844104049905 against the pinned one | ok, digest unchanged |
| field `every_shipped_preset_answers_as_the_main_pipeline_does` | the slim binding grows seed 4242 as 4243 (`telperion-field/src/lib.rs`) | `date-palm 4242` answers differ | ok |

Panics from a spread worker reach the test unchanged (`resume_unwind`), so each failure message is the one it was.

## R2: the size probes (NEEDS_HUMAN)

R2 asks for each walk at the smallest age, seed count and step count that still catches its defect. Run on the two defects R2 names, the probes say that the step count is part of the premise, and the seed count cannot drop without moving an assertion. Probe: `WALK=insertion` and `WALK=zones[0].lateral`, on the engine, with the switch patch and with the full-size patch (`raw/size-*.log`).

| STEPS, seeds | Green | Switch caught | Full-size branch caught by `every_setting` | Full-size branch caught by `branches_are_made` |
|---|---|---|---|---|
| 200, 1 2 3 (today) | yes | yes | yes | yes |
| 200, 1 | yes | yes | yes | yes |
| 100, 1 2 3 | yes | yes | yes | **no** |
| 50, 1 2 3 | yes | yes | yes | **no** |
| 50, 1 | yes | yes | yes | **no** |

- **Steps stay at 200.** At 100 or 50 steps, `branches_are_made` passes on a generator that makes branches at full size, because the step that made the most wood no longer brackets one crossing. Named: its premise needs 200 steps.
- **Seeds.** One seed still catches both defects on this sample, but `branches_are_made` also asserts `crossed > 60` over every walk and seed. A third of the crossings would fail that assertion on the engine, so dropping seeds means lowering the count it asserts, which is weakening an assertion. That is the host's call.
- **Age stays at 12.** Not probed. Several walk ranges are written for age 12 (the trunk's lifespan walked 8 to 16, the sleeping buds' delays up to 8 cycles, the trunk's own lifespan of 2 × AGE), so at a younger age those walks fall dormant and pass without testing anything.
- **The midpoint test** (`a_midpoint_between_two_undying_leaders_keeps_its_leader`, 145 s, one tree) states its premise at age 60 ("keeps its leader to age 60") and stays at full size.

Options for the host, measured where they could be:

1. **Narrow the one explosive walk.** `states[0].zones[1].lateral[0]` up to p = 0.295 grows trees of up to 617,000 branches. Measured on seed 1, the tree at p ≈ 0.18 (step 170) has 46,000 branches and takes 0.14 s, against 1.2 s at the top. A lower ceiling would drop the costly top of the walk. Whether that top is part of what the walk should cover is a coverage decision, so the change was not made or timed.
2. **Two seeds**, with `crossed > 60` lowered to match (two seeds would give about two thirds of today's crossings; inferred, not counted). Inferred to cut `walks.rs` by about a third. It lowers an assertion's count.
3. **`every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` behind `--ignored`**, like `every_setting_in_leaf_changes_the_tree_by_degree` already is, with a named command in `docs/` and CI still running them. This takes about 310 s off the local gate, and the walks would then be proven only in CI.

## R3: the other slow binaries

- **core lib (217 to 190 s)** is bound by `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` (190 s after). Its one-core builds alone take most of that. Running a preset's three builds at once was measured (`raw/core-lib-after1.log`): 233 s against 253 s at peak 18.4 GB, inside the desk's noise, so it was reverted. Its premise, the same tree on one core as on four, holds at full size, because the engine's parallel stages hand out work only above a size (`cores::map` serves 64 items or fewer on the calling thread). A smaller age might never exercise the parallel path. **Named: full size.** The option is to run every preset's one-core build at once, at the cost of up to 8 full trees in memory. The core lib already peaks at 17 GB before this task.
- **Whole-preset checks that build the same trees, named for the host and not removed.** Every shipped preset at its own age and seed is built separately by `suite::mesh` (`mesh::build`), by both `suite::packed_leaf` tests (`executor::grow` then `expansion().mesh()`), by `pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline` (skeleton, then four requests), by `ceiling_tests` (`telperion_space::grow` as the browser asks) and three times by `repeat_tests`. The field test builds each preset's field twice more. The two `packed_leaf` tests build identical meshes. A per-process cache shared by the mesh-reading tests would remove four of these builds per preset. It would also stop each test proving its own entry, which `docs/principles.md` asks the artifact test to do, so this is for the host.
- **render lib (98 s)** is bound by GPU tests that take the device one at a time in a process (`docs/gpu-tests.md`), at about 85 s each under the binary's concurrency. Not changed, because it is GPU work under the lock.
- **field lib (101 to 54 s)**: each preset on its own thread (the slim binding's engine is thread-local), peak 5.4 GB. Splitting by seed as well would halve it again, with roughly twice the trees in memory. Not done.
- **catalogue_identity (63 to 10 s)**: each pair's walk on its own thread. The pinned digest is unchanged.

## R4: nextest

Not measured to completion, and the desk's memory is why. Nextest 0.9.148 (a binary in the scratchpad, nothing installed) ran the workspace with `--cargo-profile ci --no-fail-fast` under the GPU lock. A guard stopped the run by its process group whenever available memory fell under 3 GB. Logs are `raw/nextest-after*.log`.

| Width | Config | Stopped at | Tests done | Lowest available | Running when stopped |
|---|---|---|---|---|---|
| 32 (default) | repository's + render one at a time | 49 s | 986 / 1,357 | 2 GB | many whole-preset core tests |
| 8 | same | 85 s | 431 | 2 GB | the sweep, catalogue, artifact and repeat tests |
| 4 | same | 103 s | 316 | 2 GB | packed_leaf, artifacts, repeat, mesh |
| 16 | plus groups: whole-preset tests 2 at a time, walks 1 at a time (`nextest-local.toml`) | 379 s | 1,248 | 0 GB | walks `every_setting`, repeat, mesh |

What this shows (measured):

- **Four whole-preset core tests at once use about 13 GB.** Under `cargo test` the core lib binary runs them together and peaks at 17 to 18 GB (`raw/core-lib-*.rc`) on a desk with 13 to 15 GB available, so today's gate already reaches into swap there.
- **Under nextest the walks cost twice.** `every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` share their walks through a `OnceLock`, and nextest runs each test in its own process, so each makes all the walks itself.
- **At width 16 with groups, 1,248 tests finished in 377 s.** The 109 left were mostly the walks (about 300 s each, one at a time) and the render crate's GPU tests (one at a time). Inferred: that configuration would take about 15 minutes, close to `cargo test`'s 18:48, and it ran the desk out of memory.

For the owner: nextest is not a drop-in local gate on this desk. It would first need the whole-preset tests held to one or two at a time and the two big walks merged or kept in one process. Until then, `cargo test --profile ci --workspace --no-fail-fast` remains the gate AGENTS.md names, now at 18:48 instead of 25:25.
