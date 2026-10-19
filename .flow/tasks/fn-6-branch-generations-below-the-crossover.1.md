---
satisfies: [R1, R2, R4]
---
# fn-6-branch-generations-below-the-crossover.1 The branch law, stated and measured before anything grows

## Description
State the branch law and fixed twig as a pure module, review it against the literature, and measure the tuft baseline before anything grows.

## Acceptance
- Law, twig anatomy and generations-until-twig stated with sources; fill metrics (shell occupancy, tip clustering) and tuft baseline measured on both presets.

## Done summary
`src/skeleton/law.ts` and fill metrics landed. Every handoff reaches twig radius in 5 generations; estimates 21,556 (Telperion) and 61,098 (Laurelin) nodes, inside the ceiling; Telperion's 79 cm wood gets ~21 m of first-branch reach. Sources and caveats recorded in the spec.

## Evidence
- Commits: ef90a93
- Tests: npx vitest run (338 passed), npx tsc --noEmit
- PRs:
