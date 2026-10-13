# The local gate runs in minutes

## Goal & Context

The owner (2026-10-13), watching fn-270 task 3's gate: "you sure? are tests this slow?" The gate (`cargo test --profile ci --workspace --no-fail-fast`, AGENTS.md "Gates") took about 25 minutes. Measured in fn-270's `raw/t3-gate.log`: `telperion-space`'s `tests/walks.rs` 455 s (six parameter-walk tests past 60 s each, `every_setting_changes_the_tree_by_degree`, `branches_are_made_and_unmade_at_vanishing_size` and others), three unit-test binaries 235, 112 and 102 s, `catalogue_identity` 63 s, then a tail under 45 s each. `cargo test` runs binaries one after another; CI runs nextest. Friction: fn-270 FRICTION.md (tasks 1 and 2).

## Requirements

- **R1 (measure):** per-binary and per-test wall time of the gate on the current engine, the slowest twenty tests named with what each grows (preset, age, seeds, steps).
- **R2 (walks at the smallest tree that still tests the premise):** each slow walk test runs at the smallest age, seed count and step count at which it still catches its defect: shown red on a deliberately broken generator (a setting made a switch, a branch made at full size) and green on the engine. No assertion is deleted; a premise that only holds at full size stays at full size and is named.
- **R3 (the other slow binaries):** the same treatment for the three slow unit-test binaries and `catalogue_identity`; whole-preset checks that duplicate another test are named for the host, not removed.
- **R4 (proof):** the gate's wall time before and after on this machine (one heavy run at a time), every test count unchanged except ones moved behind `--ignored` with a named command in `docs/` and the reason. Whether the local gate should run under nextest as CI does is reported with its measured time for the owner to decide (AGENTS.md names the gate command).

## Boundaries

Test-only: no generator, renderer or preset change. Release housekeeping (fn-237).
