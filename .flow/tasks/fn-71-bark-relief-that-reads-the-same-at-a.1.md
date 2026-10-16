---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
Fix the bark relief's distance drift in the shared renderer terms, over four rounds judged by the owner in the harness.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Round three (accepted) removed every amplitude fade; round four cut cost under that picture with no fade returning: shared lenticel groove read, dash window limited to rows and arc cells a box touches, grain box capped at three cells, shading cells sized per axis. Hero frames oak 8.91, birch 7.00, beech 14.59 ms p50 (round three 10.0, 15.0, 19.6). Sweep invariant holds: widest adjacent step 0.030. Receipts green and reproduced (oak 1x 1.78, 2x 2.23, 4x 2.84, beech 1.06, birch 2.54, grazing 2.70/2.89); redraw pins hold.

## Evidence
- Commits: d91434117d72ef85905d4b05689326c93b3e3cb6
- Tests: cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm test, npm run typecheck
- PRs:
