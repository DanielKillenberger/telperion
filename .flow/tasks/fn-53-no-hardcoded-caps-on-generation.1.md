---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-53-no-hardcoded-caps-on-generation.1 Implement no hardcoded caps on generation

## Description
Implement the spec's R1 to R7: inventory and classify every limit, turn caps into ranged parameters or remove them, measure cost, rewrite count-gated tests, add the guard.

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
R1 to R7 in 320c3d02; all five Fable review findings fixed in 08c5357d (NEEDS_WORK then SHIP). Caller budgets replace independent caps; default preset geometry unchanged across all 21 measured pairs; beech with a 1,000,000 budget grows 254,252 nodes uncapped; eleven former limits are named validated parameters; snapshot schema 3. Packed foliage index pages 25,105 to 4,229 on the bounded fixture. 569 Rust tests, 99 Vitest tests, Wasm bindings, typecheck, catalogue and fmt green. Owner requested direct merge without a PR.

## Evidence
- Commits: 320c3d028843ee8d4a8d21a39e7ecf753d604f9a, 08c5357df45ebdcfa3bb54cc52cc013b650ff19d
- Tests: cargo test --profile ci -p telperion-core -p telperion-wasm -p telperion-jev --no-fail-fast, npx vitest run, node scripts/test-wasm.mjs, npx tsc --noEmit, node scripts/catalogue-check.mjs, cargo test --profile ci -p telperion-core --test generation_limits caller_budget_above_old_ceiling_grows_the_complete_beech
- PRs:
