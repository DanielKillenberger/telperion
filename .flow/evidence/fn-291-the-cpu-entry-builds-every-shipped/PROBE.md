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

## Task 6: the entry returns the chunks, normals and coords compact

The wood is compact from the start. The mesh step sums each run's normals in floats, then writes each chunk's normals octahedral in two snorm16 (4 B a vertex, from 12). The ring step writes bark coords in two unorm16 (4 B, from 8): the distance along over the wood's longest, the angle over a turn. The CPU entry returns each chunk's positions, oct normals and `u16` indices (`chunk_ptr`/`chunk_len`, `surfaceChunks` in the metadata); the flat slots 0, 1, 2 and 14 are gone. The bounds are tested in `surface::encode::tests`: a decoded normal within 1e-4 rad over 200,000 directions and the octahedron's folds, a decoded coord within half a code.

`probe-dist.mjs` on the rebuilt `dist/`, one fresh engine per case, surface and foliage. Raw rows: `raw/probe-task6.jsonl`. Wasm memory in MiB, against the 3,584 MiB bar:

| case | task 5 | task 6 | chunks | outcome |
|---|---|---|---|---|
| telperion 1 | 2,132 | 1,640 | 658 | built, 1,985,625 nodes |
| telperion 7 | 3,608 | **2,777** | 1,114 | built, 3,358,968 nodes |
| telperion 42 | 3,608 | **2,777** | 1,113 | built, 3,359,470 nodes |
| laurelin 1 | 2,100 | 1,639 | 616 | built, 1,859,572 nodes |
| laurelin 7 | 3,567 | 2,783 | 1,037 | built, 3,127,437 nodes |
| laurelin 42 | 3,583 | 2,755 | 1,042 | built, 3,143,212 nodes |

Every case builds under the bar. Telperion at seeds 7 and 42 is 807 MiB under it, the 2,777 MiB task 5 projected: the 831 MiB R2b saves came off the top. `npm run test:dist` builds Telperion seed 7 through `dist/telperion.js` in 1,114 chunks and checks each one.

## Task 3: growth's tables, narrowed losslessly

### Before (measured at 293f6225b, no code changed)

`tablesize-patch.py` is a throwaway: applied, built into the driver `tablesize.rs` and reverted, never committed. It sums `capacity() * size_of` over every table the grower holds, after each cycle and as the settle begins, beside the live native heap. Raw rows: `raw/tablesize-base.jsonl`. For telperion seed 7 natively, the tables peak as the settle begins: 1,410,199 axes, 4,407,165 phytomers, 2,541 MiB of tables in a 2,543 MiB heap. So the tables are the whole growth heap; the build's native peak, 3,003 MiB, comes later, in the settle and the conversion.

| table | native MiB at the peak | element, native B | element, wasm32 B |
|---|---|---|---|
| `axes[].phytomers` (`Phytomer`) | 805 | 136 | 136 |
| `axes` (`Axis`) | 528 | 264 | 224 |
| `sketch.pencils` (`Pencil`) | 366 | 192 | 168 |
| `draws` (`Draws`) | 304 | 152 | 112 |
| working set's `by_node` | 55 | | |
| `draws[].units` (`Unit`) | 49 | 32 | 32 |
| `successor` | 48 | | |
| working set's `links` (`Link`) | 48 | 24 | 12 |
| retirement's `log` | 48 | | |
| `draws[].nodes` | 47 | 8 | 8 |
| the rest, 22 tables | 241 | | |

The wasm32 sizes were read from a wasm32 build of the same `size_of` calls (`wsizes`, scratch). On wasm32 `usize` is already 32 bits, so every index the scout read as narrowable to `u32` is already that width where the bar is measured. What can shrink on wasm32 is padding:

- `Axis.joined: Option<Join>` is 32 B on every axis and set only on a relay that joins its parent: 2,449 of 365,865 kept axes for telperion seed 7, none while the tree grows. Behind a box it is 4 B: `Axis` goes from 224 to 192 B (native 264 to 232).
- `Draws.outlived: Option<f64>` spends 8 B on its tag. Kept as an `f64` and a flag beside the other flags, `Draws` goes from 112 to 104 B (native 152 to 144).
- Nothing else shrinks a table on wasm32 without narrowing a value no bound in code proves. `Phytomer`, the largest table, is eleven `f64`s and the `u64` key plus `cycle: u32`, whose 4 B pad to 8 at any narrower width. `Pencil` reaches 160 B only with its `units` and `laid` under 16 bits, which no bound proves. `Link.parent` as a sentinel saves 4 B a link (8 MiB at the peak) for about 80 changed lines, so it is left. A `u32::MAX` sentinel for `apex_end`, or `pa` as a `u8`, saves no byte on wasm32 once `joined` is boxed: the 4-byte group stays 48 B.

Projected saving on wasm32 for telperion seed 7, at the axes' capacity of 2^21: 64 MiB for `Axis` and 16 MiB for `Draws`, about 80 MiB of the 2,693 MiB growth leaves (3%). The capacity slack is larger than any narrowing: the phytomer `Vec`s hold 805 MiB for 572 MiB of phytomers, and `axes`, `pencils` and `draws` sit at 2^21 for 1.41M axes. Reserving them is not narrowing; it is reported here for the host.

Post-growth Wasm memory: the `grown` mark of `trace-pages.py` on a traced build of the base binding, one fresh engine per case, surface and foliage. Raw rows: `raw/grown-base.jsonl`. Output hashes of every preset (`hash-dist.mjs`, updated for task 6's chunks): `raw/hash-t3-base.txt`.

| preset | seed 1 | seed 7 | seed 42 |
|---|---|---|---|
| ordinary | 80 | 105 | 126 |
| oregon-white-oak | 289 | 361 | 319 |
| norway-spruce | 1,245 | 1,215 | 1,204 |
| european-beech | 79 | 137 | 169 |
| silver-birch | 170 | 319 | 262 |
| date-palm | 1 | 1 | 1 |
| telperion | 1,519 | 2,693 | 2,740 |
| laurelin | 1,445 | 2,578 | 2,618 |

### What was narrowed

Two layouts changed. Neither narrows a value, so neither needs a range bound. No float, no lineage key and no index changed width.

- `Axis.joined` is `Option<Box<Join>>`. `Axis::joined()` still returns `Option<Join>`.
- `Draws.outlived` is an `f64` and a flag, read through `Draws::outlived()` and set by `Draws::outlive`. The callers now build a fresh axis's draws with `Draws::made`, because the private fields rule out `..Draws::default()` outside the module.
- The test `grow::tests::an_axis_and_its_draws_stay_compact` holds the native sizes at 232 and 144 B. On the base they are 264 and 152 B.
- A wasm32 build of the same `size_of` calls gives `Axis` 192 B and `Draws` 104 B, as projected.

### After (measured on the narrowed tree)

Native tables, telperion seed 7, as the settle begins. Raw rows: `raw/tablesize-after.jsonl`.
- The tables go from 2,541 MiB to 2,461 MiB (-80): `axes` from 528 to 464, `draws` from 304 to 288.
- The build's native heap peak goes from 3,003 MiB to 2,880 MiB (-123).

Wasm memory after growth, MiB, before → after. Raw rows: `raw/grown-after.jsonl`.

| preset | seed 1 | seed 7 | seed 42 |
|---|---|---|---|
| ordinary | 80 → 81 (+1) | 105 → 101 (-4) | 126 → 122 (-4) |
| oregon-white-oak | 289 → 290 (+1) | 361 → 341 (-20) | 319 → 298 (-21) |
| norway-spruce | 1,245 → 1,239 (-6) | 1,215 → 1,230 (+15) | 1,204 → 1,198 (-6) |
| european-beech | 79 → 77 (-2) | 137 → 133 (-4) | 169 → 165 (-4) |
| silver-birch | 170 → 162 (-8) | 319 → 308 (-11) | 262 → 250 (-12) |
| date-palm | 1 → 1 (+0) | 1 → 1 (+0) | 1 → 1 (+0) |
| telperion | 1,519 → 1,487 (-32) | 2,693 → 2,652 (-41) | 2,740 → 2,646 (-94) |
| laurelin | 1,445 → 1,420 (-25) | 2,578 → 2,584 (+6) | 2,618 → 2,621 (+3) |

**The saving is small, and it does not follow the table bytes case by case.** The tables shrink by about 3%. Wasm memory after growth falls by up to 94 MiB (telperion seed 42), and in three cases it rises by 3 to 15 MiB (laurelin seeds 7 and 42, the spruce at seed 7).

The likely reason is that the memory after growth is set by where the allocator places the doubling `Vec`s, not by their sum. That is inferred, not measured. The build's final high-water does not move for telperion: 2,776 MiB at seed 7 and 2,777 MiB at seed 42, before and after. The wood stage sets it, not growth.

Byte identity:
- `hash-dist.mjs` gives the same hash for all 8 presets at seeds 1, 7 and 42 (24 of 24): `raw/hash-t3-base.txt` and `raw/hash-t3-after.txt`.
- A native driver (scratch, `joins`) compares the grown tree's `telperion_space::digest` and a hash of every relay's join, bit for bit, on a base worktree and on the narrowed checkout. Both are equal for all 24 cases: `raw/digest-t3-base.txt` and `raw/digest-t3-after.txt`.

The larger lever is still the capacity slack named above: about 230 MiB in the phytomer `Vec`s, and `axes`, `pencils` and `draws` at 2^21 for 1.41M axes. It is not narrowing, so it is left to the host.

## Task 4: the proof, and the check that keeps it

Measured 2026-10-16 on fff95943e (tasks 1, 2, 3, 5 and 6 landed), `npm run build` dist, Node v26.8.1.

### All 24 cases, through `npm run test:dist`

`scripts/test-dist.mjs` now builds every shipped preset at seeds 1, 7 and 42 through `dist/telperion.js`, surface and foliage, each case in its own child process with a fresh engine. Each case checks its wood chunks and fails when `memoryBytes` passes 3,758,096,384 (3.5 GiB, 3,584 MiB). A failing case is named as `telperion.js: <preset> seed <n>: <cause>`, and all failures are listed before the script exits nonzero. The field entry's WebGPU refusal check is kept. The run took 4 min 3 s on the owner's desk (15:21:22 to 15:25:25 under `telperion-heavy 6 --threads 2`). Raw: `raw/test-dist-24.log`.

| preset | seed 1 | seed 7 | seed 42 |
|---|---|---|---|
| ordinary | 81 | 108 | 126 |
| oregon-white-oak | 291 | 342 | 298 |
| norway-spruce | 1,240 | 1,231 | 1,199 |
| european-beech | 98 | 172 | 210 |
| silver-birch | 202 | 309 | 251 |
| date-palm | 3 | 3 | 3 |
| telperion | 1,641 | **2,777** | **2,777** |
| laurelin | 1,640 | 2,789 | 2,757 |

Wasm memory in MiB after the build, one engine per case. All 24 build; the highest is laurelin at seed 7, 795 MiB under the 3,584 MiB bar. Node counts are R1's, unchanged. Against R1's table, every preset's memory fell but the date palm's, 3 MiB before and after (the spruce went from 2,036 to 1,199 MiB at seed 42), and the four failures now build.

The failure message was exercised by hand: `node scripts/test-dist.mjs --case nope 1` prints `telperion.js: nope seed 1: Cannot read properties of undefined (reading 'skeleton')` and exits 1.

CI's package job already runs `npm run test:dist` on the dist unpacked from the tarball. No job there sets `timeout-minutes`, so GitHub's 360-minute default holds and `.github/workflows/tests.yml` is unchanged.

### Every other preset's surface, against the base dist

`surface-check.mjs` builds each case in the base dist (`dist-orig`, the pre-chunk 2db376e1e, whose hashes match `raw/hash-base.txt`) and in the new dist, each in its own process. It compares the positions and the indices made absolute in draw order by hash, and each decoded normal with the base's float normal by angle. Raw: `raw/surface-check.log`.

- The 6 other presets at seeds 1, 7 and 42, and telperion and laurelin at seed 1 (20 cases): positions and indices byte-identical in every case.
- The worst decoded normal is 6.47e-5 rad (norway-spruce seed 42), inside task 6's 1e-4 rad bound in every case.
- Bark coords never reach JS through this entry, so there is nothing of theirs to compare here; task 6's `surface::encode::tests` bounds them.

### Stills of the CPU entry's own mesh

`still.mjs` rasterizes the wood a source hands over: orthographic from +z, y up, depth-tested, each triangle flat-shaded (Lambert) by the mean of its three vertex normals, 768 px a panel. Two panels: the whole tree, and the trunk's foot at a twelfth of its height, where a normal's error would show first on wide, smooth faces. The pair shares one camera. `stills.sh` draws them:

- Seed 1: the base flat dist against the new chunked dist.
- Seeds 7 and 42, which the base dist never built: the new dist against the base native flat mesh, written by `flatmesh-base.rs` (built against a worktree of 2db376e1e, wood and leaves requested as the entry asks).

The base native mesh's positions and indices hash equal to the new dist's at seeds 7 and 42 for both trees, so the only difference in those pairs is the normals' encoding. Each sheet is before, after, and their difference ×8 inverted. Raw: `raw/stills/*.png`, `raw/stills-steps.log`; the six sheets together: `stills.jpg`.

| case | whole tree | trunk's foot |
|---|---|---|
| telperion 1 | 104 px differ, max 1 level | 0 px |
| telperion 7 | 97 px, max 1 | 961 px, max 1 |
| telperion 42 | 118 px, max 1 | 0 px |
| laurelin 1 | 106 px, max 1 | 1,027 px, max 1 |
| laurelin 7 | 100 px, max 1 | 0 px |
| laurelin 42 | 168 px, max 1 | 0 px |

No pixel differs by more than one grey level of 255. I viewed all six sheets. **Verdict: unchanged to the eye** for Telperion and Laurelin at seeds 1, 7 and 42.

### Gate and `npm test`

- Gate, run once at 50ea727f2 through the declared invocation (`CARGO_TARGET_DIR` the worktree's `target/`): `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`. 1,412 passed, 0 failed, 103 ignored. It was admitted at 16:20:51 after 55 minutes of waiting (FRICTION.md) and finished at 16:58:20. Raw: `raw/gate-t4.log`.
- `npm test`: 19 files, 170 tests passed. Raw: `raw/npm-test-t4.log`.
- Not run: `npm run rust:test:wasm`. Its suite includes a browser field test, and the host excluded it for this task, so the bindings suite is not run here.

## After merging the release tip (2026-10-18, 6473b5e9e)

The release tip 0e3c8e30d (fn-257 R8 and others) was merged in with no conflicts. No file changed on both sides.

- Gate (`telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`): 1,412 passed, 0 failed, 109 ignored, 02:29:37 to 03:00:19 (`raw/gate-merge.log`).
- `npm test`: 19 files, 170 of 170 tests passed (`raw/npmtest-merge.log`). `npm run build` passed.
- `npm run test:dist`: all 24 cases built, each at or under 3,584 MiB, with every high-water mark equal to task 4's table (highest laurelin seed 7 at 2,789 MiB). `field.js` refused without WebGPU (`raw/testdist-merge.log`).

Completion review (codex, base the release branch): NEEDS_WORK on R2's error clause. Telperion seed 7 at a valid `surface.radialSegments` of 64 reaches 4,294,705,152 B of Wasm memory and throws the bare `Tree core: surface allocation`. It is not refused before allocation by a named cause. The wood's exact vertex count is known at `Layout::new` (`surface/rings.rs:165`), before any chunk is reserved. What is not settled is the budget it would be checked against. Freed heap is invisible from inside Wasm, so a check against 4 GiB minus the memory grown so far would refuse Telperion seed 7 at its preset settings, a shipped case. That is a ceiling like host decision 15's phytomer ceiling. It is taken to the owner/host and not invented here.

## Task 7: R2f, the wood refused by name before allocation (2026-10-18)

The Wasm binding wraps its global allocator in a live-byte count (`telperion-wasm/src/live.rs`). It installs `4 GiB - __heap_base - live` as the core's bytes-left source (`telperion_core::memory`). Right after `Layout::new`, before any chunk is reserved, `surface::budget::check` sums what the drawn wood then reserves from the layout's own counts. That is positions (12 B), oct normals (4 B), bark coords (4 B) or ring distances, `u16` indices, the chunk lists, the run tables and spans, and the mesh step's float normals for its longest run, plus one full chunk's positions (768 KiB) of headroom. A need over what is left is refused as `Error::WoodMemory`, with both numbers and the remedy. Natively no source is installed, so the check is skipped. There is no fallback path. Every raw `try_reserve` failure in the surface stage now reads "the wood ran out of memory while its buffers were reserved; build it through the device entry or with fewer radialSegments".

`npm run test:dist` on the task's dist (raw: `raw/testdist-t7.log`):

- All 24 shipped cases build, each high-water mark equal to the table above (highest laurelin seed 7, 2,789 MiB).
- telperion seed 7 at `radialSegments` 64 is refused at 2,653 MiB of Wasm memory: `Tree core: the wood needs 7411185660 bytes and 3679383462 are left in 32-bit Wasm memory; build it through the device entry or with fewer radialSegments`. Before the task it reached 4,294,705,152 B and threw the bare `Tree core: surface allocation` (the completion review's repro).
