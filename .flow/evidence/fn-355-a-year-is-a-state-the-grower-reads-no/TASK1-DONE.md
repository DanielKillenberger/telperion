# fn-355 task 1: a year is a state, built; stopped at a walk's vanishing bound

The host's corrected design is built in `4ca638dfd`. R1 and R2 hold to the bit, the identity test (R3) is green unchanged, and every walk holds its degree bound. One walk's made branch is 1.156e-6 of the tree at its crossing, against `VANISHING` 1e-6: `states[1].abortion_rise`, seed 3. That is the spec's stop "a walk failing its bound". The other stops are read below, and one of them, sleepers on the oak at 120, needs the host's reading. The task stays `in_progress`.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing with fn-354)

Baseline: green via handoff (narrow space and core 765 of 766 at `d0728b827`, the identity test the one red; vitest 218).

## The first stop and the host's answers

The first stop, at the table's cost, is recorded in `9c4602d7d`:
- **What stopped it:** a look by the whole lifespan needed the expected-length table 500 rows past the year on the oak, about 1.4 s on the oak at 40.
- **The table check:** rows filled to 40 equal rows filled to 120 to the bit, on five species (`an_expected_length_does_not_depend_on_how_far_the_table_runs`, green).

The host's answers (12 Nov, the spec's "Corrected by the host" section) are built here:
- the look is bounded by `LOOK` = 40;
- the table runs to the year grown plus `LOOK`;
- a sleeper is recorded when it can wake.

## What was built, per site of P4-SURVEY.md

- **G1, G2: the windows.** They live in `presence/windows.rs`, out of `presence.rs`.
  - **The look:** a draw of PA k, its axis `spent` of the way through k, looks over `min(life_k − spent, LOOK)` cycles. Its wood is the table's row at that look, and the whole is the seed's row at `cycle − 1 + look`.
  - **A fractional look** is read between two rows: by their logs, or linearly where either row is none.
  - **The contract's f32 path is unchanged.** The share is `exp(log32(log − whole))`, and `fresh[c][k]` is precomputed for a bud drawn at the start of its PA.
  - **`LOOK`** sits beside `RATE`, `SPAN` and `FLOOR`. Its comment names it the host's estimate and gives the spec's reason, and it is exported.
- **What each draw passes as spent:**

  | Draw | Spent |
  |---|---|
  | A lateral, a move on (`decide::moved`), a sleeping bud's wood | 0 |
  | A unit's survival | the apex's spent before the unit |
  | Abortion, at cycle + 1 | the spent after the unit (`apex.spent + lived` on the failed path) |
  | A relay | the spent the stop carries |
  | A node's internode share | the bearer's spent |
  | The balance's shed draw | the living apex's spent, else the axis's spent when it stopped (`Grower::units`) |

- **The table's extent:** `Windows::new(species, year)` fills the table to `year + LOOK + the longest sleep + 2`.
  - "The longest sleep" counts only ages some zone puts to sleep under a release law that wakes them.
  - A growth that goes past the year rebuilds its windows to `max(cycle, 2 × year)` (`Grower::year`).
  - Entries do not depend on the extent, so a rebuild changes no draw.
- **The closed form** (`dormant/living.rs`): `Living::borderline` reads the grower's move-on window at the cycle `windows.year() − left`. Its windows are built for the tree's age, so `year()` is that age.
- **G3: a sleeping bud's wood** is summed over `s < ceil(last_stage(pa))`, the years before it ages past its last stage (`dormant::last_stage`).
- **G4, G5: recording sleepers.**
  - `dormant::draw` records a sleeper when `aged(species, pa, whole + (years − whole))` is some. That is the same test `Grower::wake` makes, on the same float.
  - A sleeper that can never wake is not recorded.
  - `asleep` starts empty and grows as sleepers are absorbed (`parallel.rs`).
- **G6:** every paused year is laid out and lit (`Paused::cycles` passes `sketching = true`).
- **G8:**
  - `Paused::grow_to` refuses nothing ahead.
  - `admitted(committed, age)` drops its horizon.
  - `horizon()` stays, documented as the age first asked, which bounds nothing. The browser's grow-in reads it.
- **G9:** `pipeline::growth::grow_to` judges `foreseen` at the year it grows to, past the first request too.
- **Docs:** `docs/pipeline.md` (the growth bullet, and a paragraph under "The one cycle and its grower") and `docs/tree-space.md` (a line under the claims) say a year is a state.

## Judgments I made

- **Spent for the balance's shed draw** is per axis: the live apex's, else `units[i]`. It costs one copy of `units` per cycle.
- **A relay at an age's end** looks over 0 cycles, since nothing is left of that lifespan. Its window is then the floor, as a horizon year's was.
- **The sleep sum** runs to `ceil` of the stage total, not a cut part-year.
- **The table extent** includes the longest sleep (16 years on the oak, 50 on the spruce, 12 on the birch). The host said "year plus LOOK"; the sleep sum reads that far.
- **Windows are rebuilt by doubling,** so a year-by-year growth rebuilds them a logarithmic number of times.
- **The shed-certain lever** (`Shoot::shed_certain`) still reads `rules.age`. It runs only in the test-only whole-tree `run`, since the paused grower sets `lasting` to none, and by its own contract it leaves the tree the same.
- **`Rules::age`** is otherwise read only by the settle, which sets it to the age grown.

## Tests

- **R1** (`a_year_is_a_state_not_a_horizon`): red at `8bf6175e7`, green now. The oak, spruce and beech at 14 of 15 and 20 of 28 are equal to the bit.
- **R2:**
  - `a_growth_goes_on_past_its_first_request`: the oak, spruce and beech begun for 12 and grown on to 21 equal `grow_cycle` at 21. Green.
  - Core's refusal test now grows past its first age to 12.5.
- **R3** (`a_node_keeps_its_identity_across_ages`): green, unchanged.
- **The narrow space and core suites**, once (the long walk and the largest tree excluded, as in fn-354): 768 of 769 in 5,193 s, at load 40 to 57.
  - The one failure was the one-cycle pin, repinned below.
  - `every_preset_is_the_family_it_was` and `every_walk_is_the_walk_it_was` held.
- **The walks binary**, `--nocapture`, at fn-354's head (`raw/walks-before.log`) and now (`raw/walks-year.log`):
  - At the head: 11 passed.
  - Now: 10 passed, and the long walk failed as described under "The stop".
- **Also run:**
  - the repinned test, green;
  - `generation_limit_guard`, green;
  - `cargo fmt --all --check`, green.
- **Not run:**
  - `compile_size` (render crate, untouched);
  - vitest and the Wasm parity tests;
  - the full gate.

### The stop: R2's vanishing size on one walk

`every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size` judges two things:
- **R4's degree bound:** every walk holds it, at 147 walks.
- **The vanishing size:** the R2 check then fails on one walk.

```
states[1].abortion_rise seed 3 at 2.3250: 0.000001155804534200751 of the tree
```

The bound is `VANISHING` = 1e-6. Before, the walk's steepest step was seed 1 at 2.4300 (3.80). It is now seed 3 at 2.3250 (5.22). I have not changed the bound or the test.

### R4: the steepest steps

Each walk's steepest step, before and after, is in `year/walks-steepest.tsv`. Every one is under its bound. Of 147 walks, 26 moved by more than 0.5 per unit. The largest rises:

| Walk | fn-354 head | Now |
|---|---|---|
| `states[1].form.sapwood, sag 3e-5` | 4.41 | 5.90 |
| `states[1].abortion_rise` | 3.80 | 5.22 |
| `light: states[2].tolerance` | 6.21 | 7.45 |
| `light: states[2].apical_control` | 1.97 | 3.16 |
| `states[0].zones[0].nodes.mean` | 19.39 | 20.62 |
| `light: sky` | 9.59 | 10.63 |
| `light: states[0].apical_control` | 27.82 | 28.97 |
| beech to spruce / spruce to oak (species walks, not held to the bound) | 230.8 / 231.7 | 232.1 / 233.1 |

## R5: measured

Seeds 1001, 1007 and 1012. `fn355-before` is fn-354's head (`d0728b827`, its own worktree and target). `fn355-year` is `4ca638dfd`. Both used a copy of the scoreboard runner in `raw/runner/`, whose probe adds three measures:
- **Summed length:** each phytomer's tip from the one before, in drawn metres.
- **Mean presence:** the mean of `Phytomer::scale`, weighted by that length. It can exceed 1, since `scale` is relative to its axis's base.
- **Sleepers:** the grower's `asleep.each` records at the end, from a paused growth equal to `grow`.

Every measure is in `year/VARIANTS.md`; the run records are in `year/runs/`.

| Tree | Summed length, m (head → now) | Ratio | Mean presence | Sleepers at the end |
|---|---|---|---|---|
| Oak 40 | 2,864 / 4,423 / 3,118 → 2,681 / 4,337 / 3,039 | 0.936 / 0.980 / 0.974 | 1.223 / 1.322 / 1.239 → 1.192 / 1.341 / 1.233 | 0 → 8 / 23 / 12 |
| Oak 120 | 20,540 / 32,752 / 24,316 → 20,358 / 30,315 / 24,437 | 0.991 / 0.926 / 1.005 | 1.067 / 1.232 / 1.201 → 1.087 / 1.273 / 1.197 | 0 → 41 / 92 / 54 |
| Beech 40 | 1,503 / 1,872 / 2,322 → 1,442 / 1,688 / 2,294 | 0.960 / 0.902 / 0.988 | 0.868 / 0.883 / 0.860 → 0.854 / 0.863 / 0.832 | 0 → 0 |
| Birch 40 | 5,157 / 3,972 / 5,414 → 5,148 / 3,963 / 5,379 | 0.998 / 0.998 / 0.994 | 0.773 / 0.775 / 0.766 → 0.773 / 0.772 / 0.762 | 0 → 764 / 592 / 758 |
| Spruce 40 | 14,210 / 13,482 / 14,772 → 13,753 / 12,893 / 14,634 | 0.968 / 0.956 / 0.991 | 0.709 / 0.724 / 0.736 → 0.698 / 0.704 / 0.734 | 0 → 2,423 / 2,213 / 2,451 |
| Telperion 82 | 23,191 / 16,657 / 17,589 → 23,194 / 17,086 / 17,109 | 1.000 / 1.026 / 0.973 | 0.878 / 0.870 / 0.949 → 0.867 / 0.874 / 0.901 | 0 → 0 |

- **The largest fall in summed length** is the beech at seed 1007, 9.8%, inside the tenth.
- **Height, trunk and new shoots:**
  - height moves by under 0.7 m;
  - trunk by under 3 cm, except Telperion at seed 1012, 106.1 to 101.0 cm;
  - new shoots on the oak at 120 fall by 2 to 10%;
  - Telperion's rise by 1 to 13%.

**Cost,** one thread, seed 1001, peak memory:

| Tree | Head, s | Head, GiB | Now, s | Now, GiB |
|---|---|---|---|---|
| Oak 40 | 6.1 | 0.45 | 2.7 | 0.44 |
| Oak 120 | 179.3 | 3.48 | 77.9 | 3.29 |
| Beech 40 | 5.7 | 0.20 | 1.3 | 0.20 |
| Birch 40 | 18.7 | 0.48 | 3.4 | 0.47 |
| Spruce 40 | 100.1 | 1.76 | 15.0 | 1.73 |
| Telperion 82 | 176.3 | 2.35 | 24.5 | 2.46 |

- **The seconds are not comparable.** The head's cost ran at load 50 to 88, now's at 17 to 31 (RUN.md in each). The memory is comparable.
- **Peak memory on the oak at 120** fell 5%.
- **The windows' cost** at their extent, one thread, in a test at load about 47: oak at 40 0.023 s, oak at 120 0.072 s, spruce at 40 0.039 s, spruce at 120 0.067 s, beech 0.021 s, birch 0.017 s. That is at most about 1% of each tree.
  - A first build of the extent counted every age's stage chain, not only sleeping ages: 1.9 s on the oak at 40. It was corrected before the commit (FRICTION.md).

**Sheets** (whole and bare, columns fn-354 head and now) are in `year/`, for the oak at 40 and 120, the beech, birch, spruce and Telperion. Descriptions, not verdicts:
- **The oak:** the same trees in form. At 40, seed 1001, the crown is a little more open and wider (8.7 against 7.5 m). At 120 the crowns match, seed 1007 a little wider.
- **The beech:** the same trees. The crown is a little sparser at seed 1007 (its length fell 10%), and seed 1001 is a little wider.

## For the host

1. **The vanishing bound** (the stop). `states[1].abortion_rise`, seed 3, makes a branch of 1.156e-6 of the tree at its crossing, against `VANISHING` 1e-6, 16% over. The degree bound holds there (5.22 per unit). I have not traced which window makes the branch larger at its crossing.
2. **Sleepers held on the oak at 120:** 0 before, 41 / 92 / 54 now. A growth to its horizon held none at the end, because every sleeper due after the horizon was dropped. Now every sleeper that can wake is held. The stop reads "more than a fifth" above a base of 0, so I name it rather than judge it. Peak memory on that tree fell 5%. The spruce holds about 2,400 at 40 and the birch about 700.
3. **What is unchanged:** the other stops are not reached.
   - Summed length falls at most 9.8% (beech, seed 1007).
   - Peak memory on the oak at 120 is down.
   - No question on closure or retirement came up beyond what the host decided.

## Repinned

| Pin | Before | After |
|---|---|---|
| One cycle at 16, ordinary (finest, coarsest) | (1233340031879171559, 1233340031879171559) | (14187053998053303148, 14187053998053303148) |
| Oak | (17687829558383956639, 11667850423448962601) | (348693234653869032, 16728606230453458813) |
| Spruce | (12287197284966836912, 1773060326561079923) | (14706478347833265052, 10913663721868558112) |
| Beech | (17769414538517393552, 17769414538517393552) | (4636435621290452984, 4636435621290452984) |
| Birch | (3184370250942454689, 16241647282371321572) | (9307076759512720806, 9885985573802697693) |
| Palm | unchanged | (4799502948138528708, 4799502948138528708) |
| Telperion | (12061773189002990059, 12061773189002990059) | (1077519123738398758, 1077519123738398758) |
| Laurelin | (17749478251659094029, 17749478251659094029) | (16072149993745548792, 16072149993745548792) |

`every_preset_is_the_family_it_was`, `every_walk_is_the_walk_it_was` and `every_override_reads_and_writes_as_it_did` did not move.
