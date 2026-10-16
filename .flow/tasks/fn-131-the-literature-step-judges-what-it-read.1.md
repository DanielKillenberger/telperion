---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8, R9, R10]
---
# fn-131-the-literature-step-judges-what-it-read.1 Implement fn-131-the-literature-step-judges-what-it-read

## Description
Make screen, select, appearance, verify and quality judge consistently and file every gap.

## Acceptance
Spec R1-R10.

## Done summary
Organ-size and cultivar screen classes; per-field select documents with keyed spans, a calibrated floor and one quantity grammar; most-probable levels; per-pointer verify claims with routine `drop-value`/`replace-source`; retired decisions stay retired; sentence ages; build-identity stage keys. Tests in `tests/judged_select.rs`, `tests/judged_verify.rs`, `floors.rs`, `tests/screen.rs`. Gate green (1018 passed).

## Evidence
- Commits: adf16058e4c80ba2d7391290657c063017b294d8, 4379bfcc3d01156ddf91096ed34f84dea81d506d
- Tests: cargo test --profile ci -p telperion-jev --no-fail-fast; cargo test --profile ci --workspace --no-fail-fast (1018 passed)
- PRs:
