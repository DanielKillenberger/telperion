---
satisfies: [R1, R2, R3, R4]
---
# fn-93-a-test-run-leaves-the-working-tree-as.1 Implement A test run leaves the working tree as it found it

## Description
Route resolution-test receipts to the target temp directory unless `TELPERION_RECORD_EVIDENCE=1` asks for the tracked evidence.

## Acceptance
Every R-ID in the parent spec's acceptance holds.

## Done summary
`receipt_path` takes its destination and `record` reads the variable; a device-free unit test covers both settings. Red on the base on run 1 of 5 (all four receipts moved); after the fix the render crate (158 passed) and the full gate (877 passed) leave `git status --porcelain` empty. The four receipts stay as committed.

## Evidence
- Commits: 3f23637b8f97b0ed379130ab9214a3d110ce4a77
- Tests: cargo test --profile ci -p telperion-render; cargo test --profile ci --workspace --no-fail-fast (877 passed, porcelain empty)
- PRs:
