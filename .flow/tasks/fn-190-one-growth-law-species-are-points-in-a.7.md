---
satisfies: [R3]
---
# fn-190-one-growth-law-species-are-points-in-a.7 Beech point: values and mature sheet for the owner

## Description
Stage D for the beech, in parallel with the other points.

**Size:** S
**Files:** `crates/telperion-render/examples/growth_law/models/european-beech.json`, evidence `POINT-BEECH.md`
**Touches:** [crates/telperion-render/examples/growth_law/models/european-beech.json, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/POINT-BEECH.md, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/point-beech/**]

### Approach
- Values in the model's JSON file, read by `Params::apply`; no engine or tool edits (a missing mechanism stops with `NEEDS_HUMAN`).
- Stills at several ages and mature, seeds 1 and 7, bare and whole, beside `.flow/references/european-beech/` and today's beech; the worker views every still and reports each trait from `MODEL-BEECH.md`; it never declares the model passed.

### Investigation targets
**Required:**
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-BEECH.md`
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/run.sh`

## Acceptance
- [ ] The values file reproduces the beech's model on the engine
- [ ] The mature sheet and age stills are rendered, viewed and reported trait by trait
- [ ] Time and fine wood reported
- [ ] The owner's verdict is recorded

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
