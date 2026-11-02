# The cycle costs less per bud, byte for byte

## Conversation Evidence

> goal R2: "Every preset at grid 36 and seeds 1, 1234 and 5678, at least five warm samples each, has a median tree time at or under 50 ms and, for a species 0.1.4 has, at or under 0.1.4's."
> goal R3: "1,000 distinct oaks at grove fidelity and their preset age generate natively in at most 15 s on this machine"
> goal R11: "No setting gets slower than this goal's base"

## Goal & Context

Child of goal fn-321, serving R2, R3 and R11. fn-326's dial reaches 259 ms on the oak, 765 ms on the spruce and 222 ms on the birch at the coarsest, against R2's 50 ms. fn-325 measured cost following apex-cycles at 2.2 to 3.4 us each, with balance, units and sketch about 60% of it. This child makes each apex-cycle cheaper without changing what grows, so the gain carries to every preset and every fidelity. [paraphrase]

Premise, not yet run: the cycle repeats allocation and whole-tree scans it does not need to. Whether they are where the time goes is what R1 measures. [inferred]

## Acceptance Criteria

- **R1 (where the time goes):** before any change, a results file names the cycle's cost by pass and, inside the largest passes, by the operation, for oak, spruce and birch at preset age, on one thread, at fidelity 0 and 1. A claim about what is slow names a measurement. [inferred]
- **R2 (the same trees):** an ordinary test holds a digest of the grown structure on all eight presets, at a young age, at fidelity 0 and 1, and it passes unchanged from before the first optimisation to the last commit. A heavier check reads the same digest at preset age on seeds 1, 1234 and 5678 for oak, spruce and birch, before and after. [inferred]
- **R3 (the gain, stated):** a results file gives one-thread seconds on the frozen timing protocol for oak, spruce and birch at preset age, at fidelity 0 and 1, base against candidate in alternating batches on held-out seeds, each against 50 ms. No setting is slower than base by R11's rule. [inferred]
- **R4 (verification):** the gate is green and the codex review of the child's diff ends SHIP. [inferred]

## Boundaries

- No change to what a tree is: a change that moves a digest is out of this child, whatever it saves, and is recorded as a lead. [inferred]
- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. [paraphrase]

## Decision Context

Settled by the goal: one grower; the coarse path on the CPU for everyone; the full cycle as the finest setting. The result of this child is information for the next attempt, and no size of gain stops the run. Timing protocol: one thread, pinned, native `--profile ci`, five repeats less the first, a grow over 0.5 s not repeated. [paraphrase]

## Open Questions

None.
