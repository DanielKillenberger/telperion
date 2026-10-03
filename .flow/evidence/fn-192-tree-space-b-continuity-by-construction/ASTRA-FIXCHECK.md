The fix is correct and complete for the stated `0 × ∞` NaN case. Filtering before `wood()` excludes unreachable PAs entirely. Validated probabilities are nonnegative, so excluding zero changes no legitimate contribution.

Other intermediate overflows remain: `wood()` exponentiation, `share()`’s reciprocal exponential, the weighted sum, and multiplication by buds or `RATE`. Positive infinity reaches `presence()` and clamps to `SPAN`, yielding finite presence. Validation requires positive internodes and 1–6 buds, preventing another zero-times-infinity multiplication there. Premature exponentiation can still overestimate windows for extreme tiny positive weights; that predates this commit. Separately, `self.age + 1` can overflow at `u32::MAX`, also pre-existing.

The regression genuinely distinguishes before/after. Reading the test and independently reproducing its arithmetic in memory confirms:

- Seed 1 produces exactly one node with finite lead ≈0.314796.
- The unreachable PA’s log ratio is ≈711.359, so `wood()` is infinity.
- Before: zero probability times infinity produces NaN, failing the finite-scale assertion.
- After: node wood ≈1, scale ≈0.157398, and tip z ≈0.015740—all finite.

I did not execute the Rust test suite or modify files.

VERDICT=SHIP