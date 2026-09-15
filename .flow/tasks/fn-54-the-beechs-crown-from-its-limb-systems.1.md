---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-54-the-beechs-crown-from-its-limb-systems.1 Implement the beech crown from its limb systems

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## Where fn-54 stopped (2026-09-16, owner shutdown)

Done, on branch fn-53-beech-crown-limb-systems:
- R1 (4cba7a0d): B-BARE measured (trunk clear to about a fifth of the height,
  a core thickest to about half and running on toward the top, limbs 25 to
  40 degrees from vertical rising straight, an upright oval); the beech's
  habit, envelope, radius and twig rows set to it. The host accepted the
  core reading over the spec's two fifths and asked for steeper limbs and an
  oval; both done (limbs 28 degrees, spread 0.36, shoulder 1.8).
- R2 and R3 (3b6f4e7f): canopy.limbClumping (thinning toward the walls
  between limb systems, growth view included) and material.lobeShade (a
  leaf-mass depth grid built once at submission, read in the vertex stage),
  both railed, wired, blended, in the regenerated metadata, with tests;
  neutral skips both. Beech cuticle gloss 0.48 -> 0.18 (owner). Measured:
  beech hero frame total p50 10.23 -> 10.24 ms with the term on, oak 5.22 ms
  (round 17: 5.236); grid build 62 ms at submission for 4.73 M leaves.
- WIP (fe9ebd4e): the host's last ask, leaf mass down to about 2 m and the
  crown widest a little below the middle: crown base 0.06, fullness 0.3,
  twig length ratio 0.23, lobe shade 0.7. Not yet looked at in the full
  runner; the species.rs comments still name the old crown base.

Left:
- Look at the e4 quick look (fn54-scratch/e4 under measure/), then
  re-run the 24-seed node check (last heaviest 199,237 on two seeds).
- R4 check: the round-18 leaf faces, transmission and sheen are untouched;
  B-WHOLE's centre read 53 against round 18's 63 at lobe shade 1.0, which is
  why 0.7 is stated now; confirm on the record.
- R5: pins re-recorded once (identity, drop, sag, strands neutral pins fail
  on the beech now), the full runner capture, compare JSONs and stills.json
  under round19-fn54/, the Round 19 section in REPORT.md, the 48-case
  protocol into measure/protocol-fn54, and every gate.
