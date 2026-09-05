# Limbs that pass through each other, and the blunt ends they stop at

## Conversation Evidence

> user: "we also need a spec to fix cliping and overlying branches separately"
> user (on the screenshot, selecting all three): limbs passing through each other / flat truncated branch ends / the hard intersection seams
> user (on sequencing, carried): "Which of these 2 specs should add the ability to increase the branching depth? i don't see one in the parameters?"
> user (standing principle, carried): "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator."
> user (standing principle, carried): "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."
> user (standing principle, carried): "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 55% [paraphrase], 15% [inferred] -->

Looking at the tree close up, limbs pass through one another, meet in hard creases where they cross, and several stop dead in a flat disc rather than going anywhere [user]. Together these read as a pile of overlapping objects rather than one grown thing, and they are the first thing visible at the distance a hero shot is framed at.

The interpenetration is pervasive rather than occasional [paraphrase]. Measured on the presets as they ship, counting only segment pairs from different runs that share no node and no near ancestor: Telperion has 365 interpenetrating pairs across 807 segments, involving 179 of its nodes, and Laurelin has 621 across 2,441 segments involving 337 nodes. The deepest overlap reaches 92% of the two limbs' combined radius, which is two branches very nearly coincident in space.

Nothing in the library prevents it, by construction rather than by oversight [paraphrase]. A repo-wide search of the skeleton and mesh stages finds no collision, avoidance, intersection or overlap handling of any kind. Space colonization places nodes by attraction and gives no weight to wood that is already there, and the surface stage skins whatever skeleton it is handed. Fork junctions are the one place two pieces of wood are reconciled, through socketing, and that machinery only ever relates a child to its own parent.

The blunt ends are a separate defect with a shared cause [paraphrase]. Runs are closed at both ends by a fan to the ring's centre, and the code states that this is invisible because every cap is either underground, buried inside a parent, or a twig tip a few millimetres across. That last claim is no longer true: the finest wood in the skeleton is 790 mm across, so the cap that is assumed to be a speck is a disc the better part of a metre wide, facing the camera.

## Architecture & Data Models
<!-- scope: technical -->

- **Three defects, two stages, and the order matters.** Interpenetration is decided when the skeleton is grown; the seam and the cap are decided when it is skinned. Resolving the crossings first is what makes the seam question smaller, because a seam only exists where two limbs meet. [paraphrase]
- **Where the fix belongs is open, and the trade is stated rather than decided.** Growth-time avoidance is the candidate with the least downstream cost, because space colonization already walks every node each step, so an occupancy term modifies the step rather than adding a pass. Repairing intersections after skinning has to move geometry the radius solve and the sweep have already committed to, and the plait carries continuity through forks a local repair would break. Against that, growth-time avoidance is the one that can change the tree's character, which R6 exists to catch. Planning picks, having read the colonize loop. [paraphrase]
- **The tip is a termination contract, not a cap size.** A run that tapers to a point has no cap to hide. That is the cheaper reading of the defect than drawing a smaller disc, and it is the one that stays correct when the twig layer changes what terminal wood is. [inferred]
- **Every new term is a named parameter with a stated rail**, as every other term in this library is. [user]
- **The existing fork socketing is not touched.** It reconciles a child with its parent and it works; this spec is about wood that meets wood it is not related to. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **An avoidance term must not straighten the tree.** The generator's whole character is the twist and writhe the bias field produces, and a collision term strong enough to guarantee no overlap would flatten a crown into a fan. Some residual interpenetration is preferable to a tree that has stopped being this tree, and the acceptance below is written as a large reduction rather than as zero. [inferred]
- **Determinism holds.** Any avoidance term is a function of the seed and the parameters like everything else, and the same seed keeps producing the same tree. [paraphrase]
- **The baseline is taken at a stated branching depth and re-taken after it changes.** 365 pairs on Telperion and 621 on Laurelin were measured at the shipping depth of one growth step per 2.2% of height. A deeper tree puts more limbs in the same envelope, so both the baseline and the target move, and a percentage reduction measured against the old figure would be measuring the wrong tree. [paraphrase]
- **The measurement is the same instrument in both directions.** The pair count above is the metric this spec moves, so it belongs in the repo as a test rather than living in a scratch script. [paraphrase]
- **This class of bug has been fought once already, one level down.** The recorded lesson from the fork-junction interpenetration was to assert the geometric claim on every vertex against the surface as drawn, never on a centre against an analytic radius. A limb-crossing test that compares centrelines and mean radii would repeat exactly that mistake. [paraphrase]
- **Cost is bounded by the standing principle, and branching depth raises the bound.** A naive all-pairs check is quadratic in segments and Laurelin already has 2,441 of them. The depth spec measured 16,842 nodes at a 0.89 m step on Telperion alone, so the growth-time version needs a spatial structure rather than a full sweep per step, sized for the deeper tree rather than for today's. [paraphrase]
- **Judged in clay** at the distance the defect is visible from, which is close. [paraphrase]

## Acceptance Criteria

- **R1:** Limbs no longer grow through limbs they are not related to, by a mechanism this spec deliberately leaves to planning. Whatever that mechanism is, its strength is a named parameter with a documented rail, and its neutral setting reproduces today's trees byte-identically so the change is always attributable. Errors: non-finite parameters fall back to documented defaults through the same rail idiom the other stages use; a degenerate skeleton with fewer than two nodes is unaffected. [user]
- **R2:** Interpenetrating limb pairs fall by at least 80% on both presets, measured against a baseline re-taken at the branching depth the presets ship at rather than against the 365 and 621 figures recorded here at the old depth. Errors: the count is asserted on the drawn radii rather than on centrelines, and a run that produces zero segments counts as zero pairs rather than dividing by zero. [paraphrase]
- **R3:** The interpenetration count is a test in the repo, run against both presets, so the number this spec moves cannot drift back silently. Errors: no error surface beyond R2. [paraphrase]
- **R4:** A run terminates in a tip rather than a visible flat cap, and the stated reason for the cap being invisible is true of the geometry actually produced. Errors: a run too short to taper terminates without producing degenerate triangles. [user]
- **R5:** Where limbs do still meet, they meet without a hard crease reading as two objects overlapping. Errors: no error surface beyond R1's rails. [user]
- **R6:** The tree keeps its character: the twist, writhe and crown spread the presets produce are visibly the same tree, judged in clay against a before-and-after at the same seed. Errors: a preset whose character the avoidance term changes materially is reported rather than accepted. [paraphrase]
- **R7:** Same seed and parameters yield an identical tree, and the frame budget is re-measured with the existing rig if the avoidance term changes build time materially. Errors: when the GPU timer-query extension is unavailable the panel says so and reports no timing number. [paraphrase]

## Boundaries

- **The twig layer and branching depth are separate specs** and this one adds no branching orders and no depth control. It inherits whatever depth those settle on. [user]
- No foliage, canopy or leaf changes of any kind. [paraphrase]
- No change to the envelope, the radius solve's conservation rule, or the plaited surface's lobes and twist. [inferred]
- No wind, animation or texturing. [paraphrase]
- Not a general mesh boolean or CSG pass; the intent is that limbs stop needing one. [inferred]

## Decision Context

### Motivation
<!-- scope: business -->

- **The owner named this from a screenshot, at the distance it matters** [user]. These defects are invisible from far away and unmissable in a hero shot, which is exactly the framing the generator is being built for.
- **It is the last structural thing between this tree and a convincing one.** The silhouette, the taper, the plait and the twist all read; wood passing through wood is what breaks the read. [inferred]
- **The measurement is why this is a spec rather than a polish pass** [paraphrase]. 365 and 621 pairs, with the worst at 92% of combined radius, is not an artefact at the edges. Twenty-two percent of Telperion's nodes are involved.
- **The approach is deliberately not pinned here.** The owner named a defect, not a mechanism, and the two candidate stages have a real trade between them that nobody has yet checked against the colonize loop. Recording the trade and leaving the choice to planning is the honest shape for a spec captured from a screenshot. [user]
- **Rejected as the wrong cure: a cap small enough to hide.** The cap is visible because terminal wood is metre-scale, and shrinking the disc leaves a disc. A run that tapers to a tip has nothing to hide, and stays right when the twig layer changes what terminal wood is. [paraphrase]
- **Zero interpenetration is explicitly not the goal.** A term strong enough to guarantee it would straighten the crown, and the twist is the thing this generator is for. [inferred]

## Parked unknowns

- Whether avoidance belongs in the attraction step, in the step-direction blend, or as a post-step rejection. It resolves on the first implementation that keeps the presets recognisable. [inferred]
- Whether the residual crossings after R2 need any blending at all, or whether R5 is satisfied by there being far fewer of them. [inferred]
- What the avoidance term costs in build time at Laurelin's segment count. [inferred]

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
