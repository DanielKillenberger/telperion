# Each stage's output has one form, and its consumers read it

## Goal & Context

The owner (2026-10-16), after the field turned out to rebuild its own copy of the drawn tree: "we defined we wanted a clean pipeline with consumers that take the output from previous stages" and "how much clearer do i need to be?". The host's audit (`PIPELINE-AUDIT.md` beside this spec; checked by grep and an adversarial reviewer, no measurements) found eighteen places in the release where a consumer rebuilds a stage's output, a second chain re-derives it, or code no production path reaches is compiled in. The field itself (finding 1, both executors) is fn-257's R8.

The audit was taken at `d324db4fb`, before fn-257 landed. Re-read at `f8c014681` (plan, 2026-10-18): every finding in this spec's scope is still present; fn-257 kept the leaf plan (`Plan::descriptors`) as the CPU field's leaf input, so finding 12 stays live, and the device field (`field.wgsl`) became a third reader of the sweep's 39-word node table (finding 2).

## Acceptance Criteria

Every change keeps every shipped tree byte-identical (presets at their ages, seeds 1, 7, 42) unless a finding says otherwise; each is measured where it costs time or memory.

- **R1 (copies in the drawing path):** the device sweep reads the drawn records instead of copying them into its 39-word node table (finding 2); the sweep writes the renderer's curve layout directly, without a handoff pass (finding 3, re-questioning fn-212 decision 2 with its reason); a mesh request sweeps the wood once (finding 7). Errors: a device or capacity failure stays a named error; every reader of the node table (stations, rings, the device field) moves with it, so no reader is left on a removed layout.
- **R2 (retired and parallel routes out of production):** the scaffold grower and `validate_skeleton`'s scaffold preconditions leave production, and `Tree.pipe`'s fallback goes (finding 4; fn-288 keeps the test migration); the experimental growth passes and their hooks in the production grower go (finding 5); the CPU delivery route goes if nothing in production reaches it (finding 6); test-only WGSL leaves the production modules (finding 10); the zero-reporting branch diagnostics are deleted (finding 8). Errors: a family the engine cannot grow is still refused by name (`no engine species`), never grown another way; validation keeps every check a production row needs.
- **R3 (one derivation of each quantity):** the station count is called, not restated (finding 12); one preparation of an `Expansion` (finding 13); the kept recursion and its reduced predictor, the node-count prediction in core, the unquantised light field and production-compiled test-only items are each kept only where production reads them (findings 11, 15, 16, 17). Errors: no surface beyond the existing refusals.
- **R4 (carried-over contracts):** the spec, README and package requirements the audit lists as preserving a pre-engine format without a named owner (fn-257 R1/R1a beside R8, the CPU field snapshot as API, the `{structure:true}` layout's scaffold column, "twelve bytes a leaf", the three compatibility rows, the inferred byte-identity requirements in fn-15, 28, 123, 125, 126, 171, 176) are reconciled: kept with a named reason or removed.
- **R5:** gate and npm test green; codex review per task; lines, bytes and build time before and after.

How the criteria are proven:

- **Byte identity:** digests of the artifacts, never of an internal field a task deletes (`Tree.pipe`): `artifact_digest` (core, every shipped preset, seeds 1, 7, 42), `generation_digest`'s pipeline lines, and a resident digest in the render crate's test binary that hashes what the renderer receives (the packed curve's point, cluster and section buffers, its demand, the placed leaves) print the same lines after each task as the baseline recorded at `f8c014681`, unless a task's finding says otherwise and the difference is reported with its measurement.
- **Measurements (R5):** non-test lines per crate, the shipped Wasm modules' bytes (raw and gzip, through the package build the CI size budget reads), and the release Wasm build's wall time, at the baseline and at the spec's head; the device build's time on the spruce and oak, paired warm over 16 runs, before and after task 6.

## Early proof point

Task fn-293-each-stages-output-has-one-form-and-its.1 validates the core approach (a recorded baseline of every shipped preset's artifacts at seeds 1, 7 and 42, on the CPU and the device, that a later task compares against byte for byte). If it fails (the digests are not repeatable run to run), re-evaluate the byte-identity proof before continuing with .2+.

## Boundaries

Release item (fn-237), after fn-257's R8. Findings 9, 14 and 18 (test-only species constructors, capability thresholds, the harness's own checks) are fn-288's and after the release.

`scripts/benchmarks/field.mjs`, `scripts/benchmarks/generation.mjs`, `generation-inputs.mjs` and `experiments/voxel-field/voxels.mjs` still call the field query fn-257 removed, and `tests/migration/README.md` still mentions `{ field: true }` (host, 2026-10-18, fn-257 landing); fn-293 removes or migrates them.

Not in scope: the cycle, the lattice and the CPU reference cycle (`telperion_space::grow`), the predictors production reads, and the deprecated wire rows (fn-237, owner-accepted review, 2026-10-16); the field's own design (fn-257 R8). No ceiling in `scripts/artifact-budgets.json` is raised or lowered here (fn-242).

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — one lean core with small boundaries between tree state, rules and output representations: each artifact keeps one form and production carries no test or retired code.
- **Surface and rendering at scale** — "unsupported inputs remain explicit errors, never a second path": the CPU fallback in the resident generator's CPU delivery leaves production.

## Decision Context

Host readings of the requirements, from the plan's evidence (2026-10-18):

- **"Leave production" means compiled only into test binaries.** Code no test reads is deleted. Code only tests read (the scaffold, `prepare_whole`, test-only WGSL entries, `blend/size.rs`, the unquantised light field) is moved under `cfg(test)` or into the test modules, so fn-288 R1 still migrates the scaffold's 32 test files and fn-288 R2 deletes what remains. fn-288 therefore depends on this spec.
- **Findings 11 and 15 against fn-237 ("not touched: the predictors that still size and blend").** R3 keeps every predictor production reads as it is (`expected_kept`, `predictor.rs`, the `kept` helpers it calls). Only what production never reads moves out, which changes no prediction.
- **`Tree.pipe` is always empty in production** (its one writer is the scaffold's `radius/hold.rs`), so every `pipe(i)` read returns the node's radius already; reading the node radius directly is the identity.
- **The CPU delivery (finding 6)** is constructed only by `Generator::for_cpu_output*`, which only examples and tests call; the browser passes `Delivery::Resident` alone. The agreement tests keep a CPU reference: the CPU cycle (`executor::grow` and the expansion), which fn-237 keeps.
- **Finding 3** re-questions fn-212's curve decision (`CONTRACT.md:245`: "The device packs the fine curve into the renderer's own curve format, offset after the CPU's points ... The renderer reads one block and learns nothing new"). Its reason still holds (one block, the renderer's form); the offset after a CPU block does not (no CPU block since fn-256). The sweep writing the renderer's form directly keeps the reason and drops the pass. If a reader of the sweep's points needs precision the renderer's packed form drops, so the device trees cannot stay byte-identical, the task stops and brings the measured difference to the host.
- **R4.** A public wire contract (the leaf's twelve bytes, the `{structure:true}` layout, the compatibility rows) is kept with its reason named where a consumer reads it; changing one is an owner decision this spec does not take. Spec lines that are `[inferred]`, stale against the tree, or superseded by AGENTS.md "Generator evolution" (owner, 2026-09-20) are removed or marked superseded with that reason.

## Host decisions after task 5 (2026-10-18)

On task 6's probe (`BASELINE.md`, "Task 6 probe") and the open items of tasks 3 and 5:

- **Finding 3: the sweep writes the renderer's packed curve form.** Decoding the rings' frames from the packed form moves 1, 6 and 10 of about 4M spruce leaves (seeds 1, 7, 42) by at most 0.197 mm; that move is accepted under AGENTS.md "Generator evolution" (owner, 2026-09-20). Keeping the bytes by a ring buffer beside the node table (+76 MB at the spruce's peak) would keep a second layout, which this spec deletes. Conditions: the measured gain is reported (device build time, paired warm over 16 runs, spruce and oak; GPU peak bytes); stills of the spruce at seeds 1, 7, 42 before and after are compared and any visible difference reported; the affected digests are re-pinned with the measured difference written beside them. If the CPU executor then differs from the device on those leaves, it is noted for fn-289; no second path is built to match.
- **Finding 2: the dress writes every wave into one drawn-record buffer**, the stage's single output form, which the sweep's readers read directly. Task 6's files grow to the dress files it needs (`generation/cycle/`). The buffer's size comes from the existing made-count sizing (`given.rs`), with no extra readback or round trip; if one buffer needs a readback the per-wave layout avoids, or costs measurably more peak memory, the task stops and reports the numbers. A sweep-side copy of the waves into one buffer is not taken.
- **Validation (task 3's open item):** `validate_skeleton` drops its checks on the scaffold-only preconditions (`inner_envelope`, `resolved_growth`, `scaffold::forks_placed`) and keeps checks on rows a production stage reads; a family the engine cannot grow stays refused by name. If a shipped preset's refusal or a test depends on a dropped check, that check stays and its reader is named. Its own task and codex review (task 8).
- **`Delivery` and `Backend` (task 5's open item):** both are deleted; a one-variant enum is a switch with one way to build. Task 8.
- **Readiness:** the run continues without the ready flag (host, on the friction entry).

## Host decisions on the open items (2026-10-19)

After the completion review, on `CONTRACTS.md`'s Open list and task 7's follow-ups (task 9):

- **K7:** `catalogue_identity.rs`'s header is trimmed to the four digests and why they exist (a family, a walk, an override or a refusal of `Family::validate` that moves is caught); the re-pin history lives in git (`git log -- crates/telperion-core/tests/catalogue_identity.rs`). Test bodies and digests are unchanged.
- **The memory entry** `a-new-habitparams-row-changes-the-2026-09-26` is rewritten around what holds now: a new row moves the catalogue's wire tests and the generated tables; there is no specimen snapshot.
- **Stale mentions:** `scripts/benchmarks/generation.md`'s resident wood section (`Delivery::Resident`, the CPU wood fallback) and `docs/growth-path.md`'s `branching::Specimen` are corrected.
- **fn-28** stays open and untouched: the owner's call.
- Docs and comments only: focused checks, the full gate only if the gate classifier does not read docs-only.

## Quick commands

```bash
# Byte identity of every artifact (core), one preset and seed
cargo run --profile ci -p telperion-core --example artifact_digest -- oak 7 | sha256sum
# Device and CPU digests (render), every preset
DIGEST_SEEDS=1,7,42 cargo run --profile ci -p telperion-render --example generation_digest
# The gate, once at the end of a task
telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
npm test
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (copies in the drawing path): the device sweep reads the drawn records instead of copying them into its 39-word node table (finding 2); the sweep writes the renderer's curve layout directly, without a handoff pass (finding 3, re-questioning fn-212 decision 2 with its reason); a mesh request sweeps the wood once (finding 7). Errors: a device or capacity failure stays a named error; every reader of the node table (stations, rings, the device field) moves with it, so no reader is left on a removed layout. | fn-293-each-stages-output-has-one-form-and-its.4, fn-293-each-stages-output-has-one-form-and-its.6 | — |
| R2 | (retired and parallel routes out of production): the scaffold grower and `validate_skeleton`'s scaffold preconditions leave production, and `Tree.pipe`'s fallback goes (finding 4; fn-288 keeps the test migration); the experimental growth passes and their hooks in the production grower go (finding 5); the CPU delivery route goes if nothing in production reaches it (finding 6); test-only WGSL leaves the production modules (finding 10); the zero-reporting branch diagnostics are deleted (finding 8). Errors: a family the engine cannot grow is still refused by name (`no engine species`), never grown another way; validation keeps every check a production row needs. | fn-293-each-stages-output-has-one-form-and-its.1, fn-293-each-stages-output-has-one-form-and-its.2, fn-293-each-stages-output-has-one-form-and-its.3, fn-293-each-stages-output-has-one-form-and-its.5 | — |
| R3 | (one derivation of each quantity): the station count is called, not restated (finding 12); one preparation of an `Expansion` (finding 13); the kept recursion and its reduced predictor, the node-count prediction in core, the unquantised light field and production-compiled test-only items are each kept only where production reads them (findings 11, 15, 16, 17). Errors: no surface beyond the existing refusals. | fn-293-each-stages-output-has-one-form-and-its.2, fn-293-each-stages-output-has-one-form-and-its.3, fn-293-each-stages-output-has-one-form-and-its.4, fn-293-each-stages-output-has-one-form-and-its.5 | — |
| R4 | (carried-over contracts): the spec, README and package requirements the audit lists as preserving a pre-engine format without a named owner (fn-257 R1/R1a beside R8, the CPU field snapshot as API, the `{structure:true}` layout's scaffold column, "twelve bytes a leaf", the three compatibility rows, the inferred byte-identity requirements in fn-15, 28, 123, 125, 126, 171, 176) are reconciled: kept with a named reason or removed. | fn-293-each-stages-output-has-one-form-and-its.7 | — |
| R5 | gate and npm test green; codex review per task; lines, bytes and build time before and after. | fn-293-each-stages-output-has-one-form-and-its.1, fn-293-each-stages-output-has-one-form-and-its.7 | — |
