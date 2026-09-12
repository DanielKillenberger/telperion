---
satisfies: [R1, R6, R7]
---
# fn-27-coarse-shadow-casters-and-a-filtered.1 Runs in radius order, the run table, the wood caster prefix and the four row values

## Description
Emit wood runs in descending order of their largest radius, record a per-run table on the surface mesh, and make the sun's depth pass draw a prefix of the wood index buffer chosen by the row's caster threshold (R1 wood half, R7 run table). The four scene row values land here so the later tasks never touch the row file. This is the early proof point: the oak's shadow pass with the wood caster alone.

**Size:** M
**Files:** `crates/telperion-core/src/surface/paths.rs` (run table type), `crates/telperion-core/src/surface.rs` (sort runs before the emit loop, record the table), `crates/telperion-core/src/mesh.rs` (carry, validate), `crates/telperion-core/tests/surface.rs` and `tests/identity.rs` (re-pin), `crates/telperion-render/src/scene/row.rs` (four fields), `crates/telperion-render/src/submit.rs` and `wood.rs` (table upload, prefix), `crates/telperion-render/src/shadow.rs` and `shadow/fit.rs` (texel world size, threshold), `crates/telperion-render/src/lib.rs` (caster count accessor), `crates/telperion-render/tests/submit.rs`, `tests/shadow.rs`, `tests/conformance.rs`
**Touches:** [crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/**, crates/telperion-core/src/mesh.rs, crates/telperion-core/tests/**, crates/telperion-render/src/scene/row.rs, crates/telperion-render/src/submit.rs, crates/telperion-render/src/wood.rs, crates/telperion-render/src/shadow.rs, crates/telperion-render/src/shadow/**, crates/telperion-render/src/lib.rs, crates/telperion-render/tests/submit.rs, crates/telperion-render/tests/shadow.rs, crates/telperion-render/tests/conformance.rs]

### Approach
- Core: `surface/paths.rs:3-11` holds `Run{start,end,trunk}`; add the run table type there (first index, index count, largest radius) and keep `surface.rs` under 400 lines. Sort the runs by largest radius descending before the emit loop at `surface.rs:223-290`; the radius is `Sample.r` (`surface.rs:75-80`), largest over the run's samples. Record each run's index span as it is emitted. The trunk run sorts first by construction. Mesh validation (`mesh.rs:28-43`) rejects a table whose spans do not tile the index buffer exactly, naming the run.
- Identity pins: the permutation moves `tests/identity.rs` and `tests/surface.rs:70,106` once; re-pin with the reason in the test's doc comment. Counts must not change; assert that explicitly.
- Row: add `casterTexels` (0..8, default 1), `casterStride` (1..64, default 4), `shadowFilterTexels` (0..3, default 1), `shadowNormalOffset` (0..4, default 1) to the one `fields!` table at `scene/row.rs:17-64`; the struct, default, validate, to_json and parse all walk that table. Extend the row tests at `:184-223` with the four names and one out-of-range refusal each.
- Render: `submit.rs:52-80` uploads nothing new; the table stays host-side on the wood subject (`wood.rs:75-111`). The texel world size comes from the fit (`shadow/fit.rs:49-103`: the ortho extent over `RESOLUTION`). Threshold = casterTexels × texel size; the prefix is a binary search over the table's radii; `draw_shadow` (`wood.rs:136-144`) draws `0..prefix_index_count`. Expose the caster triangle count on the renderer for task 3 without touching `FrameStats` (`lib.rs:286-289` keeps the sun's pass out of it).
- Tests: `tests/shadow.rs:18-46` compares coverage shares; replace the absolute inequalities with a claim that survives the prefix: the default row's wood caster covers a non-zero share, casterTexels 0 covers at least as much as the default, and the bare view's share is below the whole view's. `tests/conformance.rs:246-285` adds `shadow.rs`, `wood.rs`, `common.wgsl` and `shadow.wgsl` to the no-anatomy list. `tests/submit.rs:107` gains a `fits` entry for the table.
- Measure once: the oak still with the timing record at the default row on the RTX 3080; record the shadow pass p50 beside fn-14's 1.813 ms and the caster triangle count beside 8,255,000. If the pass stays above about 0.9 ms, or limbs vanish from the ground shadow at any threshold that reaches it, stop and write the number.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/surface.rs:200-290` — the emit loop, samples and runs
- `crates/telperion-core/src/surface/paths.rs` — the run type and where the table belongs
- `crates/telperion-render/src/wood.rs:75-144` — submit and the shadow draw
- `crates/telperion-render/src/shadow/fit.rs:49-103` — the fit, for the texel world size
- `crates/telperion-render/src/scene/row.rs:17-64,112-177` — the field table, validation, parse

**Optional** (reference as needed):
- `crates/telperion-render/tests/shadow.rs` — the coverage test to rewrite
- `crates/telperion-core/tests/identity.rs` — the pins to move once

### Key context
- The prefix over a per-run draw list or a second index buffer is the spec's decision; do not add a buffer.
- fn-4 and fn-20 will edit the surface builder later; keep the sort and the table write next to the existing loop.
- Budget rules from CLAUDE.md bind: one timing run per change, at most four images viewed, never open receipts or frames.

## Acceptance
- [ ] Wood runs are emitted in descending largest-radius order and the surface mesh carries a run table of index span and radius that mesh validation checks tiles the index buffer exactly
- [ ] The sun's depth pass draws one prefix of the wood index buffer chosen by casterTexels times the map's texel world size; casterTexels 0 draws every run
- [ ] The four row values validate by name and range, parse from the headless flag and the page setter, and appear in the panel with no new browser line
- [ ] A caster triangle count is readable from the renderer without entering the frame stats
- [ ] Identity pins re-pinned once with the reason recorded; vertex, index and run counts unchanged; the clay pin holds at its existing tolerance
- [ ] The shadow coverage test states its new claim; the no-anatomy gate covers the shadow, wood and shared shader files
- [ ] Oak still at the default row on the RTX 3080 with the shadow pass p50 and caster count recorded beside fn-14's 1.813 ms and 8,255,000
- [ ] `cargo test --release --workspace`, fmt and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
