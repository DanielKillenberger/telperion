---
satisfies: [R2, R4, R6]
---
# fn-270-growth-continues-from-any-age-in-less.6 A grow-in past the budget is refused by name, never a crash

## Description
Owner, 2026-10-14: growing an 80-year Norway spruce in the harness crashed the tab out of memory. Reproduce natively first (peak device and wasm memory of the spruce grown in to 80, whole years, as the harness does: growth state, settled tree and drawn tree together), find which memory the guard misses, then: the guard charges what the grow-in holds and refuses by name before the browser runs out, the drawn tree untouched; and the spruce at 80 grows if the measured fix (R4's levers) brings it under the browser budget.

## Acceptance
A native repro red before and green after; the guard's charge matches the measured peak within its stated margin; the spruce at 80 either grows or is refused by name with the measured numbers; gate, npm, review.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
