---
satisfies: [R2, R5]
---
# fn-293-each-stages-output-has-one-form-and-its.1 Baseline digests, and the dead branch diagnostics and field-query scripts go

## Description
Records the byte-identity baseline every later task compares against, takes the before-measurements R5 asks for, and removes the small, independent dead outputs: finding 8 (`branch_diagnostics`, always zero) and the scripts and experiment that still call the field query fn-257 removed.

**Size:** M
**Files:** `crates/telperion-core/examples/artifact_digest.rs`, `crates/telperion-wasm/src/generate.rs`, `crates/telperion-wasm/src/generate/tests.rs`, `src/browser/core.ts`, `scripts/benchmarks/{field,generation,generation-inputs}.mjs`, `experiments/voxel-field/voxels.mjs`, `tests/migration/README.md`, `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/`
**Touches:** [crates/telperion-core/examples/artifact_digest.rs, crates/telperion-wasm/src/generate.rs, crates/telperion-wasm/src/generate/**, src/browser/core.ts, scripts/benchmarks/**, experiments/voxel-field/**, tests/migration/README.md, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- `artifact_digest` takes an optional seed argument (`<preset> [seed]`), set on `family.skeleton.seed` as `generation_digest.rs:126-128` does. Record, before any other edit, `BASELINE.md` in the evidence folder: one sha256 per shipped preset and seed 1, 7, 42 from `artifact_digest`, and the full `DIGEST_SEEDS=1,7,42 generation_digest` output saved to `raw/` with its sha256 per line in BASELINE.md. Run each twice to show it repeats. Use `--profile ci`, never `--release`.
- Record the R5 before-numbers in `MEASURE.md`: non-test lines per crate (`tokei` or `wc -l` excluding `*_tests.rs`, `tests/`, `suite/`), the packed Wasm modules' bytes raw and gzip from the package build the size budget reads (`scripts/artifact-budgets.mjs` names the files), and the release Wasm build's wall time.
- Delete `branch_diagnostics` (`generate.rs:58-81`), its call and the four JSON keys (`generationCounts`, `handoffs`, `levelCappedHandoffs`, `twigs`); drop them from the TS type in `src/browser/core.ts:51-52` and from `generate/tests.rs:47-67`. Keep `crossover` (still emitted, a separate question).
- Migrate the two benchmark scripts and `generation-inputs.mjs` to the grid field (`src/field/index.ts`, `growField`/`{ field: { n } }` as `docs/field-package.md` describes) if the migration is a few lines; otherwise delete them (no package script or workflow calls them). Delete `experiments/voxel-field/voxels.mjs`'s load of the removed `telperion-field.wasm` or the experiment itself. Fix `tests/migration/README.md:95`.

### Investigation targets
**Required:**
- `crates/telperion-wasm/src/generate.rs:58-81,150-215`
- `crates/telperion-render/examples/generation_digest.rs:100-140`
- `scripts/artifact-budgets.mjs`, `scripts/build-wasm.mjs`
**Optional:**
- `docs/field-package.md`, `src/field/index.ts`

### Key context
Do not touch the untracked `src/field/telperion-field.wasm` in other checkouts. The digest examples are the proof every later task uses; if a digest differs between two runs of the same commit, stop and report it.

### Acceptance
- [ ] `BASELINE.md` holds repeatable digests for every shipped preset at seeds 1, 7, 42 (core) and the render digest lines; `MEASURE.md` holds the before-numbers.
- [ ] No `branch_diagnostics` and none of its four keys in Rust, TS or tests.
- [ ] No file in the repo calls the removed field query or loads `telperion-field.wasm`.
- [ ] Digests after the task equal the baseline; gate and `npm test` green.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
