# A test run leaves the working tree as it found it

## Goal & Context

Running the render tests rewrites a tracked file. `crates/telperion-render/tests/common/resolution.rs` `record` writes each resolution test's measured margins to `.flow/evidence/fn71/resolution/<name>.json` on every run, and four such receipts are tracked: `bark_distance`, `bark_resolution_grazing`, `bark_resolution_trunk` and `smooth_bark`. The measurement comes off the GPU and is not stable to the printed precision: on 2026-09-20 one full-suite run moved `smooth_bark.json`'s SilverBirch row from mean 2.5470, p95 9.25 to mean 2.5604, p95 9.50. The other three did not move in the runs observed; whether they can is unknown. [user]

The cost is a dirty tree after every gate run. It was hit three times on 2026-09-20 during fn-87 and fn-92: `git add -A`, which flow-next's work skill requires, sweeps the file into an unrelated commit unless it is restored first, and the owner's local `dcg` hook blocks `git checkout -- <path>` and `git restore`, so each restore took a tagged stash. [user]

The receipts are wanted. `record`'s own comment says a receipt that cannot be written is a failure, because the margin the next change needs would be unknown. What is not wanted is a routine test run deciding what the committed evidence says. [inferred]

## Architecture & Data Models

`record` keeps writing a receipt on every run, to an untracked place: `env!("CARGO_TARGET_TMPDIR")/resolution/<name>.json`. It writes into the tracked evidence directory only when `TELPERION_RECORD_EVIDENCE=1` is set, which is how a spec that changes bark resolution refreshes the committed margins on purpose. The JSON shape, the assertion that the receipt exists after the write, and every bound a test holds are unchanged. The evidence directory stays named in one function, `receipt_path`. [inferred]

## Acceptance Criteria

- **R1** On the owner's desk, from a clean tree, `cargo test --profile ci -p telperion-render` leaves `git status --porcelain` empty. Red on the base: the same command leaves `.flow/evidence/fn71/resolution/smooth_bark.json` modified (run it until it does, at most five runs, and record the count; if it never does, stop with `NEEDS_HUMAN`, because the repro this spec rests on did not hold).
- **R2** The same run writes all four receipts under the target temp directory, asserted by the tests themselves through `record`'s existing post-write check.
- **R3** With `TELPERION_RECORD_EVIDENCE=1`, the four receipts are written to `.flow/evidence/fn71/resolution/` in the unchanged JSON shape. Covered by a unit test of `receipt_path` over both settings, not by a second GPU run.
- **R4** No test's bound, fixture or assertion changes; `git diff` touches `tests/common/resolution.rs` and its new test only, plus a line in `docs/` or the module comment naming the variable.

## Boundaries

- The four committed receipts are left as they are; this spec does not refresh them.
- No change to rendering, presets, the generator or CI.
- Not a search for other tests that write into tracked paths. If the worker meets one it is reported in FRICTION.md, not fixed here.
- No GPU captures and no image inspection.
