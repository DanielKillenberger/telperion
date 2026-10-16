---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.8 Contract A revised, and its conformance probe

## Description
Astra accepted CONTRACT-A.md with changes (ASTRA-REVIEW-CONTRACT-A.md, binding). (1) Revise CONTRACT-A.md: wider or exact intermediates for multi-factor products and W/W division with their rounding; sums with a bound proven before accumulation (for signed moments a checked bound on the sum of absolute magnitudes), refusal independent of cut and order, numerical overflow separated from probability clamps; angle semantics (wrapped orientations, signed amounts, half-angle and interpolation rules) and a precise radian constant with a scaled product; normalization with guard precision; the missing zero and endpoint cases (ripening <= 0, scalar asin, half-angle tangents, zero light with viability one, phi zero); CORDIC and log2/exp2 specified to the bit with error bounds; small mechanics kept in W; one consistent position domain; deposits conserving area by residual distribution or the loss stated; the device sweep appendix marked non-normative; the re-lay inside the contract. (2) The smallest proof: a CPU/GPU arithmetic conformance probe with exact integer expected results, Astra's counterexamples included, comparing every word and error status against a small WGSL mirror; then the feedback slice (canonical leaf position and area, integer deposits, the shared CPU sweep, integer sampling, D16's threshold and kept share) with deposit order, chunking and a forced buffer retry varied.

## Acceptance
- CONTRACT-A.md revised point by point against the review.
- The conformance probe green on every case, word for word, Astra's counterexamples included.
- The feedback slice identical across order, chunking and retry.
- No change to today's trees yet; the gate green (split: non-GPU without the lock, telperion-render under it).

## Done summary
CONTRACT-A.md's Part 2 revised point by point against Astra's review (R1.1-R1.6, R2.1-R2.7, R3, R4 cited): 256-bit exact intermediates rounded once and narrowed with a check; order-free sums (a bound proven before accumulation, or a saturating magnitude sum) whose refusal depends only on the terms; orientations (T) and amounts (A) with between/toward/half rules; 60/64-bit radian constants; normalisation with guard precision, levers normalised from W; CORDIC and W log2/exp2 specified to the bit with measured error bounds; the zero and endpoint cases; one position domain (2^12 m); deposits conserving area by residual; sweep guarantee stated precisely; the re-lay inside A; D16's threshold, equality and window. Built: telperion_space::fixed::feedback (CPU) and fine/feedback*.wgsl (GPU mirror). The conformance probe: 14 selectors, 229,400 cases with Astra's counterexamples first, every word and status equal. The feedback slice: 20,000 leaves and 3,000 buds, lattice and reads equal across three orders, three chunkings and a forced retry. No tree changed. Open for the host: one layout implementation for the fixed-point rough layout and the f64 final lay (CONTRACT-A.md 2.14).

## Evidence
- Commits: 83b34510, e9698801, 70988faa, a664b5e0
- Tests: cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-render --no-fail-fast (under gpu.lock)
- PRs: