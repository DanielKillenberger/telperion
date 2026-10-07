# fn-198 task 3: the harness and the browser through the engine

The dev harness (`vite` on this branch, the modules `npm run dev` builds) driven headless by `harness/drive.mjs`: Chromium 1243 on the desk's RTX 3080 (native Vulkan with ANGLE on it; see FRICTION.md), 1280x900, every step under a timeout. Each tree goes through `setTreeGpu` and the engine as stage 2. No full-forest capture was taken. Screenshots and per-run logs are in `raw/task3/` (ignored).

## What opens

| Tree | Age | Backend | Foliage | Viewed |
|---|--:|---|--:|---|
| European beech (the harness's opening tree, seed 1) | 40 | GPU | 65,665 | `raw/task3/beech-40.png` |
| Oregon white oak | 40 | GPU | 166,264 | `raw/task3/oak-40.png` |
| Norway spruce | 40 | GPU | 4,610,729 | `raw/task3/spruce-40.png` |
| Date palm | 40 | CPU fallback (as natively) | 6,554 | `raw/task3/palm-40.png` |
| European beech, seed 2, by the age dial | 80 | GPU | 4,493,162 | `raw/task3/beech-80.png` |
| Norway spruce, by the age dial after 70 and a refused 80 | 60 | GPU | 10,536,178 | `raw/task3/walk-spruce-60.png` |

- The harness opens on the engine's beech at 40 (`OPENING_PARAMS`), and every engine preset carries `/age = 40`. The age dial runs 1 to 80 and rebuilds through the same `setTreeGpu` call as any dial; a link may carry `?age=`.
- Silver birch, laurelin, telperion and ordinary each show "the generator rejected the tree: invalid input: no engine species" (`raw/task3/refused-silver-birch.png`, `refused-ordinary.png`), about 40 ms after the click. A failed build now hides the canvas's tree, so the error stands instead of the tree before it.

## The browser's ceiling (host decision 5)

A Wasm trap (`RuntimeError: unreachable`) leaves the build's promise unsettled: the panel said "building..." for ever. That is the hang a driver without a timeout meets.

Measured in fresh pages at seed 42; kept phytomers from the native engine (`harness/grown_count.rs`):

| Spruce age | Kept phytomers | In the harness |
|--:|--:|---|
| 60 | 2,927,243 | builds, 9.6 s |
| 70 | 3,953,068 | builds, 13.6 s |
| 72 | 4,164,440 | builds, 20.5 s |
| 73 | 4,267,676 | builds, 21.0 s |
| 75 | 4,479,766 | traps |
| 80 | 4,983,271 | traps |

The beech at 80 keeps 2,771,604 and the oak at 80 1,680,455; both build. The phytomers grown, the shed ones included, cannot draw the line: the oak at 80 grows 7.5M and fits, the spruce at 75 grows 5.0M and traps.

**The rule:** on 32-bit Wasm a grown tree that keeps more than 4,000,000 phytomers is refused by name before any of its preparation is allocated (`pipeline/space.rs`, `WASM_KEPT`), with "resource limit: the tree keeps more than 4,000,000 phytomers, more than a browser prepares in its 4 GB Wasm memory; grow it younger". Natively there is no ceiling. Each species' ceiling age under the rule, at seed 42: the spruce 70 (it keeps 4,058,956 at 71 and is refused); the beech, the oak and the palm were built at 40 and 80 and are not refused at 80.

- **Checked in one page:** the spruce at 40, then 70, then 80 (refused, in 6.4 s), then 60 (built); no trap (`raw/task3/walk-spruce-80-refused.png`).
- **Not "before the build starts":** the refusal comes after the engine's growth, before conversion and preparation. The spruce at 80 grows for 6 to 8 s in the browser before it is refused. Refusing before growth needs a prediction of the kept count (the closed form's kept tables hold one, unexposed); that is a design question for the host, below.
- **The main module** (`telperion.wasm`, every output, `harness/main-module.mjs` in Node) refuses earlier on its own: the beech at 80 and the spruce at 60 and 70 stop with "surface allocation", the spruce at 80 with the new message; none traps. The spruce at 40 builds (2.9 GB).

## Parameter change to first drawn frame

From the change in the panel to the second animation frame after `setTreeGpu` resolved, in fresh pages, 05:38 to 05:40, load 2.8 to 3.7 on 32 threads, the GPU otherwise idle. Wasm runs the engine on one thread. A dial waits 250 ms after the last notch when the last build took over 80 ms; the times include it.

| Tree | Seed change at 40 | Age 40 to 80 |
|---|--:|--:|
| European beech | 552 ms | 5,743 ms (stage 2 2,668 ms) |
| Oregon white oak | 1,615 ms | 17,544 ms (stage 2 15,104 ms) |
| Norway spruce | 4,834 ms | refused in 8,326 ms; to 70: 15,635 ms |
| Date palm | 366 ms | 47 ms |

- The oak's growth under light dominates at 80, as natively (TIMING.md).
- One slow case is open: in one page the spruce at 60 after the spruce at 70 and a refused 80 took 66 s (stage 2 4.0 s, preparation 60 s), against 9.6 s fresh. A rerun to separate the page's memory from the GPU's could not take the GPU lock (FRICTION.md).
- Separately, the beech at 80 was once refused "Out of Memory" by the GPU while another worktree's test held 4.7 GB of the 10 GB card outside the lock; it built when the card was free.

## Module sizes (host decision 6)

`npm run build` on this branch against `c11e5d85` (fn-198's base: master with F1 and F2b, before the engine became stage 2), same toolchain:

| Artifact | Base | Branch | Engine adds | Ceiling |
|---|--:|--:|--:|---|
| `telperion.wasm` | 786,373 | 1,143,604 | +357,231 | 1,600,000, unchanged |
| `telperion-render.wasm` | 1,512,937 | 1,862,516 | +349,579 | 2,200,000, unchanged |
| `telperion-field.wasm` | 323,190 | 632,580 | +309,390 | **raised** from 400,000 to 632,580 |
| `telperion.js` | 113,894 | 153,565 | +39,671 (the engine rows in the presets) | **raised** from 150,000 to 153,565 |

The two raises are on this branch only and are the owner's decision (`scripts/artifact-budgets.json` says so in each reason). Each also crosses the 5 percent growth share per PR, so a PR must name all three Wasm modules and `telperion.js` in its Decisions section.

## For the host

1. **Refuse before growth or after it.** Today the refusal follows growth (up to 8 s for the spruce at 80). Options: (a) keep it; (b) expose the closed form's expected kept count and refuse on it before growing, at the cost of the closed form's read and its approximation; (c) a per-preset browser age row, which docs/principles.md's "by degree" argues against.
2. **The ceiling's margin.** 4,000,000 sits 6 percent under the largest tree measured to build (4.27M, the spruce at 73), so the spruce stops at 70 though 73 fits in a fresh page; it is measured on the spruce only, at one seed.
3. **The slow second build** above, before the owner drives the spruce through its ages.
