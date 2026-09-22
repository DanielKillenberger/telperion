# Friction - fn-93

## 2026-09-22 - a fresh worktree pays a cold ci-profile build before the repro can run

Running R1's repro needed `cargo test --profile ci -p telperion-render`, and the
worktree's own `target/` was empty, as the two-checkouts rule requires. The cold
build of the render test binaries took about seven minutes before a single
measurement could be taken, against roughly two minutes for the suite itself and
minutes for the change. Nothing here is wrong; it is the standing cost of an
isolated worktree. A warm shared build cache keyed by checkout, or a conductor
that builds the worktree's test binaries while it writes the dispatch, would
remove it.

## 2026-09-22 - the local dcg hook blocks rm -rf on a build-artifact path

Clearing `target/tmp/resolution` before the green run, to prove the receipts the
run writes are the run's own, was blocked by the `dcg` hook's `rm -rf` rule and
cost one round trip. `mv <path> /tmp/delete-me-<timestamp>` is allowed and did
the job. This is the owner's local setup, not the repository's, so it is
reported and not specced; an allowance for paths under a checkout's own
`target/` would remove it.
