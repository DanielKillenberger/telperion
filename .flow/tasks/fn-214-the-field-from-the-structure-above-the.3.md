---
satisfies: [R3]
---
# fn-214-the-field-from-the-structure-above-the.3 The coarse field is the field package's default

## Description
Owner decision (2026-10-08): the field package grows the coarse tree by default, so killenberger.com's unchanged growField call gets it; a field only answers voxel-scale questions. The default cut is stated with its reason; an explicit cut still overrides; cut 0 stays the full tree for callers that ask. Then the site's own call pattern (FIELD-PATH.md: n = 24, 30, 36, 48; oak, birch when it exists, spruce, palm; seeds 1234 and 5678) measured end to end on the final branch against 0.1.4, Wasm one thread, at low load; the portrait stills viewed.

## Acceptance
- Default cut in the field package, documented; explicit cut overrides.
- Every species the site uses at or under 0.1.4 end to end on its own call, or the gap reported by species.
- Stills viewed; gate and npm test green.

## Done summary
Not implemented: superseded (host, 2026-10-16, Astra's release review); see the spec's 'Closed as superseded'.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits:
- Tests:
- PRs: