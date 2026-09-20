# Render crate wall time, fn-92

`cargo test --profile ci -p telperion-render`, owner's desk, 2026-09-20, tests
already built, wall time of the whole command.

| Code | Threading | Wall | Result |
|---|---|---|---|
| base `5a6b9aca` | `-- --test-threads=1` | 153 s | green, 38 suites |
| `3b9eba06` | default | 96, 102, 96, 98, 95 s | green 5 of 5, 39 suites each |

The base has no green run under default threading to compare with, so the
single-threaded run is the only baseline. The after runs carry one suite the
base does not, `concurrent_request`, about 10 s of the total.

`cargo test --profile ci --workspace --no-fail-fast` at `3b9eba06`: exit 0,
375 s, 101 suites ok, no target failed.
