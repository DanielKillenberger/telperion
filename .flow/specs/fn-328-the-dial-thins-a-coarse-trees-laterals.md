# The dial thins a coarse tree's laterals

## Conversation Evidence

> goal R2: "Every preset at grid 36 and seeds 1, 1234 and 5678, at least five warm samples each, has a median tree time at or under 50 ms and, for a species 0.1.4 has, at or under 0.1.4's."
> goal R4: "each have a median within 5 % of the reference's"

## Goal & Context

Child of goal fn-321, serving R2 and R3. fn-327 found the oak at fidelity 1 costs 165 ms over 45 k axes of about two phytomers each, with no dominant pass. The dial's `bud` row may only raise a lateral's probability, so no search has tried fewer laterals with more leaf on each. This child lets the row thin them and searches for rows that hold the statistics at less cost. [paraphrase]

Premise, not yet run: a coarse tree's cost follows its axes, so fewer laterals cut apex-cycles directly, while the limbs and trunk, grown as at the finest, hold R4's medians. [inferred]

## Acceptance Criteria

- **R1 (the row thins):** a `bud` row below 1 lowers a coarsened age's lateral probabilities by degree along the dial, and a row at or above 1 grows exactly as it did. Off its domain (not positive, or not finite) it is refused by name. [inferred]
- **R2 (a frozen search):** the search's bounds, score, tuning seeds and judgement rule are committed in evidence before it grows. It tunes on at least twelve seeds from 1001 up, and reads a candidate on seeds 1001 to 1050 before any held-out read. Seeds 1 to 50 of the goal's R4 choose nothing. [inferred]
- **R3 (the result):** a results file states, for the oak at preset age, the cheapest candidate within each departure of 5, 7, 10, 15 and 25%, with its axes, apex-cycles and one-thread seconds on the frozen protocol, against 50 ms and against the current rows' 245 ms. [inferred]
- **R4 (only if it helps):** where a candidate holds all six medians within 5% and costs less than the current rows on fifty tuning seeds, it replaces them, and the spruce and birch get the same search. Otherwise the product's rows stay as they are, and the thinning row stays only if a preset uses it. [inferred]
- **R5 (verification):** the gate is green and the codex review of the child's diff ends SHIP, where product code changed. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. [paraphrase]
- No species branch: thinning is a value in preset rows, dormant without them. [paraphrase]

## Decision Context

Settled by the goal: one grower; the coarse path on the CPU for everyone; a fast tree held to the full cycle's statistics, not its exact tree at that seed. A result is information for the next attempt and does not stop the run. Timing protocol: one thread, pinned, native `--profile ci`, five repeats less the first, a grow over 0.5 s not repeated. [paraphrase]

## Open Questions

None.
