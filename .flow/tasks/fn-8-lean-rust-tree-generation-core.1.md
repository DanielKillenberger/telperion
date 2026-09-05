---
satisfies: [R1, R2, R4, R7]
---
# fn-8-lean-rust-tree-generation-core.1 Pin the final reference and establish the lean native/Wasm foundation

## Description
Establish the numerical and ownership contracts before the botanical port. Implements the early proof point and R1 foundation.

**Size:** M
**Files:** new Cargo workspace, core math/types modules, Wasm binding scaffold, migration fixture runner, package scripts
**Touches:** [Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/**, tests/migration/**, scripts/**, package.json, package-lock.json]

### Approach
- Use one renderer-independent core crate and a thin Wasm binding crate; expose the small native contract in the parent spec. Pin a reproducible toolchain and build commands.
- Port deterministic RNG, vector operations, envelope and shared field primitives; test their boundary semantics before growth. Declare compact tree/diagnostic/output types using domain names and owned storage.
- Build a reference exporter runnable against the pinned final-FN6 Git revision in a disposable checkout, without importing the old production generator into the new production package. Capture stage fixtures, raw parameter manifests and matched visual camera definitions. Keep giant fixtures reproducible without requiring enormous committed buffers.
- Establish exact discrete comparisons and field-specific numeric tolerances before ported results are inspected. Include ordinary, Telperion, Laurelin, empty, degenerate, envelope-crossing and capped cases.
- Prove a small deterministic foundational result runs natively and through browser Wasm, including owned transfer, release/reuse and malformed-input rejection. Later tasks extend this equivalence runner rather than inventing separate baselines.

### Investigation targets
**Required:**
- `src/index.ts:27`
- `src/envelope.ts`
- `src/torsion.ts`
- `experiments/rust-surface-benchmark/shared.ts:13`
- `experiments/rust-surface-benchmark/browser.ts:89`
- `.flow/evidence/fn6-task7/README.md`

### Key context
The prototype ABI is trusted-input benchmark code. Its fixed source revision predates final FN6; reuse ownership/test principles, not its fixtures as the migration reference.

## Acceptance
- [ ] Native and browser-Wasm foundational proof passes with explicit transfer/release ownership.
- [ ] Reference provenance, comparison policy and reproducible ordinary/giant/boundary fixtures exist.
- [ ] Foundation tests cover deterministic streams, finite validation and envelope/field boundaries.
- [ ] Clean documented setup builds both targets without /tmp toolchain dependencies.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
