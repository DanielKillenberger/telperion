---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-124-growth-in-half-the-time.1 Implement Growth in half the time

## Description
Profile growth and ship the ranked fixes toward half the skeleton time.

## Acceptance
Spec R1-R5 (R2 restated by the owner on 2026-09-24).

## Done summary
Profile attributes 97-99% of skeleton time. Three byte-identical fixes (hardware-sqrt hypot, no bearing without lobes, sized planner buffers) cut browser skeleton medians 11-26% (oak 69.7 -> 61.8, spruce 55.2 -> 41.1 ms) and completed frames 4.5-8%; the half-time gate was missed (NEEDS_HUMAN) and the owner restated R2 to the measured gain, with the envelope formula moved to fn-143. Native growth -5 to -10%, RSS unchanged; Wasm SIMD measured and dropped. Codex review fix in `4e9d30bf`.

## Evidence
- Commits: 059f844d887d0e835cd7169cff6f39c9d2619665, 4a6895ae655f9ba8363136a8f1fa7578538c1377
- Tests: cargo test --profile ci --workspace --no-fail-fast; cargo test --profile ci -p telperion-core --lib hypot
- PRs:
