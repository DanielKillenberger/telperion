# fn-277 friction

## 2026-10-14, task 1, R1 first build

- Doing: building `examples/walk_leaves` (`--profile ci`) for the first repro scan.
- Slowed by: `telperion-heavy 4` waited for admission; the compile itself took 16 s, the call 4 min 48 s. Other workers held the machine's free memory (7 GB available, 2 GB reserve).
- Cost: about 4.5 minutes of wall time, no tokens.
- Would remove it: nothing in the repository; a declared size of 3 GB admitted at once on later calls.

## 2026-10-14, task 1, reading the film's frame log

- Doing: reading the per-frame leaf counts FILM.md cites as `frames.tsv`.
- Slowed by: the file is at `raw/cut3/frames.tsv`, not beside FILM.md as the brief said.
- Cost: one search, under a minute.
- Would remove it: FILM.md naming the path it cites.

## 2026-10-14, task 1, a core test run killed at 8 GB

- Doing: `cargo test --profile ci -p telperion-core -p telperion-space` under `telperion-heavy 8 --threads 4`, to find the tests the row change broke.
- Slowed by: the kernel killed the scope at its 8 GB cap after 6 minutes, before the lib tests printed their failures; `cargo nextest` is not installed here, so the run could not be resumed by failing test.
- Cost: about 7 minutes and one rerun by name.
- Would remove it: the gate's own size (9 GB) for any partial suite; nextest installed, so a rerun takes only the failures.
