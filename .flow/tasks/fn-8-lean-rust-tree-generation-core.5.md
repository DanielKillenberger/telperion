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
Implemented native leaf anatomy, twig phyllotaxis and terminal-shoot fallback, reusable elements, float32 instance transforms, conservative shell culling and exact output bounds. Native requests reject malformed inputs, invalid transforms, overflow and exhausted caller budgets; place constructs neither leaf geometry nor wood geometry.

baseline: none (approved parent/task contain no Quick commands).

Six focused tests cover leaf winding/anatomy, fixed twig stations and owned deterministic similarity frames, fallback zero-edge folding and tip clumps, finite/invalid/empty/fully culled state, budget errors, leaf-extent and underside silhouette protection, and an overflow error hidden behind an early cull keep. The initial suite failed for the absent API; the overflow test separately failed before its fix. Final tests, rustfmt and strict clippy pass.

Frozen solved-tree comparison used final FN6 fdafb099b1495519de75a6b9a66d37f7d07e47bd buffers exported by the surface worker at /tmp/fn8-surface-reference. Ordinary, Telperion, Laurelin, empty and capped retained 59,810 / 1,349,630 / 798,553 / 0 / 0 leaves respectively, exactly matching retained membership/order and bounds. Maximum transform errors were 0 / 5.96e-8 / 4.55e-13 / 0 / 0, consistent with rotation arithmetic rounding. Raw output and command receipts are in the evidence paths. No browser image comparison was run in this isolated native task; full generated-tree visuals follow browser integration.

The native stage has no surface-module dependency; its borrowed Tree input remains unchanged, and placement produces transforms independently of reusable element construction. Cross-representation selection and Wasm assembly tests remain with task 6 as dispatched. The field/browser API contract is /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/foliage.md. Default total instance budget is usize::MAX, constrained by checked byte arithmetic and fallible allocation; callers may provide a tighter budget. A request exceeding the inherited 512 stations-per-shoot safeguard returns ResourceLimit instead of silently thinning/truncating foliage.


Conductor integrated 8a32e7f and verified six focused foliage tests on the joined tree.

stage: impl-review - skipped(user: none)
stage: wave-join - ran(merge and focused integrated verification)
stage: plan-sync - skipped(config: false)
## Evidence
- Commits: 8a32e7fe69f3a2abb9c5f655159af09b9a52fecd
- Tests: baseline: none (parent/task define no Quick commands), cargo test -p telperion-core --test foliage: 6 passed; /tmp/fn8-foliage-test-final.log, REFERENCE_DIRECTORY=/tmp/fn8-surface-reference REFERENCE_CASES=ordinary,telperion,laurelin,empty,capped cargo test --release -p telperion-core --test foliage_reference -- --ignored --nocapture: 1 test / 5 cases passed; /tmp/fn8-foliage-reference-final.log, cargo clippy -p telperion-core --all-targets -- -D warnings: passed; /tmp/fn8-foliage-clippy.log, cargo fmt --all -- --check: passed; /tmp/fn8-foliage-fmt.log, git diff --cached --check: passed before commit, Initial focused suite red: absent native foliage API; /tmp/fn8-foliage-red.log, Overflow early-cull regression red before fix; /tmp/fn8-foliage-overflow-red.log, flowctl gate classify --base 09484a8bbc30a7397c093ae7f095c5571c87a404: FULL (.rs executable changes); no spec Quick gate commands defined, Conductor integrated cargo test -p telperion-core --test foliage: six passed
- PRs:
