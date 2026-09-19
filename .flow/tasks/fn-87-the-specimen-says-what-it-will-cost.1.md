---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-87-the-specimen-says-what-it-will-cost.1 Implement The specimen says what it will cost

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

## NEEDS_HUMAN (2026-09-19)

Stopped before implementation. The spec's Architecture section says the grown
tree carries the station and vertex counts, so the prediction can be an
allocation-free O(nodes) pass; `crates/telperion-core/src/tree.rs:76` carries
only `nodes`, `crossover` and `diagnostics`, and the station, run and surface
vertex counts are derived inside the builders on allocated scratch
(`src/surface/paths.rs`, `src/foliage/placement.rs::bearing_runs`,
`src/foliage/station.rs`). The route that would make the premise true - persisting
the counts on `Tree` - is closed by this spec's own Boundaries, because `Tree` is
`bincode::serialize`d into the committed digests
(`crates/telperion-core/tests/species.rs:39`) and this spec changes no stored
byte; the remaining routes (recount with allocation, or approximate from edges)
each pick a policy R1 and R2 do not decide. Separately, R2 measures against
retained allocation while `src/foliage/station.rs::reserve` grows the leaf vector
per run with `try_reserve` and `cull` uses `retain`, so stations times
`size_of::<Leaf>()` is not what a finished specimen holds; and R4 leaves the
reserve's size and the bound the VmHWM assertion checks undefined, which collides
with its own floor of one admitted seed. Three owner decisions unblock this: where
the prediction's counts come from under the no-allocation and no-stored-byte
constraints, how retained capacity is accounted for in R2's 15 percent, and the
reserve plus the bound R4 asserts. Baseline and detail are in
`.flow/evidence/fn-87-the-specimen-says-what-it-will-cost/`.
