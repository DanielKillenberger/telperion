# fn-349: the corrected balance on the CPU (steps 1 to 4, and R5)

Builder's record, 11 Nov 2026, under SCOPE-ANSWERS.md. The device's WGSL is unchanged. No gate and no review ran.

Marks: **measured** (printed by a run), **read** (seen in the code).

## What is built

1. **The species value.** `PaState::inverse_height` replaces `upkeep`. It is the reciprocal of the potential height, per metre, in the same per-age row.
   - **Rows.** The scale and walk table, the catalogue row and `edges.rs` give it the range 0 to 1. `validate.rs` checks that range.
   - **`tolerance.rs`** no longer copies the value from the oak.
   - **Templates:** oak 1/34.8, beech 1/40, birch 1/30, spruce 1/61, each with its source.
   - **Presets.** The `.values` of the oak, beech, birch and spruce state the value at every age where `upkeep` stood. The oak's is `0.02873563218390805`, the shortest literal for 1/34.8. Ordinary, laurelin and telperion lose the row and are not charged. The date palm had none.
   - **Generated tables.** `src/browser/parameters.generated.ts` and `presets.generated.ts` were regenerated with `TELPERION_WRITE_REFERENCE=1`.
2. **The path in `Pencil`** (`grow/sketch.rs`). Each pencil holds the path from the root collar to its base (`root`), and one f32 per laid phytomer (`path`).
   - `Layer::step` now returns the internode length it laid. The rough layout pushes `path_end + length`.
   - **A new axis's base** is its bearer's path at its node for a lateral or relay, its bearer's end for a continuation, and 0 for the seed.
   - **A relay of an apex that laid nothing** starts at its bearer's base. This case was **measured**: a relay at node 0 of a bearer with no phytomers.
   - **Footprint.** The paths are counted as `pencils.path`.
   - **Test:** `paused::tests::a_tips_path_survives_letting_go_and_a_pause`.
3. **The rule, CPU.**
   - **`passes.rs::balance_one`:** a living tip collects `size × light` and is charged `charge(size, path, inverse) = size × (path × inverse)²`. Anything not lit adds nothing of its own. `decides` drops `node != TIP`, so only the seed's axis is untested.
   - **`working.rs::totals`:** a child lateral is summed only where its light is at least its charge. A continuation or relay is always summed. A closed child adds nothing.
   - **Deleted:** `Working::frozen`, `closed()` and the balance's use of the pencil's wood (the `wood` field went with them).
   - **`compact.rs`** lets go of the axes inside every retired root, with no wait on a frozen sum.
   - **`contract/shed.rs::now`** returns 0, not 1, where light and charge are both nothing.
   - **The whole-tree copy (`grow/balance.rs`) is deleted.** No test held its sums against the cycle's: its users were the speed levers on unlit species, the rough-layout test and the stage report. The whole-tree grower now balances through the one cycle's working set (`open`, `balance_one`) and drops that set before its settle. Its other tests stay green.
   - **R4 tests:** `grow/cycle/balance_tests.rs` (dead twigs charged nothing; the charge's square form on a hand-built tree; one paying and one failing child, with a failing continuation tested).
4. **The captures.**
   - `capture.rs` names `pa_inverse_height` and `path`, which hold the tip's path end.
   - `capture_hierarchy.rs` names `hier_*_path`.
   - `capture_layout.rs`'s pencil row carries the path end where it carried the wood.
   - The gpucycle example's names follow.

## R5: the product against the probe's SR census (measured)

Oak `oregon-white-oak`, seeds 1001 and 1007. The product's census was read where the probe read its own, at the balance of the next year, after retirement. That needed a temporary print, kept out of the commits (`r5-held.sh`); the logs are in `raw/r5-held-*.log`.

| seed | year | held, product | held, SR | living tips, product | living tips, SR |
|---|---|---|---|---|---|
| 1001 | 39 | 304,231 | 304,231 | 24,702 | 24,702 |
| 1001 | 119 | 3,370,068 | 3,370,052 | 197,331 | 197,330 |
| 1001 | 299 | 1,824,555 (−0.66%) | 1,836,625 | 116,618 (−0.09%) | 116,724 |
| 1007 | 39 | 441,713 | 441,713 | 40,184 | 40,184 |
| 1007 | 119 | 5,291,910 | 5,291,965 | 320,333 | 320,337 |
| 1007 | 299 | 2,580,712 (+4.1%) | 2,478,268 | 142,532 (+4.5%) | 136,411 |

**Why 1007 at 299 differs (measured).** The product multiplies the path by the stored inverse height; the probe divided it by 34.8. The two differ in the last bit of an f32.

- **With the charge taken by division,** under a temporary switch kept out of the commits, the product gives the probe's six counts exactly, held and living tips alike (`raw/r5-held-divide-*.log`).
- **So the trees are the same rule.** By 119 the rounding has moved 1 to 4 tips. By 299 it has carried seed 1007 to a neighbouring trajectory, 4% away; seed 1001 stays within 1%.

## Tests that fail, and why

- **Device tests (`telperion-render`, `generation::cycle`).** Expected to fail until the device carries the rule; they were not run here. The device still charges `P_UPKEEP × T_WOOD` and adds frozen upkeep. Its walk skips continuations. Its `now()` returns 1 where the CPU's returns 0. Its capture names have changed (`wood` to `path`, `pa_upkeep` to `pa_inverse_height`).
- **`walks::light_changes_the_tree_by_degree` and `walks::a_sample_of_settings_in_leaf_changes_the_tree_by_degree`** (measured, three runs).
  - **With the fixture as it was** (`inverse_height` 0.3, walked 0 to 0.6): the walk over the inverse height is too steep (36.7 and 54.7 per unit), and there are jumps at `states[2].inverse_height` 0.111, at `leaf_area` and at `states[1].viability`.
  - **With 0.1, walked 0 to 0.2:** more of them, at `apical_control`, `shade_hazard` and `viability`.
  - **With the pays test switched off** by a temporary, uncommitted switch, so every child is summed: the in-leaf sample passes, and the light walk fails only on steepness, with no jumps.
  - **So the jumps come from the pays test, which is a switch.** A child lateral's light and charge leave its bearer's sums at once when its light crosses its charge. That is a question for the host; the fixture is left as it was.
- **`paused::tests::a_growth_that_failed_is_spent` and `light::tests::a_lattice_past_its_nodes_is_refused_by_name`** (measured). The oak with 5 m internodes at age 8 no longer passes 2^28 lattice nodes. Its lattice comes to about 233 M nodes, and each core's 1.87 GB buffer runs the run out of memory before any refusal (FRICTION.md).
  - **Neighbouring settings cannot be refused by the lattice.** Every internode from 4 to 10 m and seeds 2 to 7 either grow, or are refused for a position past 4,096 m, or run out of memory. None is refused for the lattice.
- **`light_rows::each_populated_response_moves_the_tree_through_its_consumer`** (measured). The test holds that the balance hazard moves each lit preset at 40, "where the oak's balance has shed". Under the new charge, zeroing the hazard moves no preset at 40 (the oak, beech, spruce and birch alike). At 60 it moves the oak, the beech and the birch, but not the spruce (`zz` check, not committed). So the balance now first sheds between 40 and 60. Whether the test moves to 60, at its cost, or the rule should bite earlier is the host's call.

**Updated, each with its reason in the code:**

- **`cycle_cost_tests::every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`:** repinned. The oak's finest and the birch's coarsest digests at 16 years moved, as the change intends.
- **`catalogue_identity`, three digests:** repinned for the row's new name and range.
- **`space_presets::every_engine_preset_carries_the_engines_species`:** the Two Trees are now the beech with no inverse height.
- **`tolerance.rs`'s mapping test:** the inverse height is never copied.

## BUILD-ANSWERS.md 1, 2 and 4 (built, 11 Nov 2026)

- **1, the rule restated.**
  - **The rule.** `Working::totals` makes a part earn its own light, plus `max(0, light − charge)` of each open child lateral, and cost its own charge. A continuation or relay adds its light and charge whole, as its bearer's own line.
  - **The tests.** `a_part_earns_what_its_paying_children_leave_over` replaces the pays test. A new test, `a_child_at_break_even_moves_its_bearer_by_degree`, holds the bearer within 2e-6 across break-even.
  - **What did not move.** The cost-test digests at 16 years and the catalogue digests did not move.
- **2, the span.** The scale row's span is 0 to 0.1, and validation stays 0 to 1. The walk steps the inverse height over 0 to 0.1. The walk fixture (`leafy`) sets 0.1, the top of the span, so that the balance acts on a walk tree a few metres tall; that value is the builder's choice.
  - **The walks are still red (measured, one run, 223 s), and the test is not loosened.** Returned with the numbers:
    - **`light_changes_the_tree_by_degree`:** too steep for `states[1].inverse_height` (417.4 per unit in total length) and `states[2].inverse_height` (509.4). Also too steep for `states[0..2].apical_control` (60.2, 84.0 and 58.5), `leaf_area` (88.0) and `extinction` (52.8). One jump, at `states[2].tolerance` 0.335, seed 1.
    - **`a_sample_of_settings_in_leaf_changes_the_tree_by_degree`:** one jump, at `states[1].viability` 2.58, seed 1.
    - **The steep rows other than the inverse height** (`apical_control`, `leaf_area`, `extinction`) were not steep under the fixture's earlier 0.3 per-metre upkeep. They may follow from how hard 0.1 charges a walk tree a few metres tall (unchecked).
- **4, `light_rows`.** The test grows each species with an inverse height of 0.1 (`acting`), and the presets keep their values. The balance hazard then moves every covered preset at 40, the ordinary among them. So the ordinary's "neutral by its values" exception is gone: the ordinary now moves too. Green.

## DRAWN-ANSWERS.md, the tests still red (11 Nov 2026)

- **The lattice tests are green.** `Grid::spanning`, `Field::quantised` and `Field::quantised_with` now take the node limit as an argument. The product passes `field::NODES` (2^28) wherever it spans a lattice. The grower holds it as `lattice`, set to `NODES` and copied into the settled copy.
  - The two tests grow the plain oak to 8 on `Paused::lattice_within(field::SMALL)`, 27 nodes, a 1.5 m box. The oak outgrows it in its first years.
  - No public signature changed: `Field` is crate-internal.
- **The walks** (`leafy` at 1 / 34.8, the oak's own value; one run, 230 s; measured).
  - **`a_sample_of_settings_in_leaf_changes_the_tree_by_degree` is green.**
  - **`light_changes_the_tree_by_degree` is red on one row, the walk over the inverse height itself:** `states[1].inverse_height` 181.2 per unit (in branch 070fba5575613682) and `states[2].inverse_height` 115.7 per unit (in branch 4d8dd0597d6e3c6b). Every other row of the light walk is now within its bound, with no jumps.
- **How the test measures steepness** (`tests/walk/degree.rs`, read).
  - **The setting is walked in steps** from `low` to `high`, here 0 to 0.1, linear, with `stretch` 1, so the walk's unit is one unit of the setting itself.
  - **Each step's slope** is the largest change of the tree's total length, height, spread, any branch's base or tip, or the wood made or unmade, as a share of the tree, divided by the step's length in that unit.
  - **The bound** is `MULTIPLE` = 30 per unit for every walk but the two `ROUGH_LAYOUT` exceptions (33).
  - **The steepest three steps per walk are also split finer**, six levels of eight, and must shrink 20-fold, which separates a slope from a jump. Both inverse-height rows pass that split: they are steep, not jumps.
- **What the numbers mean.** The bound is per unit of the raw setting and does not scale with the span.
  - **The span is 0.1,** so 30 per unit allows a branch to move by 3 tree-shares over the whole walk. 181 per unit is a branch moving by about 18% of the tree per 0.01 of inverse height.
  - **The old row was walked over 0 to 0.6** in a unit with six times the span. Not loosened; for the host.

## The device step (DRAWN-ANSWERS.md, built 11 Nov 2026)

- **The path.** The laid phytomer table gains a column, `PH_PATH` (`state.rs` `LAID` 14 to 15), written as `lay_draw.wgsl` lays each phytomer: the pencil's path plus the length `step` laid, which `step` now returns. The pencil's wood word, `P_WOOD` (54), is now `P_PATH`: the path to the tip, set at framing to the base path (`lay_pencil.wgsl::base_path`, with the CPU's rules, the relay of an empty bearer included). `glue.wgsl` carries it into the tree's `T_PATH` column.
- **`hier_sweep.wgsl::totals`.**
  - A lit axis earns `size × light` and is charged `size × (path × inverse height)²`.
  - A continuation or relay child adds its light and charge whole, and a lateral child adds `max(0, light − charge)`.
  - No frozen branch: a retired root holds nothing lit and adds nothing.
- **`walk`** decides wherever the bearer exists and the subtree lives, so continuations and relays are tested. **`now()`** returns 0 where light and charge are both nothing.
- **Deleted:** `freeze_main`, the axes' `A_FROZEN` column (`AXES` 52 to 51), the tree's `T_FROZEN` column (`TREE` 15 to 14), and their writes and reads.
- **Renamed:** `P_UPKEEP` to `P_INVERSE`; `hier_in_wood` to `hier_in_path`; the readback's `wood` to `path`; "charge at the open axes".
- **The device suites (measured, `device.sh`, one test at a time on the card under `telperion-heavy 16 --gpu 4`):** `generation::cycle`, 59 passed, 0 failed and 27 ignored (measurements), in 900 s. That covers the hierarchy's sums against the CPU's within tolerance, every young preset grown whole against the CPU by lineage, retirement and compaction, the settle, mirror, race, pause and growth. The suite of `hier_tests` alone was green before the frozen columns went, and green after.
- **The rest of the render crate (measured, `render-rest.log`):** 190 passed, 38 ignored and 3 failed. `shader_tests::every_shader_file_is_checked` failed only because a shader was edited during the run, and it is green on the rebuild. Two still fail, and they pass on the base commit (`.flow/tmp/base`, measured):
  - **`cpu_builds_tests::fn298_the_beech_to_spruce_at_0_216_builds_on_the_device`:** the beech-to-spruce walk frame at t 0.216, seed 7, age 40 has 241,939 nodes and 79,473 instances on the device against the CPU's 241,940 and 79,474.
  - **`film_walk_tests::the_rounds_frames_have_the_cpus_counts`:** the same frame, and the oak-to-palm frame 37 (t 0.2266, seed 1), with 68,654 nodes on the device against 68,653.
  - **Why (measured in part, the rest unchecked).**
    - **Measured:** the device grower is held to the CPU within tolerance, not to the bit. On the young presets `grown_tests` prints phytomer nodes 6e-6 apart, unit shade light 1e-6 and tips 1e-3 relative. The charge's products were kept apart (`hm`, `apart`); the counts did not move.
    - **Unchecked:** the path feeds the balance with the layout's lengths, squared. A lateral within a last bit of break-even can then be shed on one side and kept on the other, which is what a one-node difference at age 40 looks like.
    - **Not done:** making it exact would mean holding the device's layout lengths to the CPU's to the bit, or the balance to a tolerance. That is the host's call.

## A conflict in BUILD-ANSWERS.md 2 (found 11 Nov 2026, for the host)

- **Validation reads the scale row's span.** `validate.rs` admits a value only inside the scale table's span (`admits` reads the row). There is no separate validation range for the inverse height.
- **So the two answers cannot both hold.** With the row spanning 0 to 0.1, validation now refuses anything above 0.1, which is a potential height under 10 m, against "validation stays 0 to 1".
- **The tests that pin it fail (measured):** `species::scale::edges::the_table_holds_the_bounds_validate_enforced` (the row is (0, 0.1), its edges entry (0, 1)) and `validate_refuses_at_the_edges_it_always_did` ("inverse_height refuses its own edge 1").
- **The walk takes its own range.** `tests/walk/mod.rs` states 0 to 0.1, not the table's span.
- **What would end the conflict.** Either the row spans 0 to 1, which keeps validation 0 to 1 while the walk stays at 0 to 0.1, or validation becomes 0 to 0.1 and the edges entry follows. Left as built, red, for the host.

## DEVICE-ANSWERS.md 3: the node the beech-to-spruce frame disagrees on (measured, 11 Nov 2026)

The frame is beech to spruce at 40, seed 7, t 0.216. It was read with two temporary `#[ignore]` tests, kept out of the commits: one matched the drawn nodes, the other read both sides' grown trees by lineage. No test was changed and no tolerance added.

- **The grown trees agree in their topology.** Both sides hold 64,867 axes and none is unmatched by lineage; no apex ends on one side and lives on the other. The disagreement is in the drawing.
- **The node:** the one node only the CPU draws is drawn node 230,530.
  - It is a `Terminal` twig tip at (0.988, 8.240, −0.240), y up, radius 0.00020000004 m.
  - It is grown axis 33,153, phytomer 4 of 5, the axis's tip: a lateral of PA 12, order 18, its apex living, its phytomer scale 0.7076.
  - **Its radius sits at the thinnest wood the drawing draws:** `FINEST`, 2e-4 m, exceeded by 4e-11 m on the CPU.
- **Both sides' values for its axis** (pencil, device / CPU):

| value | device | CPU |
|---|---|---|
| base scale | 0.7076182 | 0.7076182 |
| running | 1 | 1 |
| path | 8.481078 m | 8.481078 m (the same bits) |
| kept | 1 | 1 |
| light at its tip | 0.26977998 | 0.26978064 (2.4e-6 relative) |
| remembered balance | 0.8213634 | 0.8213638 |
| unit size | 0.99999994 | 1.0 (one ulp) |

- **Its own light, charge, surplus and balance** (PA 12's inverse height 0.0228376, tolerance 0, hazard 0.355):

| value | device | CPU |
|---|---|---|
| light | 0.19090122 | 0.19090168 |
| charge | 0.026546165 | 0.026546165 |
| surplus | 0.16435505 | 0.16435552 |
| balance | 0.7558383 | 0.7558388 |

- **It does not sit at break-even.** Its balance is 0.76, and its remembered balance 0.82, against a tolerance of 0. The charge, the path and the kept share agree to the bit, and no shedding decision differs.
- **Where the sides part.** They part at the unit's size, one ulp below 1 on the device. That size follows the light at the tip, which differs in the sixth digit, as `grown_tests` shows across the tree ("unit shade light" up to 6.6e-5, "pencil size" up to 1.2e-7).
  - **Measured:** the twig's radius on the CPU is 4e-11 m over `FINEST`.
  - **Unchecked:** that the one-ulp-smaller size on the device puts that radius under `FINEST`, so the device does not draw the tip.
- **What the corrected balance did** (inferred, not measured). It changed the tree, and the changed tree happens to put a twig tip within the light's rounding of the drawing's finest wood on this frame. The base's tree had no node there. The decision on what the device is held to goes to the owner with this.

## FRAMES-ANSWERS.md: both frames confirmed at `FINEST`; the tolerance is not reachable (measured, 11 Nov 2026)

These were read with a temporary `#[ignore]` test in `cycle/settle_tests.rs`, kept out of the commits. It ran once on this branch and once on the base commit (`.flow/tmp/base`).

- **Beech to spruce, t 0.216, seed 7.**
  - **The grown trees match.** 64,867 axes on each side with none unmatched, and 21,261 living apices on each.
  - **The one CPU-only drawn node** is the tip of axis 33,153 (lineage 12993352631457683011, PA 12), phytomer 4 of 5.
  - **The device's own settled radius for it is 1.9999951e-4 m,** under `FINEST` (2e-4). The CPU's tree radius is 2.0000004e-4 m, over it. The two sides are 2.6e-6 apart, relatively.
- **Oak to palm, frame 37, t 0.2266, seed 1.**
  - **The grown trees match.** 21,878 axes on each side with none unmatched, and 5,676 living apices on each.
  - **The extra device node** is a twig drawn at radius 2.0000017e-4 m. It stands on axis 7,436 (lineage 3411642497217226982, PA 14), phytomer 1 of 6, whose CPU radius is 1.9999819e-4 m, under `FINEST` by 9e-6 relative. Its horizontal place agrees to 1e-5 m; the drawn node stands 0.29 m below that phytomer's tip, which was not explained.
  - **A second, older difference in this frame.** One structural node, radius 0.0288 m, is drawn on both sides at the same place but under different identities, so it shows as device-only and CPU-only at once. The base commit shows the same pair (measured: radius 0.02889 m, the same place). It does not move the counts and is not this change's.
- **Confirmed:** each frame's count difference is one part at the `FINEST` threshold, drawn on one side and not the other.
- **Not built:** the tests' new standard. The tolerance `grown_tests` holds the device to is `FLOAT_TOLERANCE` (5e-3 of a field's largest magnitude) in `crates/telperion-render/src/generation/cycle/ruled.rs`. `ruled` is a private module of `cycle` (`mod ruled;`, `cycle.rs` line 98), and the constant is `pub(super)`. So it is not reachable from `generation::cpu_builds_tests` or `generation::film_walk_tests`. Returned, as the answer asks, with no test changed. Two further facts bear on the choice:
  - **What it would count.** 5e-3 of `FINEST` is a band of ±1e-6 m. On these frames the CPU holds 3 phytomers (beech to spruce) and 3 (oak to palm) within 1e-4 relative of `FINEST`. The band at 5e-3 was not counted.
  - **How far apart the sides actually are.** The measured device-to-CPU difference at the threshold is 2.6e-6 to 9e-6 relative.

## FRAMES-ANSWERS.md 09:05: the 0.29 m, and the band (measured, 11 Nov 2026)

- **The 0.29 m.** On oak-to-palm frame 37, axis 7,436 (lineage 3411642497217226982), the draws were compared with `telperion_space::grow`'s tips (a temporary `#[ignore]` test, kept out of the commits).
  - **Phytomer 0, drawn by both sides at the same place** (horizontal 0 and 8e-6 m): tip height 7.7931 m, drawn at 7.5050 m on both sides, a drop of 0.2881 m.
  - **Phytomer 1, the device's extra node:** tip 7.7943 m, drawn at 7.5063 m, a drop of 0.2880 m.
  - **Phytomer 4 and 5's nearest draws,** shared by both sides, stand at 7.5000 and 7.3966 m, below tips at 7.798 and 7.800 m.
  - So the shared neighbour shows the same drop as the extra node, and the offset is explained as the same on both sides' drawings. The cause of the drop itself, between `grow`'s tree and the drawn one, was not read. On the beech-to-spruce twig the drawn height equals the tip height (8.2397 m).
- **The band.** `cycle.rs` re-exports `ruled::FLOAT_TOLERANCE` to `crate::generation` (test-only), and the core's `FINEST` is now public (`telperion_core::pipeline::FINEST`), so neither test types a second number. `cpu_builds_tests::finest_band` counts the CPU's phytomers whose settled radius times the growth's scale lies within `FLOAT_TOLERANCE × FINEST` of `FINEST`. The two tests now hold the drawn node count within that N, and the leaf instances within 1, as before. N beside each frame's node total (measured):

| frame | device | CPU | N |
|---|---|---|---|
| beech to spruce, t 0.216, seed 7 | 241,939 | 241,940 | 183 |
| beech to spruce, frame 41, seed 1 | 289,270 | 289,270 | 160 |
| oak to palm, frame 36 | 53,800 | 53,800 | 23 |
| oak to palm, frame 37 | 68,654 | 68,653 | 169 |
| oak to palm, frame 48 | 30,183 | 30,183 | 90 |

  - **How loose that is.** The band is ±1e-6 m, against measured differences of 2.6e-6 to 9e-6 relative. Two frames now pass with a difference of 1 against N of 183 and 169.
  - **What the tests cannot reach.** The grown trees' axes and living apices are held exactly by the device suites (`grown_tests`), not by these two tests. Their modules are private to `cycle`, so these tests read only the drawn trees.

## The gate, once (measured, 11 Nov 2026, from `699ce4d6d` plus its formatting)

- **The run.** `gate.sh` ran the two guards (green), then `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py` on a clean tree, after `npm ci` (this worktree had no `node_modules`).
- **Stage times:** build and listing 114 s, the package 2 s, the test run 1,050 s against 1,560 s, the core without geometry 128 s, and node 181 s.
- **The test run:** 1,552 tests, 1,551 passed and 1 failed, 151 skipped. The gate fails on the suite `rust-render`; every other suite and check passed.
- **The one failure:** `telperion-render::film_overlay::no_label_of_the_shipped_map_overlaps_another` (`tests/film_overlay.rs:64`). The message is "2 of the beech, Laurelin and Telperion labels sit at their dots". The test holds that the three stand almost on one point of the release film's tree-space map, so at most one of their labels keeps its own place.
  - **Inferred:** with the Two Trees no longer charged (inverse height 0, against the beech's 0.025), their point moved off the beech's, so two labels now fit at their dots.
  - **Unchecked:** that this is the only cause.
  - The test's premise is the owner's (2026-10-19), so it is left red for the host, unchanged.

## Finding: a lattice just under its limit takes 1.87 GB per core (measured)

The light lattice is refused by name only past 2^28 nodes (`light/field.rs`, `NODES`). It costs memory per core, though, not per lattice.

- **The tree.** The oak with 5 m internodes, seed 1, at age 8 under the corrected charge has a lattice of about 233 M nodes, just under the limit.
- **The memory.** Each core then asks for its own 1,869,362,208-byte buffer: `memory allocation of 1869362208 bytes failed`, under `ulimit -v 8000000` at 4 threads.
- **The kill.** Under `telperion-heavy` the run is killed at 8, 12, 16 and 20 GB, in no case refused.

So a request the limit admits can need more than the desk has, a growing amount per core, and it fails by memory rather than by name. This is reported for the owner, not fixed here.

## BUILD-ANSWERS.md 3: the lattice tests (found, not fixed)

The answer asks for the refusal to be reached through the lattice's own value on a small tree. The lattice has no value to set:

- **The cell is a crate constant,** `light::CELL = 0.5`.
- **The limit is a constant,** `field::NODES = 2^28`.
- **`Light` holds only `extinction` and `sky`,** and neither sizes the lattice.

A finer cell for a test would mean making the cell a value, or opening the limit to the tests. Either is a design change, so it comes back to the host. The two tests stay red: `paused::tests::a_growth_that_failed_is_spent` and `light::tests::a_lattice_past_its_nodes_is_refused_by_name`.

## What the device step needs

- A per-phytomer path column, written as the device's rough layout lays each phytomer (`lay_draw.wgsl`), and a base path read when a pencil is framed (`lay_pencil.wgsl`). The relay of an empty bearer starts at the bearer's base.
- The column also goes into `state.rs`'s tables, the compaction's column lists (`compact.rs`), the pause (`build_pause.rs`) and the captures.
- **`hier_sweep.wgsl::totals`:** the charge `size × (path × row)²` for a lit axis and nothing otherwise; a child lateral summed only where `light >= cost`; the `T_FROZEN` branch deleted.
- **`hier_sweep.wgsl::walk`:** `decides` without `T_NODE != NONE`.
- **The device's `now()`:** 0 where `q + c <= 0`.
- **Deleted:** `freeze_main`, `A_FROZEN` and `T_FROZEN` (`compact.{rs,wgsl}`, `glue.wgsl`, `hier_cols.wgsl`); `P_WOOD`/`T_WOOD` repurposed as the path end; `hier_cycle.wgsl`'s `P_UPKEEP` read as the inverse height.
- `hier_compare.rs`'s "upkeep at the open axes", `grown.rs`'s `"wood"` and the gpucycle example follow.
