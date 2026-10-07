# fn-198 task 7: the test audit

The owner asked on 2026-10-07 for three things: the flaky test fixed, superfluous tests removed, and the gate made faster. Each change below is its own commit on `fn-198-tests`. A removal names the test that covers it, or the path that no longer exists. The 25 ignored tests of the old grower stay until R0 (TESTS.md).

## The flaky test

`suite::species::fixed_*_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` took its process ceiling from `MemAvailable` when the binary started. When another worktree was loading the machine, that ceiling dropped below what the binary's fixed overhead needs. The beech then missed the ceiling by one or two percent and passed when run alone (fn-206 and fn-208, FRICTION.md).

The ceiling is now a stated 6 GiB, and the charge limit is two thirds of it. Nothing reads host memory any more (`crates/telperion-core/src/suite/species/budget.rs`). A new test, `the_ceiling_and_the_charge_limit_are_stated_not_read_from_the_host`, holds this. The birch's twelve seeds peak at 1.98 GB resident against the 6 GiB ceiling, with 1.84 GB charged; two runs gave the same result.

The beech test is ignored on this branch (TESTS.md). When R0 or a later spec brings it back, its earlier peaks of 5.9 to 7.3 GB will fit under 6 GiB only by charging fewer seeds at once. That is the admission working as designed, not a failure.

## Removed, with what covers each

| Removed | Covered by |
|---|---|
| space `tests/refusals.rs` `a_trunk_below_the_ground_is_an_error` | `tests/sag.rs:147` `a_trunk_into_the_ground_is_an_error`: same refusal, and it also checks axis, PA, birth and base |
| core `suite::sweep::every_shipped_preset_carries_foliage` | `src/suite/mesh.rs:63-64` (`foliage_instances() > 0` and equal to the instance buffer), on every preset |
| core `suite::packed_leaf::a_crown_is_twelve_bytes_a_leaf` | `pipeline/foliage/packed/tests.rs:32` `a_leaf_is_twelve_bytes`; `suite/packed_leaf.rs:38` (every engine preset places leaves); `suite/mesh.rs:63` (instance count) |
| render `only_an_asked_for_run_names_the_tracked_evidence`, two of its three copies | It lived in `tests/common/resolution.rs`, which three test binaries compile. One copy stays in `tests/bark_resolution.rs` |
| jev `isolation`: the assertion `endpoint_marker() == ENDPOINT` and its helper | The check could never fail. The scan now reads `ENDPOINT` itself (`src/isolation.rs:17`), so renaming the endpoint cannot hide an import |

## A path that no longer exists: the scaffold on an engine preset's rows

These tests grew the scaffold on the oak's, spruce's or beech's rows. No build grows those since stage 2 became the engine (TESTS.md). They now skip an engine preset, as `shipped_scaffolds_are_reproducible` already did, or build it the way production does.

| Test | Change | Alone, before → after |
|---|---|---|
| `audit::every_habit_trait_moves_every_shipped_preset` | skips engine presets | 66 s → 1.9 s |
| `audit::every_bias_term_reaches_every_shipped_preset` | skips engine presets | 64 s → 0.7 s |
| `foliage::attachments::every_attachment_trait_moves_every_shipped_preset` | skips engine presets | 77 s → 4.0 s |
| `pipeline::tests::every_family_builds_the_same_bytes_under_either_schedule` | skips engine presets; adds the ordinary tree with its leaves seated, in place of the spruce's seated case | 66 s → 9 s |
| `suite::mesh::every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers` | builds an engine preset through `mesh::build`, the product, in place of an uncached scaffold | 37 s → 13 s |

## Sped up, coverage kept

- space `tests/walks.rs`: `every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` each walked every setting at three seeds. They now share one `OnceLock`. The pair costs 21.6 CPU minutes, against 33.1 run apart.
- render `tests/smooth_bark.rs`: the check that each layer moves no wood rebuilt the tree at age 80 three times. It now builds at age 20, since a material row reaches no stage before the shader. The binary took 10 s under the GPU lock, against 55 s in the baseline gate.
- jev `tests/caller.rs`: the retry back-off is `Transport::wait`, and the scripted transport records it. The two retry tests used to sleep 1 s and 7 s; they now assert the waits (1 s; then 1, 2 and 4 s) and take no time.
- npm `src/field/field.test.ts`: the every-preset agreement between the slim and main Wasm builds now runs at one seed instead of two. It had timed out at 60 s in the baseline npm test (74 s). It now takes 10 to 12 s. The first test still checks the agreement at the beech's seed 5.

## The gate's own failure

The baseline gate failed `generation_limit_guard::generation_limits_are_classified` on fn-198's tip. Task 6's `shoulder.rs` had added nine sites that were never classified. They are now in `docs/generation-limits-inventory.json`: each is a bisection tolerance or an indexing guard. The host should review those nine classifications.

## Candidates for the host (kept)

- **Tests that grow the scaffold on an engine preset's rows and were not ignored with their siblings.** `codominance` (`:114`, `:215`, `:247`, `:274`, `:324`, `:367`, `:408`), `pendulous::a_family_at_hang_zero_is_the_tree_the_curtain_never_reached`, `twig_generations` (`:181`, `:224`, `:252`, `:307`, `:327`, `:342`), `troll_limbs::off_the_rail_the_build_refuses_the_row_by_name` (the `generate` half), `sag:219` and `drop:162` (the oak and the beech in the loop), `identity`'s three dead pin records, and `audit::shipped_scaffolds_are_reproducible`'s oak and spruce pins (always skipped). None is slow now: the slowest, `codominance:114`, takes 13 s. Each either goes at R0 or moves to a species-less table, which is a design choice.
- **render `src/generation/position_probe.rs` and `position_tests.rs`.** No module declares them, so their four tests never compile. `production_positions_admit_geometry_and_reject_unsupported_inputs` tests the real `emit_positions` fallback: wire it in, or delete it with the probe.
- **render `generation::fine`.** It is `#[cfg(test)]` only, so its GPU tests (`fine/tests.rs:357`, `units.rs:319`, `chain_tests.rs:134`, `realized_tests.rs:163`, `:342`, `pool.rs:55`) guard an executor that ships nothing. This is F1's work, so it is not touched here.
- **jev `curve::gould_worked_case_grows_2_82_cm_per_decade_at_40_cm`.** It tests `gould_decade_growth_cm`, which no production code calls. It is kept because it anchors the formula against the paper's own worked case.
- **jev `finish_line::the_palms_assessment_records_its_date_cluster_as_an_owner_decided_improvement`.** `stages_downstream.rs:524` covers it except for `decided_by` and `decided_on`.
- **field `src/tests.rs:122`, the date palm at four seeds.** `src/field/field.test.ts:81` repeats it across the Wasm boundary. Keep the Rust one if an ABI failure should show without a Wasm build.
- **render `bark_resolution.rs`.** The "plain" (`ridge_scale = 0`) renders, two per test at 1600×1000, feed only `eprintln!`.
- **Slow but load-bearing, kept as they are.** `boundary.rs` in render and wasm: trybuild runs cargo, 60 s. Space `light.rs:33`, `grow/tests.rs` and `fine/tests/realized.rs` (up to 20 s each). Render `submit.rs`, `shadow.rs` and `conformance.rs` (about 21 s each, GPU). Jev `replay.rs`: the oak's second "reruns nothing" replay repeats the beech's.

## Times

Both gate runs were `cargo test --profile ci --workspace --no-fail-fast` on binaries already built, each run once under the GPU lock. Neither had the desk to itself. The load average was 20 to 55 during the baseline run (10:24 to 10:41) and 11 to 37 during the final run (11:56 to 12:04), so some of the difference comes from the desk and not from the changes (FRICTION.md, 11:35). The per-test times quoted above were measured alone or by CPU time, and those are the figures to compare.

| | Before (2a458029) | After |
|---|---|---|
| Gate wall time | 980 s, red: `generation_limit_guard` | 508 s, green |
| Sum of binary times | 969.6 s | 501.3 s |
| Tests passed / failed / ignored | 1140 / 1 / 52 | 1136 / 0 / 52 |
| `npm test` | 280 s with the Wasm build, vitest's tests 136 s; red: the slim field test timed out | 14 s with the build cached, vitest's tests 18 s; green, 143 tests |

The ten slowest binaries:

| Before | s | After | s |
|---|---|---|---|
| space `walks` | 395.2 | space `walks` | 259.2 |
| core lib | 87.8 | core lib | 38.4 |
| render `boundary` (trybuild) | 59.7 | render lib | 25.8 |
| render `smooth_bark` | 54.6 | space lib | 21.8 |
| space lib | 48.5 | core `concurrent_request` | 10.0 |
| render lib | 34.2 | field lib | 10.0 |
| render `shadow` | 22.0 | render `bark_resolution` | 9.9 |
| render `conformance` | 21.7 | space `oracle` | 8.3 |
| render `submit` | 21.3 | render `boundary` (trybuild) | 7.7 |
| core `concurrent_request` | 16.9 | render `bark_occlusion` | 7.3 |

`boundary` dropping from 60 s to 8 s is trybuild's cache, not a change made here. `walks` is still the gate's long pole: its walks of every setting and of light run at 200 steps × three seeds, with six levels of refinement. Making that cheaper means fewer steps or seeds, and that is a judgment about what "by degree" needs (fn-192 R2, host decision 9). It is left to the host.
