---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-100-the-field-reads-the-plan-not-placed.1 Implement The field reads the plan, not placed leaves

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The field answers foliage from a leaf plan read off the solved tree (bearing runs, per-segment station counts, limb system, conservative reach) instead of placed leaves; the station preparation is built on the same plan; the Wasm binding places no leaf for a field request where the family has a plan and reports its stages, batch queries add a leaf-count estimate and a limb id, and the core builds with the wood surface and the placement compiled out behind the new default `geometry` feature.

Tier: session (jev intelligent 0.53); actual model: claude-fable-5-1 (session model, no bridge)

stage: impl-review - skipped(config: REVIEW_MODE=none)

Baseline: none (the spec lists no Quick commands; the CLAUDE.md gate ran once at the end and is green).

Acceptance, per criterion (measurements and sources in `.flow/evidence/fn-100-the-field-reads-the-plan-not-placed/RESULTS.md`):
- R1 satisfied: `telperion-wasm` test `a_field_request_reads_the_plan_and_names_its_stages_for_every_preset`; every shipped preset but the beech takes the plan path with placement, cull and contacts false.
- R2 satisfied: field-only medians with a 64^3 batch query, oak 940→288 / 1,071→330 ms, birch 2,362→345 / 2,598→579 ms, spruce 9,002→233 / 8,646→223 ms (seeds 1, 7), each faster than the same commit's surface-plus-foliage build; peak linear memory lower on every tree; field bytes ratio 0.33-0.34 oak, 0.52-0.53 birch, 0.044-0.045 spruce; wood share 0.65-0.68 oak, 0.66-0.68 birch, 0.48 spruce.
- R3 satisfied: `tests/field.rs` unchanged and green; wood bits byte-identical against the base binding on eight trees (the 8-tree forest harness no longer exists; the eight are named in RESULTS.md and FRICTION.md).
- R4, R5, R6 satisfied: `tests/field_plan.rs` (vertex coverage at 0.1/0.25/1 m; over-coverage 1.39/1.63/1.16 at 0.25 m; estimates sum within 1%; two-limb fixture; spruce needles; boundary cube).
- R7 satisfied: `core-plan` CI job and the local no-default-features build and lib tests. Scope note: `branching::generate` runs through the specimen, whose growth timeline carries the per-shoot placement, so the feature also gates the specimen's aged reads (FRICTION.md, first entry).
- R8 awaiting the owner: `raw/sheet-64-base-structure.png` beside `raw/sheet-64-candidate-field.png`, described in RESULTS.md. Not marked satisfied.

Follow-ups noted, not built: a flags-only batch query could keep the placed path's early exit (per-cell query cost about doubled); a consumer-facing limb order for finer clumps; the base prototype's low spruce came from clothing twig nodes only, the field clothes the leader the placement clothes.

Friction: two dated entries in `.flow/evidence/fn-100-the-field-reads-the-plan-not-placed/FRICTION.md` (R7's growth-path cascade; the missing 8-tree forest).
## Evidence
- Commits: 7c90339fd03fadcce964084f1a59b3065eb71f81, bb3c25d715b7727dde5ffe11b750b24954553fef, acd969df024c1b74e59a437f24001b7deac13831
- Tests: cargo test --profile ci --workspace --no-fail-fast (760 passed, 0 failed, 20 ignored; green receipt acd969df-unittest), cargo build --profile ci -p telperion-core --no-default-features, cargo test --profile ci -p telperion-core --no-default-features --lib (185 passed), node scripts/test-wasm.mjs (browser binding suite, all checks true), npx vitest run (112 passed), npx tsc --noEmit, node scripts/catalogue-check.mjs, node .flow/evidence/fn-100-the-field-reads-the-plan-not-placed/measure.mjs time|wood (R2, R3 rows under raw/)
- PRs: