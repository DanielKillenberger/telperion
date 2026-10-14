---
satisfies: [R3, R4]
---
# fn-281-the-oak-at-80-builds-at-every-seed.2 Fix A: wood drawn above a join thinner than FINEST is not drawn

## Description
The host's rulings of 2026-10-14 (spec Decisions): the conversion and dress.wgsl judge min(p.radius, above) against FINEST; a device check on the oak at 80, seeds 3, 7, 30, 39; the sweep re-run with byte digests against the base; the 486 nodes of seed 7 counted.

## Acceptance
R3 and R4: the red test green, the device check green, the sweep with no unsolved radii and every byte change listed with nodes removed; codex review SHIP.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
