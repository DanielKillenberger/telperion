---
satisfies: [R1, R2, R3, R4]
---
# fn-46-young-shoots-take-their-own-colour.1 Implement Young shoots take their own colour

## Description
Every piece of wood took the trunk's bark colour, so the birch's hanging
curtain drew in white and the crown read as frost. This task adds four
material rows (`shootRed`, `shootGreen`, `shootBlue`, `shootRadius`): wood
under the radius takes the shoot colour and blends to the bark colour by
twice it, in the wood shader beside the existing radius-based maturity
term, before mottle, cavity and occlusion. The birch states a dark
red-brown under 1 cm (VT Dendrology, Atkinson 1992, S-BARE's winter haze);
the beech an olive grey-brown under 4 mm, stated and not rendered here.
A host-authorised value pass then matched the birch's leaf faces to
S-WHOLE's own leaf pixels and lowered its interior darkening.

## Acceptance
- [x] **R1** the four rows railed and refused by name, on the wire,
      blended, on the harness, in the regenerated metadata; the five
      neutral presets' stills hash identically before and after.
- [x] **R2** the blend continuous on a synthetic cone (two broken shaders
      failed the test), mottle still acts on young wood, native and
      browser draw the same shader.
- [x] **R3** round 8 rendered and recorded in REPORT.md and
      `round8-fn46/stills.json`; the 48-case protocol unchanged.
- [x] **R4** `crates/telperion-render/tests/shoot_colour.rs` and the rail
      and wire tests.

## NEEDS_HUMAN — the verdict on round 8, and the dark leaf mass

Commits dbd848a, 9f83930, ff9859e, 2ec33ef. Gates green. The host read
S-WHOLE: the first birch that reads as a weeping birch, dark strands under
green leaves on two white stems. The leaf mass is far too dark (35 against
83), and the worker measured that no leaf row reaches it: even white
leaves read 78, one per cent of leaf pixels sit above half brightness
against the photograph's twenty, and shadow, transmission, orientation and
colour are not the cause. That is fn-52's (a leaf mass lit as a canopy).
The oak's hero frame measures 3.59 ms median against fn-29's 3.60 ms.

## Done summary
TBD, after the owner's verdict on the combined round.

## Evidence
- Commits:
- Tests:
- PRs:
