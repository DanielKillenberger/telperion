# The twig layer: the branching order between shoots and leaves

> **SUPERSEDED, not shipped.** This spec was merged into `fn-5-branch-until-the-tips-bear-leaves-one` before any work started. It is closed because it no longer describes work to do, not because the work was done. The merge happened because a tree is one branching recursion: splitting depth from twigs put the crossover between them in neither spec's acceptance criteria, which is how 5 mm twigs end up on 20 cm logs. The measurements and reasoning here were carried into fn-5 verbatim.


## Conversation Evidence

> user: "there's a problem with how leaves attach though? This doesn't seem natural to me. The leaves themselves seem fine but i guess we'd smaller branches to attach them to?"
> user: "It looks fine and could maybe be a tree in a desert but there's not enough volume to the tree either. With this dense of a branch system i'd be expecting a full ass covered canope"
> user: "this might be a small adjustment though you basically just need another branching layer to attach the leaves to?"
> user: "but maybe i'm wrong"
> user (boundary): "we also need a spec to fix cliping and overlying branches separately"
> user: "how would a 150m tall tree work in real life? would it have giant leaves? or would there be millions of leaves?"
> user (standing principle, carried): "hm well i'd like to take this as far as we can to get to a high perf high fidelity tree generator."
> user (standing principle, carried): "I want it to be as efficient as possible should be snappy. Top tier engineering. Each component worthy of its own library."
> user (standing principle, carried): "and it needs to be parameterizable that twist and turning ideally" / "like everything i guess"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 45% [user], 40% [paraphrase], 15% [inferred] -->

The canopy ships and the owner's judgement in clay is that the leaves do not attach to anything a leaf could attach to [user]. The generator has no branching order fine enough to bear foliage, so leaves are fastened to structural timber, and the crown reads as a desert tree rather than the covered canopy the branch density leads a viewer to expect [user].

Measurement backs the reading rather than merely agreeing with it [paraphrase]. On Telperion the finest wood anywhere in the skeleton is 0.79 m in diameter and the leaf is 0.12 m long, so a leaf is 0.15 times the diameter of the wood it grows on. A real leaf runs the other way, roughly twenty-five times its twig's diameter, which puts the current arrangement off by about two orders of magnitude. The mean shoot is 12.85 m long: the thing the canopy calls a twig is a metre-thick pole the length of a bus.

The same measurement explains the missing volume, and it is one cause rather than two [paraphrase]. Volume is bounded by attachment points and there are 133 of them, so 2,904 leaves land as 22 beads on each of 133 poles. Adding leaves per shoot makes denser poles, not a canopy.

Botany points the same way and sets the target [paraphrase]. Leaf size does not scale with tree height: it is set by the leaf's own heat and hydraulic limits, and both tighten as a tree gets taller. Koch et al. measured leaves near the tops of coast redwoods as shorter and less expandable than leaves lower on the same tree, because water tension rises about 0.06 MPa per metre from gravity alone, and extrapolating that gradient is where the roughly 122 to 130 m ceiling on terrestrial tree height comes from. A 148 m tree is therefore already past what Earth allows, which the library is free to be, but height is an argument for smaller leaves in greater number and never for larger ones. Corner's Rules state the relationship the owner's eye caught: axis thickness tracks appendage size, so stout sparsely-branched axes carry large leaves and finely divided ones carry many small leaves. Nothing in nature puts small leaves on metre-thick logs. A mature oak is counted in hundreds of thousands of leaves and a large conifer in tens of millions, which puts a full crown at this scale somewhere between 10^5 and 10^7 elements against the 2,904 there are now.

The existing density dial cannot reach this. Forty times the attractors, from 1,600 to 64,000, moves the finest wood from 0.79 m to 0.69 m and the shoot count from 133 to 171 [paraphrase]. Space colonization sets its step distance as a fraction of envelope height, so on a 148 m tree every step is metres and terminal branches stay metre-scale however many attractors they chase. The fix is a branching order below the skeleton, not more of the skeleton [user].

## Architecture & Data Models
<!-- scope: technical -->

- **A twig is a fifth-order structure the skeleton does not produce, generated per shoot rather than grown into the envelope** [paraphrase]. Twigs are short, thin and structurally trivial: they need no space colonization, no attractors and no thickness solve conserving area through forks. Deriving them from the shoots the canopy already finds is what keeps this a stage rather than a second generator.
- **Placement moves down one level.** Leaves attach to twigs; twigs attach to shoots. The phyllotaxis, clumping, orientation-bias and scatter machinery the canopy already has is the same machinery twig placement needs, one level up, and reusing it is most of why this is smaller than it sounds. [inferred]
- **Twigs are visible geometry, not attachment frames.** The camera sees them, so they carry a mesh. Whether that mesh joins the canopy's existing instanced draw or takes one of its own is the open cost question. [inferred]
- **Every term is a named parameter with a stated rail**, matching the convention every other stage in this library already holds to. [user]
- **The element's local-frame contract is unchanged**: petiole at origin, axis +Y, face normal +Z, and placement emits transforms against it without inspecting element geometry. A twig layer changes what placement walks, not what it emits. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The geometry multiplies and the budget is the standing principle.** 133 shoots at a plausible thirty twigs each is roughly 4,000 twigs, and leaves on those run to six figures against today's 2,904. The rig measured the branch-only room at 0.21 ms of a 16.7 ms frame on an RTX 3080, so the budget exists, but a two-order-of-magnitude rise in elements is exactly the case that spends it. [paraphrase]
- **The botanical target may exceed what one instanced draw can carry, and that tension is the interesting part of this spec.** 10^5 leaves at the element's 16 triangles is 1.6 million triangles, which is tractable; 10^7 is not, and is where an impostor or a smaller element stops being optional. The measurement decides which regime this tree is in rather than a guess made here. [inferred]
- **Shell culling was measured against a canopy this stage invalidates.** The current cull removes 14.8% on Telperion and 6.5% on Laurelin precisely because placement puts foliage on distal shoots that already sit near the crown surface. Twigs push foliage outward and multiply interior candidates, so both figures are re-measured rather than carried forward. [paraphrase]
- **A twig that intersects its own shoot, a sibling twig, or the wood it grows from is the same class of defect this repo already fought once at fork junctions.** The lesson recorded there was to assert the geometric claim on every vertex against the surface as drawn, never on a centre against an analytic radius. [paraphrase]
- **The stage stays deterministic in the seed**, drawing chance from its own XOR-derived sub-stream so that adding it leaves the skeleton, the bias field, the noise field and the existing canopy stream byte-identical. [paraphrase]
- **Judged in clay**, flat grey, no material and no light emitted by the library, exactly as the canopy is. [paraphrase]

## Acceptance Criteria

- **R1:** A twig stage generates second-order branches on the shoots the canopy already derives, with count per shoot, length, diameter, divergence, angle from the shoot and taper each a named parameter carrying a documented rail. Errors: a shoot too short or too thin to carry a twig yields none rather than throwing; non-finite parameters fall back to documented defaults through the same rail idiom the other stages use. [user]
- **R2:** Leaves attach to twigs rather than to shoots, and a leaf's length is a multiple of the diameter of the wood it sits on rather than a fraction of it, verified by a test asserting that ratio on both presets. Errors: a twig bearing no leaves is emitted as bare wood rather than dropped, since a bare twig is a real thing and a missing one is a hole. [user]
- **R3:** Attachment points and leaf count rise by at least two orders of magnitude at preset density, measured on Telperion and Laurelin before and after, with the reachable count at the top of the density range stated. The botanical target is 10^5 to 10^7 for a crown this size; two orders is the floor this criterion holds, not the ambition. Errors: a count the frame budget in R6 cannot carry is reported as the measured ceiling rather than quietly clamped. [paraphrase]
- **R4:** Twig geometry is generated procedurally, is visible in clay, and is drawn without adding more than one draw call per tree beyond what the canopy already costs. Errors: no error surface beyond R6's reporting. [paraphrase]
- **R5:** Same seed and parameters yield an identical twig layer and canopy, and adding the stage leaves every earlier stage byte-identical. Errors: determinism asserted by building twice from one seed and comparing the emitted typed arrays. [paraphrase]
- **R6:** The frame budget is re-measured on the named machine with the existing rig, at the new element count, and the shell cull's removal fraction is re-measured on both presets rather than carried forward from the canopy spec. Errors: when the GPU timer-query extension is unavailable the panel says so and reports no timing number. [paraphrase]
- **R7:** The owner confirms in clay that the crown reads as covered rather than beaded, and that leaves attach to something a leaf could attach to. [user]

## Boundaries

- **Branch clipping and interpenetrating limbs are a separate spec** and explicitly not this one. [user]
- No procedural leaf texturing; that remains its own spec and this one keeps the flat placeholder element. [paraphrase]
- No wind, sway or animation. [paraphrase]
- No change to the skeleton, the radius solve or the swept surface. The twig layer sits below them and does not reach back up. [paraphrase]
- No LOD ladder; a far impostor only if the re-measurement demands one. [paraphrase]

## Decision Context

### Motivation
<!-- scope: business -->

- **"A full ass covered canope" is the owner's stated definition of done** and it is the thing the current canopy fails [user]. The generator's own branch density sets that expectation: a viewer reading a dense limb structure expects the foliage to match it.
- **The owner proposed the fix and the measurement agrees with the owner** [user]. That is worth recording because the alternative diagnosis, that leaves are merely too sparse or too small, is the one a tuning pass would have chased, and the density sweep shows it would have failed.
- **Rejected as unreachable: turning up density.** Forty times the attractors buys 13% finer wood. The dial is asymptotic against a step distance set as a fraction of envelope height. [paraphrase]
- **Rejected as the wrong level: more leaves per shoot.** It makes each of the 133 poles denser and leaves the attachment count where it was. [paraphrase]
- **Rejected on the botany: bigger leaves.** The owner asked directly whether a tree this tall would carry giant leaves or millions of small ones, and the answer settles a design fork rather than being trivia [user]. Leaf size is set by the leaf's own heat and hydraulic limits and gets tighter with height, so the tallest real trees carry the smallest foliage. Scaling the element up to fill the crown would be the one change that moves this tree further from a tree. [paraphrase]
- **The cost is stated rather than hidden.** This is where a procedural tree's element count goes from thousands to six figures, and the standing principle is that the thing has to stay snappy. The rig exists to say whether it did. [paraphrase]

## Parked unknowns

- Whether twigs join the canopy's existing instanced draw or need one of their own. It resolves on the first build that has twig geometry to draw. [inferred]
- Whether twigs need culling of their own or whether culling at the leaf level is enough once the leaves are on twigs. [inferred]
- What the shell cull removes once foliage sits on twigs. The canopy's figures were measured against a distribution this stage replaces. [inferred]

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
