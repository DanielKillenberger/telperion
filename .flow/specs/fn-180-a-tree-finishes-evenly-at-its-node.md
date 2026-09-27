## Conversation Evidence

> owner (2026-09-27), on the beech run's node-capped candidates: "why do we still have a node limit i thought we removed all arbitrary limits"
> owner, on the host's proposal that a tree finishes evenly at its budget and `maxNodes` becomes a dial: "ok"

## Goal & Context
<!-- scope: business -->

fn-53 removed the hidden hardcoded caps and kept the node budget as a named row, `maxNodes`, because a runtime generator needs a bound on work and memory. Two things now go wrong at it. The default (250,000, unset in every preset) is reached by the trees the new rows draw: fn-170's plane candidate reached it on every seed, and on the beech's runner Tune (fn-62, 2026-09-27) three of four candidates in round 1 and all in round 5 reached it. And a tree that reaches it stops growing wherever the count runs out, so branches end in blunt, cut-off stubs: the reviewer named "coarse, thick branches that end in blunt, cut-off stubs" on all four of the beech's failing traits. A tree must never look truncated because of a compute bound: at its budget it gives up fine detail evenly and stays whole, and a species can state the budget its look needs, at a measured cost. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-27 on master (`563b1534`).** `maxNodes` is declared at `crates/telperion-core/src/pipeline/branching.rs:172`: unset means 250,000 (`ranges.rs:4`), it blends, and it is excluded from the dial table ("a resource cap"). The scaffold stops growing the moment the node count reaches it (`pipeline/branching/scaffold.rs:138`) and sets `diagnostics.node_capped`; the one-shot colonization loop does the same (`pipeline/colonization.rs:205`, `:309`). fn-170 found that the mature build drains structural growth before twigs, so reaching the budget can suppress the local twig pass. [checked]

**Shape.** [inferred]
- **Finish evenly.** When a build would exceed its budget, it reduces fine detail uniformly across the whole tree, for example one twig generation fewer everywhere, or a uniformly sparser finest order, and then finishes every branch to its tips. No branch ends where the count ran out; `node_capped` then means "detail was reduced", with the reduction recorded.
- **Deterministic.** The same family and seed reduce the same way, and the reduction depends on the budget and the tree, never on build order.
- **The budget is a dial.** `maxNodes` joins the dial table with its cost stated (nodes, build time and peak memory measured per step), so Tune can raise it where a species' look needs more wood, and a preset states it where it binds.
- **Unknown.** Which reduction reads best (a whole twig generation, or a uniform thinning of the finest order), measured on the beech and the plane candidate before choosing. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** On fn-170's plane candidate and the beech's node-capped Tune candidates, a build at its budget has no branch that ends short of its tips; every axis reaches its twig ends, and the reduction applied is recorded. [inferred]
- **R2:** A tree under its budget is byte-identical to master on every shipped preset at seeds 1 and 7. [inferred]
- **R3:** Raising `maxNodes` walks the tree continuously toward the unbudgeted tree; `maxNodes` is on the dial table with its measured cost per step. [inferred]
- **R4:** Stills of the plane candidate and a beech candidate at the default budget, before and after, go to the owner with one question: do the stubs go away? [inferred]
- **R5:** The workspace gate and `npm test` are green; build time and peak memory at the budget are reported against master. [inferred]

## Boundaries
<!-- scope: business -->

- Not render level of detail (fn-176). Not removing the budget. Not the beech's values (fn-62).

## Strategy Alignment

- Serves "Mature trees are the product": a compute bound never shows as a truncated tree. [strategy:Our approach]
