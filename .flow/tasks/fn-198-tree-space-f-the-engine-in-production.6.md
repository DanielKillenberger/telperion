---
satisfies: [R1]
---
# fn-198-tree-space-f-the-engine-in-production.6 The envelope measured whole, trunk ages by degree, one identity check

## Description
INTEGRATION.md host decisions 11 and 12 and task 5's decision on the identity check (2026-10-07): (11) shoulder, irregularity and lobe scale are measured from the grown structure like height, crown base, spread and fullness, and the family defaults no longer stand in (the leaf cull and the leaf box read them); (12) each preset's trunk ages become a 0-1 weight per age instead of a boolean, read by the engine by degree, so blending two presets changes the tree continuously (walk-test the blend; docs/principles.md); (13) a debug assertion in lineage_order that no two axes share a lineage. Trees change only where 11 or 12 says, measured and shown on stills of the four species at 40 and 80, viewed.

## Acceptance
- The three envelope rows measured; no family default for them; stills before and after viewed. - Trunk ages as weights; a walk test across a blend of two presets shows no jump; the four presets' trees unchanged where their weights are 0 or 1. - The debug assertion with a test. - npm test and the workspace gate green.

## Done summary
Host decision 13: `lineage_order` debug-asserts that no two axes share a lineage, with tests (a62365d3). The predictor's NaN: an undrawn PA's counts were NaN, and the eager hands table read them. Growth never queried such a cell, so no bud was wrongly classified. An undrawn PA now has no count (Option), with a test in the gate's profile; digests and measures hash 80 are unchanged (db73ae95). Decision 11a: the grown envelope's shoulder is the least shoulder whose outline holds the crown, irregularity is 0, and no family shape is used (24005de7, f7d227ee, 8dc58e0a). No shipped tree's leaves change; the palm stills are byte-identical and were viewed. Decision 12a moves the trunk weights and the walk test to R0 (spec R0 list). Gate green after the limit inventory; npm test green.

## Evidence
- Commits: a62365d3, f577eae1, db73ae95, 24005de7, f7d227ee, 8dc58e0a
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, cargo test -p telperion-space --lib canonical
- PRs: