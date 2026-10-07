# fn-211 task 12: one executor, the stepped grower settles per root (worker, 2026-10-07)

Host decision (b) on TASK11.md, then (A) on the settle: the stepped executor settles per root through the shared per-root settle, carrying the tree as grown. Dead roots are passed by. `grow_fine`'s and `grow_whole`'s paths are deleted only if all 20 trees are identical and none is slower.

## Answer in brief

- **Built (`253e06f2`).** The stepped executor's settle is realized mechanics' four phases over its own roots. The exchange carries the tree as grown (PHASES.md, host decision). Dead roots are passed by each cycle.
- **Identical on all 20 trees.** The per-phase probe (`examples/stepped.rs`, lit, every cycle) is also identical in all 36 runs at 40: the oak, beech and spruce, seeds 1 and 7, cuts 256 and 1,024.
- **Faster on 12 trees, level on 4, slower on 3 or 4.**
  - Faster: the beech, oak and lit oak, at 0.54 to 0.87 times.
  - Level: the palm, which has no roots.
  - Slower: spruce 80, at 1.17 to 1.22 times today's `grow` (`grow_fine`). Spruce 40 is at 1.04 (s1) and 0.99 (s7).
  - Per the host's condition, **nothing is deleted yet**. Options are at the end.

## The 20 trees (`raw/task12/measure-1.txt`)

Each run started at a one-minute load under 6; the load after each is in the raw file (5.3 to 13.4). Best of three warm builds, back to back. "Today" is `grow` as it routes now (`grow_whole` for the oak, `grow_fine` for the others).

| Tree | Today, s | Stepped, s | Ratio | Stepped cycles / settle, s |
|---|--:|--:|--:|---|
| beech 40 s1 / s7 | 0.042 / 0.066 | 0.032 / 0.057 | 0.76 / 0.87 | 0.013 / 0.017; 0.026 / 0.030 |
| beech 80 s1 / s7 | 0.578 / 1.015 | 0.417 / 0.745 | 0.72 / 0.73 | 0.180 / 0.228; 0.339 / 0.398 |
| spruce 40 s1 / s7 | 0.910 / 0.869 | 0.944 / 0.858 | 1.04 / 0.99 | 0.395 / 0.543; 0.330 / 0.522 |
| **spruce 80 s1 / s7** | 3.651 / 3.778 | **4.472 / 4.414** | **1.22 / 1.17** | 2.045 / 2.410; 2.128 / 2.269 |
| oak 40 s1 / s7 | 0.235 / 0.437 | 0.138 / 0.282 | 0.59 / 0.64 | |
| oak 80 s1 / s7 | 3.137 / 7.336 | 1.841 / 3.982 | 0.59 / 0.54 | |
| lit oak 40 s1 / s7 | 0.787 / 1.341 | 0.541 / 0.961 | 0.69 / 0.72 | |
| lit oak 80 s1 / s7 | 8.700 / 19.431 | 6.172 / 13.683 | 0.71 / 0.70 | |
| palm | under 1 ms | under 1 ms | 0.86 to 0.90 | no roots at the cut |

All 20 digests are identical. The palm's row is from the load-gated runs before the last one, which was stopped when the load stayed above 6. It has no roots, so neither build does more than 1 ms.

## What was built

- **The exchange (`fine/exchange.rs`; `Roots::partials`, `Roots::attach`).** It runs after phase 1, where any PA keeps its tree as grown:
  - down go the bearer's base and the node's drawn scale; up come the root's leaf partials (order-free);
  - down go the mean and the bearers' fades at the root's base; up comes its base radius as grown;
  - the root keeps its own shed pipes for phase 2.
  - Today's `attachments` is split so that both use one code: `leaf_partials`, `attachments_given` and `disused_below`.
- **The stepped roots are `fine::Root`s.** Phase 1 (mark, assign, shed) runs on each root's own grower (`grow/stepped/settle.rs`). A woken root keeps what its bearer kept of it. Its stand-in carries its draws, so its bearer's fade reads the root's vigour. `Cpu` takes roots already grown (`rules: None`).
- **Ground handling** needs no change for the stepped executor. It is the CPU reference, so every root is laid on the CPU already, as `grow_fine`'s are (`Landing` with no rows).
- **The named gap.** An executor whose `Roots` does not carry the exchange refuses by name for a species that keeps its tree as grown: "this executor's roots do not carry the as-grown exchange". The trait's default methods do the refusing, and the GPU's realized executor (`telperion-render`, `device.rs`) is one such executor. It is also refused before that point by `alone::rules` for any species that reads light. This is a gap, not a fallback.
- **Dead roots.** A root with no living apex and no sleeping bud left is marked dead after its cycle. It no longer grows, wakes or lays. Its part of the growth digest is kept once, and its last leaves are dropped. It still enters balance, allotment, re-lays and light, where its totals and wood are read.
- **Cost fixes, each byte-identical:**
  - the root pieces stay on one pool thread across cycles (`cores::pinned`);
  - each root takes its own list of advanced apexes;
  - the rules' tables are shared between growers through `Arc` (`Windows::expected`, `Rules::laterals`).
  - The cause found: glibc's allocator, when 92,703 growers' memory moves between threads. With jemalloc preloaded, the spruce's root growth falls from 1.85 to 0.77 s.

## Why spruce 80 is still slower

`grow_fine` grows each root whole on one core, all its cycles at once: 0.41 s for 92,703 roots on 8 threads (`examples/finephases.rs`). The stepped executor grows every live root one cycle at a time across all of them. It takes 1.26 to 1.46 s for the same work on 8 threads, and 3.9 s against 2.75 s on one. The working set it touches each cycle (92,703 growers) does not stay in cache, and the cycles do not spread as well over the cores. Its settle (2.3 to 2.4 s) matches `grow_fine`'s phases 2 to 4 (1.3 to 1.5 s) plus phase 1 on the grown roots and letting the growers go (0.3 to 0.6 s).

## Decision for the host

- **(i) Run ahead where nothing crosses each cycle.** For a species that reads no light, nothing goes between a root and the structure above the cut from one cycle to the next: no balance, allotment, layout or sweep. The executor could grow each root through all its cycles when it is handed over, as `grow_fine` does, then settle as now.
  - The same tree; the per-cycle growth digest is rebuilt from counts recorded as each root runs.
  - The budget check fails or passes as before, because growth only adds.
  - Expected spruce 80: about 3.0 to 3.3 s, against 3.65 s today.
  - It is a schedule inside one executor, chosen by whether any PA reads light. That is a choice of execution, not of what is built, but it is a branch.
- **(ii) Accept spruce 80 at 1.17 to 1.22 times** for one executor, and delete now.
- **(iii) A faster allocator in the consuming binaries** (jemalloc or mimalloc). This is a dependency outside this crate. It was measured only on the spruce's root growth, and it made today's `grow` slower in the same run (3.5 to 4.0 s).

## Not yet done

- The deletion of `grow_fine` and `grow_whole`, pending the decision.
- The gate. The space crate's stepped, fine and girth tests pass (20), as does the per-phase probe at 40.

## After the host's decision (i): each root runs to its next synchronisation point (2026-10-08)

Built:
- **The schedule (`7a55150b`, `160f495b`; PHASES.md, host decision on the schedule):**
  - A root runs from the cycle it is due to the next cycle at which anything crosses between it and the structure above.
  - Roots due before that point wait for it, then run together, each from its own cycle.
  - For the beech, the spruce and the unlit oak, every root runs at the last cycle. The lit oak steps every cycle.
- **The probe:** the growth digest is rebuilt from the counts each root recorded as it ran. The probe holds every phase identical: 48 runs at 40 and 24 at 80 (the oak, unlit oak, beech and spruce, seeds 1 and 7; `raw/task12/probe-*.txt`).
- **`grow` is now the stepped executor (`2c569792`).** Its cut is read from the predictor's counts, and it keeps no phase digests (`Keep::Tree`). Today's path is kept as `grow_today` only for this measurement.

### A correction to every earlier ratio

Until `160f495b`, the comparison example timed the first path with its tree's digest inside the timing:
- **Task 11:** `digest(&grow(..))` was timed, so today's `grow` was charged its digest.
- **Task 12's first run:** `digest(&grow_today(..))` was timed in the same way.

The digest canonicalises and hashes the whole tree. For small trees it is a large share of the build: the beech at 40 takes 0.025 s without it and about 0.044 s with it. The ratios in TASK11.md and in `measure-1.txt` favour the stepped executor by that much. Task 11's lit-oak gain (0.75 to 0.78) is about the same after the correction (0.64 to 0.75 below).

### The 20 trees, digests outside the timing (`raw/task12/measure-3.txt`)

Each run started at a one-minute load under 6. Best of three warm builds, back to back.

| Tree | Today, s | grow (stepped), s | Ratio |
|---|--:|--:|--:|
| beech 40 s1 / s7 | 0.025 / 0.034 | 0.030 / 0.043 | **1.20 / 1.27** |
| beech 80 s1 / s7 | 0.313 / 0.530 | 0.398 / 0.643 | **1.27 / 1.21** |
| spruce 40 s1 / s7 | 0.565 / 0.541 | 0.776 / 0.721 | **1.37 / 1.33** |
| spruce 80 s1 / s7 | 2.425 / 2.645 | 3.339 / 3.326 | **1.38 / 1.26** |
| oak 40 s1 / s7 | 0.199 / 0.316 | 0.126 / 0.207 | 0.63 / 0.66 |
| oak 80 s1 / s7 | 2.327 / 6.145 | 1.291 / 3.248 | 0.55 / 0.53 |
| lit oak 40 s1 / s7 | 0.656 / 0.980 | 0.495 / 0.708 | 0.75 / 0.72 |
| lit oak 80 s1 / s7 | 8.119 / 20.550 | 5.370 / 13.049 | 0.66 / 0.64 |
| palm 40 and 80 | under 1 ms | under 1 ms | 0.91 to 1.01 |

All 20 digests are identical. **The beech and spruce are 1.20 to 1.38 times slower, so nothing is deleted.**

### Where the beech's and spruce's time goes

Spruce 80 s1 at load 6, one run:

| | Stepped, s | `grow_fine`, s |
|---|--:|--:|
| Structure above the cut | 1.0 (coarse cycles, handover, the roots run together) | about 0.5 |
| Roots grown, marked, assigned, shed | 0.38 (marked, assigned and shed as a separate pass after growth) | 0.53 (phase 1, growth included) |
| Phases 2 to 4 | 1.45 | 1.65 |
| Assembly | 0.31 | in its total |
| Total | 3.16 | 2.44 |

The difference lies outside the shared settle:
- **The stepped coarse grower grows each fine root's first cycle itself.** It then hands the root over each cycle (`hand_over`: drained, renumbered, a grower made per root). `grow_fine`'s structure above the cut never grows a stand-in.
- **The roots' phase 1 is a second pass over 92,703 growers,** after their growth, where `grow_fine` grows, marks, assigns and sheds each root in one pass.

Two changes would close it. Neither is built:
- (a) Fold phase 1 into the roots' run where their run ends the tree (grow, then mark, assign and shed on the same core). The woken roots' kept share from the coarse grower is applied first.
- (b) Hand a fine bud over as it is made, never growing it in the coarse grower, as `grow_fine`'s `Hands` does.

(b) changes the handover's order argument (PHASES.md, host decision 2: each grower keeps the whole tree's living order), so it is a design question.
