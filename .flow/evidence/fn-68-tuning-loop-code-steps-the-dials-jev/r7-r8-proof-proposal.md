# Remaining R7/R8 proof proposal

This is a preflight, not authorization. No paid call, capture, cap increase, or runtime resume is granted here. Owner ranking is recorded in `priority-approval.json`. Browse `visual-review.html#approved-priorities`.

Packet checkpoint `dcc6a63fbfa782fb4970fb5a9fbf0a1228c39a65b6ec3c1a9736e7c6e02a8010`. Scope `903276d3348f3e5f40aa2c8ec25659c7d16f3880e61c559d2fabef33c985bed3`. Owner quotation: the 1,2,3 yes. Approved owner- IDs, not finding-0/1/2. Spend 552,431 / 570,000. Remainder 17,569. Visual 20 / 20 exhausted. Exact serialized numbers are in `r7-r8-offline-preflight.json`.

## Exact proposed authority, not granted

Computed from current `Live::visual_tokens_for` and `judgments::allowance` on the assembled requests.

| Item | Count |
| --- | --- |
| Jev route reservation | 5,281 |
| Owner-priority visual reservation | 41,274 |
| Token add for one R7 cycle | 46,555 |
| Proposed token cap | 598,986 |
| Proposed visual cap | 21 |
| Evaluations | +1 (stays inside 13) |
| Images for a new overlay | +4 still/twin (stays inside 52) |

Formula: visual = 40,000 + 466 ordered bytes + 552 required-cell bytes + 256. Route = 3,723 + 534 + 1,024. No reservation was cut to fit 17,569.

R7 live proof needs those additions. Current remainder cannot fund the 41,274 visual reservation. Unused 17,569 is not a retry.

## R8 supported blocker

A genuine known-positive fixture exists for a different protocol: birch-round26 whole crown, SHA `2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e`, owner-accepted silver birch at round 26, reconstructed bytes verified, durable copy under `local/fixtures/`. vision-replay-v2 already spent that pair. Sol accepted birch and rejected beech on v2 whole-crown seed 1 only.

That fixture is not a European-beech owner-priority positive. It is not reference-first. It does not cover hanging, bark, or fresh seed. P1/P2 in the previous proposal were desired target outcomes, not calibration fixtures. The current reframed beech is a known negative.

No owner-accepted European beech render exists for the approved cells. PROTOCOL requires a correct positive and a correct negative. Always-reject cannot pass. Owner-priority R8 qualification is blocked until a beech-positive fixture exists. Replaying the spent v2 pair would not qualify this protocol.

## Seed 1 versus fixed/fresh

This approval binds seed 1 B-WHOLE and B-BARE in the frozen packet, plus four owner-priority cells on those views. Pilot-config still lists B-WHOLE, B-BARE, B-BASE at seeds 1 and 42. Adding seed 42 or B-BASE would change scope and invalidate this checkpoint. Fixed/fresh verification remains a separate R8 gate. This ranking is not automatic protocol qualification.

## Remaining gates

1. **Priority (R12).** Recorded. Approval does not resolve gaps or qualify the reviewer.
2. **Qualification (R8).** Blocked: missing beech-positive fixture for approved cells.
3. **Scope.** Seed-1 packet only. Original runtime is not resumed.
4. **Accounting.** Visual 20/20. 17,569 remaining. Proposed caps above are not applied.
5. **R7 efficacy.** Needs one evaluation, four images, one visual pass, and the 46,555-token add after an owner-authorized cap change.
6. **R11.** Last diagnosed pause at progress 0.37. Frozen cuts stay.

## Minimal experiments, if later authorized

R7. One text-only Jev route over the three owner gaps (5,281 reserved, no retry). Then one measurement-first overlay. Then one owner-priority visual on the six seed-1 cells (41,274 reserved). Numeric drop alone is not success. Stop on overflow.

R8. Do not dispatch. Beech-positive fixture is missing. The birch fixture may only be reused later as the v2 whole-crown silver-birch positive, never as a beech-priority pass.
