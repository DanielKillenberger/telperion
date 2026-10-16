---
satisfies: [R3, R7]
---
# fn-6-branch-generations-below-the-crossover.6 The harness and the two trees: the dial retired, the read-out, no depth stated

## Description
Retire the orders dial for a derived read-out, restate both presets without a depth, and take the owner's R7 verdict.

## Acceptance
- Panel shows derived generations, handoffs, capped handoffs and twig marks; presets round-trip; owner judges in clay.

## Done summary
Panel exposes lengthRatio, ratioPower, internodes, laterals and limbRadius, and reports the derived branch stats. The owner's first R7 pass on this build returned "not yet", which became task 8.

## Evidence
- Commits: 6dd6081
- Tests: npx vitest run (345 passed), npx tsc --noEmit
- PRs:
