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
