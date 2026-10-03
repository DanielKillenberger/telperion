# One growth law: species are points in a continuous architecture space

## Conversation Evidence

> owner (2026-10-02): "We should have a branching rule that makes them grow organically according to the params set."
> owner (2026-10-03): "Problem here is that we seemingly have a fundamental flaw with the growth algo that's holding back most species."
> owner (2026-10-03): "can we have a new model that is able to smoothly transition between those models to get the continuous tree space we want?"
> owner (2026-10-03, on the R1a stills): "it's clearly not even close.. It looks like a huge regression."
> owner (2026-10-03): "obviously we have to visually evaluate the results.."
> owner (2026-10-03): "I don't see a reason as of now of why we shouldn't be able to make that happen"
> owner (2026-10-03, on today's beech): "today's code isnt doing it because today's beech looks like ass. That's the reason we are here"
> owner (2026-10-03, on the date palm): "that's probably the most realistic one we have atm"
> owner (2026-10-03): "can we actually implement each model in parallel and then make the space continuous?" and "then parallelization is clearly defined"

## Goal & Context
<!-- scope: business -->

Replace the generator's growth stage with one growth rule from trunk to twig in which every catalogue species is a point in a continuous space of architectural settings: a setting moves the tree by degree, and a new species is new coordinates. Each tree must read as its species beside its reference photographs, judged by the owner on stills, and stay fast. [user]

**Why.** Today's growth is two systems joined at a crossover (a rule-built scaffold and a separate twig layer) held together by thresholds and patches: the twig layer's thickness gate (`local/seed.rs:133`), the tip shoot that carried whole twig layers (fn-189 `TRACE.md`), girth patches, and the foliage cull. Today's beech does not read as a beech (owner). [checked, user]

**What 17 probe rounds taught (fn-188 rounds 1 to 9, fn-190 R1 rounds 1 to 5, R1a rounds 1 to 3; reports in the evidence folder, the superseded criteria in `SPEC-HISTORY.md`).** [checked]

- The probes never built a published architectural model. Troll's model in *Fagus* (LITERATURE.md, "Troll's model in *Fagus*", [M98]) builds the trunk as a sympodial stack of modules, each with an erect base and a plagiotropic tip, the next module relaying from a bud in the curvature zone; the probe's beech had a persistent orthotropic leader (`organ.rs:183–218`), which is Massart's trunk, so it drew a fir. Continuity between models is a property of working points, and no point worked.
- Each round added a new mechanism instead of reproducing one model, so a probe bug (two-ranked laterals placed above and below the shoot) survived seven rounds.
- The rounds were reported by their numeric passes while the visual gate failed every round. Numbers explain a look; they never decide it (memory: judge each generator round by its stills).
- Kept results: R1a's single-pass organogenesis engine (`growth_law/organ.rs`) met the time bar (0.27 to 0.97 of it) at 103 to 134% of today's fine wood; light from above with a carbon balance formed a bole by shedding and made size self-limiting (R1 round 4); per-bud competition costs too much at full detail and is deleted.

**The architecture space.** The 23 Hallé–Oldeman models differ in four features (LITERATURE.md §1): orthotropy or plagiotropy (including mixed axes), monopodial or sympodial growth, rhythmic, continuous or diffuse growth, and terminal or lateral flowering. Each becomes continuous settings on one engine. Catalogue models: beech and birch Troll, oak and ash Rauh, spruce Massart, date palm Corner, the plane sympodial with its model unverified. The literature marks the continuous reading as its author's, not a source's (§4.2), so it is a hypothesis this spec tests. [checked]

## Approach

**One engine, one vocabulary, models in parallel.** Every model is built on the same engine with the same settings, so the space is continuous by construction and never a merge of separate generators.

- **The engine** is R1a's single-pass organogenesis in the probe (`crates/telperion-render/examples/growth_law/`, scratch, never merged). Its axis rule is replaced by the vocabulary's settings. Engine changes are serial: one task at a time owns `organ.rs` and the engine modules.
- **The vocabulary** is one table of continuous settings, each with its neutral value, its source quote and the models that use it, for example the angle profile of a module from base to tip, relay readiness and position, rhythm strength, flowering position, phyllotaxis and the lateral position profile along a shoot (acrotony to basitony). The host owns it (AGENTS.md, "Dispatch and escalation").
- **A model is a point:** a values file and its stills, in its own file, so model tasks touch disjoint files and run in parallel worktrees.
- **Every stage ends with stills for the owner:** each model at several ages, seeds 1 and 7, bare and whole, beside its reference photographs and today's tree. The verdict age is mature (AGENTS.md, "Mature trees are the product"); younger ages show the model's development. The host views every still before the owner does; a dispatched worker produces stills and a report and never declares a model passed.
- **What the owner judges:** each model's specification lists the three to five traits visible in its reference photographs (for example leader fate, crown outline, branch angle, bole fraction), and the sheet is judged against that list. A reject names the failed traits. Rejects count per model per stage; a model rejected twice in a stage stops for a decision while the other models continue. Where the owner's eye and a numeric measure disagree, the owner's eye wins.
- **Ownership during parallel work:** the vocabulary (`params.rs`) and the engine are frozen outside their own tasks; scoring and still tooling are read-only to model tasks; a model task writes only its own values file and its own evidence files.
- **A feature found late:** a point task that needs a mechanism no engine task built stops with `NEEDS_HUMAN`; the host mints a serial engine task for it.
- **No regression across approved models:** every engine task re-renders the pinned sheets of every approved point and shows them beside the approved ones; a visible change to an approved model goes back to the owner.
- **A setting that cannot vary by degree** is recorded as a vocabulary gap and goes to the owner, never into the engine as a switch.

**Stages and what runs in parallel.**

| Stage | Work | Parallel? |
|---|---|---|
| A. Read | one model specification per model (beech, oak, spruce, palm): its rules quoted from the sources, mapped to vocabulary settings, and the engine features it needs | yes, no code, one task per model |
| B. Vocabulary | the host merges the four specifications into one table; Astra reviews it | no, host |
| C. Engine | each missing engine feature, one at a time, with the stills of the model that needs it | no, serial on the engine |
| D. Points | each model as a point on the engine, with its stills | yes, one task per model, disjoint files |
| E. Space | walks between the points | no |
| F. Light | light from above and the carbon balance on top | no |
| G. Production | design, port, presets, gates, PR | as planned then |

## Architecture & Data Models
<!-- scope: technical -->

- **Probe engine:** `growth_law/organ.rs` single pass; scorecard `score.rs` and bands `bands.rs` as diagnostics; stills via the headless renderer; references in `.flow/references/<species>/` (49 Commons photographs with licences) and, for the palm, `catalogue/date-palm/packet/references.json` (P-WHOLE, P-TRUNK, P-BASE). [checked]
- **Sources:** LITERATURE.md and its source texts in the research worktree's ignored `.firecrawl/lit/` folder (`nicolini98.md`, `millet.md`, `halle.md`, `hot_ch3.md`, `bc2007.md` and others). Every rule cites the passage it comes from. [checked]
- **What the production rule deletes:** the twig layer as a separate system, its thickness gate and tip shoot, the girth patches (`girthHold`, `girthFall`, `lateralShare` as patches), shedding as structural cleanup, the remaining crown-wall checks in growth, the foliage cull (`pipeline/foliage.rs:255` and `place.wgsl`) and limb clumping as thinning (owner, 2026-10-03: "shouldn't growth just work and not need culling later?"). Dead code is removed; rows that lose meaning are retired by name with a migration note. [user, inferred]
- **Contract with fn-125:** short shoots are grown wood (R1 round 1 decision, kept); expansion places leaves on the wood growth made, with no cull and no clumping thinning. [inferred]
- **Unchanged:** the pipeline's other stages and their contracts; the bias field (fn-175's curl becomes a term in the rule's direction). [inferred]

## Edge Cases & Constraints

- **No switches.** Every setting changes the tree by degree; no value selects a different way of building (`docs/principles.md`). Corner's unbranched stem is the point where branching readiness reaches zero, reached by degree. [principles]
- **No thresholds or walls** where a continuous rule does the job; any limit names its owner and reason. [principles]
- **Errors, not fallbacks.** A tree that collapses, wood below the ground, or an input the rule cannot draw is an error. [principles]
- **Speed is a product requirement** (owner, 2026-10-02): time and fine wood are measured and reported at every stage; growth time at most 1.5 times today's skeleton time at equal or greater fine wood binds from stage E on. Equal detail always; never faster by drawing less. [user]
- **Today's renders are the baseline, never the target** (owner, 2026-10-03), with one exception: **today's date palm is the bar** for the palm, the catalogue's most realistic tree (owner, 2026-10-03). Any visible regression on its stills fails its stage, and its stem moves to the new rule only once its stills match today's; its organs (fronds, persistent leaf bases, skirt, infructescence; fn-108 to fn-111, fn-120, fn-144, fn-155) must still draw. [user]
- **Legendary trees** (STRATEGY.md): a real base species judged on that species' photographs, plus authored character from Tolkien's text, each quote verified before use, through the bias field, scale and colour. Laurelin's base is the European beech, to be confirmed by the verified text; Telperion's base is chosen in G from the verified text with renders compared, and the owner can overrule (owner, 2026-10-03). [user]
- **Ordinary** is the rule's neutral point, checked for plausibility only. [user]
- **Images:** agents view every image a verdict rests on (AGENTS.md, owner 2026-10-03). [AGENTS.md]
- **The probe is never copied into production** (AGENTS.md, "Code rules"): the port is a rewrite with tests. [AGENTS.md]
- **The palm has no fallback stem:** if its stills on the new rule do not match today's, its stage fails and goes to the owner; production never keeps the old stem beside the new rule. [principles]
- **Several models in one spec:** this is a generator spec whose models are architectures, not species onboardings; each species is still tuned by its own spec afterwards (fn-62 and the others), so AGENTS.md's one-species-per-spec rule holds. [AGENTS.md, inferred]

## Acceptance Criteria

- **R1: model specifications.** For the beech (Troll), oak (Rauh, *Q. robur* photographs where *Q. garryana* ones are lacking; the oak preset keeps its catalogue name), spruce (Massart) and date palm (Corner; references in `catalogue/date-palm/packet/references.json`), a specification lists each rule of the model with its quoted source, maps it to vocabulary settings with values, names the engine features it needs, and lists the three to five reference-visible traits the owner judges. Errors: a rule with no source passage is marked unsourced, never built silently. [user, checked]
- **R2: the vocabulary.** One table of continuous settings, each with its neutral value, source and the models that use it, covering the four HOT features and the rules R1 found. Astra reviews it against `docs/principles.md` (no switch, a neutral value, a source for each setting); a finding blocks it until answered. Adding a setting at its neutral value leaves every approved point unchanged. The owner sees it before engine work starts. [user]
- **R3: each model reproduced.** On the shared engine, the beech, oak, spruce and palm each read as their model and their species: stills at several ages (for example 5, 10 and 20 years and mature), seeds 1 and 7, bare and whole, beside their reference photographs and today's tree. The owner approves each model's mature sheet against its trait list; the palm also meets today's palm. Time, fine wood and the scorecard are reported as diagnostics. Errors: a collapsed tree or wood below the ground is a driver error. [user]
- **R4: the space is continuous.** Walks of at least 9 steps between the points (beech to spruce, spruce to oak, oak to palm) are shown as still strips; the owner sees no jump, and the scorecard's no-jump measure (no measure changes by more than three times its median step) is reported; the owner's eye decides. Growth time is within the bar at equal fine wood from here on. [user]
- **R5: light and the carbon balance.** One light pass from above per cycle and shedding on each branch's balance form the bole and limit size (seeds within 1.5× of each other in node count) without making any approved model's stills worse: every R3 sheet is re-rendered and the owner re-approves each model; the walks stay continuous; growth time is within the bar. [inferred]
- **R6: production design.** Recorded in this spec before production code: where the rule lives in `pipeline` (stage 2), the vocabulary as catalogue rows with neutral values and windows, the deletion list, the fn-125 contract, the corrected two-ranked lateral rule, and every catalogue preset as coordinates, including Telperion's base species and the Two Trees' authored character with verified quotes. Astra reviews it. [inferred]
- **R7: the port.** The rule replaces the growth stage for every preset; the deletions are made; no direct-build path runs the twig layer, its gate, the tip shoot, the girth patches or the cull. A test walks every new setting and shows the tree changes by degree; off-rail values are refused by name; rows are declared once, blended and on the dials. [inferred]
- **R8: every preset.** Beech, oak, spruce and palm as approved in R3; birch, ash and plane against their references; Telperion and Laurelin on their base species plus authored character; ordinary as the neutral point. Stills of every preset beside today's and the references, owner-approved. [user]
- **R9: speed and size.** Growth time per preset at most 1.5 times today's skeleton time (warm median, seeds 1 and 7) at equal or greater fine wood; full build at equal or greater leaf count no slower than today; peak memory reported; every shipped artifact within its CI size budget. [user]
- **R10: gates.** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; the Codex implementation review passes. [AGENTS.md]
- **R12: the remaining catalogue models.** Specifications as in R1 for the silver birch (Troll), European ash (Rauh) and London plane (sympodial, model unverified), each placed as a point on the engine or flagged with the engine feature it lacks; and the Two Trees' authored character from Tolkien's text, every quote verified against the source, with Laurelin's base confirmed and Telperion's candidates compared. [user, inferred]
- **R11: the owner's verdict.** The PR is opened for the owner's visual verdict on the R8 stills and is not merged without it. [user]

## Early proof point

Task fn-190-one-growth-law-species-are-points-in-a.1 validates the core approach (Troll's module and relay on the shared engine draw a beech the owner recognises at maturity). If it fails, re-evaluate the engine's axis rule before the vocabulary and the other models' points.

## Quick commands

```bash
cargo build --profile ci -p telperion-render --example growth_law
target/ci/examples/growth_law grow european-beech 1 crates/telperion-render/examples/growth_law/models/european-beech.json /tmp/beech.json /tmp/beech
```

## Boundaries

- Not the species runner or its score (owner, 2026-10-03). [user]
- Not the cost of leaf placement in expansion: fn-125's (and fn-126's); this spec only agrees the leaf contract. [inferred]
- Not species tuning beyond expressing the catalogue presets: fn-62 tunes the beech through the runner afterwards. [inferred]
- Not growth over time (removed by fn-181); a later growth feature may build on this rule. [inferred]

## Decision Context

- **Supersedes** fn-182, fn-189, fn-172 and fn-173's growth-side targets; fn-188 closed into this spec with its evidence. [user, 2026-10-03]
- **Carries over** fn-175's curl as a term in the rule's direction. [inferred]
- **fn-125 proceeds in parallel** with the leaf contract above. [user, 2026-10-03]
- **fn-62 (the beech)** depends on this spec. [inferred]
- **The replan of 2026-10-03** replaced the numeric R1 with staged, owner-judged reproduction of each model on one engine, models in parallel; the superseded criteria and decisions are in `SPEC-HISTORY.md`. [user]

## Strategy Alignment

- Changes STRATEGY.md's "Our approach" line "space colonization for the crown, botanical rules below the crossover" to one growth rule from trunk to twig with species as coordinates in an architecture space; R11's merge carries that wording change with the owner's verdict. [strategy:Our approach]

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | model specifications. For the beech (Troll), oak (Rauh, *Q. robur* photographs where *Q. garryana* ones are lacking; the oak preset keeps its catalogue name), spruce (Massart) and date palm (Corner; references in `catalogue/date-palm/packet/references.json`), a specification lists each rule of the model with its quoted source, maps it to vocabulary settings with values, names the engine features it needs, and lists the three to five reference-visible traits the owner judges. Errors: a rule with no source passage is marked unsourced, never built silently. | fn-190-one-growth-law-species-are-points-in-a.1, fn-190-one-growth-law-species-are-points-in-a.2, fn-190-one-growth-law-species-are-points-in-a.3, fn-190-one-growth-law-species-are-points-in-a.4 | — |
| R2 | the vocabulary. One table of continuous settings, each with its neutral value, source and the models that use it, covering the four HOT features and the rules R1 found. Astra reviews it against `docs/principles.md` (no switch, a neutral value, a source for each setting); a finding blocks it until answered. Adding a setting at its neutral value leaves every approved point unchanged. The owner sees it before engine work starts. | fn-190-one-growth-law-species-are-points-in-a.5 | — |
| R3 | each model reproduced. On the shared engine, the beech, oak, spruce and palm each read as their model and their species: stills at several ages (for example 5, 10 and 20 years and mature), seeds 1 and 7, bare and whole, beside their reference photographs and today's tree. The owner approves each model's mature sheet against its trait list; the palm also meets today's palm. Time, fine wood and the scorecard are reported as diagnostics. Errors: a collapsed tree or wood below the ground is a driver error. | fn-190-one-growth-law-species-are-points-in-a.1, fn-190-one-growth-law-species-are-points-in-a.10, fn-190-one-growth-law-species-are-points-in-a.6, fn-190-one-growth-law-species-are-points-in-a.7, fn-190-one-growth-law-species-are-points-in-a.8, fn-190-one-growth-law-species-are-points-in-a.9 | — |
| R4 | the space is continuous. Walks of at least 9 steps between the points (beech to spruce, spruce to oak, oak to palm) are shown as still strips; the owner sees no jump, and the scorecard's no-jump measure (no measure changes by more than three times its median step) is reported; the owner's eye decides. Growth time is within the bar at equal fine wood from here on. | fn-190-one-growth-law-species-are-points-in-a.12 | — |
| R5 | light and the carbon balance. One light pass from above per cycle and shedding on each branch's balance form the bole and limit size (seeds within 1.5× of each other in node count) without making any approved model's stills worse: every R3 sheet is re-rendered and the owner re-approves each model; the walks stay continuous; growth time is within the bar. | fn-190-one-growth-law-species-are-points-in-a.13 | — |
| R6 | production design. Recorded in this spec before production code: where the rule lives in `pipeline` (stage 2), the vocabulary as catalogue rows with neutral values and windows, the deletion list, the fn-125 contract, the corrected two-ranked lateral rule, and every catalogue preset as coordinates, including Telperion's base species and the Two Trees' authored character with verified quotes. Astra reviews it. | fn-190-one-growth-law-species-are-points-in-a.14 | — |
| R7 | the port. The rule replaces the growth stage for every preset; the deletions are made; no direct-build path runs the twig layer, its gate, the tip shoot, the girth patches or the cull. A test walks every new setting and shows the tree changes by degree; off-rail values are refused by name; rows are declared once, blended and on the dials. | fn-190-one-growth-law-species-are-points-in-a.15 | — |
| R8 | every preset. Beech, oak, spruce and palm as approved in R3; birch, ash and plane against their references; Telperion and Laurelin on their base species plus authored character; ordinary as the neutral point. Stills of every preset beside today's and the references, owner-approved. | fn-190-one-growth-law-species-are-points-in-a.16 | — |
| R9 | speed and size. Growth time per preset at most 1.5 times today's skeleton time (warm median, seeds 1 and 7) at equal or greater fine wood; full build at equal or greater leaf count no slower than today; peak memory reported; every shipped artifact within its CI size budget. | fn-190-one-growth-law-species-are-points-in-a.17 | — |
| R10 | gates. `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; the Codex implementation review passes. | fn-190-one-growth-law-species-are-points-in-a.17 | — |
| R12 | the remaining catalogue models. Specifications as in R1 for the silver birch (Troll), European ash (Rauh) and London plane (sympodial, model unverified), each placed as a point on the engine or flagged with the engine feature it lacks; and the Two Trees' authored character from Tolkien's text, every quote verified against the source, with Laurelin's base confirmed and Telperion's candidates compared. | fn-190-one-growth-law-species-are-points-in-a.11 | — |
| R11 | the owner's verdict. The PR is opened for the owner's visual verdict on the R8 stills and is not merged without it. | fn-190-one-growth-law-species-are-points-in-a.17 | — |
