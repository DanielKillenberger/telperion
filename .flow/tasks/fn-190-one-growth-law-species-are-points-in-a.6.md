---
satisfies: [R3]
---
# fn-190-one-growth-law-species-are-points-in-a.6 Engine features from the vocabulary (serial; host splits per feature)

## Description
Stage C placeholder: the vocabulary task splits this into one task per missing engine feature. Each runs alone on the engine.

**Size:** M
**Files:** `crates/telperion-render/examples/growth_law/organ.rs`, `params.rs`, `tree.rs`
**Touches:** [crates/telperion-render/examples/growth_law/organ.rs, crates/telperion-render/examples/growth_law/params.rs, crates/telperion-render/examples/growth_law/tree.rs]

### Approach
- Implement each feature as one continuous setting with its neutral value; a neutral setting leaves every approved point's output unchanged (byte-identical sheet metrics).
- Re-render the pinned sheets of every approved point and of the models needing the feature; show them beside the approved ones.

### Investigation targets
**Required:**
- `crates/telperion-render/examples/growth_law/organ.rs`
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/VOCABULARY.md`

## Acceptance
- [ ] Every engine feature the vocabulary names is built, one at a time
- [ ] Each is one continuous setting; neutral leaves approved points unchanged
- [ ] Approved points' sheets are re-rendered and viewed after each feature

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
