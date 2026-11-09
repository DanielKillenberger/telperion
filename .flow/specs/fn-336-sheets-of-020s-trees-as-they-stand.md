# Sheets of 0.2.0's trees as they stand, for the owner to say what reads wrong

## Conversation Evidence

> goal, owner, 9 Nov 2026: "no i don't think we need to get back to the look of .1.4 they look a bit better because 0.2.0 looks bad. But .2 is better conceptually with the botanically accurate way of growing it just needs to be even better and grow larger trees more efficiently"
> goal, owner: "They look more organic, but some of them, like the oak, they still just don't look good enough."
> goal R6: "the run first gets from the owner, on sheets of 0.2.0's trees as they stand, what reads wrong, and measures each named fault before it changes any rule or value. 0.1.4 appears on no sheet as a target."

## Goal & Context

Child of goal fn-321, serving R6's first step. The owner says 0.2.0's trees look bad and has not yet been shown them, in this goal, in a form to mark up. This child makes the sheets and the question; it changes no rule and no value. What the owner writes on them names the faults the next children measure and fix. [paraphrase]

Premise, not yet run: the headless stills example, `scripts/contact-sheet.py` and the species run's reference photographs can produce these sheets with no change to the crates. [inferred]

## Acceptance Criteria

- **R1 (the species sheets):** for each real species preset (the oak, spruce, beech, birch, date palm and the ordinary tree) at its preset age, full cycle, on three seeds, one retained JPEG sheet per view: the grove on the website's voxel style, the whole tree, the mid view and the close view. Where the repository holds reference photographs for the species, they stand on the same sheet; where it holds none, the sheet says so. [paraphrase]
- **R2 (the old oak):** the oak, full cycle, at 120, 200 and 300 on seeds 1, 7 and 42, whole tree, one sheet, with each tree's height, trunk and crown offset from fn-332's probe printed under it. [paraphrase]
- **R3 (seen before shown):** the worker views every image a sheet holds before it is kept, and a sheet that is empty, clipped, unlit or mislabelled is remade, not passed on (`AGENTS.md`, "Read summaries, not receipts"). The host's screening notes per sheet are written beside them as observations, never as a verdict. [paraphrase]
- **R4 (the question):** the goal's log gains one question for the owner: per sheet, what reads wrong, in the owner's own words, with each sheet's path. Nothing in this child answers it. [paraphrase]
- **R5 (cost and budget):** every render runs through `telperion-heavy`, one GPU job at a time, with no forest capture; the sheets and their sizes follow `docs/evidence-retention.md`, and the time and memory each tree took are recorded. [paraphrase]
- **R6 (verification):** no crate changes; if the tooling needs a fix to draw a sheet, that fix is named and returned to the host before it is made. [inferred]

## Boundaries

- No rule, row or value of any preset changes. 0.1.4 appears on no sheet.
- Nothing merges to master, nothing is tagged or published.
- Heavy work runs through `telperion-heavy`; raw frames stay in ignored `raw/`.
- The Two Trees are not on these sheets; their look is the owner's separate judgment (STRATEGY.md, "The owner's eye"). [inferred]

## Decision Context

Settled by the goal: numbers explain a look and never decide it; only the owner's written verdict counts; the host conducts and an Opus worker builds. The old oak's known faults (fn-332: the bent trunk, the one-sided crown) are stated under its sheet so the owner need not rediscover them. [paraphrase]

## Open Questions

None.

## Result

Closed 9 Nov 2026, built by an Opus worker through the work stage; no crate changed, so no gate or review. Nineteen retained sheets (4.1 MiB): the oak, spruce, beech, birch, date palm and ordinary tree at 40 on seeds 1, 7 and 42 at whole, mid and close views beside their reference photographs, and the oak at 120, 200 and 300 with fn-332's probe under each tree. R1's grove view is not met: nothing in the repository draws the website's voxel style without a window, and none was built. The host viewed the oak, old-oak, beech and spruce whole sheets before the question went to the owner. Evidence: RESULT.md and the sheets under .flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand/.

## Friction

- The stills waited about 6 minutes for the GPU, held by a game on the owner's desk. Disposition: local setup, reported.
- No command draws the voxel style headlessly, so the grove sheet could not be made. Disposition: a line in the child that builds growth in view (goal R1), which needs that view for its benchmark and for R6's sheet.
- The space still examples take no camera option, so two clipped stills were remade through the headless renderer (10 minutes). Disposition: an obvious fix, to ride with the next child that changes the render crate.

