---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-150-the-slim-field-package-grows-every.1 Slim field grows every preset through the core pipeline

## Description
The spec's R1 to R5 as one task: the slim crate asks the core pipeline for the field, and the tests, npm test and 0.1.4 follow.

## Acceptance
- [x] R1: the date palm grows through the slim crate at seeds 1, 7, 1407 and 4242 (red on master: "family without a leaf plan").
- [x] R2: every shipped preset's slim field answers equal the main pipeline's field-only build, byte for byte, in Rust and between the two Wasm artifacts.
- [x] R3: the npm tests grow every entry of `PRESETS` through the slim entry, and `test:dist` does it through the built `dist/field.js`.
- [x] R4: `telperion-field.wasm` size reported; version 0.1.4; the workspace gate and `npm test` run.
- [x] R5: the main entry's `field: true` result carries `bounds`, equal to `FieldTree.bounds`.
- [x] Host design: one oriented box a leaflet. Against the real leaflet shapes the palm meets the quarter-metre targets at four seeds (99.7 % agreeing, 1.04 times the cells, every vertex covered). Capsule families are byte-identical.
- [ ] The owner judges the three-panel still (`raw/palm-field-boxes-seed1-x3.png`); the PR stays a draft until then.

## Done summary
The slim field crate now runs the one core pipeline with a field-only request, and every shipped preset grows through `telperion/field`, the date palm included.

- Root cause: `crates/telperion-field/src/grow.rs` kept its own chain. It refused a family without a leaf plan and skipped `clear_apical_twigs` and `clothe_leaf_bases`. R1 was red on master with "family without a leaf plan".
- The pipeline compiles without `geometry`: `pipeline/drawn.rs` (wood, placement, placed-field fallback) is behind it, and `pipeline/planned.rs` refuses those by name.
- The leaf plan describes a frond crown with ribbons (host decision), the plan's second primitive: a flat box swept by a segment and a side vector, pushed out by a thickness. Each frond has 3 chords with one ribbon per leaflet row, fitted to the leaflets drawn on placement's own stream (`foliage/leaflet.rs`, shared). Capsule families are byte-identical to master; the placed palm crown is byte-identical.
- Palm, quarter-metre grid, seeds 1, 7, 1407 and 4242: every leaflet vertex covered, 95.1-95.4 % of cells agree, 1.24-1.28 times the placed foliage cells.
- `telperion-field.wasm`: 347,167 B raw, 132,212 gzip and 108,609 brotli, names stripped in `build-wasm.mjs`. 0.1.3 was 355,100 / 126,663 / 104,737; unstripped it is now 384,319.
- R2 (Rust and npm, byte for byte), R3 (npm and `test:dist`) and R5 (`field.bounds` on the main entry) are covered. Version 0.1.4.
- Gate, run once: 950 passed, 2 failed, 21 ignored. Both failures were stale expectations (the palm's field source in a binding test, and the moved limit-inventory sites). They were fixed and their two targets rerun green. `npm test`: 128 passed.
## Evidence
- Commits: 281926e9, 509237f7, 461a96bf, c58482a2, 928d4bbb
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-wasm --lib, cargo test --profile ci -p telperion-core --test generation_limit_guard, npm test, npm run build && npm run test:dist
- PRs: