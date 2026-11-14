# fn-351: the six tests over their native memory budgets

Measured 14 Nov 2026 on `fn-351-candidate` at `a7ae69477` (worktree `.worktrees/peak`, branch `fn-351-native-peak`), all through `TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --threads 4` (so `RAYON_NUM_THREADS=4`). Raw logs are in the ignored `raw/` beside the brief's named directory (`.flow/evidence/fn-351-the-release-candidate/raw/`, see "Where this lives" below).

## Bottom line

The native peak is not more trees and not more live bytes than Wasm: each test holds one tree at a time, and in each the Telperion preset sets the peak. The gap to Wasm is glibc's malloc keeping freed memory resident in its per-thread arenas: 1.6 to 2.0 GiB of the 4.5 to 5.8 GiB. Run with one arena (`GLIBC_TUNABLES=glibc.malloc.arena_max=1`, no code changed), every test falls under its budget with room. A further 0.6 GiB of live bytes in the three `mesh::build` tests is the curve the native build traces and Wasm does not. No change inside this brief's limits (release a held tree, shrink or size a store, avoid a copy) moves a peak by more than about 35 MiB, so none was built. Every change that would reach the budgets is a host decision; the list is at the end.

## Step 1: what sets each peak

### Method

- **Gate-way peaks.** `raw/six.sh`: `cargo nextest run --cargo-profile ci --profile gate --tool-config-file gate:target/gate/budgets.toml` with `scripts/gate_test.py` as the target runner and `GATE_REPORT`, filtered to the six. `raw/one.sh` runs one test the way nextest does (the runner, the binary, `--exact <name> --nocapture`) for the runs with an allocator tunable set.
- **Live bytes and blocks.** A temporary probe (`raw/peak_probe.rs` and `raw/probe.diff`, built into `target/probe`, removed before committing): a counting `#[global_allocator]` over `System`, modelled on `telperion-wasm/src/measured.rs`, with the live high-water mark, the `telperion_space::stage` tag at it, the blocks of 8 MiB or more live when it last rose, glibc's `mallinfo2` (main arena only) and `/proc/self/status` VmHWM/VmRSS, printed at each preset boundary and reset there (`clear_refs` 5). The stage tag is one global, so a block made on the wood's thread while the leaves' thread had set `foliage` reads `foliage`.
- Not measured: the per-thread arenas' own sizes (mallinfo2 reads the main arena only); the GPU driver's share of the conformance test's RSS beyond the renderer's few MiB of live heap; per-preset VmHWM for the Wasm main-entry test (the probe's per-preset line landed in another test); time cost of the tunables beyond the single runs below.

### The table

| Test | Gate-way peak (MiB) | Budget | Preset at the peak | Live high at the peak (MiB, stage) | Not live, resident | More than one tree? |
|---|--:|--:|---|---|--:|---|
| `pipeline::tests::every_shipped_preset_builds_every_artifact...` | 4,528 | 4,096 | Telperion, the `surface` request | 2,831 @ wood's mesh | about 1,940 | No. Presets in turn; one request at a time on one clothed tree. |
| `suite::mesh::every_preset_builds_a_mesh...` | 5,459 | 4,864 | Telperion | 3,528 @ flowers | about 1,850 | No; inside `mesh::build` the wood (rings, curve, faces) runs on its own thread beside the leaves (`Schedule::Concurrent`). |
| `suite::packed_leaf::every_station_stands_inside_its_box...` | 5,100 | 4,800 | Telperion | 3,563 @ flowers | about 1,720 | No; same build as the mesh test (`executor::grow(..).expansion().mesh()`). |
| `conformance every_shipped_family_renders_through_the_one_path` | 5,851 | 4,992 | Telperion, in `mesh::build` | 3,533 @ flowers | about 2,050 | No; the `TreeMesh` (2,581 MiB live for Telperion) is held through `submit` and the render, but the render phase peaks lower (2,795 live). The renderer keeps 5 MiB of heap between presets. |
| `telperion_wasm generate::tests::every_shipped_preset_builds_through_the_main_entry` | 4,837 (4,532 in the host's run) | 4,096 | Telperion (inferred: its live high equals Wasm's Telperion figure; Laurelin's is 2,642 there) | 2,896 @ flowers | about 1,900 | No. |
| `telperion_wasm generate::tests::flowers_cross_the_binding...` | 4,608 by `one.sh` (4,674 in the host's run) | 4,096 | Telperion, the second `generate` (surface) | 2,821 @ wood's mesh | about 1,830 | Three builds of each flowering preset in turn (`pipeline::build`, then `generate` foliage, then `generate` surface); only the organ (small) and the foliage output (25 MiB for Telperion) are held across the third. |

Peaks vary by about 1 percent between runs (the main-entry test read 4,532 and 4,837).

### What is live at the peak

Telperion at seed 42, the blocks of 8 MiB or more live at the high-water mark:

- **`mesh::build` (mesh, packed_leaf, conformance):** 3,528 MiB live at the flowers: 553.6 MiB made while the stage read `foliage` (the curve or the wood's buffers, made on the wood's thread; the curve is the one store `mesh::build` keeps that the Wasm build does not trace), the clothed node store 542.8 (`nodes`, 96 bytes a node), frames 135.7, flower matrices 102.2, leafing 90.5, foliage 68.0, keys 45.2, two more conversion stores 22.6 and 13.7. Blocks of 8 MiB or more total 1,585 MiB; the rest is the wood's chunks and smaller stores.
- **Main entry (`generate`):** 2,896 MiB live at the flowers, the same as the 32-bit Wasm build's 2,896: nodes 542.8, frames 135.7, flower matrices 102.2, leafing 90.5, keys 45.2, 22.6, foliage 13.7.
- **The pipeline test's `surface` request:** 2,831 MiB at the wood's mesh: the unfitted conversion stores (nodes 560.6, frames 140.1, 93.4, 46.7, 23.4; this test calls `clothe` but not `fitted`), rings 20.4, and about 1.9 GiB of wood chunks under 8 MiB each.
- **Growth and the settle**, every test: 2,076 MiB live at the settle (226.1, 202.7 and 140.4 MiB grower stores from year 82, the settle's 128.1), below the later peaks.

### Why native RSS exceeds live bytes

After Telperion's skeleton (grown, settled, converted, the structure dropped) 886 MiB are live and the process holds 3,085 MiB: the main arena alone holds 2,259 MiB of which 2,211 are free. The growth runs on four rayon threads and the conversion on the caller's, the wood on a spawned thread; glibc gives each thread its own arena, a block freed returns to the arena it came from, and memory free in one arena is not reused by another thread's requests. The later stages' blocks come from other arenas or from fresh mappings while the earlier arenas stay resident.

Measured with the base binaries, no code changed, `GLIBC_TUNABLES=glibc.malloc.arena_max=1` (one arena for every thread):

| Test | Default (MiB, s) | `arena_max=1` (MiB, s) | Budget |
|---|--:|--:|--:|
| pipeline::tests every_shipped_preset... | 4,528, 63 | 3,048, 78 | 4,096 |
| suite::mesh | 5,459, 54 | 3,685, 66 | 4,864 |
| suite::packed_leaf | 5,100, 53 | 3,707, 74 | 4,800 |
| conformance | 5,851, 60 | 4,081, 77 | 4,992 |
| main entry | 4,837, 54 | 3,080, 75 | 4,096 |
| flowers_cross | 4,608, 129 | 3,102, 173 | 4,096 |

Other tunables on the mesh test: `glibc.malloc.mmap_threshold=131072` alone 4,976 MiB (66 s); with `arena_max=1` as well 3,664 MiB (77 s); `arena_max=2` 3,700 MiB (180 s, a run whose time is out of line with the others and was not repeated). The default times are nextest's, the tunable times `one.sh`'s wall clock; both on a shared desk, so the slowdown (about 20 to 40 percent here) is indicative, not measured under control. Under one arena the pipeline and main-entry tests, which are not on the allowlist, took 78 and 75 s: inside the gate's 120 s cut-off, over the 60 s the allowlist's rule reads as a test's own budget.

## Step 2: what was changed

Nothing in the code. Every change the brief allows was measured or sized and none reaches a budget:

| Allowed change | Where | Saving at the peak |
|---|---|--:|
| Release the foliage output before the third build | `flowers_cross_the_binding...` | about 25 MiB (Telperion's `out`, live 25 MiB) |
| Call `fitted` after `clothe`, as `pipeline::build` does | `every_shipped_preset_builds_every_artifact...` | about 35 MiB (conversion stores reserved at kept + joins + 1 against the nodes drawn: 560.6 against 542.8 MiB for nodes, and the frames, keys and lengths in proportion) |
| Drop the `TreeMesh` before the render | conformance | none: the build, not the render, sets the peak |
| Avoid `Establishing::apply`'s clone | `pipeline/space.rs` | none for these tests: the clone is made only for a rosette (`Establishing::of` is `None` for Telperion and Laurelin) |

The largest of these is 35 MiB against shortfalls of 300 to 860 MiB, so none was built: the brief's identity proof would cost a pin run for no test moved under its budget. The digest pins were not run, since no generator code changed. The `measured` rows in `.config/gate-budgets.toml` are unchanged.

## For the host: reductions not built

| Change | Kind | Saving measured or computed | Tests it takes under |
|---|---|---|---|
| One malloc arena for the process (glibc `arena_max=1`, by `mallopt(M_ARENA_MAX, 1)` at start or the environment in the gate) | allocator configuration, no tree change | measured: 1,393 to 1,774 MiB a test (table above) | all six, with 911 to 1,179 MiB room; times up about 20 to 40 percent in these runs (uncontrolled) |
| A process allocator that returns freed memory across threads (for example mimalloc or jemalloc as `#[global_allocator]` in the test binaries) | a new dependency | not measured | unknown |
| `malloc_trim(0)` after the skeleton (glibc returns free pages in every arena) | allocator call | not measured; bounded by the 2,211 MiB free in the main arena after the skeleton | unknown |
| No curve in `mesh::build` where its caller draws none, or the wood and the leaves in turn rather than side by side | a different output or order of stages | computed: about 554 to 630 MiB of live bytes (3,528 live in `mesh::build` against 2,896 in `generate`, which traces no curve) | mesh, packed_leaf, conformance would still be over their budgets on this alone (5,459 - 630 = 4,829 against 4,864 is the closest) |
| A narrower node layout (`Node` at 96 bytes, 542.8 MiB for Telperion's 5.93 M nodes) | layout change | computed: 5.7 MiB a byte cut from `Node` | none on its own |

## Where this lives

The brief named `.flow/evidence/fn-351-the-release-candidate/`; the spec's directory is `fn-351-020-the-release-candidate`, so this file and the friction entry are there. The raw logs stayed under the brief's path in the ignored `raw/` (`base.log`, `probe.log`, `exp1.log`, `exp2.log`, the `*.tsv` peaks, `six.sh`, `one.sh`, `peak_probe.rs`, `probe.diff`).

## Follow-up (host, 14 Nov 2026): `arena_max` against time

The host's full gate with `arena_max=1` for every test passed all six memory tests but timed out six others. Here: the six memory tests and those six, twelve tests in one batch for each setting, through `cargo nextest run --profile gate` with the gate's runner (`scripts/gate_test.py`), its per-test time-outs (`target/gate/budgets.toml`, written by `gate.py`'s own `timeouts`), its four threads and test groups, each batch through `TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --threads 4`, on the worktree's binaries at `3c50d2f1c` (the candidate's code). `GLIBC_TUNABLES=glibc.malloc.arena_max=N` was set for the whole batch. Raw: `raw/grid.sh`, `raw/grid.log`, `raw/grid-*.tsv`. A film render shared the machine. The batches ran in the order unset (a), 1, 4, unset (b). Then the mesh test ran alone at 2 to check whether the earlier 180 s was an outlier. The grid would have run past 75 minutes, so 8 and the full batch at 2 were dropped.

| Batch | Start, end | Load at start (1, 5, 15 min) | Load at end |
|---|---|---|---|
| unset (a) | 10:27:55, 10:41:35 | 6.16, 7.56, 13.80 | 5.73, 8.42, 12.02 |
| arena_max=1 | 10:41:35, 11:02:58 | 5.73, 8.42, 12.02 | 4.71, 17.18, 20.37 |
| arena_max=4 | 11:02:58, 11:17:03 | 4.71, 17.18, 20.37 | 2.87, 9.19, 15.58 |
| unset (b) | 11:17:13, 11:28:53 | 2.66, 8.93, 15.43 | 2.44, 6.44, 11.85 |
| mesh alone, arena_max=2 | 11:28:53, 11:29:42 | 2.44, 6.44, 11.85 | 2.16, 5.75, 11.33 |

The load was not even: during arena_max=1 and arena_max=4 the 5- and 15-minute loads stood at 9 to 20, against 6 to 13 for the unset batches. So part of those two batches' extra time may be the render's. Neither unset batch ran under the same load as arena_max=4.

### Memory tests: peak MiB (seconds), against the budget

| Test | Budget | unset (a) | unset (b) | arena_max=1 | arena_max=4 |
|---|--:|--:|--:|--:|--:|
| pipeline every_shipped_preset... | 4,096 | 4,693 (60) | 4,712 (53) | 3,048 (93) | 3,587 (52) |
| suite::mesh | 4,864 | 5,206 (54) | 5,187 (47) | 3,673 (66) | 3,832 (47) |
| suite::packed_leaf | 4,800 | 5,278 (56) | 5,221 (46) | 3,688 (63) | 3,978 (46) |
| conformance | 4,992 | 5,810 (57) | 5,859 (49) | 4,117 (66) | 4,619 (49) |
| wasm main_entry | 4,096 | 4,787 (53) | 4,331 (45) | 3,074 (53) | 3,737 (46) |
| wasm flowers_cross | 4,096 | 4,591 (125) | 4,522 (102) | 3,117 (125) | 3,462 (103) |

The mesh test alone at arena_max=2: 3,731 MiB in 49 s. The earlier 180 s was an outlier.

### The six that timed out under arena_max=1: seconds (peak MiB)

Each test's limit is the 120 s cut-off, or its row's `seconds` where it has a row.

| Test | Limit | unset (a) | unset (b) | arena_max=1 | arena_max=4 |
|---|--:|--:|--:|--:|--:|
| suite::sweep every_pair_of_presets... | 120 | 105 (449) | 96 (493) | TIMEOUT 120 | TIMEOUT 120 |
| repeat_tests the_largest_tree_and_the_palm... | 181 (row) | TIMEOUT 181 | 173 (2,708) | TIMEOUT 181 | TIMEOUT 181 |
| catalogue_identity every_walk_is_the_walk_it_was | 120 | 41 (571) | 34 (597) | TIMEOUT 120 | 81 (318) |
| walks a_walk_between_species... | 120 | 54 (726) | 65 (753) | TIMEOUT 120 | 70 (438) |
| walks light_changes_the_tree_by_degree | 120 | 54 (381) | 52 (408) | TIMEOUT 120 | 53 (300) |
| walks every_setting_changes_the_tree_by_degree... | 636 (row) | 233 (5,135) | 185 (4,985) | TIMEOUT 636 | 321 (3,836) |

A test that timed out leaves no peak: the runner reports a peak only when its child exits. repeat_tests timed out in the first unset batch too, so the machine alone can push it over its row. The rows quoted:

- `"telperion_core pipeline::space::repeat_tests::the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores"`: `seconds = 181`, `mib = 4096`, `measured = { seconds = 90, mib = 3276 }`
- `"walks every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size"`: `seconds = 636`, `mib = 5952`, `measured = { seconds = 318, mib = 5396 }`

### What the grid says

- **arena_max=1:** every memory test is under its budget, with 911 to 1,191 MiB of room. Every timing test times out, including every_setting at its 636 s row (233 and 185 s unset). The rayon-parallel tests contend on the one arena. This is the host's result again.
- **arena_max=4:** every memory test is under its budget. The room is 509 (pipeline), 1,032 (mesh), 822 (packed_leaf), 373 (conformance), 359 (main entry) and 634 (flowers) MiB. The memory tests ran as fast as unset (b). The timing tests ran slower than both unset batches: sweep timed out at 120 against 105 and 96, repeat_tests timed out against 173 (b), catalogue_identity took 81 against 34 to 41, and every_setting 321 against 185 to 233 (still half its row). The arena_max=4 batch also ran under the higher load, so this slowdown is not separated from the render's: **unknown** how much belongs to the setting.
- **arena_max=2:** measured on the mesh test only (3,731 MiB, 49 s); not on the timing tests.
- **arena_max=1 on the six memory tests only:** every memory figure is in the arena_max=1 column. The timing tests would keep their unset times, since the setting would not reach them.
- Not measured: 8, 2 on the timing tests, and any setting under the full gate's 500-test concurrency (this grid ran 12 tests).
