---
satisfies: [R1, R2, R3]
---
# fn-228-the-harness-renders-white-as-white.1 Implement the harness renders white as white

## Description
Measure what tints a neutral surface in the harness scene and re-tint the default sun to daylight balance, per the spec.

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
Default sun re-tinted (3.0, 2.85, 2.6) -> (3.078, 2.848, 2.388) at the same luminance; full daylight on a neutral floor renders neutral, white and grey; shade stays cool (host decision, 2026-10-08; R2 amended). `tests/neutral_light.rs` locates the cause (sky ambient) and asserts the sunlit bar. Stills of four species viewed; birch stem 0.93-0.94 against 0.92, leaves 0.54-0.63 against 0.46. Follow-up: `tuning/matched.rs` keeps the old sun. Review: codex SHIP.

## Evidence
- Commits: 5714dd474d2e053623507a450f5f44df98b26009, dd3ac28500d2ef724b9f6066a31595c04402b7ee, 3fb1341df3bafdda5bf705414ba6b40045e0e269
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock, exit 0), cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render (exit 0), npm test (13 files, 143 tests)
- PRs: