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

Close up, limbs pass through one another, meet in hard creases, and several stop in a flat disc [user]. The tree reads as a pile of overlapping objects rather than one grown thing, and these defects are the first thing visible at hero-shot distance.

The interpenetration is pervasive [paraphrase]. On the shipping presets, counting segment pairs from different runs that share no node and no near ancestor: Telperion has 365 interpenetrating pairs across 807 segments (179 nodes), Laurelin 621 across 2,441 segments (337 nodes). The deepest overlap reaches 92% of the two limbs' combined radius.

Nothing prevents it [paraphrase]. The skeleton and mesh stages have no collision, avoidance or overlap handling. Space colonization ignores existing wood, the surface skins whatever it is handed, and fork socketing only relates a child to its own parent.

The blunt ends share the cause [paraphrase]. Runs close with a fan to the ring's centre, and the code claims this is invisible because every cap is underground, buried in a parent, or a millimetre twig tip. That is no longer true: the finest wood is 790 mm across, so the cap is a near-metre disc facing the camera.

## Architecture & Data Models
<!-- scope: technical -->

- **Three defects, two stages, order matters.** Interpenetration is decided at growth; seam and cap at skinning. Resolving crossings first shrinks the seam problem, since a seam exists only where limbs meet. [paraphrase]
- **Where the fix belongs is open; planning picks after reading the colonize loop.** Growth-time avoidance is cheapest downstream (colonization already walks every node each step, so an occupancy term modifies the step rather than adding a pass). Post-skin repair must move geometry the radius solve and sweep already committed to, and a local repair would break the plait's continuity through forks. But growth-time avoidance can change the tree's character, which R6 catches. [paraphrase]
- **The tip is a termination contract, not a cap size.** A run that tapers to a point has no cap to hide, and that stays correct when the twig layer changes terminal wood. [inferred]
- **Every new term is a named parameter with a stated rail.** [user]
- **Fork socketing is not touched**; this spec is about wood meeting unrelated wood. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Avoidance must not straighten the tree.** A term strong enough to guarantee no overlap would flatten the crown into a fan; some residual interpenetration beats losing the twist, so acceptance asks for a large reduction, not zero. [inferred]
- **Determinism holds**: avoidance is a function of seed and parameters. [paraphrase]
- **The baseline is taken at the shipping branching depth and re-taken when it changes.** 365 and 621 were measured at one step per 2.2% of height; a deeper tree moves baseline and target. [paraphrase]
- **The pair count belongs in the repo as a test**, not a scratch script. [paraphrase]
- **Assert on every vertex against the surface as drawn**, never centrelines against analytic radii: the recorded lesson from the fork-junction interpenetration. [paraphrase]
- **Cost is bounded by the standing principle.** All-pairs is quadratic (Laurelin already has 2,441 segments; the depth spec measured 16,842 nodes at a 0.89 m step on Telperion), so growth-time avoidance needs a spatial structure sized for the deeper tree. [paraphrase]
- **Judged in clay** at the close distance the defect shows from. [paraphrase]

## Acceptance Criteria

- **R1:** Limbs no longer grow through limbs they are not related to, by a mechanism this spec deliberately leaves to planning. Whatever that mechanism is, its strength is a named parameter with a documented rail, and its neutral setting reproduces today's trees byte-identically so the change is always attributable. Errors: non-finite parameters fall back to documented defaults through the same rail idiom the other stages use; a degenerate skeleton with fewer than two nodes is unaffected. [user]
- **R2:** Interpenetrating limb pairs fall by at least 80% on both presets, measured against a baseline re-taken at the branching depth the presets ship at rather than against the 365 and 621 figures recorded here at the old depth. Errors: the count is asserted on the drawn radii rather than on centrelines, and a run that produces zero segments counts as zero pairs rather than dividing by zero. [paraphrase]
- **R3:** The interpenetration count is a test in the repo, run against both presets, so the number this spec moves cannot drift back silently. Errors: no error surface beyond R2. [paraphrase]
- **R4:** A run terminates in a tip rather than a visible flat cap, and the stated reason for the cap being invisible is true of the geometry actually produced. Errors: a run too short to taper terminates without producing degenerate triangles. [user]
- **R5:** Where limbs do still meet, they meet without a hard crease reading as two objects overlapping. Errors: no error surface beyond R1's rails. [user]
- **R6:** The tree keeps its character: the twist, writhe and crown spread the presets produce are visibly the same tree, judged in clay against a before-and-after at the same seed. Errors: a preset whose character the avoidance term changes materially is reported rather than accepted. [paraphrase]
- **R7:** Same seed and parameters yield an identical tree, and the frame budget is re-measured with the existing rig if the avoidance term changes build time materially. Errors: when the GPU timer-query extension is unavailable the panel says so and reports no timing number. [paraphrase]

## Boundaries

- **The twig layer and branching depth are separate specs**; this one adds no branching orders or depth control and inherits whatever depth they settle on. [user]
- No foliage, canopy or leaf changes. [paraphrase]
- No change to the envelope, the radius solve's conservation rule, or the plaited surface's lobes and twist. [inferred]
- No wind, animation or texturing. [paraphrase]
- Not a general mesh boolean or CSG pass; the intent is that limbs stop needing one. [inferred]

## Decision Context

### Motivation
<!-- scope: business -->

- **The owner named this from a screenshot, at the distance that matters** [user]: invisible from afar, unmissable in a hero shot.
- **It is the last structural thing between this tree and a convincing one**; wood through wood breaks the read. [inferred]
- **Measured, not polish** [paraphrase]: 365 and 621 pairs, worst at 92% of combined radius; 22% of Telperion's nodes involved.
- **The approach is deliberately not pinned.** The owner named a defect, not a mechanism, and the trade between the two stages has not been checked against the colonize loop. [user]
- **Rejected: a cap small enough to hide.** Terminal wood is metre-scale, so a smaller disc is still a disc; a tapered tip has nothing to hide. [paraphrase]
- **Zero interpenetration is explicitly not the goal**; it would straighten the crown. [inferred]

## Parked unknowns

- Whether avoidance belongs in the attraction step, the step-direction blend, or a post-step rejection; resolves on the first implementation that keeps the presets recognisable. [inferred]
- Whether residual crossings after R2 need blending at all, or R5 is met by there being far fewer. [inferred]
- What avoidance costs in build time at Laurelin's segment count. [inferred]
