# The bark rings are a plan artifact

## Conversation Evidence

> user: "so leaves on bark is what forces a different flow?"
> user: "so couldn't we simplify by creating wood surface first always and then run plan and leaves in parallel?"
> user: "something about this doesn't make sense"
> assistant: "The wood stage is secretly doing plan-stage work. [...] Split the wood surface: stage 3 computes the 32-bit ring points, stage 4 adds triangles, normals and texture coordinates around them while the leaves read the same rings."
> user: "hm ok i guess"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] from fn-102's code and evidence -->

After fn-102 the pipeline forks on `canopy.surface_contact > 0`: a family whose leaves sit on the bark runs the wood surface first and places its leaves on the wood's float32 vertices in place, while every other family runs wood and leaves side by side (`pipeline::outputs`, the `seated` flag). The fork exists because the rings the leaves need are produced inside the wood output. By fn-102's own stage model they are a Plan artifact: an input two outputs read. fn-102 first built them as a separate float64 sweep and dropped that because it raised Wasm peak memory by 7 to 24 MB. [inferred]

This spec moves the rings into stage 3 as a float32 array that the wood mesh takes ownership of, so every family runs one flow: plan (element, rings) then wood mesh and leaves side by side. [paraphrase]

## Architecture & Data Models

- `surface::build` splits along the boundary the Linux parallel builder already has (`surface/parallel.rs:71-90`). The ring step is the ranking pass, `sample_path`, `frames`, `record_edges` and `emit_run` (positions and coords), which is parallel phase 1 (`parallel.rs:192-247`). The mesh step is indices, normals, the run table and bounds, which is parallel phase 2 (`parallel.rs:249-314`); it already borrows `positions` in place and needs only `segments` and each run's `base`, `rings`, `first_index` and `index_count` (`parallel.rs:8-15, 277-283`). The ring artifact owns positions, coords, the edges, the run descriptors, each run's largest radius (run table, `build.rs:169, 321`) and the per-ring frames the serial mesh step reads for the normal of a vertex no triangle touches (`build.rs:308, 387-401`); the mesh step takes positions and coords without copying. [inferred, read 2026-09-24]
- Measured on `b8acb508` (`.flow/evidence/fn-134-the-bark-rings-are-a-plan-artifact/RING-SHARE.md`, seed 1, median of 5, load 0.6): the ring step is 31 to 34% of serial ring-plus-mesh time and 24 to 29% of parallel wall time (oak 14.4 of 66.5 ms, spruce 11.8 of 53.9 ms, birch 6.9 of 40.3 ms). The spruce's leaves could therefore start about 40 ms sooner, under 1% of its 4.9 s build: this spec's value is the single flow, not speed. [inferred, measured]
- On the parallel path, zero-filling the four output buffers costs 18 to 25 ms per build, longer than the whole ring phase and longer than phase 2 (oak: 17.8 ms of normals and indices fill against 12.5 ms of phase 2). The split reallocates these buffers anyway, so each is written once by its producer without a prior zero fill. [inferred, measured]
- Leaf seating reads the ring step's array through the in-place view fn-102 built (`AttachmentSurface::on_wood`), for every family with contact. [inferred]
- `pipeline::outputs` loses the `seated` fork: rings join the Plan stage when wood or seated leaves need them; wood mesh and leaves run side by side whenever both are wanted. [inferred]
- The Linux parallel wood builder (`surface/parallel.rs`) keeps its workers for both steps or for the one that dominates, decided by measurement. [inferred]
- fn-125 then points station preparation at the same ring array and deletes `AttachmentSurface::new`'s separate sweep. [inferred]

## Acceptance Criteria

- **R1:** The pipeline has no schedule fork on surface contact; one test proves every catalogue and IN_WORK family runs wood mesh and leaves side by side on native when both are wanted. [inferred]
- **R2:** Output is byte-identical to fn-102's merged base for every catalogue and IN_WORK preset at seeds 1 and 7, the mesh build and every binding output combination. [inferred]
- **R3:** Wasm peak linear memory for every output combination is not higher than the base's. [inferred]
- **R4:** Whole-build medians of five are no slower than base for any preset, and the spruce's result is reported against fn-102's accepted +4.6%. [inferred]
- **R6:** No wood output buffer is zero-filled before its producer writes it; the parallel wood build's wall time for the oak, spruce and birch is recorded on base and candidate, and the fill cost (18 to 25 ms on base) is gone from it. [inferred, measured]
- **R5:** Net production lines are reported; the `seated` fork and its tests are removed, not kept beside the new flow. [inferred]

## Boundaries

- No change to leaf placement, the cull or the ring geometry itself. [inferred]
- Station preparation and the separate sweep belong to fn-125. [inferred]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Alternative considered: wood first for every family, then plan and leaves. Rejected: forced-serial builds were only 0.1 to 1.7% faster than base, so it gives back fn-102's gains (oak −14.2%, ordinary −13.4%, telperion −7.7%). [inferred]
- Owner, 2026-09-24: the core's first `unsafe` (`surface/parallel/unfilled.rs`, one `set_len` after proving every worker filled its disjoint range) is accepted for R6's 10 to 19 ms per parallel wood build; a scoped codex review found it sound on worker errors, panics, refused spawns and arithmetic. [user]
- Owner, 2026-09-24: because every run's rings are swept before any is shaded, an input that fails both ways can now report a later run's float32 position overflow before an earlier run's normal overflow; valid trees are unaffected. Pinned by `a_position_overflow_in_a_later_run_answers_before_an_earlier_normal` (red on base, green here). [user]
- Depends on fn-102. fn-125's "retire the separate ring sweep" builds on this spec. [inferred]

## Resolved before ready (host, 2026-09-24)

- The ring step's share of wood time: measured, see Architecture and `RING-SHARE.md`. It is a third of serial wood work, so the gain for seated families is small and the reason for this spec is the single flow; the zero-fill finding is the speed this spec can add.
