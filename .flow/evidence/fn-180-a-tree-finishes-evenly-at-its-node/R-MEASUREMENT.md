# fn-180 measurements

2026-09-27, worker, task .1. Tool: `crates/telperion-core/examples/node_budget.rs`
(`BUDGET_FAMILY=<overlay> [BUDGET_MAX_NODES=n] [BUDGET_GENERATIONS=g] [BUDGET_MESH=1] node_budget <preset> <seed>`).
Cases: `plane-candidate.json` (fn-170's plane with fn-177's `girthHold` 0.75 and `girthFall` 4) over the oak, and
`beech-candidate.json` (fn-62 r3, round 1 `bundle@2`, case `candidate-a49edc4aab1c`) over the beech. The beech's
nine other node-capped r3 candidates are also capped on this base. Raw lines are in `raw/` (ignored).
A stub is a local branch node with no child: an axis that ends on its own wood.

## The Unknown: a whole twig generation fewer, or thinning the finest order

The trees without a budget, and at each stated `generations`:

| case | no budget | generations 2 to 6 | generations 1 | default budget |
|---|---|---|---|---|
| plane, seed 1 | 594,144 | 594,144 | 267,322 | 250,000 |
| plane, seed 2 | 580,353 | 580,353 | not run | 250,000 |
| beech a49edc4aab1c | 526,267 | 506,431 at 2, 526,267 at 3 to 6 | 68,539 | 250,000 |
| beech f413972cefa0 | 797,137 | 756,786 at 2 | 72,215 | 250,000 |

- Removing a whole generation does not fit the plane. At one generation the plane still has 267k nodes, over
  its 250k budget.
- On the beech, removing a whole generation drops the tree from 526k to 68k nodes, 27% of the budget.
- Neither tree's twig layer is as deep as its row allows. The generations the oak and the beech actually grow
  (one or two) are set by radius and length, so the row's integer steps are few and very large.
- **Chosen: uniform thinning of the finest order.** Its whole-generation points are the generation row, so it
  includes the first option and fills the steps between them.

## What was built

Detail is counted in sixteenths of a twig generation. Whole generations are grown from the scaffold outward.
Inside the next generation, a share of its laterals still branch and the rest end as twigs, drawn from each bud's
own key. Below one generation, the share is of the first generation's twigs, and the rest are not grown. The
direct build tries levels from the same scaffold: whole generations first, then a bisection inside the one that
does not fit. It keeps the highest level that finishes inside the budget and records it as
`diagnostics.twig_detail`.

## R1: stubs at the default budget

| case | master stubs | fn-180 stubs | no-budget stubs | fn-180 nodes | detail kept |
|---|---|---|---|---|---|
| plane, seed 1 | 55,626 | 178 | 417 | 243,039 | 14/16 of gen 1 twigs |
| plane, seed 2 | 62,066 | 169 | 507 | 229,481 | all gen 1 twigs |
| beech a49edc4aab1c | 34,087 | 585 | 1,446 | 232,109 | gen 1, 6/16 of gen 2 |
| beech f413972cefa0 | 52,394 | 444 | 1,424 | 236,530 | gen 1, 4/16 of gen 2 |
| beech cfa0b4339a9c | 12,367 | 271 | 556 | 228,518 | gen 1, 8/16 of gen 2 |

In every case the reduced tree has fewer stubs than the tree with no budget. Structural tips with no child rise
where detail falls below one generation: on the plane at seed 1, from 36 to 51. These are tips whose terminal twig
was refused and whose lateral twigs were thinned. They end tapered, like any structural tip.

## R2: under budget

The skeleton digests of every shipped preset and the in-work beech, at seeds 1 and 7, are identical to master.
Every one of them is under its budget.

## R3: sweep of `maxNodes` (seed 1)

| budget | plane nodes | plane detail | beech nodes | beech detail |
|---|---|---|---|---|
| 100k | 97,037 | 2 | 96,883 | 17 |
| 150k | 145,750 | 6 | 149,271 | 19 |
| 200k | 194,444 | 10 | 176,712 | 20 |
| 250k | 243,039 | 14 | 232,109 | 22 |
| 300k | 288,342 | 17 | 284,776 | 24 |
| 400k | 389,323 | 22 | 395,968 | 28 |
| 500k | 490,987 | 27 | 479,057 | 31 |
| 550k | 532,716 | 29 | 526,267 | none (whole) |
| 600k | 594,144 | none (whole) | 526,267 | none (whole) |

Node counts and detail both rise monotonically, and they reach the unbudgeted tree once the budget holds it.

### The dial's cost per step (whole mesh request, medians of 2, seed 1)

| budget | plane nodes | plane ms | plane peak MB | beech nodes | beech ms | beech peak MB |
|---|---|---|---|---|---|---|
| 250k | 243,039 | 1,669 | 608 | 232,109 | 4,993 | 736 |
| 300k | 288,342 | 2,112 | 761 | 284,776 | 5,591 | 879 |
| 400k | 389,323 | 2,484 | 959 | 395,968 | 7,342 | 1,181 |
| 500k | 490,987 | 2,891 | 1,141 | 479,057 | 8,704 | 1,379 |

A 50,000-node step (the dial's small step) adds 0.19 to 0.44 s on the plane and 0.60 to 0.88 s on the beech to the
build, and 90 to 155 MB of peak memory: about 0.2 to 0.9 s and 90 to 150 MB.
The dial's `ask` and the row's note say so. The dial window is 100,000 to 1,000,000 nodes, and an unset row
steps from `ranges::DEFAULT_MAX_NODES`.

## R5: cost at the default budget (whole mesh request, medians of 3, seed 1)

| case | master total ms | fn-180 total ms | master skeleton ms | fn-180 skeleton ms | master peak MB | fn-180 peak MB |
|---|---|---|---|---|---|---|
| plane | 1,003 | 1,887 | 206 | 862 | 620 | 590 |
| beech a49edc4aab1c | 5,127 | 4,898 | 212 | 1,016 | 851 | 727 |
| beech f413972cefa0 | 5,620 | 5,192 | 285 | 1,338 | 902 | 756 |

The skeleton is regrown from its rows for each level it tries (5 or 6), and every try stays under the budget, so
at the budget it is four to five times slower than on master. The beech builds fewer leaves than the truncated
master tree, so its whole request is faster and uses less memory.

## Size: the slim field module

CI's size check refused the first version: `dist/telperion-field.wasm` grew from 367,819 bytes (base `b6181634`) to
396,792 (+28,973, +7.9%). The `twiggy diff` of the two builds with names kept put almost all of it in the specimen's
clone. The first version saved the scaffold by cloning the whole `Specimen` and grew each level from that copy, and
the derived `Clone` pulled the retained-growth state's clone code into the module:

| cause | bytes (named build) |
|---|---|
| `Specimen::clone` | +14,121 |
| clones of the frontiers, queues, maps and identity trees it reaches | about +10,800 |
| `Tree::clone`, drop glue, the function-name table | about +6,300 |
| the search itself (`within_budget`) | +1,958 |

Each level now regrows the tree from its rows, which draw the same scaffold, and nothing is cloned. The module is
369,691 bytes (+1,872, +0.5% over base). Every budget case and every preset's skeleton digest is identical to the
first version (plane seeds 1 and 2, beech seeds 1 and 7, oak at 50,000 and 3,000, and ordinary at 6,000). Regrowing
the scaffold costs time at the budget: plane skeleton 568 to 862 ms, beech 809 to 1,016 ms.
