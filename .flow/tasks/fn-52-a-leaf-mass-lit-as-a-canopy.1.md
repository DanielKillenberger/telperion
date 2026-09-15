---
satisfies: [R1, R2, R3, R4]
---
# fn-52-a-leaf-mass-lit-as-a-canopy.1 Implement A leaf mass lit as a canopy

## Description
Every leaf-on crown rendered far darker than its photograph. The R1
diagnosis found no fault (the shading normal, one tone curve, the overcast
row all correct): each leaf was lit as a lone flat card. Transmission only
worked looking toward the sun, no sky passed through or reflected off a
leaf, the camera sees mostly undersides facing the ground half of the sky,
and few leaves both face the sun and are unshadowed. This task adds five
material rows in `canopy.wgsl`: `canopyNormal` (the lighting normal bent
toward the crown's outward direction), `lightWrap`, `diffuseTransmission`,
`leafSheen` and `crownShade` (each leaf's sky dimmed by the crown over it).

## Acceptance
- [x] **R1** the per-term diagnosis on S-WHOLE and B-WHOLE recorded in
      `.flow/evidence/fn52/REPORT.md`; no fault found, no pin moved.
- [x] **R2** five rows railed and refused by name, on the wire, blended,
      on the harness, in the regenerated metadata, native and browser; 25
      stills hash as before with every row at zero.
- [x] **R3** partially: S-WHOLE centre 35 → 58.5 (photograph 83), leaf
      pixels above half brightness 2.4% → 17.7%; B-WHOLE 45 → 65.6
      (photograph 80). Neither reaches "within about 10"; the rest is the
      birch's curtain wood in the centre crop and the beech's crown shape,
      dark front-leaf colour and the shot's exposure, all outside this spec.
- [x] **R4** the oak at 3.9836 ms total p50 with every term forced on,
      beside fn-29's accepted 3.9823 ms and a same-session base of 3.9785.
- [x] **R5** `tests/canopy_terms.rs`, `tests/canopy_light.rs`.

## NEEDS_HUMAN — the owner's verdict on the combined round

Commits 4ffab2bf, e3898efb, 5983cdb3, 4b3b3cb3, 13c6704a, 55d5023b,
9d42890c. The host read S-WHOLE: the birch's crown now reads as one lit
mass, bright on the upper shell and the sunward side, falling into shade.
The beech's leaves are lit but the crown does not read as one mass, which
is its shape (fn-50). The worker's read agrees on both.

## Done summary
TBD, after the owner's verdict on the combined round.

## Evidence
- Commits:
- Tests:
- PRs:
