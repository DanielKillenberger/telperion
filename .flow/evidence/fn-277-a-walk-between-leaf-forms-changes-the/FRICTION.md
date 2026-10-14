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
