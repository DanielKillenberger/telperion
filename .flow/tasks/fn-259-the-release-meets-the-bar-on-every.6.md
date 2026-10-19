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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
