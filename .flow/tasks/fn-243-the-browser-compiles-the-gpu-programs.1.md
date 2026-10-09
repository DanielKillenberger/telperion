---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-243-the-browser-compiles-the-gpu-programs.1 Async pipeline creation on the web route, compiled at page load

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The browser now compiles every build pipeline in the background. A pinned wgpu 30.0.1 patch (`vendor/README.md`, gfx-rs/wgpu#3794) adds `create_compute_pipeline_async`. `programs.rs` routes the generation passes and the wood's nine surfacing passes through it, and the surfacing passes are made once per device. The page calls `preparePrograms` as soon as its device exists and shows "preparing GPU programs…" until it resolves. A build waits on it and never compiles. A refused program or a lost device rejects by name, and no build starts after it.

Proof:
- Rust settlement tests cover a late compile, a refused program, a lost device and the error order. A Request test checks that a tree request never undoes a preparation. The GPU test `a_build_makes_no_program_the_preparation_did_not` checks that a build makes no new pipelines.
- TypeScript tests in `src/browser/render.test.ts` cover the pending, refused and lost states.
- Gate: the render crate passed under the GPU lock, then each other package, then `npm test`.
- R4 (`R4.md`): a cold-profile Chrome run drew the beech, Laurelin and Telperion, and the GPU process did not exit. The reload time was not measured.

Follow-ups:
- The renderer's selection compute pipelines and its render pipelines are still created synchronously.
- A pending compile is not raced against device loss.
- The render module is 3,295,462 bytes against a 2,800,000-byte ceiling; this change adds 14 KB raw.

stage: impl-review - ran (codex: NEEDS_WORK, 3 findings fixed, re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 01778a1c39df5c94017b3b6fd6c17e069f442ab6, 17634456d4b207faa18d93bc0fab24ae2857f635, b23dd53bda3808e5aaacac71210b29ea42374bce, c3cf36030abdcee492a88464b1b06f0608282d56, 0fa6e1b2ae58cc54d986cc18acc8b39199f9e608
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci -p telperion-{core,wasm,field,jev,space} --no-fail-fast, npm test
- PRs: