---
satisfies: [R1, R2, R3, R4]
---
# fn-46-young-shoots-take-their-own-colour.1 Implement Young shoots take their own colour

## Description
Add four material rows (`shootRed`, `shootGreen`, `shootBlue`, `shootRadius`): wood under the radius takes the shoot colour and blends to bark by twice it, before mottle, cavity and occlusion. Birch dark red-brown under 1 cm; beech olive grey-brown under 4 mm.

## Acceptance
- [x] R1 rows railed, wire, blend, harness, metadata; five neutral presets' stills hash identically.
- [x] R2 blend continuous on a synthetic cone; mottle still applies; native and browser share the shader.
- [x] R3 round 8 recorded (`round8-fn46` rows in `.flow/evidence/fn34/rounds.tsv`); 48-case protocol unchanged.
- [x] R4 `crates/telperion-render/tests/shoot_colour.rs` plus rail and wire tests.

## Done summary
Young wood takes its own colour below a radius. Round 8's birch read as a weeping birch; its leaf mass was too dark (35 vs 83), handed to fn-52. Oak hero frame 3.59 ms vs 3.60 ms. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Evidence
- Commits: 34bf9c2d (branch: dbd848a, 9f83930, ff9859e, 2ec33ef)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
