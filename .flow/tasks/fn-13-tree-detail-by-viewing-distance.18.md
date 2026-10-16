---
satisfies: [R1, R3, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.18 Qualify template-driven foliage input across tree and leaf shapes

## Description
Qualify template-driven foliage input across tree and leaf shapes.

## Acceptance
- Small early compatibility gate on spruce and oak with honest failures preserved.

## Done summary
Early gate qualified on retained spruce references and a bounded resolved-source oak input; the coarse oak inside failure (-64.1% coverage) is preserved, and the caller-owned input restores exact local anatomy with conservative bounds.

## Evidence
- Commits: 5436ba345c7915371556de26739dae07ae5e300a
- Tests: npm run typecheck; vitest run
