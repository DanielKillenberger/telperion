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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
