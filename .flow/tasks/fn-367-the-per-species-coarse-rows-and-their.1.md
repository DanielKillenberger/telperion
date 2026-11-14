# fn-367-the-per-species-coarse-rows-and-their.1 Delete the coarse rows and the fidelity dial

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Deleted the per-species `/space/coarse` rows (oak, spruce, birch), `Coarsening`/`COARSEST`/`Species::coarsened`, their reads, `pipeline/fidelity_tests.rs` and the `/fidelity` parameter (retired by name); generated files rewritten by their generators; Wasm 1,556,211 -> 1,547,984 bytes; 763 lines deleted, 69 added. R1 found no reader of the parameter outside the coarse rows (`R1-READERS.md`).

R3: `every_preset_grows_the_same_tree_again_and_on_any_cores` passed. The renamed pin `every_preset_grows_the_tree_it_grew_at_sixteen` is red on this branch for a cause inherited from the base (fn-364's rule); its eight digests are equal before and after this change, so no tree moved. Pinned values left for fn-364's re-pin (`72169847a`); the host checks it green after merging fn-364's tip (host, 14 Nov 2026). Details in `.flow/evidence/fn-367-the-per-species-coarse-rows-and-their/RESULT.md`.

baseline: red (the both-ends digest pin failed at 78a8e6a94, inherited)

stage: impl-review - skipped(policy: host-deferred - the host runs reviews at the phase's landing)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 476c739ed9bbca7af0a642bb16954e144a45a486, d0a73a75d2c6ad40276f3126547fcc732a16c977, d3da19cb052cb32bf623411d2c5b6bed99f2eebf
- Tests: cargo test --profile ci -p telperion-core --test catalogue_identity, cargo test --profile ci -p telperion-core --test parameter_reference, cargo test --profile ci -p telperion-core --lib -- catalogue::tests, cargo test --profile ci -p telperion-core --lib -- --ignored --exact pipeline::space::repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores, cargo test --profile ci -p telperion-core --lib -- every_preset_grows_the_tree_it_grew_at_sixteen (red, inherited from base; digests equal before and after), npx vitest run harness/params.test.ts harness/rows.test.ts harness/panel.test.ts harness/space.test.ts harness/family.test.ts src/browser/presets.test.ts, cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size
- PRs: