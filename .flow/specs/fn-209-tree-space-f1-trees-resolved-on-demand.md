# Tree space F1: trees resolved on demand, axis by axis

## Goal & Context

Realtime generation with full fidelity near the camera and a fast zoom that always shows the right detail (owner, 2026-10-05: "we should be able to zoom in fast and show the right detail. It should be that performant"; never baked offline). A tree is a lazily evaluated fractal: draws are keyed by lineage (phase B), so any part can be resolved on its own and always comes out the same, and the closed form (phase A) gives an unresolved part's expected size, wood and leaves without growing it. Today the engine grows a whole tree through all its years at once (seconds for an 80-year oak or spruce, fn-198's cost table).

## Design (host, 2026-10-05)

- **Axis by axis.** An axis is grown alone from its lineage key, birth cycle, parent frame and the coarse fields; its laterals stay unresolved buds, each a closed-form stand-in, until the view needs them.
- **Coarse fields carry every coupling between axes.** Shade comes from the coarse crown's light grid (phase E's grid, built from resolved axes plus stand-ins); sag, girth and the carbon balance read the closed form's expected mass and light of unresolved subtrees, never resolved neighbours. A one-pass tree uses the same coupling, so a streamed tree and a one-pass tree are identical. The trade (exact feedback given up) is measured on the four species and judged on stills.
- **One error budget.** A subtree is resolved when its stand-in would be off by more than half a pixel on screen (the same error F3 surfaces wood at and `select.rs` picks leaves by). Prefetch follows the camera's motion.
- **Bar.** Detail within one frame of becoming visible, at any camera speed; exactness of streamed against one-pass.

## Requirements

- **R0:** A proposal before building: what in growth, presences, light, sag, girth, shedding and the carbon balance couples axes today (file:line), and each coupling's coarse-field form; the closed form's stand-in per bud; the cost of one axis's resolution.
- **R1:** Axis-by-axis growth with coarse fields; a test that streamed equals one-pass for every species at seeds 1 and 7.
- **R2:** The half-pixel resolution rule and prefetch; a fly-in from a whole tree to a twig at 5 cm with every frame's resolution time recorded.
- **R3:** The four species re-judged on stills under the coarse coupling, against their passed sheets.
- **R4:** Workspace gate; Codex review.

## Boundaries

Depends on fn-206 (the shared chain and its keys). Surfacing is F3; the speed levers are F2; integration is F4.
