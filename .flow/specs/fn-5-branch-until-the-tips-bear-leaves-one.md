# Branch until the tips bear leaves: one recursion, trunk to twig

## Conversation Evidence

> user: "can you check the spec 5 and review it. Goal is to build trees of valinor sized trees at "realistic" branching depth and structure. I want this to be performant and close to reality level fidelity. But keep the parameters that make the trees appear "magical/supernatural"."
> user: "ok you wanna /capture --rewrite the spec then?"
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

The goal, as the owner stated it on review, has three parts and this spec is held to all three [user]: Valinor-sized trees at realistic branching depth and structure; performant; close to reality-level fidelity; and the parameters that make the trees read as magical or supernatural are kept. The first draft of this spec carried the first three and dropped the fourth without noticing [paraphrase]. Everything supernatural about these trees lives in one place, the bias field: gravitropism, lean, writhe, spiral and torsion, consulted at every growth step. The draft protected those terms from being edited and never required the twigs to obey them, so a limb would writhe and its twigs would grow like an oak's. That is the seam problem once more, in the one dimension the continuity criterion does not look at, and it is now a criterion of its own.

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

- **One structure, built in two passes, with a method that changes with scale.** The tree branches from the trunk until its tips are fine enough to bear leaves. Space colonization runs to completion while attractors are meaningful, because the questions it answers are how the crown fills its envelope and reaches for light; a second pass then continues from its tips under local rules, appending into the same skeleton. Below that scale the global questions stop deciding anything. The output is one skeleton with one parent-index invariant, never two joined together, and nothing downstream can tell which pass produced a node. [user]
- **The crossover is an invariant, not an interface.** Radius, direction and taper are continuous across the change of method, and that continuity is asserted rather than assumed. This is the whole reason the two earlier specs were merged. [paraphrase]
- **Below the crossover the rules are local**: divergence, apical dominance, branching angle, internode length, children per node, taper and branching ratio, each a named parameter. These are cheap because they need no attractors and no global solve. Their resting values are botanical defaults stated with their sources, so the tree at rest is a real tree and every dial is a departure from one rather than a guess. [paraphrase]
- **The bias field shapes twigs as it shapes limbs.** Every twig node's direction passes through the same growth-bias function colonization consults, at the same position and step, so torsion, writhe, spiral, lean and gravitropism read continuously from trunk to tip. This is where the supernatural character of the trees lives, and it is the one thing R2 does not guard: a straight twig leaving a twisted limb at a legal angle passes every continuity check and still breaks the tree. [user]
- **Interior twigs are shed by the rule the crown already uses.** A real crown is a shell because twigs in deep shade die. The leaf culler already measures distance to the envelope's profile and thins interior leaves by it; the same rule thins interior twigs, one level up, at the same cost. That is not a light-competition simulation, it is the shell rule this library already has, and it buys realism and a large part of the element budget together. [paraphrase]
- **The thickness solve is already depth-agnostic** and conserves area through every fork regardless of how a node was produced, so continuous taper across the crossover comes free rather than being faked. [paraphrase]
- **Leaves attach where wood is leaf-scale**, which the recursion now reaches, so foliage placement stops needing a special rule about which wood is young enough. [paraphrase]
- **Both presets state the depth terms**, as they state every other term. [paraphrase]

### Grounded by research

- **Switching method by scale is standard practice, not a compromise.** Weber and Penn, SpeedTree and Runions all generate per branch order with different rule sets per order, and Runions' paper scopes space colonization to macro structure explicitly. What is unusual here is only the continuity assertion: a survey of open implementations found none that transitions in place and asserts continuity at the seam. R2 is the contribution; the rest is well-trodden. [paraphrase]
- **Child radius interpolates from the parent's local cross-section at the fork, not from a global taper curve.** This is the mechanism that makes R2 satisfiable rather than merely required, and it is what ez-tree does. A global curve evaluated on both sides of a method change is exactly what produces a step. [paraphrase]
- **The local stage inherits the colonization stage's exit state** as explicit boundary conditions: position, terminal tangent, radius, and remaining vigour. Reseeding direction at the crossover shows as a visible kink, and the argument is structural rather than aesthetic: a tree is a cantilever, and an abrupt direction change is a discontinuous bending moment. [paraphrase]
- **The fine orders need their own taper law.** An area-conservation exponent authored for order 0 and 1 produces visibly wrong twigs at order 8. The local law is steeper and anchored to the parent's actual radius at handoff. [paraphrase]
- **Below the crossover the recursion is level-capped, not scale-searched.** Every rule-based generator stops on a level counter sized to real twig counts. Shrinking a search radius toward zero is the failure mode, not the method. [paraphrase]
- **Keep a lightweight sibling-collision check per whorl even after leaving colonization.** Blind local replacement with no neighbour awareness is the documented weakness of pure rule systems, and this repo has a separate spec measuring exactly that defect. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The starved-growth collapse is designed out.** The attractor search radius currently derives as nine steps, so a finer step blinds the growth. At a 0.44 m step with 1,600 attractors the tree stalls at 186 nodes, 4 runs and no usable tips, silently, from parameters that look reasonable. [paraphrase]
- **Element count crosses into the millions and the standing principle is that this stays snappy.** The surface is already 1.08 M triangles at a 0.89 m step before any twig or leaf. A full recursion to leaf scale is where this generator finds out what it costs. [paraphrase]
- **Performance is a target, not a report.** The first draft measured cost and stopped there, so a tree at 40 ms a frame that reported itself honestly would have passed. The machine and the frame are already named by the canopy work: an RTX 3080, 16.7 ms at 60 Hz, the display's native pixel ratio. The depth the presets ship at renders inside that frame, and the reachable ceiling in R5 is the count at that budget rather than the count before the tab dies. [user]
- **`maxNodes` stops being a formality** at 8,000, which the depth range passes early. [paraphrase]
- **The measurement instrument exists.** The rig from the canopy work measures GPU frame cost with timer queries at vsync off across a four-point resolution sweep, and it is what this spec's cost claims are made with. [paraphrase]
- **Depth multiplies limb interpenetration**, which is measured and specced separately; any baseline for that work is taken at a stated depth. [paraphrase]
- **Twigs below the crossover are not grown, and that is a stated approximation.** Real twigs compete for light and space; rule-generated ones do not. Scatter and phyllotaxis hide much of it, and at very close range it remains an approximation rather than a simulation. [inferred]
- **Judged in clay**, and the leaf stays the flat grey placeholder until the texturing spec. Structure is what is being judged here, not surface. [paraphrase]
- **Per-instance transform memory is the ceiling nobody budgets for.** A 4x4 transform is 64 bytes, so eight million twigs is about 512 MB before a single vertex of geometry. A packed per-instance format, position plus rotation plus scale rather than a full matrix, is the difference between the top of R5's range being reachable and being arithmetic. [paraphrase]
- **This collides with the Boundary against an impostor, and the collision is stated rather than resolved.** The Boundaries below rule out an LOD ladder and admit a far impostor only if the measurement demands one. The research says the measurement will demand one somewhere inside R5's own 10^5 to 10^7 range. R6 is where that is settled with a number; nobody relaxes the Boundary by assertion, and nobody discovers it at 400 MB. [paraphrase]
- **WebGL2 has no compute and no writable storage buffers**, so instance transforms are built on the CPU and uploaded, and the upload is the bottleneck as counts rise. A WebGPU path moves that into a compute pass behind one instanced draw. Changing renderer is out of scope here and the constraint is recorded because it bounds what the top of R5's range can mean on the current backend. [paraphrase]
- **Attractor association is O(new nodes x attractors) per round today, a linear scan with no spatial index.** Every serious implementation uses a grid or kd-tree, and at the node counts this spec reaches it stops being optional. [paraphrase]
- **`InstancedMesh` count is immutable after construction in three 0.185**, so a depth dial that changes counts rebuilds the mesh rather than resizing it. `BatchedMesh` is stable in this version and does support `setInstanceCount`, and it suits many differing twig geometries where `InstancedMesh` wants identical ones. Its per-instance matrices live in a `DataTexture`, so `maxTextureSize` becomes a real ceiling at the top of the rail. [paraphrase]
- **A geometry crossing 65,535 vertices silently doubles its index memory**, since three promotes the index to `Uint32`. That belongs in the cost table rather than in a profile. [paraphrase]

## Quick commands

```bash
npx vitest run      # 268 tests green at plan time
npx tsc --noEmit    # clean at plan time
npm run build       # clean at plan time
npm run dev         # the clay harness, where R6 is measured and R8 is judged
```

## Acceptance Criteria

- **R1:** The generator branches from trunk to leaf-bearing tips as one structure built in two passes, with branching depth a named parameter carrying a documented rail, and the change of method by scale invisible in the output structure. The local rules' resting values are botanical defaults stated with their sources, and every one of them is a dial that can be driven past its resting value. Errors: non-finite parameters fall back to documented defaults through the same rail idiom the other stages use; a degenerate skeleton with fewer than two nodes yields no tips rather than throwing. [user]
- **R2:** Radius, direction and taper are continuous across the crossover. "Continuous" means exactly these three things and nothing added later by comment: the parent-to-child radius ratio at the crossover falls inside the range that same ratio takes over the ten generations above it; the direction change at the crossover is no larger than the turn limit already enforced during growth; and the taper rate, measured as radius change per unit length, is within a stated tolerance of the rate immediately above. Continuity is sampled at several generations either side of the crossover, never only at the boundary edge, and each sample is asserted against the envelope's own local dimensions rather than as a bare comparison between two numbers, so a region where both sides are zero fails the test rather than passing it. The tolerance is measured before it is chosen: the tight case is run first, what fails is recorded, and the number that ships is the one that admits only the cases a clay render cannot distinguish. The test asserts on the surface as drawn, at every vertex of the junction ring, not on skeleton centres. Errors: a parameter set whose crossover falls outside the grown range is reported as untested rather than passing vacuously, because a recursion with no crossover has no discontinuity to find and must not be mistaken for one that has none. [paraphrase]
- **R3:** Terminal wood reaches leaf scale, measured as a leaf length that is a multiple of the diameter of the wood it attaches to rather than a fraction of it, on both presets. The baseline is today's 0.15 to 1; the target is the botanical relationship rather than a round number. Errors: no error surface beyond R1's rails. [user]
- **R4:** No combination of depth and attractor count reachable from the panel starves the growth. The attractor search radius accounts for how far apart the attractors actually are, not purely for the step, so branching more finely cannot blind the search. Errors: the measured collapse at a 0.44 m step with 1,600 attractors, today yielding 186 nodes and 4 runs, is the regression test and it passes by growing a whole tree. [user]
- **R5:** Leaf count reaches the order a tree this size carries, stated against the botanical range of 10^5 to 10^7 and measured on both presets after interior twigs are shed by the shell rule, with the count reachable inside R6's frame budget reported as the ceiling. Errors: a count the frame budget cannot carry is reported as the measured ceiling rather than quietly clamped; a shell rule that sheds everything or nothing fails, by the same floor-and-ceiling guard the leaf culler carries. [user]
- **R6:** The cost curve is measured and recorded across the depth range on both presets: nodes, tips, finest wood, triangles, build time and GPU frame cost from the existing rig. The depth both presets ship at renders inside 16.7 ms at the display's native pixel ratio on the named machine, an RTX 3080, measured with the rig at vsync off. Hitting `maxNodes` is reported rather than silently truncating, and building at depth stays usable while a dial is dragged by whatever mechanism the measurement shows is needed. Errors: when the timer-query extension is unavailable the panel says so and reports no timing number; a depth the machine cannot build interactively is reported rather than hanging the tab; a shipping depth that cannot meet the frame is reported as the reason an impostor is now demanded, which is the one condition the Boundaries admit it under. [user]
- **R7:** Same seed and parameters yield an identical tree, and the depth parameter at its default reproduces today's trees byte-identically so every change is attributable. Errors: no error surface beyond R1's rails. [paraphrase]
- **R8:** The owner confirms in clay that the tree reads as a tree of this size rather than as a coarse tree with foliage attached, that Telperion still reads as Telperion and Laurelin as Laurelin with every supernatural term at its preset value, and states the depth both presets should ship at. [user]
- **R9:** The bias field shapes growth below the crossover as it shapes growth above it. Every twig node's direction passes through the same growth-bias function colonization uses, at the same position and step, so a tree's torsion, writhe, spiral, lean and gravitropism read continuously from trunk to tip; a test drives one bias term to an extreme and asserts the twigs move with the limbs. Errors: every bias term at zero reproduces an unbiased local recursion byte-identically, so the field's effect below the crossover is attributable; a preset's bias terms are stated once and inherited by no stage. [user]

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
- **The cost is stated rather than hidden, and now it is held to a number.** This is where the generator's element count goes from thousands to millions, and the standing principle is that it stays snappy. The rig exists to say whether it did, and the frame it is measured against is the one the canopy work already named. [paraphrase]
- **Rejected: letting the local rules ignore the bias field.** It is the smaller implementation and it is wrong for these trees. The bias field is the whole of what makes them supernatural, it is a noise lookup that costs nothing at twig scale, and contorted hazel is twisted to the twig, so the botany does not object either. [user]
- **Rejected: treating "realistic structure" as satisfied by count and ratio alone.** A tree with the right leaf count and the right twig diameter can still fork like a broom. The local rules rest on botanical defaults with sources so that realism is the starting state, and the owner's eye in R8 judges the whole rather than standing in for a structural criterion. [paraphrase]

## Parked unknowns

- Where the crossover between colonization and local rules should sit. It is a measurement against the cost curve and the clay, not a number decidable here. [inferred]
- Whether interactive build at depth needs debouncing, a coarse preview while dragging, or nothing. [inferred]
- Whether the surface stage stays the cheap half at millions of triangles, or becomes the thing worth optimising. [inferred]
- What depth the presets ship at, which is the owner's judgement in clay against the cost curve, and is R8. [paraphrase]

## Early proof point

Task fn-5-branch-until-the-tips-bear-leaves-one.1 validates the approach: that the attractor search radius can be derived from attractor spacing, that it resolves to today's radius at today's step and density so every existing tree stays byte-identical, and that the measured 186-node collapse becomes a whole tree.

It runs first because nothing deeper works without it. Every step below today's is the regime where the current derivation starves the growth, so a depth dial built on the old radius would produce stumps at exactly the settings the spec exists to reach. If the derivation cannot be made both spacing-aware and byte-identical at the default, R7's attributability requirement and R1's rail are in conflict, and that conflict is worth discovering in one task rather than at the end of six.

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
|-----|-------------|---------|-------------------|
| R1 | One recursion, trunk to leaf-bearing tips, depth a named parameter | .2, .3 | — |
| R2 | Radius, direction and taper continuous across the crossover | .4 | — |
| R3 | Terminal wood reaches leaf scale, measured as leaf-to-twig ratio | .3, .4 | — |
| R4 | No reachable depth starves the growth | .1 | — |
| R5 | Leaf count reaches the botanical order, after interior shedding | .5 | Shedding lands in .5's acceptance; manual sync |
| R6 | Cost curve measured against the named frame; maxNodes reported; build stays usable | .5 | Frame target lands in .5's acceptance; manual sync |
| R7 | Determinism, and the default reproduces today byte-identically | .1, .2 | — |
| R8 | Owner confirms in clay, the trees still read as themselves, and states the shipping depth | .6 | Manual gate; .6 prepares the comparison the judgement is made against |
| R9 | The bias field shapes twigs as it shapes limbs | .3 | Task .3 needs its acceptance extended; plan-sync is off, so this is a manual sync |

