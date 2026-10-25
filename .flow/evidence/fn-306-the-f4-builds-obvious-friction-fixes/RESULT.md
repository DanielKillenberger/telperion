# fn-306 result

Raw output in `raw/` (ignored). All runs on this worktree's own `target/`, through `telperion-heavy 8 --threads 2`.

## R1, the guards run first

Six tests at one thread under each profile (`raw/order.log`, `raw/order-default.log`): the two guard binaries' tests, the compile-size ceiling, two species tests and `compile_size::tests::a_callee_counts_once_a_call_site`.

| Order | `--profile gate` (this change) | `--profile default` (what the gate inherited before) |
|--:|---|---|
| 1 | generation_limit_guard generation_limits_are_classified | species a_digest_that_moved_... |
| 2 | generation_limit_guard new_literal_limits_are_detected | species budget::the_ceiling_and_... |
| 3 | compile_size every_entry_inlines_under_the_ceiling | generation_limit_guard generation_limits_are_classified |
| 4 | species a_digest_that_moved_... | generation_limit_guard new_literal_limits_are_detected |
| 5 | species budget::the_ceiling_and_... | compile_size a_callee_counts_once_a_call_site |
| 6 | compile_size a_callee_counts_once_a_call_site | compile_size every_entry_inlines_under_the_ceiling |

Narrow runs: `cargo test --profile ci -p telperion-core --test generation_limit_guard` passed 2, 1 ignored, in 57 s from a cold target.

## R2, the digest takes an age (`raw/digest-cases.log`)

`european-beech`, seed 1, its own age 40:

- no age and age 40: `b7048ee2...fbb3d` both; age 12: `7ea3210b...8419d`.
- `abc`: `no age abc`; `-1` and `1000001`: `invalid input: age = ...` from `Family::validate`; `12.5`: `invalid input: age = 12.5` from the build; an argument after the age: the usage line. Each exits 1.

## R3, `build_time` times a walk frame (`raw/frame-cases.log`)

`BUILD_FRAME=european-beech,norway-spruce,0.5 BUILD_AGE=12 BUILD_SEED=1 BUILD_PRESETS=european-beech,norway-spruce`:

| Preset | Age | Cold ms | Warm ms | Instances | Device peak MB |
|---|--:|--:|--:|--:|--:|
| european-beech to norway-spruce t 0.5 | 12 | 155 | 56 | 1445 | 37 |
| norway-spruce | 12 | 87 | 82 | 90936 | 88 |
| european-beech | 12 | 44 | 40 | 1037 | 35 |

The frame alone ran too. Two fields, an unknown preset, `t` = `x`, `t` = 1.5, no `BUILD_AGE` and `BUILD_SEED=x` each fail naming the value.

## R5, the gate and npm

- Gate, `CARGO_NEXTEST=<0.9.145> telperion-heavy 8 --threads 4 -- python3 scripts/gate.py --base fn-198-tree-space-f-the-engine-in-production` (`raw/gate.log`): 1,473 passed, 0 failed, 123 skipped; build and listing 148 s, the test run 1,179 s against 1,560 s. Its first results were `generation_limit_guard`'s two tests (1/1473 and 3/1473); the compile-size ceiling was the first device test to finish (10/1473, 0.2 s).
- `npm test` (`raw/npm-test.log`): 22 files, 177 tests passed.
- Codex impl-review: SHIP from all three draws (correctness, contracts, integration).
