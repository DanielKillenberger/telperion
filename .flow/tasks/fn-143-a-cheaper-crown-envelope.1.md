---
satisfies: [R1, R2, R3, R4]
---
# fn-143-a-cheaper-crown-envelope.1 Implement A cheaper crown envelope

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Envelope::radius_at evaluates the crown quadrant (1 - p^s)^(1/s) as exp(ln(-expm1(s ln p))/s) from pinned libm, with shoulder 1 kept as the exact line; the pendant lower-surface search and place.wgsl use the same form. Measured max departure from the pow form is 3.68e-7 of the widest radius (bound 1e-6, tests/envelope_profile.rs). Browser oak skeleton 59.5 -> 51.9 ms and 65.8 -> 58.1 ms, short of half (34.9 / 38.3); the next limiting stage is the local advance loop body (fn-124 attribution, not re-profiled). Cull: birch -7 to -9%, spruce -6 to -8%, oak -18 to -20%, beech -20%. All 16 hero stills and every wood/leaf hash byte-identical; skeleton positions move by at most 5.5e-10 m with identical topology. Pin re-baseline is the separate final commit d40d0e4a, awaiting the owner's visual verdict.

Tier: session (actual model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)

R3: owner visual verdict on 2026-09-24: all 16 still pairs identical, no regression; the re-pin commit d40d0e4a is kept.
stage: impl-review - ran (codex, host-dispatched, one scoped pass on 04b0851a..4148f650: no defects)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: df3dc904f190e0d70d6b22050bd546d52c4b8c25, a5f31dc90a9ec33a1ec80faac8ac2d1f59379601, 4148f6504101774509f833b09da3e1b4f04cc651, 0f4496d2bd6e4033cf33d4eae764df59711cc35e, d40d0e4aa0e7473b57ef2a3b73ac858065110855
- Tests: cargo test --profile ci --workspace --no-fail-fast (final HEAD d40d0e4a: rc 0, 935 passed), cargo test --profile ci -p telperion-core --test envelope_profile (max departure 3.68e-7 of widest radius, bound 1e-6), cargo test --profile ci -p telperion-render --lib generation (GPU vs CPU cull count agrees), baseline: none (spec lists no Quick commands; the first gate run before the re-pin failed only on pinned skeleton digests and the limit inventory)
- PRs: