# fn-209 F1 task 6: realized mechanics, the CPU reference executor (worker, 2026-10-07)

fn-211's host decision 3 (DESIGN-INTERLEAVE.md, "After Astra's review"): production uses realized mechanics, and option 4's committed boundaries leave production. This file covers the CPU reference executor (built, equal to today's engine to the bit), the committed path's removal, timing, and the one design question that stops the GPU mirror.

## Answer in brief

- **Built on the CPU and equal to today's whole-tree engine to the bit.**
  - The trees: beech and spruce at 40 and 80, seeds 1 and 7.
  - The cuts: 64, 256, 1,024, 4,096 and ∞.
  - Five runs per cut, in four order and chunking settings: forward in pieces of 16 (cold, then warm), reverse in pieces of 1, shuffled in pieces of 7, and forward in pieces of 64.
  - 40 configurations, 200 builds. Every digest equals `grow`'s, over every field in canonical (lineage) order.
- **No new arithmetic.** Every pass is today's own, run on a root's axes, and every sum a root's subtree enters is taken in the whole tree's order. So equality to the bit follows from the structure; it is not a tolerance.
- **The committed-boundary path is out of production.** It is kept as a named probe, `grow_committed` (`fine/committed.rs`), only because the GPU executor's tests still compare against it.
- **Speed: the partition beats today's engine.** It is bit-identical and 3 to 7 times faster on all cores (proposal below).
- **GPU mirror: not built. One design question first (D17, below).** Whole-tree shedding decides topology (`fade <= 0`) from presence-weighted f64 sums. On the fine path that decision now belongs to the executors, and the contract does not yet represent it.

## How it works (`grow/realized.rs`, `fine/realized.rs`, `fine.rs`)

1. **Above the cut.** The structure is grown by the engine's own `Grower` with a hand table: a lateral, or a sleeping bud as it wakes, whose PA and birth are fine is made where it stands and never grown. Its stand-in keeps its index among its bearer's children, so every sum at a bearer's node meets its children in today's order.
2. **Each fine root, phase 1.** The root is grown whole from its bud alone by the same grower (P2's `start`, with `alone` off), then marked and assigned. It is shed by its own realized subtree (`shed_given`). Up goes its `Living`: the subtree's last presence-weighted living time and its most present living apex.
3. **The structure above the cut** is shed on those values (`shed_given`), then scaled. Each root receives its bearer's base scale and its node's drawn scale.
4. **Phase 2.** The root is scaled and thickened; up goes its base radius. The structure above is then thickened on those radii (`thicken_given`) and laid.
5. **Phase 3.** Down comes the bearer's laid phytomer at the node. The root is laid (`place_from`, the stub fixed) and its levers gathered; up goes its base `Load`. The structure above gathers levers on those loads (`levers_given`) and lays again.
6. **Phase 4.** Down comes the bearer's bent phytomer. The root is laid again under its own levers, from its own first lay's frames. This resolves FRAME-CORRECTION.md: the bent pass uses the first-lay frame, as `place` does.
7. **Assembly.** Each kept root's first axis takes its stand-in's index; its other axes are appended.

**Round trips:** three (four item phases) for species that sag, two (three phases) for those that do not. Each trip is one record per root up and one bearer record down, never anything per node.

**Refused here:**
- species that read light (fn-211's stepped executor);
- species whose girth reads the tree as grown (`retained`, `leaf_girth`), whose disused pipes and leaf mean are tree-wide sums that fn-211 made order-free.

**Tests** (`fine/tests/realized.rs`, in the gate, about 35 s):
- every cut at age 30, and every order and chunking;
- fractional lifespans, with woken roots present;
- relays, dormancy, and roots shed by what they realized, on the unlit oak (`retained` 0) and the spruce;
- a budget refusal taken with the roots' growth counted.

**The 40/80 sweep:** `cargo run --profile ci -p telperion-space --example realized -- 40,80 1,7 64,256,1024,4096,inf`. Raw output is in `raw/realized/` (gitignored).

## Timing (the CPU executor against today's engine; Ryzen 9 5950X, 32 threads)

All-cores run: load 12 at the start, 9 at the end (`raw/realized/sweep2-all-cores.md`). The one-core run used `RAYON_NUM_THREADS=1` at load 9 to 6 (`sweep2-one-core.md`).
- "Rows" is the closed form's table for the predictor, which the cut reads. A cut of ∞ needs no table.
- Cold is the first build in the process; warm is the second.
- The fine figures exclude "Rows".

| Tree | Whole, all cores, cold / warm ms | Fine, all cores, cut 1,024, cold / warm ms | Fine, all cores, best cut | Rows, all cores, ms | Whole, 1 core, cold / warm ms | Fine, 1 core, cut 1,024, cold / warm ms | Fine, 1 core, cut ∞ |
|---|--:|--:|--:|--:|--:|--:|--:|
| beech 80 1 | 690 / 643 | 170 / 147 | 145 / 162 (4,096) | 68 | 1,437 / 1,285 | 821 / 937 | 883 / 872 |
| beech 80 7 | 1,524 / 1,492 | 203 / 205 | 190 / 192 (4,096) | 188 | – | – | – |
| spruce 80 1 | 3,221 / 3,202 | 957 / 881 | 764 / 773 (∞) | 142 | 7,379 / 7,090 | 6,162 / 6,808 | 5,710 / 5,987 |
| spruce 80 7 | 3,124 / 2,969 | 790 / 869 | 781 / 812 (∞) | 139 | – | – | – |
| spruce 40 1 | 1,268 / 1,066 | 237 / 233 | 209 / 229 (4,096) | 64 | 1,750 / 1,809 | 1,895 / 1,456 | 1,165 / 1,137 |

The phases' split, warm, spruce 80 at 1,024: growth 200 to 210 ms, girth 50, lay 113 to 116, bend 108 to 110. The rest is the structure above the cut's own settle.

### Proposal (owner asked for speed): today's `grow` through the partition

For every species that neither reads light nor keeps shed pipes, `grow` can run as this partition. The tree is unchanged to the bit, and the build is 3.4 to 7.3 times faster on all cores at age 80 (warm, cut 1,024):
- beech: 0.64 to 1.49 s down to 0.15 to 0.21 s, plus 0.07 to 0.19 s of rows;
- spruce: 3.0 to 3.2 s down to 0.77 to 0.88 s.

The gain is parallelism. Today's engine pays a fork and a merge per cycle and runs serial whole-tree passes (fades, shed, the vigour and base-scale loops); the partition grows and settles each root independently. On one core the partition is 10 to 35 % faster.

The best cut differs by species:
- the spruce is fastest at ∞, which needs no closed-form table;
- the beech is slowest at ∞ (1.1 to 2.1 s): its few huge roots serialise.

So the cut that minimises time is a tuning value per species, and it is a work placement only. Not built; for the host.

## Design question (host): D17, whole-tree shedding, on the GPU

**What stops the GPU mirror.** Realized mechanics makes shedding by fade (FINE-CONTRACT.md D17, `shed.rs`) a decision both executors take inside fine roots. An axis is dropped where `fade <= 0`. With no living apex in its subtree, that is `until <= age - delay - 1`, where `until` is the subtree's presence-weighted living time. It is f64 sums and products of presences: unit survival × share, node presences, birth presences and vigour. The contract makes none of them exact on the GPU, which takes presences in f32 (task 4).

Under option 4 this decision was D15, P2's committed extinction, which is integer. So it is new on the fine path.

**Measured** on today's engine: the decisions taken on the time term (`raw/realized/fade-probe.txt`; probe patch alongside, not committed).

| Tree | Taken | Exactly on the bound (whole presences, fade = 0, dropped) | With partial presences | Within 1e-4 of the bound | Nearest partial |
|---|--:|--:|--:|--:|--:|
| beech 40 1 / 7 | 5,497 / 8,823 | 602 / 1,061 | 56 / 95 | 0 / 0 | 2.2e-2 / 1.4e-2 |
| beech 80 1 / 7 | 81,575 / 137,981 | 8,390 / 14,512 | 1,068 / 1,834 | 0 / 0 | 2.1e-3 / 9.5e-4 |
| spruce 40 1 / 7 | 188,213 / 181,048 | 8,367 / 8,037 | 1,639 / 1,528 | 0 / 0 | 5.0e-3 / 2.2e-4 |
| spruce 80 1 / 7 | 606,027 / 602,123 | 27,707 / 27,203 | 5,406 / 5,450 | 0 / 1 | 1.3e-3 / 2.1e-5 |

What the table shows:
- About 5 % of the decisions sit exactly on the bound. That is whole presences, integer sums, exact in f32 too.
- About 1 % read partial presences.
- One decision (spruce 80, seed 7) sits 2.1e-5 from its bound. That is within reach of f32 presences' error (task 4 measured radii within 4.2e-6 relative).

**Options:**
- **A. Fixed-point living (extends host decision 1's contract).**
  - Every presence that enters `until`, `alive` or the child weight is computed as a Q32 integer on both executors: unit survival and persistence leads from the decision's own integer thresholds, through `fixed`'s `log2`; the node and D10 presences are already Q32.
  - The living sums and products are taken in a specified fixed point, and the comparison is exact.
  - It moves today's trees at rounding level, as decision 1 did, measured on the 28-tree dump and stills. The geometry may keep reading the f64 or f32 presences.
  - The largest option: every presence on that path is ported, which FINE-CONTRACT.md avoided for D18.
- **B. An integer shedding rule.** Drop where the subtree's whole-presence idle time has passed `delay + 1` (as `fade <= 0` reads it with every presence whole). The fade then only sizes what is kept, continuously, clamped at zero.
  - Exact on both executors from integer data, with no new arithmetic.
  - It changes today's topology: the axes now dropped only because partial presences pulled their living time under the bound (at most the "partial" column, 0.1 to 1 % of decisions) are kept at size zero or near it.
  - Host decision 2 (identities botanical; omitting thin wood is a draw-time level of detail) would then drop them from the drawing.
- **C. Emulated binary64 for the shedding chain.** Add, multiply and max are cheap to emulate, but the presences feeding them would also have to be bit-exact f64. That is option A's port plus the f64 transcendentals decision 1 rejected. Not recommended.
- **D. f32 shedding on the GPU with a measured flip rate.** That breaks R1's "identical topology", so it is listed only for completeness.

**Also for the GPU mirror, implementation rather than design:**
- The device pipeline is top-down by wave today. Realized mechanics adds three bottom-up passes across a root's waves (living, girth, load) and the three per-root exchanges with the CPU's settle.
- The ground routing (host decision 3) must be decided before the bearer's realized scale exists. An upper bound that ignores the bearer's fade (fade ≤ 1) keeps the rule conservative.

## Acceptance against the task

| Criterion | State |
|---|---|
| Beech and spruce, 40 and 80, seeds 1 and 7: topology by lineage and geometry bit-identical on the CPU reference | **met** (digest over every field) |
| The same at cuts 256, 1,024, 4,096, any order or batch | **met** on the CPU (and at 64 and ∞) |
| GPU geometry within 50 µm max, 10 µm RMS | **not met**: GPU mirror not built; stops at D17 |
| Committed-boundary path out of production, kept only where named | **met**: `grow_committed`, kept for the GPU tests |
| Timing warm and cold against the CPU on 1 and all cores; round trips counted | **met for the CPU executor** (`Fine::phases`, `Fine::trips`); the GPU's waits on its mirror |
| Workspace gate green | **met**: `cargo test --profile ci --workspace --no-fail-fast` at `13f22e09`'s code, 1,148 passed, 0 failed, 10 m 49 s at load 6 to 32 (GPU lock held) |

## Host decision 7 built: `grow` through the partition (2026-10-07)

`telperion_space::grow` builds every species that neither reads light nor keeps shed pipes or leaf girth through `grow_fine` at N = 1,024 (today: the beech, spruce and palm). The oak stays on today's path, `grow_whole`, until fn-211's stepped executor lands. Every `grow`, `grow_whole`, `grow_staged` and `sketch` returns its tree in the canonical lineage order (`lineage_order`, FINE-CONTRACT.md, "Host decision on the order").

**Proof** (`examples/digests.rs`, run on this commit and on a temporary worktree at the task's base `7782a081`, since removed; `raw/realized/digests-*.txt`):
- The scope: all four species, ages 40 and 80, seeds 1 and 7, neutral light and extinction 0.5 under sky 0.5.
- The result: the lineage digest of all 32 trees is unchanged.

**`measures hash 80` is re-pinned** to the canonical order once (`raw/realized/measures-hash-80-*.txt`). It folds axes in index order, so every tree whose order moved changes hash.

| Tree (age 80) | Before | Canonical |
|---|---|---|
| beech 1 (neutral and lit) | fffe057daff5b9d5 | 59f2d4aab6c991e6 |
| beech 7 (neutral and lit) | 8f8b82bec23c1625 | 95bc336fb5f832e5 |
| spruce 1 (neutral and lit) | 710f721384661baf | 2876033b4c21be8f |
| spruce 7 (neutral and lit) | beec95c7ff4d1ba5 | f2f08bb7647f4f60 |
| oak 1 neutral / lit | a889ad4d2258ce45 / 5ffe7f9abd37c979 | e38168aff2c5680e / 37078cc15195ab33 |
| oak 7 neutral / lit | 4f5750b0a146672c / e83e5f6d4ac3d61b | 01ebe57888079896 / f0422c94e2a80ceb |
| palm 1 / 7 | d2cd16377eece1a5 / c3dee32807ab1c0a | unchanged (already in that order) |

**Timing** (`examples/timegrow.rs`, seed 1, neutral light; `grow` alone, cold and then the best of three warm; load 6 to 7):

| Tree | Before, all cores, cold / warm ms | After, all cores | Before, 1 core | After, 1 core |
|---|--:|--:|--:|--:|
| beech 40 | 49 / 33 | 36 / 26 | 78 / 56 | 205 / 192 |
| beech 80 | 533 / 445 | 257 / 221 | 1,192 / 1,033 | 1,599 / 1,480 |
| spruce 40 | 637 / 563 | 288 / 283 | 1,552 / 1,599 | 1,926 / 2,131 |
| spruce 80 | 2,170 / 2,060 | 1,288 / 1,066 | 6,824 / 6,382 | 9,724 / 7,894 |
| oak 40 / 80 (today's path) | 216 / 155, 1,624 / 1,558 | 171 / 157, 1,610 / 1,709 | 338 / 335, 4,909 / 4,418 | 342 / 343, 5,129 / 4,869 |

What the table shows:
- **On all cores,** `grow` is about twice as fast for the beech and spruce.
- **On one core it is slower** (spruce 80: 6.4 s up to 7.9 s; beech 80: 1.0 s up to 1.5 s).
- **Where the time goes** (`timeorder`, a probe not kept):
  - the closed-form table the cut reads: 0.70 s (beech 80) and 1.20 s (spruce 80) on one core, against 77 and 154 ms on all cores;
  - `lineage_order`: 20 ms (beech 80, 179k axes) and 107 ms (spruce 80, 956k axes), serial.
- The partition itself is 10 to 35 % faster than today's engine on one core (the timing table above).

**Proposals (host):**
- (a) Leave out the table where it costs more than it saves. Cut ∞ needs none: the spruce was fastest there on every core count, and the beech on one core.
- (b) A count-only predictor. The cut reads only the rows' `count`; reach bounds, sections, mass and the seed row are the committed path's.
- (c) Cache the table per species and age across `grow` calls. A retune that changes the species still pays for it.

## Host decision 8: what is built, and the GPU mirror's remaining work (2026-10-07)

**Built:**
- **CPU:** every presence the shedding sums read is Q32, and the fade is integer (FINE-CONTRACT.md, "Host decision 8 built on the CPU"). Topology is identical in 28 trees; tips move at most 0.45 µm; the stills are indistinguishable.
- **GPU contract:** `contract.wgsl` mirrors `under`, `over`, `times`, `wood_at`, `window`, `carry` and `fade` bit for bit. The contract probe covers 262,144 cases each (selectors 12 to 14), and every result is bit-identical.

**Not built: the GPU executor itself under realized mechanics.** The device loop (`roots.rs`, `chain.wgsl`, `shed.wgsl`, `emit.wgsl`, `resolve.wgsl`) runs option 4 today. The work, in the order it can be tested:

1. **Growth without committed shedding.** `shed.wgsl` keeps every axis, so every borne bud becomes the next wave's chain. Test: every root's axes by lineage before shedding, against `grow::realized::root`.
2. **Q32 presences in the chain grower** (`presence.wgsl` alongside its f32 geometry presences):
   - unit survival and persistence, the relays and the stop of a failed unit;
   - a lateral's and a woken bud's made presences;
   - a woken bud's carrying and keeping;
   - the living share.
   - Each is stored per unit and per axis in the packed records. Test: bit identity with the CPU's `Draws`, per axis.
3. **Phase 1 on the device**, one invocation per chain as now:
   - Q32 assign (the running products, unit presences, phytomer presences and vigour);
   - the fades bottom-up across waves. Each chain reads its borne buds' chains, contiguous in the next wave, at the range the emit scan already computes, so the 64-bit max needs no atomics;
   - the shedding flags top-down;
   - one `Living` record per root up.
   - Test: topology by lineage after shedding, against `grow_fine`.
4. **Phases 2 to 4.** The CPU settles the structure above the cut between them, as `grow_fine` does:
   - scale top-down, girth bottom-up across waves (base radius up);
   - the unbent lay top-down, the levers bottom-up across waves (base load up);
   - the bent lay from the first lay's frames (`lay.wgsl`'s `was_heading` and `was_side`).
   - These replace `resolve.wgsl`'s committed girth fit and borne loads.
   - Test: fn-125's screen against `grow_fine`.
5. **The conservative ground bound.** A root goes to the CPU where its reach bound, with the bearer's fade taken as 1, can reach the landing zone (host decision 3). It is decided after phase 1, from the down record's base scale.
6. **Timing, and removal of the probe.** Timed cold and warm against `grow_fine` on 1 and all cores, round trips counted. Then `grow_committed` and the option-4 kernels leave the GPU tests.

**Sizing (worker's reading, for the host):** tasks 2 to 5 each took a session for one comparable slice (growth, geometry, the device loop). Steps 1 to 3 are one such session; step 4 is another.

## The GPU mirror built (task 6, worker, 2026-10-07)

Steps 1 to 6 of "Host decision 8" above, from `837edc7c` to `689ede1c`.

### Answer in brief

- **Built.** The GPU executor runs realized mechanics as the roots executor of one driver.
  - The driver is `telperion_space::settle` (`fine/expansion.rs`). It grows the structure above the cut and settles it through the four phases exactly as `grow_fine` did.
  - `grow_fine` is now the same driver with the CPU reference as its executor (the `Roots` trait). Its trees are unchanged to the bit (`fine/tests/realized.rs`).
  - The GPU's side is `telperion-render`'s `generation/fine/device.rs`.
- **Topology, presences and living equal the CPU executor's to the bit.**
  - At 25: beech, spruce and the unlit oak, seeds 1 and 7, forward in runs of 4,096, reversed in runs of 777, and forward in runs of 64. The gate test is `every_root_grows_whole_on_the_gpu_as_on_the_cpu`.
  - Compared: every root's axes as grown, by lineage; every axis's Q32 presences (vigour, its units' presences summed, each phytomer's scale, its living apex, its sleep); each root's living as reported up; each root's axes kept by its own subtree's shedding.
- **The whole tree settles on the GPU as `grow_fine` settles it.**
  - At 25: cuts 256, 1,024 and 4,096, in the same three orders and batches (gate test `every_root_settles_on_the_gpu_as_on_the_cpu`).
  - At 40 and 80: beech and spruce, seeds 1 and 7, cuts 256, 1,024 and 4,096 (`realized_at_40_and_80`, ignored).
  - Every live root's kept axes equal `grow_fine`'s by lineage.
  - Tips are at most **25.3 µm** from `grow_fine`'s (beech 80, seed 1, cut 4,096), RMS at most **1.44 µm**; radii within 6.4e-6 relative.
  - The laid floats are equal by digest, forward in runs of 65,536 against reversed in runs of 4,096 (and the three orders at 25).
- **Round trips** are the runs plus the waves plus one record per root per phase: living, base radius and load (two records where no PA sags). That is **14 to 40 a tree**, held by the tests.
- **Option 4 is gone** (`d00eb019`).
  - On the GPU: committed shedding, its emit and resolve kernels, and their tests.
  - On the CPU: `grow_committed`, its executor and records, `fine_cases`, `roots_geometry`, and the committed rows and extinction curves the GPU read.
  - P2 (`alone.rs`) stays as the named probe that the chain grower's tests and two examples read.
- **Speed:** the spruce at 80 builds on the GPU in about the time 32 CPU cores take (1.08 to 1.12 s against 1.09 to 1.13 s warm), 8 times faster than one core. The beech is 2 to 7 times slower than 32 cores. Phase 1's growth is most of it (below).

### How it works

1. **Growth without per-chain shedding** (`837edc7c`). The chain grower keeps every axis. `emit.wgsl`'s `realized` entry hands every borne bud on as a chain of the next wave.
2. **Q32 presences in the chain grower** (`assign.wgsl`, `carry.wgsl`). Every presence a fade reads is the CPU's integer.
   - No decision reads a presence. So growth records only each one's inputs (its draw's key, its bounds, its wood), and the chain's end takes them all in one loop, through one call of the contract's leads and windows.
   - With a call at each of about fifteen sites, the driver's compiler inlined the 128-bit `log2` and `exp2` at each and never finished (11 GB resident; FRICTION.md).
   - The products then run in the grower's order: running survival, units' presences, phytomers' scales, each lateral's time to its birth and its node's scale, a woken bud's carrying (walked again from its pending record), `Woken::kept`, `Grower::living` and vigour.
3. **Phase 1 on the device** (`fades.wgsl`, `shedding.rs`).
   - `up` carries each subtree's living from the deepest wave to the first. Each chain reads its borne buds' chains in the next wave at the range the emit scan gave it, so the maxima need no atomics.
   - One record per root crosses: lived (Q32.32) and alive (Q32). The CPU sheds the structure above the cut on them.
   - `down` then keeps each axis from the first wave down: its bearer kept and its fade more than none.
4. **Phases 2 to 4** (`settle.wgsl`). The CPU settles between them as `grow_fine` does.
   - Scales top-down: vigour times fade, the Q32 scales.
   - Girth bottom-up from the next wave's base radii; one base radius per root goes up.
   - The unbent lay top-down from the CPU's bearer frames.
   - Levers bottom-up from the next wave's loads; one load per root goes up. Its moment is the root's own, moved to the tree's frame on the CPU.
   - The bent lay from each phytomer's first-lay frame (FRAME-CORRECTION.md).
5. **The conservative ground bound** (`fine/landing.rs`, `cae01463`). It is decided in the driver, so it holds whichever executor grows the roots.
   - The bound: a live root whose closed-form reach at its base scale (fade taken as whole), from the lower of its node and the point below it, can reach the landing zone. The zone takes twice its realized base radius and the longest internode.
   - Such a root is laid on the CPU, from its bearer's unbent frame in phase 3 and from either frame in phase 4, on the plan's cores. Its load replaces the executor's.
   - `tests/landing.rs` holds that no other root's wood enters the zone (beech and spruce at 30).
   - Counted at 80: 16 to 50 of the beech's roots and 4,843 to 5,623 of the spruce's (about 5 % of its phytomers).
6. **Removal and cleanup** (`d00eb019`, `689ede1c`). The grower's f32 presences, which only option 4's geometry read, went with option 4. The relay's blend and the whorl stay.

**Three f32 fixes the screen needed** (`19d8ad2e`, `7baafe8c`, `8d6b2fac`).
- The fault was a drift of the heading along the beech's long modules. It came from three places:
  - a step's share of a pull, `1 - e^-x` near none;
  - the elevation, `asin` of a near-one z;
  - the GPU's own sine, good to about 5e-7 whatever its argument. The elevation's pull and a module's sixty wander turns gathered it.
- The fixes:
  - an expm1-like series;
  - `atan2` of the lean;
  - the elevation's step as a rotation in its vertical plane;
  - a short turn's sine, cosine and versine by their series.
- Beech 80, seed 1: 137 µm down to 18.8 µm. The option-4 path's oak at 25 had also fallen, from 20.8 to 9.8 µm, before that path was removed.

### Geometry (`raw/gpu-realized/final-cut*.txt`)

The GPU's roots against `grow_fine`'s. "Roots on the CPU" are the roots the ground bound lays there; their tips are compared with `grow_fine`'s from their own origins. The beech's are equal: no PA sags, so the structure above the cut is the CPU's to the bit. The spruce's differ by the structure above the cut settled on the GPU's f32 radii and loads.

| Tree | Cut | Roots | Phytomers | Tips most / RMS µm | Roots on the CPU (their tips most µm) |
|---|--:|--:|--:|--:|--:|
| beech 40 1 | 256 | 3769 | 90866 | 1.5 / 0.10 | 4 (0.0) |
| beech 40 1 | 1024 | 553 | 106588 | 2.8 / 0.43 | 4 (0.0) |
| beech 40 1 | 4096 | 380 | 107430 | 2.8 / 0.45 | 4 (0.0) |
| beech 40 7 | 256 | 6062 | 153720 | 1.4 / 0.12 | 2 (0.0) |
| beech 40 7 | 1024 | 975 | 176142 | 7.9 / 0.74 | 3 (0.0) |
| beech 40 7 | 4096 | 649 | 177689 | 7.9 / 0.78 | 3 (0.0) |
| beech 80 1 | 256 | 46644 | 1321079 | 18.8 / 1.26 | 16 (0.0) |
| beech 80 1 | 1024 | 7865 | 1535951 | 18.8 / 1.42 | 16 (0.0) |
| beech 80 1 | 4096 | 6362 | 1521728 | 25.3 / 1.44 | 25 (0.0) |
| beech 80 7 | 256 | 79607 | 2238625 | 18.7 / 1.13 | 28 (0.0) |
| beech 80 7 | 1024 | 12829 | 2597942 | 18.7 / 1.31 | 29 (0.0) |
| beech 80 7 | 4096 | 10288 | 2556270 | 18.7 / 1.26 | 50 (0.0) |
| spruce 40 1 | 256 | 21626 | 1088389 | 1.7 / 0.25 | 1337 (0.4) |
| spruce 40 1 | 1024 | 21307 | 1088613 | 1.7 / 0.25 | 1337 (0.4) |
| spruce 40 1 | 4096 | 19826 | 1089589 | 1.9 / 0.27 | 1337 (0.4) |
| spruce 40 7 | 256 | 20869 | 1071286 | 1.6 / 0.26 | 1021 (0.5) |
| spruce 40 7 | 1024 | 20444 | 1071594 | 1.6 / 0.26 | 1021 (0.5) |
| spruce 40 7 | 4096 | 19024 | 1072522 | 2.2 / 0.28 | 1021 (0.5) |
| spruce 80 1 | 256 | 93093 | 4350964 | 2.7 / 0.39 | 5623 (39.1) |
| spruce 80 1 | 1024 | 92703 | 4351256 | 2.7 / 0.39 | 5623 (39.1) |
| spruce 80 1 | 4096 | 91293 | 4352152 | 2.7 / 0.39 | 5623 (39.1) |
| spruce 80 7 | 256 | 91587 | 4342978 | 2.6 / 0.39 | 4843 (44.5) |
| spruce 80 7 | 1024 | 91262 | 4343230 | 2.6 / 0.39 | 4843 (44.5) |
| spruce 80 7 | 4096 | 89987 | 4344034 | 2.6 / 0.40 | 4843 (44.5) |

### Timing (`raw/gpu-realized/clean-cut1024.txt`; cut 1,024)

Ryzen 9 5950X (32 threads) and RTX 3080. Load 1.35 at the start and 3.58 at the end.
- **GPU:** the whole tree's build through `settle` with the device as executor, from table upload to the last bent lay. The structure above the cut, settled on the CPU, is included. In brackets: phase 1 on the device with its round trips (and of it the fades), then phases 2 to 4 on the device.
- **CPU:** `grow_fine` on all cores and on one, cold and then the better of two warm.
- **Cold** is the first build of that tree in the process. The pipelines were built before, from the driver's cache (35 ms; a first build after a shader change takes 35 to 45 s).

| Tree | GPU cold ms (device phase 1 / phases 2 to 4; trips) | GPU warm ms | CPU all cores cold / warm ms | CPU 1 core cold / warm ms |
|---|--:|--:|--:|--:|
| beech 40 1 | 104 (91, of it 1 fading / 6; 23 trips) | 103 (90, of it 1 fading / 6; 23 trips) | 17 / 15 | 86 / 70 |
| beech 40 7 | 95 (81, of it 1 fading / 6; 25 trips) | 96 (82, of it 1 fading / 6; 25 trips) | 24 / 24 | 129 / 115 |
| beech 80 1 | 575 (503, of it 2 fading / 16; 37 trips) | 376 (305, of it 2 fading / 16; 37 trips) | 174 / 175 | 1349 / 1115 |
| beech 80 7 | 690 (590, of it 3 fading / 21; 38 trips) | 532 (433, of it 3 fading / 21; 38 trips) | 282 / 281 | 2022 / 1882 |
| spruce 40 1 | 536 (454, of it 2 fading / 19; 15 trips) | 307 (225, of it 2 fading / 18; 15 trips) | 274 / 272 | 1851 / 1850 |
| spruce 40 7 | 496 (419, of it 2 fading / 18; 14 trips) | 285 (210, of it 1 fading / 18; 14 trips) | 271 / 267 | 1805 / 1809 |
| spruce 80 1 | 1328 (983, of it 5 fading / 73; 25 trips) | 1079 (735, of it 5 fading / 73; 25 trips) | 1132 / 1131 | 9239 / 8637 |
| spruce 80 7 | 1245 (917, of it 5 fading / 72; 25 trips) | 1124 (790, of it 5 fading / 72; 25 trips) | 1092 / 1093 | 8510 / 8508 |

What the table says (the worker's readings):
- **Phase 1's growth is the cost**: 305 to 433 ms of the beech's 376 to 532 ms at 80, and 735 to 790 ms of the spruce's 1.08 to 1.12 s.
  - Under option 4 the same chains grew in about 165 ms (GPU-FINE.md).
  - Two things changed. Every borne bud now grows (205k chains for the beech at 80, where option 4 grew 126k). And each chain's end takes its Q32 presences through the contract's 128-bit `log2`: four of them per presence, two for the draw and two for the threshold.
- **The fades, the girth, the lays and the levers cost little**: 1 to 5 ms and 6 to 73 ms.
- **The beech is slow on the device for the reason it was under option 4.** Its chains are long and few, so each wave is a handful of runs, and the round trips (37 a tree) and serial waves dominate.

### Open, for the host

- **Speed of phase 1 (proposals, not built).**
  - (a) A thresholds' `logit` table, computed on the CPU by `fixed::logit` and read wherever a presence's bound is a species constant: the whole-unit survival, the relays, the runs of laterals and sleeping buds. This would take about half of the `log2` calls out.
  - (b) One `logit_draw` per two-sided lead (the run leads) instead of two.
  - (c) A workgroup per heavy chain for the beech (GPU-FINE.md's lever).
- **The geometry's floor.** f32 on the device leaves 19 to 25 µm on the beech's 80-year roots, under half the screen. The CPU-laid spruce roots near the ground differ from `grow_fine`'s by up to 44.5 µm: the structure above the cut settles on the device's f32 radii and loads. Settling it on f64 copies of the roots' records would remove that, at no round trip.
- **The ground bound's tail.** The reach is the closed form's quantile bound, as under option 4. `tests/landing.rs` holds that no unrouted root's wood came near the ground at 30. At 40 and 80 no GPU chain set its below-ground status bit.

### Acceptance against the task

| Criterion | State |
|---|---|
| Beech and spruce, 40 and 80, seeds 1 and 7: topology by lineage, bit-identical geometry on the CPU reference, within 50 µm max and 10 µm RMS on the GPU | **met**: topology equal; GPU tips at most 25.3 µm, RMS at most 1.44 µm; CPU-laid roots at most 44.5 µm |
| The same at cuts 256, 1,024 and 4,096, in any order or batch | **met** at 25 (three orders and batches, gate test) and at 40 and 80 (two orders and batches) |
| The committed-boundary path removed from production code | **met**: removed outright (`d00eb019`); P2 stays as a named probe |
| Timing warm and cold against the CPU on 1 and all cores; round trips counted | **met**: table above; 14 to 40 round trips a tree, asserted |
| Workspace gate green | **met**: `cargo test --profile ci --workspace --no-fail-fast` at `689ede1c`, 2,291 passed, 0 failed, 48 ignored, 14 m 19 s at load 4.5 to 33 (GPU lock held; `raw/gpu-realized/gate.txt`) |
## Host decision 9 built: a cut predictor that costs what the cut reads (task 7, 2026-10-07)

**Built** (`closed_form/predictor.rs`, `predictor/axis.rs`): the kept recursion cut down to the two columns the count reads, the extinction curve and the expected kept phytomers. Every operation on them is the kept tables' own, in their order; sections, masses, moments and reach bounds are left out. `grow_fine` builds it from the grower's own windows (`grow_fine(species, request, plan)`; no `Expected`), and reports its seconds in `Fine::predict`.
- **Iterative.** A bud's table waits on younger tables only; they are built on a heap stack of pending buds, never by recursion. Test: age 300 on a 128 kB thread (`an_old_tree_needs_no_deep_stack`). `cores::deep` stays for the full rows (`Expected`: the bounded and committed paths), which `grow` no longer builds. The grower's windows (`expected_log_lengths`) still recurse, as before this task.

**Proof** (raw output in `raw/predictor/`):
- **Every count equal to the closed form's row to the bit:** every borne PA, every age 0 to the tree's. In the gate: the four species at ages 1, 2, 9 and 30 (690 counts). Run once at 40 and 80 (`counts_at_40_and_80`, ignored): beech 246 / 486, spruce 123 / 243, oak 246 / 486, the palm none (it bears no lateral). So every cut decision is unchanged.
- **Every tree unchanged:** the lineage digest of all 32 trees (`examples/digests.rs`: four species, 40 and 80, seeds 1 and 7, neutral and lit) and `measures hash 80` (seeds 1 and 7, neutral and lit, 16 trees) equal to the task's base `3fb527eb`.

**Cost of the cut's reading** (one core pinned, `taskset -c 2`, `RAYON_NUM_THREADS=1`; then all cores):

| Tree | Table (rows), one core / all cores ms | Predictor, one core / all cores ms |
|---|--:|--:|
| beech 40 | 7 / 17 | 2 / 3 |
| beech 80 | 31 / 61 | 10 / 11 |
| spruce 40 | 34 / 47 | 8 / 8 |
| spruce 80 | 95 / 142 | 25 / 24 |

**The premise did not hold on this tip** (FRICTION.md): the table cost 31 and 95 ms on one CPU, not 0.70 and 1.20 s. The earlier reading most likely ran 32 `cores::deep` threads, each with its own memo, under `RAYON_NUM_THREADS=1` alone. So the predictor saves 20 to 70 ms on one core. That is within the noise of `grow`.

**`grow` against `grow_whole`, warm ms** (`examples/timegrow.rs`, seed 1, neutral light; best of three after a cold build; base and new interleaved, two rounds each, plus one earlier run; `raw/predictor/time-*.md`):

| Tree | One core: `grow` base | `grow` new | `grow_whole` | All cores: `grow` base | `grow` new | `grow_whole` |
|---|--:|--:|--:|--:|--:|--:|
| beech 40 | 76, 80, 81 | 74, 78, 74 | 69 to 81 | 27, 27, 28 | 19, 18, 17 | 41 to 43 |
| beech 80 | 1,115, 1,149, 1,152 | 1,151, 1,178, 1,143 | 1,272 to 1,471 | 219, 218, 219 | 381 (one outlier), 172, 183 | 446 to 477 |
| spruce 40 | 1,857, 1,947, 1,908 | 1,913, 1,907, 1,883 | 1,816 to 2,054 | 303, 294, 295 | 276, 261, 277 | 576 to 590 |
| spruce 80 | 8,798, 8,931, 8,582 | 8,747, 9,057, 8,679 | 8,290 to 9,369 | 1,149, 1,182, 1,153 | 1,056, 1,059, 1,044 | 2,206 to 2,253 |

Load ran from 1 to 13 across the runs, shared with the other worktree's GPU work.

What the table shows:
- **All cores:** `grow` is 6 to 37 % faster than before (beech 40: 27 to 18 ms; spruce 80: 1.16 to 1.05 s), about what the table cost there, and still twice as fast as `grow_whole`.
- **One core:** the beech at 80 is faster than `grow_whole` (1.14 to 1.18 s against 1.27 to 1.47). The beech at 40 and the spruce at 40 and 80 are at parity within the noise. The spruce at 80 averages 8.83 s against `grow_whole`'s 8.81 s, and single pairs differ by up to 3 % either way.
- **Where the spruce's one-core time goes:** the partition's four phases alone take 8.5 s at age 80 (`Fine::phases`), as long as the whole tree. The remaining 0.4 s is the structure above the cut, its settle and `lineage_order`. A clear one-core gain needs less work in the partition, or a different cut for one core (REALIZED.md's timing: the spruce was fastest at ∞). The cut is a host decision (decision 7); this task leaves it at N = 1,024.

**Gate:** `cargo test --profile ci --workspace --no-fail-fast` at `c104e058`'s code: 1,151 passed, 0 failed, 11 m 27 s, load 5.5 at the start and 9.5 at the end (31.7 at its one-minute peak), GPU lock held.

**Acceptance, as the worker reads it:** the cut decisions, trees, iteration, all-core timing and gate are met. One-core `grow` at least as fast as `grow_whole` is met for the beech at 80 only; the beech at 40 and the spruce at 40 and 80 are at parity within ±3 % noise, not measurably faster or slower. Not marked done; for the host.
