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
