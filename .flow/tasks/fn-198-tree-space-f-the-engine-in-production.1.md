---
satisfies: [R1]
---
# fn-198-tree-space-f-the-engine-in-production.1 Species as preset values, and the integration survey

## Description
Overnight scope step 1 (spec, 'Overnight scope'). First a survey, written as INTEGRATION.md in .flow/evidence/fn-198-tree-space-f-the-engine-in-production/, each claim checked against code (file:line): how today's presets reach stage 2 (family JSON, Generator::prepare, preparation.rs:14 per GPU-PATH.md), the preset schema and its generation (presets.generated.ts), how the engine's species are defined today (crates/telperion-space/src/{beech,spruce,oak,palm}.rs, Species, the examples' ROWS overlays), the four missing pieces GPU-PATH.md names (prepare takes no grown tree; executor::expand test-only door; tree::convert only in example code with telperion-space a dev-dependency; setTreeGpu and the Wasm binding), and what crosses the Wasm boundary. Then propose, for the host to decide before building: the value table format for an engine species inside a shipped preset, how stage 2 reads it, and where conversion lives. Stop after the survey and proposal; the host decides.

## Acceptance
- INTEGRATION.md with every claim checked against code, file:line. - A proposal with options for the value format, the stage 2 door and conversion's home. - No product code changed.

## Done summary
Survey and proposal in INTEGRATION.md (7b87437b); host decisions 1-9 appended (b7b2acbd).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7b87437b, b7b2acbd
- Tests:
- PRs: