# Friction — fn-140-generate-writes-three-holdout-seeds

## 2026-09-24 — `cargo nextest` is not on PATH, though CLAUDE.md says the CI profile runs under it

CLAUDE.md's "Gates and checked claims" section says "the profile CI runs under
nextest." Before running the gate I checked for `cargo-nextest` (`which
cargo-nextest`, `cargo nextest --version`) to decide whether to invoke `cargo
nextest run --profile ci` or plain `cargo test --profile ci`. Neither found
it; `cargo nextest` errors with "no such command: `nextest`". Cost: under a
minute, no rerun needed, because the CLAUDE.md instruction line I was given
directly ("Gate `cargo test --profile ci --workspace --no-fail-fast`") already
named the plain-cargo invocation, so I used that instead of chasing nextest
further. Would have been removed by either installing `cargo-nextest` in the
dev environment (if the CI runner truly uses it) or dropping the "runs under
nextest" claim from CLAUDE.md if the local gate is meant to run under plain
`cargo test`.
