# fn-211 task 4, probe (a): the per-root interface by phase (worker, 2026-10-06)

DESIGN-INTERLEAVE.md, after Astra's review, binds this probe. A fine root is a lateral whose bearer stands above the cut and whose expected kept phytomers (the closed form's `count`) are at most N; its subtree is the root and everything it bears. Line numbers are as of commit `81acdd9b`.

## Answer in brief

- **Every phase of a cycle is a reduction a fine root can hand its bearer, except three.** In each of the others a root's subtree reports a few scalars in the tree's own arithmetic; partitioned this way, the result is the whole tree's to the bit.
  - **The per-PA size normalisers** (decision 23), in phase (iii).
  - **The lattice's deposits**, in phase (iv).
  - **The leaf-light girth's tree-wide mean**, in each re-lay and at the settle.
- **These three sum over every bud, leaf or phytomer of the tree in index order.** A partition can only add them root by root and then combine the roots' partials. That regroups a floating-point sum, and the oak's state differs from the cycle where the first fine root contributes.
- **Measured** (`examples/partition.rs`, `raw/task4/partition.txt`): the whole tree against the same tree with those sums grouped as a partition groups them, after every phase of every cycle.
  - Cases: the oak lit at seeds 1 and 7, ages 40 and 80; the beech and spruce; cuts 256, 1,024 and 4,096; roots combined forward, reversed or shuffled; chunks of 1, 7 or 64.
  - **The oak:** the first difference falls at cycle 2 to 7, in allocation or light. The topology is identical in every case, with every phytomer kept. Tips move at most 2.1e-11 m, and radii at most 1.0e-8 relative.
  - **The beech and spruce:** identical in every phase. Neither reads light, allocates or thickens by leaf light, so none of the three sums exists for them.
  - **Each sum alone:**
    - sizes first differ at cycle 3 to 6, in allocation;
    - leaves at cycle 2 to 7, in light;
    - girth at cycle 20 (the second re-lay; the first, at cycle 10, does not differ, for a reason not looked into).
- **Bit identity therefore needs these three sums made order-free.** Options are at the end. This is a design question, so the probe stops here.

## What was run

`grow_partitioned` (`src/grow/oracle.rs`, `src/grow/partition.rs`) is today's grower with the fine roots classed by the cut as the oracle classes them. Each of the three sums can be regrouped: the structure above the cut first in index order, then each root's partial in its own index order, the roots in the plan's order, `chunk` partials summed before they join. Each cycle is hashed after balance, allocation, growth with waking, and layout with light.

- **The phases hashed:**
  - **balance:** every living apex's kept share and memory;
  - **allocation:** every living apex's size;
  - **growth:** the axes, phytomers and living apexes;
  - **light:** every living apex's light and tip.
- **With no regrouping the run is today's tree to the bit**, on all twelve trees.
- **What this probe is, and what it is not:** it partitions the arithmetic of every reduction that crosses the cut, inside one grower. It is not a separate executor stepping fine roots in their own buffers.
  - The interface below is what such an executor must exchange.
  - The phases where it is exact are exact because they are bearer-to-child reductions in the tree's index order, and a fine root's subtree is a contiguous child of its bearer. This was checked by reading the code (sites below), not by running a stepped executor.

## The phases, and what crosses the cut

`Q` below is a subtree's collected light. In the code it is `light` in `balance.rs` and `lit` in the allocation.

| Phase | The whole tree does (site) | A fine root reports up | It receives down | Exact when partitioned? |
|---|---|---|---|---|
| (i) balance | `grow/balance.rs:26`. Each living apex's `Q` (base scale × running × light) and each axis's upkeep × wood are summed child to parent in reverse index order, with living status carried up. Then each lateral, parents first, updates its memory, draws its shedding and passes on its kept share. | Its subtree's `Q`, its upkeep and whether anything in it lives, each summed inside it in index order. | Its bearer's shed flag and kept share. The root then decides its own and its internal laterals' balance; their memory stays inside the root. | **Yes.** At the bearer the root's totals join its siblings' in the tree's index order. |
| (ii) demand and incoming vigour | `allocation.rs:vigours_from`, called from `grow/sketch.rs:allot`. A basipetal `sub` sum (presence × light), then an acropetal split at each node by λ over that node's children's `sub`. | Its subtree's `sub` (light-weighted presence). | Its incoming vigour, which it then splits inside itself. | **Yes.** A node's children are summed in index order, the root among them by its index. |
| (iii) per-PA normalisers | `allocation.rs:111` (`sizes`). Each PA's Σp and Σp(v/p)^ψ over every growing bud of the tree, in index order; each bud's size is (v/p)^ψ · Σp / Σp(v/p)^ψ. | Per PA it holds: Σp and Σp(v/p)^ψ, 2 × PAs scalars. | Per PA: the tree's two sums. | **No.** It regroups a tree-wide sum: first difference at cycle 3 to 6. |
| (iv) growth and waking | `grow/parallel.rs:70` and `grow/wake.rs:18`. Each apex's unit, read from its light, size and kept share; sleeping buds woken on the axes that carry their bearer. | Nothing: each apex reads only its own. | Nothing. | Yes, given that new axes keep the relative index order among the coarse apexes' children. To verify in a stepped executor. |
| (iv) layout | `grow/draw.rs:22`. Each axis laid from its parent's walk; new axes a generation at a time. | Nothing. | Its bearer's laid frame at its node (the pencil's walk ends there). | Yes. |
| (iv) re-lay, every 10 cycles | `grow/relay.rs:25`. Standing, presences, shed, scale, thicken, place, then sag levers and place again, with positions written back. | Its subtree's section at its bearer's node (the pipe sum in the bearer's exponent), its retained pipes, and its mass and the mass's first moment on the unbent tree. Also its standing and last living time; standing and shedding are max and integer reductions. | Its bearer's frame before and after bending; the lever its load sits on is turned inside the bearer. | Yes for sections, loads, standing and frames, all bearer-to-child in index order. **No** for the leaf-light girth's mean (`girth.rs:41`, a tree-wide Σpipe and Σpipe·light^χ): first difference at cycle 20. |
| (iv) light | `grow/sketch.rs:shade`. Every leaf of the cycle deposited into the lattice in gather order; the lattice swept; each living apex's tip read. | Its subtree's leaves of the cycle: positions and areas, deposited into the shared lattice. | The swept field, read at each of its buds. | **No.** It regroups each cell's deposit sum: first difference at cycle 2 to 7. The sweep and the reads are exact once the lattice is. |
| settle (once) | `grow/settle.rs`. Marks, presences, attachments and disused pipes, shed, scale, thicken with the grown tree's leaf mean, place, sag. | As the re-lay, plus its shed branches' radii as grown (retained pipes) and its leaf-girth partials. | As the re-lay. | As the re-lay: **no** for the leaf mean. |

Scalars per root per cycle, as found:
- **Up:** 3 for balance; 1 for demand; 2 × its PAs for the normalisers; and, in a re-lay, 1 section, 1 retained section, 1 mass and 3 for the moment.
- **Down:** 2 for balance; 1 for vigour; 2 × PAs (shared by every root); and, in a re-lay, the bearer's frame at the node.
- **Leaves:** each leaf of the cycle, 4 values, into the lattice. That is per-leaf data, not per-root data. The lattice is the exchange, as the design has it.

## Design question for the host: the three tree-wide sums

Bit identity across a cut is what probe (a) expects. With today's floating-point sums it cannot hold for the oak. The options:

- **A. Make the three sums order-free now:** accumulate them in fixed point, exact in any grouping, inside today's engine. This brings change 2 forward for these three sums only.
  - It changes today's trees at rounding level, under AGENTS.md "Generator evolution".
  - Probe (a) can then expect bit identity at every cut, order and chunking.
  - The lattice deposit needs integer deposit weights. That is the start of change 2's lattice part, which is the larger change.
- **B. Leave the arithmetic for change 2 and accept rounding-level drift until then.**
  - Measured so far: no topology change on four oak trees at three cuts.
  - Over 80 cycles a decision near its threshold could flip. That was not seen, but it is not excluded.
- **C. Exchange per-bud data for the normalisers** (per-leaf data already crosses for the lattice) so that the sums keep the tree's order. This is against Astra's change 3 and the design's "no per-node data".

The stepped executor itself (fn-209's CPU executor resumable cycle by cycle) is not built. The interface above is what it would implement. Whether it is fn-209's chain grower (committed mechanics, chains grown alone) made resumable, or the engine's own grower scoped to a root, is the host's call. Host decision 3 (realized mechanics) points to the second: under that decision a root grows and reports, and the CPU settles as today's engine does.

## After host decision A (2026-10-06): the three sums made order-free

`fixed/sum.rs` now accumulates all three sums. `allocation::sizes`, the lattice's deposits (`Field::deposit_all`) and the leaf-light mean (`girth::leaf_terms`, `girth/disused.rs`) take every term through it, and the partition's partials merge as integers.

- **What it moves** (`contract dump` of 28 trees against `38ced830`; `raw/task4/sums-vs-base.md`):
  - The beech, spruce and palm do not move at all.
  - The oak keeps its topology in all four trees. Its tips move at most 1.85e-11 m (RMS at most 5.9e-13 m) and its radii at most 1.0e-8 relative.
- **Probe (a) re-run** (`raw/task4/partition-sums.txt`, finished): every phase of every cycle is identical, and so is the tree, in all 432 runs.
  - Trees: the oak at seeds 1 and 7, ages 40 and 80, and the beech and spruce at the same seeds and ages.
  - Each at cuts 256, 1,024 and 4,096, combined in forward, reverse and shuffled order, in chunks of 1, 7 and 64.
  - The unpartitioned reference is today's tree to the bit for all 12 trees.
- **The oak's stills** at 80, seeds 1 and 7, bare and in leaf, against today's: 0 pixels differ out of 691,200 (`raw/task4/stills/sums-oak-80-{1,7}.png`, viewed).

## Host decision 2: the stepped executor, its build plan (not built yet)

The executor is the engine's own `Grower` scoped to one fine root. Each root is an item: its own axes, draws, pencils, live apexes and sleepers. Item axis 0 is a stub standing for the bearer; axis 1 is the root. A coarse `Grower` holds the structure above the cut and keeps a placeholder at each root's index, so its bearers' children stay in their order. One cycle runs the four phases through hooks the grower already has a place for:

1. **(i) Balance.** Each item sums its subtree's light, upkeep and living status (`balance.rs`, the sums only). The coarse grower injects these at the placeholder and decides the root's own shedding with the placeholder's memory. The root's shed flag and kept share go down, and the item decides its internal laterals with that as its root's.
2. **(ii) Demand.** Each item reports its subtree's `sub`. The coarse grower's `vigours_from` takes it as the placeholder's light and returns the root's incoming vigour. The item splits it with its root as the split's root (`links[1].parent = None`, so the base reaches it unrounded).
3. **(iii) Normalisers.** Every grower's per-PA `Sum` partials merge, the coarse grower's and the items' in any order, and every grower sizes its buds by the same `[Σp, Σp(v/p)^ψ]` (`read_sizes`).
4. **(iv) Growth, waking and layout.** Each grower grows its own apexes (`grow_live`) and wakes its own sleepers. The coarse grower's sketch classes new laterals at the cut and hands each one, with its framed pencil, its draws and its sleepers, to a new item. Each item lays its own axes (`draw`, `pencil`), the root from the handed pencil.
5. **(iv) The re-lay, every 10 cycles, and the settle.** The tree is assembled for today's `relay` and `settle`: the coarse axes in order, each placeholder replaced by its root, each item's other axes after. Then positions and walks are scattered back. Every reduction they take is either bearer to child, with each parent's children in their order, or order-free (`fixed::sum`), so the assembly order does not change them.
6. **(iv) Light.** Every grower's leaves are deposited into one lattice (`Sum`, order-free). It is swept once, and every grower reads its own buds.

The order argument that makes it bit-identical rests on one assumption, not yet checked by a run. Each grower's live list must keep the whole tree's live order restricted to that grower: apexes appended as they are made, and removed ones dropped in order. Then each parent's children are numbered in the same relative order in its grower as in the whole tree. The phase digests are now order-free (keyed by lineage), so the stepped run compares with `grow_partitioned(None)` cycle by cycle, phase by phase.

## The stepped executor built, and probe (a) through it (2026-10-07)

`grow/stepped.rs` (with `stepped/handover.rs`, `stepped/assemble.rs`) is host decision 2 as planned above. Today's passes were factored so it calls them, not copies of them: the grower's `start` and `year`; balance's `totals` and `shed_by` (axis 1 given by its bearer); the allotment's `inputs` and `set_sizes`; allocation's `split` (incoming and bud vigour, since `(v·x)/x` is not `v` to the bit), `subtotals`, `size_sums` and `sized`. Today's 32 trees (4 species, seeds 1 and 7, ages 40 and 80, lit and unlit) hash identically before and after (`measures hash`, `8a28b918` against HEAD).

- **The assumption:** each grower's living list is the whole tree's restricted to it, at every cycle's end (`each_grower_keeps_the_whole_trees_living_order`: oak 24 s7 lit, cut 256, 3,608 roots, whole tree and executor stepped side by side). It holds.
- **Probe (a)** (`examples/stepped.rs`; `raw/task4/stepped.txt`, `stepped-parallel.txt`): the oak lit at seeds 1 and 7, the beech and the spruce, at 40 and 80; cuts 256, 1,024 and 4,096; at 40 every order (forward, reverse, shuffled) with every chunking (1, 7, 64), at 80 each order once with another chunking. **Every phase of every cycle and the tree are the whole tree's to the bit in all 216 runs**, and again in the 54 runs at 80 after the items were spread over the cores. At 80 the items grow 97 to 99.7 % of the phytomers (oak s1 cut 1,024: 7,179 roots, 5,515,030 of 5,534,295).
- The beech and spruce read no light, so only the growth digest and the tree compare there.
- **What is checked by running, not by reading:** handover at birth (with a woken root's same-cycle chain and sleeping buds), the bearer-to-child exchanges of phases (i) and (ii), the merged normalisers, one lattice, and today's re-lay and settle on the assembled tree.

### Fidelity against today's oak

The executor's tree is today's by lineage, every field to the bit, at 40 and 80, seeds 1 and 7, so kept phytomers per order, per-PA growth, bud light per fine root, trunk base and order-1 radius are today's exactly (0 % against the bars of 10 %, 2 % and 10 %).

The stills (oak 80, seeds 1 and 7, bare and in leaf; `raw/task4/stills/`) are not identical in their own axis order: 0.5 to 1.1 % of pixels differ. Rendered in canonical order (`space_oak --canonical`, by depth then lineage identity), the stepped tree's stills equal today's to the pixel, all four. Today's own tree differs from its canonical rendering by 0.5 to 1.3 %. The difference is therefore the axis order alone: the still converter numbers pipeline nodes, and settles a relay's girth over laterals drawn after it, in axis order, and the assembly's order (coarse axes, then each item's) is a different parents-first order from today's creation order. Side by side (`pair-stepped-oak-80-{1,7}.png`, today left): viewed, indistinguishable.

### Cost (`examples/stepcost.rs`, cut 1,024; `raw/task4/stepcost-{all,one}.txt`)

Ryzen 9 5950X; "1 core" is `taskset -c 0`. The desk's load was 10 to 12 from the other workers on all cores and 5 to 11 on one.

| Oak | Today, all cores | Stepped, all cores | Today, 1 core | Stepped, 1 core | Roots |
|---|--:|--:|--:|--:|--:|
| 40 s1 | 0.34 to 0.39 s | 0.40 to 0.45 s | 0.58 s | 0.70 s | 1,821 |
| 40 s7 | 0.48 s | 0.59 to 0.61 s | 0.86 s | 1.12 s | 2,937 |
| 80 s1 | 3.63 to 3.68 s | 4.39 to 4.56 s | 7.79 s | 9.17 s | 7,179 |
| 80 s7 | 7.83 s (10.78 s once, load) | 9.74 to 11.04 s | 16.05 s | 20.07 s | 17,370 |

- **Per year:** the stepped years take 1.1 to 1.4 × today's (oak 80 s1, all cores: median 19 to 20 ms against 17 to 18; worst, the re-lay of year 70, 452 to 487 ms against 332 to 338). The re-lay years carry the assembly's moves. Assembly and settle: 0.89 to 0.91 s against today's settle 0.77 to 0.79 s.
- This is the CPU reference's cost, the items stepped as engine growers. It prices nothing of the GPU executor; probe (b) (synchronisation) is next in DESIGN-INTERLEAVE.md.

### Open for the host

1. **The axis order of an assembled tree.** Its renders differ from today's by the order alone. Either every build is put in canonical order before conversion (today's stills move once, by 0.5 to 1.3 % of pixels), or the executor reproduces today's creation order (a global rank per living apex and sleeping bud each cycle), or the converter is made order-free. A design choice; not taken.
2. **The task's acceptance** predates DESIGN-INTERLEAVE.md. Its third bullet (the aggregate against fine subtrees grown under recorded histories) has no subject once the aggregate is deleted, and the description's last step (lift the oak's refusal in fn-209's executors) waits on the GPU executor's stepping (probe (b)). Neither is done here.
