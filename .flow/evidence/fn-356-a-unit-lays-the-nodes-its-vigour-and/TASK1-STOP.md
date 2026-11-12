# fn-356 task 1: stopped at a refutation, 12 Nov 2026

The rule is in, built to the host's design (`9bc970234`). The census and the drawn oak on seed 1002 give three of the spec's stops at once:
- **The oak's leaf area falls by more than a tenth.** The drawn leaf falls 42% at 120 and 78% at 40.
- **A walk fails its bound.** The release-law walk now jumps.
- **The quota needed a definition the design does not give.** The one cycle lays a rough layout for every tree, so the rule also changes unlit trees, and every closed-form count test fails.

Nothing past R4 was built. No pin was repinned and no docs were written.

Tier: product code, built by Opus to the host's written design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing)

## What was built

| Commit | What |
|---|---|
| `6da2ff9a3` | The host's decision: the beech takes the 0.9 shed bound, with the measured 0.81 in a comment. |
| `308435521` | R1's fixture test, red (`grow/vigour_tests.rs`). |
| `9bc970234` | The rule, and R1's test corrected and green. |

**The rule** (`grow/unit.rs`, `lineage.rs`, `grow/shoot.rs`, `grow/sketch.rs`):
- A unit lays `min(1, size x presence)` of the summed presences of all its zones' nodes, after a lifespan's cut. They are counted from its base: zones in `ALONG` order, and nodes in index order within a zone, as `lineage::laid` counts them.
- The last node grows in under its own draw, keyed `VIGOUR` (`u64::MAX - 20`, under the node).
- `lineage::lay` is `laid`'s loop with the quota held across zones. `laid` calls it, to the bit as before.
- A unit at 1 or more takes no new step.

**Mechanical choices made (for the host to confirm or overrule):**
- **Presence** is `Pencil::presence()`, `base_scale x running x kept`. This is the presence the allotment normalises the size against in the same cycle (`Grower::inputs`, `allot_one`; both now call it). It is also the presence the survey's census read. It is not the unit's survival lead.
- **Size** is `Pencil::size`, this cycle's allotment.
- **An axis with no pencil** (made this cycle) is whole, as `size`, `light` and `kept` are.

**R1 test** (`a_unit_lays_the_share_its_size_and_presence_give`):
- **The fixture:** the lit oak, with whole units of a fixed node count (no death, abortion, relay, shedding or sleeping bud), seeds 1 and 7, age 12.
- **The check:** each unit lays its whole count where its sized scale is 1 or more, and otherwise `floor` to `floor + 1` of `share x count`.
- **Red with the rule held at 1:** "5 laid at 0.786 of 5". Green with the rule.
- The first red commit read the unit's share wrongly: the settle lays scale at the unit's size, so the share is the sized scale itself. `9bc970234` corrects that reading.

## R4: the census, the oak on seed 1002

Head is `6da2ff9a3` (the branch before the rule), built in the new detached worktree `.worktrees/fn-356-base` with its own target. Now is `9bc970234`. The reader is probe-side (`raw/runner/probe/src/census.rs`): `Paused` with its census, settled at 80 and at 120.
- **A tip** is an axis whose apex has not stopped.
- **Its presence** is its last phytomer's scale over its size.
- **The grower's leaf** is the settled scale x leafArea x the leaf-years factor.

The logs are in `raw/census/`.

| | Head 80 | Now 80 | Head 120 | Now 120 |
|---|---|---|---|---|
| Living tips | 180,727 | 54,022 | 333,373 | 90,859 |
| Summed tip presence | 98,076 | 50,823 | 125,344 | 86,274 |
| Tip size, 10th / 50th / 90th | 0.26 / 0.63 / 1.75 | 0.58 / 0.93 / 1.53 | 0.18 / 0.55 / 1.57 | 0.64 / 0.93 / 1.39 |
| Tip presence, 10th / 50th | 0.0051 / 0.68 | 0.76 / 1.00 | 0.00004 / 0.10 | 0.84 / 1.00 |
| Held phytomers | 4,683,475 | 1,196,227 | 9,175,834 | 2,125,355 |
| Under 0.2 mm | 1,161,509 | 3,074 | 3,888,756 | 4,762 |
| Grower's leaf, m² | 1,000.6 | 440.7 | 1,263.9 | 742.4 |
| Census run, s (peak GiB, whole run) | 26.8 | 5.1 | 64.8 (11.2) | 12.9 (2.0) |

Living apexes by decade, from the grower's census (cycles 10 to 120):
- **Head:** 1.8, 7.6, 26, 61, 115, 192, 283, 375, 470, 565, 650, 719 thousand.
- **Now:** 1.3, 3.9, 9.5, 17, 30, 47, 67, 89, 113, 132, 143, 149 thousand.

The tip count now flattens, rising 4% from 110 to 120 against 11% on the head. Held phytomers at 120 are 2.1 million, against the paper's 2.7 million.

## The refutation: the drawn oak on seed 1002

The scoreboard probe's `tree` measure, with no shots. Head ran at load 5; now ran at load 25 (another job). The seconds include the probe's second grow.

| | Head 40 | Now 40 | Head 120 | Now 120 |
|---|---|---|---|---|
| Height, m | 10.61 | 9.79 | 30.03 | 25.58 |
| Spread x / z, m | 9.57 / 8.86 | 6.65 / 8.13 | 39.0 / 38.8 | 29.1 / 29.1 |
| Trunk at 1.3 m, m | 0.350 | 0.235 | 1.470 | 1.036 |
| Drawn leaf, m² | 221.7 | 49.8 | 1,946.1 | 1,122.4 |
| New shoots | 35,585 | 13,217 | 390,396 | 115,794 |
| Held phytomers | 658,537 | 220,303 | 9,175,834 | 2,125,355 |
| Seconds, peak GiB | 3.4, 1.07 | 1.2, 0.39 | 227.0, 11.19 | 33.7, 2.72 |

- **Drawn leaf:** down 78% at 40 and 42% at 120, against the spec's bound of a tenth.
- **Grower's leaf at 120:** down 41%.
- **The tree:** shorter (25.6 against 30.0 m), narrower (29 against 39 m), and its trunk thinner (1.04 against 1.47 m).

The paper's estimate assumed the faint units carry the leaf they carry today. Measured: the rule also cuts units of moderate size x presence, and those compound over the years. At 40, the tip presence 10th percentile rises from 0.005 to 0.76: the faint lineages are gone, and with them a large part of the crown. No sheets were drawn.

## The narrow suites at `9bc970234`

770 tests run: 751 passed, 19 failed, in 726 s (`raw/narrow1.log`).

**Expected to move with any rule change** (pins, not repinned):
- `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`;
- the geometry spread tests (`the_spreads_move_no_node...`, `the_spread_moves_no_node...`, scale 0.0014 and 0.0007 on the oak).

**The closed form no longer counts the grower:** `the_settings_grow_the_expected_counts`, `mortal_apices_grow_the_expected_counts`, `sleeping_buds_grow_the_expected_counts`, `a_bud_never_wakes_in_a_stage_it_outlived`, `a_fractional_bud_place_is_counted_at_its_share`, `a_bud_reaching_its_stage_in_part_is_counted_whole`, `the_closed_form_counts_the_relay_of_a_move_all_but_made`, `branches_keep_on_average_what_the_formula_keeps`, and `stochastic_structures_match_the_simulators_in_distribution` (47.6 standard errors).
- **The cause:** these trees are unlit, but the one cycle always lays a rough layout (`Paused::with`). A lateral's presence there is below 1 (its base scale carries its bud's presence and dominance), so it lays fewer nodes.
- **What it means:** the rule changes every tree, lit or not, and the GreenLab closed form has no term for it.

**Locality of draws:**
- `adding_a_branch_reshuffles_nothing_else` ("s/10.0/4.0 regrew");
- `adding_a_sleeping_bud_moves_no_other_draw` (a node vanished);
- `dominance_leaves_a_few_laterals_whole` (742 against 166).

  Adding a branch moves the allotment's mean size for its kind, and the size moves every other unit's node count.

**A walk fails its bound:** `the_release_law_changes_the_tree_by_degree` jumps on `sleeping: states[0].zones[1].rate`, at seeds 3 and 1.

**Others:**
- `woken_buds_that_stop_at_once_retire_as_the_reference_has_them` ("seed 1: 9 retired").
- `a_tree_grows_the_same_without_what_it_is_certain_to_shed`: the "beech with long-kept bearers" fixture at 0.93 against its own 0.9 bound. The rule does not cause this. With the beech at 0.9 the test reaches this fixture, which was hidden behind the beech's failure at the base (FRICTION.md).

## For the host

1. **The leaf refutation:** the rule as designed removes 42 to 78% of the oak's drawn leaf.
2. **Which trees the rule reaches:** every tree, since presence below 1 exists on unlit trees in the one cycle. The closed form and its nine count tests then need a decision.
3. **Locality:** a unit's count now reads its kind's mean size, so one branch added moves others' node counts. That breaks fn-192's "adding a branch moves no other draw" tests.
4. **The shed test's second fixture bound** (0.93 against 0.9).

## Left as it is

- **R2's walk check:** written, never run, set aside as `raw/r2-walks-unrun.diff`. It folds a vanishing-size check on the shade-size walks into `light_changes_the_tree_by_degree`.
- **Not done:** R3, R5, R6, the long walk, vitest, and both guards.
- **`.worktrees/fn-356-base`:** detached at `6da2ff9a3`, with its own target in the runner's `raw/raw/target-fn-356-base`.
- **The runner copy** in `raw/runner/` carries the `census` subcommand.

# The second design: stopped at the narrow suites, 12 Nov 2026

The host withdrew the quota and wrote the second design: a tip whose unit is less than one node ends. Built, it fails the spec's refutation: closed-form oracles and independence tests fail, and three walks jump. The census and the drawn trees were not run, since the order stops at the first refutation.

## What was done

| Commit | What |
|---|---|
| `0eb6c4fd3`, `022ec1d2c` | The quota and its R1 test reverted. The crates are as at `6da2ff9a3`. |
| `5f07de93f` | The "beech with long-kept bearers" fixture takes the 0.95 shed bound. Measured at seed 1: 0.932 (234,037 of 251,179) with the beech's short shoot at 12 years, and 0.793 (148,906 of 187,687) with the fixture's short shoot set back to 4 years. The 12-year short shoot is what moved it. Seed 7 passes under 0.95. |
| `d954b888c` | R1's tests, `grow/tip_tests.rs`, red before the rule. |
| `90e3b3dda` | The rule. |

**The rule** (`grow/advance.rs`, `grow/unit.rs`, `grow/shoot.rs`):
- Beside the ground hazard, a living tip's viability is multiplied by `min(1, presence x N)`, f32 in the one cycle as the other factors are. It is decided on the same VIABILITY draw, and its lead sizes the unit's presence as the shade hazard's does.
- At 1 or more no new step is taken.
- An axis with no pencil (made this cycle) is whole.

**Choices made (for the host):**
- **Presence** is `Pencil::base_scale x running`, which is also the balance's "size". `kept` is not included.
- **N** is the sum of the unit's node presences over all its zones, by degree, before any lifespan's cut. The survival already runs over the unit's share, so a cut unit is not treated as faint.

**Dominance in the stored scale:**
- `base_scale` = the bearer node's unsized scale x the birth draws' presences x the lateral's dominance share. Dominance is not kept apart, so the stored scale is used.
- On the oak and the beech every age's `form.dominance` is 0, in `oak.rs`, `beech.rs` and the presets' value files. So on both, at 40 or any age, no lateral of a whole draw falls under `1/N` by dominance alone: 0, by code; no run was needed.
- Dominance is set on the spruce (0.35), the birch (0.2) and the ordinary tree (0.117). There a share is at least `1 - dominance`, so only a unit under about 1.5 nodes on the spruce could fall under by dominance alone. That was not counted.

**R1 tests:**
- `a_tip_under_a_node_ends`: the lit oak fixture, every draw whole, the lateral kinds' dominance at 0.99, age 7, seeds 1 and 7. Before the rule, the trunk's laterals that lay under a quarter of a node grew 100 of the 100 units their age allowed: red. With the rule, they grow under half, and whole laterals grow every unit.
- `a_tree_whose_every_tip_lays_a_node_is_as_it_was`: the same fixture without dominance. Every lateral's made presence is exactly 1. The digests are pinned from `6da2ff9a3` and are equal under the rule: (8086766086517289291, 16931195750158535152).

## The narrow suites at `90e3b3dda`

771 tests run: 756 passed, 15 failed, in 822 s, starting at load 1.7 (`raw/narrow2.log`). Nothing was repinned and no test was changed.

**Closed-form oracles** (the GreenLab closed form, or a simulator, against the grower):

| Test | What it promises | By how much it fails |
|---|---|---|
| `closed_form::kept::tests::branches_keep_on_average_what_the_formula_keeps` | the kept wood on average matches the formula | born 3: P(T ≤ 0) 0.103 against 0.05 |
| `closed_form::a_fractional_bud_place_is_counted_at_its_share` | a fractional bud place is counted at its share | PA 2 cycle 2: 2.156 against 2.3 |
| `closed_form::a_bud_reaching_its_stage_in_part_is_counted_whole` | a bud reaching its stage in part counts whole | PA 3 cycle 4: 1.499 against 2 |
| `closed_form::the_closed_form_counts_the_relay_of_a_move_all_but_made` | a relay's count | age 2, PA 2 cycle 2: 0.323 against 0.5 |
| `death::mortal_apices_grow_the_expected_counts` | mortal apexes grow the expected counts | PA 2 cycle 3: 0.19 off |
| `dormant::sleeping_buds_grow_the_expected_counts` | sleeping buds' expected counts | PA 2 cycle 4: 18.13 against 19.57 |
| `dormant::a_bud_never_wakes_in_a_stage_it_outlived` | waking counts by stage | PA 2 cycle 4: 16.15 against 16.96 |
| `settings::the_settings_grow_the_expected_counts` | every setting grows the expected counts | PA 2 cycle 4: 0.89 off |
| `oracle::stochastic_structures_match_the_simulators_in_distribution` | the grower matches the simulators in distribution | the grower refused a tree: `Err(Collapsed)` |

**Independence:**

| Test | What it promises | What failed |
|---|---|---|
| `lineage::adding_a_branch_reshuffles_nothing_else` | adding a branch moves no other | seed 1, p 0.20: s/10.0/1.1 regrew, units [6, 6] against none |
| `dormant::adding_a_sleeping_bud_moves_no_other_draw` | a sleeping bud added moves no other draw | seed 2, p 0.50: s/5.0/r/7.0/2.0/4.0/0.0 vanished |
| `form::dominance_leaves_a_few_laterals_whole` | "the same tree" | 644 against 156 |

**Walks** (by degree):
- `the_release_law_changes_the_tree_by_degree` jumps on:
  - `sleeping: states[1].zones[0].delay`, seed 2 at 0.32;
  - `sleeping: states[0].zones[1].rate`, seed 3 at 0.7618;
  - `sleeping: states[0].zones[1].delay`, seed 3 at 1.68, plain and "limbs rise and erect".
- `light_changes_the_tree_by_degree` jumps on `light: states[1].shade_hazard`, seed 2 at 0.525 and at 0.495, with a refinement floor of 0.0.

**Pins:** `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` moved on oak, spruce and birch (and others), as any tree change moves it. It was not repinned.

**Not determined:** why the independence tests move. These are inferences, not measured:
- The windows of a zone's node presences read the expected wood of its laterals and sleeping buds (`node_wood`), so a setting that adds a bud moves `N` by degree.
- A tip's presence reads its bearer node's scale.

## Not done

- R4's census and the drawn oak on the second design. The probe for the head is rebuilt (`raw/raw/target-fn-355`), so both are one command each.
- R2's walk check (`raw/r2-walks-unrun.diff` is the first design's), R3, R5, R6, the long walk, vitest, and both guards.

# The third design: stopped at the narrow suites, 12 Nov 2026

The host withdrew the survival rule and wrote the third design: the grower does not grow what is under its grain. Built, it fails one closed-form oracle in the narrow suites, which the spec names as a refutation. The base census shows why: the shares the windows read are one to two orders under the spec's estimate. The census after the rule and the drawn oak were not run.

## What was done

| Commit | What |
|---|---|
| `c9119b103`, `12b4e5479` | The survival rule and its tests reverted. |
| `d29f29cc5` | `GRAIN = 1e-7` in `presence.rs` beside `LOOK`, with the spec's comment, and exported. `Windows::start(pa, cycle)`, the share at a cycle's start that sizes a window; `window` now reads it. The census reads every living tip against the grain at the next cycle's start, as percentiles, and counts the tips under it. Nothing grows differently. |
| `b198faa60` | R1's two fixture tests (`grow/grain_tests.rs`). |
| `551494642` | The rule. |

**The rule:**
- **One site:** `Shoot::advance`, at the start of a tip's unit, after its share is known and before survival.
- **The test:** presence (`base_scale x running`) x `Windows::start(pa, cycle)` < `GRAIN`. When it holds, the apex ends as a failed apex without a relay does: `apex_end = cycle - 1`, `failed`, and its spent units recorded. It returns before any unit is grown.
- **A lateral bud under the grain** keeps its axis record but grows nothing: it ends at its first unit.
- **An axis made this cycle** (no pencil) is whole. So a woken sleeping bud is first read at its next cycle, and needs no second site.

**R1 tests:**
- `a_tip_under_the_grain_is_not_grown`:
  - **The fixture:** the oak with every draw whole, no death, and the lateral kinds' dominance at 1, so a lateral of the trunk stands at its dominance draw's fourth power.
  - **The check:** the trunk's laterals under half the grain at their first unit grow nothing, and those over twice it grow.
  - **Result:** red before the rule (seed 1: one at 1.4e-9 grew), green with it.
- `a_tree_whose_every_tip_stands_over_the_grain_is_as_it_was`:
  - **The fixture:** the lit oak at 5. The census reads no tip under the grain at any cycle.
  - **The check:** digests pinned at `d29f29cc5`, (14144581626978832127, 17731531182853926843). Equal under the rule.
  - **Why age 5:** the lit oak at 12 has tips under the grain from cycle 8 (seed 1: 2 of 745) and from cycle 6 (seed 7: 1 of 231). Its median tip reads 1.2e-6 to 2.4e-6 from cycle 3 to 12, 12 to 24 times the grain.

## The narrow suites at `551494642`

771 tests run: 769 passed, 2 failed, in 827 s (`raw/narrow3.log`).
- **`closed_form::deterministic_trees_grow_the_closed_form_counts`:** a deterministic tree grows exactly the closed form's counts. It fails on the "reiterated" set: PA 2, cycle 3, grew 0 against the formula's 2.
  - Every tip there is whole (presence 1). A PA 2 draw's window reads 2.7e-9 at cycle 3, falling to 1.5e-10 at cycle 9.
  - The whole in that share is the tree expected at the cycle plus `LOOK`, and a reiterating tree grows exponentially over 40 cycles. So whole elements of a small tree fall 37 times under the grain.
  - This refutes the design's premise that the closed form's fixtures stand far over the grain.
- **`every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`:** a digest pin, moved. Not repinned, since the run stopped.
- **Held:** every independence test, the release-law and light walks, the other closed-form oracles, and the simulator comparison.

## The grain's place on the oak at 120, seed 1002, before the rule

Read at `d29f29cc5` by the census (`raw/census/grain-head.log`). Each value is a living tip's presence x the share its window reads at the next cycle's start.

| Cycle | Living tips | Under the grain | 1st | 10th | 25th | 50th | 90th percentile |
|---|---|---|---|---|---|---|---|
| 80 | 374,527 | 316,652 (85%) | 2.1e-11 | 8.2e-10 | 4.8e-9 | 3.4e-8 | 5.7e-7 |
| 120 | 719,340 | 650,480 (90%) | 5.2e-13 | 5.7e-11 | 4.5e-10 | 3.9e-9 | 8.1e-8 |

Tips under the grain at a cycle's end, summed over the cycles of each decade before the rule. On the head they live on, so this counts the tip-years the rule would end, not tips:

| Decade ending at | Tip-years |
|---|---|
| 10 | 2 |
| 20 | 638 |
| 30 | 14 thousand |
| 40 | 85 thousand |
| 50 | 265 thousand |
| 60 | 668 thousand |
| 70 | 1.9 million |
| 80 | 2.7 million |
| 90 | 3.5 million |
| 100 | 4.5 million |
| 110 | 5.3 million |
| 120 | 6.0 million |

At 120, the 90th percentile tip reads under the grain. The spec's estimate puts a whole twig at 1e-6 to 1e-5. Measured, a tip at full presence reads about 1e-7 to 1e-6 on the young oak (cycles 3 to 12) and less as the tree grows. So the grain at 1e-7 would end about nine in ten of the oak's living tips at 120, not the faint third to half. The same census at `d29f29cc5` repeats the earlier head numbers to the unit: tips 333,373 at 120; held phytomers 9,175,834; grower's leaf 1,263.9 m²; 83 s and 11.16 GiB peak.

## Not done

- The census after the rule, and the drawn oak at 40 and 120. The probes for both sides are built against `d29f29cc5` and `551494642`, so each is one command.
- R2 to R6, the long walk, vitest and both guards.
- **The base worktree** `.worktrees/fn-356-base` is now detached at `d29f29cc5`.
