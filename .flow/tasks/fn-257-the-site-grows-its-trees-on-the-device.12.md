---
satisfies: [R8, R8c]
---
# fn-257-the-site-grows-its-trees-on-the-device.12 Probe: the field splatted from the drawn stage's output, timed and compared on the spruce

## Description
The owner asked for a quick probe before anything is deleted (spec, "The field is a read-out of the build"). Measurement only, in a test module: nothing in the shipped path changes.

**Size:** S
**Files:** a probe test beside `crates/telperion-render/src/generation/resident/field_probe_tests.rs`, a probe WGSL kernel, `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/PROBE-R8.md`
**Touches:** [crates/telperion-render/src/generation/resident/**, .flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/**]

### Approach
- Inputs: exactly what the drawn stage hands its consumers (the renderer reads the same): the swept nodes (position, parent, start and end radius, flags) and the station segments (`stations.wgsl`: endpoints, count, and `owns`, a node id). No field-primitive format, no index.
- Derivations the splat still needs, which today live in `field_build.wgsl` and must survive as passes reading those records directly (no stored intermediate format): each node's limb system (`widest`/`carrier`/`opens`/`order`/`own`/`inherit` pointer jumps and the dense naming), a segment's leaf reach (`Expansion::leaf_reach`), and a frond's leaflets as oriented boxes in place of the whole-frond segment (`LEAFLETS`). Name each, keep its rule, and time it in the probe; say which could fold into the splat pass itself.
- One device pass over those records: each record writes the cells it touches in the spec's grid ("The grid": voxelize's definition, n = 24, 36, 48): wood flag, thickest radius (atomic max on the f32 bits), leaf estimate (fixed-point atomic add of the share inside the cell), owning limb (the system holding the most leaves in the cell: a per-cell tally, or a second pass; report which and why).
- The cell rule matches the CPU reference's per-cube `query` (`telperion_core::pipeline::field` `query(centre, half)`: wood is point-in-capsule at `half × √3` inflation, the radius the max of start and end, leaves `Σ count × share_inside`). Compare per cell against it, at the same grid's cells, with `field_tests.rs`'s `compare` / `within` on the spruce (seed 1), the oak (seed 1) and the date palm (seed 1, fronds).
- Task 11's measurements (FIELD-TIME.md, if it recorded the unread stages) are reused, not repeated.
- Time, warm, paired against today's `grow_field` + 36³ `query_field` (16 runs interleaved): the splat plus readback, and the stages before it. List which stages the field path runs whose outputs neither the splat nor the stations read (sweep's rings, curve, normals, bark; boots; trunk; dress extras), with their time: the spec's "also check whether the build path runs work the drawing needs but the field doesn't".
- Stop and report in PROBE-R8.md: agreement per measure against the tolerances, times, the unread stages. If the splat cannot meet the tolerances, say which measure and why; the host decides before task 13.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/resident/field/make.rs` — today's path, `readied`, stations
- `crates/telperion-render/src/generation/resident/field_tests.rs:25-150` — compare, grid, within
- `crates/telperion-core/src/pipeline/field.rs:280-413` — the reference's cell rule
- `crates/telperion-render/src/generation/resident/stations.rs` — the leaf-bearing segments
- `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/DRAWN.md`, `RESULTS.md`
## Acceptance
- [ ] PROBE-R8.md: per measure (wood, radius, foliage, leaves, total, limbs) the splat against the CPU reference on the spruce, oak and date palm at 24, 36, 48, against `within`'s tolerances.
- [ ] Paired warm times (16 interleaved): today's grow + 36³ query against the drawn stage + splat + readback, on the spruce; held memory of each.
- [ ] The stages the field path runs whose outputs it does not read, each with its time.
- [ ] Nothing in the shipped path changed; the probe is ignored-by-default test code.
## Done summary
The R8 probe is built and measured (PROBE-R8.md). It splats the field from the drawn nodes and the station segments in one device pass. Every measure falls within `within`'s tolerances on the spruce, the oak and the date palm at 24, 36 and 48, seeds 1 and 5678, as close as today's device field or closer. On the spruce, paired warm over 16 pairs, today runs 497.4 ms and the splat path 432.3 ms: a median of −63.8 ms, faster in 16 of 16. Between growth and the answers today holds 57.1 MB on the device and the splat path holds none. The leaf input recommended for both executors is the station segments, on the CPU the plan's descriptors. Limits for task 13: the 32-bit table key, the palm's frond margin at fine grids, the palm's per-executor boxes, and the palm's 5.6 ms of serial leaflets.

stage: impl-review - skipped(policy: probe, measurement-only test code, nothing shipped)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 97265ecc4310863c288c8099bdbe3d7e7942c34c
- Tests: cargo test --profile ci -p telperion-render shader_tests, cargo test --profile ci -p telperion-render splat_ -- --ignored --nocapture --test-threads 1
- PRs: