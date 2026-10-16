---
satisfies: [R1, R2, R3, R4]
---
# fn-179-codominance-builds-on-its-own-and-a.1 Implement Codominance builds on its own and a refused bundle is split

## Description
Give the fork rows defaults that build, split a bundle that fails to build, and read curated references as stored.

## Acceptance
- R1-R4 of the spec.

## Done summary
`forkDivergence` defaults to 90 and `forkLean` to 26 (reasons in the row notes); the birch is unchanged and the refusal names the fork rows. A bundle failing at every strength is halved and its halves drawn one a round; a dial failing alone is dropped. Curated references resolve view from scale and image from `matched.refs` by hash. Beech tape extended live (8 Jev, 2 sheet calls), 27 dead answers removed. Gates green (1,088 tests with GPU). Codex SHIP after a host reset.

## Evidence
- Commits: 56551041c1b6166a5ea824bf7586977c7223d129, 681fdb1454defe08f363a7126912c13875deda42, 71c94899236c3c7689ce0f95803d88459852f35a, 07063a991510744fe0b2dbf464ec0cb5780c4b56, c37c70aaf2c5cd1ca145ffe627978ac5660b1af7, e9e2db1334cc7923bad1e7f062daf2655b53a560, 25028de391c54c874d6d3592be315ad684bd3c62, 43add1dbec4a68314ec4eb1df6947598f54eb74f
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, python3 scripts/test-reviewers.py
- PRs:
