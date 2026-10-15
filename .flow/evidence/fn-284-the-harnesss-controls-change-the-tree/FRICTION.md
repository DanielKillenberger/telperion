# fn-284 friction

## 2026-10-15, task 1: the reference rewrite waits for admission

Doing: regenerating `docs/parameters.md` and `parameters.generated.ts` after dropping the scaffold rows' stages (one `cargo test --test parameter_reference` with `TELPERION_WRITE_REFERENCE=1`).
Slowed by: `telperion-heavy` held the job behind another worktree's full workspace gate; a cold `--profile ci` build of this worktree follows. Over 10 minutes before the first line of output, for a file rewrite.
Would remove it: the catalogue's generated files written by a small binary (or `cargo run --example`) that builds only `telperion-core` without the test profile's whole graph, so a metadata rewrite need not queue as a heavy job.

## 2026-10-15, task 1: a 10 GB claim is never admitted on this desk

Doing: the same reference rewrite. After the other worktree's gate finished, no heavy job ran, but `free` showed 8 GB available against the 10 GB claim plus the 2 GB reserve, so admission never came: 34 minutes queued (10:28 to 11:02) for nothing.
Done instead: reran it under `telperion-heavy 6`, a cap that kills rather than swaps.
Would remove it: `telperion-heavy` saying why it is waiting (needed versus available) once a minute, and a narrow single-crate build being dispatched with a claim sized to it, not the gate's.

## 2026-10-15, task 1: the probe outgrows the memory the desk admits

Doing: the on-demand controls probe over every shipped preset.
Slowed by: the first run spawned one build per core (32) and was killed at the 6 GB cap in a minute; the second, at four builds, was killed on the Norway spruce after two presets (about 9 minutes lost). The desk admits about 6 GB (8 GB available, 2 GB reserve), less than the 10 GB the dispatch named.
Done instead: the probe follows `RAYON_NUM_THREADS` (which `telperion-heavy --threads` sets) and runs one preset at a time with `TELPERION_PRESETS`.
Would remove it: a per-preset memory estimate printed before the probe starts, or the probe sizing its own concurrency from the cap.

## 2026-10-15, task 1: the gate cannot run on this desk now

Doing: the workspace gate at the end of task 1.
Slowed by: the 10 GB claim is never admitted (8 GB available, 2 GB reserve), and under a 6 GB cap the gate is killed. The gate is unproven for this commit; the narrow suites that cover the change (core `catalogue`, `parameter_reference`, the probe's compile, the harness typecheck) passed.
Would remove it: memory freed on the desk (the owner's dev server and browser hold most of it), or the gate split so a crate's suite can run under a smaller cap.

## 2026-10-15, closing: three presets outgrow every cap the probe was given

Doing: the controls probe on Norway spruce, Telperion and Laurelin.
Slowed by: each was killed at 6 GB with two builds and with one, and again at 10 GB with four (host: retry once at the normal setting). About 25 minutes across the attempts; those three presets are unprobed.
Would remove it: a probe that builds those trees younger or one row at a time under a measured per-build memory figure, or a desk slot with more than 10 GB for this check.
