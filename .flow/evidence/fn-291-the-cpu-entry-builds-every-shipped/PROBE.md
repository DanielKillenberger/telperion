# fn-291 R1: every shipped preset through the dist CPU entry in Node

Measured 2026-10-16 on 6a6d5abcb + the fn-291 plan commits (no generator or entry code changed), Node v26.8.1, `npm run build` dist. `probe-dist.mjs` runs each case in its own child process under `telperion-heavy 6 --threads 2`. Each child creates a fresh `TreeEngine`, builds the preset at its preset age with `skeleton.seed` set, and reads `engine.memoryBytes` (Wasm linear memory, which only grows, so it is the case's high-water mark). Raw rows: `raw/probe.jsonl`; `raw/probe-classified.jsonl` relabels them with the probe's current outcome classes (`refused` only for an accepted named refusal, `core-error` for any other `Tree core:` error, `trap:` for anything else). A failing case is re-run as surface-only and as foliage-only.

## Table

Memory is the Wasm linear memory in MiB after the build, or after the throw. Time is wall seconds for the build call.

| preset | age | seed | outputs | outcome | nodes | memory MiB | s |
|---|---|---|---|---|---|---|---|
| ordinary | 40 | 1 | both | built | 86361 | 159 | 0.6 |
| ordinary | 40 | 7 | both | built | 130355 | 203 | 1.0 |
| ordinary | 40 | 42 | both | built | 152178 | 304 | 1.1 |
| oregon-white-oak | 40 | 1 | both | built | 275902 | 566 | 2.1 |
| oregon-white-oak | 40 | 7 | both | built | 384593 | 748 | 3.1 |
| oregon-white-oak | 40 | 42 | both | built | 285930 | 583 | 2.3 |
| norway-spruce | 40 | 1 | both | built | 994902 | 1792 | 13.0 |
| norway-spruce | 40 | 7 | both | built | 966173 | 1745 | 12.3 |
| norway-spruce | 40 | 42 | both | built | 1005581 | 2036 | 12.9 |
| european-beech | 40 | 1 | both | built | 114700 | 212 | 0.6 |
| european-beech | 40 | 7 | both | built | 205400 | 376 | 1.0 |
| european-beech | 40 | 42 | both | built | 251816 | 462 | 1.4 |
| silver-birch | 40 | 1 | both | built | 242453 | 457 | 1.6 |
| silver-birch | 40 | 7 | both | built | 355649 | 680 | 3.0 |
| silver-birch | 40 | 42 | both | built | 300787 | 619 | 3.1 |
| date-palm | 40 | 1 | both | built | 993 | 3 | 0.0 |
| date-palm | 40 | 7 | both | built | 993 | 3 | 0.0 |
| date-palm | 40 | 42 | both | built | 993 | 3 | 0.0 |
| telperion | 82 | 1 | both | built | 1985625 | 3805 | 18.0 |
| telperion | 82 | 7 | both | **core-error: surface allocation** | - | 4078 | 29.0 |
| telperion | 82 | 7 | surface | **core-error: surface allocation** | - | 4079 | 27.7 |
| telperion | 82 | 7 | foliage | built | 3358968 | 2694 | 27.5 |
| telperion | 82 | 42 | both | **core-error: surface allocation** | - | 3571 | 27.8 |
| telperion | 82 | 42 | surface | **core-error: surface allocation** | - | 3571 | 24.0 |
| telperion | 82 | 42 | foliage | built | 3359470 | 2740 | 25.6 |
| laurelin | 80 | 1 | both | built | 1859572 | 3586 | 17.1 |
| laurelin | 80 | 7 | both | **core-error: surface allocation** | - | 3352 | 23.2 |
| laurelin | 80 | 7 | surface | **core-error: surface allocation** | - | 3352 | 21.8 |
| laurelin | 80 | 7 | foliage | built | 3127437 | 2578 | 22.0 |
| laurelin | 80 | 42 | both | **core-error: surface allocation** | - | 3396 | 24.0 |
| laurelin | 80 | 42 | surface | **core-error: surface allocation** | - | 3396 | 23.0 |
| laurelin | 80 | 42 | foliage | built | 3143212 | 2618 | 23.0 |

There were 4 failures in 24 cases: telperion and laurelin at their preset ages, seeds 7 and 42. Every failure is a `core-error`: the core's `ResourceLimit("surface allocation")`, surfaced as `Tree core: surface allocation`. None is an accepted named refusal, and none is a trap or a `RangeError`. Every failure is in the surface stage; foliage-only builds the same trees. The other six presets build at all three seeds, with at most 2,036 MiB (the spruce). No preset is refused "no engine species". At about 3.36M nodes, the failing trees sit under the existing wasm32 phytomer ceiling (`WASM_KEPT`, 4,000,000), so that ceiling does not refuse them before growth.

The README's first example, telperion at seed 7, is one of the failures.

## Cause

**The allocation.** `reserved()` (`crates/telperion-core/src/pipeline/surface.rs:64-69`) calls `Vec::try_reserve_exact`. When that fails, it returns `ResourceLimit("surface allocation")`. On wasm32 that means the 4 GiB linear memory (65,536 pages) cannot grow far enough to hold the request.

**The order on the wasm path.** The parallel sweep only runs on linux x86_64, so wasm takes the serial path. The surface reserves its buffers in this order:
1. `rings()` reserves `positions`, then `coords` (`surface/rings.rs:139-140`).
2. `faces()` reserves `normals` (the same length as `positions`), then `indices`, which is `ring_vertices * 6` (`surface/build.rs:112-113`).

**Each failure traced.** The traced wasm is a throwaway build in which `reserved()` reports its call site, the request and the memory size at the failure (`trace-reserved.patch`). It was built into a scratch target and run on a scratch copy of `dist/`, then reverted; the crate is unchanged. Rows for the four failing cases, surface only:

| case | failing reservation | elements | requested MiB | memory at failure MiB (pages) | request + memory |
|---|---|---|---|---|---|
| telperion 82, seed 7 | `normals`, `surface/build.rs:112` | 217,870,704 f32 | 831 | 4,080 (65,277) | 4,911 > 4,096 |
| telperion 82, seed 42 | `coords`, `surface/rings.rs:140` | 145,263,916 f32 | 554 | 3,571 (57,143) | 4,125 > 4,096 |
| laurelin 80, seed 7 | `normals`, `surface/build.rs:112` | 202,888,194 f32 | 774 | 3,352 (53,639) | 4,126 > 4,096 |
| laurelin 80, seed 42 | `normals`, `surface/build.rs:112` | 203,944,122 f32 | 778 | 3,397 (54,346) | 4,175 > 4,096 |

Each request is exact; it is not an estimate. It is the wood's own vertex count: `extent_of()` (`surface/build.rs:21-50`) counts one ring per drawn sample, and each ring has `max(radialSegments, 4·lobes)` sides (`surface/build.rs:14-15`). Every shipped preset has radialSegments 12 and lobes 5, so every ring on every branch has 20 sides, twigs included. That comes to about 21.6 vertices per node. No reservation is oversized, and the `indices` reservation, the largest at about 1.6 GiB, is never reached.

**The whole build's size.** A native trace (`woodsize.rs`, built against this tree's `telperion-core`, surface only) runs the same request under a counting allocator. Its node counts and vertex counts match the wasm ones exactly. Raw output: `raw/woodsize.log`.

| case | nodes | wood buffers MiB (positions + normals + coords + indices) | largest single buffer MiB | heap peak MiB |
|---|---|---|---|---|
| telperion 82, seed 1 (builds) | 1,985,625 | 2,286 | 976 (indices) | 2,591 |
| telperion 82, seed 7 | 3,358,968 | 3,867 | 1,651 (indices) | 4,476 |
| telperion 82, seed 42 | 3,359,470 | 3,868 | 1,651 (indices) | 4,476 |
| laurelin 80, seed 7 | 3,127,437 | 3,601 | 1,537 (indices) | 4,209 |
| laurelin 80, seed 42 | 3,143,212 | 3,620 | 1,545 (indices) | 4,228 |

For telperion at seed 7, the buffers are:
- positions: 217,870,704 floats (831 MiB)
- normals: 831 MiB
- coords: 554 MiB
- indices: 432,771,840 u32 (1,651 MiB)

The four wood buffers alone take 3.5 to 3.8 GiB, which is 88 to 94% of the 4 GiB address space. The whole build's live heap peaks at 4,209 to 4,476 MiB. That is more than 4 GiB in every failing case, before any allocator slack. So no allocator and no estimate makes these builds fit as they are drawn. The cause is not a sizing estimate, and it is not one allocation's cap. The drawn wood needs more memory than 32-bit Wasm has. The point where each case fails, at `coords` or at `normals`, depends only on how much the growth stage left in use.

Seed 1 builds because its tree is 41% smaller: 1.99M nodes against 3.36M. Even so, it reaches 3,805 MiB of Wasm memory against a 2,591 MiB native peak. The gap is the Wasm allocator's growth slack and fragmentation.

**Why nothing caught it earlier.** The phytomer ceiling was measured on the spruce at seed 42 (fn-198 BROWSER.md). It bounds kept phytomers, not the wood's bytes. The native entry test `every_shipped_preset_builds_through_the_main_entry` runs on the host allocator and builds these trees without complaint.

## What R2 leaves open (for the host)

This is a design choice, and task 1 does not make it. The options the measurements leave:

1. **Refuse by name.** Add a wasm32-gated prediction of the wood's bytes (`extent_of(...).bytes()` plus the tree) before `rings()` reserves anything, following `OVER`/`foreseen`/`fits`. Its message would name the cause and the remedy (grow it younger, or ask for foliage only, or use a native build). This refuses two shipped presets at their preset ages for two of three seeds, including the README's first example, and needs the owner's word.
2. **A memory fix that keeps the drawing.** Lower the peak below about 3.5 GiB of Wasm memory for these trees. For example, drop the tree's and the sweep's working state before `faces()`, or stop holding `coords` (554 MiB), or emit the index buffer without a full reservation. To fit with the observed allocator slack, the peak for telperion at seed 7 must fall by at least about 1 GiB from 4,476 MiB. Even then the four wood buffers alone are 3,867 MiB. Dropping `coords` and building `normals` and `indices` without holding the rings' working state would still leave about 3.3 GiB of output, so the headroom is thin and other seeds may exceed it.
3. **A geometry change.** Draw fine wood with fewer sides. At present the `4·lobes` floor puts 20 sides on every twig. This changes the drawn tree (allowed under "Generator evolution" with a visual check), and it is generator design.
4. **Lower the two presets' ages.** Telperion is at 82 and laurelin at 80. This changes shipped presets and is the owner's call.
5. **memory64 or a second binding.** Out of scope under the spec's Boundaries.

Options 1, 3 and 4 change what a shipped preset promises. Option 2 is the only one that changes nothing the owner set, but the measured margin says it may not hold for every seed.

## Task 2: where the Wasm memory goes, and the floor R2a and R2b can reach

I measured this on 2db376e1e, before any R2a code change. It is the measure-first step of task 2.
- `trace-pages.py` is a throwaway traced build of the binding. It is applied, built into a scratch target and reverted, so it is never committed patched.
- It records the Wasm memory at the end of growth and around every wood reservation of at least 16 MiB.
- `trace-case.mjs` runs one case through it. Raw rows: `raw/trace-base.jsonl`.
- Native live sets come from `woodsize.rs` (now with a `skeleton` selector). Raw rows: `raw/woodsize-skeleton.log`.

### Wasm memory (MiB) at each step, surface and foliage

| case | after growth | rings `positions` | rings `coords` | faces `normals` | faces `indices` |
|---|---|---|---|---|---|
| telperion 1 | 1,519 | +491, 1,519 → 2,011 | +327, → 2,338 | +491, → 2,829 | +975, → 3,805 |
| telperion 7 | 2,693 | +831, → 3,524 | +554, → 4,078 | +831, **fails** at 4,078 | — |
| telperion 42 | 2,740 | +831, → 3,572 | +554, **fails** at 3,572 | — | — |
| laurelin 7 | 2,578 | +773, → 3,352 | +515, reused, 3,352 | +773, **fails** at 3,352 | — |
| laurelin 42 | 2,618 | +777, → 3,396 | +518, reused, 3,396 | +777, **fails** at 3,396 | — |

The growth stage alone leaves 2.6 to 2.7 GiB of Wasm memory, and those pages never return. A native skeleton-only build of telperion seed 7 peaks at 3,003 MiB and keeps 604 MiB. Its nodes `Vec` is the largest single allocation at 528 MiB.

About 2 GiB of that memory is free once growth ends. Even so, the first large wood reservation never reuses it: `positions` grows the memory by its full size in every case. Only laurelin's `coords` fit into the freed space. The allocator needs one contiguous run for each buffer, and growth leaves its free space in pieces.

### The floor

The floor is the smallest live set at the end of the build that R2a and R2b allow. These allocations are all alive together, because the CPU entry hands them to JS:
- positions as they are (f32 ×3)
- normals oct-encoded at 4 B a vertex (R2b)
- indices as they are (u32; R2b covers per-vertex data only)
- the leaf instances (12 B a leaf)
- the tree, which `faces()` and the leaves read and which the metadata reads at the end

Bark coords never reach JS. The floor is given twice: with coords freed by R2a, and with coords kept at 2×u16 as R2b names them. It assumes perfect R2a: no fragmentation and nothing else alive.

| case | positions | normals (oct) | indices | tree | leaves | floor, coords freed | floor, coords u16 |
|---|---|---|---|---|---|---|---|
| telperion 7 | 831 | 277 | 1,651 | 604 | 32 | **3,395** | **3,672** |
| telperion 42 | 831 | 277 | 1,651 | 604 | 32 | **3,395** | **3,672** |
| laurelin 7 | 774 | 258 | 1,537 | 604 | 30 | **3,203** | **3,461** |
| laurelin 42 | 778 | 259 | 1,545 | 604 | 30 | **3,216** | **3,475** |

The bar is 3,584 MiB (3.5 GiB).
- **Coords u16, as R2b names them:** telperion at seeds 7 and 42 is 88 MiB over the bar even with perfect R2a.
- **Coords freed:** telperion's floor is 189 MiB (5%) under the bar. That margin assumes zero allocator slack.

The measurements contradict zero slack.
- Telperion seed 1 builds today at 3,805 MiB against a live end set of 2,586 MiB, so the slack is 1,219 MiB.
- Every failing case grows the memory past its 2.6 to 2.7 GiB growth peak for its first wood buffer.
- At a 2.69 GiB growth peak, the 2,759 MiB of wood outputs cannot all land in the about 2.09 GiB that growth frees. At least about 670 MiB must grow the memory beyond the peak, so the high-water mark is at least about 3.36 GiB before any fragmentation.

Other seeds are not covered. Telperion at seeds 7 and 42 is 3.36M nodes, under the wasm32 ceiling of 4M kept phytomers. A larger seed takes the floor up in proportion.

**Gap (stop condition met).** With R2b as written (oct normals, u16 bark coords), the floor for Telperion is 3,672 MiB against a 3,584 MiB bar: 88 MiB over before any slack. With coords also freed, it is 3,395 MiB: 189 MiB under the bar only at zero slack, while the measured slack is 1,219 MiB on the one case that builds. R2a and R2b cannot reach the bar with headroom.

The lever the measurements point to is the index buffer: 1,651 MiB, 43% of the wood. Every index follows from the ring topology (`surface/prepared.rs`; base, rings and sides per run), except dropped zero-area triangles. Handing JS a run table instead of `indices` would bring the floor to about 1.7 to 2.0 GiB. Growth's 2.7 GiB peak would then be the high-water mark. That changes the package's output format, it is not per-vertex data, and it is the host's call. So is any change to positions' precision or to growth's peak.

### R2a, implemented and measured

**Write once.** On the wasm path, every wood buffer is already reserved once at its exact final size (`surface/rings.rs:139-140`, `surface/build.rs:112-113`) and moved into the mesh, never grown or copied. The one exception is the run table, which is copied only where a triangle was dropped. That part of R2a needed no change.

**Free what the surface does not read: the tree's spare capacity.** The node buffer grew by doubling, so it kept up to twice its nodes past growth. For telperion at seed 7 that is a 528 MiB buffer for 423 MiB of nodes, and `frames`, `keys` and `grown` had spare too. `pipeline::build` now calls `fitted()` after clothing, which gives that spare back before the wood reserves anything. The test `pipeline::tests::the_built_tree_keeps_no_spare_capacity` was red on the base (left `[109966, 0, 0, 109966, 109966, 109966, 0, ...]` for ordinary) and is green after.

**Output unchanged.** I built `dist/` before and after and hashed every buffer the entry hands JS (surface positions, normals and indices; foliage positions, indices and leaves) with `hash-dist.mjs`, for all 8 presets at seed 1. All 8 are identical: `raw/hash-base.txt` against `raw/hash-r2a.txt`. The change touches no value, only capacity.

**Measured after R2a.** Traced as before; raw rows in `raw/trace-r2a.jsonl`. Wasm memory in MiB:

| case | after growth | rings `positions` | rings `coords` | faces `normals` | faces `indices` | outcome |
|---|---|---|---|---|---|---|
| telperion 1 | 1,519 | → 2,011 | → 2,338 | → 2,829 | → 3,805 | built, 3,805 (unchanged) |
| telperion 7 | 2,693 | → 3,524 | → 4,078 | **fails** | — | 4,078 (unchanged) |
| telperion 42 | 2,740 | → 3,572 | **fails** | — | — | 3,572 (unchanged) |
| laurelin 7 | 2,578 | **reused**, 2,578 | → 3,094 | → 3,868 | **fails**, 1,537 | 3,868 |
| laurelin 42 | 2,618 | **reused**, 2,618 | → 3,137 | → 3,915 | **fails**, 1,545 | 3,915 |

- **Laurelin.** The freed spare lets laurelin's `positions` land in memory growth gave back. The failure moves from `normals` to `indices`. The build would still need about 5.4 GiB of Wasm memory to finish.
- **Telperion.** Its `positions` still finds no contiguous free run, so nothing changes.
- **No case gets closer to the bar.** Laurelin's high-water mark at failure rises, because it now gets one reservation further.

The other R2a candidate is `edges`. It costs 40 B a node, about 128 MiB for telperion. Leaves read it, and on the serial path they run after the wood's `faces()`. Freeing it before `faces()` means running leaves before the wood, which changes the order in which the stage reports errors (`pipeline::tests::the_wood_fails_before_the_plan`). Even then it would save at most 128 MiB against the gap below. It was not taken.

### The floor after R2a

`fitted()` gives back the tree's spare capacity. I measured the native live set again after it; raw rows are in `raw/woodsize-skeleton-r2a.log`:
- telperion at seeds 7 and 42: 483 MiB, down from 604
- laurelin at seed 7: 450 MiB
- laurelin at seed 42: 452 MiB

This table supersedes the floor table above.

| case | positions | normals (oct) | indices | tree after R2a | leaves | floor, coords freed | floor, coords u16 |
|---|---|---|---|---|---|---|---|
| telperion 7 | 831 | 277 | 1,651 | 483 | 32 | 3,274 | **3,551** |
| telperion 42 | 831 | 277 | 1,651 | 483 | 32 | 3,274 | **3,551** |
| laurelin 7 | 774 | 258 | 1,537 | 450 | 30 | 3,049 | 3,307 |
| laurelin 42 | 778 | 259 | 1,545 | 452 | 30 | 3,064 | 3,323 |

With R2b as written, the live floor for telperion is 3,551 MiB. That is 33 MiB under the 3,584 MiB bar, and only if Wasm adds no slack at all.

The traces show what Wasm actually adds. Every buffer must be one contiguous run of memory:
- After growth, telperion's memory stands at 2,693 MiB (seed 7) and 2,740 MiB (seed 42).
- The first wood buffer, `positions` at 831 MiB, grows the memory by its full size, before and after R2a. So no free run of 831 MiB exists anywhere below the top: neither inside the memory growth freed nor at its end.
- The index buffer is a single 1,651 MiB `u32` run. R2b keeps it, because it is not per-vertex data. It cannot land in any run smaller than 831 MiB.
- So wherever it is reserved, it grows the memory by its whole size. The high-water mark is at least 2,693 + 1,651 = **4,344 MiB** for telperion seed 7, and 2,740 + 1,651 = 4,391 MiB for seed 42.
- Both bounds are over 4 GiB itself, before positions, normals or coords are counted.
- Laurelin is not measured as tightly. After R2a, its `positions` did find a free run of 773 MiB, and its index reservation then failed at 3,868 MiB without finding 1,537 MiB of free memory.
- If no free run of 1,537 MiB exists after growth, the same bound for laurelin is 2,578 + 1,537 = 4,115 MiB at seed 7 and 2,618 + 1,545 = 4,163 MiB at seed 42. That condition is inferred from the failed reservation, not measured on its own.

The bar is 3,584 MiB. R2b changes no term in this bound: its smaller normals and coords may fit in freed pieces, but the indices do not.

### Gap (stop condition met)

R2a is built and measured. It does not lower the high-water mark of any failing case.

R2b cannot close the gap. Its live floor (3,551 MiB for telperion) is only under the bar at zero slack. The contiguity bound shows the slack cannot be zero. Growth leaves 2.6 to 2.7 GiB of Wasm memory with no free run of 831 MiB in it, and the 1,651 MiB index buffer then needs memory of its own above that. The high-water mark is at least 4,344 MiB for telperion seed 7, which is 760 MiB over the 3,584 MiB bar and over 4 GiB itself.

The levers the measurements point to are both outside R2a and R2b as decided, and both are the host's call:
- **The index buffer.** It is 1,651 MiB, 43% of the wood. It is derivable from the ring topology, so it could be handed to JS as a run table, or in pieces under a run of 831 MiB.
- **Growth's 2.7 GiB peak and its fragmentation.**

## Task 5: the wood in chunks

Measured 2026-10-16 on 5b2636d62 (the chunked wood). The CPU wood is now written as chunks of at most 65,536 vertices, each with its own positions, normals, coords and `u16` indices, each buffer reserved on its own at exact size as its chunk is written. No wood buffer needs a free run larger than 768 KiB (a full chunk's positions or normals).

### Values unchanged

The longest run of any shipped preset at its own seed is 9,642 vertices (date-palm), so no shipped run splits. `flatdigest-base.rs` digested each preset's flat wood on the base commit 9eed854d0: FNV-1a over every position, normal and coord's bits, then every index in draw order. `flatdigest.rs` digests the chunks laid end to end (`SurfaceMesh::flattened`, indices made absolute) in the same way. All 8 digest lines are identical (raw: `raw/flatdigest-base.log`, `raw/flatdigest-chunks.log`):

| preset (seed 42) | digest | vertices | chunks |
|---|---|---|---|
| ordinary | 5faf7f491f2f7cf0 | 3,338,382 | 52 |
| oregon-white-oak | 755c892c91da12dd | 6,300,258 | 97 |
| norway-spruce | 6b12178fadc5850e | 24,527,020 | 376 |
| european-beech | 13e40e280c7135c2 | 5,499,992 | 85 |
| silver-birch | b8a46eac9d5c6c78 | 6,708,960 | 103 |
| date-palm | 060aa52679caaeb4 | 25,494 | 1 |
| telperion | e31b0b89bcbbb198 | 72,631,958 | 1,113 |
| laurelin | da746f7a93aa7735 | 67,981,374 | 1,042 |

After the review the check asserts. Both tools take seeds, and `flatdigest.rs --expect <base log>` exits nonzero on any digest line the base did not print, or on any preset that fails to build. The base tool was rebuilt from a temporary worktree at 9eed854d0 (its own target directory) and run at seeds 1 and 42; the chunked tool ran against that log and exited 0. All 16 lines match (raw: `raw/flatdigest-base-1-42.log`, `raw/flatdigest-chunks-1-42.log`). At seed 1: ordinary bcbc6a9d6d555136, oregon-white-oak 255a56e1508e0030, norway-spruce 1a60a707e6b79534, european-beech 4a6bca73900882dc, silver-birch e23a3e810440fdc3, date-palm 19b3b6a53841c6b8, telperion 8dc054ab905a885b, laurelin f170ef631674d8e6.

A split run keeps every value and the run's draw order over its chunks laid end to end (host decision after review, option 3). Its last piece repeats the run's first ring and bottom-cap vertex, one ring plus one vertex more a split run (13 at the default 12 segments), so every cap triangle draws in the last chunk, after every strip, interleaved as in the unsplit run. `chunks::tests::a_run_longer_than_a_chunk_splits_with_no_value_changed` asserts that sequence exactly; it failed on the previous layout (caps in the first chunk) and passes on this one. No shipped run splits, and after the change the chunked tool still matches all 16 base lines at seeds 1 and 42 (`--expect`, exit 0; raw: `raw/flatdigest-chunks-5b-1-42.log`).

The species digests (`suite/species.rs`) and the bark hashes (`telperion-render/tests/bark_*.rs`) hash the chunks laid end to end, and none was re-pinned. A run longer than a chunk is covered by `chunks::tests::a_run_longer_than_a_chunk_splits_with_no_value_changed` (140,000 vertices over three chunks).

### The high-water mark

`probe-dist.mjs` on the built `dist/`, one fresh engine per case, surface and foliage. Raw rows: `raw/probe-chunks.jsonl`. The trace is `trace-pages.py`, now also marking the end of the wood's mesh step, because chunk reservations fall under its 16 MiB floor. Raw rows: `raw/trace-chunks.jsonl`. Wasm memory in MiB:

| case | before (task 2) | after growth | after the wood | high-water | outcome |
|---|---|---|---|---|---|
| telperion 1 | 3,805 | - | - | 2,132 | built, 1,985,625 nodes |
| telperion 7 | fails at 4,078 | 2,693 | 3,574 | **3,608** | built, 3,358,968 nodes |
| telperion 42 | fails at 3,572 | - | - | **3,608** | built, 3,359,470 nodes |
| laurelin 1 | 3,586 | - | - | 2,100 | built, 1,859,572 nodes |
| laurelin 7 | fails at 3,868 | 2,578 | 3,329 | 3,567 | built, 3,127,437 nodes |
| laurelin 42 | fails at 3,915 | 2,618 | 3,345 | 3,583 | built, 3,143,212 nodes |

Every case builds. The chunks land in the memory growth freed: telperion seed 7's wood keeps 3,042 MiB live (positions 831, normals 831, coords 554, `u16` indices 826) and grows the memory by only 881 MiB past growth's 2,693. Telperion at seeds 7 and 42 is still 24 MiB over the 3,584 MiB bar. Laurelin 42 is 1 MiB under it.

### R2b's projected effect

R2b stores each vertex's normal oct-encoded in 4 B (from 12) and its bark coords in 2 × `u16` (from 8): 12 B saved a vertex. Telperion seed 7 has 72.6M vertices, so R2b saves 554 MiB of normals and 277 MiB of coords, 831 MiB in all. The wood stage grew the memory by 881 MiB, so a wood 831 MiB smaller fits all but about 50 MiB of itself into what growth freed. Projected high-water: about 3,608 - 831 = 2,777 MiB for telperion, under the bar by about 800 MiB. That assumes the saving comes off the top of the memory, which is inferred, not measured: it holds if the smaller chunks land where the larger ones did. The bar needs only 24 MiB of the 831 to come off the top. **The bar is in reach; no stop.** Task 6 measures it.

The trace patch was applied, built into a scratch target and reverted; the crate was never committed patched.
