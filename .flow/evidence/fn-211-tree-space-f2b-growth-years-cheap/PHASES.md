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
