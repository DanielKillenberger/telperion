## Goal & Context
<!-- scope: business -->

One family per preset identity.

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** `presets::by_identity` fills `maxTurnPerStep` with 35 while `Preset::parameters()` leaves it `None` (`presets.rs:50`). It is not yet shown whether the generated trees differ. [checked]
- **Shape.** Measure first. If the trees differ, one source of truth decides a preset's family, and both paths return it. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A test builds every preset through both paths. If they differ, both return one family afterwards, and the changed outputs carry evidence under the generator-evolution rule. [paraphrase]
- **R2:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Backlog, from fn-152's read map and Astra's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/`). Not part of the fn-152 to fn-160 stack.
