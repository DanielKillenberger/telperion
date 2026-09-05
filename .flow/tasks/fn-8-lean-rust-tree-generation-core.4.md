---
satisfies: [R1, R3]
---

# fn-8-lean-rust-tree-generation-core.4 Port the final swept surface to independent Rust output

## Description
Port surface generation against frozen solved-tree fixtures, independently of the live growth port.

**Size:** M
**Files:** Rust surface/path/frame/normal modules and geometry tests
**Touches:** [crates/telperion-core/src/surface/**, crates/telperion-core/src/surface.rs, crates/telperion-core/tests/surface*, crates/telperion-core/tests/surface_reference*]

## Approach
- Adapt FN7's flat preallocated output patterns to the final FN6 surface contract; account for final endpoint-radius and all current surface details.
- Keep surface generation independent of foliage and Three objects, exposing positions, indices, normals, bounds and needed diagnostics.
- Use checked size arithmetic and explicit allocation failure; verify every generated index and vertex.
- Reuse the task-1 fixture runner for topology/winding invariants and measured attribute differences, including degenerate edges, forks, taper and empty surfaces.

## Investigation targets
**Required:**
- `src/mesh/surface.ts:195`
- `src/mesh/paths.ts`
- `src/mesh/frames.ts`
- `src/mesh/surface.test.ts`
- `experiments/rust-surface-benchmark/rust/lib.rs:69`
- `experiments/rust-surface-benchmark/REPORT.md`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.


## Acceptance
- [ ] Final-FN6 surface fixtures pass geometry equivalence, including normals and bounds.
- [ ] Empty, degenerate and invalid-input outcomes match the new contract without partial successful buffers.
- [ ] Surface creation performs no foliage work and imports no renderer/binding types.

## Done summary
Ported the final swept wood surface to the shared solved Tree contract with caller-owned float32 positions/normals, uint32 indices and optional bounds. Flat adjacency/path storage, reusable sample/frame scratch, checked size math and fallible reservations replace FN7 experiment allocations; the stage imports no foliage or renderer types.

baseline: none (approved parent and task define no Quick commands). Initial focused test failed on the missing surface API before implementation. Final workspace suite passes 11 tests; the explicitly invoked reference integration also passes. Rustfmt and workspace clippy with warnings denied pass. Gate classification is FULL; no spec-defined gate skip lines or shared receipts were fabricated.

R1: Final FN6 ordinary (377568 vertices), Telperion (6554490), Laurelin (2246930), empty and capped fixtures match connectivity, positions, Three area-weighted normals and bounds. Maximum position and normal component errors are both zero. The comparison uses pinned final FN6 solved radius buffers, preserving endpoint taper, rather than FN7's earlier input semantics. The reproducible runner and policy are in crates/telperion-core/tests/surface_reference.mjs and surface_reference.md. Raw buffers are /tmp/fn8-surface-reference; no large generated buffers committed. This does not claim new-grower equivalence, browser visual QA or total migration completion.

R3/boundaries: five focused integration tests cover closed outward winding and nonzero area, endpoint taper, unit normals/bounds, contained sockets, deterministic output, reversal frames, duplicate-edge collapse, empty/root-only output, malformed parent order, invalid parameters/heights and float32 representability. One unit test exercises allocation rejection. Invalid/unrepresentable output returns an error with no partial successful mesh. Nonfinite/out-of-range surface parameters now fail clearly rather than silently using TS fallback/clamp behavior; valid lobe counts still raise radial resolution. Empty output uses None bounds, including raw zero-radius root-only trees.

API and notes: /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/surface.md. All implementation changes stay within declared Touches. Tooling rejected truncating writes into the notes directory; an append-only note publication succeeded without changing unrelated files.


Worker handover was followed by conductor integration and verified completion; the integration checks are recorded below.

Integrated and focused checks passed on conductor branch.
stage: impl-review - skipped(user: none)
stage: wave-join - ran
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8e9295f70e182fb85620fa56e94485ef8b094238
- Tests: baseline: none (approved parent/task define no Quick commands), RED: cargo test -p telperion-core --test surface (expected unresolved surface API; /tmp/fn8-surface-red.log), cargo test --workspace (11 passed, reference integration separately invoked; /tmp/fn8-surface-workspace-test.log), cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings (/tmp/fn8-surface-clippy.log), REFERENCE_OUTPUT=/tmp/fn8-surface-reference node crates/telperion-core/tests/surface_reference.mjs (/tmp/fn8-surface-comparison.log), SURFACE_REFERENCE=/tmp/fn8-surface-reference SURFACE_CASES=ordinary,telperion,laurelin,empty,capped cargo test -p telperion-core --test surface_reference -- --ignored --nocapture (/tmp/fn8-surface-final-comparison.log), flowctl gate classify --base 09484a8bbc30a7397c093ae7f095c5571c87a404 (FULL; no spec-defined Quick gate receipts), Integrated target: cargo test -p telperion-core --test surface (5 passed), Integrated target: SURFACE_REFERENCE=/tmp/fn8-surface-reference SURFACE_CASES=ordinary cargo test -p telperion-core --test surface_reference -- --ignored --nocapture (passed, zero position/normal error), Integrated target: cargo fmt --all -- --check (passed)
- PRs:
