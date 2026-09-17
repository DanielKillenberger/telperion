# Calibrated growth: the oak and the spruce through time

> **Shelved 2026-09-18.** Growth over time is a hidden feature since fn-65: the mature tree is the product and the harness draws the direct build. This spec resumes only when the owner un-hides growth; until then it is not ready and not in progress. The reason is recorded in CLAUDE.md under "Mature trees are the product".

## Conversation Evidence

> user (2026-09-13): "should we have split the spec?"
> user (2026-09-13, on splitting calibration and the stills out of fn-11 so fn-11 lands after its bindings and the page, and holding the research-heavy part for fresh quota): "alright let's do it"
> user (refine of fn-11, 2026-09-12): "we should try and find open-grown curves and match them as closely as possible. The closer to reality the better. We'll judge what rules over what once we get there."
> user (refine of fn-11, 2026-09-12): "age parameter has to apply to any kind of tree. Spruce and oak we calibrate specifically. But growing should [work] for any tree parameter set."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 45% [paraphrase], 15% [inferred] -->

Growth over time (fn-11) delivers the machine: a deterministic specimen with an age, a chronicle that reads at any age, bindings and a harness page where a tree grows live. It does not deliver a tree that grows like an oak or a spruce. The growth traits it carries are placeholders, the mature tree grown through time is not yet the tree today's one-shot build makes, and nobody has judged an age strip. This spec is that judgment: the oak and the spruce calibrated against open-grown height and trunk-diameter curves, the production build routed through growth and re-pinned once, the age strips and mature stills rendered, and the owner's verdict recorded. [paraphrase]

It was split out of fn-11 on 2026-09-13 because it is a different activity from building the machine: sourcing and checksumming published curves, fitting numeric traits, and looking at trees, with a failure mode of its own, a missing curve or a rejecting eye. Keeping it separate lets fn-11 land as a pull request when its bindings are done, and lets this work start on fresh quota. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **Curves by named composition.** No single open-grown age curve is published for Quercus garryana or Picea abies, so the reference is composed as fn-11's amended R8 states: an age axis from a stand-grown source converted to open-grown proportions through open-grown allometry, each provenance recorded, the height-to-diameter ratio stated per age, unless the Urban Tree Database or another age-recorded open-grown source lists the species, in which case it is used directly with its urban caveat. The sources, checksums and the search already done live in fn-11's `## Resolved via Research`, and that section is the starting point, not a search to redo. [paraphrase]
- **Traits, not tables.** Calibration sets each family's growth rate and shape, and if needed its shedding threshold and tolerance period and leaf lifetime, so that the specimen meets the curves at three ages; the traits stay numeric rows the blend walks, and no species branch enters the growth code. The derived mature age of each preset is documented, never authored. [paraphrase]
- **Routing and the re-pin.** Once calibrated, production generation is the growth path built to the derived mature age; the identity pins are re-pinned once with the reason recorded, counts, bounds and element pins moving only by amounts the convergence numbers explain. [paraphrase]
- **The strips and the stills.** An age strip per species from a young age to maturity, and the mature oak and spruce beside fn-14's stills, rendered through the headless example, within the owner's capture budget. [paraphrase]
- **The report** in fn-14's shape, carrying fn-11's measured costs from its commit record and this spec's curves, deviations and verdict slots. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **A missing curve is recorded, never invented**, with its stand-grown fallback named; a miss outside the tolerance stops the spec with the number unless the owner's recorded judgment accepts it. [paraphrase]
- **Where curve and look disagree**, the deviation is recorded and the owner judges which rules at that age. [user]
- **Convergence is stated plainly**: the calibrated tree's node count, crossover and bounds against today's one-shot build for both species, before any pin moves. Last known, uncalibrated: nodes +29% oak and −15% spruce, crossover +50% and +22%, bounds within 2.3% and 6%. [paraphrase]
- **Every family still grows.** The Two Trees and the ordinary tree build and grow at every age under the same rule, with no reference or verdict applied to them here. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The oak and the spruce at their calibrated growth traits follow height and trunk-diameter curves at three ages each, young, middle and mature, within 15 percent, with each curve's source, URL and SHA-256 in the report and the composition's provenance stated per age; a miss outside the tolerance stops the spec with the number unless the owner's recorded judgment accepts it. [user]
- **R2:** Production generation is routed through the growth path built to the derived mature age, the identity pins are re-pinned once with the reason recorded, and the convergence numbers against today's build are stated for both species before the pins move. [paraphrase] Errors: a pin moving by an amount the convergence numbers do not explain is a defect, not drift.
- **R3:** An age strip per species from a young age to maturity and the mature oak and spruce beside fn-14's stills are rendered through the headless example within the capture budget, and the owner accepts them in the owner's own words, recorded in this spec. [user] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R4:** The report, in fn-14's shape, carries fn-11's measured costs, the curves and their checksums, every deviation the owner may judge, and the verdict slots. [inferred]

## Boundaries
<!-- scope: business -->

- No change to fn-11's growth rule, chronicle or determinism contract; calibration sets trait values and routing. [paraphrase]
- No death, seasons, environment or damage response. [user]
- No smoothing between years and no page work; those are fn-28's. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner values realism first: open-grown curves matched as closely as possible, with the owner judging the residual. [user]
- Split from fn-11 on 2026-09-13 so the machine lands on its own and the research-heavy, judgment-heavy part gets its own brief, its own run and fresh quota; the plan review of fn-11 had proposed a split that was not applied. [paraphrase]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | the one task |
