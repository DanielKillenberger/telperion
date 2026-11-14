---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-336-sheets-of-020s-trees-as-they-stand.1 Implement Sheets of 0.2.0 trees as they stand

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Nineteen retained JPEG sheets of the full cycle (fidelity 0), 4.1 MiB, in `.flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand/`. Eighteen show the six real presets at age 40 on seeds 1, 7 and 42, one sheet per view (whole, mid, close), with the matching reference photographs or a line saying there are none. One shows the oak at 120, 200 and 300 with fn-332's probe printed under each tree. RESULT.md holds the view mapping, the screening notes, the time and memory per tree, and R4's question. No crate change. The grove on the voxel style is not drawn: no headless entry renders it, so R1's grove view is returned to the host, not built.

R3: every sheet was viewed. Two old-oak stills were clipped (seed 7 at 200 and 300). They were remade with the existing headless still and a pulled-back `--camera`; the matching leaf counts confirm the same trees. R6: no tooling was changed. Two tooling gaps are named in FRICTION.md for the host: a headless voxel still, and a camera row on the space still examples.

R4 draft for the goal's log: "What reads wrong on each sheet, in your own words? One line per sheet is enough. The sheets are under `.flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand/`: oak `sheet-oak-whole.jpg`, `sheet-oak-mid.jpg`, `sheet-oak-close.jpg`; spruce `sheet-spruce-whole.jpg`, `sheet-spruce-mid.jpg`, `sheet-spruce-close.jpg`; beech `sheet-beech-whole.jpg`, `sheet-beech-mid.jpg`, `sheet-beech-close.jpg`; birch `sheet-birch-whole.jpg`, `sheet-birch-mid.jpg`, `sheet-birch-close.jpg`; date palm `sheet-palm-whole.jpg`, `sheet-palm-mid.jpg`, `sheet-palm-close.jpg`; ordinary tree `sheet-ordinary-whole.jpg`, `sheet-ordinary-mid.jpg`, `sheet-ordinary-close.jpg`; the old oak at 120, 200 and 300 `sheet-oak-old-whole.jpg`. No grove sheet: no headless entry draws the voxel style yet."

Tier: implementer (project routing block, AGENTS.md "Model routing": implementer: opus) (model: claude-opus-5-5)

stage: impl-review - skipped(policy: risk - commits hold only .flow/ evidence, no crate change (fn-322))

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: eb79cead90422dc3ff05d7f0a579a8a4dadb7f4c
- Tests: GATE_SKIPPED:none:docs-only - tier-B, spec defines no Quick commands; stills via probes/stills.sh in telperion-heavy
- PRs: