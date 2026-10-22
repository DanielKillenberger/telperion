# A walk's midpoint draws a tree halfway between its ends

## Goal & Context

Owner (2026-10-22), on the fn-299 still of oak to palm at 40, seed 1, t = 0.5: "there's no branches still". The frame is a bare, palm-like stem with a thin crown and one leafless side shoot. Almost none of the oak shows halfway through the walk. The film walks through it.

What is measured (fn-299 `R1.md`, 2026-10-21): age 2's blended `lifespan` (a `Log { floor: 1 }` blend) is 43 years at t = 0.5, 117–150 years from t = 0.9 on, and 136.6 cycles at t = 0.962.

What is inferred, not traced: the tree is 40 years old, so its main axis never leaves age 2 at t = 0.5. The oak's branching ages are therefore never entered, and the drawn tree has almost no laterals. If that holds, the walk's rows change by degree but the drawn tree does not. It keeps the oak's branching only while age 2's blended lifespan is under the tree's age, and then drops it. That would be a cliff in t, which breaks the owner's "change by degree" rule (fn-277, host ruling 5a) for what is drawn.

## Unknown, check before the plan

- Whether age 2's lifespan is the whole cause. Trace the t = 0.5 frame: which ages its axes enter, and why each lateral that the oak would grow is absent.
- The curve, measured: the drawn wood in laterals (share of wood length in axes of order ≥ 1) and the lateral count, against t, over the oak-to-palm walk at seeds 1, 7 and 42, at a resolution that shows any cliff (e.g. 41 frames). Do the same for the other five film walks at a coarser step, to find whether this is one walk's defect or the blend's.
- The oak's and the palm's own age-2 `lifespan`, and each lifespan row's blend kind.

## Requirements

- **R1 (measure):** the curves and the trace above, before any rule is proposed. Stills at t = 0.25, 0.5 and 0.75 for oak to palm, seed 1.
- **R2 (rule, host's call):** if the cliff is confirmed, propose up to three candidate rules, each with its curve and its stills, and bring them to the host. The host rules. Candidates the host expects to see measured:
  - blending a stage's rate (1/lifespan) instead of its lifespan;
  - weighting the lifespan blend by the share of the tree's age each end spends in the stage;
  - anything R1's trace shows to be the real cause.
  - No candidate switches between the ends' rows, adds a species branch, or grows two trees and mixes them. One pipeline, rows by degree (`docs/principles.md`).
- **R3:** the ruled rule lands. The midpoint curves change by degree, meaning no step in the lateral share larger than the steps at the walk's ends. fn-299's guards hold: the units within the ends' max + 2, and the walks' laws. The ends are byte-identical. Digests move only where the rule says, measured.
- **R4:** the gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task. The owner sees the before and after stills.

## Boundaries

- Not fn-300 (CPU and device agreement) and not fn-305.
- Joins the release (fn-237), because the release film walks through this frame.
