# CI under five minutes: skip unchanged suites, split the jobs, a lighter profile

## Goal
fn-69's workflow ran the whole test surface on every push in about 24 minutes (10.8 min compiling `cargo test --release` cold, 7.1 min of serial test binaries with `species.rs` at 200 s, 3 min of `npm test`, 80 s compiling `wasm-bindgen-cli`), and most master commits touch only `.flow/` or docs. The owner wanted CI below five minutes, ideally much below, and asked whether unchanged results can be cached.

## Decisions
- > user (2026-09-18): "do we have a spec already to improve the test run speed? that seems outrageously slow"
- > user (2026-09-18): "can we not cache test results if code is unchanged or smth like that? or anything else? i like ci being below 5min. Ideally much below. But with rust building i guess that's hard."
- > user (2026-09-18): "ok /flow-next:capture and /flow-next:flow it incl. merge"
- Receipts keyed on a suite's input hash (exact skip, removes the whole cost from the common commit) rather than cached specimens or per-test caching; path filters for flow bookkeeping; Rust and Node as parallel jobs; a `ci` cargo profile (release without fat LTO, 16 codegen units) for the workflow only.
- The species suite stays whole; no bigger or self-hosted runner, no change to any assertion. Browser suites, fmt and clippy stay out.

## Acceptance Criteria
- **R1:** A push or PR touching only `.flow/**`, `docs/**` or markdown starts no run — met (the four test inputs under those trees are listed back in).
- **R2:** Per-suite receipts keyed on input hash (toolchain and profile included); a matching key skips the suite with a log line; red saves nothing — met.
- **R3:** Rust and Node run as two parallel jobs — met.
- **R4:** The `ci` profile moves no result; any differing suite stays on release — met (80 rows, 499 passed, 0 failed, 11 ignored, identical row for row).
- **R5:** Docs-only commit starts no run; unchanged rerun under 2 min; every-suite run under 6 min — partial: rerun 18 s; every-suite 10 min 59 s cold, 9 min 56 s warm (missed, bound kept as written).
- **R6:** Warm Rust compile under 3 min — met without grouping test binaries (desk 114 s -> 13 s; runner under 2 min).

## Result
`scripts/ci-key.mjs` hashes each suite's inputs from git objects; a `receipts` job looks keys up in the Actions cache; `rust` (per crate) and `node` jobs run in parallel and skip on a receipt. Runner, PR #35: cold every-suite 10 min 59 s (core 8 min 13 s), rerun 18 s, warm every-suite 9 min 56 s; master's first run 13 min 11 s (branch-scoped caches). The remaining cost is serial test execution, `species.rs` 214 s on the four-core runner; parallel binaries (nextest) or a larger runner are the ways under six minutes, the owner's call.

## Open
The six-minute every-suite bound (taken up by fn-76).

## Friction
- R4 proof ran the whole workspace twice on the desk (~17 min of waiting) -> proposed a one-off dispatch running both profiles on the runner.
