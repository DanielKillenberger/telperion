---
satisfies: [R1, R2, R3, R4]
---
# fn-246-the-harness-holds-60-fps-on-every.1 Thin dense crowns by screen size, cache sun work, draw on change, cheaper wood

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The harness now draws only when the picture changes. It reuses the sun's wood and shadow map until what the sun sees changes. It thins leaves and florets under 2 px with their area kept, draws florets under 4 px as one card, and shades the bark prefiltered to the pixel, one cell per pixel. Natively at 1920x1080 with 4x MSAA, every shipped species holds 60 fps whole and close except the Two Trees: Telperion 50 and 34, Laurelin 56 and 52, against 8 and 13 before (FPS.md, "R3 final").

Decisions, all recorded in FPS.md:
- The prefiltered bark is the owner's (b). Its distance to the 4x truth is the accepted picture. The bark tests are re-pinned to its measured value plus 0.5 mean, with both shaders' truth distances beside each bound.
- 2 px thinning is fitted on `examples/crown_truth`.
- The floret card below 4 px is the owner's, on the labelled comparison https://claude.ai/artifact/8WwaynDsif2JekPvHDDtDi, past the crown bound (+4.43). `tests/thinning.rs` pins it at 30%.
- The Two Trees stay below 60 for now. Follow-ups after the release: cluster cards, and texture bark (owner's (c)).

Tests that carry the acceptance:
- `tests/sun_cache.rs`: the cache across views, stride, sun and a new tree; the map and the frame byte-identical.
- `sun.rs` unit tests: a casters-only change redraws the map over the same wood.
- `tests/timing.rs`: a cached map counts as zero and is flagged.
- `redraw.rs` unit tests and `src/browser/render.test.ts`: an idle canvas, a resize, a ground-only edit, a move during a frame in flight, an unchanged-size resize.
- `tests/thinning.rs`: leaves and florets.
- `card.rs` unit tests.
- `bark_resolution.rs`, `bark_distance.rs`: re-pinned.

Probes: `examples/frames.rs`, `bark_truth.rs`, `crown_truth.rs`.

Gate: run package by package, the render package under the GPU lock. All green except the load-sensitive cold-compile test, which passed when run alone (FRICTION.md). npm test: 154 passed.

stage: impl-review - ran [codex fan-out NEEDS_WORK (4 findings, all fixed in a30b1fa4)..codex re-review SHIP]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5a26e075b1e4e2b1574b46fbbfac0dfa66c20d3d, 203938e7536698512dc35cc81f85b04f065357b9, 177dbe4c2b3fe5aa58ea28161594c9ff0aed4419, 707d4f4b92b42832b82ab52ab022eeb6d75b3d1f, fccb20c6e1904b66953a85d280e5c92aa76d936f, 0ce78c7c7316357d0f988d0eb95d613b5d1e5495, 5194550c578f707d9dbc5d24441ceffd185877a7, b6735ca43262160077451caff6e0a300994cebc7, ab12d645e79ac0f808790692eabee27b43109f4b, 206856225f2b5bd29a06e3f79ee69722ac907564, 5fd63ef8c7ded0092bcea627472579984637d856, 3fc0eec6585baa855ed1bc8c87d4724b590510e6, e4e0f290e55ca7b9a17b3ff192270c890171da45, 00832b02a7dcfac27f7b7c207471dd4540d92b4e, 686289a449efaae2677e485b661aa26063f986b1, 47018e5f6252472689e7886c4366be75da4d0329, cf8a5bc3138cc2cfcad1af1aae97ec505562525b, a8ae7993e03ff5acd771bdd769db6e40010a165e, 85539e89564eaeabe0426263f320ff15f13c4a0b, 362852821af9126bdc1e7db1e76f0f7ac8c95150, a2797a85009bc5b72e8b7873e7736bd2b56632f9, d1cca32cdb1624fe300c110e7d0c09ac43093781, 8fa0b1d0b1311d000693d86c6ce952040096b66c, a30b1fa49f70682ccf54a218c862a2812354bc6a
- Tests: cargo test --profile ci -p <each package> --no-fail-fast (render under the GPU lock), npm test, cargo run --profile ci -p telperion-render --example frames -- <preset> full whole close
- PRs: