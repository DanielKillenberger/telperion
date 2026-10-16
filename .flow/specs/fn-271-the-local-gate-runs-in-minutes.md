# The local gate runs in minutes

## Goal

The owner (2026-10-13), watching fn-270 task 3's gate: "you sure? are tests this slow?" The gate (`cargo test --profile ci --workspace --no-fail-fast`) took about 25 minutes, dominated by `telperion-space`'s `tests/walks.rs` (455 s), three unit-test binaries and `catalogue_identity`. Measure it, shrink the slow tests without weakening any assertion, and report whether the local gate should run under nextest as CI does. Test-only; release housekeeping (fn-237).

## Decisions

- Owner (2026-10-13): "you sure? are tests this slow?"
- Host (2026-10-13, after task 1): R2 — the walks stay at full size: the vanishing-size walk passes a full-size-branch defect at 100 or 50 steps, so 200 steps is its premise; fewer seeds would lower its asserted crossing count. Named under the spec's own clause ("a premise that only holds at full size stays at full size and is named").
- Host: R3 — the per-entry full-preset builds stay separate, each proving its own entry (`docs/principles.md`).
- Host: R4 — nextest locally runs the desk out of memory (whole-preset tests hold several GB each); the gate stays `cargo test`, declared under `telperion-heavy`. Reported to the owner.

## Requirements

- **R1:** per-binary and per-test gate times, the slowest twenty named with what each grows — met.
- **R2:** each slow walk at the smallest size that still catches its defect, shown red on a broken generator; no assertion deleted — met by naming: the walks' premise needs full size (200 steps, three seeds, age 12).
- **R3:** the same for the slow unit-test binaries and `catalogue_identity`; duplicate whole-preset checks named for the host — met.
- **R4:** gate time before and after, test counts unchanged; nextest measured for the owner — met (nextest not completed: out of memory at every width).

## Result

Every gain came from growing at once trees a test grew one after another; no assertion, age, seed, step count or preset changed, no test moved behind `--ignored`. Gate 1,525 s → 1,128 s (25:25 → 18:48), 1,357 run and 89 ignored both times. Per binary: walks.rs 481.6 → 310.2 s (peak memory 2.7 → 6.9 GB), core lib 217.4 → 190.4 s (bound by the repeat test's one-core full-size builds), field lib 100.9 → 54.0 s, `catalogue_identity` 62.8 → 10.1 s (digest unchanged), render lib unchanged (GPU). Each changed test was shown red on a temporary broken-generator patch (insertion made a switch, branches made at full size, a NaN mesh position, a frozen blend, a wrong field seed) and green on the engine. One setting, `states[0].zones[1].lateral[0]` up to p = 0.295 (trees up to 617,000 branches), is most of the walks' time. Nextest at widths 4-32, with and without test groups, ran available memory under 4 GB and was stopped at 986-1,275 of 1,357 tests; the core lib alone peaks at 17-18 GB under `cargo test`. Codex review kept R2 and R4 as partial; overridden as the host's decisions above.

## Open

Narrowing the explosive trunk-lateral walk, sharing walk trees across the two big walk tests under nextest, and a smaller core-lib peak are later work if the gate's time bites again (the core-lib peak was taken by fn-274).

## Friction

- `cargo test` prints no per-test times; `--report-time` needs `RUSTC_BOOTSTRAP=1`, which rebuilt crates and broke a trybuild test -> nextest or a checked-in runner script.
- The desk was shared during every timing (load 10-47) -> quiet window or CPU-seconds as the compared figure.
- Nextest at default width, then with groups, ran the desk out of memory -> per-test memory budgets (fn-274 measured them).
