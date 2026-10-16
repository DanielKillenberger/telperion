# fn-209: how much of an engine tree the CPU keeps if its fine subtrees grow elsewhere (worker, 2026-10-06)

The host asked (2026-10-06): if the finest branch orders of an engine tree were grown on the GPU inside the pipeline's expansion stage, how many nodes would the CPU still handle? This file measures that. It changes no product code and proposes no design. GPU-PATH.md, beside it, measured the CPU preparation at about 0.21 to 0.26 µs per node per pass, three passes (rings, stations, curve).

## Answer in brief

- **The spruce splits cleanly.** Each order-2 shoot, with the order-3 shoots it bears, holds 60 to 250 phytomers. Each order-1 limb holds more than 4,096. With a cut at 256 phytomers, the CPU keeps the trunk and limbs:
  - At 80: 52k of 4.64M nodes (1.1%), about 36 ms of preparation at 0.7 µs per node, against the 3.9 s measured.
  - At 40: 12k of 1.15M nodes (1.0%).
  - The GPU then gets about 73k work items at 80 (18.5k to 19k at 40). Each item's nodes run p50 65, p95 100 and at most about 250.
  - Raising the cut to 4,096 barely changes it. Cuts at 64 or below leave 22 to 55% on the CPU.
- **The beech and oak have no clean break.** Their nodes peak at orders 4 to 6, and the CPU's share falls steadily as the cut grows:
  - At 1,024, the CPU keeps 3.7 to 5.2% (beech 80: 79k nodes in 7.3k items; oak 80: 36k to 81k nodes in 4.4k to 11.4k items).
  - At 256, the CPU keeps 16 to 21%.
  - At 4,096, the CPU keeps about 2%, in 3.1k to 7.6k items. The items are then larger and skewed (beech 80: nodes p50 262, p95 1,443, at most 4,043).
- **The reach cut does the same for the spruce at 1 m (1.1% left). For the broadleaves it does worse:**
  - At 2 m, the beech at 80 still keeps 11% on the CPU (176k nodes) and the oak keeps 5%.
  - Some items cut by reach are huge (up to 43k nodes and 114k phytomers, oak 80 at 2 m). A short reach does not bound the work inside an item.
- **Compared with the shipped presets:**
  - After the count cut, the CPU's share at 80 is smaller than the whole shipped tree's node count: spruce 52k against 96k, beech 79k against 213k, oak 36k to 81k against 134k to 144k.
  - It is larger than the shipped trees' orders 0 to 2 (13.5k to 24.7k nodes) and their structural nodes below the crossover (2.8k to 18.6k).
- **The palm is one axis** (480 or 960 phytomers) and has no lateral branch to cut.
- **What this leaves out:**
  - Growth still runs over the whole tree on the CPU (3.1 to 5.9 s at 80, BASELINE.md).
  - The conversion is not counted.
  - The fine subtrees' own growth and dressing are not measured.
  - The 0.7 µs per node figure is GPU-PATH's per-node cost times three passes. It is an extrapolation, not a timing.

## How measured

- **Build:** this worktree (`9f6c5138`), the `ci` profile (release without fat LTO), its own `target/`. Ryzen 9 5950X, 32 threads. Load 1.6 to 2.5. The figures are counts, so they do not depend on load.
- **Probe:** `crates/telperion-render/examples/space_split.rs`. Rerun it with `cargo run --profile ci -p telperion-render --example space_split -- [engine|presets|all]`.
  - It lives in the render crate, not in `telperion-space/examples` as the host suggested. A pipeline node exists only after `tree::convert`, which is example code in `crates/telperion-render/examples/space/tree.rs`, and `telperion-space` does not depend on the pipeline.
  - All 16 engine trees and 6 preset trees take 36 s in total.
- **Trees:**
  - `telperion_space::grow` for the beech, spruce, oak and palm, at ages 40 and 80 and seeds 1 and 7, with a 20M budget.
  - The oak grows under light 0.5,0.5, as in round 10. The others grow under neutral light.
  - Every figure is taken after shedding: the kept tree that is drawn.
- **Pipeline nodes:** the nodes of `tree::convert(structure, trunk)`, the root left out. `convert` pushes one node per drawn phytomer, in axis order and then phytomer order. A phytomer thinner than `FINEST`, or shorter than 1 mm, is merged and makes no node.
  - The probe matches each node to its phytomer by its exact position, and asserts that every node is matched.
  - The totals equal BASELINE's (spruce 80 seed 1: 4,637,481).
- **Branch order (engine):** the seed axis is order 0. A lateral is one order above the axis that bears it. A continuation or relay keeps its parent's order.
  - As a cross-check, the raw output also gives each pipeline tree's order by a geometric rule: the thickest child carries the axis on, and every other child is one order up.
  - The two rules agree within a few points. For example, the spruce at 80 has 54.4% of its nodes at order 2 by axis and 55.7% by the geometric rule.
- **Subtrees and cuts:**
  - A subtree is a lateral branch: a lateral axis, its continuations and relays, and every branch it bears.
  - For a cut, a branch is fine where the condition holds for it and not for the branch that bears it (the largest fine subtrees). Each such branch is one GPU work item.
  - The trunk's branch is never fine.
  - A lateral branch with no kept phytomers is no work and is left out: 3.6k to 197k of them per tree, listed per tree in the raw output.
  - The CPU's share is everything not inside a fine subtree.
- **Reach, not closed-form reach:** the closed form exposes no per-bud reach. `closed_form::expected_log_lengths` is `pub(crate)` and gives expected total wood length, not extent. Building one would change product code, so it was not done.
  - Section 3 therefore cuts by measured reach: the farthest kept phytomer tip from the branch's base on the grown tree.
  - A closed-form envelope would see an expected value before growth. This cut sees the realised one.
- **Presets:** `executor::grow(&family)` and then `.expansion()`, the pipeline tree the family builds, at seeds 1 and 7. The order is by the geometric rule above, because a structural node is a run of its own, so `Node::branch` does not give the order. The node counts equal BASELINE's (norway-spruce seed 1: 96,173 plus the root).
- **Raw data:** `raw/split/engine.md` and `raw/split/presets.md` (gitignored). They hold every table, for both seeds, including the PA tables, the geometric-order tables, and the empty-branch counts.

## 1. Nodes per branch order and per PA (engine, after shedding)

Pipeline nodes per axis order as a share of the tree. The right-hand columns are the nodes the CPU would keep if every order above it went elsewhere. Phytomer shares are within 2 points of the node shares. The fine orders lose some phytomers as too thin or too short to draw: the oak at 80 seed 1 has 287k phytomers at order 5 and 214k nodes there.

| Tree | Seed | Nodes | Order 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7+ | Orders 0-1 kept (CPU nodes) | 0-2 | 0-3 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 | 1 | 102.3k | 0.2% | 2.1% | 7.9% | 19.6% | 29.6% | 26.2% | 10.6% | 3.7% | 2.4k (2.3%) | 10.5k (10.3%) | 30.6k (29.9%) |
| beech 40 | 7 | 176.6k | 0.1% | 1.3% | 5.7% | 14.9% | 26.8% | 30.1% | 17.1% | 3.8% | 2.5k (1.4%) | 12.6k (7.1%) | 38.9k (22.0%) |
| beech 80 | 1 | 1.53M | 0.0% | 0.4% | 2.2% | 7.9% | 18.2% | 26.2% | 23.9% | 21.2% | 5.6k (0.4%) | 38.5k (2.5%) | 158.7k (10.4%) |
| beech 80 | 7 | 2.53M | 0.0% | 0.2% | 1.7% | 6.4% | 16.8% | 26.1% | 26.1% | 22.6% | 6.4k (0.3%) | 49.9k (2.0%) | 211.9k (8.4%) |
| spruce 40 | 1 | 1.15M | 0.0% | 1.0% | 40.8% | 58.1% | 0.0% | 0.0% | 0.0% | 0.0% | 11.9k (1.0%) | 480.7k (41.9%) | 1.15M (100.0%) |
| spruce 40 | 7 | 1.11M | 0.0% | 1.0% | 41.5% | 57.5% | 0.0% | 0.0% | 0.0% | 0.0% | 11.5k (1.0%) | 471.1k (42.5%) | 1.11M (100.0%) |
| spruce 80 | 1 | 4.64M | 0.0% | 1.1% | 54.4% | 44.5% | 0.0% | 0.0% | 0.0% | 0.0% | 52.1k (1.1%) | 2.57M (55.5%) | 4.64M (100.0%) |
| spruce 80 | 7 | 4.61M | 0.0% | 1.1% | 53.9% | 45.0% | 0.0% | 0.0% | 0.0% | 0.0% | 51.4k (1.1%) | 2.54M (55.0%) | 4.61M (100.0%) |
| oak 40 | 1 | 157.6k | 0.2% | 2.2% | 8.3% | 18.0% | 27.6% | 26.2% | 11.0% | 6.5% | 3.8k (2.4%) | 16.8k (10.7%) | 45.3k (28.7%) |
| oak 40 | 7 | 281.2k | 0.1% | 1.9% | 8.0% | 19.0% | 28.6% | 29.4% | 9.4% | 3.5% | 5.7k (2.0%) | 28.3k (10.1%) | 81.8k (29.1%) |
| oak 80 | 1 | 869.0k | 0.1% | 0.7% | 4.9% | 15.4% | 22.7% | 24.7% | 15.9% | 15.7% | 6.2k (0.7%) | 49.0k (5.6%) | 182.9k (21.0%) |
| oak 80 | 7 | 2.20M | 0.0% | 0.3% | 2.7% | 10.7% | 18.8% | 25.7% | 20.3% | 21.5% | 7.7k (0.3%) | 67.7k (3.1%) | 302.4k (13.7%) |

- **The spruce has four axis orders**, and only orders 2 and 3 carry mass: 41 to 54% and 45 to 58%. The order-3 axes are many and short, with about 2.6 phytomers each (825k axes at 80).
- **The beech and oak have up to 16 orders.** Most nodes sit at orders 3 to 6, and order 7 and above holds 3.5 to 23%.
- **At age 80, the seeds differ by up to 2.5x in size:** the oak has 0.87M and 2.20M nodes, the beech 1.53M and 2.53M. The shares hold within a few points.

By PA, the finest PA holds about half of every tree's nodes:

| Tree | Seed | Finest PA | Its nodes (share) | Next PA | Its nodes (share) |
|---|--:|--:|--:|--:|--:|
| beech 40 | 1 | 14 | 53.5k (52.3%) | 12 | 23.2k (22.7%) |
| beech 40 | 7 | 14 | 96.3k (54.5%) | 12 | 39.1k (22.2%) |
| beech 80 | 1 | 14 | 781.8k (51.3%) | 12 | 350.4k (23.0%) |
| beech 80 | 7 | 14 | 1.31M (51.6%) | 12 | 581.4k (23.0%) |
| spruce 40 | 1 | 12 | 666.9k (58.1%) | 10 | 384.8k (33.5%) |
| spruce 40 | 7 | 12 | 636.5k (57.5%) | 10 | 376.1k (34.0%) |
| spruce 80 | 1 | 12 | 2.06M (44.5%) | 10 | 1.63M (35.2%) |
| spruce 80 | 7 | 12 | 2.07M (45.0%) | 10 | 1.61M (35.0%) |
| oak 40 | 1 | 14 | 82.7k (52.5%) | 13 | 36.3k (23.1%) |
| oak 40 | 7 | 14 | 144.9k (51.5%) | 13 | 66.6k (23.7%) |
| oak 80 | 1 | 14 | 397.0k (45.7%) | 9 | 237.5k (27.3%) |
| oak 80 | 7 | 14 | 1.03M (46.8%) | 9 | 580.0k (26.3%) |

The finest PA is the beech's 14, the spruce's 12 and the oak's 14. It holds 45 to 58% of the nodes. The next PA holds another 22 to 35%.

## 2. Cut by size: every lateral branch with at most N kept phytomers is fine

"GPU items" is the number of fine subtrees. The CPU's phytomers and nodes are everything outside them. "CPU prep" is the CPU nodes times 0.7 µs, GPU-PATH's three passes at about 0.23 µs each. It is an extrapolation. The item columns are the fine subtrees' sizes.

| Tree (seed 1) | N | GPU items | CPU phytomers | CPU nodes | CPU share of nodes | CPU prep at 0.7 µs/node | Item nodes p50 / p95 / max | Item phytomers p50 / p95 / max | Seed 7: items, CPU nodes (share) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 | 1 | 0 | 113.2k | 102.3k | 100.0% | 72 ms | 0 / 0 / 0 | 0 / 0 / 0 | 0, 176.6k (100.0%) |
| beech 40 | 4 | 2.1k | 105.9k | 96.0k | 93.8% | 67 ms | 3 / 4 / 4 | 4 / 4 / 4 | 3.6k, 165.0k (93.4%) |
| beech 40 | 16 | 7.9k | 47.0k | 45.2k | 44.2% | 32 ms | 7 / 14 / 16 | 8 / 14 / 16 | 13.4k, 73.1k (41.4%) |
| beech 40 | 64 | 5.3k | 29.0k | 28.1k | 27.4% | 20 ms | 10 / 51 / 64 | 11 / 53 / 64 | 9.0k, 44.5k (25.2%) |
| beech 40 | 256 | 2.8k | 17.9k | 17.4k | 17.0% | 12 ms | 13 / 97 / 255 | 14 / 108 / 256 | 4.7k, 27.7k (15.7%) |
| beech 40 | 1024 | 570 | 4.0k | 3.9k | 3.8% | 3 ms | 60 / 674 / 989 | 67 / 744 / 1024 | 870, 6.3k (3.6%) |
| beech 40 | 4096 | 266 | 1.8k | 1.8k | 1.7% | 1 ms | 217 / 1176 / 3618 | 258 / 1298 / 3751 | 438, 3.1k (1.8%) |
| beech 80 | 1 | 0 | 1.60M | 1.53M | 100.0% | 1068 ms | 0 / 0 / 0 | 0 / 0 / 0 | 0, 2.53M (100.0%) |
| beech 80 | 4 | 24.2k | 1.51M | 1.45M | 94.9% | 1013 ms | 3 / 4 / 4 | 4 / 4 / 4 | 41.6k, 2.40M (94.8%) |
| beech 80 | 16 | 103.0k | 716.2k | 702.2k | 46.0% | 492 ms | 8 / 14 / 16 | 9 / 14 / 16 | 175.9k, 1.16M (45.6%) |
| beech 80 | 64 | 67.8k | 466.0k | 457.8k | 30.0% | 320 ms | 11 / 52 / 64 | 11 / 53 / 64 | 116.1k, 749.4k (29.6%) |
| beech 80 | 256 | 38.0k | 309.0k | 303.2k | 19.9% | 212 ms | 14 / 99 / 256 | 14 / 103 / 256 | 65.7k, 507.2k (20.0%) |
| beech 80 | 1024 | 7.3k | 81.7k | 79.2k | 5.2% | 55 ms | 79 / 716 / 1022 | 84 / 749 / 1023 | 11.5k, 122.4k (4.8%) |
| beech 80 | 4096 | 3.6k | 33.7k | 32.1k | 2.1% | 22 ms | 262 / 1443 / 4043 | 276 / 1514 / 4090 | 6.1k, 54.0k (2.1%) |
| spruce 40 | 1 | 21 | 1.18M | 1.15M | 100.0% | 803 ms | 0 / 1 / 1 | 1 / 1 / 1 | 14, 1.11M (100.0%) |
| spruce 40 | 4 | 204.9k | 481.1k | 477.6k | 41.6% | 334 ms | 4 / 4 / 4 | 4 / 4 / 4 | 197.0k, 468.2k (42.3%) |
| spruce 40 | 16 | 202.0k | 474.9k | 471.5k | 41.1% | 330 ms | 4 / 4 / 16 | 4 / 4 / 16 | 194.2k, 462.3k (41.7%) |
| spruce 40 | 64 | 154.4k | 257.3k | 255.4k | 22.3% | 179 ms | 4 / 21 / 64 | 4 / 27 / 64 | 147.1k, 246.0k (22.2%) |
| spruce 40 | 256 | 19.1k | 12.5k | 11.9k | 1.0% | 8 ms | 60 / 104 / 239 | 62 / 105 / 250 | 18.5k, 11.4k (1.0%) |
| spruce 40 | 1024 | 18.9k | 12.3k | 11.7k | 1.0% | 8 ms | 61 / 104 / 936 | 62 / 105 / 962 | 18.3k, 11.2k (1.0%) |
| spruce 40 | 4096 | 17.5k | 11.3k | 10.8k | 0.9% | 8 ms | 61 / 104 / 3981 | 62 / 105 / 4049 | 16.9k, 10.3k (0.9%) |
| spruce 80 | 1 | 68 | 4.78M | 4.64M | 100.0% | 3246 ms | 0 / 1 / 1 | 1 / 1 / 1 | 43, 4.61M (100.0%) |
| spruce 80 | 4 | 632.3k | 2.60M | 2.57M | 55.4% | 1797 ms | 4 / 4 / 4 | 4 / 4 / 4 | 628.9k, 2.53M (54.9%) |
| spruce 80 | 16 | 626.2k | 2.59M | 2.55M | 55.1% | 1788 ms | 4 / 4 / 16 | 4 / 4 / 16 | 622.8k, 2.52M (54.6%) |
| spruce 80 | 64 | 477.5k | 1.66M | 1.65M | 35.5% | 1152 ms | 4 / 40 / 64 | 4 / 42 / 64 | 474.9k, 1.62M (35.1%) |
| spruce 80 | 256 | 73.2k | 53.3k | 52.0k | 1.1% | 36 ms | 65 / 100 / 249 | 66 / 101 / 254 | 72.3k, 51.2k (1.1%) |
| spruce 80 | 1024 | 72.9k | 53.1k | 51.8k | 1.1% | 36 ms | 65 / 100 / 925 | 66 / 101 / 961 | 72.0k, 51.0k (1.1%) |
| spruce 80 | 4096 | 71.5k | 52.0k | 50.7k | 1.1% | 36 ms | 65 / 100 / 3985 | 66 / 101 / 4036 | 70.8k, 50.0k (1.1%) |
| oak 40 | 1 | 0 | 190.7k | 157.6k | 100.0% | 110 ms | 0 / 0 / 0 | 0 / 0 / 0 | 0, 281.2k (100.0%) |
| oak 40 | 4 | 3.2k | 179.4k | 149.6k | 94.9% | 105 ms | 3 / 4 / 4 | 4 / 4 / 4 | 5.7k, 264.5k (94.1%) |
| oak 40 | 16 | 13.4k | 78.4k | 69.1k | 43.9% | 48 ms | 7 / 14 / 16 | 8 / 14 / 16 | 22.0k, 123.4k (43.9%) |
| oak 40 | 64 | 7.8k | 45.2k | 40.9k | 25.9% | 29 ms | 11 / 50 / 64 | 12 / 53 / 64 | 12.9k, 70.7k (25.1%) |
| oak 40 | 256 | 4.5k | 29.6k | 27.1k | 17.2% | 19 ms | 15 / 89 / 256 | 24 / 104 / 256 | 7.2k, 44.5k (15.8%) |
| oak 40 | 1024 | 858 | 6.7k | 6.3k | 4.0% | 4 ms | 60 / 681 / 1013 | 90 / 737 / 1016 | 1.4k, 8.3k (3.0%) |
| oak 40 | 4096 | 454 | 1.9k | 1.8k | 1.1% | 1 ms | 183 / 1255 / 3597 | 276 / 1393 / 3783 | 873, 3.3k (1.2%) |
| oak 80 | 1 | 0 | 1.10M | 869.0k | 100.0% | 608 ms | 0 / 0 / 0 | 0 / 0 / 0 | 0, 2.20M (100.0%) |
| oak 80 | 4 | 18.1k | 1.03M | 830.7k | 95.6% | 581 ms | 3 / 4 / 4 | 4 / 4 / 4 | 43.3k, 2.09M (95.0%) |
| oak 80 | 16 | 71.0k | 511.7k | 441.1k | 50.8% | 309 ms | 5 / 13 / 16 | 8 / 14 / 16 | 166.9k, 1.08M (49.1%) |
| oak 80 | 64 | 41.6k | 316.5k | 284.0k | 32.7% | 199 ms | 10 / 48 / 64 | 13 / 52 / 64 | 98.0k, 672.2k (30.5%) |
| oak 80 | 256 | 23.3k | 198.2k | 182.6k | 21.0% | 128 ms | 15 / 96 / 256 | 25 / 148 / 256 | 54.6k, 412.7k (18.7%) |
| oak 80 | 1024 | 4.4k | 38.0k | 35.8k | 4.1% | 25 ms | 94 / 673 / 1019 | 177 / 757 / 1023 | 11.4k, 80.6k (3.7%) |
| oak 80 | 4096 | 3.1k | 17.0k | 15.9k | 1.8% | 11 ms | 161 / 952 / 3440 | 250 / 1073 / 3598 | 7.6k, 36.3k (1.6%) |

- **The spruce's step between 64 and 256:**
  - Below 256, a cut takes only some of each order-2 shoot's order-3 shoots. Their p50 is 4 phytomers, so there are up to 0.63M items at 80, and the CPU keeps a fifth to a half.
  - At 256, it takes whole order-2 shoots, and every limb stays above the cut.
  - At N = 1, there are only a few dozen single-phytomer branches.
- **For the beech and oak at N = 1 there are no items:** no lateral branch of theirs is a single kept phytomer. The cut at 4 takes only 5%. Their growth units are longer.

## 3. Cut by reach: every lateral branch whose reach is under R is fine

This cut uses the measured reach: the farthest kept tip from the branch's base on the grown tree. The closed form has no per-bud reach to use (see How measured).

| Tree (seed 1) | R (m) | GPU items | CPU phytomers | CPU nodes | CPU share of nodes | CPU prep at 0.7 µs/node | Item nodes p50 / p95 / max | Item phytomers p50 / p95 / max | Seed 7: items, CPU nodes (share) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 | 0.1 | 6.9k | 42.5k | 42.3k | 41.3% | 30 ms | 8 / 15 / 118 | 9 / 15 / 877 | 12.4k, 73.7k (41.7%) |
| beech 40 | 0.25 | 4.8k | 29.3k | 29.2k | 28.5% | 20 ms | 10 / 56 / 420 | 10 / 58 / 1024 | 11.4k, 64.0k (36.3%) |
| beech 40 | 0.5 | 3.0k | 19.3k | 19.3k | 18.8% | 13 ms | 12 / 72 / 1946 | 12 / 74 / 4807 | 6.6k, 36.9k (20.9%) |
| beech 40 | 1 | 1.5k | 11.4k | 11.4k | 11.1% | 8 ms | 14 / 225 / 3536 | 14 / 259 / 4807 | 4.7k, 27.8k (15.8%) |
| beech 40 | 2 | 390 | 3.4k | 3.4k | 3.3% | 2 ms | 50 / 760 / 17523 | 50 / 784 / 18060 | 1.5k, 11.5k (6.5%) |
| beech 80 | 0.1 | 95.8k | 695.5k | 692.2k | 45.4% | 485 ms | 8 / 14 / 167 | 9 / 14 / 1919 | 162.4k, 1.15M (45.5%) |
| beech 80 | 0.25 | 80.9k | 588.2k | 585.7k | 38.4% | 410 ms | 10 / 33 / 713 | 10 / 36 / 1995 | 133.1k, 950.3k (37.5%) |
| beech 80 | 0.5 | 46.9k | 386.8k | 384.7k | 25.2% | 269 ms | 12 / 69 / 3497 | 12 / 70 / 7561 | 80.1k, 637.3k (25.2%) |
| beech 80 | 1 | 32.6k | 295.0k | 293.2k | 19.2% | 205 ms | 13 / 91 / 5365 | 13 / 94 / 8982 | 54.4k, 491.3k (19.4%) |
| beech 80 | 2 | 14.0k | 177.4k | 175.8k | 11.5% | 123 ms | 29 / 415 / 17007 | 29 / 427 / 26461 | 20.9k, 267.3k (10.6%) |
| spruce 40 | 0.1 | 192.3k | 460.7k | 459.1k | 40.0% | 321 ms | 4 / 4 / 63 | 4 / 4 / 113 | 184.2k, 449.4k (40.6%) |
| spruce 40 | 0.25 | 115.3k | 358.8k | 357.9k | 31.2% | 250 ms | 4 / 33 / 122 | 4 / 35 / 124 | 112.5k, 352.0k (31.8%) |
| spruce 40 | 0.5 | 21.3k | 31.9k | 31.3k | 2.7% | 22 ms | 56 / 103 / 254 | 58 / 104 / 267 | 20.3k, 28.0k (2.5%) |
| spruce 40 | 1 | 18.4k | 12.0k | 11.4k | 1.0% | 8 ms | 61 / 104 / 2722 | 62 / 105 / 2799 | 17.8k, 10.9k (1.0%) |
| spruce 40 | 2 | 13.3k | 8.6k | 8.3k | 0.7% | 6 ms | 60 / 104 / 12121 | 61 / 105 / 15080 | 13.2k, 8.1k (0.7%) |
| spruce 80 | 0.1 | 596.7k | 2.52M | 2.51M | 54.2% | 1760 ms | 4 / 4 / 65 | 4 / 4 / 8282 | 597.8k, 2.48M (53.8%) |
| spruce 80 | 0.25 | 420.8k | 2.22M | 2.22M | 47.8% | 1552 ms | 4 / 4 / 124 | 4 / 4 / 16654 | 417.4k, 2.19M (47.5%) |
| spruce 80 | 0.5 | 100.1k | 383.5k | 382.2k | 8.2% | 268 ms | 50 / 96 / 2137 | 52 / 97 / 16654 | 98.7k, 392.2k (8.5%) |
| spruce 80 | 1 | 72.0k | 52.6k | 51.4k | 1.1% | 36 ms | 65 / 100 / 3078 | 66 / 101 / 16654 | 71.4k, 50.7k (1.1%) |
| spruce 80 | 2 | 67.8k | 49.7k | 48.7k | 1.1% | 34 ms | 65 / 99 / 8112 | 66 / 100 / 16654 | 66.7k, 47.7k (1.0%) |
| oak 40 | 0.1 | 8.7k | 70.0k | 69.3k | 44.0% | 49 ms | 8 / 26 / 256 | 9 / 36 / 13304 | 15.6k, 127.1k (45.2%) |
| oak 40 | 0.25 | 5.3k | 35.8k | 35.4k | 22.5% | 25 ms | 12 / 68 / 522 | 12 / 71 / 13304 | 10.7k, 71.0k (25.3%) |
| oak 40 | 0.5 | 3.5k | 26.2k | 26.0k | 16.5% | 18 ms | 19 / 99 / 1243 | 20 / 130 / 13304 | 7.1k, 51.1k (18.2%) |
| oak 40 | 1 | 1.7k | 15.0k | 14.9k | 9.4% | 10 ms | 27 / 368 / 10149 | 28 / 390 / 21365 | 3.4k, 28.2k (10.0%) |
| oak 40 | 2 | 637 | 7.4k | 7.4k | 4.7% | 5 ms | 52 / 894 / 10149 | 53 / 915 / 21365 | 1.3k, 11.4k (4.0%) |
| oak 80 | 0.1 | 40.2k | 420.3k | 407.8k | 46.9% | 285 ms | 8 / 37 / 313 | 9 / 51 / 2321 | 108.9k, 1.10M (50.1%) |
| oak 80 | 0.25 | 27.1k | 236.1k | 231.5k | 26.6% | 162 ms | 12 / 70 / 1629 | 12 / 82 / 10091 | 80.0k, 639.9k (29.0%) |
| oak 80 | 0.5 | 16.5k | 157.2k | 155.0k | 17.8% | 109 ms | 17 / 157 / 7042 | 19 / 233 / 19679 | 51.7k, 455.3k (20.7%) |
| oak 80 | 1 | 9.0k | 99.4k | 97.9k | 11.3% | 69 ms | 27 / 358 / 8518 | 28 / 407 / 19679 | 28.9k, 290.7k (13.2%) |
| oak 80 | 2 | 3.9k | 46.9k | 46.0k | 5.3% | 32 ms | 48 / 710 / 26107 | 49 / 765 / 114740 | 11.9k, 124.7k (5.7%) |

- **The spruce:** 1 m does what N = 256 does (1.1%, about 72k items). At 0.5 m, 8% stays on the CPU.
- **The broadleaves:** the CPU's share falls slowly with reach. At 2 m, 10.6 to 11.5% stays for the beech at 80, and 5.3 to 5.7% for the oak.
- **Item sizes are heavy-tailed:**
  - A branch that grew small or shrank (scale near 0) can hold thousands of phytomers inside 0.1 to 0.25 m. Most of them are too thin to draw: spruce 80 has an item of 16,654 phytomers and 124 nodes under 0.25 m.
  - A wide, dense branch under 2 m can hold tens of thousands of drawn nodes: the oak at 80 has items of 26k and 43k nodes.

## 4. The shipped presets: pipeline nodes per order

These are the pipeline trees the families build (`executor::grow` then `expansion`), ordered by the geometric rule: the thickest child carries the axis on. "Below crossover" is the structural (space colonization) nodes.

| Preset | Seed | Nodes | Below crossover (structural) | Order 0 | 1 | 2 | 3 | 4 | 5 | Orders 0-1 | 0-2 | 0-3 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| norway-spruce | 1 | 96.2k | 18.6k | 0.0% | 1.3% | 12.7% | 40.8% | 36.1% | 9.0% | 1.3k | 13.5k | 52.8k |
| norway-spruce | 7 | 91.1k | 17.6k | 0.0% | 1.4% | 13.2% | 41.9% | 35.2% | 8.3% | 1.3k | 13.3k | 51.5k |
| european-beech | 1 | 213.0k | 3.9k | 0.0% | 0.7% | 10.7% | 32.5% | 39.3% | 16.7% | 1.6k | 24.5k | 93.8k |
| european-beech | 7 | 215.5k | 3.9k | 0.0% | 0.7% | 10.7% | 32.7% | 39.3% | 16.6% | 1.6k | 24.7k | 95.0k |
| oregon-white-oak | 1 | 133.6k | 2.8k | 0.1% | 1.5% | 13.2% | 39.6% | 35.3% | 10.4% | 2.1k | 19.8k | 72.6k |
| oregon-white-oak | 7 | 143.7k | 3.6k | 0.1% | 1.2% | 11.3% | 38.2% | 38.5% | 10.8% | 1.7k | 18.0k | 72.9k |

- **The shipped trees have six orders under this rule.** Their mass sits at orders 3 and 4, like the engine's beech and oak at 40. The engine's beech and oak at 80 peak at orders 5 and 6.
- **Matching budgets:** for the engine's CPU share to match the shipped tree's whole node count (91k to 216k), the count cut is about:
  - N = 256 for the spruce, at 52k nodes;
  - N = 1,024 for the beech at 80, at 79k to 122k nodes;
  - N = 1,024 for the oak at 80, at 36k to 81k nodes.
- **Matching the shipped trees' orders 0 to 2** (13k to 25k nodes) needs N = 4,096 or more for the beech and oak at 80. That cut leaves 16k to 54k nodes.

## Not measured

- **Splitting inside an axis.** A fine subtree here is always a whole lateral branch. A cut along an axis (the tip of a limb as its own item) could move more off the CPU for the beech and oak. Measuring it needs a rule for where an axis is cut.
- **The GPU's side:** the work items' growth, their dressing and their upload. The cost of handing 4k to 600k items to the GPU is not known.
- **Growth.** The engine still grows the whole tree before any cut: 3.1 s for the spruce at 80 and 5.9 s for the oak at 80 (BASELINE.md).
- **A closed-form envelope per bud.** It does not exist as a public function (see How measured).
