# 0.2.0 clean-up from the release review

## Goal & Context

Astra's release review (2026-10-16, `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/ASTRA-RELEASE-REVIEW.md`, at F4 8dab10634), asked for by the owner ("figure out what can be cut/simplified or what we're doing stupidly"), and its incorporation into the release plan ("astra review is in, incorporate into the release plan"). These are the review's checked cuts and cheap simplifications before 0.2.0; none changes an approved tree.

Planned 2026-10-16 at `afa009bd9` (F4 with fn-257's device field merged at f018b6ed5, and fn-186 built on this branch). Two of the review's premises did not hold against the code and are restated below with the check that showed it:

- **R1:** the resident position programme has a production reader. The resident build seats its leaves' contacts by running the programme's `emit` entry on the rings (`ring_vertices`, from the resident build's rings stage when leaves are seated). Only `admit`, `radii` and `finish` serve the old comparison route alone (`begin_positions`, `complete_positions`, `emit_positions`, and `prepare_whole`'s resident branch, which production never reaches because `prepare_async` sends every resident request to `prepare_resident`). [checked]
- **R4:** fn-271 R4 says only that the gate is "declared under `telperion-heavy`"; it carries no size. The stale 20 GB lives in the convention memory note "one heavy run at a time across workers" (stacked updates reading 12 GB, 20 GB, 12 GB at 12 threads, 9 GB at 4 threads) and in fn-259/fn-237 evidence, which is history. AGENTS.md names the gate command and no size. [checked]

## Acceptance Criteria

- **R1 (no oracle in production):** production generator start creates, of the position programme, only the `emit` pipeline the resident build's ring seating reads; `admit`, `radii`, `finish` and the comparison route (`begin_positions`, `complete_positions`, `emit_positions`, `prepare_whole`'s resident branch) exist only under `cfg(test)`, where they remain the oracle the comparison tests (`route_tests`, `lobed_tests`, the generation tests' `emit_positions` case) read. CPU delivery, `ring_vertices` and the shared leaf-placement programme stay. Proof: the non-test build of the render crate and of the Wasm entry compiles with the oracle code absent (so no production path can reach it); a test that fails if the production entry table names any entry but `emit`; the seated resident build's existing tests still pass. Errors: a resident build that seats leaves without the `emit` pipeline is impossible by construction (no `expect` reachable on a production path); no new runtime switch selects entries.
- **R2 (dead experiment):** `telperion-space`'s `closed_form/envelope.rs` and `closed_form/envelope/tests.rs` (152 + 254 lines, no consumer: `rg 'StandIns|closed_form::envelope|super::envelope'` finds only themselves) are deleted with the `dead_code` allow on their module; doc comments that name the experiment (`along.rs`, `events.rs`, `kept/bound.rs`, `closed_form.rs`) are reworded; shared event/count machinery, `along.rs` and `Count::wood` (`kept/path.rs`) stay. Errors: the space crate builds and tests with no new `dead_code` warning; an item that becomes unused only because the experiment went is removed, not allowed.
- **R3 (true metadata):** `docs/pipeline.md` describes the cycle's arithmetic as built (ordinary floats on both executors, time in f64 ticks of 2^-20 cycle, a 53-bit integer draw; no `telperion_space::fixed`) and the fn-257 field as built (grown and held on the device, `query(cells)` answered on the device, no `cut`, no CPU coarse route, the slim field crate gone); `telperion-space/src/lib.rs`'s "Nothing here is wired into the pipeline" note, `package.json`'s description, and the catalogue's `FLOWERS` text say what is true (flowering is live on Telperion and Laurelin); the derived parameter docs are regenerated through the existing writer. No dial eligibility changes. Errors: the regenerated reference differs from the base only in the `FLOWERS` text; `rg 'telperion_space::fixed|until fn-257 replaces|Nothing here is wired'` over `docs/`, `crates/` and `src/` finds nothing.
- **R4 (one gate resource declaration):** AGENTS.md's gate paragraph declares the one `telperion-heavy` invocation for the gate, `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, with the revision it was measured at, its pass/fail/ignored counts and wall time, its scope (the whole workspace gate, one run, no other heavy job admitted beside it) and the rule that a different size needs a new measurement; the convention memory note is reduced to point at it, with its 20 GB update removed. Evidence under fn-259 and fn-237 is history and stays. Errors: no stale gate size remains in AGENTS.md or `.flow/memory/` (`rg '20 GB|telperion-heavy 20'` there finds nothing).
- **R5:** fn-186 (the no-geometry job tests the build without geometry) lands: the self dev-dependency without default features, geometry-only tests gated, both configurations' feature trees and test lists checked. Built and reviewed on this branch before this plan (commits 18657d5fa..03602bbdc; fn-186.1 SHIP).

## Early proof point

Task fn-287-020-clean-up-from-the-release-review.1 validates the core approach (the oracle entries can leave production while the ring seating keeps `emit`). If it fails, re-evaluate whether the position programme's WGSL must be split before continuing with fn-287-020-clean-up-from-the-release-review.2.

## Boundaries

- Release item (fn-237). fn-257's field merged at f018b6ed5, which R3's field description needs; its tasks 4-5 (fewer dispatches, compact records) remain open and touch the same generation files, so whichever lands second rebases.
- Out of scope: splitting `positions.wgsl` or the shader module (R1 saves three pipeline creations, not the module); the `position_*` metrics `prepare_whole`'s resident branch wrote, which production now leaves at zero (a protocol change for the harness readers, listed as a follow-up); `grow_coarse`/`Coarse`/`Stand` in `telperion-space`, which look consumer-less after fn-257 (follow-up); README's field section; dial eligibility; any approved tree's bytes; nextest or gate runtime (fn-271, fn-274); editing fn-259/fn-237 evidence.
- Not in this spec: `scripts/ci-key.mjs` still lists the deleted field crate's `FIELD` (fn-257's b7d7b346e), so every CI receipts job throws; reported to the host.
- The review's post-release items are fn-288..fn-290, fn-285 and fn-242.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** - removes test-only work from the renderer's production start and a dead experiment from the space crate, and makes the pipeline doc and package metadata describe the engine that ships.

## Decision Context

- R1 keeps `emit` in production because the resident build's ring seating reads it (checked); the review's "four pipelines" premise assumed all four served only the oracle. Splitting the WGSL to drop the shader module too was rejected as beyond a release clean-up.
- R4 declares 12 GB at two threads, not fn-274's 9 GiB at four: 9 GiB left about 3 % anonymous-memory headroom (fn-274), 8 GiB was killed, and 12 GB at two threads passed both fn-259's release gate (1,413 passed) and this branch's fn-186 gate at 18657d5fa (1,406 passed, 0 failed, 103 ignored). AGENTS.md is the home because hosts and reviewers read it and it already names the gate command.

## Quick commands

```bash
CARGO_TARGET_DIR=$PWD/target telperion-heavy 12 --threads 2 -- cargo test --profile ci -p telperion-render generation
CARGO_TARGET_DIR=$PWD/target cargo test --profile ci -p telperion-space --lib
TELPERION_WRITE_REFERENCE=1 cargo test --profile ci -p telperion-core --test parameter_reference
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | No oracle in production: only `emit` created; the comparison route test-only | fn-287-020-clean-up-from-the-release-review.1 | — |
| R2 | The envelope experiment deleted | fn-287-020-clean-up-from-the-release-review.2 | — |
| R3 | True metadata, derived docs regenerated | fn-287-020-clean-up-from-the-release-review.2 | — |
| R4 | One gate resource declaration | fn-287-020-clean-up-from-the-release-review.2 | — |
| R5 | fn-186 lands | — | Built by fn-186.1 on this branch (18657d5fa..03602bbdc, codex SHIP); its own spec carries it |
