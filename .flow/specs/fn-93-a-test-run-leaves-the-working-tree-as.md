# A test run leaves the working tree as it found it

## Goal

`crates/telperion-render/tests/common/resolution.rs` `record` rewrote the four tracked resolution receipts in `.flow/evidence/fn71/resolution/` on every render-test run, with GPU-measured margins not stable to the printed precision. Every gate run left a dirty tree; three times on 2026-09-20 (fn-87, fn-92) `git add -A` swept the file into an unrelated commit or a tagged stash was needed to restore it. The receipts are wanted; a routine test run deciding what the committed evidence says is not. [user]

## Decisions

- `record` keeps writing a receipt on every run, to `env!("CARGO_TARGET_TMPDIR")/resolution/<name>.json`; it writes to the tracked evidence directory only when `TELPERION_RECORD_EVIDENCE=1` is set. JSON shape, the post-write assertion and every bound are unchanged; the evidence directory is named in one function, `receipt_path`.
- The four committed receipts are left as committed; refreshing them on the owner's hardware is the owner's call.

## Acceptance Criteria

- **R1:** From a clean tree, `cargo test --profile ci -p telperion-render` leaves `git status --porcelain` empty; red on the base first — met: red on run 1 of 5 (all four receipts moved, not only `smooth_bark`), empty after the fix, also after the full workspace gate.
- **R2:** The run writes all four receipts under the target temp directory, asserted by `record` — met.
- **R3:** With `TELPERION_RECORD_EVIDENCE=1` the receipts go to `.flow/evidence/fn71/resolution/`, covered by a unit test of `receipt_path` — met.
- **R4:** No bound, fixture or assertion changes; the diff is `resolution.rs`, its test and module comment — met.

## Result

Shipped in `3f23637b`. The committed rows are not this machine's measurements (e.g. `bark_distance` OregonWhiteOak 2x mean 1.6262 / p95 4.50 here vs committed 1.9390 / 5.75), so the drift was a machine difference on top of run-to-run wobble. Gate: 877 passed, 0 failed, porcelain empty afterwards.

## Open

Whether to refresh the four fn-71 receipts on the owner's hardware (owner). No search was made for other tests writing into tracked paths.

## Friction

- A fresh worktree paid a ~7 min cold ci build before the repro could run -> standing cost of isolated worktrees; a pre-built cache would remove it.
- Local `dcg` hook blocked `rm -rf` on a `target/` path -> local setup, reported only.
