---
satisfies: [R1, R2, R3, R4]
---
# fn-52-a-leaf-mass-lit-as-a-canopy.1 Implement A leaf mass lit as a canopy

## Description
Diagnose the dark leaf mass term by term, then add five material rows in `canopy.wgsl`: `canopyNormal`, `lightWrap`, `diffuseTransmission`, `leafSheen`, `crownShade`.

## Acceptance
- [x] R1 per-term diagnosis recorded; no fault found, no pin moved.
- [x] R2 five rows railed, wire, blend, harness, metadata, native and browser; 25 stills hash as before at zero.
- [x] R3 partially: S-WHOLE centre 35 to 58.5 (photo 83), above-half 2.4% to 17.7%; B-WHOLE 45 to 65.6 (photo 80).
- [x] R4 oak 3.9836 ms p50 with every term on vs fn-29's 3.9823.
- [x] R5 `tests/canopy_terms.rs`, `tests/canopy_light.rs`.

## Done summary
The leaf mass is lit as a canopy; the birch reads as one lit mass, the beech's leaves are lit but its shape keeps it from one mass. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Evidence
- Commits: 19b418f3 (branch: 4ffab2bf, e3898efb, 5983cdb3, 4b3b3cb3, 13c6704a, 55d5023b, 9d42890c)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
