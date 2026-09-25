# fn-151-the-design-principles-are-checked.1 The design principles are checked before a push

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The design principles are enforced by structure, not policing (owner, 2026-09-25). One ordinary test builds every shipped preset's skeleton, surface, leaves, field and structure through the pipeline, beside the main Wasm and slim field entry tests; CI's package job holds every shipped artifact to scripts/artifact-budgets.json. STRATEGY.md, AGENTS.md, docs/pr-format.md and docs/principles.md carry the guidance and name the two sanctioned exceptions.

R2, R3 and R8 are met in their revised form. R1 moved to fn-152, R4, R5 and R7 to fn-156, and R6 (the hook) was withdrawn. The policing layer and the Jev reviewer that fn-151 first built are in git history at 061ff37d and 132257f3, with the measurements in .flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 9734cb0c29f7b529bd9d12ddcb7b55bf74071e35, 70c0dd230119916337d1e2c11c7b81205e37fdea, dd895923f1249351ccf8fa1ab97978db7bd61daa, 40491095860d1ce409bb3abc844d317db117054b, 132257f3577adc99d29628d80c7f526d1d4e981e, 2800846c0627f34e7cb76dbfbfda46c4dd2b7e66, 061ff37daa6ce52a9dd2fcf67e44cd81ec153e55, bde67062f81adef128ea5108ef6b1417cd3c7e94
- Tests: cargo test --profile ci --workspace --no-fail-fast (structural version: 954 passed, 0 failed, 21 ignored, 127 suites), npm test (129 passed, 12 files, the budget test included), node scripts/artifact-budgets.mjs on npm run build (all six artifacts at their 0.1.4 sizes), slim entry leg on 39348def (red: preset date-palm, invalid input: family without a leaf plan), baseline: none run (AGENTS.md: the gate runs once, at the end)
- PRs: