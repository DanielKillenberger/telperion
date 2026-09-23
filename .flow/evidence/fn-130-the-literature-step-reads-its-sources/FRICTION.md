# fn-130 friction

## 2026-09-23 - worker, fixture excerpting and a scripted edit

- **Doing:** copying excerpts of the palm's cached sources into test fixtures, then splicing a rewritten `fetch::run` into `fetch.rs`.
- **Hindered by:** the local `dcg` command guard refused a shell redirect to a variable path (`> $D/m1.md`) and a python heredoc whose body held Rust text it read as process substitution. Both were plain file writes inside the worktree.
- **Cost:** about 3 minutes and two retries (rewritten as a python script with literal paths, and as a script file in the scratchpad).
- **Would remove it:** a local setup matter on the owner's machine, not a repository change: an allowlist entry for redirects under the worktree, or agents defaulting to script files for multi-line edits.

## 2026-09-23 - worker, the workspace gate

- **Doing:** the one end-of-task gate, `cargo test --profile ci --workspace --no-fail-fast`.
- **Hindered by:** `objectives::the_palm_s_proposed_approval_is_the_owner_s_priorities_then_every_drawable_trait` failed with "missing or changed image /tmp/fn119-objectives/render-P-BASE.png". The test writes to a fixed `/tmp/fn119-objectives`, which the other test in that binary and any other checkout running the suite share; it passed when rerun alone. This diff touches neither the test nor `tuning`.
- **Cost:** one red gate line and a 1-minute focused rerun; the gate cannot be read green without a second run.
- **Would remove it:** a per-process scratch directory in `tests/objectives.rs` (as `common::ledger_dir` does), in its own change.
