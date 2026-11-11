# The corrected balance in the product, built as a candidate off the integration branch

## Conversation Evidence

> owner, 10 Nov 2026, on the charge for dead wood: "ok well then we need to fix this bug properly. And we don't need byte identical trees to come from the fix."
> owner, 11 Nov 2026: "i want you to visually gauge if it's roughly same visual fidelity or better ideally"
> AGENTS.md, owner, 8 Sep 2026: "Nothing is copied from a prototype or experiment without a rewrite and a test."

## Goal & Context

Child of goal fn-347, serving R1, R2, R6 and R7. fn-346's probes found a balance under which the oak pays only for living wood and reaches a steady state by its own economy (`.flow/evidence/fn-346-the-carbon-balance-charges-living-wood/R6-ANSWERS.md`, `R7-ANSWERS.md`, `SQUARE-RESULT.md`, `probes/square.patch`, variant SR). The host's verdicts for SR: the same as today's at 40 and 80, better at 120 on both seeds, every other preset the same, and from 200 worse on one seed and not worse on the other, because the settle's sag lays the tree down on wood whose girth has fallen. Under the goal's bar (no tree worse) SR may not enter the integration branch on the host's word alone. [paraphrase]

This child prepares SR as product code on its own branch, proved and reviewed, so that the owner's decision in the morning is one merge and not a build. Nothing here lands on the integration branch. [inferred]

## Acceptance Criteria

- **R1 (the scope on paper, and back to the host):** a note naming every place the change touches: the balance's sums and their order, the walk that sheds, the main line's test, where a tip's path is carried and what it costs to carry, the species value and every preset, the captures and bindings that carry `upkeep` today (Wasm, the device grower and whatever holds the CPU to it, the browser source), every test and baseline that pins the balance or a grown tree, and the docs. For each: what changes, and what is at risk. It also says how long the build and its gate are expected to take. No code before the host answers. [inferred]
- **R2 (the rule, rewritten):** in the one cycle, with no second path: wood that has died adds nothing to any balance; a living tip's charge is its size times the square of its path from the root collar over the species' potential height; a lateral subtree's sums take its own tips and those child laterals that pay; an axis that continues or relays its bearer is tested on its own subtree as a lateral is, and only the seed's own axis is never shed. The memory, the hazard, `kept`, tolerance and the idle rule read that balance as they read today's. [paraphrase]
- **R3 (one species value):** the per-metre `upkeep` is replaced by the species' potential height, a measured trait with its source in the preset's comment (the oak's 34.8 m and the others as fn-345 sourced them). A species without the value is not charged, which is the neutral value; the charge moves the tree by degree with it. No species or template branch. [inferred]
- **R4 (tests):** an ordinary test holds that a limb whose twigs have died and been let go is charged what the same limb without them is charged (goal R1); a test holds the charge's form on a small hand-built tree; a test holds that a subtree with one paying and one failing child is not shed while the failing child is; the tests and baselines the change moves are updated, each with the reason. [inferred]
- **R5 (the same trees as the probe):** on the oak, seeds 1001 and 1007, at 40, 120 and 300, the product's held phytomers and living tips match the probe's SR census within 1%, or the difference is explained. [inferred]
- **R6 (verification):** the gate is green and the `codex` review of the diff ends SHIP, both inside the queue. [paraphrase]

## Boundaries

- Work only in the worktree `.worktrees/fn-349` on branch `fn-349-the-corrected-balance-in-the-product`. Nothing is merged, cherry-picked or pushed; the integration branch is not written to. [inferred]
- No rule A, B or G; girth, sag, the insertion angles, the foliage and the renderer's drawing are not changed. [paraphrase]
- No cap, no value fitted per species, none tuned to a sheet. One cycle, one pipeline. [paraphrase]
- Heavy work runs through `telperion-heavy`, whose admission rules are never overridden. This worktree has its own `target/`.

## Decision Context

Settled by the host from fn-346's measurements: the form of the charge (square, at the potential height), the unit of decision (what would remain), the main line tested. Listed as the host's estimates for the owner: the exponent 2 (the source, McDowell et al. 2002, read in abstract only, states partial compensation), and reading the potential height against path. [paraphrase]

Not settled: whether SR lands before the old tree's mechanics are fixed. That is the owner's. [inferred]

## Open Questions

None.
