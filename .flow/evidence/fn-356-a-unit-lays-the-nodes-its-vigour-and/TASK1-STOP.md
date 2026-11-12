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
