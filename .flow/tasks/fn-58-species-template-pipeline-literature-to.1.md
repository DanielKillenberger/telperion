---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-58-species-template-pipeline-literature-to.1 Implement Species template pipeline: literature to preset

## Description
Build the `species-pipeline` stages, adapter, routes, verification, fit, swap tooling and data-quality gate per the spec.

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
A driver takes a taxon from admitted sources to a packet, provenance sidecar, fitted curves, decision list and report by running eleven `species-pipeline` commands from `docs/species-pipeline.md`, passing only paths; resolutions in `resolutions.json` bind to decision input checksums. R1 to R5 and R8 landed; the fit reproduces fn30 exactly; parked checks recorded in `.flow/evidence/fn58/parked-checks.md`. Not met: the live model-swap test (codex quota until 2026-09-19); held-out accuracies not run live; screen/select/verify/gate/generate not run live on oak or spruce. Implemented on the session model.

## Evidence
- Commits: 05023ebf3a013b394c074a7689f32594e546374e
- Tests: cargo test --release -p telperion-core (332), cargo test --release -p telperion-render -p telperion-wasm -p telperion-jev (253), npm test (85), npm run typecheck, cargo clippy -p telperion-jev --all-targets -- -D warnings
- PRs:
