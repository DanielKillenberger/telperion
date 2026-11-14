# fn-355-a-year-is-a-state-the-grower-reads-no.1 The grower reads no horizon: windows by lifespan, every sleeper recorded, growth goes on

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The grower reads no horizon: every draw's window looks LOOK = 40 cycles on, a sleeper is recorded when it can wake, every year is laid out and lit, and growth goes on past its first request. R1 to R3 hold to the bit, every walk holds its degree and vanishing bounds, and summed length at the asked age is within 6.2% of fn-354's head. Report: .flow/evidence/fn-355-a-year-is-a-state-the-grower-reads-no/TASK1-DONE.md.

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing with fn-354)

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5)
baseline: green via handoff (765 of 766 at d0728b827, the identity test the one red)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8bf6175e7f868cf1b4a42e503f0c6efd3add362d, 9c4602d7dcd638bb631d02462a142811ee1e13f8, b2b45f3c6341fe73db4cc4a324bc6d77209118dc, 4ca638dfd8d403f17cfdaa6c51ef612d090b76bd, 27a6be172723ea830a9407f58ee6d83b1d389d46, 24bde7682257d06b52244b601ee2530ea9f193a5, 958e0b28cb15b2afddd232013ba656809f293899, 60052c3a5dcc302e67107496ee2d7f3f04e15827
- Tests: telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 (long walk and largest tree excluded): 768 of 769, the pin repinned and rerun green, telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --test walks -- --nocapture: 11 passed, cargo test --profile ci -p telperion-core --test generation_limit_guard; cargo test --profile ci -p telperion-render --lib compile_size: green, npm run wasm:build && npm test: vitest 218 of 218
- PRs: