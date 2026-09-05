---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-7-rust-surface-benchmark-prototype.1 Implement and measure the isolated Rust surface benchmark

## Description
Implement the isolated experiment described by the parent spec and collect actual native and Chromium measurements. Freeze committed source 019234b554a1b387e23b05dd3c7e2626f428f907 so later FN-6 edits cannot change the reference. Read production surface/frames/paths/radius types, but do not modify them. Optimize the TypeScript allocation path for a fair baseline; reuse one Rust implementation across native and Wasm. Include a report that separates evidence from unmeasured memory/GPU claims.

**Touches:** [experiments/rust-surface-benchmark/**]

**Investigation targets:** src/mesh/surface.ts, src/mesh/frames.ts, src/mesh/paths.ts, src/radius.ts, src/presets/two-trees.ts; existing surface tests; STRATEGY.md.

**Review:** none per CLAUDE.md. Completion review remains conductor-owned.

## Acceptance
- [ ] R1: isolated files and frozen, reproducible inputs with provenance
- [ ] R2: equivalent reference TS, optimized TS, native Rust and browser Wasm
- [ ] R3: meaningful geometry equivalence and determinism tests pass
- [ ] R4: repeated actual measurements with raw samples and named environment
- [ ] R5: memory measurements clearly labeled; unavailable metrics explicit
- [ ] R6: build, test and benchmark commands documented and exercised
- [ ] R7: measured report makes a justified migration recommendation without claiming GPU gains

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
