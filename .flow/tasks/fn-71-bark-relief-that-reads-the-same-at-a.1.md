---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Round 3, on the owner's second R4 verdict and the birch frames one notch apart: every amplitude fade is gone from the relief, the plate network, the lenticel dashes and the grain, and each term leaves by the box integral of its own shape over the footprint or by more reads of it inside the pixel. The ridge band and the plates are read under a ridge width and half a plate, a wider pixel shaded as three or four cells a side; a dash is a rounded bar whose box integral is a span on each axis over the cells the box reaches; the grain's window grows to six cells with its gradient from the one integral; wood reads its means only where nothing is resolvable (six widths or three plates a pixel, a twig spanning its radius). The footprint sweep (`sweep/res6.md`, `sweep/hero6.md`) is flat within 0.03 between every pair of factors on the beech base (0.94 to 0.99), the birch bark (1.03 to 0.93) and the birch at the hero pose through the owner's 1.12 notch (1.00 to 1.08, mean within 0.4%); the registered pairs read as one bark. Every resolution receipt green, redraws byte-identical. Frame cost: oak 10.0 ms, birch 15.0, beech 19.6 against round two's 5.9, 4.0, 11.4, with the terms named and the reductions listed in `.flow/evidence/fn71/REPORT.md`, "Round 3". The lichen's fade stays a follow-up. The owner judges the walk in the harness on 5174, whose modules were rebuilt.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: implement - skipped(reach: gpt-6-astra unreachable, codex usage limit until 2026-09-19 16:27; session model used)
## Evidence
- Commits: cccd12602bc4f29bde7b83adc35ea0dba979d42d, dd54d1311a426f5626180ef4de7f8f9c2701ac24, 42756e57973cd6f2955c1f5fe6c6facbc7151925, 3601b756ca85782e1305643f2b052daee5fcd234, 5beb0818d373bfefdfed23097d61037273d1e259, a9be84d1e693a9b73e1703e913471cdc1e3a46d9, 013e99814d9ecbf252f85e9ab5e5fb52a80a9236, b25fda469bdcfa51967381e92f0a734674701cc4, e8d595f6858b36d3bcfa144d0eb622de3e880995
- Tests: baseline: green (cargo test --release --workspace exit 0; npm run typecheck && npm test exit 0, pre-edit, round 1), rustfmt on the touched test files (cargo fmt --all -- --check carries an inherited diff in crates/telperion-core/examples/species_measure.rs and src/params/tests.rs, untouched here), cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace (81 suites ok, exit 0; bark_distance oak 2x 2.225, 4x 2.840 <= 3.0; smooth_bark beech 1.035, birch 2.538 <= 3.0; grazing 2.894/2.701; grain far 0.085 against 3.97 near), GREEN_RECEIPT: .flow/tmp/green-receipts/e8d595f6-unittest.json, npm run typecheck && npm test (85 passed; browser modules rebuilt for the dev server on 5174), headless --timing x9 under .flow/evidence/fn71/timing (round3-*), footprint sweeps: uv run --with numpy --with pillow .flow/evidence/fn71/sweep.py .flow/evidence/fn71/sweep/res6 beech|birch and sweep/hero6 birchhero 1.12,1.25,1.5,2,3,4
- PRs: