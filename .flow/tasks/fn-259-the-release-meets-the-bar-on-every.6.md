---
satisfies: [R2]
---
# fn-259-the-release-meets-the-bar-on-every.6 Audit the shipped artifacts for replaced code and report their sizes

## Description
R2's content and reachability audit of every shipped module, the inventory derived from today's `npm run build` `dist/` and the `npm pack --dry-run` file list (fn-293 `CONTRACTS.md` records the field module's removal: report it absent) and every embedded shader, for contract A, the stepped light executor, the fine-root cut and any grower or predictor the stages replaced; the lattice reported as present by decision. Sizes reported separately, raw/gzip/brotli.

**Size:** M
**Files:** `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/AUDIT.md`; an audit script (`scripts/shipped-audit.mjs` with its test) only if a check is worth keeping
**Touches:** [.flow/evidence/fn-259-the-release-meets-the-bar-on-every/AUDIT.md, scripts/shipped-audit.mjs, scripts/shipped-audit.test.mjs]

### Approach
- Name list first: from fn-293's `PIPELINE-AUDIT.md` and `CONTRACTS.md`, fn-245 `ASTRA-ARCH.md`, and fn-237's "Astra's release review, incorporated", write the exact symbols, modules and shader files each replaced thing was called. A name that cannot be pinned to a symbol is written as unknown, never assumed absent.
- Content: search the built Wasm's name section or exports (`wasm-objdump`/`wasm-tools` if installed, else a name-section reader), the JS bundles, and `shaders::all()` + `generation::sources::all()` (`crates/telperion-render/src/shader_tests.rs:31-36`) for each name.
- Reachability: for a name found in source (e.g. `fine.rs`'s "fine roots grown below a cut", `cycle/units.rs:3`'s fine-root shader binds), show whether a shipped entry reaches it; report found-and-reachable, found-and-unreachable, or absent.
- Sizes from `npm run build`'s `dist/` against `scripts/artifact-budgets.json`.

### Investigation targets
**Required:**
- `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/PIPELINE-AUDIT.md`, `CONTRACTS.md`
- `crates/telperion-render/src/shader_tests.rs`
- `scripts/artifact-budgets.mjs`, `scripts/artifact-budgets.json`
- `crates/telperion-core/src/pipeline/branching/lattice.rs` - the lattice that stays

### Key context
Qualification only: a replaced thing still shipped is reported as a finding with its size, and the fix is its own spec (the host mints it), not done here.
## Acceptance
- [ ] AUDIT.md names each item's symbol(s), where searched, and found/reachable/absent; unknowns written as unknown.
- [ ] The lattice reported as present by the 2026-10-16 decision.
- [ ] Artifact sizes raw/gzip/brotli against their ceilings, apart from the audit.
## Done summary
`AUDIT.md` audits today's `npm run build` package (19 files from `npm pack --dry-run`) for the code fn-245's stages replaced. It covers both Wasm modules through their name sections (twiggy names, then `twiggy paths` to an export), every `.wgsl` file matched against the renderer module's bytes, the 44 deleted shaders and 14 renamed ones from git history, and the JS and `.d.ts` files.

- **Absent:** contract A (Q32/Q62, `fixed`, the WGSL mirror), the stepped light executor, the fine-root executor (CPU and device), the experimental growers, the kept recursion, the scaffold grower, the species constructors, the CPU delivery, `branch_diagnostics`, `telperion-field.wasm` and the field snapshot.
- **Present by decision, found and reachable:** the light lattice (CPU `telperion_space::light` and device `cycle/light*.wgsl`; host 2026-10-16) and the cut's predictor.
- **Leftovers that ship:**
  - a never-constructed `Error::Executor` whose message names "the fine roots' executor";
  - a stale "chain grower" comment in the shipped `zones.wgsl`;
  - the leaf-base lattice (`branching/lattice.rs`), inlined into `pipeline::clothe`.
- **Open for the host:**
  1. Does R2's "contract A" cover the exact arithmetic that ships: `soft64.wgsl`, `held.wgsl` and the pinned transcendentals (fn-256 DESIGN §8, fn-198)?
  2. Is the task's `lattice.rs` pointer the leaf-base lattice, when the decision means the light lattice?
- **Sizes:** raw, gzip-9 and brotli-11 are reported apart, and all six artifacts are under their ceilings (`artifact-budgets.mjs` exit 0). The name sections are 15.8% and 15.3% of the two modules, a size question for fn-242. No script was kept.

baseline: green via handoff (fn-259.3 at d59dac4e7; only .flow/ paths changed since). Gates: tier-B docs-only, so GATE_SKIPPED for fmt, bindings, npm-test and the cargo gate.

Tier: not provided by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex gpt-6.1-sol, SHIP; P2 leaf-base lattice path and P3 shader match counts both fixed in 944032f1d)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1453a8c26be915ac099ce18daed6db0cf3e17764, 944032f1df51b5ff76cb0cd731d0938cf917e113
- Tests: telperion-heavy 8 --threads 4 -- npm run build (rc 0), node scripts/artifact-budgets.mjs (rc 0), GATE_SKIPPED:fmt:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:bindings:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:npm-test:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:gate:docs-only - cumulative diff classified tier-B (no executable paths touched)
- PRs: