---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-102-one-build-pipeline-in-place-of-the.1 Implement One build pipeline in place of the copied chains

## Description
Replace the copied build chains with one stage pipeline in the core that the mesh build and the binding both call.

## Acceptance
Spec R1-R8.

## Done summary
`telperion_core::pipeline` runs the mature build, each stage once and only where a requested output needs it; the binding's recipe copy and the chain-equality test are gone. Native wood and leaves run concurrently with stage-ordered joins and errors; a contact family gets one ring sweep read in place from the wood's vertices. Byte identity holds for every preset, seed and binding combination (palm binding fixed as allowed). Whole builds faster except the spruce (+4.6%, accepted by the owner for ~70 MB less memory).

## Evidence
- Commits: 49f004e26f6bc7589351ddd5d824d8d35372c9f7, 72577e73d6fdfb208b4fb341c3e72e8567343e05, 7cfa4676ae0f5c35727e70fb9509e0639e548fbc, c1b4988904e45d1113a3be136d13276b05db9cda
- Tests: cargo test --profile ci --workspace --no-fail-fast (927 passed); cargo test --profile ci -p telperion-core --lib pipeline; npm test (124 passed); npm run rust:test:wasm
- PRs:
