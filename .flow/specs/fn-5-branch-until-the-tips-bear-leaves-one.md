# Branch until the tips bear leaves: one recursion, trunk to twig

## Conversation Evidence

> user: "but definitionally branches should branch until the last depths are twigs that attach to leaves no?"
> user: "just to be clear this will allow for basically "realistic" 150m trees that'll fill up with whatever a tree that size amount of branches would have?"
> user: "so fn-3 and fn-2 will allow for a realistically constructed tree at that size with right amount of branches and twigs and leaves?"
> user: "Which of these 2 specs should add the ability to increase the branching depth? i don't see one in the parameters?"
> user: "that would already help the leaves problem dramatically"
> user: "there's a problem with how leaves attach though? This doesn't seem natural to me. The leaves themselves seem fine but i guess we'd smaller branches to attach them to?"
> user: "It looks fine and could maybe be a tree in a desert but there's not enough volume to the tree either. With this dense of a branch system i'd be expecting a full ass covered canope"
> user: "how would a 150m tall tree work in real life? would it have giant leaves? or would there be millions of leaves?"
> user (standing principle, carried): "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator."
> user (standing principle, carried): "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."
> user (standing principle, carried): "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 45% [paraphrase], 15% [inferred] -->

The tree stops branching about halfway down. Its finest wood is 0.79 m across and its leaves are 0.12 m long, so foliage is fastened to structural timber, and the crown carries 2,904 leaves where a tree this size carries millions [user]. The owner's framing is the one this spec is built on: branches should branch until the last depths are twigs that leaves attach to [user]. There is no point in the structure where a tree stops being a branching structure and becomes something else.

Two earlier specs proposed to reach that in two pieces, a deeper skeleton and a twig layer bolted onto its tips. This spec supersedes both, because the seam between them is where the defect would have lived [paraphrase]. A single procedural order jumping from a 20 cm limb to a 5 mm twig reproduces exactly the fault the owner spotted in the first place, one level up: growth sprouting from a log. When the recursion is continuous there is no crossover to get wrong.

Measured, the gap is about eleven fork generations [paraphrase]. This library conserves area through a fork, so a balanced fork multiplies radius by roughly 0.707. The trunk is 7.4 m and terminal wood is 0.40 m, which is about twelve generations and matches the 126 terminal runs the generator produces. Reaching a 2.5 mm twig from that trunk is a factor of about 2,960, which is twenty-three generations and on the order of eight million tips. That is the size of the real object, and it is the reason the recursion cannot be one method all the way down.

The step distance is the lever and it is unreachable [paraphrase]. Growth distances derive from envelope height at a fixed ratio, so every step on a 148 m tree is 3.26 m. Nothing on the panel or in either preset moves it. Moving it works and is the cheapest large improvement available:

| step | attractors | nodes | terminal runs | finest wood | triangles | full build |
|---|---|---|---|---|---|---|
| 3.26 m (today) | 1,600 | 809 | 126 | 79.8 cm | 73,192 | 194 ms |
| 1.63 m | 8,000 | 5,443 | 844 | 33.8 cm | 352,072 | 160 ms |
| 0.89 m | 8,000 | 16,842 | 2,473 | 20.6 cm | 1,081,640 | 413 ms |
| 0.44 m | 8,000 | 38,864 | 4,700 | 14.9 cm | — | — |

It also runs out. Each halving of the step costs about 2.3 times the nodes, so the last eleven generations by colonization would be tens of millions of nodes. That is not a browser workload and it is not a workload at all.

## Architecture & Data Models
<!-- scope: technical -->

- **One recursion, one structure, a method that changes with scale.** The tree branches from the trunk until its tips are fine enough to bear leaves. Space colonization runs while attractors are meaningful, because the questions it answers are how the crown fills its envelope and reaches for light; below that scale those questions stop deciding anything and local rules take over. The output is one skeleton, not two joined together. [user]
- **The crossover is an invariant, not an interface.** Radius, direction and taper are continuous across the change of method, and that continuity is asserted rather than assumed. This is the whole reason the two earlier specs were merged. [paraphrase]
- **Below the crossover the rules are local**: divergence, apical dominance, taper and branching ratio, each a named parameter. These are cheap because they need no attractors, no envelope query and no global solve. [inferred]
- **The thickness solve is already depth-agnostic** and conserves area through every fork regardless of how a node was produced, so continuous taper across the crossover comes free rather than being faked. [paraphrase]
- **Leaves attach where wood is leaf-scale**, which the recursion now reaches, so foliage placement stops needing a special rule about which wood is young enough. [paraphrase]
- **Both presets state the depth terms**, as they state every other term. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The starved-growth collapse is designed out.** The attractor search radius currently derives as nine steps, so a finer step blinds the growth. At a 0.44 m step with 1,600 attractors the tree stalls at 186 nodes, 4 runs and no usable tips, silently, from parameters that look reasonable. [paraphrase]
- **Element count crosses into the millions and the standing principle is that this stays snappy.** The surface is already 1.08 M triangles at a 0.89 m step before any twig or leaf. A full recursion to leaf scale is where this generator finds out what it costs. [paraphrase]
- **`maxNodes` stops being a formality** at 8,000, which the depth range passes early. [paraphrase]
- **The measurement instrument exists.** The rig from the canopy work measures GPU frame cost with timer queries at vsync off across a four-point resolution sweep, and it is what this spec's cost claims are made with. [paraphrase]
- **Depth multiplies limb interpenetration**, which is measured and specced separately; any baseline for that work is taken at a stated depth. [paraphrase]
- **Twigs below the crossover are not grown, and that is a stated approximation.** Real twigs compete for light and space; rule-generated ones do not. Scatter and phyllotaxis hide much of it, and at very close range it remains an approximation rather than a simulation. [inferred]
- **Judged in clay**, and the leaf stays the flat grey placeholder until the texturing spec. Structure is what is being judged here, not surface. [paraphrase]

## Acceptance Criteria

- **R1:** The generator branches as one recursion from trunk to leaf-bearing tips, with branching depth a named parameter carrying a documented rail, and the change of method by scale invisible in the output structure. Errors: non-finite parameters fall back to documented defaults through the same rail idiom the other stages use; a degenerate skeleton with fewer than two nodes yields no tips rather than throwing. [user]
- **R2:** Radius, direction and taper are continuous across the crossover, asserted by a test that finds no discontinuity in the radius ratio between a node and its parent, and no direction change beyond the turn limit, anywhere the method changes. Errors: a tree whose parameters put the crossover outside the grown range still passes, because a recursion with no crossover has no discontinuity to find. [paraphrase]
- **R3:** Terminal wood reaches leaf scale, measured as a leaf length that is a multiple of the diameter of the wood it attaches to rather than a fraction of it, on both presets. The baseline is today's 0.15 to 1; the target is the botanical relationship rather than a round number. Errors: no error surface beyond R1's rails. [user]
- **R4:** No combination of depth and attractor count reachable from the panel starves the growth. The attractor search radius accounts for how far apart the attractors actually are, not purely for the step, so branching more finely cannot blind the search. Errors: the measured collapse at a 0.44 m step with 1,600 attractors, today yielding 186 nodes and 4 runs, is the regression test and it passes by growing a whole tree. [user]
- **R5:** Leaf count reaches the order a tree this size carries, stated against the botanical range of 10^5 to 10^7 and measured on both presets, with the count reachable at the top of the depth rail reported. Errors: a count the frame budget cannot carry is reported as the measured ceiling rather than quietly clamped. [user]
- **R6:** The cost curve is measured and recorded across the depth range on both presets: nodes, tips, finest wood, triangles, build time and GPU frame cost from the existing rig. Hitting `maxNodes` is reported rather than silently truncating, and building at depth stays usable while a dial is dragged by whatever mechanism the measurement shows is needed. Errors: when the timer-query extension is unavailable the panel says so and reports no timing number; a depth the machine cannot build interactively is reported rather than hanging the tab. [paraphrase]
- **R7:** Same seed and parameters yield an identical tree, and the depth parameter at its default reproduces today's trees byte-identically so every change is attributable. Errors: no error surface beyond R1's rails. [paraphrase]
- **R8:** The owner confirms in clay that the tree reads as a tree of this size rather than as a coarse tree with foliage attached, and states the depth both presets should ship at. [user]

## Boundaries

- **Limb interpenetration and blunt tips are a separate spec.** This one only notes that depth raises that baseline. [user]
- No procedural leaf texturing; the flat placeholder element stands. [paraphrase]
- No wind, sway or animation. [paraphrase]
- No change to the envelope's authored silhouette, the bias field's terms, or the plaited surface's lobes and twist. [inferred]
- No LOD ladder; a far impostor only if the measurement demands one. [paraphrase]
- Not a light-competition simulation below the crossover. The approximation is stated rather than hidden. [inferred]

## Decision Context

### Motivation
<!-- scope: business -->

- **The owner's framing is the architecture** [user]. "Branches should branch until the last depths are twigs that attach to leaves" is not a preference about implementation; it is what a tree is, and a design with a seam in the middle of it was answering a smaller question.
- **The seam was where the bug would have lived** [paraphrase]. Two specs, worked at different times, would have met at a transition that was neither one's acceptance criterion. A single procedural order spanning eleven fork generations puts 5 mm twigs on 20 cm logs, which is the original defect at a different scale.
- **Rejected as unreachable: colonization all the way down.** Twenty-three fork generations is on the order of eight million tips, and each halving of the step costs 2.3 times the nodes. The method has to change; only the structure has to stay continuous. [paraphrase]
- **Rejected as insufficient: the attractor dial.** Forty times the attractors buys thirteen percent finer wood. Attractors decide where the tree is asked to grow; the step decides how finely it answers. [paraphrase]
- **Rejected as too many knobs: exposing step, kill distance and influence radius separately.** Their ratios are art direction the code already states, and their coupling is what produces the silent collapse. [inferred]
- **Rejected on the botany: larger leaves.** The owner asked whether a tree this tall would carry giant leaves or millions of small ones, and the answer settles a fork rather than being trivia. Leaf size is set by the leaf's own heat and hydraulic limits and tightens with height, so the tallest real trees carry the smallest foliage. Scaling the element up would move this tree further from a tree. [user]
- **The cost is stated rather than hidden.** This is where the generator's element count goes from thousands to millions, and the standing principle is that it stays snappy. The rig exists to say whether it did. [paraphrase]

## Parked unknowns

- Where the crossover between colonization and local rules should sit. It is a measurement against the cost curve and the clay, not a number decidable here. [inferred]
- Whether interactive build at depth needs debouncing, a coarse preview while dragging, or nothing. [inferred]
- Whether the surface stage stays the cheap half at millions of triangles, or becomes the thing worth optimising. [inferred]
- What depth the presets ship at, which is the owner's judgement in clay against the cost curve, and is R8. [paraphrase]

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
| R8 | TBD — populate via /flow-next:plan |
