# The bark rings are a plan artifact

## Goal

After fn-102 the pipeline forked on `canopy.surface_contact > 0`: a family whose leaves sit on the bark ran the wood first and seated its leaves on the wood's float32 vertices, while every other family ran wood and leaves side by side. The fork existed because the rings the leaves need were produced inside the wood output, although by fn-102's own stage model they are a Plan artifact. This spec moves the rings into stage 3 as a float32 array the wood mesh takes ownership of, so every family runs one flow.

Owner quotes:
> "so leaves on bark is what forces a different flow?"
> "so couldn't we simplify by creating wood surface first always and then run plan and leaves in parallel?"
> "something about this doesn't make sense"
> On splitting the wood into a ring step and a mesh step: "hm ok i guess"

## Decisions

- Split `surface::build` along the parallel builder's existing phase boundary: the ring step (`surface/rings.rs`: rank pass, `sample_path`, `frames`, `record_edges`, `emit_run`) and the mesh step (`surface::faces`: indices, normals, run table, bounds); `Rings::into_mesh` moves buffers without copying. Rings live in a `OnceLock` shared by wood and seated leaves.
- Measured before ready (host, 2026-09-24, base `b8acb508`): the ring step is 31-34% of serial and 24-29% of parallel wood time, so the spruce's leaves start ~40 ms sooner (<1% of its build) — the value is the single flow. Zero-filling wood buffers cost 18-25 ms per parallel build, so each buffer is now written once by its producer.
- Rejected: wood first for every family, then plan and leaves (forced-serial builds give back fn-102's gains).
- Owner (2026-09-24): the core's first `unsafe` (`surface/parallel/unfilled.rs`, one `set_len` after proving the parts tile the buffer and were written in full) is accepted for R6's 10-19 ms; a scoped codex review found it sound.
- Owner (2026-09-24): an input failing both ways may now report a later run's position overflow before an earlier run's normal overflow; valid trees unaffected (pinned by `a_position_overflow_in_a_later_run_answers_before_an_earlier_normal`).

## Acceptance Criteria

- **R1:** No schedule fork on surface contact; every catalogue and in-work family runs wood mesh and leaves side by side — met.
- **R2:** Byte-identical to fn-102's merged base — met (16/16 mesh, 240/240 binding).
- **R3:** Wasm peak memory not higher — met (equal in all 240; the first candidate was 0.5-5.4 MB higher in 44 builds until four allocation-layout causes were fixed).
- **R4:** Whole-build medians no slower — met (-2.0% to +0.6%, noise); spruce 0.9-2.0% faster than fn-102's merged base, which had accepted +4.6%.
- **R5:** Net lines reported; seated fork and its tests removed — met (production net +341, tests +30).
- **R6:** No zero fill before a producer writes; parallel wood faster — met: oak 72.5 -> 53.2 ms, spruce 50.9 -> 36.9 ms, birch 42.8 -> 32.7 ms.

## Result

Landed in `7d51f3ab`; gate 932 passed. `docs/generation-limits-inventory.json` re-keyed for moved limit sites. Station preparation and retiring `AttachmentSurface::new`'s separate sweep belong to fn-125.

## Open

None.

## Friction

- Wasm peak regressions found only by trial: four rebuild-and-measure rounds of the 240-build pass (~25 min) -> proposed a peak-attributing allocator for the Wasm binding in evidence tooling.
