---
satisfies: [R1, R2, R3, R4]
---
# fn-93-a-test-run-leaves-the-working-tree-as.1 Implement A test run leaves the working tree as it found it

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
A resolution test's receipt now lands in the checkout's own `target/tmp/resolution`
unless `TELPERION_RECORD_EVIDENCE=1` asks for the tracked evidence, so a test run
no longer decides what `.flow/evidence/fn71/resolution` says.

The cause the owner asked about: `crates/telperion-render/tests/common/resolution.rs`
`record` wrote a GPU-measured receipt straight into the tracked evidence directory on
every render-test run, which is what keeps touching the four fn-71 artifacts.
`receipt_path` now takes its destination, `record` reads the variable, and a unit test
covers both settings without a device. No bound, fixture or assertion moved; the diff
is that one file plus its module comment.

R1 was red on the base on the first of the five allowed runs, and it was worse than the
spec assumed: all four receipts moved, not only `smooth_bark`. The committed rows are not
this machine's measurements at all - `bark_distance` OregonWhiteOak 2x read mean 1.6262,
p95 4.50 here against the committed 1.9390 / 5.75, `bark_resolution_trunk` 1.5415 / 5.25
against 1.8089 / 6.25 - so the drift is a machine difference on top of the run-to-run
wobble, and every run of the suite was rewriting four tracked files. The four receipts are
left exactly as committed, per the spec's boundary; whether they should be refreshed on the
owner's hardware is the owner's call, not this task's.

After the fix a full `cargo test --profile ci --workspace --no-fail-fast` left
`git status --porcelain` empty - R1 proved on the whole gate, not just the render crate.

Follow-up, not built here: nothing else in the workspace was searched for tests that write
into tracked paths (spec boundary); the two friction entries are in
`.flow/evidence/fn-93-a-test-run-leaves-the-working-tree-as/FRICTION.md`.

Tier: session (explicit IMPLEMENTER preserved; actual model not exposed by the host)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 3f23637b8f97b0ed379130ab9214a3d110ce4a77
- Tests: cargo test --profile ci -p telperion-render (red on base: run 1 of 5 left all four fn71 receipts modified), cargo test --profile ci -p telperion-render (after the fix: 158 passed, 0 failed, git status --porcelain empty, four receipts under target/tmp/resolution), cargo test --profile ci -p telperion-render --test smooth_bark only_an_asked_for_run (mutation check: fails when the plain branch names the evidence dir), cargo test --profile ci --workspace --no-fail-fast (gate: 877 passed, 0 failed, 21 ignored, porcelain empty afterwards)
- PRs: