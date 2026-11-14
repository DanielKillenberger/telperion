# fn-354-wood-that-lasts-the-budget-the-girth.1 The budget fix carried onto one grower; the Two Trees' height; device leftovers deleted (R1 to R3)

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
fn-349's CPU budget fix is carried onto the one grower (R1). Its four code commits were cherry-picked and resolved onto this branch's Arc<Species> and its deleted captures, and the carried patch differs from fn-349's only in formatting and those resolutions. The Two Trees take the beech's inverse height, 1/40, with its source (R2). Error::Executor, Crown::rows, expected_axes with its axes walk and its one test, and units_bound.rs are deleted (R3). most_units stays because blend_reach reads it.

Census on the oak, seed 1001: 354,856 held and 24,711 tips at 39; 3,975,234 and 195,807 at 119. These match fn-349's own head binary exactly. DRAWN.md's 354,758 / 24,702 and 4,025,632 / 199,874 came from a binary that no committed fn-349 state reproduces (FRICTION.md).

Repinned: catalogue_identity's preset digest (13_698_512_124_933_910_138 to 5_784_278_485_669_999_534) and walk digest (9_472_083_953_483_493_605 to 10_352_803_689_928_916_410), both moved by R2's rows. FINEST stays private.

Tier: product code. Narrow tests in this task; the gate and review come at the spec's end, and nothing merges before then. (model: claude-opus-5-5)

baseline: green via handoff (fn-353's gate on its branch; only .flow/ changed since, checked)

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 095e24bf3283c0f77b0a4227abcbc75e82f2a0b4, e7baaf9761436452f886d4850f5ce24c18bdbbdb, 83e4aa57cb01a9e757cff58d5096ea5ad0d616c7, ae07c8d920828af5337024abd673a9e4dbd682eb, a8ff25dfe5b317445618fc49998ecb5b1594c9de, 4ba51392f82b413272f66fcb35fed7b242d57e0f, cc2859e5ff00806c8c20787fae93ca2535235ef5, be850d49430f2f395c514d086e816a97de7d6dd3, 279faa3dbb7740c66f4b2fb9d8149b338d73c4bf, e60adedb8e412752f6d27a79f605ae966eacd344
- Tests: telperion-heavy 8 --threads 4 -- cargo nextest run --cargo-profile ci --profile gate -p telperion-space (227 passed; the long walk passed alone in 307 s under cargo test), telperion-heavy 8 --threads 4 -- cargo nextest run --cargo-profile ci --profile gate -p telperion-core (537 passed; catalogue_identity repinned then 5/5; repeat_tests passed alone in 70 s), cargo test --profile ci -p telperion-render --test film_overlay, cargo test --profile ci -p telperion-core --test generation_limit_guard; cargo test --profile ci -p telperion-render --lib compile_size, npx tsc --noEmit; npm run catalogue:check && npx vitest run (218 passed)
- PRs: