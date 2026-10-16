# The browser compiles the GPU programs in the background

## Goal

After fn-235, the owner's first harness load in a freshly restarted Chrome on 2026-10-09 still ended in "the GPU device was lost: A valid external Instance reference no longer exists" on the first build. fn-235 had measured the `grow` pipeline at 12-17 s to compile cold and kept it synchronous; Chrome compiles a synchronous `createComputePipeline` on its GPU process's main thread, where its watchdog can end the process. WebGPU cannot ship precompiled programs, so the first load on a machine always compiles. Astra's review (2026-10-09): wgpu 30.0.1 has no async compute-pipeline creation; a small pinned patch to its WebGPU backend can await the Promise and return an ordinary `ComputePipeline`. Host (2026-10-09): release blocker for fn-237. Scope: the web route's pipeline creation only; depends on fn-235.

## Decisions

- R1 confirmed with the owner (2026-10-09, fresh Chrome profile, GPU logging): the GPU process exited (`exit_code=512`) ~19 s into the first build; the Crashpad dump's crashing frame is in `gpu_watchdog_thread.cc`. The watchdog killed it; the async route stands.
- A pinned `[patch.crates-io]` of wgpu 30.0.1 (documented in `vendor/README.md`, upstream gfx-rs/wgpu#3794) adds `create_compute_pipeline_async`; native creation unchanged.

## Acceptance Criteria

- **R1:** the GPU process's exit reason on the first build recorded in one Chrome run (owner's go) — met: watchdog.
- **R2:** minimal pinned wgpu patch for browser-only async compute-pipeline creation; web route creates every pipeline through it; rejection and device loss propagated by name — met.
- **R3:** the harness starts creating pipelines as soon as the device exists, shows "preparing GPU programs" until they resolve; the first build waits and never compiles twice — met.
- **R4:** pending and failure states tested with controlled failures; native output unchanged; one cold-profile Chrome probe (owner's go) builds without device loss; second load's cache time recorded — met except the reload time, which was not measured.
- **R5:** gate green — met.

## Result

- `programs.rs` routes the generation passes and the wood's nine surfacing passes (made once per device) through async creation; the page calls `preparePrograms` when its device exists. A refused program or lost device rejects by name and no build starts after it.
- Tests: Rust settlement tests (late compile, refused program, lost device, error order), a Request test that a tree request never undoes preparation, GPU test `a_build_makes_no_program_the_preparation_did_not`, TypeScript tests in `src/browser/render.test.ts`.
- R4 probe (owner, 2026-10-09, cold profile): the beech, Laurelin and Telperion each loaded and drew; no `GPU process exited` line; R1's crash did not recur. Reload time unrecorded.

## Open

- The renderer's selection compute pipelines and render pipelines are still created synchronously.
- A pending compile is not raced against device loss.
- The render module is 3,295,462 bytes against a 2,800,000-byte ceiling; this change adds 14 KB raw.

## Friction

- Vitest's `globalSetup` builds two native release binaries before any test file (1 min 30 s cold for a 6 ms test) -> build them only when a selected file needs them (proposed).

## Evidence

- `.flow/evidence/fn-243-the-browser-compiles-the-gpu-programs/R1.md` (the Chrome crash record; cited by `vendor/README.md`)
