---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.1 Establish paired generation baselines and portable field snapshots

## Description
Establish paired CPU generation baselines and an optional owned, portable field snapshot as the experiment's inputs.

## Acceptance
- Reproducible Ordinary and Telperion CPU stage profiles with raw samples; no render meshes.
- Snapshot query semantics and ownership agree with the indexed CPU field, including empty/outside/contact cells and stale handles.

## Done summary
Optional owned f64 field snapshots preserving the CPU wood/foliage BVHs, revision-checked, with native staging released immediately. Paired mesh-free browser baseline saved in `scripts/benchmarks/generation-baseline.json`; schema and skeleton dependency findings in `scripts/benchmarks/generation.md`. Existing consumers keep zero snapshot copies.

## Evidence
- Commits: 539e656b7b1b22a2c0bc83cfd0065806f1903760
- Tests: cargo test --release -p telperion-core --test field; cargo test --release -p telperion-wasm; npm run typecheck; BINDINGS_ONLY=1 npm run test:browser
