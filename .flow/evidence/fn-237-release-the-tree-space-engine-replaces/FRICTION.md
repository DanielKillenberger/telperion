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
