# The gate runs in ten minutes and stays there

## Goal & Context

Owner (2026-10-21), on hearing the gate takes 35 minutes: "test suite takes 35 min?!" and "let's do it now.. how long have we been carrying tis drag..?"

The gate has crept back up every time it was cut:

| Date | Spec | Gate |
|---|---|---|
| 2026-09-18 | fn-76 | target under three minutes |
| 2026-10-13 | fn-271 | 1,525 s cut to 1,128 s; nextest locally ran out of memory |
| 2026-10-14 | fn-274 | 1,551 s at 9 GiB, 4 threads |
| 2026-10-16 | fn-287, AGENTS.md | 2,069 s, at `telperion-heavy 12 --threads 2` |
| 2026-10-21 | F4 at bb20f3877 | about 4.9 min compiling plus 2,494 s of tests, about 46 min (`telperion-heavy 8 --threads 2`) |

Every task on the release runs it once, and this build has spent hours waiting on it. Nothing stops it growing back: no test has a time budget, and no new sweep is weighed against its neighbours.

The slowest binaries on F4 (bb20f3877):

| Binary | Time |
|---|--:|
| a lib unit-test binary, likely `telperion-core` | 530 s |
| `walk_leaves.rs` | 346 s |
| `telperion-render` lib | 282 s |
| `walks.rs` | 276 s |
| a lib, likely `telperion-space` | 185 s |
| `ground_reach.rs` (new in fn-301, 168 builds) | 165 s |
| `boundary.rs` | 88 s |
| `walk_sizes.rs` | 75 s |
| `light_rows.rs` | 52 s |
| `conformance.rs` | 52 s |

## Requirements

- **R1 (measure):** per-binary and per-test times on F4. Name what grew since fn-271's 1,128 s, test by test, and the spec that added it.
- **R2 (question, delete, then optimise):**
  - Each slow test either stays in the gate at the smallest size that still catches the defect it names, or its full sweep becomes an ignored test with an ordinary subset in the gate.
  - The subset must be shown red on a temporary broken-generator patch that the full sweep catches (fn-271's method).
  - No assertion is deleted. A sweep that only re-proves what a neighbour proves is named for the host to delete.
  - fn-271's decisions are revisited, not assumed. The full-size walk premise and the separate per-entry builds stand unless the measurement shows a cheaper test that catches the same defect.
- **R3 (run wider):**
  - Measure the gate under nextest with test groups, using fn-274's per-test memory figures for the whole-preset tests, at the widths the desk holds beside the owner's desktop (about 8 GB).
  - The runner that gives the shortest wall time inside 8 GB becomes the declared gate. AGENTS.md's gate line is re-measured and rewritten. This also settles the open 8 GB against 12 GB question.
- **R4 (stays there):** a check that fails when one test's own time passes a stated budget, or when the gate's total passes ten minutes on the declared invocation, so growth is caught by the change that causes it. Name the mechanism: a nextest slow-timeout with terminate-after, a CI step, or a checked-in report. The budget is set by the host from R1's figures.
- **R5:** the target is the whole gate in under 10 minutes on the declared invocation. Test counts change only by named moves to ignored. The gate is green, `npm test` is green, and each task gets a codex review.

## Boundaries

- Test-only. No generator, renderer or preset change.
- CI's own runner is touched only if R4 needs it.
