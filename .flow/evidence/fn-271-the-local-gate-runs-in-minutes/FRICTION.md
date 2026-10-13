# fn-271 friction

## 2026-10-13, task 1 (worker): no per-test times in the gate's output

- **Doing:** R1, the slowest tests by name.
- **Hindered:** `cargo test` prints a time per binary only. Libtest's `--report-time` needs `RUSTC_BOOTSTRAP=1`, and setting that for cargo rebuilt 28 crates each time it was turned on or off. Run through a cargo runner instead, it reached trybuild's rustc and failed the render crate's `boundary` test on a changed diagnostic.
- **Cost:** about 10 minutes and a 49 s rerun of `boundary`.
- **Would have removed it:** the local gate under nextest, which times every test, or a checked-in runner script that exempts the trybuild binaries.

## 2026-10-13, task 1 (worker): the desk was shared during every measurement

- **Doing:** before-and-after wall times of the gate and of single binaries.
- **Hindered:** another project's test runs held the desk at load 10 to 47 throughout (154 for a moment), so equal runs differ by tens of seconds and a 20 s gain cannot be told from noise.
- **Cost:** two measurements thrown away as inside the noise (the core lib's repeat test, the walks' refinement levels).
- **Would have removed it:** a quiet window for timing runs, or CPU-seconds rather than wall time as the figure compared.

## 2026-10-13, task 1 (worker): nextest at its default width runs the desk out of memory

- **Doing:** R4, the workspace under nextest to time it.
- **Hindered:** at 32 test processes, available memory fell to 2 GB in 49 s (many full preset builds at once), and the guard stopped the run at 986 of 1,357 tests.
- **Cost:** one aborted run, about 2 minutes.
- **Would have removed it:** a stated local width for nextest (`-j`) or a test group for the whole-preset tests in `.config/nextest.toml`.

## 2026-10-13, task 1 (worker): even with test groups, nextest ran memory to zero

- **Doing:** R4, a fourth nextest attempt at width 16, with the whole-preset tests two at a time and the walks one at a time.
- **Hindered:** the walks' `every_setting` (spread over every core), the repeat test and the mesh test together took available memory from 15 GB to 0 between two 2 s polls, before the guard stopped the run at 1,248 of 1,357 tests.
- **Cost:** about 7 minutes, and a moment of swapping on the owner's desk. No further nextest runs were made.
- **Would have removed it:** a memory budget per test (or per group) known before the run; the whole-preset tests' peak memory is not written down anywhere.
