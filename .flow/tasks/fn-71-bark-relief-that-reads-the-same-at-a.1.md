---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Round 2, on the owner's R4 verdict: detail now leaves at the pixel, not an octave before it. The grain is box-averaged over the pixel by an exact integral of the lattice noise (`bark_noise2_box`), the ridge band's fade runs from one ridge width a pixel to two with the shortcut moved to match, the lichen fades from two pixels across to one, a lenticel dash leaves by its length, and the tints and cavity read over the range the band's fade took out of the height, scaled by the furrow row. The birch carries a 2 mm grain at 0.15 (2.90 against 3.0) and the beech's 2 mm at 0.3 reads 1.40. A footprint sweep (`.flow/evidence/fn71/sweep.py`, curves in `sweep/res1.md` and `sweep/res4.md`) shows the beech's 4x step of 0.62 at the 2 px band gone to 0.85, the mean within 2% along the walk; what is left at 4x to 8x is the lichen and lenticel discs integrated as one edge, named in the report. Every resolution receipt is green, redraws byte-identical, frame cost 7 to 17% over round one and recorded beside it. Report: `.flow/evidence/fn71/REPORT.md`, "Round 2". The owner judges the walk in the harness on 5174.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: implement - skipped(reach: gpt-6-astra unreachable, codex usage limit until 2026-09-19 16:27; session model used)
## Evidence
- Commits: cccd12602bc4f29bde7b83adc35ea0dba979d42d, dd54d1311a426f5626180ef4de7f8f9c2701ac24, 42756e57973cd6f2955c1f5fe6c6facbc7151925, 3601b756ca85782e1305643f2b052daee5fcd234, 5beb0818d373bfefdfed23097d61037273d1e259, a9be84d1e693a9b73e1703e913471cdc1e3a46d9
- Tests: baseline: green (cargo test --release --workspace exit 0; npm run typecheck && npm test exit 0, pre-edit, round 1), rustfmt --check on the touched files (cargo fmt --all -- --check carries an inherited diff in crates/telperion-core/examples/species_measure.rs and src/params/tests.rs, untouched here), cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace (81 suites ok, exit 0; bark_distance oak 2x 2.295, 4x 2.518 <= 3.0; smooth_bark beech 1.403, birch 2.896 <= 3.0; grazing 2.914/2.684), GREEN_RECEIPT: .flow/tmp/green-receipts/5beb0818-unittest.json, npm run typecheck && npm test (85 passed; browser modules rebuilt for the dev server), headless --timing x9 under .flow/evidence/fn71/timing (round2-*), footprint sweep: uv run --with numpy --with pillow .flow/evidence/fn71/sweep.py .flow/evidence/fn71/sweep/res4 beech|birch
- PRs: