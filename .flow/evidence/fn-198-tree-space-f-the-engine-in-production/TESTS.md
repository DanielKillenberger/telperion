# fn-198 task 2: the tests the engine as stage 2 changed (for the owner)

Stage 2 is the engine; a preset with no engine species (silver birch, Laurelin, Telperion, ordinary) is refused by name, `no engine species` (host decision 9). The scaffold's grower is reached by the suite alone until R0 removes it. Each test below was changed for one of these reasons.

## Expected-refusal tests (host decision 9)

A species-less preset through the pipeline or an entry now asserts the refusal by name:

- `pipeline::tests::every_shipped_preset_builds_every_artifact_through_the_pipeline` (core)
- `generate::tests::every_shipped_preset_builds_through_the_main_entry`, `a_field_request_reads_the_plan_and_names_its_stages_for_every_preset` (wasm)
- `tests::every_shipped_preset_answers_as_the_main_pipeline_does` (field)
- `conformance::every_shipped_family_renders_through_the_one_path` (render)
- `suite::sweep::every_pair_of_presets_grows_a_tree_at_every_step` (a walk between two species-less presets)

## Ignored: the scaffold's grower on an engine preset's rows (R0 removes both)

The four engine presets lost their envelope rows (host decision 7), took the examples' dressing rows and the age 40, so the scaffold no longer grows the tree these tests pinned or measured; no build grows it now.

- core suite: `codominance::{at_rate_zero_no_table_forks_whatever_the_other_rows_say, a_fork_growing_in_moves_no_wood_already_there}`, `node_budget::{a_tree_at_its_budget_finishes_every_axis, a_larger_budget_walks_toward_the_unbudgeted_tree, a_budget_the_tree_cannot_meet_leaves_it_incomplete_and_says_why}`, `short_shoots::the_beech_carries_most_of_its_leaves_on_short_shoots`, `terminal_taper::only_childless_species_structure_gets_terminal_taper`, `sweep::the_oak_to_birch_walk_ramps_the_curtain_up_from_nothing`, `clump::a_stem_born_on_a_stem_leaves_from_a_socket_not_the_ground`, `crown_retention::spruce_bearing_shoots_follow_the_secondary_span`, `generation_limits::caller_budget_above_old_ceiling_grows_the_complete_beech`, `species::{fixed_oaks_…, fixed_spruces_…, fixed_beeches_…, beech_identity_resolves_to_frozen_profile_and_native_anatomy}`, `field_plan::{count_estimates_sum_to_the_plan_total_over_a_grid, spruce_needles_are_found_by_cells_far_larger_than_their_reach}`, `growth_reference::{beech_and_birch_record_mature_height_by_age, every_shipped_species_growth_reference_matches_its_preset}`, `leaf_base_lattice::bases_at_full_width_tile_the_trunk_edge_to_edge`
- core: `family::tests::the_build_and_validate_refuse_the_same_values_by_the_same_name` (its premise, that the build reads the scaffold's rows, no longer holds)
- core: `tests/dormancy.rs::{a_dormant_row_moves_no_artifact, zero_hold_leaves_every_preset_to_the_pipe_model}` (the scaffold's rows on species-less presets)
- render: `look::the_clay_view_draws_the_still_the_room_always_drew` (pinned to fn-24's still of the scaffold's ordinary tree), `conformance::parameter_sets_nobody_wrote_by_hand_render_the_same_way` (jitters tuned to the scaffold presets; on the engine presets most leave a rail, and a run takes four minutes)

## Kept on the scaffold, engine presets skipped

Pins and walks of the scaffold over the presets keep the species-less ones: `audit::shipped_scaffolds_are_reproducible`, `drop`, `sag`, `strands` neutral pins, `identity::shipped_species_meshes_are_the_tree_recorded_before_the_levels`, `twig_extent::the_twig_layer_asks_the_crown_nothing_and_builds_one_tree`, `sweep::every_shipped_preset_carries_foliage`; the specimen cache holds the scaffold's species-less tables only; `field_plan`'s species are the birch and the ordinary tree; `outline`'s blend walks two species-less presets; `radius::hold_tests::a_hold_adds_or_loses_no_leaf` and `pipeline::tests::one_sweep_and_one_element_serve_a_request` grow with the scaffold.

## Re-fixtured on engine trees

- core: `executor::tests` (a young engine beech, age 15), `packed_leaf` (the engine presets; the box is the expansion's), `suite::mesh::a_family_the_generator_rejects_surfaces_its_own_message` (the palm), `field_plan::every_shipped_preset_has_a_plan` (the beech has one now).
- field: the ordinary tree's tests on the European beech.
- wasm: `the_field_request_selects_the_limb_order_and_defaults_to_the_family`: a shallower order parts the engine oak into fewer systems (41 at order 0 against 144 at 2; no system past order 2 at 40).
- render: a test that drew the ordinary preset draws its dressing on the engine beech at 40 (`tests/common::ordinary`); `bark_occlusion` draws the engine oak at 80 and `smooth_bark` the engine beech (the birch's dressing on it) at 80; `conformance`'s refusal names `age` (12.5) where it named the envelope; `drew_the_tree` no longer compares a frame's triangles to the CPU wood's (the GPU surfaces the curve at the screen's error, fn-208); the GPU-side leaf rows are read off the expansion.
- jev: the species runner's pins record the grown tree's height; the four engine presets' `catalogue/*/pins.json` regenerated; `/space` rows are no catalogue dial.

## Re-pinned or renewed

- `catalogue_identity` family, walk and override digests (the four engine presets' rows; the beech ships; the age walks in whole years). The refusal digest did not move.
- `sweep`'s held rows: `limbClumping`, `shortShootSpacing`, `shellDepth` move only between a species preset and a species-less one, which no walk joins.
- `material_detail` lists in catalogue order (the beech ships after the spruce).
- `docs/parameters.md`, `parameters.generated.ts` (the age row is read by the build); `docs/generation-limits-inventory.json` (the conversion's limits classified).
- New: `tests/space_presets.rs` (species as values, read back through rows, file and wire; refusals), `tests/space_stems.rs` (moved from render, through the pipeline), `pipeline::space::tests` (the canonical axis order).
