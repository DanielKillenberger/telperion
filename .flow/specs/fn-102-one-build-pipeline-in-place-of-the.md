# One build pipeline in place of the copied chains

## Goal

The core's mature build recipe was written out more than once: `mesh::build` and the Wasm binding each spelled out growth, wood surface, leaf placement and cull, a test existed only to hold them equal, and the copies had already diverged (`mesh::grow` ran `clear_apical_twigs` and `clothe_leaf_bases`, the binding skipped both, latent until the date palm ships). The owner wants one pipeline with set stages, consumers reading what stages produce, each artifact built once and independent stages run in parallel. This spec does the structural half: one place that names stages, artifacts, order and scheduling, with every algorithm and random draw unchanged.

Owner quotes:
> "should we think about how to simplify this design in core and telperion over all facets? Make it really a clean building pipeline with set stages that all coherently make sense as part of the whole and also what consumes them."
> "this pipeline restructure's performance must be faster or at worst equal to current performance. Is there a way this streamlining can actually improve performance?"
> "do we need the cpu chain for leaves? beacuse if you're going to render millions of leaves you'll need a gpu anyway to not stutter like crazy..?"
> "ok makes sense so we just simplify cut the engine in 102?"
> (2026-09-23) "In the end this needs to be a high performing pipeline at every stage of it to be able to achieve the performance goals we have."
> (2026-09-23) "well yes the pipeline should create output needed for other stages exactly once that's the whole point"
> (2026-09-23) "yes and that should also be a principle of the pipeline. We build stages that later stages depend on only once. But if we have depending stages that are independent of eachother we run them in parallel."

## Decisions

- Five stages, one artifact each: Family, Skeleton (direct build growth incl. apical clearing and leaf-base clothing), Plan (leaf element, leaf plan, ring sweep for contact families), Outputs (wood, leaves, field, structure), Tree mesh. One function `telperion_core::pipeline` takes the wanted artifacts; `mesh::build`, `mesh::assemble` and the binding call it.
- Scheduling principles (owner, 2026-09-23): **once** (every needed artifact built exactly once, unrequested ones never), **parallel where independent** (native wood and leaves concurrently unless the family has surface contact), **correct serially** (Wasm serial; own random streams, fixed join order, earliest stage's error wins).
- The wood's float32 vertex array is the one ring store (host, 2026-09-23): a contact family (only the Norway spruce) builds wood first and placement reads rings in place; the first build's two-mode sharing (~260 lines) was replaced.
- Error order follows base (host, 2026-09-24): Skeleton, wood, Plan, leaves, field, structure.
- R3 exception (owner, 2026-09-23): the binding's output may change for rosette-frond/leaf-base families, where it now equals the mesh build.
- R4 rulings (owner): 2026-09-23, the whole build must be no slower; a concurrent stage's own wall time may rise (deferred as item J). 2026-09-24, accepted the spruce +4.6% CPU-path slowdown for ~70 MB less native peak memory (up to 75 MB less in Wasm); faster ring reads belong to fn-126.
- Direction (owner, 2026-09-23): consumers are GPU applications; the generator owns the plan's expansion into leaves and wood as a contract and the GPU becomes the drawn default — its own spec (fn-125). CPU hot-path ideas (cull early exit, cull while placing, forward cursor, parallel cull) went to fn-126; growth speed (attractor grid, Wasm SIMD) to fn-124. Design notes: `.flow/evidence/fn-102-one-build-pipeline-in-place-of-the/REVIEW-2026-09-23.md`.

## Acceptance Criteria

- **R1:** One pipeline function names the stages; mesh build and binding reach every stage only through it — met.
- **R2:** The chain-equality test is deleted, nothing replaces it — met.
- **R3:** Byte-identical to base for every catalogue and in-work preset at seeds 1 and 7, mesh and all binding output combinations, except the palm's binding fix — met (16/16 mesh; 7 families x 30 binding builds identical, palm changed as allowed).
- **R4:** Stage and whole-build medians recorded; whole build no slower; peak Wasm memory not higher — met under the owner's rulings (spruce exception accepted).
- **R5:** Binding exports, request shape and metadata unchanged; tests pass unedited — met.
- **R6:** Specimen view and GPU preparation build and pass unedited — met.
- **R7:** A contact family gets one ring sweep and one element per request, counted by test — met.
- **R8:** Serial and concurrent schedules byte-identical for every preset; two failing stages return the earlier error — met.

## Result

Native median of five, seed 1, base -> candidate (shared machine, load 11-17): ordinary 155 -> 135 ms (-13.4%), oak 483 -> 414 (-14.2%), spruce 4,720 -> 4,939 (+4.6%, placement +5.5%, peak RSS 360 -> 290 MB), birch 2,695 -> 2,601 (-3.5%), telperion 1,123 -> 1,036 (-7.7%), laurelin 449 -> 432 (-3.9%). Spruce Wasm surface+foliage peak 360 -> 285 MB. Production diff +786/-269 lines. Gate 930 passed; `npm test` 124/124; codex review one pass, one finding (R8 error precedence under concurrency) fixed with a red-to-green test.

## Open

- J: the concurrent wood stage's own wall time rises (+12-15 ms ordinary, +54-67 ms spruce); profile on an idle machine. K: thread oversubscription when the wood starts 8 surface workers beside leaves and field. Both deferred.
- The native family wire encodes `maxInstances` as 64-bit `usize::MAX`, which wasm32 refuses; needs a width-independent "no limit" encoding.

## Friction

- `maxInstances` wire not round-trippable native/Wasm (~5 min) -> follow-up above.
- dcg hook refused variable redirects and `git checkout <ref> -- <path>` (several times) -> local setup, reported only.
- Finding a flat Wasm peak layout took three designs and four probe rebuilds (~60 min), and a plan-before-wood reorder raised the oak's peak 10 MB found only by the full R3 pass -> proposed a per-stage Wasm memory probe in the binding metadata.
- Spruce ring-read A/B needed hand env switches and rebuilds (~20 min) -> proposed a placement microbenchmark over a fixed skeleton.
- `generation_limit_guard` failed late on moved loop sites (one extra gate run) -> run it in the per-change check.
- Evidence tools went stale against HEAD; the base worktree vanished from the scratchpad -> keep base binaries under ignored `raw/`; compile tools as example targets.

## Evidence

- `.flow/evidence/fn-102-one-build-pipeline-in-place-of-the/REVIEW-2026-09-23.md` (owner-reviewed design notes and deferred optimisation items A-K)
