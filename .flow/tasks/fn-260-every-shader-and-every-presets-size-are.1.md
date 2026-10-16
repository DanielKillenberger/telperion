---
satisfies: [R1, R2, R3]
---
# fn-260-every-shader-and-every-presets-size-are.1 Shader and preset-size checks without a device

## Description
Implement fn-260 R1-R3.

## Acceptance
The spec's R1-R3.

## Done summary
Drawing-pass WGSL composed once in `crates/telperion-render/src/shaders.rs`; `shader_tests.rs` validates it with generation's sources under the device's naga capabilities, and faulty fixtures fail. `ceiling_tests.rs` grows every preset at its seed and age against `WASM_KEPT` (Telperion smallest margin, 5.7 %). Shader checks 0.11 s, preset check 5.16 s; both fail on the fn-248 incidents. `tests/material_shaders.rs` deleted. Codex SHIP.

## Evidence
- Commits: 1395d288b4b4b43c6b586f741fa6e6c518bca36f, 631ff608c57267f9d0e22e732be875bf57b928fe, c18ceed87b40ff996a4c7cfb82033b1a032c0b90
- Tests: cargo test --profile ci per package (render cold-compile test red under load, green alone), npm test
- PRs:
