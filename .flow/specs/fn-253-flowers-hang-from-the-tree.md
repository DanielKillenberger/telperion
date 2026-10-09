# Flowers hang from the tree

## Goal & Context

The owner, looking at Laurelin and Telperion in the harness after fn-248 (2026-10-09): "flowers seem placed randomly and attached to the air. The shape looks good." Two causes are on record: fn-217's review found the inflorescence's axis and stalks are used to place florets but never drawn as connecting geometry (fn-217 ASTRA-REVIEW-DESIGN.md §5), and fn-234 measured that every node of a flowering shoot becomes a flowering site (`space.rs::convert`, `blooms.wgsl`), not the shoot tips or short shoots where racemes and clusters actually hang. fn-248 scaled the florets 3× and thinned them, which makes both visible. Host (2026-10-09): release item (the Two Trees).

## Requirements

- **R1 (measure first):** where each inflorescence is seated today (node, age, wood age, distance from the shoot tip) and what connects it to the wood, on both executors; checked against the code.
- **R2 (seated):** inflorescences seated where the species bears them, by degree: at shoot tips and short-shoot tips (fn-223's tip seating), with the flowering share choosing which; a per-age row only if sources or the brief need one; neutral values byte-identical.
- **R3 (attached):** the rachis and the pedicels drawn as thin stalks from the seat to each floret, on both executors, in the existing organ path (one draw, levels by projected size, fn-246's thinning respected); dormant where an inflorescence has no rachis. Invariant, stated before code: a stalk's two ends are its fixed seat (on the wood or the rachis) and its floret's base after every transform the floret receives (growth scale, presence, season, taper, scatter, projected-size selection and fn-246's thinning enlargement), so a stalk is drawn between endpoints computed after those transforms, never scaled with the floret around its origin; the representation (stalks as their own instances, or the floret anchored at its stalk's end) is chosen and stated in the design.
- **R4 (proved):** close and mid stills of both Two Trees, viewed, every floret visibly hung from wood; a test that every drawn floret connects to wood through its stalk, on both executors, with non-unit floret scales, active thinning and partial floret survival; attachment counts and per-pass timing in the evidence; CPU and device agree; frame rate held to fn-246/fn-248's; Astra's agreement; gate and npm test green; the owner's verdict.

## Boundaries

Flower seating and stalks. Release item for fn-237.
