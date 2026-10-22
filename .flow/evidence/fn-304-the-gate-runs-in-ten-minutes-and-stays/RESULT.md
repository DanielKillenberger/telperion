# fn-304 result

Each red run applied one temporary patch to the generator with `git apply`, ran the named tests under nextest inside `telperion-heavy 8 --threads 4`, and reversed the patch; none was committed. The patches and the runner are kept in ignored `raw/` (`raw/patches/specs.py` writes each patch; `raw/proofs.sh` applies, runs and reverses; `raw/proofs/summary.tsv` lists every run).

## Task .1: the core lib (rulings 7, 8, 9)

| Test | Change | Own time before → after (nextest) |
|---|---|--:|
| `repeat_tests::the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores` | the gate keeps Telperion and the date palm; the print precondition stays | 182.2 s (all presets, rayon 2) → 79–92 s (rayon 4) |
| `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` | the full catalogue, now `#[ignore = "fn-304: ..."]` | 180–201 s (rayon 4), outside the gate |
| `suite::packed_leaf::every_station_stands_inside_its_box_at_a_quarter_millimetre_step` | the two `packed_leaf` tests merged: one mesh a preset, both assertions | 49.8 + 49.4 s → 51–56 s |
| `family::tests::every_row_past_its_bound_is_refused_on_every_family` | the eight families at once (`thread::scope`, `resume_unwind`) | 70.1 s → 11.8–13.2 s |

Test count in the core lib: the two `packed_leaf` tests become one (merge, ruling 8); the repeat test becomes a gate test plus an ignored sweep (move to ignored, ruling 7). Run: 459 → 458; ignored: 37 → 38; total 496 unchanged (gate: 458 passed, 38 ignored in the core lib).

### Red and green

| Patch (temporary) | What it breaks | Gate subset | Full sweep |
|---|---|---|---|
| `tail-piece` (`telperion-space/src/cores.rs`, `fill`) | with more than one pool thread the last piece of a parallel fill is never written, so the tree follows the core count | red: `telperion: on one core` | red: `ordinary: on one core` |
| `sum-order` (`telperion-space/src/light.rs`) | the light's sum over directions reversed on more than one thread | green | green |
| `offset` (`telperion-space/src/cores.rs`, `map`) | a parallel piece loses its offset | red (panic in `girth.rs:263`) | red (the same panic) |
| `box-small` (`foliage/reference.rs`, `Reference::grown`) | the leaf box half its size | merged test red: `ordinary: leaf 0 stands on the wall ...` | — |
| `box-coarse` (the same) | the leaf box twenty times too coarse | merged test red: `ordinary: half a step is 0.00192 m ...` | — |
| `bloom-rails` (`family.rs`, both bloom checks) | the bloom rows lose their rails | row-bound test red on every family | — |

Green on the engine: all four tests pass (`raw/proofs/t1-green.log`).

`sum-order` is the one nondeterminism that neither the subset nor the full sweep sees: the shipped presets' light sums over too few directions for their order to move a bit. It is recorded, not acted on; `tail-piece` is the ruling's proof (deterministic, the defect "on any cores" names, caught by the "on one core" assertion on both).

Gate at the end of task .1 (`telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, 3b880374b..b1feb485a): 1,466 passed, 2 failed, 116 ignored, 2,374 s wall including 215 s of compile, peak 7,637 MiB anon. The two failures are fn-302's (`film_walk_tests::the_film_walks_build_on_the_device`, `the_rounds_frames_have_the_cpus_counts`), red on F4 before this spec.

## Task .2: the core sweeps (rulings 1, 2, 5)

| Test | Change | Own time (nextest, rayon 4) |
|---|---|--:|
| `walk_leaves` `oak_to_palm_keeps_its_leaves_by_degree`, `spruce_to_birch_...`, `birch_to_oak_...` | each walks seed 1 | 20 s, 81 s, 29 s (280 + 93 + 65 s at three seeds, rayon 2) |
| `walk_leaves` `every_walk_changes_its_leaves_by_degree_at_every_seed` | the three walks at seeds 1, 7 and 42, ignored | 359 s |
| `ground_reach` `nothing_drawn_passes_below_the_ground` | fn-301 R1's 45 touching rows and every preset at its own age, seed 1 (50 rows: three own-age rows are already touching) | 54–58 s, peak 4.68 GB (was 159 s, 7.77 GB) |
| `ground_reach` `nothing_drawn_passes_below_the_ground_on_every_row` | all 168 rows, ignored | 136 s |
| `walk_sizes` `the_teaser_frame_fits` | the teaser frame alone | 21 s (was 79.5 s with the exceptions) |
| `walk_sizes` `the_named_exceptions_grow_no_further_out` | fn-273's eight exceptions, ignored | 46 s |

Test counts: `walk_leaves` 3 run + 1 ignored → 3 + 2; `ground_reach` 1 → 1 + 1; `walk_sizes` 1 → 1 + 1 (the test is split: the teaser stays, the exceptions move to ignored, renamed `the_teaser_frame_fits` and `the_named_exceptions_grow_no_further_out`).

### ground_reach's 7.8 GB is one tree, not accumulation

Measured with a temporary probe that prints `VmRSS` and `VmHWM` after each row's tree is dropped (`raw/patches/probe-rss.patch`, `raw/proofs/ground-probe2.log`, `ground-probe3.log`):

| Row | Alone, fresh process: peak | In the sweep: peak so far, RSS after the drop |
|---|--:|--:|
| Telperion at 82, seed 1 | 4,623 MiB | 4,658 MiB, 828 MiB |
| Telperion at 82, seed 7 | 7,677 MiB | 7,666 MiB, 1,253 MiB |
| Laurelin at 80, seed 1 | 4,326 MiB | under the Telperion peak |
| Norway spruce at 40, seed 1 | 2,697 MiB | 2,689 MiB |
| Telperion at 82, seed 42 | not run alone | 7,692 MiB, 1,195 MiB |
| the other rows run alone (birch, oak, beech and ordinary tree at 40, palm at 40, Telperion at 20) | 20 to 713 MiB | |

Telperion at 82, seed 7 peaks at 7.68 GB on its own, and the sweep's high-water mark rises only at the Telperion rows (seed 42 to 7.69 GB): the sweep's peak is one tree, not trees held together. The test already drops each tree before the next is built; the RSS left after a drop (up to 1.3 GB) is memory the allocator keeps, not a tree held. The gate's rows include Telperion at its own age only at seed 1, so the gate test peaks at 4.68 GB; seed 7 stays in the ignored sweep.

### Red and green

| Patch (temporary) | What it breaks | Gate subset | Full sweep |
|---|---|---|---|
| `cluster-round` (`telperion-space/src/lineage.rs`, `cluster`) | a short shoot's leaves rounded, not drawn (fn-277's defect) | red: `spruce_to_birch` and `birch_to_oak` at seed 1 | red |
| `leaflets-round` (`foliage/organs.rs`, `placement`) | a placement's leaflets rounded | red: `oak_to_palm` at seed 1 (`leaflets per station ... against ...`) | red |
| `fronds-round` (`foliage/organs.rs`, `apex`) | the palm's fronds rounded | green | green |
| `no-rest` (`foliage/rest.rs`, `leaf`) | a leaf that would cross the ground is never rested (fn-301) | red: the touching rows named | red |
| `budget-eighth-cycle` (`telperion-space/src/grow.rs`) | the cycle's phytomer budget an eighth of the asked | red: `the teaser's frame: the tree outgrew its budget` | red (the exceptions) |

`fronds-round` is caught by neither: the frond count on the oak-to-palm walk stays below the oracle's 30 organs, where a form is dormant. Recorded, not acted on.

Gate at the end of task .2 (same invocation, ffce4e524): 1,466 passed, 2 failed (fn-302's two device tests), 119 ignored, 1,995 s wall including about 217 s of compile, peak 6,867 MiB anon.

## Task .3: walks merged, the wasm flower check folded, the device field at one grid (rulings 6, 3, 4)

| Test | Change | Own time (nextest, rayon 4) |
|---|---|--:|
| `walks` `every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size` | `every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` merged: the walks made once in the process, both assertions kept, no walk narrowed | 306 s (183 + 114 s apart at rayon 2, each making the walks) |
| wasm `every_shipped_preset_builds_through_the_main_entry` | also holds a preset with a null organ reference to empty organ slots, and the flowering presets to Telperion and Laurelin | 46 s (47 s before) |
| wasm `flowers_cross_the_binding_where_the_pipeline_grows_them` | builds the two flowering presets only, each still through the pipeline, the foliage request and the surface-only request | 90 s at four tests at once (128 s at rayon 2 over every preset) |
| render `every_preset_s_device_field_answers_within_its_tolerance` | grid 36 at seeds 1 and 5678 | 72 s (92 s at three grids, rayon 2) |
| render `every_preset_s_device_field_on_every_grid_answers_within_its_tolerance` | grids 24, 36 and 48, ignored, reason `fn-304: ...` (not a `device limits:` or `release check:` probe) | 102 s |

Test counts: `walks` 12 run → 11 (merge, ruling 6); wasm 7 → 7; render lib +1 ignored. The non-flowering presets are no longer built a second time. The folded check reads the binding's own organ reference rather than a separate pipeline build, and the flowering list now comes from that reference too. One case the old test caught is no longer caught: a non-flowering preset whose pipeline grows an organ that the binding drops with its reference (codex review, task .3). Named for the host as the cost of ruling 3.

### Red and green

| Patch (temporary) | What it breaks | Result |
|---|---|---|
| `insertion-switch` (`telperion-space/src/lineage.rs`), `WALK=insertion` | the insertion angle jumps at 0.55 (fn-271's switch) | merged walks test red on its by-degree assertion: `states[1].insertion seed 1 at 0.5535: a jump` |
| `vanishing-half` = `window-1e6` (presence window × 1e-6, `contract/presence.rs`) with the by-degree assertion skipped in the test, `WALK=zones[0].lateral` | a branch made at full size | merged test red on its vanishing-size assertion: `states[2].zones[0].lateral[2] seed 1 at -0.3025: 0.00145 of the tree`, fn-271's own message |
| `window-1e6` alone | the same | red on the by-degree assertion first |
| `window-narrow`, `window-1e3`, `window-1e4` | the window a hundredth to a ten-thousandth as wide | green: no step and no crossing moves past its bound |
| `stray-organ` (`telperion-wasm/src/generate.rs`) | organ slots written where nothing flowers | main-entry test red: `ordinary: organ slots without an organ`; the flower test red on its surface-only request |
| `field-leaves` (`resident/field/make.rs`) | the device's leaf density half as much again | gate test (grid 36) red; every-grid test red |

Green on the engine: every test above passes, and the whole walks binary passes (`raw/proofs/t3-walks-all-green.log`).

Gate at the end of task .3, after merging F4 at 26a3b19ab (fn-303), 7f3d88403: 1,473 passed, 0 failed, 123 ignored, 1,971 s wall including compile, peak 6,041 MiB anon. Green.

## Task .4: the runner and the width (R3, threads ruling)

The shrunk suite after tasks .1–.3 and the F4 merge (1,473 run, 123 ignored), each run alone in `telperion-heavy 8 --threads <N>` (rayon and test threads at N, nextest `-j N`), build already done. nextest 0.9.145 (CI's pin) with the one-at-a-time and device groups (`raw/nextest-final.toml`); per-test RSS from `raw/rss_runner.py`.

| Runner | Threads | Wall | Peak anon | Result |
|---|--:|--:|--:|---|
| `cargo test` | 2 | 1,971 s with compile (the task .3 gate) | 6,041 MiB | green |
| `cargo test` | 4 | 1,538 s | 7,489 MiB | green |
| `cargo test` | 8 | 1,494 s | 7,648 MiB | green |
| nextest, groups | 2 | 1,448 s | 4,867 MiB | green |
| nextest, groups | 4 | **1,241 s** | 4,913 MiB | green |
| nextest, groups | 8 | 1,271 s | 5,164 MiB | green |
| nextest, memory tokens (`threads-required` a GiB, 7 a run) | 4 | 891–1,072 s | 4,947–5,322 MiB | 57 to 239 device tests never scheduled; not a passing run |

Nothing passes in ten minutes inside 8 GB. Rayon past 4 buys nothing: the merged walks test is 223 s at 4 and 230 s at 8 (it spreads its own walks), and the other heavy tests hold flat. The floor is the tests over 1.5 GiB, which take one turn between them: at rayon 4 the ten over 3 GiB sum to 661 s and the twenty between 1.5 and 3 GiB to 421 s. Memory tokens let a 3 GiB test run beside a 4 GiB one, but nextest left whole device binaries unscheduled under them with no failure named (FRICTION.md), so they are not declared. `cargo test` peaks within 0.4 GB of the cap at 4 and 8 threads, since a binary's whole-preset tests share one process.

### The candidate gate, run once (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`)

`.config/nextest.toml`'s `gate` profile (four tests at once; the 30 tests measured at 1.5 GiB or more take one turn; device tests one at a time; 60 s slow-timeout), `scripts/gate_test.py` as the runner (4 GiB per test, or the allowlisted figure), `.config/gate-budgets.toml` drafted from the threads-4 run. Result (`raw/measure/gate2.log`): 1,124 s, peak 5,095 MiB anon, 1,469 passed and 4 ended at 60 s: the artifact test, `suite::mesh` and the merged `packed_leaf` (41–45 s on the threads-4 run, now past 60 s) and the device retire test (35 s before). No test passed its memory budget. The same tests' own times moved by up to 2× between two runs of the same suite on the shared desk (`light_no_bud_reads_leaves...` 26 → 54 s, `a_walk_between_species...` 26 → 51 s), so a 60 s wall budget fails tests that measured well under it. Held for the host: the gate's best figure (about 1,124–1,241 s) misses ten minutes, and the time budget needs a rule for that noise before the allowlist's figures can be set.

### Declared (host ruling, 2026-10-22)

`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`: nextest's `gate` profile, four tests at once, rayon at 4, inside the desk's 8 GB (this settles 8 GB against 12 GB: 8). Its passing runs: 1,241 s (the threads-4 measurement, `raw/measure/n4.log`) and 1,348 s (`gate.py` with the ruled budgets, `raw/measure/gate3.log`: 1,473 passed, 0 failed, 123 ignored, peak 5,304 MiB anon). A third, the final `gate.py` run (`raw/measure/gate4.log`): 1,137 s, 1,473 passed, 0 failed, peak 5,191 MiB. Median 1,241 s, range 1,137–1,348 s. AGENTS.md's gate line carries these figures.

## Task .5: the budgets (R4, host ruling 2026-10-22)

- **Per test, time.** The `gate` profile cuts a test off at 120 s of wall time (`slow-timeout`, `terminate-after = 1`), twice the 60 s budget for the desk's 2× swing. A test whose worst completed time across the recorded runs (`raw/measure/n2`, `n4`, `n8`, `t4tok*`, `gate2`) passed 60 s is held to twice that time by its own override, which `scripts/gate.py` writes from the allowlist as a nextest tool config.
- **Per test, memory.** `scripts/gate_test.py`, the gate's target runner, reads each test process's peak resident memory (`getrusage` of the child; nextest runs one test a process) and fails a test past 4 GiB, or past its allowlisted figure (the worst measured peak and a tenth more, rounded up to 64 MiB).
- **The allowlist** (`.config/gate-budgets.toml`, nine tests):

| Test | Time budget (worst) | Memory budget (worst) |
|---|--:|--:|
| `walks` every_setting_..._at_vanishing_size | 636 s (318 s) | 5,952 MiB (5,396) |
| `telperion_wasm` flowers_cross_the_binding_... | 239 s (119 s) | 4,096 MiB (3,977) |
| `walk_leaves` spruce_to_birch_... | 186 s (93 s) | 4,096 MiB (1,649) |
| `telperion_core` repeat_tests::the_largest_tree_and_the_palm_... | 181 s (90 s) | 4,096 MiB (3,276) |
| `telperion_render` retire_tests::the_node_table_with_retirement_... | 164 s (82 s) | 4,096 MiB (491) |
| `telperion_core` suite::packed_leaf::every_station_... | 139 s (69 s) | 4,800 MiB (4,344) |
| `ground_reach` nothing_drawn_passes_below_the_ground | 124 s (62 s) | 5,184 MiB (4,702) |
| `telperion_core` suite::mesh::every_preset_builds_a_mesh_... | 120 s (54 s) | 4,864 MiB (4,374) |
| `conformance` every_shipped_family_renders_through_the_one_path | 120 s (50 s) | 4,992 MiB (4,529) |

- **Shrink only.** `gate.py --base <ref>` (default `master`) compares the allowlist with the base's copy: a new entry or a raised figure fails, a missing base ref fails with its error, and a base without the file is reported as this branch setting the baseline. An entry for a test the gate does not run (gone or ignored) fails as stale.
- **Total.** The test run fails past 1,560 s: 1.25 × the 1,241 s median, rounded up to the minute (1,620 s while the median stood at 1,295 s over two runs).

### Demos

| Demo | Result |
|---|---|
| a temporary test sleeping 125 s, run as the gate runs it | `TIMEOUT [120.002s]` (`raw/proofs/demo-budgets.log`) |
| the same with an allowlist override of 130 s | `PASS [125.042s]` (`demo-raised.log`) |
| a temporary test holding 4.5 GiB | `fn-304 memory budget: tmp_budget_demo holds_past_its_memory_budget peaked at 4609 MiB, over its 4096 MiB` |
| the allowlist with a new entry, against HEAD | `walks light_changes_the_tree_by_degree: new on the allowlist` (`allowlist-demo.log`) |
| a raised figure | `walks every_setting_...: seconds raised from 636 to 700` |
| an entry for a test that does not exist | `walks no_such_test: stale, the gate runs no such test` |
| the allowlist as committed | no finding |

The demo test file was removed and never committed.

## R5: not met

The spec's target, the whole gate under ten minutes on the declared invocation, is not met: the median is 1,241 s. At 4 threads the tests at 1.5 GiB or more take one turn between them inside 8 GB, and their own times sum to about 661 s (the ten over 3 GiB) + 421 s (the twenty between 1.5 and 3 GiB) ≈ 1,080 s before any other test runs. More threads do not shorten them (rayon 8: 1,271 s). The rest of the gap, the heavy tests' memory and the memory-token scheduling that left device tests unscheduled, goes to a follow-up spec the host mints (host, 2026-10-22). `npm test`: green (22 files, 177 tests).

## Open

- The case lost to ruling 3: a non-flowering preset whose pipeline grows an organ that the binding drops along with its reference is no longer caught (task .3).
- nextest runs no doc-tests; the workspace has none today (the `cargo test` gates ran five doc-test targets, none with a test). `gate.py` fails before the run on a doc comment that opens a fence rustdoc would test (demo: `raw/proofs/doctest-demo.log`), so one added must be marked `text` or the gate taught to run it (codex review, task .4).
- The device tests share the card two at a time at most: a heavy device test takes the one-at-a-time turn with the heavy CPU tests, so it can run beside one light device test (codex review, task .4). This is the arrangement measured.
- Two patches were caught by neither a subset nor its sweep: the light-sum order (`sum-order`) and the frond rounding on the oak-to-palm walk (`fronds-round`), each below what the tests read.
- The gate needs `cargo-nextest` 0.9.145 on the desk (the runs here used a pinned copy in the session's scratchpad); installing it is local setup, not a repository change.

### Host rulings on task .5 (2026-10-22)

- **Total budget on the test run only.** OVERRIDDEN: codex's task .5 finding (the total excludes the build and listing before it). Host: "the budget is there to catch tests growing. Build time depends on cache state, not on the change being made, so counting it would fail a cold build for no defect." `gate.py` prints the build and listing time beside the test run; the spec's R4 and R8 now say "the test run's total".
- **Build time, measured** (`raw/measure/build-time.log`, `telperion-heavy 8 --threads 4`, nextest's listing build in a separate, empty target directory): 155 s cold at four jobs, 1 s warm. The task gates' `cargo test` rebuilt about 215 s at two jobs after a change to the core crates' tests. AGENTS.md states these beside the 1,241 s median.
- **nextest checked first.** `gate.py` stops before anything else when `cargo-nextest` is missing or not 0.9.145, with one line naming `cargo install cargo-nextest --version 0.9.145 --locked` (`raw/proofs/nextest-check-demo.log`: the desk's missing binary, then 0.9.100, both refused with that line; 0.9.145 passes). It is not installed on the desk.
