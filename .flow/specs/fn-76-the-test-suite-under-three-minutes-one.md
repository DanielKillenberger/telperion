# The test suite under three minutes: one generation per specimen, nextest, shards

## Conversation Evidence

> user (2026-09-18, turn 1): "aren't there tools to increase rust test suites that we can use? surely there must be some software improvement that can be done not using stronger runners."
> user (2026-09-18, turn 2): "well we can /flow-next:capture this into one new spec"
> user (2026-09-18, earlier): "i like ci being below 5min. Ideally much below."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

After fn-74 the workflow skips unchanged suites and compiles in seconds, and a run where every suite executes still takes about ten minutes, eight of them test execution in the rust job. The owner wants that brought down by software means, not by a stronger runner. [paraphrase]

Where the eight minutes go, from the fn-74 runs of 2026-09-18: `tests/species.rs` is 214 seconds on the four-core runner and builds every fixed seed of every species twice to prove determinism by comparing the pair; the oak preset is generated in 22 test files, the spruce and the birch in 20 each, the beech in 14, always at the same seeds and each file in its own process; and cargo runs the 74 test binaries one after another, so the binaries with one or two tests leave three cores idle. The work is duplicated, then serialized. [inferred]

The same trees generated once per run, every test drawing on them, the binaries run in one parallel pool and partitioned across ordinary runners, is the whole change. Coverage does not move: every assertion stays, every seed stays. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Determinism by digest.** The species suite generates each fixed seed once and compares a stable digest of the result with a committed digest per preset and seed. A digest mismatch names the preset and seed. This also catches a result that differs between machines, which the in-process pair comparison never could. [inferred]
- **A per-run specimen cache.** A shared test-support crate or module builds a fixed specimen (preset, seed) once per run and stores it serialized under a key of the preset's parameter wire, the seed and a hash of the generator crate's sources, in a directory under the cargo target; every test that needs that specimen loads it from there. A key miss generates and stores; the cache is never committed and never shared between runs on different sources. [inferred]
- **One parallel pool.** cargo-nextest runs the workspace's tests in one scheduler across all binaries, installed in the workflow from a prebuilt binary, not compiled. Local commands may keep `cargo test`. [inferred]
- **Shards.** The rust job becomes a matrix of N standard runners, each running one nextest partition of the workspace, with the receipt saved once every partition is green. N is chosen from the measured per-partition time and recorded. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The cache key must move with the generator.** A change under the core crate's sources changes the key; a stale specimen can never satisfy a test after a generator change. The key includes the parameter wire so a preset value change also misses. [inferred]
- **Serialization must be lossless.** A specimen loaded from the cache must compare equal to one freshly generated; the first run asserts this once per specimen. [inferred]
- **Digests are committed values.** A deliberate generator change that moves a digest updates the committed value in the same commit, with the reason in the message; an unexplained digest move is a review finding. [inferred]
- **The receipt shape from fn-74 stays.** A partition failure saves no receipt; the receipt for the crate is saved only when every partition is green. [inferred]
- **The species suite floor per runner.** With one generation per specimen and no sharing, the species suite is about 107 seconds on four cores; sharing it across the other 21 files that build the same trees is what removes the rest. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The species suite generates each fixed seed of each preset once and proves determinism against a committed digest per preset and seed. Errors: a mismatch fails naming the preset, the seed, the expected and the actual digest. [inferred]
- **R2:** Fixed specimens are generated once per test run through a shared cache keyed on the parameter wire, the seed and the generator sources, and every core test that builds a fixed specimen reads it from that cache. Errors: a key miss generates and stores; a loaded specimen that differs from a fresh generation fails on the first run; the cache directory is ignored by git. [inferred]
- **R3:** The workflow runs the Rust suites through cargo-nextest as a partitioned matrix of standard runners, and the crate receipts are saved only when every partition is green. Errors: a failed partition fails the workflow and saves nothing. [inferred]
- **R4:** No test's assertions, seeds or coverage change; the count of tests per crate before and after is recorded and equal. Errors: a lost test is a review finding. [paraphrase]
- **R5:** Measured on real runs and recorded in the spec's evidence: a run where every suite executes finishes in under three minutes on standard runners, and a single-runner run of the whole workspace, cache warm, finishes in under five. Errors: a missed bound is recorded with the measured number, never moved. [paraphrase]

## Boundaries
<!-- scope: business -->

- No bigger or self-hosted runner; the shards are the same standard runner the repository uses today. [user]
- No change to the generator's algorithms; making generation itself faster is its own measured-slowness spec with a baseline. [paraphrase]
- No test is skipped, ignored or trimmed; the render and browser suites keep their fn-69 and fn-74 arrangements. [inferred]

## Decision Context
<!-- scope: both -->

Digest determinism before anything else, because it halves the longest suite with a ten-line change and strengthens the property it proves. The specimen cache second, because 21 files regenerate what the species suite already built and the cache removes that without touching an assertion. nextest and shards last, because they only rearrange time that is still spent, and their gain is bounded by the longest partition. Rejected: ignoring the species suite on pull requests, which trades coverage for time; caching specimens across runs, which fn-74's receipts already cover; and a larger runner, which the owner ruled out. [inferred]
