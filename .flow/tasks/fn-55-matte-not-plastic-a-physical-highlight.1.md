---
satisfies: [R1, R2, R3, R4]
---
# fn-55-matte-not-plastic-a-physical-highlight.1 Implement Matte, not plastic: a physical highlight and a pixel grain

## Description
The physical highlight and the grain rows, implemented in-host (the routed gpt-6-astra bridge refused on quota).

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
One Fresnel-weighted, hemisphere-normalised highlight per material with its energy taken from the diffuse, and footprint-faded grain rows on bark and leaf; frame cost unchanged; close-ups re-rendered. The owner recalibrated the 4x distance-series bound to 3.25 (2x 2.463 <= 3.0, 4x 3.134 <= 3.25) and moved the relief's drift and the bark grain's value to fn-71; bark tables hold grain off. Owner R4 verdict: "i see barely a difference".

## Evidence
- Commits: b143f7dd24d878dc4b7b6e0451d6ae223532fa72, 45b59852ecfc9424b1f35b4aae5ee61eadeb59d6
- Tests: cargo test --release --workspace (81 suites), npm test (85), npm run typecheck, clippy, fmt
- PRs:
