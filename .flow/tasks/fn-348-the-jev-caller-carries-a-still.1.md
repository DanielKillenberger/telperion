# fn-348-the-jev-caller-carries-a-still.1 Carry image assets through the shared Jev caller

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
`jev ask --assets <file>` sends images through the shared caller as the API's image assets. A reference string in the state or questions names each image, and requests that carry one get the `TypeSafe-Danger-Zone-Images: true` header. Images outside the documented limits are refused by name before any key load or call. The ledger records each asset's name, path and SHA-256, never its bytes. A call without `--assets` sends the bytes it always sent, pinned by `a_call_without_assets_sends_the_request_it_always_sent`. One live call on the fn-346 still answered `in_leaf` at 0.90 (`ask-still:18e6d6556451f68d-3cf88c-0`; RESULT.md). docs/typesafe.md gains "Asking a question of a still".

Tests: tests/caller_assets.rs (request carries the asset, ledger holds the digest; limits refused by name: too wide, too small, too tall JPEG, not an image, over 16 MiB); the jev.rs unit test `ask_refuses_an_oversized_asset_by_name_before_the_key_or_a_call`; the no-assets pin in tests/caller.rs.

Decision for the host: the ledger identity folds the asset digests into the state checksum when assets are present, so the same state with a different picture keeps a distinct identity. Without assets the identity is unchanged.

Follow-ups: (1) all three reviewers raised the same P2: a trailing `--assets` with no value is read as absent by the shared `flag()` helper, which already treats every trailing flag this way. (2) The 32 MiB request and all-images totals are left to the API's 413. (3) FRICTION.md: a fresh worktree has no gate receipts, so a jev-only change ran every suite (about 29 minutes).

Tier: single minted implicit task (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws, SHIP; rid 965a1495824c44babbeb0dbdc4648976)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: c2dd9d20ee49be545fb95b7bc3f65f78d93cb9c1, d0ab47f5c3db10e3d9f01de356d3cd7522bef47e
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (green: 1547 passed, test run 1218 s of 1560 s), cargo test --profile ci -p telperion-core --test generation_limit_guard && cargo test --profile ci -p telperion-render --lib compile_size, cargo test --profile ci -p telperion-jev --test caller --test caller_assets --bin jev
- PRs: