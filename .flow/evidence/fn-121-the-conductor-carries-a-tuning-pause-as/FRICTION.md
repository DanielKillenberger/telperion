# fn-121 friction

## 2026-09-23 - the gate's cold build in a fresh worktree

- Doing: the single end-of-task gate, `cargo test --profile ci --workspace --no-fail-fast`, in the fn-121 worktree.
- Slowed: the worktree had no `target/` of its own (correctly, per the no-shared-target rule), so the gate compiled the whole workspace cold; 521 s wall for about 1 s of new tests.
- Cost: about 9 minutes of waiting.
- Would remove it: a per-worktree warm cache that is not a shared `target/` (for example `sccache` keyed on the source, which does not reuse another checkout's test binaries by name), or seeding the worktree's `target/` by copy from the base checkout at creation.
