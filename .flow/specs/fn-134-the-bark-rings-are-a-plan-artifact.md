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

- `surface::build` splits into a ring step (path sampling, frames, the angular profile, the float32 ring points and each node's ring offsets, today's `WoodWithContacts` edges) and a mesh step (indices, normals, coordinates, bounds, dropped triangles) that consumes the ring array as its `positions` without copying. [inferred]
- Leaf seating reads the ring step's array through the in-place view fn-102 built (`AttachmentSurface::on_wood`), for every family with contact. [inferred]
- `pipeline::outputs` loses the `seated` fork: rings join the Plan stage when wood or seated leaves need them; wood mesh and leaves run side by side whenever both are wanted. [inferred]
- The Linux parallel wood builder (`surface/parallel.rs`) keeps its workers for both steps or for the one that dominates, decided by measurement. [inferred]
- fn-125 then points station preparation at the same ring array and deletes `AttachmentSurface::new`'s separate sweep. [inferred]

## Acceptance Criteria

- **R1:** The pipeline has no schedule fork on surface contact; one test proves every catalogue and IN_WORK family runs wood mesh and leaves side by side on native when both are wanted. [inferred]
- **R2:** Output is byte-identical to fn-102's merged base for every catalogue and IN_WORK preset at seeds 1 and 7, the mesh build and every binding output combination. [inferred]
- **R3:** Wasm peak linear memory for every output combination is not higher than the base's. [inferred]
- **R4:** The ring step's share of wood time is recorded for the oak, spruce and birch before the split is designed; whole-build medians of five are no slower than base for any preset, and the spruce's result is reported against fn-102's accepted +4.6%. [inferred]
- **R5:** Net production lines are reported; the `seated` fork and its tests are removed, not kept beside the new flow. [inferred]

## Boundaries

- No change to leaf placement, the cull or the ring geometry itself. [inferred]
- Station preparation and the separate sweep belong to fn-125. [inferred]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Alternative considered: wood first for every family, then plan and leaves. Rejected: forced-serial builds were only 0.1 to 1.7% faster than base, so it gives back fn-102's gains (oak −14.2%, ordinary −13.4%, telperion −7.7%). [inferred]
- Depends on fn-102. fn-125's "retire the separate ring sweep" builds on this spec. [inferred]

## Open before ready

- The ring step's share of wood time is unmeasured; it decides how much the spruce gains and how the parallel builder splits.
