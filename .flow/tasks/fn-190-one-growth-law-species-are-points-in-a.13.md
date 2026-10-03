---
satisfies: [R5]
---
# fn-190-one-growth-law-species-are-points-in-a.13 Light from above and the carbon balance

## Description
Stage F: round 4's mechanics on the reproduced models.

**Size:** M
**Files:** `crates/telperion-render/examples/growth_law/organ.rs`, a light module restored from history (`light.rs` at fbd82732), `params.rs`
**Touches:** [crates/telperion-render/examples/growth_law/**]

### Approach
- One Beer-Lambert pass per cycle from above; shedding on each branch's remembered balance; neutral values leave every approved point unchanged.
- Re-render every R3 sheet and the walks; the owner re-approves each model.

### Investigation targets
**Required:**
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/R1-ROUND4.md`
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/SPEC-HISTORY.md` (host decisions rounds 2 to 4)

## Acceptance
- [ ] Bole by shedding and seed sizes within 1.5x on every point
- [ ] Every R3 sheet re-rendered; the owner re-approves each model
- [ ] Walks stay continuous; time within the bar

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
