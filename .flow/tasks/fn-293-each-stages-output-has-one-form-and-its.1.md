---
satisfies: [R2, R5]
---
# fn-293-each-stages-output-has-one-form-and-its.1 Baseline digests, and the dead branch diagnostics and field-query scripts go

## Description
Records the byte-identity baseline every later task compares against, takes the before-measurements R5 asks for, and removes the small, independent dead outputs: finding 8 (`branch_diagnostics`, always zero) and the scripts and experiment that still call the field query fn-257 removed.

**Size:** M
**Files:** `crates/telperion-core/examples/artifact_digest.rs`, `crates/telperion-render/examples/generation_digest.rs`, `crates/telperion-render/src/generation/resident/digest_tests.rs` (new), `crates/telperion-render/src/generation/resident.rs`, `crates/telperion-wasm/src/generate.rs`, `crates/telperion-wasm/src/generate/tests.rs`, `src/browser/core.ts`, `scripts/benchmarks/{field,generation,generation-inputs}.mjs`, `experiments/voxel-field/voxels.mjs`, `tests/migration/README.md`, `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/`
**Touches:** [crates/telperion-core/examples/artifact_digest.rs, crates/telperion-render/examples/generation_digest.rs, crates/telperion-render/src/generation/resident/digest_tests.rs, crates/telperion-render/src/generation/resident.rs, crates/telperion-wasm/src/generate.rs, crates/telperion-wasm/src/generate/**, src/browser/core.ts, scripts/benchmarks/**, experiments/voxel-field/**, tests/migration/README.md, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- The digests are of artifacts, not of internal fields: `artifact_digest` prints the skeleton as the tree's nodes and the fields a consumer reads, not `Tree`'s derived `Debug` (which carries `pipe`, deleted by task 3, and would fail a correct change); the same projection in the render digest. `artifact_digest` takes an optional seed argument (`<preset> [seed]`), set on `family.skeleton.seed` as `generation_digest.rs:126-128` does.
- The resident digest moves into the render crate's test binary as an ignored test (`generation/resident/digest_tests.rs`, `#[ignore = "a digest"]`, printing one line a preset and seed; `DIGEST_SEEDS`, default 1,7,42), so it reads back what production keeps private without a public verification API: the placed leaves (`read_instances` there), and the resident curve as the renderer receives it, the packed point, cluster and section buffers of `Prepared.wood` with the curve's demand, each hashed (`Prepared::curve()` is the empty CPU row on the resident path, so its counts prove nothing). `generation_digest`'s pipeline lines (CPU artifacts, mesh) stay in the example. Record, before any other edit, `BASELINE.md` in the evidence folder: one sha256 per shipped preset and seed 1, 7, 42 from `artifact_digest`, and the full `generation_digest` example output and the resident digest test output (`DIGEST_SEEDS=1,7,42`) saved to `raw/` with its sha256 per line in BASELINE.md. Run each twice to show it repeats. Use `--profile ci`, never `--release`.
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
## Acceptance
- [ ] The digests project artifacts (no internal `Tree` field such as `pipe`); the resident digest hashes the GPU curve's point, cluster and section buffers, its demand, and the placed leaves.
- [ ] `BASELINE.md` holds repeatable digests for every shipped preset at seeds 1, 7, 42 (core) and the render digest lines; `MEASURE.md` holds the before-numbers.
- [ ] No `branch_diagnostics` and none of its four keys in Rust, TS or tests.
- [ ] No file in the repo calls the removed field query or loads `telperion-field.wasm`.
- [ ] Digests after the task equal the baseline; gate and `npm test` green.
## Done summary
The digests now project artifacts (`artifact_digest <preset> [seed]` and `generation_digest` read the skeleton's consumer fields, never `Tree.pipe`), and a new ignored render test (`resident/digest_tests.rs`) hashes what the device hands the renderer: placed leaves and the packed curve's point, cluster and section words with its demand. BASELINE.md records every shipped preset at seeds 1, 7, 42 on all three digests, each run twice and identical; MEASURE.md records the before-numbers (lines per crate, package bytes raw and gzip, cold release Wasm build time). `branch_diagnostics` and its four JSON keys are gone from Rust, TS and tests; `scripts/benchmarks/{field,generation,generation-inputs}.mjs` and `experiments/voxel-field` are deleted (the migration was a rewrite, and Node has no WebGPU for `growField`); `tests/migration/README.md` names `{ field: { n: 36 } }`. All three digests equal the baseline after the task.

Outside the declared Touches, two changes were needed: `crates/telperion-render/src/curve.rs` (a `#[cfg(test)]` sections accessor, and `COPY_SRC` on uploaded curve blocks, so the digest can read the section words), and `docs/generation-limits-inventory.json` and `docs/generation-limits.md` (the deleted diagnostics' limit site and note, which the gate's `generation_limit_guard` flagged as stale).

Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, 1416 passed and 1 failed (`generation_limits_are_classified`, the stale inventory site). Fixed in 55c317fde; the focused guard test passes (2/0). `npm test`: 170/0.

Tier: not reported (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e927da9f27204e10d709d024c2a2a791fbeb9a0c, 55c317fdec1297eaa7bbd87ca1fe171e1945dcf7
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1416 passed, 1 failed: generation_limits_are_classified, fixed in 55c317fde), cargo test --profile ci -p telperion-core --test generation_limit_guard (2 passed), npm test (170 passed), cargo test --profile ci -p telperion-wasm (6 passed), digests: artifact_digest, generation_digest, resident digest_tests::digest at seeds 1,7,42 equal BASELINE.md
- PRs: