# fn-353 friction

## 11 Nov 2026, task 1: the space and core tests killed under a 16 GB cap

Doing: running `cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core` through `telperion-heavy 16 --threads 12` after the species-ownership change.
Slowed: after 59 s the scope hit its memory cap and every running test was killed (32 SIGTERM/SIGKILL, 148 not run). None of them was a real failure. Nothing says what cap or test-thread count these two crates need under nextest's default parallelism.
Cost: about 2 minutes and one wasted run.
Would have removed it: a stated memory figure and `--test-threads` for running the narrow suites through `telperion-heavy` (in `docs/gpu-tests.md` or `.config/nextest.toml`).

## 11 Nov 2026, task 2: a shell hook blocks heredoc edits that contain `<(` and redirects to a variable path

Doing: scripted multi-line edits of Rust files (the ledger rewrite in `buffer.rs`) with a Python heredoc, and redirecting a narrow test's log to `$L`.
Slowed: the dcg hook refused the heredoc because Rust text such as `Mutex<(u64, ...)>` reads to it as process substitution, and refused `> $L` as a redirect to a dynamic path. Each refusal cost a rewrite: edit scripts now go to files in the scratchpad, and every log path is written out literally.
Cost: about 5 minutes and two refused calls.
Would have removed it: a line in the dispatch or `docs/` saying edits go through script files or the Edit tool, never a heredoc, and logs through literal paths.

## 11 Nov 2026, task 2: the harness vitest suite rebuilds the jev binary after any core edit

Doing: `npx vitest run src/browser src/field harness` after the R5 core message edit.
Slowed: the harness tests run the jev crate's species binary, so a one-line change in `telperion-core` rebuilt telperion-core and telperion-jev in release profile first (about 3.5 minutes) before 142 tests that took seconds.
Cost: about 3.5 minutes of wall time.
Would have removed it: the harness tests that need the jev binary split from those that do not, or the binary built in the ci profile that the narrow Rust runs already compiled.

## 11 Nov 2026, task 3: `npx vitest run src/browser` waited four minutes for twelve tests

Doing: the narrow check of decision 7 (`render.test.ts`, `module.test.ts`) after a one-file edit in `telperion-render`.
Slowed: the run took 239 s, of which the tests took 4.8 s; the rest was the same jev-binary rebuild task 2 reported, reached here although no test in `src/browser` runs the binary.
Cost: about 4 minutes of wall time.
Would have removed it: the jev binary built only by the suites that run it (a vitest project or global setup scoped to `harness`), not by every vitest invocation.
