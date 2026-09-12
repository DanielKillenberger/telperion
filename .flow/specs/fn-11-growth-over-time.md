# Growth over time

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"
> user (refine, 2026-09-12): "it should be a continuous parameter like every other parameter. We're building a continuous multidimensional tree space. Trees should eventually be able to die too. But maybe we can just grow to maturity for now and leave dying to a later spec (probably needs to be optional for games anyway)"
> user (refine, 2026-09-12): "it should be deterministic so we don't need to store history, parameters+seed+age should give the exact same tree."
> user (refine, 2026-09-12): "age parameter has to apply to any kind of tree. Spruce and oak we calibrate specifically. But growing should [work] for any tree parameter set."
> user (refine, 2026-09-12): "we should try and find open-grown curves and match them as closely as possible. The closer to reality the better. We'll judge what rules over what once we get there."
> user (refine, 2026-09-12): "if we build the generator to incrementally build based on age steps efficiently enough that a mature tree still generates within reasonable time it might be the better approach?"
> user (refine, 2026-09-12): "Time should be an input like every other parameter and you should be able to step along any dimension of the tree space and have a valid tree."

This approves the preceding seven-item roadmap, including this outcome. [paraphrase] Refined 2026-09-12 in a business and technical interview; fn-21's shoot model was absorbed here at the owner's choice and fn-21 narrowed to variation and the light benchmark.

## Goal & Context

A tree is a specimen with an age. Age in years is one more continuous input to the tree space, so any family at any age is a valid tree, a walk along the age dimension is the same tree growing, and the same family, seed and age always give the same tree. Growth is live: a running scene advances a specimen by any fraction of a year at whatever rate the game chooses, and the tree grows in place, branches keeping their identity, thickening and extending, with shaded limbs dying and dropping as real open-grown trees self-prune. Every family grows by the same rule; the oak and the spruce are calibrated against open-grown growth curves for height and trunk diameter. Dying, seasons and the environment's influence are later specs. [user]

## Architecture & Data Models

- **Stepped growth over the existing builder, retained as a frontier.** Production generation is the scaffold queue (`scaffold::generate` consumes whole axes through a queue of growth units), then the radius solve, then `local::append` for local branches and twigs, then shell shedding, then the solve again. Growth over time keeps that order but makes each stage resumable: the specimen retains the scaffold frontier (every axis still growing, its remaining units and its per-axis random stream, which is already keyed by parent, station and member), the attractor set with spent flags, and the local frontier (every shoot still able to bud, with its stream). A monthly slice spends a budget of growth units, set by the family's growth rate at the current age, across the scaffold frontier first and the local frontier second, expands the envelope and the attractor reach along the species' curve, runs shedding by vigour, and runs the radius update. A fresh build at age t runs slices from a seedling to t; native builds, wasm builds and a retained advance all go through that one implementation, so they agree by construction. [user]
- **A fixed monthly step with the remainder carried.** An advance of any fraction of a year is applied as whole monthly slices, and any sequence of advances that reaches the same age applies exactly the same slices, so the result is independent of tick size. A consumer that wants faster growth passes more years per tick; no artificial rate lives in the core, and a long advance is one call with no progress callback, the consumer slicing its own advances if it wants progress. [user]
- **Stable identities beside storage indices.** Today a node's parent and branch are indices into the node vector, the structural nodes must precede the local ones (the crossover), and shedding compacts the vector and renumbers survivors. Each node and each run therefore gains an identity assigned at birth from a per-specimen monotone counter, never reused and never renumbered; storage stays index-based and may compact, with the index-to-identity map rebuilt after any compaction. The crossover invariant is kept by growing new structural nodes into the structural segment and re-indexing the local segment at the end of each slice; consumers and change records speak identities only, never indices. [paraphrase]
- **Thickening for structural and local wood.** Structural radii come from the pipe-model solve each slice, and the trunk radius it distributes is a fraction of the envelope's height, so the trunk and every structural branch thicken as the envelope grows with age. Local branches and twigs, which the solve leaves at their birth radii, are re-derived each slice from their parent's current radius by the existing child-radius rule, so a surviving local branch never thins as its parent thickens; the twig's anatomy stays in metres. When topology changes, forks or shedding, a surviving branch's radius is monotone non-decreasing across the slice. [paraphrase]
- **Shoot state on nodes.** Each node carries its birth year, its bud fate as terminal or lateral, and a vigour proxy read from its depth inside the crown at the current age. Vigour drives extension and survival; dominance and the light-response benchmark stay with fn-21, which extends this state rather than replacing it. [user]
- **Shedding by vigour.** A shoot whose vigour stays below its family's threshold for the family's tolerance period dies and is shed with its subtree; the existing shell shedding becomes one input to vigour rather than a separate pass; identities are retired and never reused, and a shed run leaves the live structure. [user]
- **Growth traits, not a maturity age.** A family carries numeric growth traits, the rate and shape by which height and girth approach the authored envelope, and a leaf lifetime in years; maturity is what those traits produce, never a field. Today's preset parameters describe the full-grown envelope, and the age at which a preset reaches its current look is derived and documented, not authored. A point between two families blends their growth traits and is a valid tree at any age. [user]
- **Foliage by leaf lifetime, placements by identity.** A shoot bears leaves from its birth for the family's leaf lifetime, one year for the oak and several for the spruce's needles, so young shoots carry foliage and older wood goes bare by one rule on every tree. Placements carry the identity of the shoot and station they sit on; because a station's transform is derived from the wood's positions, radii, frames and surface contact, a surviving placement moves when its wood thickens or extends, and the change record says so. [user]
- **The skeleton is the persistent incremental state.** Identities are append-only across a specimen's life so that a later spec can sweep the wood surface on the GPU from the skeleton alone; in this spec the wood surface is rebuilt and re-submitted per advance while leaf placements update from the change record. [user]

## API Contracts

- **Age is a family field**, in years, validated like every field and walked by the blend; a preset states the age its calibration was judged at only as documentation. [user]
- **A retained specimen**, native and in the wasm binding as a handle in the shape of the field handle, built from a family and its age and advanced by a number of years. Advancing returns a change record over identities: runs born, runs resized, runs shed, placements born, placements moved with their new transforms, placements shed. Applying a record to the consumer's previous buffers reproduces exactly the buffers a fresh read of the handle returns, and building at an age and advancing to it yield byte-identical outputs. [user]
- **Errors** are explicit and leave the specimen usable: a negative advance is refused naming the value; an age outside its range is refused naming the field; a node ceiling reached mid-growth stops growth with the existing capped diagnostic and the specimen at the last complete slice, and the next advance refuses with the same diagnostic until the ceiling is raised. A handle whose specimen was released or rebuilt is invalid, as the field handle is today. [paraphrase]
- **The harness** gains an age dial that scrubs deterministically, a rebuild at the chosen age, and a play control that advances live at a years-per-second rate chosen on the page; both drive the same advance path. [user]

## Edge Cases & Constraints

- **Any family grows.** The rule reads only numeric traits; no species branch, no preset name, in the growth code. The Two Trees take the same age and must build and grow at every age, with no reference or verdict applied to them here. [user]
- **Determinism across tick sizes** is the load-bearing property: one jump and a chain of small advances to the same age must be byte-identical, including which limbs shed and in which slice. Random draws are keyed by identity and slice, never by call order; the scaffold's per-axis streams already are. [user]
- **Growth after twigs.** New structural growth lands after local branches and twigs exist; the structural segment grows and the local segment re-indexes, identities unchanged. Shedding with surviving siblings compacts storage without touching a survivor's identity. Both are pinned by tests on identities and on output reconciliation. [paraphrase]
- **Backward is a rebuild.** Scrubbing to an earlier age rebuilds the specimen from seed to that age at about the cost of today's build; no history is stored. [user]
- **Cost is measured, not yet gated.** The cost of one monthly slice and of a full mature build on the oak and the spruce is recorded beside today's build time in the report; the per-slice fixed cost, chiefly the radius update and the placement re-derivation, is the number a fast-forwarding game pays twenty times a second at a century a minute. [user]
- **Today's presets may drift** within a judged tolerance when the growth rule needs it; identity pins are re-pinned once with the reason recorded and the mature oak and spruce are judged again by the owner. [user]
- **Open-grown references first.** The calibration sources open-grown height and diameter curves for the oak and the spruce the way fn-9 sourced its profiles, with sources and checksums in the report; stand-grown yield tables are the fallback, named as such. Where the curve and the specimen's look disagree, the deviation is recorded and the owner judges which rules at that age. [user]
- **Coupling named.** fn-4 and fn-20 edit the wood surface and terminal radii; fn-21 extends the shoot state; fn-16 adds events on top of this timeline; fn-27's caster prefix re-sorts runs by radius at submit and continues to do so per advance. [inferred]

## Acceptance Criteria

- **R1:** A specimen develops recognizably through successive ages, with coherent branch identity, thickening and foliage development; no error surface beyond R2. [paraphrase]
- **R2:** Define supported ages and time controls, including pause, advance and backward inspection. Replaying the same inputs reproduces the same history. Invalid time requests and resource limits are explicit and leave a usable state. [inferred]
- **R3:** Validate developmental form against selected species profiles and measure update latency and memory across growth stages; report missing age references and performance limits. [inferred]
- **R4:** Age is a continuous family field in years and every family grows by one rule: building any of the five presets and any blend of them at any supported age yields a valid tree, and stepping the age dimension is the same specimen growing. [user] Errors: an age outside its range is refused naming the field; the node ceiling stops growth with the capped diagnostic at the last complete slice.
- **R5:** Family, seed and age determine the tree exactly: a fresh build at age t and any sequence of advances reaching t, in monthly slices with the remainder carried, produce byte-identical structure, radii, placements and shed set. [user] Errors: a negative advance is refused naming the value; no history is stored.
- **R6:** Branches keep their identity through growth and thicken and extend by the radius solve; a shoot whose vigour stays below the family threshold for the family's tolerance period is shed with its subtree, its identity retired and never reused, and each advance returns a change record naming born, resized and shed runs and born, moved and shed placements, whose application to the previous buffers equals a fresh read. [user] Errors: a change record that does not reconcile with the structure fails validation naming the run.
- **R7:** Shoot state per node holds birth year, bud fate and a vigour proxy from crown depth; foliage follows a leaf-lifetime trait so a shoot bears leaves from birth for that many years on every tree. [user] Errors: no error surface beyond field validation.
- **R8:** The oak and the spruce at their calibrated growth traits follow open-grown height and trunk-diameter curves at three ages each, young, middle and mature, within 15 percent, with the curves' sources and checksums in the report and any age where the owner's judgment overrides the curve recorded with the deviation. [user] Errors: a missing open-grown curve is recorded and a stand-grown fallback named; a miss outside the tolerance stops the spec with the number unless the owner's recorded judgment accepts it.
- **R9:** Live growth on the harness page: the retained wasm specimen advances at a chosen years-per-second rate with the tree growing in place, leaf placements updated from the change record and the wood re-submitted, and the age dial scrubs to any age by rebuild; the native API offers the same advance. [user] Errors: as R4 and R5.
- **R10:** The cost of one monthly slice and of a full build at the derived mature age is measured on the oak and the spruce and recorded beside today's build time, with the report naming the per-slice fixed cost; no bound is gated in this spec. [user] Errors: no error surface beyond the measurement protocol.
- **R11:** The mature oak and spruce, re-judged by the owner beside fn-14's stills after any drift, and an age strip of each from a young age to maturity, are accepted in the owner's own words. [user] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.

## Approach, in one pass

One task builds the whole spec in the order below; the owner's standing rule is one task per spec, so the order and the tests live here rather than in a task breakdown. Each numbered step lands with its tests green before the next begins.

1. **Identities and the retained frontier.** Add the identity counter and per-node identity, the index-to-identity map, and the retained scaffold, attractor and local frontiers as a `Specimen` beside `Tree`; make the existing whole build a specimen grown to the mature age in one call. Tests: identities survive growth after twigs and shedding with surviving siblings; the crossover invariant holds after every slice; the byte-identity of a specimen grown to the mature age against itself across two runs.
2. **The monthly slice and replay equivalence.** The unit budget from the growth traits, the envelope and attractor expansion along the curve shape, the slice loop with the remainder carried. Tests: any partition of an advance into slices gives the same tree; a fresh build at t equals advance-to-t, byte for byte; a negative advance and an out-of-range age refuse naming the value; the node ceiling mid-growth leaves the last complete slice with the capped diagnostic and the next advance refuses.
3. **Thickening and shedding.** The per-slice radius update for structural and local wood; vigour from crown depth and birth year; shedding by threshold and tolerance with the shell rule folded in. Tests: surviving radii monotone non-decreasing across extension, forks and shedding; a shoot under the threshold for the period is shed in the expected slice and never earlier.
4. **Foliage by lifetime and the change record.** Leaf lifetime as a family field; placements by shoot and station identity; the record of born, resized and shed runs and born, moved and shed placements. Tests: applying a record to previous buffers equals a fresh read, including a surface-contact case; leaf expiry at the lifetime boundary.
5. **Bindings and the page.** The wasm specimen handle with build, advance and re-read, invalid after release or rebuild; the native API mirrored; the harness age dial and play rate driving the same path; the wood re-submitted and placements updated from the record. Tests: handle recovery after a failed rebuild, the existing binding ownership tests extended.
6. **Curves and calibration.** Source open-grown height and diameter curves for the oak and the spruce with checksums; set the growth traits so both meet the curves within 15 percent at three ages; record any deviation the owner may judge. Tests: the three-age checks per species.
7. **Cost, stills and the report.** Measure the per-slice cost and the mature build on both trees with the core's timers; render the age strips and the re-judged mature stills through the headless example; write the report in fn-14's shape; write the R11 verdict slots and end for the owner.

## Boundaries

Environmental competition, damage response and seasonal appearance are separate extensions; the first pass establishes age-driven development. [inferred]

- No death of the whole tree; growth to maturity and beyond by the same rule, dying is a later spec and optional for games. [user]
- No seasons, no leaf flush or fall beyond the leaf-lifetime rule. [user]
- No environment or damage response; fn-16 adds events on this timeline. [user]
- No foliage shape variation and no light-response benchmark; fn-21 keeps them on top of this spec's shoot state. [user]
- No forest-scale growth and no GPU-swept wood; the skeleton is shaped so a later spec can sweep the wood on the GPU. [user]
- No artificial growth rate in the core; a consumer chooses years per tick. [user]

## Decision Context

### Motivation

Growth is the strategy's lifecycle promise made real: one persistent botanical structure from trunk to twig that a game can place at any age and grow live, generated from a seed rather than swapped between baked stages. Live, incremental growth was chosen over a static age dial because a running scene is the use; determinism without history was chosen so a game never has to persist a tree's past; identity with natural shedding was chosen over strict identity because real open-grown trees self-prune. The owner values realism first, so calibration targets open-grown curves and the owner judges the residual. [user]

### Implementation Tradeoffs

- Stepped growth over a closed-form evaluation at any age: the builder already steps, the skeleton stage is a small fraction of a build (11 ms of 60 for the ordinary tree, 315 of 1,608 ms for the giant, mesh-free, 2026-09-06), so a mature build costs about today's plus one radius solve per slice, advancing is the same loop continued, and equivalence holds by construction; feedback effects for fn-16 attach naturally. The closed-form alternative, birth times fixed by seed and identity, would give free scrubbing both ways at the cost of no memory, and remains available if the per-slice cost proves too high. [user]
- A monthly slice over a per-tick integration: discrete slices make equal ages reached by different tick sizes byte-identical; continuous integration would weaken that to a tolerance. [user]
- Age as a family field over a specimen input: the field is what the blend walks, so a walk between two families walks age too, and growth traits rather than a maturity field let families of different maturities blend into valid trees. [user]
- One task over a task breakdown: the owner's standing rule since 2026-09-12 is one task per spec with a strong implementer, so the ordered implementation and test sequence lives in the spec's Approach rather than in tasks; the reviewer's split proposal is recorded here and not applied. [user]
- Rebuild per advance for the wood, incremental for placements: thickening moves nearly every wood vertex a year, so a partial upload buys little, while placements append and retire cheaply; the efficient path is sweeping the wood on the GPU from the skeleton, which is its own spec after fn-4 and fn-20. [user]

Coordinate botanical rules with Real-species profiles and procedural templates. That spec supplies the initial validation profiles; legendary templates are not a prerequisite. [paraphrase]

## Resolved via Project Docs

- STRATEGY.md, Our approach: one persistent botanical structure with growth, surroundings and damage shaping its lifecycle; every generator parameter a numeric trait; same seed and parameters give a byte-identical tree. The age field and the growth traits follow that rule.
- STRATEGY.md, Key metrics: the Build metric holds dial-to-tree time to what keeps dragging usable; this spec records the mature build and per-slice costs against it without gating.
- fn-21 spec, Boundaries: "Fn11 owns whole-specimen timeline controls and replay"; the shoot model half of fn-21 is absorbed here at the owner's choice and fn-21 keeps variation and the light benchmark.
- fn-16 spec, Decision Context: depends on Growth over time; its events attach to this timeline.
- docs/species-onboarding.md, Stage handoffs: research supplies attributed observations with rights and confidence; the growth curves are sourced through the same stage.
- scripts/benchmarks/generation.md, stage table of 2026-09-06: skeleton 11.2 ms of a 59.9 ms mesh-free build for the ordinary tree and 314.5 of 1,607.9 ms for the giant, the numbers behind the stepped-growth decision.

## Resolved via Codebase

- `crates/telperion-core/src/colonization.rs:1-40`: deterministic space colonization grows nodes toward attractors iteration by iteration with a `GrowthConfig` of influence radius, kill distance, step distance, trunk height and a node budget; the loop a year slices.
- `crates/telperion-core/src/branching.rs:1-25`: the scaffold builder runs crown, local branches, shell shedding and the final radius solve in botanical order, with `NODE_CEILING` 250,000 and a capped diagnostic; the per-slice radius solve reuses the final solve.
- `crates/telperion-core/src/tree.rs:1-40`: a node carries position, parent, solved radii, its branch run and a kind; no age, birth or vigour exists today, so the shoot state is additive.
- `crates/telperion-core/src/radius.rs:1-30`: the pipe-model radius parameters, trunk radius, fork exponent and length taper, are the terms thickening scales per slice.
- `crates/telperion-core/src/twigs.rs:1-25`: twig anatomy is fixed in metres, so leaf-bearing shoots are the units the leaf lifetime acts on.
- `crates/telperion-core/src/presets.rs:71,153`: the oak and spruce presets describe mature open-grown specimens; those parameters become the full-grown envelope.
- README.md, Architecture: the wasm binding owns outputs until the next build and the field handle is the retained-handle precedent the specimen handle follows.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R11 | the one task |
