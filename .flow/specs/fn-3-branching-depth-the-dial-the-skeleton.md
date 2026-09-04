# Branching depth: the dial the skeleton never exposed

> **SUPERSEDED, not shipped.** This spec was merged into `fn-5-branch-until-the-tips-bear-leaves-one` before any work started. It is closed because it no longer describes work to do, not because the work was done. The merge happened because a tree is one branching recursion: splitting depth from twigs put the crossover between them in neither spec's acceptance criteria, which is how 5 mm twigs end up on 20 cm logs. The measurements and reasoning here were carried into fn-5 verbatim.


## Conversation Evidence

> user: "Which of these 2 specs should add the ability to increase the branching depth? i don't see one in the parameters?"
> user: "that would already help the leaves problem dramatically"
> user: "yes /capture that and then the clipping one"
> user (on the canopy, carried): "It looks fine and could maybe be a tree in a desert but there's not enough volume to the tree either. With this dense of a branch system i'd be expecting a full ass covered canope"
> user (standing principle, carried): "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator."
> user (standing principle, carried): "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."
> user (standing principle, carried): "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 35% [user], 50% [paraphrase], 15% [inferred] -->

The generator has no control over how deeply it branches, and the owner noticed the absence from the panel [user]. Every other property of the tree is a named dial; the number of branching orders is not one, and it is arguably the property that decides most about what a tree looks like.

The lever exists and is not reachable [paraphrase]. Growth distances are derived from envelope height by a fixed ratio: the step is 2.2% of height, the kill distance two steps, the influence radius nine. On a 148 m tree that makes every growth step 3.26 m long, which is why terminal wood is metre-scale. The ratio is stated as art direction in the code and is a reasonable default; the problem is that it is the only value the library can produce. Nothing on the panel and nothing in either preset can move it.

Moving it works, and the effect is large [paraphrase]:

| step | attractors | nodes | shoots | finest wood | triangles | full build |
|---|---|---|---|---|---|---|
| 3.26 m (today) | 1,600 | 809 | 126 | 79.8 cm | 73,192 | 194 ms |
| 1.63 m | 8,000 | 5,443 | 844 | 33.8 cm | 352,072 | 160 ms |
| 0.89 m | 8,000 | 16,842 | 2,473 | 20.6 cm | 1,081,640 | 413 ms |
| 0.44 m | 8,000 | 38,864 | 4,700 | 14.9 cm | — | — |

Attachment points go from 126 to 2,473 at a step of 0.89 m, which is most of the canopy's missing volume recovered from a value the library already computes. It also makes the tree itself better independent of foliage: a 126-run tree is a coarse tree whether or not anything grows on it.

The dial has a hard failure mode and it is currently unguarded [paraphrase]. Because the influence radius is derived as nine steps, shrinking the step shrinks the distance a growing tip can see. At a step of 0.44 m with 1,600 attractors the growth stalls: 186 nodes, 4 runs, 540 cm of finest wood and no shoots at all, silently, from a parameter set that looks reasonable. Step and attractor count are coupled and nothing says so.

## Architecture & Data Models
<!-- scope: technical -->

- **Depth is one dial over the growth ratios, not four separate ones** [inferred]. The step and the kill distance move together and their ratio is the art direction the code already states. The influence radius stops being a pure multiple of the step and becomes a function of attractor spacing as well, which is what R2 requires; that is a change to how the default is derived, not a new dial.
- **The dial belongs on the skeleton stage, beside the parameters it modifies.** It is a growth term, not an envelope term and not a foliage term. [paraphrase]
- **Both presets state it**, as they state every other term. [paraphrase]
- **`maxNodes` stops being a formality.** It is documented as a stop rather than a target and sits at 8,000, which the depth range reaches. A run that hits it is reporting that the panel asked for something it should not have, and that has to be visible rather than silent. [paraphrase]
- **Every consumer downstream scales with node count**: the radius solve, the swept surface, the shoot derivation and the canopy. The surface is the expensive one, at 1.08 M triangles where it is 73 k today. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The collapse case is designed out, not caught.** The search radius is currently nine steps, so the dial that makes branching finer also makes the growth blinder, and below some ratio the tips cannot see any target at all. Tying the radius to attractor spacing is what removes the failure rather than explaining it: a dial the owner can drag into a stump is a dial that will be dragged into a stump. [paraphrase]
- **Build time is the user-facing cost and the standing principle is that this stays snappy.** 413 ms for a full build at 0.89 m against 194 ms today is noticeable on a dragged slider. Whether that is paid on every drag or only on release is a real question this spec has to answer. [paraphrase]
- **Triangle count rises about fifteen-fold before foliage.** The rig measured the branch-only room at 0.21 ms of a 16.7 ms frame at 59 k triangles; 1.08 M is the case that finds out whether the surface stage was ever the cheap half. [paraphrase]
- **Depth multiplies the interpenetration this repo has separately measured.** More limbs in one envelope is more opportunity for wood to pass through wood, so any baseline taken for that work is taken at a stated depth. [paraphrase]
- **Determinism holds.** The same seed and the same depth give the same tree. [paraphrase]
- **Judged in clay**, and the thing being judged is whether the tree reads as more finely made rather than merely as more expensive. [paraphrase]

## Acceptance Criteria

- **R1:** Branching depth is a single named parameter on the skeleton stage, carried on the panel and stated in both presets, that scales the growth distances together while preserving the ratios the defaults encode. Errors: non-finite values fall back to the documented default through the same rail idiom the other stages use; the rail's ends are the measured range rather than round numbers. [user]
- **R2:** The starved-growth case is made unreachable rather than reported. A growing tip searches for attractors within a radius derived from how far apart the attractors actually are, not purely as a multiple of the step, so shrinking the step to branch more finely cannot shrink the search below the gap between the targets it is searching for. No combination of depth and attractor count reachable from the panel produces a stalled tree. Errors: the measured collapse at a 0.44 m step with 1,600 attractors, which yields 186 nodes, 4 runs and zero shoots today, is the regression test and it passes by growing a whole tree rather than by reporting a failure. [user]
- **R3:** Hitting the `maxNodes` stop is reported rather than silently truncating the tree, and the stop's value accommodates the stated depth range. Errors: a truncated tree is reported as truncated in the panel. [paraphrase]
- **R4:** The cost curve is measured and recorded across the depth range on both presets: nodes, runs, finest wood diameter, triangles, build time, and the GPU frame cost from the existing rig. Errors: when the timer-query extension is unavailable the panel says so and reports no timing number. [paraphrase]
- **R5:** Build behaviour at depth stays usable while a dial is dragged, by whatever mechanism the measurement shows is needed. Errors: a depth the machine cannot build interactively is reported as such rather than hanging the tab. [paraphrase]
- **R6:** Same seed and depth yield an identical tree, and depth at its default reproduces today's trees byte-identically. Errors: no error surface beyond R1's rails. [paraphrase]
- **R7:** The owner confirms in clay that the deeper tree reads as more finely made, and states the depth both presets should ship at. [user]

## Boundaries

- **The twig layer is a separate spec.** Depth gets terminal wood from 80 cm to roughly 15 to 20 cm; it does not reach leaf scale and does not try. [user]
- **Branch interpenetration is a separate spec**, and this one only notes that it raises the baseline. [user]
- No foliage, canopy or leaf changes. [paraphrase]
- No change to the envelope, the bias field, the radius solve's conservation rule, or the plaited surface. [inferred]
- No LOD ladder; a far impostor only if the measurement demands one. [paraphrase]

## Decision Context

### Motivation
<!-- scope: business -->

- **The owner found this by looking for it in the panel and not finding it** [user]. Every other property of the tree is a dial, and the one that decides how tree-like it is was derived and hidden.
- **It is the cheapest large improvement available.** The lever already exists, already has a stated rationale, and moving it takes attachment points from 126 to 2,473. Nothing else in the backlog changes the tree that much for that little. [paraphrase]
- **It reorders the work in front of it.** The twig layer's targets were written against a 133-shoot tree; on a 2,473-shoot skeleton it has far less to do and its numbers deserve re-measuring before anyone builds it. That is the reason this goes first rather than alongside. [paraphrase]
- **Rejected as insufficient: the attractor dial.** It is the density control the panel already has, and forty times the attractors buys thirteen percent finer wood. Attractors decide where the tree is asked to grow; the step decides how finely it answers. [paraphrase]
- **Rejected as too many knobs: exposing step, kill and influence separately.** Their ratios are the art direction and their coupling is exactly what produces the silent collapse. One dial that moves them together is the safer surface and loses nothing the owner asked for. [inferred]

## Parked unknowns

- Whether interactive build at depth needs debouncing, a coarse preview while dragging, or nothing at all. It resolves on the first measurement at the depth the owner actually wants. [inferred]
- What depth the presets should ship at. That is the owner's judgement in clay against the cost curve, and it is R7. [paraphrase]
- Whether the surface stage stays the cheap half at a million triangles, or becomes the thing worth optimising. [inferred]

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | TBD — populate via /flow-next:plan |
| R2 | TBD — populate via /flow-next:plan |
| R3 | TBD — populate via /flow-next:plan |
| R4 | TBD — populate via /flow-next:plan |
| R5 | TBD — populate via /flow-next:plan |
| R6 | TBD — populate via /flow-next:plan |
| R7 | TBD — populate via /flow-next:plan |
