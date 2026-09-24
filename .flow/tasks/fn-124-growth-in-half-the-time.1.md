---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-124-growth-in-half-the-time.1 Implement Growth in half the time

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Blocker (NEEDS_HUMAN, 2026-09-24)
R2's half-time gate is missed, so the task stops rather than being marked done. Commit 059f844d ships three byte-identical fixes: a hardware-sqrt port of libm's hypot, no bearing computed for an outline without lobes, and sized planner buffers. They cut the browser skeleton medians from 69.7, 76.5, 55.2 and 51.6 ms to 61.8, 65.3, 41.1 and 40.3 ms, against targets of 34.9, 38.3, 27.6 and 25.8 ms. R1, R3, R4 and R5 are met; Wasm SIMD was measured and dropped. On one Wasm thread, byte-identical, the profile names nothing that closes the rest. The oak's largest item is `pow` in `Envelope::radius_at`, 22 of 65 ms. The one fix for it is fn-91's rejected prepared envelope bounds, projected to take the oak to about 45 to 53 ms. The spruce's remainder is spread in 1 to 3 ms pieces. The owner decides between three routes: revive the envelope bounds, authorize byte-changing candidates, or restate R2 (`.flow/evidence/fn-124-growth-in-half-the-time/REPORT.md`, "Ranked remainder").

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
