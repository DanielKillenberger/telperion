## Goal & Context
<!-- scope: business -->

Deprecated parameter rows leave the package.

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** `canopy.spacing`, `clump` and `clumpSpan` are never read in production. fn-152 marks them deprecated, but they remain exported TypeScript properties, and `parse` and `overlay` reject unknown keys (`params.rs:297`, `:337`). [checked]
- **Shape.** Remove them from the parser, overlay, metadata, types and public entry points as one package compatibility change, released as a minor version, with a note to consumers. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The rows are gone from every surface. A family carrying them gets a clear deprecation error naming the row, and the change is released with a version bump and a note. [paraphrase]
- **R2:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Backlog, from fn-152's read map and Astra's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/`). Not part of the fn-152 to fn-160 stack.
