---
satisfies: [R5]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.11 Astra's implementation review: nine findings

## Description
The spec's R5 review. Astra's findings are in raw/ASTRA-REVIEW-IMPL.md (copy it beside TASK10.md as ASTRA-REVIEW-IMPL.md): deferred destroys drained by an unrelated submit; Oversize unwrapped into panics; tied widest nodes publishing mixed coordinates; fine foliage bypassing max_instances; octahedral sign(0) reversing frames; divergence narrowed to f32 unreduced; zero leaf size still placing instances; lobed surfaces and surface_contact=0 refused by the fine path; test-only run_words kept alive in production. Each finding is verified by a test that fails before the fix (a finding that does not reproduce is recorded with the attempt) and then fixed.

## Acceptance
- Each of the nine findings reproduced red then fixed green, or shown not to reproduce.
- Gate (two parts, render first) and npm test green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
