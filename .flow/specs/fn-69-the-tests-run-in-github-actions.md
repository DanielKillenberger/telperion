# The tests run in GitHub Actions

## Conversation Evidence

> user (2026-09-18, turn 1): "a new spec for ci/cd"
> user (2026-09-18, turn 2): "so run tests in github actions"
> user (2026-09-18, turn 3): "you can work and merge autonomously once it works"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The repository has no continuous integration. Every test runs only when someone runs it on their own machine, and the fn-58 friction report of 2026-09-18 records the cost of that: a ten-minute workspace test run blocking a three-hook chore commit, with the harness's ten-minute shell cap sitting right at the run's length. The owner wants the tests to run in GitHub Actions. [paraphrase]

The deliverable is one workflow on the GitHub repository that runs the workspace tests on every push to master and every pull request, and is green on master before the spec closes. Once it is green, the owner has authorized the build and the merge to proceed without further questions. [paraphrase]

The README's "Build and develop" block is the documented check sequence and the workflow runs the test commands from it in the same order, so a contributor and the runner see the same commands. The workflow adds no test of its own. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **One workflow, one job per platform surface.** A single workflow file under the repository's GitHub Actions directory, triggered on push to master and on pull requests, with superseded runs on the same ref cancelled. [inferred]
- **The toolchain comes from the repository's pins.** The Rust channel, components and the wasm target from `rust-toolchain.toml`, the `wasm-bindgen-cli` version Cargo.lock pins and `render:build` checks, and a Node version that satisfies the `engines` field in package.json. The workflow restates no version the repository already pins. [inferred]
- **The commands are the package scripts.** `npm run rust:test` for the Rust workspace, `npm test` for the Vitest suites (its pretest builds the wasm and render modules, so the Wasm bindings compile on every run), and `npm run typecheck`. No command is copied out of a script into the workflow. [inferred]
- **Caches keyed on the lockfiles.** The cargo registry and target directory keyed on Cargo.lock and the toolchain file, the npm cache on package-lock.json, so a run that changes no dependency rebuilds only what changed. The release profile builds with LTO and one codegen unit, so the first uncached run is the slow one. [inferred]
- **No GPU, no secrets.** Hosted runners offer no WebGPU adapter. The render crate's suites already report `skipped:` and pass on a device that returns `WebGpuUnavailable` or `FallbackOnly`, so they compile and skip. `TYPESAFE_API_KEY` is never set in the workflow; the jev tests keep the mock transport with the key unset, as `docs/typesafe.md` requires. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Run length.** The workspace test run took ten minutes on the owner's machine on 2026-09-18, most of it in the core `species` suite generating every catalogue species at fixed seeds. A hosted runner has fewer cores, so the uncached run is longer. The spec records the measured length of the first green run and of a cached rerun in its evidence and sets no bound; a bound is the owner's later call, and the fn-58 friction report's `test:quick` idea is a separate fix. [inferred]
- **The wasm-bindgen pin.** `render:build` fails with the install command when the installed `wasm-bindgen` does not match the crate's pin. The workflow installs that exact version and caches the binary, so a pin bump changes one place, Cargo.lock, and the workflow follows it. [inferred]
- **Browser suites stay off.** `test:render` needs a hardware adapter that headless Chromium on a runner does not offer, and `rust:test:wasm` needs a Playwright Chromium install. Neither runs in this spec (see Boundaries). [inferred]
- **Formatting is red today.** `cargo fmt --all -- --check` fails on three files as of 2026-09-18 and clippy passes. The format and lint checks are outside this spec, so the workflow does not fail on them. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A GitHub Actions workflow runs on every push to master and every pull request, and runs the Rust workspace tests through `npm run rust:test`. Errors: a failing test fails the run; the render crate's GPU suites print `skipped:` on the runner and do not fail it. [paraphrase]
- **R2:** The same workflow runs the Vitest suites through `npm test`, which builds the wasm and render modules first, and the TypeScript check through `npm run typecheck`. Errors: a wasm or render build failure, a failing suite, or a type error fails the run. [inferred]
- **R3:** The toolchain the workflow installs is read from the repository's pins (`rust-toolchain.toml`, the `wasm-bindgen-cli` version Cargo.lock pins, the `engines` field). Errors: a version restated in the workflow that the repository already pins is a review finding. [inferred]
- **R4:** The cargo and npm caches are keyed on the lockfiles and the toolchain file, and a rerun with no dependency change restores them. Evidence: the first green run's length and one cached rerun's length are recorded in the spec's evidence directory. Errors: no error surface beyond a cache miss, which only costs time. [inferred]
- **R5:** The workflow sets no secret. `TYPESAFE_API_KEY` is absent on the runner and the jev tests pass on the mock transport. Errors: a workflow step that reads a repository secret is a review finding. [inferred]
- **R6:** The workflow is green on master, observed on GitHub after the merge, before the spec closes. Errors: a red run on master keeps the spec open. [paraphrase]

## Boundaries
<!-- scope: business -->

- The browser suites (`rust:test:wasm`, `test:render`) and the species QA command do not run in this workflow. The render suite needs a hardware adapter; the binding suite would add a browser install to every run and is a later line when the owner wants it. [inferred]
- `cargo fmt --check` and `cargo clippy` are not gates in this spec. Formatting is red on three files today; adding the gate is a one-line follow-up once the files are formatted. [inferred]
- No deployment, release, or package publish. The owner asked for tests in Actions; "cd" in the first message is not a delivery pipeline in this spec. [paraphrase]
- No change to any test, build script, or package script. If a command cannot run on the runner as written, the spec stops with the reason instead of editing the command. [inferred]

## Decision Context
<!-- scope: both -->

One workflow file that calls the package scripts, and nothing that duplicates them, because the README already names the check sequence and a second copy of the commands drifts. The GPU suites stay in the Rust run rather than being excluded by name, because they already skip cleanly and an exclusion list is a second place that rots. Caching is in scope because the release profile with LTO makes the uncached build the whole run time, and a workflow nobody waits for is a workflow nobody reads. [inferred]

Rejected: a matrix over operating systems (one Linux job is what the owner runs), a nightly schedule (push and pull request cover every change), and running the browser suites with a software adapter (the renderer refuses SwiftShader by design). [inferred]
