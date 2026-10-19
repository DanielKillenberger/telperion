---
satisfies: [R1, R3, R7, R10, R14]
---
# fn-14-bark-and-foliage-appearance.4 The look: bark and leaf colour, per-leaf variation, crown depth, sky, tone map and the clay view

## Description
The look: shaders read the material and scene rows, per-leaf colour offsets and interior darkening.

## Acceptance
- Bark and leaves lit from the rows; per-leaf seeded colour; interior darkening by crown depth; clay view unchanged.

## Done summary
Bark takes colour and roughness from the row under sun, hemisphere and shadow; each leaf takes a seeded colour offset by placement index; interior leaves darken by depth into the crown; filmic tone map on the lit frame only.

## Evidence
- Commits: 5fdbf0a1bb13a7dfc28e5344e3cc6d84093d7ef2
- Tests: cargo test --release --workspace; npm test; npm run typecheck
