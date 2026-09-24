# fn-134 results (2026-09-24)

The base is `b8acb508`, fn-102 merged, built in its own worktree with its own `target/`. The candidate is this branch. The tools are fn-102's `binding.mjs`, `rss.py` and `meshhash.rs`, plus this spec's `tools/woodtime.rs` (the public wood build timed alone) and `tools/r3cmp.py` (the binding rows, base against candidate). Raw rows are in `raw/`, which git ignores. The machine had 32 CPUs, and other sessions held the load average at 1.5 to 6.

## Design as built

- **The ring step** (`surface/rings.rs`) sweeps positions, coords, contact edges and the run table as swept. It is parallel phase 1 where the tree is admitted, else it runs in turn. **The mesh step** (`surface::faces`) adds indices, normals and bounds around the rings. It reuses the ring step's workers, and a dropped triangle hands it to the serial step on the same rings. `Rings::into_mesh` moves positions, coords and the run table into the wood without a copy.
- **The pipeline** has one flow. The rings sit in a `OnceLock`. The wood thread and seated leaves both call `get_or_init`, so whichever reaches the rings first sweeps them and the other waits. The wood mesh and the leaves run side by side for every family. Where no wood is drawn, seated leaves sweep bare rings of their own: ring points in path order, with no caps, coords or run table.
- **Written once.** In the parallel phases every output buffer is filled through `parallel/unfilled.rs`. Each worker writes its part front to back into reserved, uninitialised capacity. The buffer becomes a `Vec` only when the parts tile it and were all written in full. This is the crate's one `unsafe` block, a `set_len` behind that check.
- **The facing fallback.** A vertex that no triangle shades takes its ring's frame. Frames are no longer kept per ring: the rare run that needs them is swept again (`Resweep`). No catalogue or in-work preset reaches that path at seed 1 or 7 (0 of 16 builds).

## R2: bytes

- `mesh::build`: 8 presets at seeds 1 and 7 hash the same as base, 16 of 16.
- The binding: 8 families, 2 seeds and all 15 output combinations hash the same as base, 240 of 240. The date palm is included.

## R3: Wasm peak linear memory

The peak equals base in all 240 builds, with none higher and none lower. The first candidate was higher in 44 builds, by 0.5 to 5.4 MB. Four layout causes are fixed:
- bare rings for leaves without wood, where the first candidate had capped, ranked rings
- serial normals summed in place, with no growing scratch
- the run table allocated after the rings, so the ranking's freed space joins the freed sweep scratch
- a single sweep scratch shared by the ranking and the sweep, as base had

FRICTION.md records the cost of finding them.

## R4: whole-build medians of five (native, seed 1, `generation_stages`, mesh request)

| Preset | Batch 1, base → candidate | Batch 2 (order swapped) |
|---|---|---|
| ordinary | 121 → 120 (−0.5%) | 122 → 120 (−1.3%) |
| oak | 397 → 396 (−0.2%) | 394 → 391 (−0.8%) |
| spruce | 4920 → 4822 (−2.0%) | 4746 → 4701 (−0.9%) |
| birch | 2266 → 2274 (+0.3%) | 2226 → 2220 (−0.2%) |
| telperion | 1009 → 1006 (−0.3%) | 973 → 964 (−0.9%) |
| laurelin | 422 → 424 (+0.6%) | 403 → 403 (+0.1%) |

- Every row is within run-to-run noise. The two positive rows in batch 1 were not reproduced in batch 2. Peak RSS is unchanged, within about 5 MB.
- **The spruce.** fn-102 accepted +4.6% against its base `18da42fa`. Against fn-102's merged base, the spruce is now 0.9 to 2.0% faster. Its leaves can start as soon as the rings exist, and its wood no longer fills zeros first.
- **Wasm** (binding timings, 5 builds, combinations 3, 8 and 15): every family and combination is within ±5% both ways. Field-only builds, which run no changed code, show the same spread, so it is noise.

## R6: the parallel wood build (`surface::build`, 27 samples a side, interleaved)

| Preset | Base | Candidate | Change |
|---|---|---|---|
| oak | 72.5 ms | 53.2 ms | −19.4 ms (−26.7%) |
| spruce | 50.9 ms | 36.9 ms | −14.1 ms (−27.6%) |
| birch | 42.8 ms | 32.7 ms | −10.1 ms (−23.6%) |

- **No zero fill remains.** No wood output buffer is zero-filled before its producer writes it: positions, coords, normals and indices are reserved, then written once by their workers.
- **Where the rest went.** The saving is 10 to 19 ms against a fill of 18 to 25 ms on base. The first touch of each fresh page now falls on the workers' writes, in parallel.
- **Serial normals.** On the serial path each run's normals still start at zero, because they are sums. The producer does that per run, just before it sums them. The native parallel path sums into a per-worker scratch the size of the widest run and writes each normal once.

## R1, R5 and the gates

- **R1.** `every_family_builds_the_same_bytes_under_either_schedule` covers 6 catalogue and 2 in-work families. It asserts that each builds the same bytes under either schedule and that the concurrent schedule splits wood and leaves.
- **Seated fork removed.** The `seated` fork is gone, with its `!split` assertions and its schedule branch.
- **Error order.** `the_leaves_own_rings_fail_after_the_plan` pins it for leaves that sweep their own rings.
- **R5, lines.** `git diff --numstat 580ad009`, crates:
  - production: +909 −568, **net +341**
  - tests: +115 −85, net +30
  - examples: net 0
- **Where the lines went.** `rings.rs` holds the ring step with its fallback resweep and bare mode, 358 lines. `unfilled.rs` is 96 lines. The prepared surface is now its own pass over the rings rather than branches inside the one build loop.
