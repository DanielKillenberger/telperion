# Tune the oak's fine-wood dial to hold its statistics at the lowest cost

## Conversation Evidence

> goal spec, "The route now": "The next attempt tunes leaf area, pipe and scope on seeds 1000 and up, and judges on held-out seeds. It reports time against both R2's 50 ms and 0.1.4's."
> user, 1 Nov 2026: "You should try and achieve a goal and stop only when stuck to where it doesn't seem achievable in the way the goal was initially planned/needs major rethink."

## Goal & Context

Child of goal fn-321, serving R1, R2 and R4. fn-324 grew the oak with branches, twigs and shoots at fewer, longer phytomers, untuned: at factor 4 it costs 111 ms at age 40, the oak's preset age, with height 0.97, spread 0.91, limbs 0.77, trunk 0.40 and leaf area 0.11 of the reference. [paraphrase]

The premise, unmeasured: values can close those gaps within the cost the dial saves. This child searches the values on a changed species table through the one cycle, and it changes no shipped tree. [inferred]

## Acceptance Criteria

- **R1 (frozen before measuring):** each search round's values, seeds and objective are committed in this child's evidence before that round grows a tree, shown by commit order. A later round may use what an earlier one found. [inferred]
- **R2 (separate seeds):** values are chosen on seeds 1001 to 1005 only. The result is judged on 1, 7, 42, 1234 and 5678, which choose nothing. [inferred]
- **R3 (the numbers):** for the best candidate at each cost level, at age 40 and at age 80: the six R4 medians as ratios to the full cycle, and one-thread cycle seconds under fn-323's timing protocol. A metric that cannot be read is a failure. [inferred]
- **R4 (the reading):** a results file states the lowest measured cost at which all six medians are within 5% on the held-out seeds, against 50 ms and against 0.1.4's oak time, and whether going on looks achievable. Without a pass, it names what limits the result and the next attempt. [inferred]

## The search

- **Scope:** the first age scaled on the oak's reference axis, among its limb, branch and later ages. [inferred]
- **Factor:** node bounds divided and internode multiplied by the same `s`, so a unit keeps its length. [inferred]
- **Compensation:** leaf area per node and pipe per phytomer, each with its own multiplier, no longer tied to the unit's total. [inferred]
- **Further values:** a later round may add others of the species table if a round names the metric that needs them. [inferred]
- **Objective:** the largest departure from 1 over the six medians, read against cost, keeping the best candidate at each cost level. [inferred]

## Boundaries

- No shipped tree, preset table, shader or package entry changes; the search lives in an ignored measuring test kept in evidence. [paraphrase]
- No neural network, no closure. No threshold of the goal is lowered. [paraphrase]
- Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. Generated data stays in ignored `raw/` directories. [paraphrase]

## Decision Context

Settled by the goal: one grower with the fidelity as its dial and the full cycle as its finest setting; the coarse path on the CPU for everyone; a fast tree held to the full cycle's statistics, not its exact tree at that seed. A result does not stop the run; only being stuck does. [paraphrase]

Five seeds cannot read R4's interquartile limit, and this child reads the oak alone, so a pass here is not R4 met. It measures cycle cost only, with no field, Wasm or browser cost. [inferred]

## Open Questions

None.
