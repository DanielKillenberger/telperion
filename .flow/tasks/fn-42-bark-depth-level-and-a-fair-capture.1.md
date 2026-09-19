---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-42-bark-depth-level-and-a-fair-capture.1 Implement bark depth, level and a fair capture

## Description
Implement fn-42 directly against its current R1–R5 requirements. The owner selected qualitative reference matching on 2026-09-19; the earlier missing-photo and reference-contract blockers below are historical and resolved. Work is isolated on the fn-42 branch. Correct bark parallax, tune existing oak/spruce material rows, retain all filtering bounds, and present recorded views for final owner acceptance. Timing remains pending a valid uncontended session.
## Acceptance
Every R-ID in the parent spec's Acceptance Criteria is satisfied; judge this task against the spec directly.

## Done summary
Implemented calibrated oak/spruce bark captures, corrected parallax, continuous shallow chipped scale profiles with an authored plateEdgeShape blend, and restrained warm recess colours. Other species retain their material settings. Deterministic material, shader, profile, distance, resolution, redraw, browser round-trip and catalogue checks pass at unchanged bounds; performance costs are recorded in REPORT.md.

R1–R5 are satisfied under the recorded owner decisions: qualitative references with unknown photo scale, cost disclosure instead of a ceiling, shallower structure replacing the rejected deeper candidate, and the owner's final appearance/distance acceptance and squash-merge authorization. The historical hero reconstruction remains a disclosed inherited absolute-gate failure; the accepted exception uses the pre-change control and owner live verdict. Directional overlapping flakes are captured separately in fn-90.

All FRICTION.md entries reviewed; REPORT.md and REJECTED-RELIEF-REPORT.md preserve findings and remedies. Evidence recipe/provenance improvements are proposed for owner consideration only; local environment issues are not new specs.

stage: work - ran (implementation, measured validation, owner visual acceptance)
stage: impl-review - skipped(config: review.backend=none; host inspected diff)
stage: completion-review - skipped(config: review.backend=none)
Tracker sync: n/a (bridge inactive)
## Evidence
- Commits: 361dedc1c93781edc29f512fb228a559b7d22b2f
- Tests: cargo test --profile ci -p telperion-core --test material_detail, cargo test --profile ci -p telperion-render --test bark_plates --test bark_relief_range --test material_shaders -- --test-threads=1, cargo test --profile ci -p telperion-render --test material_shaders --test bark_structure_colour --test bark_resolution --test bark_distance -- --test-threads=1, cargo clippy --profile ci -p telperion-render --all-targets -- -D warnings, cargo fmt --all -- --check, npx tsc --noEmit, npx vitest run harness/material-detail.test.ts, node scripts/build-wasm.mjs, node scripts/build-render.mjs
- PRs: