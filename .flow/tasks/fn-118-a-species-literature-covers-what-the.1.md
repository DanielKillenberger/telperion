---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-118-a-species-literature-covers-what-the.1 Implement fn-118-a-species-literature-covers-what-the

## Description
Enforce a per-growth-form requirements table at admission and quality, stopping the run with NEEDS_HUMAN when literature falls short.

## Acceptance
Spec R1-R5 (R4 moved to the host in fn-80).

## Done summary
`species-requirements.json` (broadleaf, conifer, palm; size fields and six appearance traits mapped to `MaterialParams` ranges). `manifest::validate` checks coverage at `schema_version` 2; `quality`/`select` file owner-only `requirements-unmet` (no `lower-bar`); the conductor routes it to `AwaitOwner` and the CLI prints `NEEDS_HUMAN`. Appearance copied into `profiles[0].appearance` and an article block. Gate green (944 passed).

## Evidence
- Commits: 0118ca37c5185af4f8061d580133ead1ce014f76, 5c4f0a1f0b34edd06474d6f1911c7704a60eedcc, 281b80bafc0993fc855806e02d20cec8567da398
- Tests: cargo test --profile ci --workspace --no-fail-fast (944 passed); npx vitest run scripts/catalogue-docs.test.mjs; npm run catalogue:check
- PRs:
