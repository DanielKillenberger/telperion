## Conversation Evidence

> user (2026-09-15, on whether the look is achievable): "are we making good progress? optimistic we can achieve the look?"
> host (2026-09-15): "Even a perfectly shaped beech will look like a dark scatter until the canopy is lit as one mass."
> user (2026-09-15): "makes sense"
> worker, fn-46 round 8 (2026-09-15): "The leaves there sit in shadow. Even pure-white leaves reach only 78, or 81 with sky occlusion also off. Leaf pixels above half brightness are 1% of the crop in ours against 20% in the photograph ... Casting every leaf reads 31.8 against 32.0 ... Full transmission reads 35.9 ... What would reach it is ... finding out why the leaves facing the camera get no sun under a side sun."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 45% [paraphrase], 35% [inferred] -->

Every leaf-on pair in fn-34 renders the leaf mass far darker than its photograph: the beech's centre at 41 to 47 against 80 under overcast, the birch's at 32 to 35 against 83 under sun, with the leaf colours now matched to the photographs' own pixels. In a photograph a crown reads as one lit mass: the leaves on the sunward side and the outer shell are bright, a fifth of the leaf pixels sit above half brightness, and the shading falls off into the crown. In ours one per cent do. The worker measured that neither the shadow settings, the transmission, the leaf's orientation nor the leaf's colour is the cause. [paraphrase]

This spec finds why a leaf facing the camera under the matched light receives almost no light, and gives the renderer the canopy lighting that fixes it, as rows, inside the frame budget fn-29 accepted, so the beech and the birch read as lit canopies. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Diagnose first, on one pair.** On S-WHOLE (sun) and B-WHOLE (overcast), render the foliage pass with each lighting term isolated (sun diffuse, sky, transmission, specular, fn-29's crown-depth sky occlusion, the shadow map) and record per-term means over the leaf pixels. Check the leaf's shading normal against the sun for leaves visibly facing it, the sign and handedness of the back face, the exposure and tone mapping applied to foliage against wood and sky, and whether the matched shot's scene row (`overcast` scaling the sun) is applied to foliage as to wood. Record the findings, with numbers, in `.flow/evidence/fn52/REPORT.md`; a fault found here (a sign, a missing term, a unit) is fixed as a bug, not tuned around. [inferred]
- **Canopy lighting as rows.** Whatever remains after the faults is the missing physics of a leaf mass, and it lands as rows on the canopy or material: the standard foliage terms are a shading normal bent toward the crown's outward direction (so the mass shades like a volume and not a spray of cards), light wrapped around the terminator, and a leaf's forward transmission and sheen. Each is a row, neutral reproducing today's frame. [inferred]
- **Both renderers.** The native and browser paths take the same shader; the clay view is untouched. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Every pinned still renders byte-identically at the rows' neutral values unless a found bug is fixed, in which case the pins move once with the bug named. [paraphrase]
- **Frame budget.** The oak's native hero frame stays at or under fn-29's accepted 3.98 ms p50 and the browser orbit holds 60 fps, recorded beside fn-29's numbers. [paraphrase]
- **No species branch.** Rows on the family only. [strategy:Surface and rendering at scale]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The per-term diagnosis on S-WHOLE and B-WHOLE is recorded with numbers, and every fault it finds is fixed and named. Errors: none beyond the record. [inferred]
- **R2:** The canopy-lighting rows exist with rails, neutral reproducing today's frame, refused by name outside them, on the wire, blended, in the regenerated metadata and on the harness, in the native and browser renderers. Errors: a value outside its rail is refused naming the field. [inferred]
- **R3:** The beech's and the birch's rows are set so that on B-WHOLE and S-WHOLE the leaf-pixel brightness distribution approaches the photograph's (the share above half brightness and the centre mean within about 10), the pairs are rendered again with the numbers beside the previous round's, the implementer does visual QA before returning (does the crown read as one lit mass, yes or no), and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The frame cost stays within fn-29's accepted bound, recorded. Errors: a number over the bound stops the spec with the number. [paraphrase]
- **R5:** The tests cover: neutral byte identity on the pinned stills, the rails, each term's direction on a synthetic leaf facing and facing away from the sun, native and browser agreement, and the diagnosis's fixed faults. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No new light, no global illumination, no change to the shadow map's resolution. [inferred]
- No leaf geometry or placement; fn-50 owns short shoots. [paraphrase]

## Resolved via Codebase

- fn-29's accepted terms and frame numbers: `.flow/specs/fn-29-colour-cavity-and-occlusion.md` (R4 crown-depth sky occlusion; R6 the oak at 3.9823 ms p50, accepted).
- The foliage shader and scene rows: `crates/telperion-render/src/shaders/` (foliage and common), `crates/telperion-render/src/scene.rs`, `scene/frame.rs`.
- The leaf material rows: `crates/telperion-core/src/material.rs` (leaf front and back colours, ranges, `interior_darkening`).
- The measurements: `.flow/evidence/fn34/REPORT.md`, round 8 (fn-46), and round 6 (fn-45) for the beech's leaf-on centre.
