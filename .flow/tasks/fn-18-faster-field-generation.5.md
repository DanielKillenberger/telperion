---
satisfies: [R1, R2, R3, R4]
---
# fn-18-faster-field-generation.5 Assess bounded native parallelism and separate edit-time reuse

## Description
Evaluate the remaining parallel opportunity within current platform/API contracts and assess reuse without creating a new editing framework.

**Size:** M
**Files:** crates/telperion-core/src/branching/local.rs, crates/telperion-core/src/field.rs, new crates/telperion-core/src/parallel.rs if earned, crates/telperion-core/tests/growth.rs, experiments/fn18-generation/.
**Touches:** [crates/telperion-core/src/branching/local.rs, crates/telperion-core/src/field.rs, crates/telperion-core/src/parallel.rs, crates/telperion-core/src/lib.rs, crates/telperion-core/Cargo.toml, Cargo.lock, crates/telperion-core/tests/growth.rs, crates/telperion-core/tests/field.rs, experiments/fn18-generation/**, .flow/evidence/fn18/**]

### Approach
- Use task 1's remaining serial fraction to choose one bounded native candidate: immutable local-shoot plans with ordered commits or independent exact bounds work. Reuse standard scoped-thread facilities where sufficient; avoid an executor framework.
- Preserve original RNG and reduction order. Workers prepare indexed local results; the existing serial frontier/parent append enforces caps/IDs. Never reorder f64 sums or share mutable Tree appends.
- Measure 1/2/4/bounded-host worker counts including startup/join, cold latency, retained/temporary memory and distinct-tree throughput separately. Missing support selects the serial path before dispatch. Spawn/worker failure must join/clean up and return a whole-build error; no partial field.
- Keep wasm32/browser on the exact sequential path. Document that messaging is asynchronous, main-thread blocking waits are unavailable, and shared-memory deployment changes are outside this API-preserving pass. Native gains cannot satisfy the browser 3× target.
- Assess existing query-only reuse and at most one representative parameter edit in a disposable experiment. Compare exact full rebuild, invalidation scope and saved work; classify possible edit gains separately. Without an existing editable ownership API, report the opportunity/limits and ship no cache or new editing interface.
- Integrate only qualified native changes; retain failed experiments and remaining 3×/10× limits.

### Investigation targets
**Required:**
- crates/telperion-core/src/branching/local.rs:259 — keyed shoot plans and serial commits.
- crates/telperion-core/src/colonization.rs:213 — ordered reductions.
- crates/telperion-core/src/field.rs — independent field inputs.
- crates/telperion-core/Cargo.toml — current lean dependencies.
- crates/telperion-wasm/src/lib.rs:50 — thread-local Engine.
- src/browser/core.ts — synchronous public API and revision ownership.
- scripts/benchmarks/field-generation.md — frozen cold/steady boundaries.

### Acceptance
- [ ] Native worker-count experiment reports exact repeat/source/flags and full startup/join/memory costs; serial browser behavior remains validated.
- [ ] Controlled worker/spawn failure, no-parallel support, tight caps and deterministic ordered commits have explicit outcomes and cleanup.
- [ ] Native adoption/no-change decision is evidence-backed, with browser and throughput targets clearly separate.
- [ ] Edit-reuse assessment names invalidation/full-rebuild equivalence or concrete blockers; no unsupported reuse is claimed as cold-generation gain.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
