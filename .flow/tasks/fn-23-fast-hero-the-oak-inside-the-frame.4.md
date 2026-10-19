---
satisfies: [R1]
---
# fn-23-fast-hero-the-oak-inside-the-frame.4 The selection pass timed, the orbit session, and the native gate

## Description
Time the selection pass, add the orbit session, and run the native gate.

## Acceptance
- Selection has its own timestamp pair; R1 measured with a valid verdict.

## Done summary
R1 met: oak 1.750 ms p50 (selection 0.057 ms, vegetation 1.693 ms); orbit 1.912 ms with wall p50 2.17 ms; spruce 32.619 ms recorded. All sessions valid on first run.

## Evidence
- Commits: 1e42b0969fb8344b067b47a04c76cfb410af095f, c9918333d03f782ca354b4323506de9b0abc3dbd
- Tests: cargo test --release --workspace
