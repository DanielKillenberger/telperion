# Every shader and every preset's size are checked without a device

## Goal

fn-248's friction (2026-10-09): two WGSL locals shadowed builtins (`floor` in `dress.wgsl`, `length` in `stations.wgsl`), which naga reports only when the shader is built on the GPU, each costing a full GPU run; and Telperion's age, set from a mean over seeds, kept 4.30M phytomers at seed 42 against the browser's 4M ceiling, caught only by `npm test` after the gate (about 20 minutes). Both are obvious fixes that change no product behaviour (AGENTS.md "Friction reports"; host spec, 2026-10-09): check every shader and every preset's size without a device.

## Decisions

- Every drawing pass's WGSL is composed once (`crates/telperion-render/src/shaders.rs`); the device and the test read the same source.
- `tests/material_shaders.rs` (a copy of the wood and foliage composition) deleted; the dump test renamed `dump_shaders`.
- Entry points validated as naga validates every entry point; names are not checked against pipeline descriptors.

## Acceptance Criteria

- **R1:** an ordinary test parses and validates every shipped WGSL module through naga as the device builds it; a shadowed builtin in a fixture fails it — met (`shader_tests.rs`; fixtures: shadowed `floor` and `length`, a reserved word, a missing return).
- **R2:** an ordinary cargo test grows each preset at its own seed and age and checks kept phytomers against `WASM_KEPT`, margin printed — met (`ceiling_tests.rs`).
- **R3:** each test under 60 s run time, measured; gate and npm test green — met (0.11 s and 5.16 s).

## Result

Margins against `WASM_KEPT` (4,000,000) at seed 42: Telperion (age 82) 3,773,761 kept, 5.7 %; Laurelin (80) 11.6 %; spruce 69.7 %; every other preset over 87 %. Red proofs: Telperion at age 88 fails (4,540,863, -13.5 %); `let floor = thick;` in `dress.wgsl` fails. The old generation-only naga test already covered `dress.wgsl` and `stations.wgsl`, so fn-248's GPU runs were most likely spent without running it; the real gain is the drawing shaders and device capabilities. Gate green except the cold-compile timing test under load (green alone, fn-237 R9); `npm test` 155 green.

## Open

None.

## Friction

- dcg hook blocked variable-path redirects and `git checkout --` -> local setup, not specced.
- `cargo nextest` not installed -> local setup.
- `cargo clippy -p telperion-render` stops on telperion-core's clippy errors -> fn-237 R7.
- Package-by-package gate breaks telperion-jev's replay test (needs core's `species_measure` example) -> fn-237 R10.
- Cold-compile test red under load -> fn-237 R9.
