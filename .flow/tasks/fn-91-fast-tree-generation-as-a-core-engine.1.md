---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.1 Implement fast tree generation as a core engine capability

## Description
TBD

## Acceptance
Every R-ID in the parent spec acceptance criteria is satisfied; judge this task against the spec criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## Remaining blocker after invocation 5

NEEDS_HUMAN — the five-invocation implementation budget is exhausted; task remains in_progress. The experimental shared GPU/browser path is checkpointed, but R5 is not satisfied: comparable desktop completed-frame warm gains are 3.24×/3.44× for oak seeds 1/7 (below 10×), 15.23×/15.47× for spruce, and all four exceed the 100 ms stretch. Native GPU CPU-output peak memory remains above pure CPU output; browser/device total-memory qualification and phone evidence are absent. Visual acceptance remains scoped to actual reviewed captures, not all supported views/motion. See BROWSER-CANDIDATE.md, FOLLOWUP-CANDIDATE.md and their raw evidence. Host/owner must decide further scope/budget; no sixth attempt or completion is authorized by this checkpoint.
