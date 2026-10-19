---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-40-smooth-bark-lichen-lenticels-and-peel.1 Implement Smooth bark: lichen, lenticels and peel

## Description
Add fourteen material rows for lichen, lenticels and peel, relief tied to trunk radius, and a second wood pipeline so a material without them draws without them (R1-R5).

## Acceptance
- [x] R1 lichen rows; beech against B-BASE
- [x] R2 lenticel rows; birch and beech
- [x] R3 peel rows; birch against S-BARK
- [x] R4 mesh bytes and pins unchanged; cost oak +0.006 ms, birch +0.41, beech -0.40
- [x] R5 pairs re-rendered; verdict in fn-34

## Done summary
Smooth bark carries lichen, lenticel and peel rows; beech smooth pale grey with lichen, birch chalk white with lenticels. The birch retuned at round 24 against the owner's contrast note (near-black 40% to 6%, photograph 11%). The owner called round 16 "plasticesque" and placed the matte fix in a later spec. The owner accepted the silver birch in fn-34; the beech moved to fn-62.

## Evidence
- Commits: 9437d3fd
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, ...
- PRs:
