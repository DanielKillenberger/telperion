---
satisfies: [R1, R2, R3, R4]
---
# fn-119-tuning-aims-at-every-trait-the.1 Implement Tuning aims at every trait the generator can draw

## Description
Propose every drawable inventory trait as an objective and judge each track's stride on its own objectives.

## Acceptance
Spec R1-R4.

## Done summary
`tuning/objectives.rs` proposes owner priorities then expressible core/secondary traits (`left_out` records the rest); gaps may name a track; per-track sheets and strides; empty tracks skipped. Caps 32 per approval, 16 per request. Tests in `tests/objectives.rs` and `tests/tuning_engine.rs` (the stride test was red on the old run-wide lead). Gate green (955 passed).

## Evidence
- Commits: 9819518815459f9d916001f11e24e28cd6853bb3, 0c4d8c864107609a77fd29f1baf4bf587d3fcf4e, 872fd73bf84871e40575f29cc7eff3036d6a962d
- Tests: cargo test --profile ci --workspace --no-fail-fast (955 passed)
- PRs:
