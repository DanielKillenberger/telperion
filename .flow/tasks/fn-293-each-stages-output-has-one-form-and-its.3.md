---
satisfies: [R2, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.3 The scaffold grower, its validation preconditions and Tree.pipe leave production

## Description
Finding 4 and finding 15: the retired scaffold grower compiles only into tests (fn-288 migrates them and deletes it), `validate_skeleton` keeps only checks production rows need, `Tree.pipe` goes, and `blend`'s node-count prediction compiles only for the suite.

**Size:** M (wide but mechanical)
**Files:** `crates/telperion-core/src/pipeline.rs`, `pipeline/{bias,branching,colonization,radius,twigs}.rs` and their directories, `pipeline/branching/specimen.rs`, `family.rs`, `tree.rs`, `pipeline/foliage/{placement,clumping,plan}.rs`, `pipeline/executor.rs`, `blend.rs`, `blend/size.rs`, `pipeline/contract.rs`, `crates/telperion-wasm/src/generate.rs` (if it still names `twigs::`)
**Touches:** [crates/telperion-core/src/pipeline.rs, crates/telperion-core/src/pipeline/bias*, crates/telperion-core/src/pipeline/branching*, crates/telperion-core/src/pipeline/colonization*, crates/telperion-core/src/pipeline/radius*, crates/telperion-core/src/pipeline/twigs*, crates/telperion-core/src/family.rs, crates/telperion-core/src/tree.rs, crates/telperion-core/src/pipeline/foliage/placement.rs, crates/telperion-core/src/pipeline/foliage/clumping.rs, crates/telperion-core/src/pipeline/foliage/plan.rs, crates/telperion-core/src/pipeline/executor.rs, crates/telperion-core/src/pipeline/contract.rs, crates/telperion-core/src/blend*, crates/telperion-core/src/catalogue/**, crates/telperion-core/src/lib.rs, crates/telperion-core/src/mesh.rs, crates/telperion-wasm/**, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Replace the five `cfg_attr(not(test), allow(dead_code))` modules (`pipeline.rs:262-280`) with `cfg(test)` for everything production does not read. What production reads stays compiled: `clothe_leaf_bases`, the parameter and wire types re-exported through `contract.rs:6-24` and mounted by `catalogue/mount.rs:118-144`, `twigs::TwigParams`, `twigs::child_radius` if still read. Move a production-read item out of a test-only module only if it cannot stay where it is; do not rename.
- `validate_skeleton` (`branching.rs:29`, `specimen.rs:40-50`, called from `family.rs:121`): list each check and the production reader of the row it guards; keep checks on rows the engine or a drawing stage reads, drop the ones guarding scaffold-only rows (`inner_envelope`, `resolved_growth`, `scaffold::forks_placed` per the audit). Record the list in the task summary. A family that cannot be grown is still refused by name.
- Delete `Tree.pipe` (`tree.rs:137,256-261,337`) and `pipe()`; each reader (`executor.rs:206`, `placement.rs:102,238,271,276`, `clumping.rs:36`, `plan.rs:472,475`) reads the node radius `pipe()` already returned in production. The scaffold's `radius/hold.rs:58` writer stays test-only; if a suite test reads `pipe`, give it the scaffold's own value locally.
- `blend/size.rs` and `blend`'s suite-only items: `cfg(test)`; `expected_kept` stays (production reads it at `pipeline/space.rs:223`).
- Keep `docs/pipeline.md`'s compile-fail boundary tests (`crates/telperion-wasm/tests/boundary.rs`) meaningful: if the scaffold's grower no longer exists outside tests, its compile-fail case still fails to compile; adjust its positive control only if it named a removed item.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline.rs:180-285`
- `crates/telperion-core/src/pipeline/branching.rs:1-60`, `branching/specimen.rs:30-60`
- `crates/telperion-core/src/tree.rs:130-140,250-265,330-340`
- `crates/telperion-core/src/catalogue/mount.rs:110-150`
**Optional:**
- `.flow/specs/fn-288-the-scaffold-test-generator-retires.md`

### Key context
The 32 suite and test files that grow on the scaffold are fn-288's; they must still compile under `cfg(test)`. Do not migrate them. The `core-plan` CI job builds `--no-default-features`; build it once too (`cargo build --profile ci -p telperion-core --no-default-features`).
## Acceptance
- [ ] The scaffold grower's modules compile only under `cfg(test)`, except the named production items.
- [ ] `Tree.pipe` and `pipe()` are gone; `validate_skeleton`'s kept and dropped checks are listed with their readers.
- [ ] Digests equal the baseline (core and render, seeds 1, 7, 42).
- [ ] Non-test lines of `telperion-core` before and after in `MEASURE.md`; gate and `npm test` green; the no-default-features build passes.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
