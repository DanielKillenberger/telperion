# The field engine's own size comes down

## Goal & Context

fn-227 (2026-10-09, SIZE.md) measured that packaging alone cannot bring `telperion-field.wasm` near 0.1.4's size: the best speed-keeping build (the full tree compiled out, `wasm-opt -O3`) is 1,064,107 bytes stripped, 332 KB over the wire with brotli, against 0.1.4's 108 KB. Half the binary is the engine (`telperion_space`, 616 KB before the optimiser), its `grow` module 241 KB with the stepped grower 117 KB. Engine-side levers fn-227 named but could not take: rayon in one-thread Wasm (~34 KB), panic and formatting code (~40 KB, needs `build-std`), and per-module weight in `grow`. Host (2026-10-09): after the release.

## Requirements

- **R1 (measure):** the field binary's size by crate and module with symbols, and which code its exports reach that a one-thread coarse field never runs.
- **R2 (cut):** each lever taken where it changes no answer: rayon out of one-thread Wasm behind a feature, unreachable code made unreachable to the linker, formatting kept out of the release path; every shipped preset's field byte-identical.
- **R3:** the over-the-wire size and the site's grow times measured against fn-227's release build; CI's ceiling lowered to the new size; gate and npm test green.

## Boundaries

Depends on fn-227. Not a release item.
