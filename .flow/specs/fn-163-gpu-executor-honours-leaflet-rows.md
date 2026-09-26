## Goal & Context
<!-- scope: business -->

GPU executor honours leaflet rows.

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** The read map suggests the GPU executor ignores every leaflet row, so a twig-borne compound leaf is drawn as plain blades: GPU configuration packs canopy values without leaflet parameters (`telperion-render/src/generation/data.rs:35`). Not yet confirmed by a test. [unknown]
- **Shape.** First a repro test compares the GPU and CPU executors on a family with twig-borne compound leaves. Only if it fails does the fix carry leaflet rows through the executor interface (fn-158). [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A repro test is run first. If the GPU and CPU executors agree, the spec closes as not a defect; otherwise the GPU executor draws the same leaflets as the CPU reference within its stated tolerance. [paraphrase]
- **R2:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Backlog, from fn-152's read map and Astra's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/`). Not part of the fn-152 to fn-160 stack.
