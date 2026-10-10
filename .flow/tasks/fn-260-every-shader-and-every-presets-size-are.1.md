---
satisfies: [R1, R2, R3]
---
# fn-260-every-shader-and-every-presets-size-are.1 Shader and preset-size checks without a device

## Description
Implement fn-260 R1-R3 as written.

## Acceptance
The spec's R1-R3.

## Done summary
Every drawing pass's WGSL is now composed once, in `crates/telperion-render/src/shaders.rs`. The device and the new crate-level `shader_tests.rs` read that same source. The test validates it together with generation's sources, under the naga capabilities wgpu grants this device. Faulty fixtures fail the check: shadowed `floor` and `length`, a reserved word, and a missing return. Separately, `ceiling_tests.rs` grows every catalogue preset at its own seed and age and checks it against `WASM_KEPT`, with the margin printed. Measurements: the shader checks take 0.11 s and the preset check 5.16 s. Telperion has the smallest margin, 5.7 %. Both checks fail on the fn-248 incidents, reproduced as temporary edits. Details are in `.flow/evidence/fn-260-every-shader-and-every-presets-size-are/CHECKS.md`.

Decisions: `tests/material_shaders.rs`, which held a copy of the wood and foliage composition, is deleted, because the new check covers both. The dump test is renamed `dump_shaders` and now writes the drawing shaders too. The new dev-dependency `wgpu-naga-bridge` is already in the lockfile through wgpu-core. Entry points are validated as naga validates every entry point in a module. Entry-point names are not checked against the pipeline descriptors.

Note: the old generation-only naga test already covered `dress.wgsl` and `stations.wgsl`, and it goes red on the shadowed `floor`. So fn-248's two GPU runs were most likely spent without running that test first. The real gain here is the drawing shaders and the device capabilities.

stage: impl-review - ran (codex, 3 draws, SHIP, no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1395d288b4b4b43c6b586f741fa6e6c518bca36f, 631ff608c57267f9d0e22e732be875bf57b928fe, c18ceed87b40ff996a4c7cfb82033b1a032c0b90
- Tests: cargo test --profile ci -p telperion-render (GPU lock; cold-compile test red under load, green alone), cargo test --profile ci -p telperion-core / -space / -field / -wasm / -jev, npm test
- PRs: