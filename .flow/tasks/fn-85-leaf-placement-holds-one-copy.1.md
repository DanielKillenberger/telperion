---
satisfies: [R1, R2, R3]
---
# fn-85-leaf-placement-holds-one-copy.1 Implement Leaf placement holds one copy

## Description
Make `foliage::cull` retain in the buffer it is handed.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
`cull` takes `Instances` by value and `retain`s in place; the second allocation is gone; errors are flagged and raised after the pass. A pointer/capacity test was red on the old body. Peak RSS: spruces 4,994 -> 3,244 MB, beeches 4,786 -> 3,945, oaks 1,672 -> 1,469, birches 820 -> 778. Implemented by the session model in-host.

## Evidence
- Commits: c9aed1c0, 66e5229d
- Tests: cargo test --profile ci -p telperion-core --test foliage, cargo test --release --workspace --no-fail-fast -- --test-threads=1, cargo clippy --profile ci --workspace --all-targets
- PRs:
