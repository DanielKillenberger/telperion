## Goal & Context
<!-- scope: business -->

sheddingThreshold splits into its two meanings.

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** A family's `sheddingThreshold` is a shell depth on the direct build (`branching.rs:300`) and an annual vigour threshold on the growth path (`branching/specimen/survival.rs:13`). One row, two meanings. [checked]
- **Shape.** Split it into two rows, one per meaning. For compatibility, a legacy value is copied into both, and a new key wins over the old one when both are given; overlay and serialisation behaviour are specified. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Both paths read their own row. A legacy family with a nonzero `sheddingThreshold` builds byte-identically on both paths; the precedence of new over old keys is tested. [paraphrase]
- **R2:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Backlog, from fn-152's read map and Astra's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/`). Not part of the fn-152 to fn-160 stack.
