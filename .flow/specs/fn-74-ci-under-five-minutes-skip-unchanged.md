# CI under five minutes: skip unchanged suites, split the jobs, a lighter profile

## Conversation Evidence

> user (2026-09-18, turn 1): "do we have a spec already to improve the test run speed? that seems outrageously slow"
> user (2026-09-18, turn 2): "can we not cache test results if code is unchanged or smth like that? or anything else? i like ci being below 5min. Ideally much below. But with rust building i guess that's hard."
> user (2026-09-18, turn 3): "ok /flow-next:capture and /flow-next:flow it incl. merge"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The fn-69 workflow runs the whole test surface on every push and pull request and takes about 24 minutes on the standard runner, which the owner finds outrageously slow; the owner wants CI below five minutes and ideally much below. [paraphrase]

Where the 24 minutes go, from the master run of 2026-09-18 (run 35354444073): 10.8 minutes compiling for `cargo test --release` from scratch, because a PR branch's cargo cache is invisible to master and master had none yet; 7.1 minutes of test binaries run one after another, of which `tests/species.rs` alone is 200 seconds; 3 minutes for `npm test`, mostly the two wasm builds its pretest runs; 80 seconds compiling `wasm-bindgen-cli`. Most commits on master touch only `.flow/` or documentation and still pay all of it. [inferred]

The owner asked whether test results can be cached when the code is unchanged. Cargo cannot, but the workflow can: a green run records a receipt keyed on the hash of the suite's inputs, and the next run with the same hash skips the suite. That makes the common case free and leaves the cost only where the code changed. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Receipts keyed on inputs.** Each suite has an input set: the Rust suite of a crate is that crate's sources, the sources of the workspace crates it depends on, `Cargo.lock`, `rust-toolchain.toml` and the cargo profile; the Node suite is `src/**`, `harness/**`, `tests/**`, the build scripts, the lockfile, the TypeScript and Vitest configs, and the two wasm crates' inputs. A green run saves an Actions cache entry whose key is the input hash; a run that restores that key skips the suite and says so in the log. A red run saves nothing. [inferred]
- **Path filters.** The workflow does not start for a push or pull request whose changed files are all under `.flow/**`, `docs/**`, or are markdown. [inferred]
- **Two jobs.** Rust and Node run as parallel jobs so the wasm build no longer waits behind the Rust tests. [inferred]
- **A `ci` cargo profile.** Inherits `release` with thin or no LTO and more codegen units, used only by the workflow's cargo invocations; every local command keeps the release profile. The fat LTO with one codegen unit in `Cargo.toml` is what makes the 68 test binaries link slowly. [inferred]
- **Fewer test binaries, if still needed.** When the warm compile is still above three minutes after the profile change, the integration tests of a crate are grouped into a few binaries by a `tests/<group>.rs` that declares the existing files as modules, so each file keeps its content and only the link count drops. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Cache scope.** GitHub restores a cache only from the same branch or the default branch. A receipt written on a pull request is invisible to master; master writes its own after its green run, and later pull requests read master's. The key carries the toolchain and the profile so a toolchain bump never reuses a receipt. [inferred]
- **Eviction is a miss, never a failure.** A receipt evicted after seven unused days or by the 10 GB limit means the suite runs again. [inferred]
- **The species floor.** `tests/species.rs` generates every catalogue species at fixed seeds and takes 200 seconds on the 4-core runner. It runs whenever the core crate changes and its coverage is not reduced; a change to the core crate therefore has a floor of about four to six minutes on the standard runner. A bigger or self-hosted runner is the only way under that, and is the owner's separate call. [inferred]
- **A lighter profile must not move a result.** Rust does not reorder floating-point operations under LTO or codegen-unit changes, so the byte-identical identity tests should hold; the first run under the `ci` profile is compared suite by suite with the release run, and any suite that differs stays on release. [inferred]
- **A skipped suite is stated, never silent.** The step's log line names the receipt key it matched. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A push or pull request whose changed files are all under `.flow/**`, `docs/**`, or markdown starts no workflow run. Errors: a change to any other file starts the run; the filter lists paths, never file names guessed from a diff. [paraphrase]
- **R2:** Each Rust crate's test run and the Node test run record a receipt keyed on the hash of their inputs after a green run, and a later run whose inputs hash to a recorded receipt skips that suite with a log line naming the key. Errors: a red run records no receipt; a missing or evicted receipt runs the suite; the key includes the toolchain file and the cargo profile. [paraphrase]
- **R3:** The Rust suites and the Node suite run as two parallel jobs. Errors: a failure in either job fails the workflow. [inferred]
- **R4:** The workflow's cargo invocations use a `ci` profile that inherits `release` without fat LTO and with more codegen units; every package script and local command keeps the release profile, and every suite's result under `ci` matches its result under `release` on the same commit, with any differing suite left on release and named in the evidence. Errors: a differing result is reported, never a relaxed assertion. [inferred]
- **R5:** Measured on real runs and recorded in the spec's evidence: a commit touching only `.flow/` or documentation starts no run; a rerun of an unchanged commit finishes in under two minutes; a run where every suite executes, which is what a change to the core crate costs, finishes in under six minutes on the standard runner. Errors: a bound that is missed is recorded with the measured number, never moved. [paraphrase]
- **R6:** The warm compile of the Rust job is under three minutes; when the profile change alone does not reach that, the integration tests are grouped into fewer binaries per crate without changing any test's content. Errors: the measured warm compile is recorded either way. [inferred]

## Boundaries
<!-- scope: business -->

- No bigger runner, no self-hosted runner, no paid minutes; the owner decides those separately. [paraphrase]
- No change to any test's assertions or to the species suite's coverage; a suite is skipped by receipt or run whole. [inferred]
- The browser suites, species QA, fmt and clippy stay outside the workflow, as fn-69 left them. [inferred]
- No caching of generated specimens: when the generator changed the cache is invalid, and when it did not the receipt already skips the suite. [paraphrase]

## Decision Context
<!-- scope: both -->

Receipts keyed on inputs instead of cached specimens or cached test results per test, because a suite's inputs are cheap to hash, the skip is exact, and it removes the whole cost from the common commit rather than shaving it. Path filters on top because most of master's commits are flow bookkeeping. Two jobs and a lighter profile because compile and link are the largest fixed cost on a run that must happen, and the 68 separately linked test binaries under fat LTO are the reason. The species suite stays whole: it is the test that matters when the generator changes, and the owner's five-minute wish is met for every other change while the core-change floor is stated honestly. [inferred]
