---
satisfies: [R1]
---
# fn-217-tree-space-the-supernatural-group-on.1 The group's design: facts and options for each trait

## Description
R1's first half. For each trait the brief names (scale; two-sided leaf colour with an edge term; emitted light from flowers with a time cycle, two trees in counterphase mingling at the crossover; a falling dew of light) and for the two blossom follow-ups (five-petal floret form; blossom reading as a mass at whole-tree distance): what exists today in code (fn-10's supernatural group, the bias field, material rows, the organ draw from fn-216/fn-229, the renderer's light model), what each needs (engine rows or renderer terms), options with costs (frame cost for emitted light measured or estimated from the pass structure), each dormant at its neutral value. Read the brief VISION-TWO-TREES.md in fn-198's evidence. Write DESIGN.md in fn-217's evidence; no product code. Host decides; Astra reviews before building (R1).

## Acceptance
- DESIGN.md: per trait, today's code (file:line), options, recommendation, neutral value, cost.
- No product code changed.

## Done summary
DESIGN.md (.flow/evidence/fn-217-tree-space-the-supernatural-group-on/) covers each trait: scale, the two-sided leaf with an edge, emitted light, the cycle in counterphase, and the dew. It also covers the two blossom follow-ups. For each it gives today's code with file:line, the options with frame-cost estimates, a recommendation and the neutral value, and it ends with 12 numbered decisions for the host. Key facts:

- fn-10's group is read only by the test-only scaffold, so it changes no engine tree.
- telperion and laurelin have no /space rows.
- The renderer holds one tree, has no clock and no HDR target, and tone-maps each fragment.

No product code changed.

baseline: none (evidence-only task; the spec defines no Quick commands)
stage: impl-review - skipped(config: REVIEW_MODE=none)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7e89f8a3a54b34cc130bc287b9b72e8da708e01d
- Tests: flowctl gate classify (TIER_B docs-only; no Quick commands defined)
- PRs: