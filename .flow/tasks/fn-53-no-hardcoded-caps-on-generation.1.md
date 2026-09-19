---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-53-no-hardcoded-caps-on-generation.1 Implement no hardcoded caps on generation

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Implemented R1–R7 in 320c3d02 and fixed all five Fable findings in 08c5357d. Caller budgets replace independent caps; default preset geometry is unchanged across all 21 measured sample pairs. Beech with an explicit 1,000,000-node budget produces 254,252 nodes uncapped. Eleven former limits have named validated parameters; snapshot schema is now 3. Inventory, guard, compatibility and cost evidence are committed under docs/ and .flow/evidence/fn-53-no-hardcoded-caps-on-generation/.

Fable medium returned NEEDS_WORK then SHIP after reviewing the fixes in the same read-only session. All R1–R7 were marked met. Final gates: 569 Rust tests passed, 13 ignored; 99 Vitest tests passed; browser/Wasm bindings, typecheck, catalogue and formatting passed. Packed foliage index pages fell from 25,105 to 4,229 on the bounded review fixture; see MEASUREMENTS.md for qualifications.

stage: impl-review - ran (model: claude-fable-5-1 medium)
stage: qa - skipped(config: pipeline.qa=auto: no UI-observable acceptance criteria, Jev 0.06)
stage: plan-sync - skipped(config: disabled)

Owner requested direct merge without a PR. Review and tests are complete; no PR was created. The host read all five FRICTION.md entries and will report proposed remedies without creating new specs.
## Evidence
- Commits: 320c3d028843ee8d4a8d21a39e7ecf753d604f9a, 08c5357df45ebdcfa3bb54cc52cc013b650ff19d
- Tests: baseline: no spec Quick commands; additional pre-edit cargo test --profile ci -p telperion-core passed, timeout 600s cargo test --profile ci -p telperion-core -p telperion-wasm -p telperion-jev --no-fail-fast (exit 0; .flow/tmp/fn53-rust-green.log), npx vitest run (exit 0; 98 tests; .flow/tmp/fn53-vitest2.log), node scripts/test-wasm.mjs (exit 0; .flow/tmp/fn53-wasm-parity2.log), npm run render:build (exit 0; .flow/tmp/fn53-render-build.log), npx tsc --noEmit (exit 0; .flow/tmp/fn53-typecheck2.log), node scripts/catalogue-check.mjs (exit 0), cargo fmt --all -- --check (exit 0), git diff --check (exit 0), Matched direct mature benchmark: all 21 before/after sample pairs identical tree/wood/foliage hashes and counts; .flow/evidence/fn-53-no-hardcoded-caps-on-generation/MEASUREMENTS.md, cargo test --profile ci -p telperion-core --test generation_limits caller_budget_above_old_ceiling_grows_the_complete_beech -- --nocapture (exit 0; 254252 nodes uncapped; .flow/tmp/fn53-high-budget-measurement.log), gate classify --base 38e6c57f1ee00836a6e44e667dd468dfebed51ce => FULL, Post-Fable: timeout 600s cargo test --profile ci -p telperion-core -p telperion-wasm -p telperion-jev --no-fail-fast (exit 0; .flow/tmp/fn53-review-rust-green.log), Post-Fable: npx vitest run (exit 0; 99 tests; .flow/tmp/fn53-review-vitest.log), Post-Fable: node scripts/test-wasm.mjs (exit 0; .flow/tmp/fn53-review-bindings.log), Post-Fable: npx tsc --noEmit; node scripts/catalogue-check.mjs (both exit 0), Post-Fable: cargo fmt --all -- --check; git diff --check (both exit 0), Packed-cache density red-to-green: 1569 pages for 256 births before; nested maps pass the active density bound; .flow/tmp/fn53-map-{red,green,focused}.log, Packed-cache bounded benchmark: three paired processes, three samples each, 4096 births x16 stations; index pages 25105 before,4229 nested,6211 historical dense; raw .flow/evidence/fn-53-no-hardcoded-caps-on-generation/packed-map-{before,after}.jsonl, Slider working-window regression red-to-green; above-window hydration exact; /tmp/fn53-slider-{red,green}.log, INCONCLUSIVE: helper broad debug cargo test -p telperion-wasm --lib interrupted after diagnostics passed; superseded by final optimized full gate; /tmp/fn53-review-wasm.log
- PRs: