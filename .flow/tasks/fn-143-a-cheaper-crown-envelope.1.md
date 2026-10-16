---
satisfies: [R1, R2, R3, R4]
---
# fn-143-a-cheaper-crown-envelope.1 Implement A cheaper crown envelope

## Description
Replace the envelope's per-test `pow` pair with a cheaper evaluation of the same curve, CPU and WGSL.

## Acceptance
Spec R1-R4.

## Done summary
`radius_at` evaluates `exp(ln(-expm1(s ln p))/s)` (shoulder 1 exact); pendant search and `place.wgsl` match. Max departure 3.68e-7 of widest radius. Browser oak skeleton 59.5 -> 51.9 / 65.8 -> 58.1 ms (short of half); cull 6-20% faster. Stills and wood/leaf hashes byte-identical; skeleton digests re-pinned in `d40d0e4a` after the owner's verdict (2026-09-24: all 16 pairs identical).

## Evidence
- Commits: df3dc904f190e0d70d6b22050bd546d52c4b8c25, a5f31dc90a9ec33a1ec80faac8ac2d1f59379601, 4148f6504101774509f833b09da3e1b4f04cc651, 0f4496d2bd6e4033cf33d4eae764df59711cc35e, d40d0e4aa0e7473b57ef2a3b73ac858065110855
- Tests: cargo test --profile ci --workspace --no-fail-fast (935 passed); cargo test --profile ci -p telperion-core --test envelope_profile; cargo test --profile ci -p telperion-render --lib generation
- PRs:
