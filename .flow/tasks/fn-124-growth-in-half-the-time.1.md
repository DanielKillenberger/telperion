---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-124-growth-in-half-the-time.1 Implement Growth in half the time

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Blocker (NEEDS_HUMAN, 2026-09-24; resolved by the owner the same day: R2 restated, the envelope formula captured as its own spec)
R2's half-time gate is missed, so the task stops rather than being marked done. Commit 059f844d ships three byte-identical fixes: a hardware-sqrt port of libm's hypot, no bearing computed for an outline without lobes, and sized planner buffers. They cut the browser skeleton medians from 69.7, 76.5, 55.2 and 51.6 ms to 61.8, 65.3, 41.1 and 40.3 ms, against targets of 34.9, 38.3, 27.6 and 25.8 ms. R1, R3, R4 and R5 are met; Wasm SIMD was measured and dropped. On one Wasm thread, byte-identical, the profile names nothing that closes the rest. The oak's largest item is `pow` in `Envelope::radius_at`, 22 of 65 ms. The one fix for it is fn-91's rejected prepared envelope bounds, projected to take the oak to about 45 to 53 ms. The spruce's remainder is spread in 1 to 3 ms pieces. The owner decides between three routes: revive the envelope bounds, authorize byte-changing candidates, or restate R2 (`.flow/evidence/fn-124-growth-in-half-the-time/REPORT.md`, "Ranked remainder").

## Done summary
NEEDS_HUMAN: R2's half-time gate is missed. Three byte-identical growth fixes shipped in 059f844d: a hardware-sqrt port of libm's hypot tested bit for bit, no bearing computed for an outline without lobes, and sized planner buffers. Browser skeleton medians moved from 69.7, 76.5, 55.2 and 51.6 ms to 61.8, 65.3, 41.1 and 40.3 ms (-11 to -26%); completed frames fell 4.5 to 8%. The targets are 34.9, 38.3, 27.6 and 25.8 ms. R1 profile: native 97 to 99% and browser 98 to 99% stage attribution. R3: byte-identical output. R4: native growth -5 to -10%, the worst other stage within 3% after a serial rerun, peak RSS unchanged. R5: SIMD measured on the full setTreeGpu path, no gain, dropped. Details and the ranked remainder are in .flow/evidence/fn-124-growth-in-half-the-time/REPORT.md.

Tier: session
stage: impl-review - skipped(config: REVIEW_MODE=none)

R2 resolved by the owner on 2026-09-24: restated to the measured gain in the spec; the path to half time is fn-143 (a cheaper crown envelope).

stage: impl-review - skipped(policy: owner - review only where useful; the one numeric change, the hypot port, is pinned bit for bit against libm on 250,000 inputs, and output is byte-identical)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 059f844d887d0e835cd7169cff6f39c9d2619665, 4a6895ae655f9ba8363136a8f1fa7578538c1377
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --lib hypot, baseline: none (no Quick commands; gate run once at the end per CLAUDE.md)
- PRs: