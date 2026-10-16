---
satisfies: [R1, R2, R3, R4, R6]
---
# fn-13-tree-detail-by-viewing-distance.1 Pin fidelity references, ground reproduction and measurement protocol

## Description
Pin fidelity references, a ground-visible reproduction and the GPU measurement protocol.

## Acceptance
- Deterministic full-source capture and reference runner; strict GPU timing validity.

## Done summary
Full-source protocol, deterministic ground-visible capture runner and strict GPU timing validity; references converge, negative controls fail as designed, and the unoptimized native whole-stage baseline is about 53.4 ms p95.

## Evidence
- Commits: 590d812732e49f45bcbf35d5fd2a230a042cb51f, 317ed2742d71d730e4a0e25378a3a31b13e065e3, b2e5cc6880439a2ff1a1eafbf360d6b3e68b8271, 1693540f4d4bc2c49109f70440972714b493883b
- Tests: npm run typecheck; npm test
