# The gate runs in ten minutes and stays there

## Goal & Context

Owner (2026-10-21), on hearing the gate takes 35 minutes: "test suite takes 35 min?!" and "let's do it now.. how long have we been carrying tis drag..?"

The gate has crept back up every time it was cut:

| Date | Spec | Gate |
|---|---|---|
| 2026-09-18 | fn-76 | target under three minutes |
| 2026-10-13 | fn-271 | 1,525 s cut to 1,128 s; nextest locally ran out of memory |
| 2026-10-14 | fn-274 | 1,551 s at 9 GiB, 4 threads |
| 2026-10-16 | fn-287, AGENTS.md | 2,069 s, at `telperion-heavy 12 --threads 2` |
| 2026-10-21 | F4 at bb20f3877 | about 4.9 min compiling plus 2,494 s of tests, about 46 min (`telperion-heavy 8 --threads 2`) |

Every task on the release runs it once, and this build has spent hours waiting on it. Nothing stops it growing back: no test has a time budget, and no new sweep is weighed against its neighbours.

The slowest binaries on F4 (bb20f3877):

| Binary | Time |
|---|--:|
| a lib unit-test binary, likely `telperion-core` | 530 s |
| `walk_leaves.rs` | 346 s |
| `telperion-render` lib | 282 s |
| `walks.rs` | 276 s |
| a lib, likely `telperion-space` | 185 s |
| `ground_reach.rs` (new in fn-301, 168 builds) | 165 s |
| `boundary.rs` | 88 s |
| `walk_sizes.rs` | 75 s |
| `light_rows.rs` | 52 s |
| `conformance.rs` | 52 s |

## Rulings

Host, 2026-10-21, on R1 (`.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/R1.md`: `cargo test` 2,440 s and nextest with groups 2,270 s at `telperion-heavy 8 --threads 2`; the tests over 3 GB run one at a time in 8 GB and sum to 1,245 s).

**Dispositions.** Each subset is proven by a temporary broken-generator patch that the full sweep catches and the subset also catches (fn-271's method), recorded in RESULT.md. Each full sweep becomes an ignored test whose reason names fn-304.

1. `walk_leaves`: seed 1 runs in the gate; the three-seed sweep is ignored.
2. `ground_reach`: the gate keeps the rows fn-301 R1 found touching (palm at 6–20, birch, spruce at 40, ordinary tree and oak at 40 seed 42) plus every preset at its own age, seed 1; the full sweep is ignored. First find whether its 7.8 GB is accumulation across the loop; if so, hold one tree at a time. No gate test holds more than one tree at once.
3. wasm flowers: the non-flowering presets' empty-slot check folds into `every_shipped_preset_builds_through_the_main_entry`, so no preset is built twice; the flowering presets keep their own check.
4. Device field: one grid size at both seeds in the gate; all grids ignored.
5. `walk_sizes`: the teaser frame runs in the gate; the eight exceptions are an ignored test whose reason names fn-273.
6. `walks`: `every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` merge into one test that grows the walks once, every assertion kept. The trunk-lateral walk is not narrowed.
7. `repeat_tests`: it proves the pipeline's determinism, not each preset's. The gate keeps Telperion (the largest) and the date palm, the two that exercise the most stages; the rest are ignored. Shown red with only those two on a nondeterminism patch (for example a HashMap's iteration order leaking into output).
8. `packed_leaf`: the two tests merge; each mesh is built once and both assertions kept.
9. `every_row_past_its_bound_is_refused_on_every_family`: out of the one-at-a-time group, spread over threads.
10. The per-entry whole-preset tests stay, under fn-271 and fn-274.

**Threads.** Measure the heavy chain inside the 8 GB cap at `--threads 4` and `--threads 8`, under both runners, with peak memory beside wall time. The cap is the owner's desk; threads may rise while memory stays inside it. The declared invocation is the fastest that passes inside 8 GB, and AGENTS.md's gate line is rewritten with its measurement.

**R4 budgets.**
- Per-test own time at most 60 s on the declared invocation, enforced by nextest's slow-timeout with terminate-after in a checked-in profile (or the equivalent if `cargo test` stays the runner).
- Per-test peak memory at most 4 GB, by a cheap mechanism this spec proposes.
- Tests over either budget after this spec sit on a named allowlist in the repository, each with its figure. The allowlist may only shrink; a test that is new or newly over budget fails the check.
- The whole gate at most ten minutes on the declared invocation, reported. If the heavy chain still makes ten minutes impossible inside 8 GB, the best figure goes to the host; no check is loosened.

**Host, 2026-10-22, on tasks .4 and .5.**

- Declared gate: `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, nextest with groups at 4 threads, at the median of its passing runs with the range beside it; 8 GB, not 12. R5's ten minutes is not met, recorded with its arithmetic (the heavy tests' one turn, 661 s + 421 s ≈ 1,080 s); the rest of the gap goes to a follow-up spec the host mints.
- Per-test time: a 120 s wall cut-off; a test whose worst measured time passes 60 s is allowlisted at twice that worst. Memory: 4 GiB, unchanged. The allowlist only shrinks; stale entries fail.
- Total: the test run's total fails past 1.25 × the declared median, rounded up to the minute. It times the test run only: build time follows the cache, not the change (override of codex's task .5 finding). AGENTS.md states the build time beside the median.
- `gate.py` checks for cargo-nextest 0.9.145 first and names `cargo install cargo-nextest --version 0.9.145 --locked` when it is missing or another version.
- The flower case lost to ruling 3 stays lost, named in RESULT.md's Open section.

## Requirements

- **R1 (measure):** per-binary and per-test times on F4. Name what grew since fn-271's 1,128 s, test by test, and the spec that added it.
- **R2 (question, delete, then optimise):**
  - Each slow test either stays in the gate at the smallest size that still catches the defect it names, or its full sweep becomes an ignored test with an ordinary subset in the gate.
  - The subset must be shown red on a temporary broken-generator patch that the full sweep catches (fn-271's method).
  - No assertion is deleted. A sweep that only re-proves what a neighbour proves is named for the host to delete.
  - fn-271's decisions are revisited, not assumed. The full-size walk premise and the separate per-entry builds stand unless the measurement shows a cheaper test that catches the same defect.
- **R3 (run wider):**
  - Measure the gate under nextest with test groups, using fn-274's per-test memory figures for the whole-preset tests, at the widths the desk holds beside the owner's desktop (about 8 GB).
  - The runner that gives the shortest wall time inside 8 GB becomes the declared gate. AGENTS.md's gate line is re-measured and rewritten. This also settles the open 8 GB against 12 GB question.
- **R4 (stays there):** a check that fails when one test's own time passes a stated budget, or when the test run's total passes ten minutes on the declared invocation, so growth is caught by the change that causes it. Name the mechanism: a nextest slow-timeout with terminate-after, a CI step, or a checked-in report. The budget is set by the host from R1's figures.
- **R5:** the target is the whole gate in under 10 minutes on the declared invocation. Test counts change only by named moves to ignored. The gate is green, `npm test` is green, and each task gets a codex review.

## Boundaries

- Test-only. No generator, renderer or preset change.
- CI's own runner is touched only if R4 needs it.
- The broken-generator patches that prove a subset are temporary: never committed, and `git diff --stat crates/*/src` is clean of them before each gate run.
- `telperion-heavy` is the owner's local wrapper, outside the repository; the checked-in pieces are the nextest profile, the budget allowlist and the scripts that read them.

## Strategy Alignment

_No active strategy track served — review for drift._ The gate serves every track equally; this is release housekeeping beside fn-237.

## Decision Context

- **Measured, not assumed (R1).** Under `cargo test` a test's time includes its wait on `one_tree::turn` or `whole_tree`; budgets and dispositions read nextest's per-process times. The tests over 3 GB run one at a time inside 8 GB, so the heavy chain is the floor whatever the runner; the rulings cut that chain, and the threads ruling lets each heavy test use more cores.
- **Where the budgets bind.** The per-test time and memory budgets bind on the declared local invocation, the desk's. CI keeps its own nextest partitions; a checked-in nextest profile carries the budgets, so CI may adopt it, but CI's runner is touched only if R4 needs it (Boundaries).
- **Budgets need a process per test.** nextest gives each test its own process, its own time and its own peak; libtest offers neither without `RUSTC_BOOTSTRAP`, which breaks the trybuild tests (fn-271). If `cargo test` measures faster inside 8 GB, task .4 stops for the host before the budgets are built.
- **The allowlist's first version is this spec's baseline;** from then on the check compares it to the base branch and lets it only shrink.
- **Merges change test counts.** Rulings 3, 6 and 8 merge tests; each merge is a named count change in RESULT.md beside the moves to ignored (R5).
- **Red on F4 before this spec.** `film_walk_tests::the_film_walks_build_on_the_device` and `the_rounds_frames_have_the_cpus_counts` fail on the base (fn-302, fn-303). They are not this spec's to fix; each gate result names them until fn-302 lands.
- **Each merged assertion is shown red on its own patch,** so "every assertion kept" is checked, not claimed. The nondeterminism patch for ruling 7 is deterministic (an injected reorder keyed on the preset), not a random HashMap order that can pass by luck.

## Acceptance Criteria

- **R1:** met; `.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/R1.md`. Errors: none beyond the measurement record.
- **R6 (R2 as ruled):** the ten rulings above, each landed as ruled, with each subset and each merged assertion red on its temporary patch and green on the engine, recorded in RESULT.md; each full sweep is an ignored test whose reason names fn-304 (and fn-273 for the walk-size exceptions) and whose doc comment holds its run command. Errors: a subset its patch does not turn red is not landed and goes to the host; a full sweep its patch does not turn red is named for the host to delete; a render-crate ignore reason must not collide with `docs/gpu-tests.md`'s `device limits:` and `release check:` prefixes.
- **R7 (R3 as ruled):** the shrunk gate measured inside 8 GB at `--threads 2`, `4` and `8`, under `cargo test` and under nextest with groups, wall time and peak memory for each; the fastest that passes inside 8 GB is declared and AGENTS.md's gate line rewritten with its figures. Errors: a run killed at the cap is recorded as killed, never declared; a width that passes only on a quiet desk is reported with the desk's load.
- **R8 (R4 as ruled):** a checked-in nextest profile ends any test past 60 s own time (slow-timeout with terminate-after); a cheap per-test peak-memory check fails any test past 4 GB; tests over either budget on the declared invocation sit on one allowlist in the repository with their measured figures; a check fails when the allowlist gains an entry or raises a figure against the base branch; the test run's total wall time is reported against its budget by the checked-in gate command. Errors: a new test over budget fails by name with its figure and the budget; a renamed or deleted allowlisted test fails as stale; a total over ten minutes is reported to the host with the best figure, no budget loosened.
- **R9 (R5 as ruled):** the gate is green and `npm test` is green; while fn-302 has not landed on F4, its two red device tests are reported by name in each gate result, never ignored or excepted by this spec, every task has a codex review, and test-count changes are named (moves to ignored, merges).

## Early proof point

Task fn-304-the-gate-runs-in-ten-minutes-and-stays.1 validates the core approach (the subset-and-patch method on the core lib's repeat test, the packed_leaf merge and the row-bound spread, the largest share of the core lib's 660 test-seconds). If a subset cannot be shown red where its sweep is, re-evaluate the subset rule with the host before .2+.

## Quick commands

```bash
# One test's own time and peak, as the gate runs it (inside the heavy wrapper)
~/.local/bin/telperion-heavy 8 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-core --lib -E 'test(repeat_tests)'
# An ignored full sweep by name
~/.local/bin/telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --test walk_leaves -- --ignored
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | met; `.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/R1.md`. Errors: none beyond the measurement record. | — | Met before planning: R1.md, commit 96ab20a85 |
| R6 | (R2 as ruled): the ten rulings above, each landed as ruled, with each subset and each merged assertion red on its temporary patch and green on the engine, recorded in RESULT.md; each full sweep is an ignored test whose reason names fn-304 (and fn-273 for the walk-size exceptions) and whose doc comment holds its run command. Errors: a subset its patch does not turn red is not landed and goes to the host; a full sweep its patch does not turn red is named for the host to delete; a render-crate ignore reason must not collide with `docs/gpu-tests.md`'s `device limits:` and `release check:` prefixes. | fn-304-the-gate-runs-in-ten-minutes-and-stays.1, fn-304-the-gate-runs-in-ten-minutes-and-stays.2, fn-304-the-gate-runs-in-ten-minutes-and-stays.3 | — |
| R7 | (R3 as ruled): the shrunk gate measured inside 8 GB at `--threads 2`, `4` and `8`, under `cargo test` and under nextest with groups, wall time and peak memory for each; the fastest that passes inside 8 GB is declared and AGENTS.md's gate line rewritten with its figures. Errors: a run killed at the cap is recorded as killed, never declared; a width that passes only on a quiet desk is reported with the desk's load. | fn-304-the-gate-runs-in-ten-minutes-and-stays.4 | — |
| R8 | (R4 as ruled): a checked-in nextest profile ends any test past 60 s own time (slow-timeout with terminate-after); a cheap per-test peak-memory check fails any test past 4 GB; tests over either budget on the declared invocation sit on one allowlist in the repository with their measured figures; a check fails when the allowlist gains an entry or raises a figure against the base branch; the test run's total wall time is reported against its budget by the checked-in gate command. Errors: a new test over budget fails by name with its figure and the budget; a renamed or deleted allowlisted test fails as stale; a total over ten minutes is reported to the host with the best figure, no budget loosened. | fn-304-the-gate-runs-in-ten-minutes-and-stays.5 | — |
| R9 | (R5 as ruled): the gate is green and `npm test` is green; while fn-302 has not landed on F4, its two red device tests are reported by name in each gate result, never ignored or excepted by this spec, every task has a codex review, and test-count changes are named (moves to ignored, merges). | fn-304-the-gate-runs-in-ten-minutes-and-stays.5 | — |

