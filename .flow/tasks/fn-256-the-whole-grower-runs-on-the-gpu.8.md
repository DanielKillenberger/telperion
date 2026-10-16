# fn-256-the-whole-grower-runs-on-the-gpu.8 Two suspected races, reproduced and fixed

## Description
COUNCIL round 5 step 8: fault_out (layout.wgsl:182) sets a fault bit with plain read-write; node_laterals (grow.wgsl:504-520) writes in the counted pass. Focused tests red first (e.g. a two-fault dispatch), then fix (atomic OR; counted branch counts only).

## Acceptance
Each confirmed race has a red-then-green test; an unconfirmed one is recorded with the test that failed to reproduce it.

## Done summary
Both suspected races reproduced red and fixed green, each through the real kernel's function by a probe entry (`race_tests.rs`): `fault_out` now ORs into the layout's tally with `atomicOr` (`lists` is `array<atomic<u32>>`), and `node_laterals` only counts where `what` is 0 (no index, no window read, no axis written). Red: the tally kept one bit in 16 of 16 dispatches; 2 axis words written. Green: every bit; none written. Node tables byte-identical on every preset (PROOF.md).

## Evidence
- Commits: 564be9296, 8f2fe513c
- Tests: cargo test --profile ci -p telperion-render --lib race_tests (red at 564be9296, green at 8f2fe513c), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (1066 passed), cargo test --profile ci -p telperion-render --no-fail-fast, GPU lock (272 passed, 37 ignored), npm test (162 passed)
- PRs:
