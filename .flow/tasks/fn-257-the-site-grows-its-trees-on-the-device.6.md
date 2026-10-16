---
satisfies: [R1, R1b, R2, R3a]
---
# fn-257-the-site-grows-its-trees-on-the-device.6 The field stays on the device

## Description
TBD

## Acceptance
- `growField` grows on the device and keeps the field's primitives there; `query(cells)` runs on the device and reads back only answers; `release()` frees them.
- Answers within measured tolerances of the CPU reference at 24, 36 and 48 cells; a named error without WebGPU; `cut` removed with a migration note; dead stand, aggregate and predictor paths deleted; Node tests per R3a.

## Done summary
telperion/field grows on the page's or worker's WebGPU device and keeps the field there (primitives: each drawn node's wood, each station segment's leaves, each frond leaflet's box; limb systems by pointer jumps, named densely; a grid of buckets); queries run on the device, only answers read back; crowded cells resolved a window of systems at a time; named errors without WebGPU and on a lost device; cut removed with a migration note; the slim crate and the core's coarse stands deleted (telperion-space's grow_coarse, stepped and couple remain, read by their own tests and examples). R2: every preset within tolerance at 24/36/48 cells (rosette palm held to its own draws). Codex review NEEDS_WORK (5) -> fixed -> SHIP.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7aa5e7620, b7d7b346e, 36f16a333, a95343285
- Tests: cargo test --profile ci -p telperion-render --lib field_tests, cargo test --profile ci -p telperion-core --lib field, npx vitest run src/field
- PRs: