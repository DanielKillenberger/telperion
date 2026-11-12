# fn-355 task 1: a year is a state, built

The grower no longer reads the requested age. Every draw's window looks `LOOK` = 40 cycles on, and a growth goes on past its first request. R1, R2 and R3 hold to the bit. Every walk holds its degree bound, and the vanishing check now passes as well. Summed length at the asked age is within 6.2% of fn-354's head on every tree. Built in `4ca638dfd` and `958e0b28c` to the spec's design as corrected twice by the host.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing with fn-354)

Baseline: green via handoff (narrow space and core 765 of 766 at `d0728b827`, the identity test the one red; vitest 218).

## The two stops and the host's answers

**First stop** (`9c4602d7d`): a look by the whole lifespan needed the expected-length table 500 rows past the year on the oak, 1.4 s on the oak at 40.
- The host bounded the look by `LOOK`.
- A sleeper is recorded only when it can wake.

**Second stop** (`27a6be172`): with the look at `min(lifespan left, LOOK)`, one walk's branch entered at 1.156e-6 of the tree against `VANISHING` 1e-6.
- The host made the look `LOOK` for every draw, its own error in the spec.
- The host accepted the sleepers held, 41 to 92 on the oak at 120 over a base of none.

## What was built, per site of P4-SURVEY.md

**G1, G2: the windows** (`presence/windows.rs`):
- **What a draw of PA k in cycle c takes:**
  - its wood is the table's row `LOOK` of k: what a bud of k grows in 40 cycles, its own stages and those it goes on into;
  - its whole is the seed's row at `c − 1 + LOOK`.
- **Shares from the start of a cycle** are taken once a table into `woods[c][k]`, by the contract's f32 path.
- **A draw `used` of the way through its cycle** reads between the cycle's log and the next's, as before (`presence::wood_at`).
- **`LOOK`** is a contract constant beside `RATE`, `SPAN` and `FLOOR`. Its comment gives the host's estimate and reason, and it is exported.
- **No draw passes its spent time,** and there is no fractional row read. The call sites are back to their base form, apart from `share`, which takes its cycle.

**The table:**
- **Extent:** `Windows::new(species, year)` fills it to `year + LOOK + the longest sleep + 2`, the longest sleep counted over ages some zone puts to sleep under a release law that wakes them.
- **Growth going on:** past `year`, the grower rebuilds its windows to `max(cycle, 2 × year)` (`Grower::year`).
- **Extent-independence:** an entry is the same however far the table runs, to the bit, on five species (`an_expected_length_does_not_depend_on_how_far_the_table_runs`).
- **Cost** at the extent, one thread, load about 47:

  | Tree | Windows, s | Share of the tree |
  |---|---|---|
  | Oak 40 | 0.023 | under 1% |
  | Oak 120 | 0.072 | under 1% |
  | Spruce 40 | 0.039 | under 1% |
  | Spruce 120 | 0.067 | under 1% |
  | Beech 40 | 0.021 | under 1% |
  | Birch 40 | 0.017 | under 1% |

  These were measured on the first build. The second fills the same extent and reads one fixed row, so it costs no more.

**The closed form:** `Living::borderline` reads the grower's move-on window at the cycle `windows.year() − left`. The closed form builds its windows for the tree's age, so `year()` is that age.

**G3:** a sleeping bud's expected wood is summed over `s < ceil(last_stage(pa))`, the years before it ages past its last stage (`dormant::last_stage`).

**G4, G5:**
- `dormant::draw` records a sleeper when `aged(species, pa, whole + (years − whole))` is some. That is the waking's own test, on the same float.
- One that can never wake is not recorded and holds nothing open.
- `asleep` starts empty and grows as sleepers are absorbed.

**G6:** every paused year is laid out and lit.

**G8:** growth past the first request is open.
- `Paused::grow_to` refuses nothing ahead.
- `admitted(committed, age)` drops its horizon.
- `horizon()` stays as the age first asked, which bounds nothing. The browser's grow-in reads it.

**G9:** `pipeline::growth::grow_to` judges the kept ceiling at the year it grows to.

**Docs:** `docs/pipeline.md` (the growth bullet and a paragraph under "The one cycle and its grower") and `docs/tree-space.md` (a line under the claims) say a year is a state.

## Judgments

- **The test-only reference path:** `Shoot::shed_certain` still reads `rules.age`. It runs only in the whole-tree `run`, which is test-only; the paused grower sets `lasting` to none.
  - R1's structural test covers the product path. It compares `grow_cycle`, which `grow` and the pipeline call, with a paused growth begun for a later age.
  - The host accepted this as it stands.
- **`Rules::age`** is otherwise read only by the settle, which sets it to the age grown.
- **The table extent** includes the longest sleep: 16 years on the oak, 50 on the spruce, 12 on the birch.
- **Windows are rebuilt by doubling** the year as a growth goes on.

## Tests

| Test | Result |
|---|---|
| R1 `a_year_is_a_state_not_a_horizon` (oak, spruce, beech at 14 of 15 and 20 of 28) | red at `8bf6175e7`, green |
| R2 `a_growth_goes_on_past_its_first_request` (begun for 12, grown on to 21, equal to the tree asked at 21) | green |
| R2 in core: the refusal test grows past its first age to 12.5 | green |
| R3 `a_node_keeps_its_identity_across_ages` | green, unchanged |
| Narrow space and core, once (`raw/narrow2.log`; the long walk and the largest tree excluded, as in fn-354) | 768 of 769 in 1,470 s; the one failure was the one-cycle pin, repinned and rerun green |
| The walks binary with `--nocapture`, the long walk in it | 11 passed, 1 ignored (the slow in-leaf suite) |
| `generation_limit_guard`, `compile_size` | green |
| `cargo fmt --all --check` | green |
| `npm run wasm:build`, then `npm test` (catalogue check and vitest) | vitest 218 of 218 |
| `every_preset_is_the_family_it_was`, `every_walk_is_the_walk_it_was`, `every_override_reads_and_writes_as_it_did` | held |

**The narrow command:** `telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size) & not test(the_largest_tree_and_the_palm)'`.

**The walks command:** `telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --test walks -- --nocapture`. It also ran at fn-354's head (`raw/walks-before.log`).

### R4: the walks

Every walk holds its degree bound, and `made_and_unmade_at_vanishing_size` passes. The steepest step of `states[1].abortion_rise` is now 4.99, seed 3 at 2.2950. All 147 steepest steps, before and after, are in `year2/walks-steepest.tsv`. Of these, 21 moved by more than 0.5 per unit. The largest rises:

| Walk | fn-354 head | Now |
|---|---|---|
| `states[0].zones[1].nodes.min` | 6.30 | 9.47 |
| `in leaf: states[1].form.sag` | 0.61 | 3.63 |
| `states[1].form.sapwood, sag 3e-5` | 4.41 | 7.12 |
| `in leaf: states[1].viability` | 3.10 | 4.50 |
| `states[1].abortion_rise` | 3.80 | 4.99 |
| `sleeping: states[1].viability` | 3.03 | 4.09 |
| `light: states[1].balance_hazard` | 1.83 | 2.84 |
| `light: states[0].apical_control` | 27.82 | 17.79 (down) |
| Species walks, not held to the bound: oak to palm, spruce to oak, beech to spruce | 80.6, 231.7, 230.8 | 93.1, 241.8, 240.1 |

## R5: measured

**Runs:**
- **Seeds:** 1001, 1007 and 1012.
- **Labels:**
  - `fn355-before` is fn-354's head, `d0728b827`, built in the `fn-355-base` worktree with its own target;
  - `fn355-year2` is this build, `958e0b28c`;
  - `fn355-year` is the first build, kept for comparison.
- **The runner:** a copy of the scoreboard's in `raw/runner/`. Its probe adds three measures:
  - **Summed length:** each phytomer's tip from the one before, in drawn metres.
  - **Mean presence:** the mean of `Phytomer::scale`, weighted by that length. It can exceed 1, since `scale` is relative to its axis's base.
  - **Sleepers:** the `asleep.each` records at the end, from a paused growth equal to `grow`.
- **Records:** every measure is in `year2/VARIANTS.md` (three columns), and the run records are in `year2/runs/` and `year/runs/`.

| Tree | Summed length, m (head → now) | Ratio | Mean presence (head → now) | Sleepers at the end |
|---|---|---|---|---|
| Oak 40 | 2,864 / 4,423 / 3,118 → 2,685 / 4,338 / 3,059 | 0.938 / 0.981 / 0.981 | 1.223 / 1.322 / 1.239 → 1.195 / 1.316 / 1.236 | 0 → 8 / 23 / 12 |
| Oak 120 | 20,540 / 32,752 / 24,316 → 20,884 / 31,439 / 24,398 | 1.017 / 0.960 / 1.003 | 1.067 / 1.232 / 1.201 → 1.090 / 1.225 / 1.196 | 0 → 41 / 79 / 54 |
| Beech 40 | 1,503 / 1,872 / 2,322 → 1,538 / 1,841 / 2,455 | 1.023 / 0.984 / 1.057 | 0.868 / 0.883 / 0.860 → 0.878 / 0.918 / 0.836 | 0 → 0 |
| Birch 40 | 5,157 / 3,972 / 5,414 → 5,178 / 3,841 / 5,357 | 1.004 / 0.967 / 0.989 | 0.773 / 0.775 / 0.766 → 0.770 / 0.762 / 0.769 | 0 → 764 / 594 / 758 |
| Spruce 40 | 14,210 / 13,482 / 14,772 → 14,227 / 13,482 / 14,784 | 1.001 / 1.000 / 1.001 | 0.709 / 0.724 / 0.736 → 0.709 / 0.724 / 0.736 | 0 → 2,423 / 2,225 / 2,451 |
| Telperion 82 | 23,191 / 16,657 / 17,589 → 22,998 / 16,769 / 17,117 | 0.992 / 1.007 / 0.973 | 0.878 / 0.870 / 0.949 → 0.873 / 0.880 / 0.904 | 0 → 0 |

- **Summed length** falls 6.2% at most (the oak at 40, seed 1001). The first build's largest fall, 9.8% on the beech at seed 1007, is now 1.6%.
- **The spruce** is within 0.1% of the head on length and presence.
- **Against the first build (`fn355-year`),** some measures move more than 3%:
  - the beech's summed length rises 7 to 9%;
  - the oak at 120, seed 1001, is 6.4% taller;
  - spread on single seeds moves by up to 32%: the birch at seed 1007, with its x and z spreads swapped against the first build.
- **The sheets** (`year2/`, whole and bare, columns fn-354 head, fn355-year and fn355-year2) cover the oak at 40 and 120, the beech, birch, spruce and Telperion. Descriptions, not verdicts:
  - **The beech:** the same trees in all three columns. At seed 1007 the second build's crown is broader than the first build's, near the head's.
  - **The birch:** the same trees. Seed 1007's crown leans differently between the columns, which is the swapped spread.

**Cost,** one thread, seed 1001. Both sides ran alternately in one window (`year2/runs/cost-paired.jsonl`): base then now, per tree.

| Tree | Head, s | Head, GiB | Now, s | Now, GiB | Load during the pair |
|---|---|---|---|---|---|
| Oak 40 | 3.1 | 0.45 | 3.4 | 0.44 | 45 |
| Oak 120 | 79.9 | 3.47 | 60.3 | 3.39 | 43 then 32 |
| Spruce 40 | 11.8 | 1.76 | 11.7 | 1.72 | 20 then 16 |

The oak at 120's pair straddled a fall in load, so its 25% is not a measured gain. Peak memory is down on all three.

## Repinned

| Pin | fn-354 head | After |
|---|---|---|
| One cycle at 16, ordinary (finest, coarsest) | (1233340031879171559, 1233340031879171559) | (3477975257458263232, 3477975257458263232) |
| Oak | (17687829558383956639, 11667850423448962601) | (15182297322291910030, 7299178540230679106) |
| Spruce | (12287197284966836912, 1773060326561079923) | (12665486350558508414, 9318247602647309830) |
| Beech | (17769414538517393552, 17769414538517393552) | (15521184810353786578, 15521184810353786578) |
| Birch | (3184370250942454689, 16241647282371321572) | (2054489571488262802, 18014430067925384448) |
| Palm | (4799502948138528708, 4799502948138528708) | unchanged |
| Telperion | (12061773189002990059, 12061773189002990059) | (6841973736915866750, 6841973736915866750) |
| Laurelin | (17749478251659094029, 17749478251659094029) | (12434381965102712846, 12434381965102712846) |

The first build's pins (`4ca638dfd`) were an intermediate state and are superseded. `every_preset_is_the_family_it_was`, `every_walk_is_the_walk_it_was` and `every_override_reads_and_writes_as_it_did` did not move.

## Left as it is

- **The `fn-355-base` worktree** (fn-354's head, detached, its own target) is still there for the host's comparisons.
- **The first build's evidence** (`year/`) is kept beside `year2/` as the record of the second stop.
