---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-150-the-slim-field-package-grows-every.1 Slim field grows every preset through the core pipeline

## Description
The slim crate asks the core pipeline for the field; tests, npm test and 0.1.4 follow.

## Acceptance
- [x] R1: the date palm grows through the slim crate at seeds 1, 7, 1407 and 4242 (red on master).
- [x] R2: every preset's slim field equals the main pipeline's field-only build byte for byte.
- [x] R3: npm tests grow every `PRESETS` entry through the slim entry and `dist/field.js`.
- [x] R4: `telperion-field.wasm` size reported; version 0.1.4; gate and `npm test` run.
- [x] R5: the main entry's `field: true` result carries `bounds`.
- [x] Host design: one oriented box a leaflet meets the targets against the real leaflet shapes.
- [ ] The owner judges the box-field still.

## Done summary
`grow.rs`'s private chain is gone; the pipeline compiles without `geometry` (`pipeline/drawn.rs` gated, `pipeline/planned.rs` refuses placement by name). The leaf plan describes a frond crown (final form: one oriented box per leaflet). Palm agrees with the leaflets on 99.7% of quarter-metre cells; capsule families byte-identical; slim Wasm 107,758 brotli. Gate 950 passed after two stale expectations were fixed; `npm test` 128.

## Evidence
- Commits: 281926e9, 509237f7, 461a96bf, c58482a2, 928d4bbb
- Tests: cargo test --profile ci --workspace --no-fail-fast; cargo test --profile ci -p telperion-wasm --lib; cargo test --profile ci -p telperion-core --test generation_limit_guard; npm test; npm run build && npm run test:dist
- PRs:
