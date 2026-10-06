# Tree space F2b: growth years, cheap

## Goal & Context

Retuning a tree in the harness near-instant (owner, 2026-10-06; STRATEGY.md, Build track) and an age slider that follows from it (owner, 2026-10-06: "sounds like that's the right architecture then. We 'just' need to figure out how to have growth steps be super efficient to compute"; "aye" to this spec). F1 (fn-209, rescoped) moves the finest orders into the expansion stage; what stays on the CPU is the structure above the cut, grown year by year. On that structure the cost is each year's whole-tree passes: the oak at 80 spends 1.2 s a retune in re-lays, light, allocation and balance over its coupling structure alone (fn-209 P4), mostly on one thread; today's whole oak grows in 5.6 to 6.0 s, the spruce in 3.1 to 3.3 s, the beech in 0.7 s (fn-209 BASELINE.md, GPU-PATH.md). F2 (fn-210) took the exact levers and left, largest first: the oak's rough layout, re-lays and settle under light; the spruce's and oak's settle (`assign`, `shed`, `scale`, `thicken`); the sag levers; the oak's shed subtrees it still grows (fn-210 RESULT.md).

## Design (host, 2026-10-06)

Two layers, measured in order.

1. **Exact parallelism for what F2 left on one thread.** Allocation, balance and settle are reductions over the tree; re-lays lay per chain; the light sweep's directions are independent (P4 made it parallel, bit-identical). Hash-identical, so verified by the hashes.
2. **Incremental years.** Each year today rescans and re-lays the whole tree, so a year costs as much as the tree. A year's change is mostly its new growth: the light grid is updated by the leaves that changed; allocation and balance keep running sums that only changed chains touch; a re-lay redoes the chains whose load changed. This changes how the work is done, not the botany; values may move at rounding level (AGENTS.md, "Generator evolution"), shown against the exact passes on the four species.

Design questions beyond these (a coarser time step for the light, the oak's shed subtrees under light) go to the host.

## Requirements

- **R1:** a per-year, per-pass profile of the structure F1 leaves on the CPU (and of today's whole tree, until F1 lands), every species at ages 40 and 80, 1 and all cores.
- **R2:** layer 1 with its hash proof and timings.
- **R3:** layer 2 with its comparison against the exact passes (kept phytomers per order, per-PA drift, junction radius, stills) and timings.
- **R4:** the end-to-end retune gate of fn-209 (parameter change to a completed frame), with F1's numbers once it lands; the age walk unchanged.
- **R5:** workspace gate; Codex review.

## Boundaries

Depends on fn-206 and fn-210. The fine orders are F1 (fn-209); integration is F4 (fn-198); dressing on the GPU is fn-125, fn-126 and fn-171.

## Host decision on layer 2 (2026-10-06, after PROFILE.md)

Layer 2 as designed is dropped (docs/principles.md, "Question, delete, then optimise"). PROFILE.md shows the only year passes whose cost follows the whole tree are the oak's: the full re-lay (1.0 s at 80, seed 1) and balance and allot (0.8 s). Under fn-209's cut (N = 1,024 kept phytomers) the oak's CPU structure is about 36k nodes, 4 % of the tree, and every one of those passes runs over that structure; and an incremental re-lay cannot be exact (a sagging limb changes its descendants' angle to gravity, and so their sag).

In its place: **the oak under fn-209's cut.** The CPU grows the structure above the cut year by year with today's passes; each fine root carries DESIGN-LIGHT.md's history-dependent aggregate state (Astra change 1) for light, allocation moments, balance and upkeep, and its light history is recorded for fn-209's executor. Measured against today's whole oak: cost per year and per build, kept phytomers per order, per-PA drift under 2 %, bud light per fine root, stills. P4's light error (p50 16-38 %, cause unnamed) was at a cut of 77k-375k fine roots; at 4.4k it is measured again, its cause found first.
