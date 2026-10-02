# The no-geometry job tests the build without geometry

## Conversation Evidence

> user (2026-10-02, on fn-185's R4.1, the dependency fix): "ok"

## Goal & Context

CI's `core without geometry` job (`.github/workflows/tests.yml`, added by #55 for fn-100) runs `cargo test --profile ci -p telperion-core --no-default-features --lib`. Since #126 (2026-09-26) it runs the whole core suite under libtest in 333 to 454 s, and it is the slowest job: CI takes 7 to 8.5 min where it took about 4 (fn-185 REPORT, R3). [checked]

The cause: the crate's self dev-dependency `telperion-core = { path = ".", features = ["json"] }` (`crates/telperion-core/Cargo.toml:37`) keeps default features. Cargo unifies them into the test build, so `--no-default-features` does not drop `geometry`. Checked 2026-10-02 at `33bc835a`: the no-default lib test binary lists 518 tests, 262 of them in `suite::` (gated `cfg(all(test, feature = "geometry", feature = "json"))`). `cargo tree -e features -i telperion-core --no-default-features` shows `default` and `geometry` enabled through `[dev-dependencies]`. The job therefore tests the geometry build a second time and never tests the build without geometry; only its `cargo build` step sees that configuration. [checked]

This is an obvious fix in the sense of AGENTS.md "Friction reports": the cause is named and the remedy changes no product behaviour. fn-185 R4 item 1. [checked]

## Architecture & Data Models

- The self dev-dependency sets `default-features = false`. The package's own default features still come from the command line, so a plain `cargo test -p telperion-core` and the workspace gate keep `geometry`. [inferred, to be checked by R2]
- **Without geometry, the lib tests do not compile.** Checked 2026-10-02 with `default-features = false` on the dev-dependency: 196 errors. 190 of them are in the growth-path tests (`pipeline/branching/specimen/*`), which fn-181 deletes; `specimen/measurement.rs` mixes a type that regular code uses with test functions that need geometry. The other six are in four places: `pipeline/surface.rs`'s `#[cfg(test)] pub use build::extent`, and one test each in `pipeline/tests.rs`, `pipeline/foliage/packed/tests.rs` and `params/tests.rs`. Each of those gets a `geometry` gate. The partial gates are in a git stash, "fn-186 partial: no-geometry test gates (resume after fn-181)". [checked]
- The job has never tested the build without geometry since #55 added it: the unification hid the broken test build. [checked]

## Acceptance Criteria

- **R1:** `cargo test --profile ci -p telperion-core --no-default-features --lib -- --list` lists no `suite::` test, and `cargo tree -e features -i telperion-core --no-default-features` shows no `geometry`. That command passes. [checked repro]
- **R2:** With default features, the core lib's test list is unchanged: 518 tests at the base. The workspace gate passes. [inferred]
- **R3:** On the PR's CI run, `core without geometry` finishes in under 3 min and runs only the tests without geometry. The run's total time is recorded against fn-185's 7.2 to 8.4 min. [inferred]

## Boundaries

- No other CI job, test or pin changes. The other R4 items stay out (fn-185). [user]
- The workflow file changes only if R1 cannot hold without it. [inferred]

## Decision Context

- fn-185 measured the regression and named this fix as obvious. The owner agreed to build it on its own (2026-10-02). [user]
- **Built after fn-181 (owner, 2026-10-02).** Most of the breakage is in growth-path tests that fn-181 deletes, so gating them first would be throwaway work. CI stays at 7 to 8.5 min until then. The alternatives offered were an interim cut of the job's test step, or gating everything now. [user]
