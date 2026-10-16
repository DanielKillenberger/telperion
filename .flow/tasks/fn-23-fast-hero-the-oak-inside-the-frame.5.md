---
satisfies: [R2]
---
# fn-23-fast-hero-the-oak-inside-the-frame.5 The browser at 60 frames per second: the orbit session on the page

## Description
The browser at 60 frames per second: the orbit session on the page.

## Acceptance
- Ten-second orbit with wall p95 under 16.7 ms and no frame over 33 ms.

## Done summary
R2 met: oak wall p50 10.00 ms, p95 10.10 ms, worst 10.10 ms over 999 frames (100 Hz display); GPU 2.315 ms. Spruce recorded at wall p50 30.00 ms.

## Evidence
- Commits: 1dc7829b229f7c7099d6916b66fcd601937a7e5e
- Tests: cargo test --release --workspace; RENDER_EVIDENCE=.flow/evidence/fn23 npm run test:render
