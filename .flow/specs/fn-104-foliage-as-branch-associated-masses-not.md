# Foliage as branch-associated masses, not a veil

## Conversation Evidence

> user (2026-09-22): "let's mint them specs but check how things are correlated. Maybe there's an overarching issue that fixes things more elegantly"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The reviewer's first blocker in every all-view review of the beech (fn-68, 44 rounds): foliage "forms a continuous veil across broad shoulders" where the photographs show overlapping branch-associated masses with dark recesses between them. The canopy dials that place foliage (clump system order, limb clumping, shoot radius, tip clump) moved in the adopted bundles and the veil stayed. [paraphrase]

fn-103 names the likely cause: foliage-bearing wood is chosen by a radius threshold, and an even fork split makes every branch cross it at the same distance from the trunk, so foliage switches on as one shell. This spec is the check and, only if the check fails, the fix. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Step one is a measurement, not a parameter.** On fn-103's tree at the beech's seed 1 with an authored lateral share, render the matched whole-crown still and ask the reviewer's sheet the one question it already answers: closest to the reference on "crown shape and foliage organization", graded against the fn-68 round-42 tree. Code records the grade. [host design]
- **If the masses appear**, this spec closes with that evidence and no code change. [inferred]
- **If they do not**, the gap is in placement: `foliage/placement.rs` keys on `shootRadius` and the clump rows, and none of them leaves a gap between limb systems on purpose. The fix then is one rule that clears foliage within a distance of a structural axis, as a share of the crown, so masses hang off lateral systems and the axes read between them. One row, default zero, byte-identical. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The measurement above is run and recorded with stills, seed, overlay and the reviewer's grade, before any parameter is added. Errors: a grade taken without fn-103 landed is not this measurement.
- **R2:** If a parameter is added, it defaults to byte-identical output on every shipped preset, is range-checked and carries a dial-table row.
- **R3:** The owner's eye on the whole-crown still decides; the sheet's grade is evidence, not acceptance.

## Boundaries
<!-- scope: business -->

- Depends on fn-103. No species branch. Bark and droop are fn-106 and fn-105.

## Strategy Alignment

- Serves "Growth and botanical fidelity". [strategy:Growth and botanical fidelity]
