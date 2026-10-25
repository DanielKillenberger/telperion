# The field engine's own size comes down

## Goal & Context

fn-227 (2026-10-09, SIZE.md) measured that packaging alone cannot bring `telperion-field.wasm` near 0.1.4's size: the best speed-keeping build (the full tree compiled out, `wasm-opt -O3`) is 1,064,107 bytes stripped, 332 KB over the wire with brotli, against 0.1.4's 108 KB. Half the binary is the engine (`telperion_space`, 616 KB before the optimiser), its `grow` module 241 KB with the stepped grower 117 KB. Engine-side levers fn-227 named but could not take: rayon in one-thread Wasm (~34 KB), panic and formatting code (~40 KB, needs `build-std`), and per-module weight in `grow`. Host (2026-10-09): after the release.

## Requirements

- **R1 (measure):** the field binary's size by crate and module with symbols, and which code its exports reach that a one-thread coarse field never runs. *(superseded by the re-scope below: `telperion-field.wasm` and the one-thread coarse field no longer exist (fn-257; `git ls-files` names neither) (fn-293 task 7).)*
- **R2 (cut):** each lever taken where it changes no answer: rayon out of one-thread Wasm behind a feature, unreachable code made unreachable to the linker, formatting kept out of the release path; every shipped preset's field byte-identical. *(superseded by the re-scope below, for the same reason as R1; a lever that changes answers is judged under AGENTS.md "Generator evolution" (owner, 2026-09-20) (fn-293 task 7).)*
- **R3:** the over-the-wire size and the site's grow times measured against fn-227's release build; CI's ceiling lowered to the new size; gate and npm test green.

## Boundaries

Depends on fn-227. Not a release item.

## Re-scope (host, 2026-10-16)

Astra's release review (2026-10-16, `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/ASTRA-RELEASE-REVIEW.md`), "After 0.2.0" 3: this spec's plan profiled a single-thread coarse-field Wasm engine and depended on fn-227, which fn-257 superseded. It is rewritten against the artifacts fn-257 ships: the final package's reachable modules and transfer sizes measured, unreachable dependencies and needless entry coupling removed, and the ceilings fn-237 raised for 0.2.0 brought back down. The 616 KB engine attribution is not a forecast for a device-held field. The three published Wasm entries stay until their consumers and a migration justify a change. The fn-227 dependency is removed. (fn-293 task 7: the package publishes three entries, `telperion`, `telperion/field` and `telperion/field/voxelize`, over two Wasm modules, `telperion.wasm` and `telperion-render.wasm`; there is no slim field module since fn-257.)
