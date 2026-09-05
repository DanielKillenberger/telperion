---
satisfies: [R1, R2, R4, R7]
---

# fn-8-lean-rust-tree-generation-core.1 Pin the final reference and establish the lean native/Wasm foundation

## Description
Establish the numerical and ownership contracts before the botanical port. Implements the early proof point and R1 foundation.

**Size:** M
**Files:** new Cargo workspace, core math/types modules, Wasm binding scaffold, migration fixture runner, package scripts
**Touches:** [.gitignore, Cargo.toml, Cargo.lock, rust-toolchain.toml, crates/**, tests/migration/**, scripts/**, package.json, package-lock.json]

## Approach
- Use one renderer-independent core crate and a thin Wasm binding crate; expose the small native contract in the parent spec. Pin a reproducible toolchain and build commands.
- Port deterministic RNG, vector operations, envelope and shared field primitives; test their boundary semantics before growth. Declare compact tree/diagnostic/output types using domain names and owned storage.
- Build a reference exporter runnable against the pinned final-FN6 Git revision in a disposable checkout, without importing the old production generator into the new production package. Capture stage fixtures, raw parameter manifests and matched visual camera definitions. Keep giant fixtures reproducible without requiring enormous committed buffers.
- Establish discrete invariants, comparison metrics and field-specific numeric tolerances before ported results are inspected. Include ordinary, Telperion, Laurelin, empty, degenerate, envelope-crossing and capped cases.
- Prove a small deterministic foundational result runs natively and through browser Wasm, including owned transfer, release/reuse and malformed-input rejection. Later tasks extend this equivalence runner rather than inventing separate baselines.

## Investigation targets
**Required:**
- `src/index.ts:27`
- `src/envelope.ts`
- `src/torsion.ts`
- `experiments/rust-surface-benchmark/shared.ts:13`
- `experiments/rust-surface-benchmark/browser.ts:89`
- `.flow/evidence/fn6-task7/README.md`

## Key context
The prototype ABI is trusted-input benchmark code. Its fixed source revision predates final FN6; reuse ownership/test principles, not its fixtures as the migration reference.

## Approved capture alignment
The rewritten parent spec supersedes the earlier plan. Baseline comparisons diagnose drift; byte-identical old output and exact topology are not compatibility obligations. Preserve meaningful botanical invariants and record differences. The data contract must support later mesh-free wood/foliage field sampling. Lean code is a primary acceptance requirement. Establish simple public modules for colonization, branching, surface, foliage and field so later tasks can work in their modules without colliding on root declarations; empty module files are scaffolding, never fake successful generation APIs. Keep representative comparative fixtures reproducible and small; avoid checking in giant full-tree buffers.


## Acceptance
- [ ] Native and browser-Wasm foundational proof passes with explicit transfer/release ownership.
- [ ] Reference provenance, comparison policy and reproducible ordinary/giant/boundary fixtures exist.
- [ ] Foundation tests cover deterministic streams, finite validation and envelope/field boundaries.
- [ ] Clean documented setup builds both targets without /tmp toolchain dependencies.

## Done summary
Added dependency-free Rust core and thin Wasm foundation with owned tree storage, deterministic math/RNG/envelope/noise/bias primitives, and independently owned stage modules. The native/browser ownership proof passes; final-FN6 provenance, seven reproducible compact fixtures, comparison policy and standard rustup build instructions are in tests/migration/README.md.

baseline: none (the approved parent and task contain no Quick commands). The first new-suite run failed for the expected absent Cargo workspace; final native suite has 5 passing tests. Browser proof compares 192 owned values (max error 0) and verifies idempotent release, reuse, malformed-input clearing, limits, and empty results. Rustfmt, clippy, npm build and reference comparison pass. All seven reference fixtures ran sequentially, including Telperion and Laurelin; large raw buffers remain opt-in and ignored. Tests and raw logs are recorded in the evidence JSON.

Contracts for downstream workers: /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/foundation.md. Node.base_radius holds branch allocation independently of solved edge radii. Raw skeleton radii may be zero; validate_solved enforces positive output radii. Stage modules remain empty until their assigned ports; the foundation does not claim a botanical or visual migration comparison is complete. The pinned compiler is installed by rustup and no committed build command depends on /tmp; this execution reused the provided temporary toolchain and Playwright installation via environment variables.


Worker handover was followed by conductor integration and verified completion; the integration checks are recorded below.

Integrated and verified on the conductor branch.
stage: impl-review - skipped(user: none)
stage: wave-join - ran
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b410f3ba1fb304ed0810712262cc1534ea122442
- Tests: baseline: none (approved parent and task define no Quick commands), Initial red: cargo test --workspace failed because Cargo.toml did not yet exist; /tmp/fn8-foundation-red.log, cargo test --workspace: 5 passed; /tmp/fn8-foundation-final-native.log, cargo clippy --workspace --all-targets -- -D warnings: passed; /tmp/fn8-foundation-clippy.log, cargo fmt --all -- --check: passed, PLAYWRIGHT_MODULE=/tmp/fn20-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=/usr/bin/chromium node scripts/test-wasm.mjs: passed; 192 values, native/Wasm maxError=0, ownership/release/reuse/malformed/resource/empty checks; /tmp/fn8-foundation-final-wasm.log, node scripts/export-reference.mjs: all 7 final-FN6 cases passed; /tmp/fn8-foundation-reference-all.log, REFERENCE_OUTPUT=/tmp/fn8-foundation-reference-check node scripts/export-reference.mjs ordinary: passed final provenance metadata; /tmp/fn8-foundation-reference-check.log, node scripts/compare-migration.mjs tests/migration/fixtures tests/migration/generated: passed; /tmp/fn8-foundation-final-compare.log, npm run build: passed; /tmp/fn8-foundation-build.log, git diff --check: passed, flowctl gate classify --base 16070c144b79d6b738d05fc198ba297a52de9248: FULL (executable changes); no spec-defined full gates to receipt, Integrated target: cargo test --workspace (5 passed), Integrated target: cargo fmt --all -- --check (passed), Integrated target: actual Chromium scripts/test-wasm.mjs (192 values, maxError 0, ownership/error checks passed)
- PRs:
