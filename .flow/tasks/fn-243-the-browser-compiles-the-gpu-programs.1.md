---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-243-the-browser-compiles-the-gpu-programs.1 Async pipeline creation on the web route, compiled at page load

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
Every build pipeline compiles in the background in the browser: a pinned wgpu 30.0.1 patch (`vendor/README.md`, gfx-rs/wgpu#3794) adds `create_compute_pipeline_async`; `programs.rs` routes the generation passes and the wood's nine surfacing passes through it, made once per device; the page calls `preparePrograms` when its device exists and shows "preparing GPU programs…"; a build waits on it and never compiles; a refused program or lost device rejects by name.
- Tests: Rust settlement tests (late, refused, lost, error order), a Request test, GPU test `a_build_makes_no_program_the_preparation_did_not`, `src/browser/render.test.ts`.
- R4: a cold-profile Chrome run (owner) drew the beech, Laurelin and Telperion without a GPU process exit; reload time not measured.
- Follow-ups: selection compute and render pipelines still synchronous; a pending compile is not raced against device loss; render module 3,295,462 bytes against a 2,800,000-byte ceiling (+14 KB).
Impl-review SHIP after 3 fixes.

## Evidence
- Commits: 01778a1c39df5c94017b3b6fd6c17e069f442ab6, 17634456d4b207faa18d93bc0fab24ae2857f635, b23dd53bda3808e5aaacac71210b29ea42374bce, c3cf36030abdcee492a88464b1b06f0608282d56, 0fa6e1b2ae58cc54d986cc18acc8b39199f9e608
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci -p telperion-{core,wasm,field,jev,space} --no-fail-fast, npm test
- PRs:
