# Friction - fn-142-select-keeps-the-reference-photographs

## 2026-09-24 - full-workspace gate cold compile

Doing: running the mandatory end-of-task gate, `cargo test --profile ci
--workspace --no-fail-fast`, in this fresh worktree
(`fn-142-select-keeps-the-reference-photographs`, based on `fn-139`).

What slowed it: the actual fix and its focused red/green test
(`select.rs` + `judged_select.rs`) took a few minutes end to end; the gate
itself then spent roughly 10 minutes compiling the whole workspace
(`telperion-core`, `telperion-render`'s `wgpu` stack, `telperion-wasm`,
`telperion-jev`) from a cold `target/ci` before any test ran, dwarfing the
work the task actually needed.

Cost: about 10-12 minutes of wall-clock wait on top of the fix.

What would remove it: nothing new - this is the exact cost CLAUDE.md's
"Gates and checked claims" section already names ("Two checkouts of the same
crate never share a `target/` directory"), inherent to one worktree per
task/spec. No new fix proposed; noted per the standing friction-report rule
in case a future spec looks at shared `target/` caching across worktrees.
