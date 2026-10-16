# fn-151-the-design-principles-are-checked.1 The design principles are checked before a push

## Description
Hold the design principles by structure, a test and a size budget.

## Acceptance
Spec R2, R3, R8 in their revised form (R1 to fn-152; R4, R5, R7 to fn-156; R6 withdrawn).

## Done summary
One ordinary test builds every shipped preset's skeleton, surface, leaves, field and structure through the pipeline, beside the main Wasm and slim entry tests; CI's package job holds every artifact to `scripts/artifact-budgets.json`; STRATEGY.md, AGENTS.md, `docs/pr-format.md` and `docs/principles.md` carry the guidance. The policing layer and Jev reviewer first built are in git history at `061ff37d` and `132257f3`; measurements in EVALUATION.md.

## Evidence
- Commits: 9734cb0c29f7b529bd9d12ddcb7b55bf74071e35, 70c0dd230119916337d1e2c11c7b81205e37fdea, dd895923f1249351ccf8fa1ab97978db7bd61daa, 40491095860d1ce409bb3abc844d317db117054b, 132257f3577adc99d29628d80c7f526d1d4e981e, 2800846c0627f34e7cb76dbfbfda46c4dd2b7e66, 061ff37daa6ce52a9dd2fcf67e44cd81ec153e55, bde67062f81adef128ea5108ef6b1417cd3c7e94
- Tests: cargo test --profile ci --workspace --no-fail-fast (954 passed); npm test (129); node scripts/artifact-budgets.mjs; slim entry leg red on 39348def
- PRs:
