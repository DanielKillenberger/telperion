---
satisfies: [R4, R10]
---
# fn-14-bark-and-foliage-appearance.2 The material row on the family and the scene row on the renderer

## Description
Add the material row to the family and the scene row to the renderer.

## Acceptance
- Material and scene rows are numeric, validated by field name and blended; headless and page take a scene row.

## Done summary
Every preset carries a material row (bark colour and roughness, leaf front/back colour, hue and brightness offsets, interior darkening); the blend walks all fifteen fields. The renderer holds a scene row with an outdoor midday default, set by `--scene <json>` headless and `setScene` on the page; the panel shows both through generic numeric controls.

## Evidence
- Commits: 0d3a3f42761061f14ae0e0de90448e5b69366974, 8e001cdacae1ce654c0e9f8d2a13bcf83e7e10ab
- Tests: cargo test --release --workspace; npm run wasm:build && npm test; npm run typecheck
