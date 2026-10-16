---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-27-coarse-shadow-casters-and-a-filtered.1 Implement Coarse shadow casters and a filtered shadow

## Description
Implement coarse wood and foliage shadow casters and a filtered shadow comparison from four scene-row values (R1-R7).

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Wood casters are a prefix of radius-ordered runs (102769e); foliage casters every `casterStride`-th placement at the coarsest level with the quad scaled by sqrt(stride); one shared filtered read with a normal offset, out-of-map taps lit. Oak native total p50 4.949 -> 3.4243 ms, sun pass 1.813 -> 0.2724 ms; oak orbit 10.00/10.10/10.20 ms. Spruce native 23.4161 ms, browser wall p50 30.00 -> 20.00 ms; needle aggregation stays the follow-on. Implemented via a codex bridge in two passes. R2 and R5 accepted by the owner on 2026-09-12.

## Evidence
- Commits: 418e6d37c7199ab0eb126de52a7fefb4df8abe9a
- Tests: cargo test --release --workspace, cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, npm run typecheck, ...
- PRs:
