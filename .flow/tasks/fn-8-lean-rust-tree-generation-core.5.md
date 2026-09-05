---
satisfies: [R1, R3, R4]
---

# fn-8-lean-rust-tree-generation-core.5 Port foliage and expose independent representation requests

## Description
Port foliage placement/culling and establish representation selection against task-1 solved-tree fixtures.

**Size:** M
**Files:** Rust foliage modules, representation API, Wasm output bindings and tests
**Touches:** [crates/telperion-core/src/foliage/**, crates/telperion-core/src/foliage.rs, crates/telperion-core/src/output*, crates/telperion-core/tests/foliage*, crates/telperion-core/tests/foliage_reference*, crates/telperion-core/tests/outputs*]

## Approach
- Port twig-based placement, phyllotaxis, leaf-element geometry, shell culling, transforms and bounds as plain data, retaining the existing silhouette verification as test support, with a reusable element separate from instances.
- Expose foliage through the native domain module only; task 6 owns complete Wasm binding assembly. Publish its input/output contract in shared notes for the field and browser tasks.
- Demonstrate a structural-only request avoids surface and foliage allocations, and foliage selection never constructs wood geometry, including hidden diagnostics work.
- Use frozen solved-tree inputs until tasks 3/4 join; extend the shared equivalence runner to placement, retained membership, transforms and bounds.

## Investigation targets
**Required:**
- `src/canopy/place.ts:171`
- `src/canopy/cull.ts:136`
- `src/canopy/element.ts`
- `src/canopy/shoots.ts`
- `src/canopy/place.test.ts`
- `experiments/rust-surface-benchmark/shared.ts:13`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.


## Acceptance
- [ ] Foliage equivalence passes at ordinary and giant scales, including empty/fully culled results.
- [ ] Output selection tests prove omitted representations do no construction work.
- [ ] Malformed native requests and overflow cases pass focused tests.
- [ ] Native outputs follow the foundational ownership and numerical contracts, ready for the browser binding task.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
