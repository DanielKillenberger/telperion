---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-29-colour-cavity-and-occlusion.1 Implement Colour, cavity and occlusion

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## NEEDS_HUMAN (host, 2026-09-14)

NEEDS_HUMAN: R5 owner judgment on .flow/evidence/fn29/stills — and first the R6 number: the oak's native total p50 is 4.0005 ms (p95 4.3791) against the 3.8 ms bound, 0.2005 ms over, on the exact fn-26 protocol (RTX 3080, Vulkan, seed 7, 1600x1000, 120 measured frames, verdict valid; fn-26 final 3.6879/4.0284). The first valid run was 3.9549 ms; the cost cut in 47b1af8 did not recover it. The browser orbit was not run and the eight R5 stills are unrendered: codex stopped at the spec's R6 rule before capture, so the owner-verdict table in .flow/evidence/fn29/REPORT.md is blank and stills.json is empty.

Implementation is complete and every gate is green at a0fc5b1 (fmt, clippy, cargo test 314/0 with zero adapter skips, npm 77/0, typecheck). The owner's choices: accept a bound of about 4.0 ms for the oak with every term on; ask for a further cost cut in the same codex session (six of ten commits used); or order the eight stills rendered first so the R5 judgment informs the R6 decision. No bound, protocol, tolerance or pinned number was moved.
