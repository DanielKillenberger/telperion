# Tree space F1: trees resolved on demand, axis by axis

## Goal & Context

Realtime generation with full fidelity near the camera and a fast zoom that always shows the right detail (owner, 2026-10-05: "we should be able to zoom in fast and show the right detail. It should be that performant"; never baked offline). A tree is a lazily evaluated fractal: draws are keyed by lineage (phase B), so any part can be resolved on its own and always comes out the same, and the closed form (phase A) gives an unresolved part's expected size, wood and leaves without growing it. Today the engine grows a whole tree through all its years at once (seconds for an 80-year oak or spruce, fn-198's cost table).

## Design (host, 2026-10-05)

- **Axis by axis.** An axis is grown alone from its lineage key, birth cycle, parent frame and the coarse fields; its laterals stay unresolved buds, each a closed-form stand-in, until the view needs them.
- **Coarse fields carry every coupling between axes.** Shade comes from the coarse crown's light grid (phase E's grid, built from resolved axes plus stand-ins); sag, girth and the carbon balance read the closed form's expected mass and light of unresolved subtrees, never resolved neighbours. A one-pass tree uses the same coupling, so a streamed tree and a one-pass tree are identical. The trade (exact feedback given up) is measured on the four species and judged on stills.
- **One error budget.** A subtree is resolved when its stand-in would be off by more than half a pixel on screen (the same error F3 surfaces wood at and `select.rs` picks leaves by). Prefetch follows the camera's motion.
- **Bar.** Detail within one frame of becoming visible, at any camera speed; exactness of streamed against one-pass.

## Design: host decisions 1 to 7 on the R0 proposal (2026-10-05)

The proposal is accepted: the chain as the unit of resolution, the read rule, the coupling tree, and the four closed-form additions (expected section, mass, envelope, extinction curve), each with an oracle test first.

1. **Error metric (Q1).** Geometry is judged by positional half-pixel error: wood and a stand-in's envelope by their projected deviation. Foliage density (a stand-in's leaf volume) is judged on stills by the host, against the resolved subtree at the same view. No numeric proxy for foliage looks.
2. **The first-view bar (Q2): two bars.**
   - (a) Along continuous camera paths with prefetch: detail within one frame.
   - (b) When a tree first appears or the camera cuts: progressive refinement. Frame 1 draws stand-ins sized by the closed form, and full detail lands within a stated budget (0.1 to 0.35 s on 32 cores), with no frame over the per-frame resolution budget.
   - Both are measured. The owner is told of (b).
3. **The coupling threshold is no switch (Q3).** A bud's membership in the coupling pass is a smooth weight w(expected extent / cell), rising from 0 to 1 over a band of about 0.5 to 2 cells. The coarse fields read the exact subtree times w plus its stand-in times (1 - w). It is walk-tested within the bound of 30, red first on the hard threshold.
4. **Conservation (Q4).** Allocation stays exactly conserved within the coupling pass; fine subtrees take their expected share. The drift per age is measured and recorded, and accepted under 2 % of an age's total; above that it is reported.
5. **Light response (Q5).** The closed form takes the coarse light at the bud as an input: an expected subtree under a constant light I, neutral at I = 1, so shaded stand-ins are expected under their light. If that is a deep change to the closed form, stop and report its size before building.
6. **The pipeline door (Q6).** `refine(&Structure, view)` sits in the executor interface, beside `expand`, as the streamed door the GPU executor uses. `pipeline::build` stays the one-pass build and calls refine with a view that resolves everything, so one-pass and streamed share one code path. `docs/pipeline.md` is updated with it.
7. **One light read per fine subtree (Q7)** is accepted. Its p95 of 5 to 9 % is measured on stills of the oak.

**Build order:**
1. The stand-in tables with their oracle tests.
2. Beech, spruce and palm under the read rule, unchanged topology as the test.
3. The oak's coupling pass, with decision 3's smooth weight.
4. Streamed equals one-pass.
5. R2 (the fly-in and the first view) and R3 (stills of the four species).

The worker stops after step 2 with a report, and at any design question.

## Requirements

- **R0:** A proposal before building: what in growth, presences, light, sag, girth, shedding and the carbon balance couples axes today (file:line), and each coupling's coarse-field form; the closed form's stand-in per bud; the cost of one axis's resolution.
- **R1:** Axis-by-axis growth with coarse fields; a test that streamed equals one-pass for every species at seeds 1 and 7.
- **R2:** The half-pixel resolution rule and prefetch; a fly-in from a whole tree to a twig at 5 cm with every frame's resolution time recorded.
- **R3:** The four species re-judged on stills under the coarse coupling, against their passed sheets.
- **R4:** Workspace gate; Codex review.

## Boundaries

Depends on fn-206 (the shared chain and its keys). Surfacing is F3; the speed levers are F2; integration is F4.
