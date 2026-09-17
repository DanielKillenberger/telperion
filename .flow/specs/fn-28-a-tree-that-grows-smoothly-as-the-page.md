# A tree that grows smoothly as the page scrolls

> **Shelved 2026-09-18.** Growth over time is a hidden feature since fn-65: the mature tree is the product and the harness draws the direct build. This spec resumes only when the owner un-hides growth; until then it is not ready and not in progress. The reason is recorded in CLAUDE.md under "Mature trees are the product".

## Conversation Evidence

> user (turn 1): "pls continue fn-11 (growing trees over time)"
> user (turn 2): "do we have a path to increase speed to hit the target?"
> user (turn 4, part 1): "but after fn-11 i won't be able to have a tree grow smoothly with the scrolling through the page on my personal website right?"
> user (turn 4, part 2): "we'd need smaller grow ticks. And also each tick needs to be fast enough to happen within a scroll."
> user (turn 5): "that's the goal i wanted to achieve. A smooth growing tree as you scroll my website."
> user (turn 6): "ideally we'd see leaves sprout even. But that's probably just not possible in terms of fidelity?"
> user (edit cycle 1): "ideally we would get the cost of a tick of growth driven to almost zero. I wonder if we rethink generation in a way that is O(1) for generation and not dependant on previous steps this spec would be much easier. But with growth affecting other leaves it's just not possible to get? Let's review with astra on this tradeoff and the spec in general"
> user (2026-09-13): "can we make the growth rule easily reversable? => if you have tree at t you could apply the step function in reverse and get tree at t-1?"
> user (2026-09-13): "hm will this be clean the way we're building this with fn-11 and fn-28 separate? I'd like to have a clean algo that is well designed from the ground up not patched together."
> user (2026-09-13, on rewriting fn-11 around the chronicle and this spec down to presentation): "ok let's go"

<!-- Rewritten 2026-09-13 after fn-11 was redesigned around the chronicle. The first draft carried checkpoints, a worker scheduling contract and a lookahead; the chronicle makes every age at or below the frontier a read, so those fell away and this spec is the presentation layer only. The two Astra review rounds on the first draft (2026-09-13) are answered by the redesign; their surviving finding, ring continuity under a bent extension, is kept below. -->

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 45% [user], 40% [paraphrase], 15% [inferred] -->

The owner's personal website scrolls, and as it scrolls a tree grows. That is the goal, stated in the owner's words: "A smooth growing tree as you scroll my website." [user] The visitor never sees a step, a pop or a stall; leaves sprout on new shoots rather than appearing whole; the tree at every scroll position is one specimen at one age, and moving the page in either direction moves it through its life. [paraphrase]

Growth over time (fn-11) supplies everything but the smoothness. Its specimen is a chronicle: a lifetime record in which every node, run, placement, leaf cohort and radius keyframe carries a birth year and, once gone, a death year, written only by the simulation. The tree at any age at or below the simulated frontier is a filter over that record, byte-identical to a fresh build and costing no simulation, and the change between two ages is the stamps inside the interval. What fn-11 leaves discrete is the year: the tree changes once per tree-year, and a scroll would land one visual step per year. The owner named the gaps this spec closes: the visible ticks must be smaller than a year, and a step must cost so little that it fits inside a scroll. [paraphrase]

This spec is the presentation layer over the chronicle: a blend between two consecutive yearly states, driven by a fractional age, exposed to a browser page. The page itself, its scroll binding and its composition belong to the website's own repository. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **A fractional age reads two years and blends.** A presentation age between two whole years selects the chronicle's state at each year and the elements stamped inside the interval, and blends on the skeleton, never on mesh buffers: surviving nodes interpolate position and radius between their keyframes; a run born inside the interval grows from its parent's surface with zero length and radius toward its state at the later year; a run dead inside the interval does the reverse, shrinking to nothing before it disappears; a placement whose station reaches its cohort offset inside the interval sprouts, its leaf element scaling up from a bud, and a placement whose shoot dies inside the interval fades with it. Cohorts are a function of the shoot's age, not stamps, so a mature tree's foliage holds still between years and the blend has nothing to do there. The blend reads the chronicle and writes nothing to it. [paraphrase]
- **Continuity at the tip.** The presentation geometry is swept from the blended skeleton by the surface builder, so rings, caps and forks follow from the skeleton the way they do for any tree, with one rule the builder alone would not give: a segment born inside the interval enters its neighbours' frame averages with a weight equal to its grow-in fraction, and a dying segment with its remaining fraction, so a ring at a tip that gains a bent extension turns continuously from the terminal frame at fraction zero to the state's own interior frame at fraction one instead of snapping by a finite angle the moment an arbitrarily short extension exists. The reverse transition is the same rule run backward. At either end of the interval the swept geometry equals that year's own geometry byte for byte. [inferred]
- **The sweep runs at the presentation cadence, off the page's main thread.** A presentation step sweeps the blended skeleton once and hands the page a mesh and a placement set; the page draws that until the next step. The step count per frame is the consumer's choice; the cost of one step is measured in R6 and bounds how fine the steps can be at 60 frames a second. [inferred]
- **Scrolling is reading.** A scroll position maps to a target age. Any target at or below the chronicle's frontier is a read, forward or backward, so scrubbing costs the blend and the sweep and nothing else. The page simulates once at load to the oldest age it will show, in a worker, and never simulates again while the visitor scrolls; a target beyond the frontier advances the specimen by whole years first. [paraphrase]
- **Nothing here changes the model.** The blend is a pure function of the chronicle and a fractional age; it never writes a stamp, never alters a keyframe, and never invents a state. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **A growing view over a specimen**, in the browser binding: built from a family, a seed and the oldest age the page will show, the display limit, which may be fractional; at construction it simulates through the first whole year at or above that limit, so that both whole-year endpoints of every presentation age up to the limit exist before the view is ready, and the display limit stays distinct from the simulated frontier; given a presentation age, fractional, at any time; read for the blended structure, wood and placements at that age; queried for its display limit and its frontier. [inferred]
- **Errors** are explicit and leave the view usable: a presentation age outside the family's range or above the display limit is refused naming the field or the limit; a growth failure at construction leaves the view with a display limit lowered to the last complete year and reports the failure. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Determinism is untouched.** The two states the blend reads are fn-11's tree at those ages, byte for byte, so a page reloaded at the same scroll position shows the same tree; the blend is deterministic in the fraction. [paraphrase]
- **The needle.** A cohort rule with a lifetime of several years means a spruce shoot's foliage changes little year to year; the sprout and fade apply per cohort and read as a slow turnover, never a flicker. [inferred]
- **The website tree need not be the mature hero oak.** Every cost scales with node count; the page picks a family and size that fit its budget, and the measurement names which. [inferred]
- **The measurement protocol.** The fixture is the harness page with the growing view bound to a scripted scroll position, on the named machine (an RTX 3080, Chrome, 1600 by 1000 at native pixel ratio, the machine fn-14 and fn-27 measured on). The family, seed and age range are stated in the report before any number is taken; until the owner names the website's tree, the fixture family is the oak at seed 7 from the youngest age to the mature age plus twice the leaf lifetime. Two scroll traces: a reference trace that sweeps the whole range in ten seconds, and a reversal trace that sweeps forward for three seconds and back for three. The frame statistic is the wall-clock p50, p95 and max over the trace and the frame count, in the shape of fn-14's orbit records. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Under the reference and reversal scroll traces on the fixture, the tree grows continuously from the youngest age to the mature age plus twice the leaf lifetime, in both directions: no visible step between years, no pop when a branch is born or dies, leaves sprout from a bud on new shoots rather than appearing whole, and the crown keeps its foliage at maturity; judged by the owner on the fixture and recorded in the owner's words. [paraphrase] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R2:** A scroll step never blocks the page: the blend and the sweep run off the main thread, the page draws every frame from the newest presentation step, and both traces hold a wall-clock p95 at or under 16.7 ms per frame on the named machine. [paraphrase] Errors: a frame-time miss stops the spec with the number.
- **R3:** Scrolling backward costs what scrolling forward costs: no simulation runs for a target at or below the frontier in either direction, and the two traces' frame statistics agree within the protocol's noise. [paraphrase] Errors: a backward trace slower than the forward one beyond noise is a defect, not drift.
- **R4:** The tree at any whole-year scroll position is fn-11's tree at that age: the two states the blend reads are byte-identical to fresh builds at their ages, the swept geometry at either end of a blend interval equals that year's own geometry byte for byte, and blending changes presentation only. [paraphrase] Errors: a state or an endpoint that differs fails validation naming the age.
- **R5:** The capability is engine-neutral and lives in Telperion: the browser binding exposes the growing view, and no page-specific code enters the core or the renderer. [strategy:The core and integration] Errors: no error surface beyond field validation.
- **R6:** The cost of the growing view is measured and recorded on the named family, seed and age range: the load-time simulation to the oldest age, the chronicle's memory at that age, the cost of one presentation step, the blend and the sweep, on the mature tree, and the resulting bound on steps per frame at 60 frames a second. [inferred] Errors: no error surface beyond the measurement protocol.
- **R7:** The automated tests cover, on the fixture family: endpoint equality of the blend against the two years it reads; a blend at an interior fraction keeps every child base on its parent's surface; a bent extension at a tip, blended at successively smaller positive fractions, converges to the endpoint geometry with the surviving ring's frame turning by an angle that goes to zero with the fraction, and the reverse transition does the same; a sprouting placement scales monotonically from zero to its full element over its first interval; a view built with a fractional display limit answers every presentation age up to that limit from two existing whole-year states with no further simulation, and refuses an age above the limit naming it; a growth failure at construction leaves the last complete year drawable. [inferred] Errors: a missing case is a review finding, not implementer discretion.

## Boundaries
<!-- scope: business -->

- The website page, its scroll binding and its composition are built in the website's repository, not here; this spec ends at a browser binding a page can call. [inferred]
- No change to fn-11's model: no stamp is written, no keyframe altered, no state invented; smoothness is a read over the chronicle. [inferred]
- No death, seasons, environment or damage response; those stay with their own specs. [paraphrase]
- No forest; one specimen on one page. [inferred]
- No modelled bud opening: a sprouting leaf scales in from a bud, and folded blades unfurling is a later fidelity extension. [inferred]
- The fixture is the repository's harness page driven by a scripted scroll trace, not the live website; the website page consumes the same binding in its own repository. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The goal is a smooth growing tree as you scroll the website; smoothness, a tick that fits within a scroll, and leaves that sprout are the measures the owner named. [paraphrase]
- The owner asked for a reversible growth rule and for one clean design rather than two specs patched together; the answer was to make fn-11's specimen a chronicle, so that reversal and scrubbing are reads, and to reduce this spec to presentation. [paraphrase]

### Implementation Tradeoffs

- Presentation over the chronicle rather than checkpoints, a lookahead and a worker scheduling contract, which the first draft carried: with every age at or below the frontier a read, forward and backward scrubbing cost the same, there is nothing to catch up on, and the reviewer's findings about buffer exhaustion, obsolete worker results and reversal latency have no object. [paraphrase]
- Interpolation between two yearly states over a finer slice: a finer slice would need the per-slice fixed cost driven near zero and would still step at some rate; blending removes the step at any quantum and costs the presentation step, which R6 measures. [inferred]
- The closed-form alternative, examined with the implementer on 2026-09-13: emitting N elements costs O(N) however the tree is evaluated, so a closed form would make cost independent of elapsed steps rather than small, at the price of deciding which history-dependent behaviours to keep as functions of age. The chronicle keeps them all and gives the free scrubbing the closed form promised. [paraphrase]

## Parked unknowns

- Which family and size the website page uses; it sets every measured number in R6 and is the owner's choice. Until named, the fixture family in the protocol stands in.

## Strategy Alignment

- Follows "The core and integration": one core serving browser Wasm, consumers requesting only the representations they need, engine-specific drawing kept out of portable tree data.
- Tension noted, not a contradiction: the strategy sequences integration proofs Unreal first. A personal website is a browser consumer the renderer already serves, so it precedes the Unreal proof without displacing it; the owner may want to record it in STRATEGY.md.
