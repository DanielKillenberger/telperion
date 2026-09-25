---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-150-the-slim-field-package-grows-every.1 Slim field grows every preset through the core pipeline

## Description
The spec's R1 to R5 as one task: the slim crate asks the core pipeline for the field, and the tests, npm test and 0.1.4 follow.

## Blocker (NEEDS_HUMAN, 2026-09-25, third)
The tapered ribbons are built, but both of the owner round's targets are missed, and the agreement target cannot be met against the placed field. At 4 chords, with width and thickness tapered, the plan holds 1.15 to 1.18 times the placed foliage cells (target 1.10) and 95.3 to 95.5 % of cells agree (target 97.5 %), with every leaflet vertex covered. Chords 3, 5, 6 and 8 and a finer roll step do no better on both counts. `examples/palm_field_ceiling.rs` shows why agreement stalls: the exact leaflets, each leaf's own oriented box, agree with the placed field on only 94.9 to 95.3 % of cells and hold 0.56 times its cells. The placed field answers from world-aligned leaf boxes, so it inflates a diagonal leaflet. The plan holds about 2.1 times the exact leaflets' cells, and that is what makes its fronds read blunt in the 3x still (`raw/palm-field-tapered-seed1-x3.png`). The host chooses: re-base the targets on the exact leaflets, change what the placed field answers, or carry the plan closer to the leaflets (for example, finer rows or one descriptor per leaflet). The owner's eye on the still decides the look.

## Acceptance
- [x] R1: the date palm grows through the slim crate at seeds 1, 7, 1407 and 4242 (red on master: "family without a leaf plan").
- [x] R2: every shipped preset's slim field answers equal the main pipeline's field-only build, byte for byte, in Rust and between the two Wasm artifacts.
- [x] R3: the npm tests grow every entry of `PRESETS` through the slim entry, and `test:dist` does it through the built `dist/field.js`.
- [x] R4: `telperion-field.wasm` size reported; version 0.1.4; the workspace gate and `npm test` run.
- [x] R5: the main entry's `field: true` result carries `bounds`, equal to `FieldTree.bounds`.
- [x] Host design: the ribbon primitive; the palm meets the quarter-metre target at four seeds; capsules byte-identical.

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