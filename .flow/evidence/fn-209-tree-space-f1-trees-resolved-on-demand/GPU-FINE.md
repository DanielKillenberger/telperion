# fn-209 F1 task 2: fine roots on the GPU executor (worker, 2026-10-06)

Task 2 built host decision 1 on the CPU reference, then the GPU executor's growth and shedding of fine roots. Geometry on the GPU (scale, girth, lay) and the device-resident handoff are not built; the second needs a host decision (section 5).

## Answer in brief

- **Host decision 1 is built** (`telperion-space/src/fixed.rs`, `fixed/decide.rs`; FINE-CONTRACT.md, "Host decision 1 built").
  - Time is ticks of 2^-20 cycle. Every per-bud threshold (D5, D6, D10, D11, D15, D18) is one integer procedure.
  - Topology is identical in all 28 dumped trees. Tips move at most 24.5 µm and radii at most 1.56 %, on woken buds.
  - Stills are indistinguishable; the worker viewed all of them.
- **The contract runs in WGSL bit for bit** (`telperion-render/src/generation/fine/wide.wgsl`, `contract.wgsl`).
  - Every function was run over 262,144 cases each, the threshold-adjacent and saturating ones included. The GPU's bits equal the CPU's in every case.
  - A one-bit change to one constant fails the test. fn-12's failure mode, f32 against f64, cannot occur, because no decision reads a float.
- **Fine roots grow and shed on the GPU, with topology by lineage identical to the CPU's.**
  - The work splits into a chain grower, dormancy, shedding and relay nodes.
  - Each root is expanded wave by wave: the buds of one wave's kept axes become the next wave's chains.
  - The trees compared:
    - the beech, spruce and oak (light off) at 25, seeds 1 and 7;
    - every chain of five adversarial variants at 25, the fractional lifespans among them;
    - the beech and spruce at 40 and 80, seeds 1 and 7.
  - Each was compared on the botanical tree before conversion (host decision 2), forward and reversed, in batches of 65,536, 4,096, 1,000 and 777.
- **The overflow rule is the chain.** An item is split along its chains, each grown from its own bud's record. A chain that outgrows its region is grown again in a region eight times larger, which grows the same chain.
- **Timing, for growth and shedding only** (load 0.2 to 4):
  - Spruce at 80, 92,703 roots and 951k chains: **about 165 ms warm and 280 ms cold** on the GPU, uploads included.
  - The same chains on one CPU core take 6.8 to 7.4 s.
  - The CPU executor's whole tree on all cores takes 1.1 s, and that includes geometry.
  - The beech at 80 takes 82 to 114 ms warm on the GPU.
- **Not built:**
  - scale, girth and lay on the GPU;
  - the handoff of fine geometry to rings, stations and curves on the device. No device-side stage of that kind exists yet; see section 5.

## 1. Host decision 1

See FINE-CONTRACT.md, "Host decision 1 built": the procedure, the table of decisions, what it moves and the stills. Raw data is in `raw/fixed/` (gitignored).

## 2. The GPU executor (`crates/telperion-render/src/generation/fine/`)

| File | What it is |
|---|---|
| `wide.wgsl` | u64, i64 and 128-bit products in 32-bit words |
| `contract.wgsl` | `fixed.rs` and the integer forms of `fixed::decide`, plus D1's hash and draw |
| `zones.wgsl` | a growth unit's zones: node counts (D2), acrotony (D4), node leads, bud PAs (D3) |
| `unit.wgsl` | a unit kernel: a tree's units drawn on their own (a probe, held against every unit of the grown trees) |
| `chain.wgsl`, `apex.wgsl`, `sleep.wgsl`, `shed.wgsl` | P2's chain grower, one invocation per chain: survival, abortion and the schedule in ticks; stops, moves and relays, with `carry_on`'s same-cycle growth kept in order by an explicit stack; sleeping buds and their waking; a woken bud's chain; shedding and the buds' contexts; relay nodes |
| `pack.wgsl` | packs the chains' regions end to end, so a reader takes only what grew |
| `chains.rs`, `roots.rs` | the host side for the tests: tables, buds, dispatch in regions, overflow, and the wave expansion of roots |

On the CPU side, `telperion_space` gains the executor's surface. `ChainTables` holds a species point's tables as integers: PA rows, unit tables, log woods and extinction curves. `chain_cases` and `fine_cases` return every chain or fine root with its bud and its topology as the CPU reference grows it. `Fine::starts` and `Phytomer::key` complete it. `grow_unit`'s node wood is now one shared function (`grow::node_wood`), so the tables are built from the grower's own rule.

The shaders run only in tests for now (`#[cfg(test)] mod fine`). No product path reads them until the executor's geometry and its handoff exist.

## 3. Equality (R1)

| Comparison | Trees | Result |
|---|---|---|
| contract functions | 12 functions × 262,144 cases | bit-identical |
| unit kernel against the grown tree's phytomer keys and laterals | beech and spruce 40 and 80, seeds 1 and 7 | every unit's nodes in order; every lateral on the place that drew its PA |
| every chain (axes, phytomers, apex ends, relay nodes, woken buds, kept, contexts) | beech, beech fractional, spruce, oak light off, oak light off fractional, at 25, seeds 1 and 7; beech and spruce at 40 and 80 (topology) | identical; forward in batches of 4,096, reversed in batches of 333 or 1,000 |
| every fine root (kept axes by lineage, before geometry) | beech, spruce and oak light off at 25; beech and spruce at 40 and 80; seeds 1 and 7 | identical; forward and reversed, in batches of 65,536, 4,096 and 777 |

One contract bug was found this way: the GPU's sum of two unbounded logs wrapped before it saturated. The oak's contexts caught it, and `and_q` is now part of the probed contract.

**Not covered: geometry within fn-125's screen.** No geometry is computed on the GPU yet.

## 4. Timing (R2, growth and shedding only)

`cargo test --profile ci -p telperion-render fine_roots_at_40_and_80 -- --ignored --nocapture`, run under the GPU lock, load 0.2 to 4. The machine is a Ryzen 9 5950X with 32 threads and an RTX 3080. The raw output is in `raw/gpu/`.

- The GPU columns run from building and uploading each batch's buffers to the device finishing, and they include the uploads of buds and tables.
- "Read" is packing plus readback, which the device-resident path would not do.
- "Again" counts chains grown a second time in larger regions.
- Pipelines build in about 12 ms.

| Tree | Roots | Chains | Waves | Kept phytomers | Tables ms | GPU cold / warm ms | Read ms | Again | µs a root, warm | CPU grow + shed, every chain, 1 core ms | CPU executor Σ items, 1 core ms (incl. geometry) | CPU whole tree, all cores ms |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 s1 | 553 | 12,405 | 6 | 81,297 | 4 | 20 / 20 | 5 | 105 | 36 | 100 | 87 | 34 |
| beech 40 s7 | 975 | 22,258 | 6 | 141,265 | 4 | 24 / 23 | 7 | 145 | 24 | 142 | 156 | 28 |
| beech 80 s1 | 7,865 | 126,350 | 7 | 847,545 | 15 | 171 / 114 | 44 | 2,802 | 14 | 1,034 | 1,141 | 171 |
| beech 80 s7 | 12,829 | 221,386 | 8 | 1,485,934 | 16 | 257 / 82 | 69 | 4,636 | 6.4 | 1,751 | 1,987 | 277 |
| spruce 40 s1 | 21,307 | 305,626 | 3 | 1,171,257 | 12 | 256 / 52 | 74 | 758 | 2.5 | 1,510 | 2,734 | 308 |
| spruce 40 s7 | 20,444 | 294,003 | 3 | 1,135,099 | 12 | 174 / 52 | 68 | 622 | 2.5 | 1,432 | 2,616 | 288 |
| spruce 80 s1 | 92,703 | 958,770 | 3 | 4,731,672 | 34 | 280 / 166 | 275 | 5,959 | 1.8 | 7,376 | 13,256 | 1,137 |
| spruce 80 s7 | 91,262 | 951,424 | 3 | 4,672,415 | 32 | 279 / 162 | 251 | 5,884 | 1.8 | 6,832 | 12,951 | 1,115 |

What the table says (the worker's readings):

- **Region sizing dominates the GPU's time.**
  - Regions of 256 axes and 1,024 phytomers in batches of 4,096 took the spruce at 80 1,090 ms warm.
  - Regions of 32 and 64 in batches of 65,536 take 165 ms, with 0.6 % of chains grown again.
  - A predictor-sized region per chain is the next lever.
- **One invocation per chain is uneven.** The beech's long chains make it the slowest per root. A workgroup per heavy chain is the lever there.
- **`log2` costs 32 squarings of 128-bit products** in 32-bit words. A table-driven `log2` and `exp2` would be cheaper, specified the same way on both executors.
- **The CPU's grow-and-shed column is P2's every chain on one core**, including the above-cut chains (about 1 % of the spruce's nodes and 5 % of the beech's). No all-cores figure for growth and shedding alone exists on the CPU side.
- Per item, the mean is 1.8 to 36 µs a root, warm. The worst item cannot be timed alone inside a batch. It is bounded by the overflow re-run, a batch of at most 8,192 chains in a region eight times larger.
- **GPU memory** for one batch of 65,536 chains is about 0.45 GB of regions; the uploads are small.
- **Contract primitives**, one batch of 262,144 cases from upload to readback: 2 to 6 ms on the GPU, against 4 to 92 ms on one core (`contract_cost`, ignored test).

## 5. What remains, and the design question

**Not built on the GPU: scale, girth and lay**, meaning option 4's resolution of each chain against its bearer's laid frame and committed boundary. That covers:
- presences and `assign`;
- `scale` and dominance;
- the girth fit to the committed base radius;
- the unbent lay (frame, tropism, keyed wander, straightening, erection, ground);
- sag under the chain's own wood and its borne buds' committed mass and moment.

The CPU source is about 1,800 lines. Geometry reads continuous values only, so it ports to f32 and would be held to fn-125's screen (50 µm max, 10 µm RMS).

**Design question (host): where fine geometry goes on the device (Astra change 3).** The task says "device-resident through to the expansion's existing ring, station and curve stages". Those stages exist on the CPU only: `compact` or `compact_with_contacts`, `stations` and `curve` in `telperion-core` (GPU-PATH.md). The engine's `Structure` reaches the pipeline only through F4's example converter (`tree::convert`), and nothing on the device consumes per-node fine geometry today. Options:

- **(a)** Build device-side ring, station and curve stages for fine roots in F1. This takes over part of fn-171's GPU ring preparation and F4's converter. It is large, and it moves integration ahead of F4, which host decision 4 kept for later.
- **(b)** F1's GPU executor ends at the botanical tree and its laid geometry, resident in device buffers. F4 (fn-198, with fn-171) defines the device-side conversion and preparation that read those buffers. F1 measures to that boundary. This is the worker's reading of the boundaries as decided.
- **(c)** Read fine geometry back to the CPU stages. That breaks Astra change 3, and GPU-PATH.md measured those passes at about 0.2 µs a node each (the spruce at 80: about 3.9 s).

**Also open, unchanged from task 1:** the lit oak stays refused until F2b. The light-scaled viability of whole-tree growth is computed in f64 before the fixed-point share (CPU only).
