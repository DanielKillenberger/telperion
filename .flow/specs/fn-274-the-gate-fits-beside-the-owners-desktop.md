# The gate fits beside the owner's desktop

## Goal

fn-251 and fn-270 (2026-10-14): the workspace gate was killed for memory below a 13 GB cap or above 2 test threads, in `telperion-core`'s unit tests, where the repeat test held a whole preset while growing it again beside the `suite::` whole-preset tests. With the owner's desktop open the desk has about 13-15 GB, so a gate occupied the whole machine. Owner (2026-10-13): "doesn't completely block the machine but if there is capacity you can run multiple things". Test-only, an obvious fix (AGENTS.md, friction); release housekeeping (fn-237).

## Decisions

- Owner (2026-10-13): "doesn't completely block the machine but if there is capacity you can run multiple things".
- Host (2026-10-14): the gate passes at a 9 GiB cap with 4 test threads (peak 8,860 MiB, 1,551 s, every test count unchanged); the 8 GB target is missed by under 1 GiB and accepted. The four ~6 GB whole-mesh tests stay separate: each proves its own entry (fn-271's decision); merging them would trade coverage for 0.9 GiB. The gate is declared `telperion-heavy 9 --gpu 4 --threads 4`.

## Requirements

- **R1:** the repeat test compares builds by a build-order digest, one tree held at a time; a perturbed build still fails — met (build-order fold of the derived `Debug`, NaN refused; a one-bit change, swapped leaf axes and a NaN checked).
- **R2:** each whole-preset unit test's peak measured; those over 2 GB hold one tree at a time — met (eleven tests over 0.4 GB take one process-wide turn, `one_tree::turn`).
- **R3:** gate peak at 4 threads before and after, target under 8 GB, counts unchanged — partial: 8,860 MiB, accepted by the host.

## Result

Per-test peaks alone: the `packed_leaf` pair, `suite::mesh` and the pipeline artifact test 5.75-6.19 GB each (one Telperion build); the repeat test 5,036 → 3,752 MiB; ceiling test 2.6 GB; relay test 2.0 GB; the rest under 1 GB. `telperion-core` unit tests at 4 threads: killed at 13 GiB (over 11,651 MiB) before → 7,541 MiB after. Workspace gate at 4 threads: passed at 9 GiB (8,860 MiB anon, 1,551 s), OOM-killed at 8 GiB. Counts unchanged (451 in telperion-core; 1,348 run, 87 ignored). `npm test` passed at 8 GB. Codex SHIP after a test-count and NaN fix.

## Open

The last gigabyte would need the heavy tests to share one build per preset — declined by the host as a coverage trade.

## Friction

- No GNU `time` on the desk; dcg hook refused variable-path redirects -> peak-measuring wrapper in `scripts/` proposed.
- `telperion-heavy`'s cgroup `memory.peak` reads the cap (page cache) -> report anonymous peak at exit.
- Codex reviewers ran the whole-preset repeat test outside `telperion-heavy` beside the gate -> reviewer brief forbids it.
- Gate OOM-killed at 8 GiB though the binary alone peaked 7.5 GiB -> gate-scope measurement mode in `telperion-heavy`.
