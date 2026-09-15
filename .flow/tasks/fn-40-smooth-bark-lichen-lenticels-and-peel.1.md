---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-40-smooth-bark-lichen-lenticels-and-peel.1 Implement Smooth bark: lichen, lenticels and peel

## Description
fn-32's plates left the beech's bark covered in vertical fibrous ridges and
the birch's a painted white. This task adds fourteen material rows for
lichen (an ellipsoid patch field with its own axes), lenticels (horizontal
dashes with a shallow groove) and peel (fn-32's plate network at band
values with a curl and an inner-bark tint), relief tied to trunk radius so
the thick base darkens, and a second wood pipeline so a material with none
of these rows draws without them. The beech is smooth pale grey with
lichen; the birch chalk white with lenticels and dark marks.

## Acceptance
- [x] **R1** lichen rows; the beech states them against B-BASE.
- [x] **R2** lenticel rows; the birch and beech state them.
- [x] **R3** peel rows; the birch states them against S-BARK.
- [x] **R4** wood mesh bytes and pins unchanged with every row on and off;
      distance, resolution and redraw tests unchanged; frame cost recorded
      (oak +0.006 ms within spread, birch +0.41, beech -0.40); no owner
      bound yet.
- [x] **R5** round 16 rendered and recorded in `round16-fn40/`.

## NEEDS_HUMAN — the owner's verdict on the combined round, and a choice

Commits d4b982a4 through f628d69c. Gates green. The owner on the round-16
close-ups: "from close: round 16 looks good", then "it's all still to
plasticesque", then that the matte fix is a later spec (fn-55). The beech's
bark is round 16's exactly at the owner's word; the birch's blotches were
softened lightly and both barks' roughness raised to 0.95, which the worker
measured changes the plastic look almost not at all (the renderer's
highlight, fn-55's). A reverted lichen rework (lobed, crisp patches, a fine
grain) scores far closer on both close-ups (B-BASE 0.60 against 1.22,
S-BARK 0.75 against 0.90) and is kept as `round16-fn40/lichen-lobed.patch`
for the owner to choose. Out of reach within the spec: the birch's old,
deeply fissured, peeling base, which needs bark that changes with age.

## Done summary
TBD, after the owner's verdict and the lichen choice.

## Evidence
- Commits:
- Tests:
- PRs:
