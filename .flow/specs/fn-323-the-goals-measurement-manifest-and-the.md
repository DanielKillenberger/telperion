# The goal's measurement manifest and the coarse route's floor

## Goal & Context

Child 1 of goal fn-321 ("0.2.0: one grown tree at any scale, in realtime"). Every goal criterion needs one frozen definition of what is measured, and the goal's proposed route has an unmeasured premise: that a coupled scaffold, light included, can cost 50 ms or less at age 80 on the CPU, with the wood it leaves out standing as density in the field. [paraphrase]

This child answers those two things and changes no tree. It serves R1, R2, R4, R5, R6 and R7 through the manifest, and the goal's "Route stop" through its verdict. [inferred]

Known before it starts: the full cycle's oak at grid 36 takes about 240 ms at age 40 and 1,300 ms at age 80 on the device, and refuses ages 100 and 120 because the axes buffer passes 2 GiB. A CPU scaffold axis costs 900-2,270 ns a cycle (fn-245). No light cost on the CPU is known, and it may dominate the scaffold. [paraphrase]

## Acceptance Criteria

- **R1 (the manifest comes first):** a measurement manifest is committed in this spec's evidence before any measurement below runs, shown by commit order. It states the fidelity values, ages, cameras, viewport, the R5 walk, the first image's completion boundary, and the definitions of limb count and shed share. A later change to it is a dated, reasoned entry in the file, and no change follows a result it would affect. [paraphrase]
- **R2 (the stop is frozen too):** the same commit freezes finite cutoff and density candidates, a numerical limit on field error, and a repeated single-thread timing protocol. [paraphrase]
- **R3 (the two new metrics are read):** limb count and shed share are added to the measuring code as the manifest defines them, each with an ordinary test that fails when the value is wrong on a tree built for it. A tree it cannot read is an error, never a zero. [paraphrase]
- **R4 (what the view needs):** from existing full trees, with axes under the manifest's length skipped, and across the manifest's seeds: live axes by order and year, for the oak at ages 40 and 80. Each candidate's 36-cell field, drawn from orders up to k with the rest as density, is compared with the full tree's field against R2's limit. [paraphrase]
- **R5 (the floor's cost):** the scaffold's CPU cost per year and the light's CPU cost, measured separately under R2's protocol, for the oak at 40 and 80 and as far toward 300 as this machine reaches. Replay measurements are named apart from coupled growth, and a cost nobody measured is written as unknown. [paraphrase]
- **R6 (the full cycle's reach):** how far the full cycle reaches on each executor, with each refusal named by its limit and the age it comes at. [paraphrase]
- **R7 (the verdict):** a results file applies the goal's Route stop to these numbers and states one of: go on to child 2, naming the candidate; or stop `NEEDS_HUMAN`, with the numbers and next options. The stop applies when every candidate fails the field limit, when every passing candidate's scaffold-plus-light cost exceeds 50 ms at age 80, or when feasibility is unresolved at this child's limit of 3 hours. [paraphrase]

## Boundaries

- No shipped tree, preset table, shader or package entry changes; this child adds measuring code and evidence only. [inferred]
- It builds no closure, trains nothing and tunes nothing. "No neural network" holds throughout, as a network is outside the goal. [paraphrase]
- Generated data stays in ignored `raw/` directories, within the goal's 10 GB. Heavy work runs through `telperion-heavy`, one GPU job at a time. [paraphrase]
- No forest capture past the budget in `AGENTS.md`. [paraphrase]

## Decision Context

From the goal, settled and not reopened here:

- A fast tree is held to the full cycle's statistics and the owner's blind verdict, not to its exact tree at that seed (owner, 1 Nov 2026, "a is fine").
- One grower: the current cycle gains a fidelity dial; "we only want one."
- The coarse path runs on the CPU for every visitor (owner, 1 Nov 2026, chose option A). It depends on scaffold cost, which this child measures.
- The route is a proposal, not a requirement. If it fails its stop, the run reports and the bar stays.
- Evidence against the route: the light field and low-limb shedding depend on the twig density a closure replaces. R4 replays density on a tree grown whole, so it cannot show that coupling, and its results say so. [paraphrase]

Field bytes changed at fn-320. The manifest's base commit is at or after `e04c51ae5`. [inferred]
