# fn-256-the-whole-grower-runs-on-the-gpu.16 Experiment: log-depth hierarchy sums on a captured cycle

## Description
COUNCIL.md round 7 experiment, measurement only (test-only code). On the captured busy Telperion cycle and the shedding oak at 80 (seed 1), tiny branching and chain cases first: compute the upward totals by (a) today's level sweep, (b) heavy-path decomposition, (c) rake-and-compress, (d) a preorder interval range-sum tree. Each recovers every axis's subtree result; fixed arithmetic graph from topology alone; direct dispatches; no atomics; 8 storage buffers a stage. Charge schedule construction. Report per candidate: dispatches, GPU time, scratch, the measured light depth (b), per-axis error against an f64 reference and against today's device, hazard margins and draw-versus-threshold changes, non-finites. Also prototype the shedding-walk segment composition and six-round parent doubling for the downward passes on the same capture and report their time and agreement.

## Acceptance
SUMS.md with the per-candidate table and the stop verdict: under 2x net beyond run spread or any agreement failure stops the line.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
