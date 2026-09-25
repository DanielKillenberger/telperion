# fn-152-one-parameter-table-the-generator-flows.1 One parameter table the generator flows through

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Blocked:
NEEDS_HUMAN: one pass is not feasible within the task budget, and part of R1 is a host design decision. The read map (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/READ-MAP.md`) traced all 249 wire rows.

R1 cannot be written honestly without the host ruling on:
- three rows no production stage reads (`canopy.spacing`, `clump`, `clumpSpan`);
- two rows that do something other than their doc (`sheddingThreshold` has two meanings; `killDistance` is capped by the growth unit);
- leaf bases shaped without a rosette;
- the growth path disagreeing with the direct build;
- a likely GPU defect that drops every leaflet row;
- a stage vocabulary with no place for growth-path-only or validation-only rows.

R2, R3, R6 and presets as data are then five separate migrations: about 107 stage signatures plus 16 GPU-executor entry points, 64 test and example files behind a test-only feature, six hand copies of the row list plus `blend.rs`, and eight presets. Each needs its own byte-identity proof and timings, which does not fit 10 commits. No refactor was started.

Proposed split, in READ-MAP.md: 0 host decisions, 1 the table and the consumers it generates as data, 2 resolve and stage views, 3 private stages, 4 generated dials and sliders, 5 presets as data.
## Evidence
- Commits:
- Tests:
- PRs:
