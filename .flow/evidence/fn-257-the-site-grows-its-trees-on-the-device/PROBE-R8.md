# fn-257 task 12: the field splatted from the drawn stage's output (R8 probe)

Measured on 2026-10-16 on an RTX 3080, warm, seed 1 unless a row says otherwise. This is a probe: it is test code only and ignored by default (`field/splat_probe_tests.rs`, `field/splat_probe.wgsl`), and nothing in the shipped path changed. The raw logs are in `raw/t12/` (gitignored). To run it: `cargo test --profile ci -p telperion-render splat_ -- --ignored --nocapture --test-threads 1`, narrowed with `SPLAT_PRESETS`, `SPLAT_SEEDS`, `SPLAT_SIDES` and `SPLAT_PAIRS`.

## What the probe does

The probe's path is `tree::drawn`, then `build::readied` (hang, folds, sweep), then the stations, exactly as `grow_field` runs them. After that it uses no field primitives and no index:

1. **Limb systems** (`field_build.wgsl`'s rule, unchanged). `clear`, `widest`, `carrier` and `opens` run, then 22 `order` jumps, `own`, 22 `inherit` jumps, and `marks` with a scan that names the systems densely. They read the drawn nodes and nothing else.
2. **Box** (`wood`, `leaves` and `leaflets` in their box-growing round), then a 7-word readback of the box and the system count. The grid is `voxelize.ts`'s `grid` over this box: `cell = longest axis / (n - 2)`, with one empty cell below.
3. **Splat**. There is one invocation per record:
   - `splat_wood` takes each drawn node's tapered sweep.
   - `splat_leaves` takes each station segment, with the leaf reach computed inline.
   - `splat_leaflets` takes each frond's leaflets as oriented boxes, computed inline with `leaflets`' rule kept line for line.

   Each record visits the cells whose centres its box, inflated by `half × √3`, can reach. It then applies the CPU's `query` rule exactly:
   - point-in-capsule at `half × √3` for wood and for leaf capsules;
   - box against cube for leaflets;
   - leaves as `count × share_inside(cube)`.

   The writes are: wood as an `atomicOr` flag; the thickest radius as an `atomicMax` on the f32 bits; leaves as a fixed-point (1/4096) `atomicAdd` with a carry word.
4. **Owning limb: a table and two passes, not a per-cell tally.** The splat adds each share to a global open-addressed table keyed by `cell + cells × system` (32 bits, CAS insert, linear probe). Then `splat_best` takes an `atomicMax` of each cell's sums, and `splat_limb` takes an `atomicMin` of the system on cells where the sum equals the best. This gives the CPU's "most leaves, lower id on a tie", in any order. A per-cell tally would reserve K slots in every cell, most of them empty. The table holds only the pairs that occur, measured at 0.09 to 0.47 a cell, so it is smaller at every grid. `splat_finish` writes the answer words, and only the grid comes back (16 B a cell).

## Agreement with the CPU reference (`within`'s tolerances)

The cells are the grid over the CPU reference's box. The CPU answers come from today's per-cube `Field::query(centre, cell/2)`. "today" is today's device field asked the same cells. The tolerances are: wood ≥ 0.999, radius off ≤ 0.005, foliage ≥ 0.999 (fronds 0.70), leaves error ≤ 0.002 (fronds 0.40), |total − 1| ≤ 0.001 (fronds 0.02), limbs ≥ 0.95 (fronds 0.70).

| Preset | Seed | n | Path | wood J | radius off | foliage J | leaves err | total | limbs | within |
|---|--:|--:|---|--:|--:|--:|--:|--:|--:|---|
| norway-spruce | 1 | 24 | splat | 1.0000 | 0.0016 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes |
| norway-spruce | 1 | 24 | today | 1.0000 | 0.0016 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes |
| norway-spruce | 1 | 36 | splat | 1.0000 | 0.0000 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes |
| norway-spruce | 1 | 36 | today | 1.0000 | 0.0003 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes |
| norway-spruce | 1 | 48 | splat | 1.0000 | 0.0002 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes |
| norway-spruce | 1 | 48 | today | 1.0000 | 0.0005 | 1.0000 | 0.0001 | 1.0000 | 1.0000 | yes |
| norway-spruce | 5678 | 24/36/48 | splat | 1.0000 | 0.0000 | 1.0000 | 0.0000 | 1.0000 | 1.0000 | yes (all three) |
| oregon-white-oak | 1 | 24 | splat | 1.0000 | 0.0000 | 1.0000 | 0.0001 | 1.0000 | 0.9932 | yes |
| oregon-white-oak | 1 | 24 | today | 1.0000 | 0.0000 | 1.0000 | 0.0001 | 1.0000 | 0.9932 | yes |
| oregon-white-oak | 1 | 36 | splat | 1.0000 | 0.0016 | 0.9995 | 0.0002 | 1.0000 | 0.9885 | yes |
| oregon-white-oak | 1 | 36 | today | 0.9997 | 0.0018 | 0.9997 | 0.0002 | 1.0000 | 0.9888 | yes |
| oregon-white-oak | 1 | 48 | splat | 1.0000 | 0.0010 | 0.9999 | 0.0002 | 1.0000 | 0.9889 | yes |
| oregon-white-oak | 1 | 48 | today | 1.0000 | 0.0008 | 0.9997 | 0.0003 | 1.0000 | 0.9888 | yes |
| oregon-white-oak | 5678 | 24 | splat | 1.0000 | 0.0000 | 1.0000 | 0.0001 | 1.0000 | 0.9917 | yes |
| oregon-white-oak | 5678 | 36 | splat | 1.0000 | 0.0008 | 1.0000 | 0.0001 | 1.0000 | 0.9891 | yes |
| oregon-white-oak | 5678 | 48 | splat | 0.9999 | 0.0001 | 1.0000 | 0.0002 | 1.0000 | 0.9828 | yes |
| date-palm | 1 | 24 | splat | 1.0000 | 0.0000 | 0.8457 | 0.1210 | 1.0000 | 0.8457 | yes |
| date-palm | 1 | 24 | today | 1.0000 | 0.0000 | 0.8438 | 0.1210 | 1.0000 | 0.8438 | yes |
| date-palm | 1 | 36 | splat | 1.0000 | 0.0000 | 0.8052 | 0.2076 | 1.0000 | 0.8052 | yes |
| date-palm | 1 | 36 | today | 1.0000 | 0.0000 | 0.8062 | 0.2075 | 1.0000 | 0.8062 | yes |
| date-palm | 1 | 48 | splat | 1.0000 | 0.0000 | 0.7732 | 0.2987 | 1.0000 | 0.7732 | yes |
| date-palm | 1 | 48 | today | 1.0000 | 0.0000 | 0.7725 | 0.2987 | 1.0000 | 0.7725 | yes |
| date-palm | 5678 | 24 | splat | 1.0000 | 0.0000 | 0.8498 | 0.1203 | 1.0000 | 0.8498 | yes |
| date-palm | 5678 | 36 | splat | 1.0000 | 0.0000 | 0.8000 | 0.2042 | 1.0000 | 0.8000 | yes |
| date-palm | 5678 | 48 | splat | 1.0000 | 0.0000 | 0.7646 | 0.2945 | 1.0000 | 0.7646 | yes |

For seed 5678, today's rows match the splat's within ±0.002 on every measure (`raw/t12/r8-agree.log`).

**Every measure meets its tolerance at every grid, on all three presets and both seeds. The splat is as close to the CPU as today's device field, or closer.** It reads unquantized positions, radii, counts and flanks, while today's 16-byte primitives quantize them.

Two things the host should weigh before task 13:

- **The palm's frond tolerances are where the margin is thin.** Leaves error rises with the grid: 0.12, then 0.21, then 0.30 against 0.40. Foliage J falls: 0.85, then 0.81, then 0.77 against 0.70. Today's device field shows the same trend, because both executors draw leaflets on their own streams. Extrapolated, not measured: past roughly n = 60 the palm would fail its frond tolerances on the same cause, and no splat rule can fix that.
- **The palm's boxes differ by executor.** The device's box is up to 0.85 m off the CPU's (seed 5678, −x), again because leaflets come from each executor's own draws. The spruce and the oak agree within 2e-5 m. The rows above use the CPU's grid on both sides. If each executor splats over its own box, as R8a reads, the palm's two grids have different cells, and a per-cell comparison would need one shared grid.

## Paired warm times, spruce (16 interleaved pairs)

Path A is today's `grow_field` plus a 36³ `query_field` over the field's box. Path B is the probe: the drawn stage, hang, folds, the full sweep, the stations, the derivations, the splat to 36³ over the device's box, and the readback. The two paths alternate order pair by pair, in one binary, after one warm run each.

| | Median ms |
|---|--:|
| today (grow + 36³ query) | 497.4 |
| splat path (drawn + derivations + splat + readback) | 432.3 |
| paired difference | **−63.8** (range −61.1 to −69.9) |
| splat faster | 16 of 16 |

The desk had no other GPU job running: two other sessions' gates were queued in `telperion-heavy`, waiting for room. The `nvidia-smi` samples read 33-98 %, mostly the probe's own work.

The splat path still runs the sweep's curve passes, which it never reads. Task 11's remedy (lay the nodes only) took 11.7 ms paired off the spruce. It is not applied here, so the read-out with both changes is estimated, not measured, at about 420 ms.

## Held memory, spruce

| | Today | Splat |
|---|--:|--:|
| Held on the device between growth and the answers | 57.1 MB (primitives, systems, index) | 0: the grid is read back in the same call |
| The answer on the host at 36³ | 0.75 MB | 0.75 MB |
| The splat's own buffers while it runs | n/a | 49.9 MB: work words 38.4, names 5.5, grid and table 6.1 |
| Build peak (`take_peak`) | 1,586.8 MB | 1,586.8 MB (growth's peak dominates both) |

The table was sized at 8 slots a cell rounded up to a power of two: 1.6 / 6.1 / 12.8 MB at 24 / 36 / 48. The measured fill was 0.09-0.47 pairs a cell with no slot lost, so 1 slot a cell would do.

## The derivations, each timed (spruce, oak, palm; 36³)

The wall times below are staged medians of 8 runs, each stage waited on separately. The GPU times come from one `profile` run.

| Stage | Spruce wall ms | Oak | Palm | Spruce GPU ms | Could fold into the splat? |
|---|--:|--:|--:|--:|---|
| drawn (`tree::drawn`) | 381.8 | 222.7 | 68.4 | | no: it is the build |
| hang | 0.00 | 0.00 | 3.44 | | no: the splat reads the boots |
| folds → expansion | 1.72 | 1.66 | 0.82 | | no: leaf reach, canopy, element, bounds and station rows |
| sweep (gather + the curve) | 18.82 | 6.30 | 2.46 | gather 2.19 | only `gather` is read (see below) |
| stations | 13.42 | 5.15 | 1.30 | | no: they are the leaf-bearing segments |
| **limb systems + dense names** | 8.37 | 3.86 | 0.72 | 4.3 (jumps 2.4, scan 1.86 of which `spine` 1.47) | The pointer jumps, no: they are tree-wide. The dense naming, yes: it exists only so the table key fits in 32 bits, and a two-word key (cell, node id) would let it go, about 1.9 ms GPU on the spruce. Dense names keep node order, so ties resolve the same either way. |
| **box** + 7-word readback | 0.55 | 0.28 | 0.26 | 0.36 | Partly. The grid could be computed on the device and read by the splat from storage, which drops the one sync. |
| **leaf reach** | inline | inline | inline | 0 | Already folded: computed per segment inside `splat_leaves`. |
| **leaflets** | inline | inline | inline | | Already folded. On the palm, though, `splat_leaflets` takes 5.6 ms GPU, because one invocation runs a whole frond (58 invocations). One invocation per leaflet would need a leaflet-to-frond map, and would be task 13's fix. |
| splat (wood, leaves) | 0.85 | 0.28 | 5.80 | 0.68 | |
| owners (best, limb, finish) | 0.15 | 0.15 | 0.16 | 0.02 | |
| readback of the grid | 0.23 | 0.27 | 0.25 | | |

After the stations, the splat path takes about 10.2 ms on the spruce. Today's path takes 12 ms of primitives, 4-7 ms of index and 65 ms of query (FIELD-TIME.md).

## Stages the field path runs whose outputs it never reads

These were measured on the splat path. They are the same as today's path, because the stages before the field are shared.

| Stage | Spruce | Oak | Birch | Read? |
|---|--:|--:|--:|---|
| The sweep's curve passes: `lead`, `ties`, `starts`, `walk`, the bitonic sort, `block`, `blocks`, `place`, `emit`, two count readbacks | about 16 ms wall (task 11); 11.7 ms paired when dropped; this run's sweep 18.8 total against `gather` 7.1 | about 5 | about 5 | **No.** Their outputs feed only the curve's handoff and the twig contact words. GPU: `lead` 0.25, `ties` 2.35, `walk` 1.02, `bitonic` 0.89 over 171 dispatches, `block` 0.07, `place` 0.01, plus the sweep's share of `starts`, `blocks` and `emit`, whose labels the stations share. |
| Rings, placement, mass grid, handoff (the curve's packing, normals, bark) | not run | not run | not run | No pass of theirs appears in the profile of the field path. |
| Boots (hang) | 0 | 0 | (palm 3.4) | Read: the boots are wood, and `structural` reads their `BOOT` flag. |
| Trunk (root, stem) | in `drawn` | | | Read: the folds' summaries feed the expansion the leaf rows come from. |
| Dress extras (colour and season words) | in `dress` (5.9 ms GPU, 4 dispatches) | | | Unread, but written by the same kernel as the node words the splat reads, so it is not a stage that can be dropped. |
| Stations: florets and blooms | 0 (none flower) | | | Unread by the splat; they run only on a flowering age. |

## The CPU's records and the leaf form (host's scope note, R8 "Both executors")

**The matching CPU records:**

- **Wood.** The CPU's `Tree::nodes` (`Field::wood`): each node's position, its parent's position, `start_radius` (the node's own `radius` at the root) and `radius`. The device splat reads the same from the swept nodes: position (words 0-2), end radius (3), start radius (4), parent (8), and the `MADE`/`BOOT` flags (9).
- **Leaves.** The CPU's leaf plan, `Plan::descriptors` (`pipeline/foliage/plan.rs`): each leaf-bearing segment's endpoints, radii, station `count` and limb `system`. A frond's descriptors are its leaflets' oriented boxes (`side ≠ 0`). The device splat reads the same from the station segments (endpoints and radii in words 0-7, count in word 25, the segment's node and through it the system) and from `leaflets`' boxes. The CPU's placed-leaf form (`Field::new` over `Instances`) is used only where `plan::plan` gives no plan.

**The leaf input should be the station segments: on the CPU, the plan's descriptors, which describe the same segments.** The evidence:

- The splat over station segments meets every tolerance above, limbs included (0.98-1.00 on the spruce and oak).
- Placed leaves carry no limb system, so the CPU's placed form answers `limb: None` everywhere and could not meet the limbs measure.
- Placed leaves are one record per leaf: PROBE.md read 13-358 MB back for them.
- Inferred, not run here: the gate test `every_preset_s_device_field_answers_within_its_tolerance` holds limbs ≥ 0.95 on every shipped preset, which a placed CPU field cannot do. So every shipped preset's CPU field today is already the planned form, and choosing segments for both executors changes no shipped preset's leaf input.

## The splat's limits for task 13

- **The 32-bit table key.** It holds `cells × systems < 2^32`. That is ample at n ≤ 48: the spruce has 150 systems, the oak 447-464, the palm 2. At the spec's n = 256 (16.7 M cells), only 255 systems fit, so the oak, and the birch with its 387, would be refused. A two-word key, or keeping node-id systems in a second word, removes the limit and also lets the dense naming go.
- **Grid memory.** The grid's words are 24 B a cell plus the table. At n = 256 that is about 400 MB plus the table: at the measured 0.1-0.5 pairs a cell the table is 13-67 MB, while the probe's fixed 8 slots a cell would reserve far more. The grid's refusal against the device's buffer limits has to come before any of that is allocated.
- **The palm's frond tolerance at fine grids** (above): a property of the executors' leaflet draws, not of the splat.
