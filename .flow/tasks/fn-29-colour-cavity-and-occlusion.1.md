---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-29-colour-cavity-and-occlusion.1 Implement Colour, cavity and occlusion

## Description
Implement colour, cavity and crown occlusion as material rows (R1-R7).

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Nineteen validated, blended material rows (fissure, crest, bark mottle, cavity, blade mottle, margin, cuticle gloss, sky occlusion), preset values set against the references. Cavity derives from the relief field's face mean and amplitude (`bark_colour_range()`). Five rounds in one codex session; the eight R5 stills captured in round 3, the oak fissure softened in round 4, the crest/fissure map recentred in round 5 (owner: "pls go ahead", 2026-09-14). Oak native total p50 3.9823 ms against 3.8 ms; browser orbit 60 fps. Owner accepted the spec on 2026-09-14.

## Evidence
- Commits: 9fd53a6a119bc52b428a32dc4d1ecb61d917554b, 4c5bf29b33c3a809153ee44e35d0e62eba28aed7, 47b1af862b6e0f4deaba3d607b919f52e4a3eb94, 08ece339ac09563a99387a71dc54820e37ab30c8, a0fc5b101a23f199a64a25d36f6757ca958cbed9, 6d9b325f5ae22bfc7aff5c8dce9a2550875db7cc, 8ffb672767642ea5dfee4ac1858420ffcb11f033, baca4a66948a905781cbc431a1c0f9c2c10beeab, d7c44b30db87be3e348e8dd8712bf0e188ee513e, e9c4b7d7d1bee6025e6d16580a19b399e53d2b4e, 209cc53b988d56bc48eb7e13845624b553b009d8, aa17e5ebf275396f2907aa96c23d4ef7581acf2b, dc109e4734a219782fd26fcb101dde42cf52a150
- Tests: baseline: green (prior worker record at 2f33981, all five gates), cargo fmt --all -- --check (0 at a0fc5b1), cargo clippy --workspace --all-targets -- -D warnings (0 at a0fc5b1), ...
- PRs:
