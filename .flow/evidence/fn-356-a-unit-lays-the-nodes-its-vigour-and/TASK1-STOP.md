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

# The fourth design: stopped at the narrow suites, 12 Nov 2026

The host withdrew the grain rule and wrote the fourth design: under its floor, an element's faintness is rounded, not carried. Built, it fails a closed-form test, the extent bound. It also fails the shed-certain shortcut's two tests. The census and the drawn oaks were not run.

## What was done

| Commit | What |
|---|---|
| `131350768`, `53c2baa62` | The grain rule and its tests reverted. `GRAIN`, `Windows::start` and the census reading stay (`d29f29cc5`). |
| `ec9efa1a7` | The rule, with R1's tests (`grow/round_tests.rs`). |

**Built** (`grow/advance.rs`, `grow/unit.rs`, `grow/shoot.rs`, `grow/wake.rs`, `presence.rs`, `lineage/keys.rs`):
- **Faintness:**
  - `Draws::faint` (an f32) is an axis's faintness at its base, set where every axis is made (`Shoot::push`). It is what the axis inherits times its made presences.
  - **What an axis inherits:** a lateral takes its node's faintness (`presence::faint_at_node`). A woken sleeper takes the faintness of the node it slept at, read by the grower (`Shoot::waking`). A continuation or relay takes its axis's end (`presence::faint_at_end`). The seed takes its own.
  - A tip's faintness is its base times its units' presences, in `assign_one`'s order.
- **Floor:** `presence::floor` = min(1, GRAIN / `Windows::start(pa, cycle)`).
- **The rule:**
  - **A tip, at its unit's start:** read at `Windows::start(pa, cycle)`, after the unit's cut is known and before survival.
  - **A lateral, at its birth:** in `grow_unit`, after its made presences, read at `start(lateral_pa, cycle + 1)`.
  - **Under the floor:** a draw keyed `ROUND` (`u64::MAX - 20`), under the unit's key for a tip and the bud place's key for a lateral, keeps the element with chance f / floor (`presence::rounded`). A kept element stands at the floor; the factor floor / f is folded into the unit's survival presence, or into the bud's made presence (`birth[0]`). An element not kept ends as a failed apex with no relay does, or its bud is not made.
  - **At or over the floor:** no draw is taken and nothing changes.
  - **A woken sleeper:** whole in its waking unit.
- **The census** counts the elements kept and ended each cycle (`Cycle::rounded`, through `Shoot`, `Share` and `Grower`).
- **`Draws` stays 144 bytes on 64-bit**: `an_axis_and_its_draws_stay_compact` passes. On wasm32 it is inferred to grow from 104 to 112 bytes; that was not measured.

**Settled in code, as asked:**
- **Where the by-degree presences enter the stored scale today:**
  - a lateral's birth draws as `Draws::birth`, which becomes `vigour`, then `base_scale` (`draw.rs`) and `geometry::scale` (the final lay);
  - each unit's `survive` and `persist`, multiplied into `running` (`draw_bent` and `assign_one`);
  - each node's presence (`Draws::nodes`), and each unit's `size` (the share of a node it grew);
  - the bearer's node scale, inherited unsized.
  - Dominance enters only at a lateral's `base_scale` (`draw.rs` `share`; `geometry::dominance`), and shade size only when a phytomer is laid.
  - So f is the same chain with those two left out. It is carried beside the scale, and no scale changes when the rule does not fire. Every lift goes into an existing presence (a unit's `survive`, a bud's `birth[0]`), so the rough and final lays both read it.
- **Running presence** is re-multiplied from the axis's units each time it is read, not carried. A rounded element stands at or just over its floor at that cycle (`next_up` where f32 would leave it a part under). But the floor rises as the tree grows, so a faint tip can be rounded again in a later cycle, each time by a fresh, unbiased draw.
- **Relays and "both outcomes grown":**
  - Each side has its own f. The unit the parent grows carries its lift, including the failed unit pushed in `fail`.
  - A relay's base is the parent's end, read when the relay is made, times its own made presences.
  - The final lay reads a relay's vigour from the parent's end at the tree's age (`end[parent]`), which can differ where the parent grows on as the other outcome. f reads the end at the relay's birth.

**R1 tests** (both green):
- **`a_tree_of_whole_elements_is_as_it_was`:**
  - **The fixture:** the lit oak at 7, every element whole (every lateral's vigour exactly 1).
  - **The checks:** the census counts no rounding at any cycle, and the digests equal those at `53c2baa62`, (5471803321526034208, 12893453816766107291).
- **`a_faint_element_is_kept_in_its_share_and_stands_at_its_floor`:**
  - **The fixture:** 200,000 keys at four (faint, floor) pairs.
  - **The checks:** the kept share is within four standard errors of faint / floor, and a kept element stands in [floor as f32, floor x (1 + 1e-6)].
  - It calls the new rounding function, so it could not be written red first.

## The narrow suites at `ec9efa1a7`

771 tests run: 767 passed, 4 failed, in 867 s (`raw/narrow4.log`).

| Test | What it promises | What failed |
|---|---|---|
| `closed_form::kept::tests::bound::no_wood_reaches_past_the_extent_bound` | No wood stands farther from a lateral's base than the closed form's extent bound x its base (the bearer's node scale x its vigour). Host decision 16. | Axis 588 (PA 14, born 10) reaches 0.009710 against its bound 0.009621: 0.93% past, against a tolerance of 1e-5 relative. |
| `grow::tests::a_lateral_certain_to_be_shed_is_shed` | Every lateral the shed-certain rule marks is gone at the tree's age. | Beech at seed 1: 16 marked laterals stand. |
| `grow::tests::a_tree_grows_the_same_without_what_it_is_certain_to_shed` | Leaving those laterals ungrown leaves the tree the same to the bit. | Beech at seed 1: the tree changed. |
| `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` | A digest pin. | Moved. Not repinned, since the run stopped. |

**Held:** every other closed-form oracle (the counts, death, dormant, settings, relays and kept wood on average), the simulator comparison, every independence test, the release-law and light walks, and the shed test's bounds.

**On the failures** (inferred, not measured):
- **The extent bound** assumes an axis's presences never exceed 1 past its base. A tip kept at its floor stands at the floor / f times its former presence, with no limit, so its realised reach can pass the bound even where its expectation is unchanged.
- **The shed-certain tests** run on unlit trees. Since f is read from the draws and not from a pencil, the rule now also rounds unlit trees. Why marked laterals now stand was not determined.

## Not done

- The census and the drawn oaks at 40 and 120 on seeds 1001, 1002 and 1007. The head probe is built against `ec9efa1a7`, with kept and ended per decade in its census.
- R2 to R6, the long walk, vitest and both guards.

# The fourth design with one site: returned with two cases, 12 Nov 2026

The host withdrew the rounding at a tip's unit start and kept it at a lateral's birth. That is built (`edd90ebba`). Two of the failures stay, and neither can be fixed inside the host's instructions, so both are returned. The narrow suites, the census and the drawn oaks were not run past the four tests below.

## Built

- **`Shoot::advance`** is back to its form at `53c2baa62`, the base: a living tip is never rounded.
- **The rounding stays only at a lateral's birth** in `grow_unit`, where the lift goes into the bud's made presence (`birth[0]`), and so into its vigour and base.
- **`ROUND`** is now registered under a bud place's key.

**Relays:** not rounded. A relay has a made presence of its own. But each of its makers carries the relay on as soon as it is made (`relay_bud` carries it on, `fail` pops it for its abortion, `move_on`'s relay follows the continuation). Not making a relay would change those stop flows, which is a design change, so it was not done.

**The difference in the final lay (moot):** the final lay reads a relay's base from its parent's end at the tree's age. A relay is only ever made once its parent's apex has stopped: `fail`, `stop` and an ended age all set the parent's `apex_end` first. So the parent grows no unit after the relay's birth, and its end at the relay's birth is its end at any later age. With tips no longer rounded, nothing else lifts a parent after a birth either.

**Woken sleepers:** not rounded. They are made in `wake`, not at a lateral's birth in `grow_unit`.

## Four tests run on this build

`round_tests` (both), `keys`, the two shed-certain tests and the extent bound.

### Case 1: the extent bound cannot hold for a lateral whose descendants are lifted above it

`closed_form::kept::tests::bound::no_wood_reaches_past_the_extent_bound` promises that no wood stands farther from a lateral's base than the closed form's extent bound x that base. It fails by more than before:
- **What failed:** axis 202 (PA 10, born 12) reaches 0.024512 against its bound 0.016956, 44.6% past.
- **Its base:** the test computes the base from the axis's vigour, which already includes its own lift. So the axis's own base is the lifted one, as asked.
- **The cause, measured:** a lateral among axis 202's descendants (PA 12) has a vigour of 4.68 relative to its bearer's node.
  - A bud is lifted to its own floor, `GRAIN / Windows::start(its PA)`. Its lift is floor / (its bearer's node faintness x its made presence).
  - Where the bearer's node is itself fainter than the bud's floor, which can hold where the bud's PA has a smaller share (a higher floor) than its bearer's, the bud's vigour comes out over 1.
- **Why this is returned:** the extent bound assumes every element of a subtree stands no larger than its bearer allows. A lifted lateral stands larger than its bearer, so a bound taken from the subtree's root cannot hold, whatever base it uses. Computing it from each lifted base along the subtree would change what the bound is.

### Case 2: the shed-certain shortcut

Both shed-certain tests still fail on the beech at seed 1, now with one marked lateral standing, against 16 with tips rounded:
- `a_lateral_certain_to_be_shed_is_shed`: "1 marked laterals stand";
- `a_tree_grows_the_same_without_what_it_is_certain_to_shed`: "the tree changed".

**Found, measured:**
- **The lateral:** lineage 17490269221437826867, PA 12, born 10, borne at node 9 of axis 14. It is whole in its units (each unit's presence 1.0), its apex ended at 15, it has 20 phytomers and no laterals, and its vigour is 0.0675.
- **At the base** (`d29f29cc5`, no rounding), the same tree at 30 holds no such axis, with the same 1,929 laterals marked.
- **When it is marked:** at its birth, before the rounding: in `grow_unit` the shed-certain check comes before the rule.
- **What the certainty reads:** settings only (`shed::lasting`, its bearer's horizon, the PA's shedding delay). It reads no presence.
- **What the shedding reads:** the fade (`shed::fades`) measures a subtree's lived time in presence-weighted cycles. A child carries its time up to its bearer weighted by its vigour x its bearer's node scale, and a lateral is dropped once its fade reaches nothing.

**Not determined:** which presence the lift changes so that this lateral's fade no longer reaches nothing by 30.
- **Inferred:** its own units are whole, so it is something carried through the presence-weighted times of the lateral itself or of what it stands on.
- **Moving the marking after the rounding** (marking from the lateral as rounded) would not change whether this lateral is marked, since the certainty reads no presence. A kept lateral whose lifted base lengthens its shedding would still be marked "certain".
- **What would be needed:** for the certainty to take the rounding into account, it would have to read the lifted presence the fade reads. That changes what "certain" means: today it is from the settings alone.
