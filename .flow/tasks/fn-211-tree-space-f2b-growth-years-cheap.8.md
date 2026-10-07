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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
