---
satisfies: [R1, R2, R3, R4]
---
# fn-179-codominance-builds-on-its-own-and-a.1 Implement Codominance builds on its own and a refused bundle is split

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
`codominance` now builds on its own: `forkDivergence` defaults to 90 and `forkLean` to 26, with the reason stated in each row's note. The birch states its divergence of none and is unchanged. A refusal now names the fork rows. In the tuning loop, a bundle that fails to build at every strength is halved, and its halves are drawn one a round without a new proposal. A dial that fails alone is dropped for the revision. Curated references are read as stored: the view comes from the record's scale, and the image comes from `matched.refs` by the file name its url ends in, checked against its hash and copied into the run's cache.

Tier: (no conductor line supplied)

- R1: `suite::codominance::a_little_codominance_alone_builds_on_every_table` covers all 8 tables at seeds 1 and 7, and was red on the old defaults. The catalogue and birch pins are unchanged. The identity digests were re-pinned; with the defaults at 0 and the old refusal wording, all four matched fn-177's.
- R2: `tests/tuning_engine.rs` has 5 new tests. One is a regression built from the fn-62 refused bundle (`tests/fixtures/fn62-refused-bundle.json`). Halves rounds buy no proposal and do not count toward the runaway limit.
- R3: the `runner::cells` view test uses the shipped beech references. `tests/curated_references.rs` checks that the search does not run, that the copy is verified by hash, and that a mismatched file is refused.
- Beech replay: the Tune round was extended live (`--extend`: 8 Jev calls and 2 Opus sheet calls). The 27 answers no beech replay opens any more were removed, as the fixture README records.
- Gates: the workspace gate was green (1,088 tests, with a GPU), and so were `npm test` and `scripts/test-reviewers.py`.
- Follow-up: the proposal tape key hashes the whole effective wire. This is the third time it has cost a live re-record (see FRICTION.md).

stage: impl-review - ran (codex: round 1 fan-out NEEDS_WORK with 4 P2s, round 2 NEEDS_WORK with 1 left; the host reset the task review cycle once; round 3 SHIP)
## Evidence
- Commits: 56551041c1b6166a5ea824bf7586977c7223d129, 681fdb1454defe08f363a7126912c13875deda42, 71c94899236c3c7689ce0f95803d88459852f35a, 07063a991510744fe0b2dbf464ec0cb5780c4b56, c37c70aaf2c5cd1ca145ffe627978ac5660b1af7, e9e2db1334cc7923bad1e7f062daf2655b53a560, 25028de391c54c874d6d3592be315ad684bd3c62, 43add1dbec4a68314ec4eb1df6947598f54eb74f
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, python3 scripts/test-reviewers.py, baseline: none (the spec lists no Quick commands; the gate runs once at the end)
- PRs: