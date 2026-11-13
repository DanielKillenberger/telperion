# fn-364 friction

## 13 Nov 2026, 20:28 (builder, Opus)

- **Doing:** the R2 tests green after the change, one narrow `cargo test -p telperion-space --lib surplus_tests` through `telperion-heavy 8`.
- **What slowed it:** the yield watcher paused the queue at 20:22:53 while the owner used the machine, and fn-363's builder's old-oak runs held the memory (the queue read "room -1 to 7 GB for 8"). A build that took 9 s at 20:16 waited from 20:21.
- **Cost:** wall time only; the docs (R7), the R5 runner, its scripts and its table script were written meanwhile.
- **What would have removed it:** nothing in the repository; it is the shared machine's admission working as designed.
