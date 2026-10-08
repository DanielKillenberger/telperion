# fn-212 task 3: progress (worker, 2026-10-07)

The host's decisions D1' to D5 against what is built. The native probe is not started. D1' is recorded, with its measurements, in TASK3-D1.md.

## Built: D4 and D5 (`7afc0499`)

- **Fine roots are marked.** `telperion_space::grow_marked` marks the axes the engine grew in fine roots: the stand-in axis each root's first axis replaces, and every axis appended past the structure above the cut. The marks are carried through the canonical lineage order. A tree grown whole marks none: the oak, which reads light, and the palm.
- **The conversion keeps each fine root's first drawn node as a seam** (`Tree::seams`, sorted, never stored, as `sections`).
- **D4:** `paths` never carries a run on into a seam. The fine root's wood starts a run of its own and meets its bearer at the socket every branch run has. A node whose children are all seams ends its run, and each child seeds its own.
- **D5:** the ranking sorts the structure above the cut's runs first, then the fine roots', widest first within each; no sort reaches across the seam. The mesh suite's order test now holds on each side. `docs/pipeline.md` says so.

### What it changed (seed 1 unless named)

| Tree | Nodes | Seams | Curve runs | Curve points before → after |
|---|--:|--:|--:|---|
| beech 80 s1 | 1,525,191 (same) | 4,591 | 122,177 | 1,647,289 → 1,647,368 (+79 runs) |
| beech 80 s7 | 2,532,841 (same) | 7,407 | 203,036 | 2,735,730 → 2,735,877 |
| spruce 80 s1 | 4,637,482 (same) | 70,592 | 673,758 | 5,310,918 → 5,311,240 (+322 runs) |
| spruce 80 s7 | 4,610,322 (same) | 70,033 | 675,507 | 5,285,504 → 5,285,829 |
| oak 40 and 80, palm | same | 0 | same | unchanged |

- **Node counts are unchanged:** no identity moved.
- **A small share of the seams was in a crossing run.** 322 of the spruce's 70,592 seams and 79 of the beech's 4,591 continued their bearer's run before; each now starts its own run with a socket.
- **The run order changes on every beech and spruce,** and so do the bytes of their curve and wood.
- **Stills at 80, seed 1, before (`2e85549e`) and after, all four viewed:** indistinguishable at the hero view (`raw/task3/still-*`). Leaf counts are the same, and the wood triangle counts move by +3,160 (beech) and +12,880 (spruce).
  - The headless still has only the hero pose, so the seams themselves were not inspected up close; that close look needs a camera nearer the crown.

## Not built

- **D2 (counts from record data):** today only the CPU converts and classifies, so both counts are already single-sourced. Per the host they read today's radius until contract A. Nothing changes until the device classifies fine wood in the probe; it is the probe's step 6.
- **D3 (ground-routed roots never reach the device): a design question.** A root's routing is decided in phase 3, from its bearer's laid node (`fine/landing.rs`, checked unbent in phase 3 and from either frame in phase 4). Every root is uploaded and grown in phase 1, before that node exists. "Excluded before upload" therefore needs either:
  - a routing bound that reads no laid geometry (for example the reach from the bearer's unbent node, or a bound on the whole tree's height), which is conservative and moves roots to the CPU; or
  - the phase-1 upload split into the roots routed by such a bound and the rest.
- **The native probe (R3): not started.** CONTRACT.md's steps 2 to 7 are each a device kernel or pass not in the code today:
  - fine nodes assembled in the tree's frame;
  - the curve's sweep, frames and clusters per run;
  - the five whole-tree reductions with the shoulder's front;
  - stations per bearing run;
  - contact rings from curve points;
  - a `submit_prepared` that takes device buffers.

  Against baseline B (spruce at 80, 8.1 s; the gate at about 2.7 s) the CPU work these steps replace is:
  - curve and rings: 2.6 s;
  - stations: 2.1 s;
  - conversion: 1.0 s;
  - placement: 0.9 s (already on the device).

  This is several tasks' work, not one. Splitting it into tasks (fine nodes and reductions; the curve's sweep; stations and contacts; the resident handoff and the timing harness) is the host's call.

## Gate

- The two-part gate after D4 and D5 is green: the workspace less the render crate 987 passed (fn-198's merge closed the inventory red), and the render crate under the GPU lock 158 passed. `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.
- npm test was not re-run after D4 and D5; the browser's binding builds the same pipeline.
