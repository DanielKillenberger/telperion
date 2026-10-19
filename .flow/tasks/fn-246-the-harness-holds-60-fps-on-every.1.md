---
satisfies: [R1, R2, R3, R4]
---
# fn-246-the-harness-holds-60-fps-on-every.1 Thin dense crowns by screen size, cache sun work, draw on change, cheaper wood

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
The harness draws only when the picture changes, reuses the sun's wood and shadow map until what the sun sees changes, thins leaves and florets under 2 px with area kept, draws florets under 4 px as one card, and shades the bark prefiltered to the pixel. Natively at 1920x1080 4x MSAA every shipped species holds 60 fps whole and close except the Two Trees: Telperion 50 / 34, Laurelin 56 / 52 (8 / 13 before). Decisions and tables in FPS.md: the prefiltered bark is the owner's (b), bark tests re-pinned to it plus 0.5 mean; 2 px fitted on `examples/crown_truth`; the floret card is the owner's on the labelled comparison; the Two Trees stay below 60 for now.
Tests: `tests/sun_cache.rs`, `sun.rs`, `tests/timing.rs`, `redraw.rs`, `src/browser/render.test.ts`, `tests/thinning.rs`, `card.rs`, re-pinned `bark_resolution.rs` and `bark_distance.rs`. Gate green except the load-sensitive cold-compile test, which passed alone (FRICTION.md); npm test 154. Impl-review SHIP after 4 fixes.

## Evidence
- Commits: 5a26e075b1e4e2b1574b46fbbfac0dfa66c20d3d, 203938e7536698512dc35cc81f85b04f065357b9, 177dbe4c2b3fe5aa58ea28161594c9ff0aed4419, 707d4f4b92b42832b82ab52ab022eeb6d75b3d1f, fccb20c6e1904b66953a85d280e5c92aa76d936f, 0ce78c7c7316357d0f988d0eb95d613b5d1e5495, 5194550c578f707d9dbc5d24441ceffd185877a7, b6735ca43262160077451caff6e0a300994cebc7, ab12d645e79ac0f808790692eabee27b43109f4b, 206856225f2b5bd29a06e3f79ee69722ac907564, 5fd63ef8c7ded0092bcea627472579984637d856, 3fc0eec6585baa855ed1bc8c87d4724b590510e6, e4e0f290e55ca7b9a17b3ff192270c890171da45, 00832b02a7dcfac27f7b7c207471dd4540d92b4e, 686289a449efaae2677e485b661aa26063f986b1, 47018e5f6252472689e7886c4366be75da4d0329, cf8a5bc3138cc2cfcad1af1aae97ec505562525b, a8ae7993e03ff5acd771bdd769db6e40010a165e, 85539e89564eaeabe0426263f320ff15f13c4a0b, 362852821af9126bdc1e7db1e76f0f7ac8c95150, a2797a85009bc5b72e8b7873e7736bd2b56632f9, d1cca32cdb1624fe300c110e7d0c09ac43093781, 8fa0b1d0b1311d000693d86c6ce952040096b66c, a30b1fa49f70682ccf54a218c862a2812354bc6a
- Tests: cargo test --profile ci -p <each package> --no-fail-fast (render under the GPU lock), npm test, cargo run --profile ci -p telperion-render --example frames -- <preset> full whole close
- PRs:
