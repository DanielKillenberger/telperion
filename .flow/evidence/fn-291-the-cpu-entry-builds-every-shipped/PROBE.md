# fn-291 R1: every shipped preset through the dist CPU entry in Node

Measured 2026-10-16 on 6a6d5abcb + the fn-291 plan commits (no generator or entry code changed), Node v26.8.1, `npm run build` dist. `probe-dist.mjs` runs each case in its own child process under `telperion-heavy 6 --threads 2`. Each child creates a fresh `TreeEngine`, builds the preset at its preset age with `skeleton.seed` set, and reads `engine.memoryBytes` (Wasm linear memory, which only grows, so it is the case's high-water mark). Raw rows: `raw/probe.jsonl`. A failing case is re-run as surface-only and as foliage-only.

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
| telperion | 82 | 7 | both | **Tree core: surface allocation** | - | 4078 | 29.0 |
| telperion | 82 | 7 | surface | **Tree core: surface allocation** | - | 4079 | 27.7 |
| telperion | 82 | 7 | foliage | built | 3358968 | 2694 | 27.5 |
| telperion | 82 | 42 | both | **Tree core: surface allocation** | - | 3571 | 27.8 |
| telperion | 82 | 42 | surface | **Tree core: surface allocation** | - | 3571 | 24.0 |
| telperion | 82 | 42 | foliage | built | 3359470 | 2740 | 25.6 |
| laurelin | 80 | 1 | both | built | 1859572 | 3586 | 17.1 |
| laurelin | 80 | 7 | both | **Tree core: surface allocation** | - | 3352 | 23.2 |
| laurelin | 80 | 7 | surface | **Tree core: surface allocation** | - | 3352 | 21.8 |
| laurelin | 80 | 7 | foliage | built | 3127437 | 2578 | 22.0 |
| laurelin | 80 | 42 | both | **Tree core: surface allocation** | - | 3396 | 24.0 |
| laurelin | 80 | 42 | surface | **Tree core: surface allocation** | - | 3396 | 23.0 |
| laurelin | 80 | 42 | foliage | built | 3143212 | 2618 | 23.0 |

There were 4 failures in 24 cases: telperion and laurelin at their preset ages, seeds 7 and 42. Every failure is the named `ResourceLimit("surface allocation")` (the `Tree core:` error), never a trap or a `RangeError`. Every failure is in the surface stage; foliage-only builds the same trees. The other six presets build at all three seeds, with at most 2,036 MiB (the spruce). No preset is refused "no engine species". At about 3.36M nodes, the failing trees sit under the existing wasm32 phytomer ceiling (`WASM_KEPT`, 4,000,000), so that ceiling does not refuse them before growth.

The README's first example, telperion at seed 7, is one of the failures.

## Cause

**The allocation.** `reserved()` (`crates/telperion-core/src/pipeline/surface.rs:64-69`) is `Vec::try_reserve_exact`. When the request cannot be satisfied, it maps the failure to `ResourceLimit("surface allocation")`. On wasm32 that means the 4 GiB linear memory cannot grow far enough.

**The order on the wasm path.** The parallel sweep is linux x86_64 only, so wasm takes the serial path.
- `rings()` reserves the wood's `positions` and `coords` (`surface/rings.rs:139-140`).
- `faces()` then reserves `normals` and `indices` (`surface/build.rs:112-113`, with `indices` = `ring_vertices * 6`).

The failing request is one of the two `faces()` reservations. Seed 7 fails with the memory at 4,078 MiB, against a 4,096 MiB maximum. Seed 42 fails at 3,571 MiB, where an `indices` request of about 1.6 GiB cannot fit. That `faces()` is the site is inferred from the sizes; the probe does not name which of the two reservations failed.

**What sized it.** `extent_of()` (`surface/build.rs:21-50`) counts one ring per drawn sample. Each ring has `max(radialSegments, 4·lobes)` sides (`surface/build.rs:14-15`). Every shipped preset has radialSegments 12 and lobes 5, so every ring of every branch has 20 sides, twigs included. The wood is about 21.6 vertices per node.

**Measured sizes.** A native trace (`woodsize.rs`, built against this tree's `telperion-core`) makes the same request (surface only) with a counting allocator. Its native node counts match the wasm ones exactly. Raw output: `raw/woodsize.log`.

| case | nodes | wood buffers MiB (positions + normals + coords + indices) | largest single buffer MiB | heap peak MiB |
|---|---|---|---|---|
| telperion 82, seed 1 | 1,985,625 | 2,286 | 976 (indices) | 2,591 |
| telperion 82, seed 7 | 3,358,968 | 3,867 | 1,651 (indices) | 4,476 |
| laurelin 80, seed 7 | 3,127,437 | 3,601 | 1,537 (indices) | 4,209 |

Breakdown for telperion at seed 7:
- positions: 217,870,704 floats (831 MiB)
- normals: 831 MiB
- coords: 554 MiB
- indices: 432,771,840 u32 (1,651 MiB)

The wood's four buffers alone are 3,867 MiB, which is 94% of the 4 GiB address space. The whole build's live heap peaks at 4,476 MiB (telperion seed 7) and 4,209 MiB (laurelin seed 7), both more than 4 GiB. So no allocator and no estimate makes these builds fit as they are drawn. The cause is neither a sizing estimate nor one allocation's cap: the drawn wood needs more memory than 32-bit Wasm has.

Seed 1 builds because its tree is 41% smaller (1.99M nodes against 3.36M). Even so, it reaches 3,805 MiB of Wasm memory against a 2,591 MiB native peak. The difference is growth slack and fragmentation in the Wasm allocator.

**Why nothing caught it earlier.** The phytomer ceiling was measured on the spruce at seed 42 (fn-198 BROWSER.md). It bounds kept phytomers, not the wood's bytes. The native entry test, `every_shipped_preset_builds_through_the_main_entry`, runs on the host allocator and builds these trees without complaint.

## What R2 leaves open (for the host)

This is a design choice, and task 1 does not make it. The options the measurements leave:

1. **Refuse by name.** Add a wasm32-gated prediction of the wood's bytes (`extent_of(...).bytes()` plus the tree) before `rings()` reserves anything, following `OVER`/`foreseen`/`fits`. Its message would name the cause and the remedy (grow it younger, or ask for foliage only, or use a native build). This refuses two shipped presets at their preset ages for two of three seeds, including the README's first example, and needs the owner's word.
2. **A memory fix that keeps the drawing.** Lower the peak below about 3.5 GiB of Wasm memory for these trees. For example, drop the tree's and the sweep's working state before `faces()`, or stop holding `coords` (554 MiB), or emit the index buffer without a full reservation. To fit with the observed allocator slack, the peak for telperion at seed 7 must fall by at least about 1 GiB from 4,476 MiB. Even then the wood alone is 3,867 MiB, so the headroom is thin and later seeds may exceed it.
3. **A geometry change.** Draw fine wood with fewer sides. At present the `4·lobes` floor puts 20 sides on every twig. This changes the drawn tree (allowed under "Generator evolution" with a visual check), and it is generator design.
4. **Lower the two presets' ages.** Telperion is at 82 and laurelin at 80. This changes shipped presets and is the owner's call.
5. **memory64 or a second binding.** Out of scope under the spec's Boundaries.

Options 1, 3 and 4 change what a shipped preset promises. Option 2 is the only one that changes nothing the owner set, but the measured margin says it may not hold for every seed.
