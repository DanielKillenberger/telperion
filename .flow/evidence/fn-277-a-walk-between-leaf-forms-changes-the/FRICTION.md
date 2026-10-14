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

## 2026-10-14, task 1, waits for room on the heavy runner

- Doing: the bound measurement, the reference rewrite and the film's walk frames, each under `telperion-heavy` at 4 to 6 GB.
- Slowed by: other workers' gates held the machine; each call waited 5 to 15 minutes for admission, one background run lost its output when it was moved to the background and its duplicate later rewrote a finished file.
- Cost: about 40 minutes of wall time, a few thousand tokens of polling.
- Would remove it: a queue position printed by `telperion-heavy` while it waits, so a caller can tell a long wait from a stuck one.

## 2026-10-14, task 1, the gate at 9 GB

- Doing: the local gate, `telperion-heavy 9 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast`.
- Slowed by: the first run was killed by the kernel at its 9 GB cap after 11 minutes in telperion-core's lib tests, before any summary printed (its one failure, the sweep's held rows, was found by rerunning it alone). The second waited 20 minutes for room and was cancelled; other workers' gates held the machine. Whether 9 GB at four threads holds on the base was not checked.
- Cost: about 35 minutes, no gate result.
- Would remove it: a stated gate size that the base is known to fit in, and nextest, so a killed run can be resumed by test.

## 2026-10-14, task 1, waits for room again

- Doing: the device agreement tests and the film walks under the form oracle.
- Slowed by: `telperion-heavy` admission waits of 10 to 20 minutes on each of three calls while other workers' gates ran; the scratchpad's share lists were removed by another session's cleanup between calls and had to be rewritten into the evidence directory.
- Cost: about 45 minutes of wall time.
- Would remove it: inputs kept in the worktree's own evidence directory from the start.

## 2026-10-14, task 1, the gate and npm unadmitted

- Doing: the final gate (`telperion-heavy 10 --gpu 4 --threads 4`) and `npm test`, and the Laurelin to Telperion walk's draws at 80.
- Slowed by: none was admitted: the gate queued 30 minutes for 10 GB, npm 30 minutes for 9 GB (its first run at 6 GB was killed at the cap), the Laurelin frames an hour for 9 GB, while other workers held the machine. Each was cancelled as ruled.
- Cost: about two hours of wall time, no result from any of the three.
- Would remove it: the host's run on the merged branch, as ruled; a queue that tells a waiting job its place.
