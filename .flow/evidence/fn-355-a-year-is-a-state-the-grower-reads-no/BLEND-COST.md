# The blend's cost after fn-355

Branch `gate/blend-cost`, cut from 34b28aa43. Dispatched by the host on 12 Nov 2026 after `.flow/evidence/fn-354-wood-that-lasts-the-budget-the-girth/GATE-FAILURES.md` item 3 bisected a 15-fold slower blend to fn-355's 4ca638dfd. The fix is 574afa7b4. It leaves every result equal to the bit.

## Numbers first

The seven tests ran alone, each under `telperion-heavy 8 --threads 4`. The base (34b28aa43) and the fix (574afa7b4) ran back to back per test, on 12 Nov 2026 between about 20:10 and 21:00. This was before the yield watcher existed. These are nextest's wall seconds; the load average at each start was 24 to 63. Every test passed on both commits, with its assertions and digests unchanged.

| Test (wall s) | Base | Fix | GATE-FAILURES, b2b45f3c6 (before fn-355) |
|---|---:|---:|---:|
| `suite::sweep::every_pair_of_presets_grows_a_tree_at_every_step` | 322.5 | 95.3 | 43.9 |
| `suite::sweep::every_wire_parameter_a_preset_moves_is_walked` | 321.1 | 67.6 | 32.2 |
| `catalogue_identity every_walk_is_the_walk_it_was` | 224.4 | 29.4 | 19.3 |
| `space_walks a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80` | 431.2 | 67.2 | 47.9 |
| `space_walks the_engines_walk_converges_on_either_end_at_40_and_80` | 303.0 | 23.6 | 19.7 |
| `space_walks the_grown_count_binds_where_a_blend_sheds_more_than_its_ends` | 151.6 | 27.8 | 24.0 |
| `walk_leaves birch_to_oak_changes_its_leaves_by_degree` | 118.0 | 41.3 | 44.4 |

All seven are now under the gate's 120 s cut-off. The sweeps are still 2 to 2.2 times their time before fn-355; the rest are within 1.0 to 1.5 times.

**One oak-to-beech blend at 0.5, and one small tree built from it**, using the scratch example below. Runs were back to back; wall seconds as the example timed them; load 78 to 93:

| Commit | Blend (s) | Build (s) |
|---|---:|---:|
| b2b45f3c6, before fn-355 | 0.77 | 0.11 |
| 4ca638dfd, fn-355 | 9.72 | 0.21 |
| 34b28aa43, base | 12.71 (earlier window, load 50) | 0.38 |
| 574afa7b4, fix | 1.53 | 0.13 |

CPU time (user+sys), which is the comparison to read: two rounds back to back on 13 Nov 2026 between 00:44 and 00:48, at load 2 to 9, through `TELPERION_HEAVY_RESERVE=8 telperion-heavy 6 --threads 4`. One base run's wall time takes in a watcher PAUSE, but its CPU time is unaffected:

| Commit | Blend alone, CPU s | Blend and build, CPU s |
|---|---:|---:|
| b2b45f3c6, before fn-355 | 0.40 / 0.43 | 0.47 / 0.50 |
| 4ca638dfd, fn-355 | 6.00 / 7.11 | 6.39 / 6.91 |
| 34b28aa43, base | 5.74 / 6.95 | 6.88 / 7.23 |
| 574afa7b4, fix | 1.47 / 1.43 | 1.60 / 1.62 |

The fix cuts the blend's CPU time by 4.3 times. What is left is 3.5 times its time before fn-355.

## Where the time went

`perf` is not installed, and `samply` cannot map its buffers (FRICTION.md). So the stages were timed with counters, temporary and reverted.

- **Building the expected-length table is all of the blend.** One blend at 40 built 18 tables, 7.37 s of the run's 7.38 s. The kept predictor's 9 calls took 0.11 s, and the grown walk's 9 took 0.04 s.
- **Built where:** `presence::Windows::new` fills `closed_form::expected_log_lengths(species, rows)`. Its callers are `expected_kept`, `expected_grown` and `expected_counts` (`closed_form.rs`), plus the grower (`grow.rs`). The blend's size solve (`telperion-core/src/blend/size.rs`, `sized` then `factor` then `solve`) calls `expected_kept` and then `expected_grown` once per step, on a species whose laterals are scaled by that step's factor. That made 9 kept and 9 grown steps here, each building a new table on a new species. A walk's ends are cached (`ENDS`), so a pure species pays once per walk and a blend once per solve step. A built tree pays twice: once in `pipeline::space::foreseen` (`expected_kept`) and once in the grower.
- **Size.** The table has `year + LOOK + slept + 2` rows: 98 for the oak at 40, where `slept`, the sleepers' longest stage chain, is 16. Before fn-355 it had `age` rows, 40. Every PA was filled at every row, 15 PAs × 99 rows. The windows read only row `LOOK` for every PA, and PA 0 from row `LOOK - 1` on.
- **Why a blend's table costs more than its ends'.** Each of the 98-row tables below was built alone:

  | Species | Memo entries | Sleeping-bud events | Table (s) |
  |---|---:|---:|---:|
  | oak | 13,527 | 2,595 | 0.03 |
  | beech (82 rows, no sleeper) | 14,726 | 465 | 0.03 |
  | oak-beech at 0.5, one solve step | 55,662 | 2,085,030 | 0.76 |

  In the blend, 2.79 M bud events come from 55,662 substructures. Most of them are sleeping buds: `closed_form/events.rs::sleeping` emits one per bearer, per year of sleep, per stage it may wake in. Each one recurses on a woken bud keyed by its sleep and first-unit law. The time per substructure is about 25 times the oak's. Why the blend has 800 times the oak's sleeping events was not split further: it is unknown which blended rows (zones, dormancy, release delay, lifespans) open them.

## What was fixed (574afa7b4)

1. `closed_form::log_lengths(species, asked)` walks only the `(pa, m)` entries asked. Each entry is a function of its memo key alone, so it is the same to the bit whatever else is asked. `Windows` now holds `look[pa]` and `whole[c]` and asks only for those. `expected_length` asks for one entry. `expected_log_lengths` stays for the tests, built on `log_lengths`. A probe compared both ways on the blend's and the ends' tables: memo 28,418 against 55,662 entries, 0.28 s against 0.80 s, and every read entry equal to the bit.
2. `Windows::drawn(species, year)` serves the closed form without the sleepers' `slept` rows. Only the grower's `sleeping` reads those rows, and the grower keeps `Windows::new`. At load 94 this took the blend from 4.6 s to 2.8 s. A new test, `presence::windows::tests::the_drawn_windows_are_the_growers_without_the_sleepers`, holds both windows' woods equal to the bit on the oak and the spruce. `sleeping` on a drawn window panics, since its stage table is empty.

`whole[0]` is row `LOOK - 1`. The old index `cycle - 1 + LOOK` wrapped there in the release-derived profiles.

**Proof of equality.** `telperion-space --lib` passed in full: 129 tests, among them every `paused` test (prefix, year, let-go, retired root, horizon), the closed form's pins and `the_rows_hold_their_values`. The seven tests above passed unchanged. The core's narrow suites passed: 47 of 47 tests, with no pause in the run, on 13 Nov 2026 at 00:50. They cover `cycle_cost_tests::` (the one-cycle digest pins), every `pipeline::space::` test (repeat, growth year by year, ceiling, relay), `suite::species::` (the species digests), and all of `space_presets` and `space_walks`.

A first run of that suite was cut short by SIGTERM when the machine's rules changed. In it, `repeat_tests::the_largest_tree_and_the_palm...` printed "a NaN in the tree" before its abort. Run alone afterwards, it passed on the fix (CPU 128.6 s; wall 614 s across a 17-minute watcher pause) and on the base (CPU 162.5 s, wall 111 s). It passed again in the full rerun. The NaN was not reproduced, and its cause is unknown. It is flagged for the host.

## What remains: the host's

What remains is the design's. The windows need PA 0's expected length out to `year + LOOK`, and the grower needs `slept` more rows past that. The blend solve builds one such table per step, about 18 per blend.

Here is the same blend with the fix at three looks. Each `LOOK` was set by a scratch edit in a throwaway worktree (`looks`, reverted there, never committed). The runs were back to back, two rounds, at load 22 to 28; wall seconds as the example timed them:

| LOOK | Blend (s) | Build (s) | Wood vertices |
|---:|---:|---:|---:|
| 10 | 0.39 / 0.44 | 0.07 / 0.10 | 338,198 |
| 20 | 0.68 / 0.73 | 0.08 / 0.08 | 418,726 |
| 40 | 1.86 / 1.74 | 0.14 / 0.14 | 439,530 |

CPU time (user+sys) at each look, from the same window as the table above:

| LOOK | Blend alone, CPU s | Blend and build, CPU s |
|---:|---:|---:|
| 10 | 0.36 / 0.36 | 0.44 / 0.46 |
| 20 | 0.59 / 0.59 | 0.68 / 0.70 |
| 40 | 1.47 / 1.43 | 1.60 / 1.62 |

Before fn-355, the blend alone took 0.40 to 0.43 CPU s.

For the decision:

- The blend's cost grows about as `(year + LOOK)^2` to `^2.5`, through its sleeping-bud substructures. In CPU time at 40, it is 3.5 times the cost before fn-355. At 20 it is 1.4 times that cost, and at 10 it is about equal.
- The tree changes with the look (wood vertices above). Choosing the look is the host's.
- Two remaining costs could be cut without changing a result. Neither was built, because each is a judgment about the structure:
  - (a) Share one table across a solve's kept and grown steps when they land on the same factor. This saves at most one table per solve.
  - (b) Make `Windows` extend its column lazily rather than to `year + LOOK + slept` up front. The grower shares it across threads behind an `Arc`.

## Commands

- Scratch example (untracked): `crates/telperion-core/examples/blend_cost.rs`. It runs `blend::families(oak, beech, 0.5)`, then `mesh::build` with `max_nodes = 8000` and age at most 12, as the sweep does. Its source is in the dispatching session's scratchpad, `blend_cost.rs`.
- Timing: `telperion-heavy 6 --threads 4 -- target/ci/examples/blend_cost both` in a worktree of its own per commit (`git worktree add --detach`), each with its own `target/`.
- Seven tests: `telperion-heavy 8 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-core <--lib|--test X> -E 'test(=<name>)'`, base and fix in turn per test.
- Narrow suites: `telperion-heavy 8 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space --lib`. Then the core's `cycle_cost_tests`, `pipeline::space::`, `suite::species::` with `space_presets` and `space_walks`.
- Raw logs: the dispatching session's scratchpad, `bc/`.
