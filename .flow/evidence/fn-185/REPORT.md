# Where the test suite's time goes (fn-185)

Measured 2026-10-02 at master `ebd2e37b` on the owner's machine (Ryzen 9 5950X, 32 threads), and from the CI runs of 2026-09-24 to 2026-09-27 (the last ones; master at `2a6b5645` is 11 commits behind `ebd2e37b`). Raw logs, the per-test JSON and the load samples are in `raw/` (ignored).

fn-183's #137 (`d4b1c86a`) merged while this report was being written. It removes the GPU beech Tune replay and changes the generator in 44 files. The local numbers below predate it. The self dev-dependency it leaves untouched is behind the CI finding. Each claim is marked **measured**, **inferred** or **owner** (an owner decision already on record).

## Headline

1. **CI has been 7 to 8.5 minutes since 2026-09-26 because one job reruns the whole core suite.** The `core without geometry` job runs `cargo test -p telperion-core --no-default-features --lib`. The crate's self dev-dependency (`telperion-core = { path = ".", features = ["json"] }`, `crates/telperion-core/Cargo.toml:34`) does not set `default-features = false`, so `geometry` is unified back on for the test build. On 09-26, #126 (`bf5564fd`) moved 41 integration tests into the lib's `suite` module, so this job now runs all 515 lib tests under libtest. That takes 333 s of a 400 s job, which is the critical path. The run log shows `suite::species::*` running in this job, and those tests are `cfg(feature = "geometry")`. The job tests nothing that a no-geometry configuration differs on; only its 38 s build step checks that configuration. **measured**
2. **The local gate is 427 s, and the same tests under nextest are 193 s.** `cargo test` runs its 110 test binaries one after another, so the jev replay binary (120 s) and the core lib (112 s) add up rather than overlap. Compile time is not the cost: a cold build of every test target takes 47 s, an incremental build after touching the core takes 34 s, and an unchanged tree takes 0.2 s. **measured**
3. **The longest local test is jev's GPU beech replay, 152 s.** CI skips it for lack of a GPU. fn-183 removes it (owner, 2026-10-02). After that, the next longest test is 94 s. **measured; owner**
4. **No pin family in the last 30 days caught a defect; every touching commit was a re-pin or a move.** The species digests caught one real defect, the empty `Tree::pipe` in fn-177. **inferred from commit history and FRICTION entries**

## R1: time by test, binary and module

### Local gate, ebd2e37b (measured; `raw/timings.tsv`)

| Run | Wall | Of which compile | Notes |
|---|---|---|---|
| Cold build of every test target (`--no-run`, empty target dir) | 47 s | 47 s | 155 crates |
| Incremental build after touching `telperion-core/src/lib.rs` | 34 s | 34 s | load about 12 at start: another session was active |
| Warm build, no change | 0.2 s | 0.2 s | |
| Gate, specimen cache empty (the state after any generator change) | **427 s** | 0.1 s | libtest sum of binaries 424 s |
| Gate, specimen cache full | 342 s | 0.2 s | |
| nextest, same workspace, cache empty | **193 s** | 0.1 s | 1,103 tests, 2,894 CPU-s |

Under `cargo test` the gate's time is the sum of its binaries, run one at a time. The largest binaries are: jev `replay` 125 s, the core lib 112 s, core `dormancy` 20 s, render `boundary` 35 s, wasm `boundary` 13 s, render `smooth_bark` 13 s, wasm lib 9 s, and about 70 small render or jev binaries of 3 to 11 s each. Most of those are a GPU device start-up plus a few small tests.

The specimen cache is worth 85 s per gate (427 against 342). Only six of the slowest 30 tests read it, and on CI it is always cold, because its key is a digest of the sources. **measured; inferred for CI**

### The slowest 30 tests (nextest, cache empty, local)

CPU-s is each test's own wall. Together they are 1,339 of 2,894 CPU-s.

| # | s | Test |
|---|---|---|
| 1 | 152.4 | jev `replay::the_recorded_beech_replays_through_tunes_first_revision` (GPU; skipped on CI) |
| 2 | 93.6 | core `family::tests::the_build_and_validate_refuse_the_same_values_by_the_same_name` |
| 3 | 79.9 | core `suite::field_plan::every_retained_leaf_vertex_lies_in_a_foliage_cell` |
| 4 | 77.1 | core `suite::sweep::every_pair_of_presets_grows_a_tree_at_every_step` |
| 5 | 63.2 | core `pipeline::tests::every_family_builds_the_same_bytes_under_either_schedule` |
| 6 | 62.4 | core `suite::foliage::attachments::every_attachment_trait_moves_every_shipped_preset` |
| 7 | 59.7 | core `suite::codominance::at_rate_zero_no_table_forks_whatever_the_other_rows_say` |
| 8 | 49.6 | core `…specimen::interval::tests::sparse_spruce_projects_only_changed_contact_paths` (growth path) |
| 9 | 44.1 | core `suite::species::fixed_beeches_…` |
| 10 | 43.4 | core `suite::species::fixed_spruces_…` |
| 11 | 41.8 | core `pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline` |
| 12 | 41.4 | core `pipeline::branching::audit::every_habit_trait_moves_every_shipped_preset` |
| 13 | 39.7 | core `…specimen::cohort_tests::mature_cohorts_hold_for_twice_the_lifetime` (growth path) |
| 14 | 35.9 | core `…specimen::tests::local_resume_preserves_pending_runs_across_irregular_budgets` (growth path) |
| 15 | 33.8 | core `suite::field_plan::over_coverage_is_at_most_twice_the_placed_field_at_a_quarter_metre` |
| 16 | 33.2 | render `concurrent_request::devices_asked_for_at_once_all_arrive` (GPU) |
| 17 | 32.5 | core `pipeline::surface::attachment::tests::contacts_read_from_the_wood_are_the_swept_ones` |
| 18 | 32.0 | core `pipeline::foliage::packed::tests::every_preset_placement_round_trips_inside_r2` |
| 19 | 30.5 | core `dormancy::a_dormant_row_moves_no_artifact` |
| 20 | 30.1 | core `suite::identity::shipped_species_meshes_are_the_tree_recorded_before_the_levels` |
| 21 | 29.2 | core `suite::sag::the_same_seed_and_row_grow_the_same_curtain` |
| 22 | 29.1 | core `suite::mesh::every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers` |
| 23 | 28.3 | render `conformance::every_shipped_family_renders_through_the_one_path` (GPU) |
| 24 | 26.9 | core `suite::species::fixed_oaks_…` |
| 25 | 26.6 | core `suite::codominance::the_seed_decides_each_fork_and_the_rate_adds_them` |
| 26 | 26.3 | render `canopy_light::each_leaf_term_lights_a_leaf_the_way_its_sun_stands` (GPU) |
| 27 | 25.1 | core `suite::packed_leaf::no_station_of_any_shipped_species_falls_outside_its_box` |
| 28 | 25.0 | core `suite::packed_leaf::the_reference_box_does_not_move_with_age` (partly growth path) |
| 29 | 23.7 | core `suite::sweep::every_shipped_preset_carries_foliage` |
| 30 | 22.8 | core `…specimen::interval::tests::interval_records_reconcile_all_age_pairs_presets_and_blend` (growth path) |

**Per binary (CPU-s):** core lib 1,669, render lib 192, jev `replay` 171, render `canopy_light` 72, render `conformance` 51, render `bark_plates` 47, render `shadow` 47, render `smooth_bark` 44, core `dormancy` 43.

**Per core module (CPU-s):** `pipeline::branching` 397, of which the hidden growth path (`specimen::*`, 128 tests) is 344. Then `suite::codominance` 147, `suite::species` 139, `suite::field_plan` 121, `pipeline::tests` 112, `suite::sweep` 109, `family::tests` 105, `suite::foliage` 63, `suite::packed_leaf` 61, `suite::sag` 55.

### CI, run 36356869053 (master 2a6b5645, warm caches; measured from the job log)

| Job | Total | Build | Tests |
|---|---|---|---|
| core without geometry | **400 s** | 6 s + 38 s | **333 s** (515 lib tests under libtest) |
| node | 303 s | | suite 277 s |
| core 3/4 | 212 s | 42 s | 145 s |
| core 1/4 | 186 s | 49 s | 112 s |
| wasm, field and jev | 166 s | 15 + 16 + 82 s (the render examples jev needs) | 27 s |
| core 2/4, core 4/4, render | 115–122 s | render 51 s | render 28 s |

The rust-cache restored with a full match on every job. A cold CI run was not observed in this window. fn-76's cold run was 6 min 23 s, against 2 min 55 s warm.

**Same-commit CI run: not available.** This branch changes only `.flow/`, so its PR run will find every Rust receipt and skip the Rust jobs. The CI numbers above come from the last master runs. Between those and `ebd2e37b` are 11 commits.

## R3: from 2 min 55 s to 7 to 8.5 minutes

Successful `Tests` run times, by day:

| Days | Typical full run | What changed |
|---|---|---|
| 09-18 (fn-76) | 2:55 to 4:10 | Six shards; core shards 2:00 to 3:13 |
| 09-22 to 09-25 | 3.8 to 4.9 min | #55 (`6aab6192`) added `core without geometry` (then 79 to 150 s) and the `package` job. The node job, about 3.5 min, became the critical path. |
| 09-26 onward | 6.2 to 8.8 min | #126 (`bf5564fd`) moved the core integration tests into the lib's `suite`. `core without geometry` jumped to 347 to 479 s and became the critical path. |

The core shards themselves are about where fn-76 left them, 2 to 3.9 min. **The whole regression is the one job**, made possible by the feature unification described above. Without that job's test step, the critical path is node, about 5 min. **measured (job timings); inferred (attribution)**

## R2: what each slow test and pin family catches

Each verdict is given with its reason. "Named" means the test cites an owner decision, a principle or a spec; "unnamed" means it cites nothing (`docs/principles.md`, step 1). No real defect catch was found in git or FRICTION for any row unless the row says otherwise.

| # | Catches | Also caught by | Requirement | Verdict |
|---|---|---|---|---|
| 1 beech Tune replay | Tune's first revision runs and a shot is chosen | — | fn-157 R8 (host, 2026-09-26) | **Delete** (fn-183, owner) |
| 2 build/validate refuse alike | A rail that validate has and the build lacks, or the reverse | `every_row_past_its_bound…` (validate side) | fn-123, host design | **Speed up.** A full build per row × 9 values × 8 families; only the error is compared. The build could stop at its first stage error, or validate could be checked against the rails' table. |
| 3 retained leaf vertex in a foliage cell | A non-conservative plan field | #15 (same subject) | fn-100 R4 | **Keep, speed up.** Hundreds of thousands of spruce leaves × 3 cell sizes; one size, or a sampled subset, says the same. |
| 4 every pair of presets at every step | NaN or validate holes between presets | the two named walks (skeleton only) | fn-24, unnamed in code | **Speed up.** 210 builds; endpoints rebuild each preset 6 times. Fewer steps, or skeleton only. |
| 5 same bytes under either schedule | Races in the threaded build | #17 (partial) | fn-102 R8, unnamed in code | **Keep.** It is the only race check. Could run on 2 or 3 families. |
| 6 every attachment trait moves every preset | An inert canopy trait | #12 (same pattern for habit) | fn-24 task 3, unnamed | **Merge** with #12 into one "every trait moves" table on fewer presets. |
| 7 at rate zero no table forks | Fork rows leaking at rate 0 | `dormancy` (Ordinary only) | fn-170 | **Speed up.** The dialled tree is grown twice. **Merge** into `dormancy`'s table. |
| 8, 13, 14, 30 growth path (`specimen::*`) | Growth-path state and projection | Each other | AGENTS.md: "kept buildable and pinned, never a gate" (owner, 2026-09-18) | **Question (owner).** The 128 growth-path tests cost 344 CPU-s (12%) and gate every PR, against "never a gate on species work". "Pinned" could mean one digest test. `sparse_spruce` caught a defect in fn-48. |
| 9, 10, 24 species fixed seeds | Any change to mature species output; profile regressions | #20 (seed 7) | Digests: "updated in the same commit"; thresholds cite fn-37 | **Keep.** The species digests are the one family with a real catch (fn-177). |
| 11 every preset, every artifact | A preset the pipeline cannot build | wasm entry test, #22 | `docs/principles.md` item 2 (owner) | **Keep** (owner principle). It could share #22's trees. |
| 12 every habit trait moves | An inert habit trait | #6 | unnamed | **Merge** with #6 |
| 15 over-coverage at most twice | The plan field bloating | #3 | fn-100 R5 | **Keep**; share #3's placement |
| 16, 23, 26 render GPU tests | Device concurrency, one render path, leaf lighting | — | conformance: principles; others unnamed | **Keep.** GPU start-up dominates. Out of scope until measured per device start. |
| 17 contacts read are the swept ones | In-place ring read drifting from the sweep | #5 | unnamed | **Keep.** Could cover fewer families. |
| 18 placement round-trips inside R2 | Codec loss beyond R2 | #27, constructed codec tests | fn-86 R2 | **Speed up.** Use cached specimens; one seed of two species suffices for a codec. |
| 19 dormant row moves no artifact | A false dormancy claim | #7, `zero_hold…` | Principle "dormant where absent" (uncited) | **Keep.** It caught a false claim in fn-159. Absorb #7. |
| 20 identity pins, seed 7 | Any change to four species at seed 7 | #9/#10/#24 digests (a superset) | unnamed | **Delete or merge.** It is a second pin of the same output; the digests already hold seed 7's trees at other seeds. It costs 13 re-pins in 30 days. |
| 21 same seed, same curtain | Nondeterminism in the curtain | the species digests, cache `verify` | fn-37/44, unnamed | **Merge** into one determinism test. |
| 22 every preset mesh describes its buffers | Renderer-facing buffer inconsistencies | #29 (contained) | unnamed | **Keep**; read #11's trees |
| 25 seed decides each fork | Fork placement by seed and rate | — | fn-170 | **Keep** |
| 27, 28 packed leaf box | A leaf box too small; a box that moves with age | #18 | fn-86 R3 | **Keep #27, speed up** (use the cache). #28 is growth path; see 8. |
| 29 every preset carries foliage | A preset with no leaves | #22 (fully contained) | unnamed | **Delete.** #22 asserts the same on the same presets. |

### Pin families

| Family | Commits, 30 days | Defects caught | Re-recorded by | Verdict |
|---|---|---|---|---|
| Species digests (`suite/species/digests.json`) | 5 | 1 (fn-177 empty `Tree::pipe`) | Hand, from failure text | **Keep**; recorder |
| `identity.rs` `PINS` | 13 | 0 | `print_pins`, pasted by hand; doc command stale since `bf5564fd` | **Merge into the digests** |
| `catalogue/<id>/pins.json` | 6 | 0 | Species runner on accept, otherwise by hand | **Keep** (the runner's record); derive from the recorder |
| `NEUTRAL` × 3 (`drop`, `sag`, `strands`) | 6 to 7 each | 0 | `print_neutral` by hand; doc commands stale | **Recorder** |
| `audit.rs` scaffold hashes | 9 | 0 | Hand, from `assert_eq` | **Delete** (duplicates the digests' reproducibility) or recorder |
| Generation-limits inventory | 21 (most churned) | 0 | Hand; `list_candidates` dumps sites | **Question (owner).** Keyed on source text, it re-keys on any rewrite. |
| `docs/parameters.md`, `parameters.generated.ts` | 7, 5 | 0 | `TELPERION_WRITE_REFERENCE=1` | **Keep** (a generated contract) |
| `catalogue_identity` digests | 5 | 0 | Hand | **Recorder.** The refusal digest pairs rows by index, so any new row moves it. |
| jev replay tape | 7 | 0 | Live re-record, about 8 to 9 paid Jev calls | **Speed up**: narrower request key (below) |
| trybuild `.stderr` | 1 | 0 | `TRYBUILD=overwrite`, undocumented | **Keep** |

One new preset row re-pins up to seven of these families in one PR (fn-61, `ed23f773`). **inferred**

## R4: ranked follow-up specs

Savings are measured where stated, otherwise estimated (**est.**). "Obvious" marks a fix whose cause is named and whose remedy changes no product behaviour or owner decision (AGENTS.md, "Friction reports").

1. **The no-geometry job tests the no-geometry build.** Add `default-features = false` to the self dev-dependency, and keep `geometry` on where the workspace tests need it, so `--no-default-features` really drops it. That job then runs only the plan-layer unit tests. Saving: CI critical path from about 400 s to node's about 300 s, so **CI ~8 min → ~5 min**. **est.**, from the measured job times. It may surface no-geometry test failures that never ran; that is the job's point. **Obvious.** Friction: (h) CI wait.
2. **The local gate runs under nextest, as CI does.** Saving **427 → 193 s per gate, measured**, and after fn-183 about **120 s est.** It also ends the memory-ceiling flake that `cargo test`'s shared process causes (fn-158, fn-149), and keeps a gate inside the 10-minute background cap (fn-35, fn63, fn-143). It changes the AGENTS.md gate command, so it needs the **owner's word**. nextest 0.9.145 is the CI pin. Friction: (a) 42 min, (e) memory-ceiling entries, (g) 80 min.
3. **One recorder rewrites every pin from one run.** It covers `PINS`, `NEUTRAL` × 3, `audit.rs`, `catalogue_identity` and `pins.json` (the digests, which already print both values, too). It also fixes the stale `--test identity/sag/drop/strands --release` doc commands. Saving: about 10 to 25 min per intended tree change (fn-183, fn-61, fn71), **est.** Friction: (c) 51 min.
4. **The replay tape keyed by request shape.** Key requests on their stage and the decision inputs, not on dial-table text and row order. fn-183 removes the GPU beech Tune test, but the offline beech, oak and trim replays still key on full request text. Saving: four re-records in four days, each about 8 to 9 paid Jev calls and 10 to 20 min (fn-170, fn-177, fn-179, fn-180). Friction: (d) 248 min, about 50 Jev calls, the largest.
5. **Delete what another test already asserts.** Delete #29 `every_shipped_preset_carries_foliage` (24 CPU-s), and the `identity.rs` pins (#20, 30 CPU-s plus 13 re-pins a month). Merge #6 and #12 into one trait table, and #7 into `dormancy`. **est.** 100 to 150 CPU-s. #29 is **obvious**; the rest delete pins, so they go to the owner.
6. **The growth path stops gating.** Keep one pinned digest of the growth path and move the other 127 tests behind an ignored or opt-in run. Saving 344 CPU-s (12%), measured. The **owner** decides what "pinned" requires.
7. **The slow suites build each tree once.** Route #2, #4, #5, #11, #17, #18 and #27 through the specimen cache, or shrink them as R2 says. Saving: **est.** 200 to 300 CPU-s, enough to drop the core lib's tail below the species tests.
8. **The generation-limits inventory keyed on something that survives a rewrite**, or deleted. It has the most churn of any pin, 21 commits, and no recorded catch. Delete it or key it per function and kind. **Owner** call, because the guard is an owner-set principle check. Friction: (f) 31 min.
9. **A gate policy that surfaces every failing assertion in one run.** `--no-fail-fast` already runs every test. What remains is tests that stop at their first assertion (fn-183 task 2, five runs). A collecting assertion helper in the slow suites would fix that. Friction: (b) 5 min plus fn-183's 8 min. **est.**, low.
10. **The objectives race** (`objectives.rs:28`): already fixed on fn-183 (host, 2026-10-02). Nothing left.

## How this was measured

- `raw/timings.tsv` holds the step walls; `raw/load.tsv` holds the 1-minute load every 10 s.
- The script waited until no other checkout's build or test ran and the load was under 4 before it started. Its "foreign process" column also counted the run's own test processes (they run with the crate directory as cwd), so it shows no contention either way. The load trace matches the run's own phases.
- The incremental-compile sample started at a load of about 12, while another session was active.
- Per-test times come from `cargo nextest run --cargo-profile ci --workspace --message-format libtest-json-plus` (nextest 0.9.145, the CI pin, from a scratch install) and are summarised in `raw/per-test.json`.
- CI job and step times come from `gh run view --json jobs` and the job log of run 36356869053.
