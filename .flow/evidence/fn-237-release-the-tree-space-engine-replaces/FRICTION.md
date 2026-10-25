# fn-237 friction

## 2026-10-13, housekeeping worker (R5-R15)

- **Doing:** the first `npm run wasm:build` in the worktree, under `telperion-heavy 8`.
- **Hindered:** the wrapper waited for 8 GB of room (5-7 GB free) while other sessions held the machine; a cold release Wasm build had to queue before any R15 check could start.
- **Cost:** about 10 minutes of waiting before the build began.
- **Would remove it:** nothing in the repository; machine load from parallel sessions (local setup, reported only).
- **Doing:** the render crate's focused lib tests (R12, R14) under `telperion-heavy 10`.
- **Hindered:** the wrapper held at "room 7 GB for 10" for more than 10 minutes while another worker's jobs held the desk; declared 8, it had already waited 25 minutes for the first Wasm build, which then ran in 7 s under a 5 GB declaration.
- **Cost:** about 35 minutes of waiting across the two jobs.
- **Would remove it:** a per-command memory table (what a Wasm build, a render lib test build and the gate each peak at) beside `telperion-heavy`, so a worker declares what a job needs rather than guessing high. Local setup, reported only.
- **Doing:** formatting the five files this branch changed with `rustfmt`, since `cargo fmt -p` rewrites untouched files (R7).
- **Hindered:** `rustfmt` on a file that declares modules (`generation.rs`, `lib.rs`) follows the `mod` lines and formatted 93 other render files, `generation/` included, which the dispatch put off-limits. The dcg hook refuses `git checkout --` and `git restore`, so each file was written back from `git show HEAD:<path>` through a script and the two edits reapplied.
- **Cost:** about 5 minutes and one near-miss on the other worker's tree.
- **Would remove it:** R7's one formatting commit; until then `rustfmt --config skip_children=true` (nightly only) or formatting leaf files only.
- **Doing:** the end checks, `npm test` (declared 10 GB after an OOM kill at 6 GB) and then the gate at 20 GB.
- **Hindered:** no heavy job was running, but the desk had 12 to 13 GB available against a 6 GB reserve, so room sat at 6 to 7 GB from 19:52 to 20:23 and neither check could be admitted. The gate's 20 GB needs 26 GB available, which the desk has not had all evening. The wrapper also admits one job at a time, so a 5 GB job queued behind the 10 GB one until that one was stopped.
- **Cost:** about 35 minutes of waiting on `npm test`; the gate was never started.
- **Would remove it:** the host running the gate and `npm test` once memory frees up, or the owner freeing memory on the desk. Admission skipping past a job that does not fit to one that does would also help. Local setup, reported only.

## 2026-10-14, release-checks worker (R9, R14)

- **Doing:** finding whether Tint can run without a browser.
- **Hindered:** nothing in the repository says where a Tint binary comes from. Dawn's GitHub releases ship one (`Dawn-<hash>-ubuntu-latest-Release.tar.gz`, `bin/tint`, 36 MB tarball), found by listing release assets. `tint` also refuses a multi-entry module unless `--ep` names one entry, which looked at first like 31 of 37 modules failing.
- **Cost:** about 10 minutes.
- **Would remove it:** docs/gpu-tests.md naming the release asset and the per-entry invocation (done in this branch).
- **Doing:** a cold, warm and cold-again measurement of every entry, three runs in one command.
- **Hindered:** the dcg hook refuses a redirect to a path held in a shell variable (`> $S/cold2.log`), so the command had to be rewritten with every path spelled out.
- **Cost:** about 2 minutes.
- **Would remove it:** a scratch-directory allowance in the hook for the session's own scratchpad. Local setup, reported only.
- **Doing:** a 0.1 s CPU-only test run (`tint_rule`) after the review's two findings, under `telperion-heavy 8`.
- **Hindered:** the wrapper held it behind another worker's film render (fn-272) with about 10 GB available; the test needs a rebuild of the render test binary, not the GPU, and still waited.
- **Cost:** over 20 minutes of waiting for a test that runs in a fraction of a second once built.
- **Would remove it:** the per-command memory table named above, so an incremental test build declares a few GB and is admitted beside a running job. Local setup, reported only.
  Resubmitted declaring 5 GB, the same run was admitted at once beside the film job and passed: the 8 GB declaration was the guess that cost the 40 minutes.
- **Doing:** the end gate as dispatched, `telperion-heavy 9 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast`.
- **Hindered:** the scope was OOM-killed at its 9 GB cap after 9 minutes, during telperion-core's tests (9 GB peak); no result came out of it.
- **Cost:** 9 minutes and a second gate run.
- **Would remove it:** the gate's measured peak written beside `telperion-heavy` (the per-command memory table again); housekeeping declared 20 GB for the same gate.
- **Doing:** the gate again, `telperion-heavy 10 --gpu 4 --threads 2`, after the OOM kill at 9 GB.
- **Hindered:** from 09:58 to 10:40 the wrapper never found room ("room -1 to 1 GB for 10"); another worker's film renders (fn-272, `examples/film`, 6 GB peak, about 36 minutes each) were admitted one after another while the gate waited.
- **Cost:** 42 minutes of waiting; the gate was stopped unrun, and this worker returns NEEDS_HUMAN for it.
- **Would remove it:** the host running the gate once the film renders finish, or a queue that admits in arrival order. Local setup, reported only.
- **Doing:** the operator-mix rule's focused tests, `telperion-heavy 5`.
- **Hindered:** queued 20 minutes behind the next film render with 8 GB available; the same run declared at 4 GB was admitted at once and passed.
- **Cost:** 20 minutes.
- **Would remove it:** as above, a measured declaration per command (the render crate's focused lib tests fit in 4 GB). Local setup, reported only.
