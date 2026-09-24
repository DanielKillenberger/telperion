# Friction reports - fn-141-a-resolved-claim-stays-resolved-while

## 2026-09-24 - full-workspace gate cold-compiles every crate in a fresh worktree

While running the end-of-task gate (`cargo test --profile ci --workspace
--no-fail-fast`) for task fn-141.1, the worktree at
`.worktrees/fn-141-a-resolved-claim-stays-resolved-while` had no `target/`
of its own (the project's own gate rule: "Two checkouts of the same crate
never share a `target/` directory"), so the gate cold-compiled the entire
workspace - including `telperion-wasm`, `telperion-field`, `gpu-allocator`,
and the whole `telperion-jev` dependency tree - rather than reusing any
cached build artifacts. The scoped `cargo build -p telperion-jev` and
`cargo test -p telperion-jev` runs earlier in the task were fast
(single-digit seconds after the first compile), but the mandatory
workspace-wide gate paid a full cold build on top of that, costing several
minutes against the task's 90-minute timebox even though the change
touched exactly one crate (`telperion-jev`).

What would remove it: a pre-warmed `target/` per worktree (e.g. a
`cargo check --workspace` kicked off by the worktree-creation tooling
before the task is dispatched, or a shared sccache/incremental cache keyed
by workspace lockfile rather than by checkout path) so the mandatory
end-of-task gate is not also the first full compile of the branch.
