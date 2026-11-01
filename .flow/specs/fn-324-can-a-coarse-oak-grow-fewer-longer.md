# Can a coarse oak grow fewer, longer phytomers and stay the same tree?

## Conversation Evidence

> goal log, 1 Nov 2026 14:50 Zurich, option A: "one more route, 3 hours, under its own frozen stop: the dial changes growth itself. A coarse tree grows fewer, longer phytomers from the first year, so the whole cycle stays coupled and each year costs less."
> user, 1 Nov 2026 17:53 Zurich: "A"

## Goal & Context

Child of goal fn-321, on the owner's answer A. It serves R1, R2 and R4. fn-323 stopped the cut-and-density route: no candidate passed the field limit at both ages, and the only pass cost seconds against 50 ms. No route to those criteria remains. [paraphrase]

The premise, unmeasured: a tree grown by the one cycle from fewer, longer phytomers, with the same wood length and leaf area per growth unit, costs far less per year and holds R4's statistics. This child runs that as a probe and answers go or no-go. It changes no tree. [inferred]

The goal stops at 19:00 Zurich on 1 Nov 2026, with its last 30 minutes for evidence. Option A named 3 hours and the answer came at 17:53, so this child has until 18:30, and a verdict it cannot reach by then is written as unresolved. [inferred]

## Acceptance Criteria

- **R1 (the stop comes first):** the candidates, limits and verdict rule below are committed before any candidate is grown, shown by commit order. [inferred]
- **R2 (the probe is parameters only):** each candidate grows through the one cycle from a changed species table, in an ignored measuring test kept in this spec's evidence and not on the workspace's test commands. [inferred]
- **R3 (the numbers):** for the oak at ages 40 and 80, seeds 1, 7, 42, 1234 and 5678: the reference's and each candidate's medians of height, crown spread, trunk diameter, limb count, shed share and leaf area, and one-thread cycle seconds under fn-323's timing protocol. A metric that cannot be read is a failure. [inferred]
- **R4 (the verdict):** a results file applies the rule below and states one verdict. [inferred]

## The candidates, frozen

A candidate is a factor `s` and a scope. Factors: 1.5, 2, 3, 4. Scopes: **all** ages on the oak's reference axis, or **fine**, its branch, twig, shoot and short ages only, so the limbs keep their phytomers. That is 8 candidates. On each scaled age: [inferred]

- the internode length is multiplied by `s`;
- each zone's node bounds are divided by `s`, so a unit keeps its length and has fewer nodes and fewer laterals;
- the leaf area per node is multiplied by `s`, so a unit keeps its leaf area;
- the pipe per phytomer is multiplied by `s^(1/exponent)`, so a unit keeps its pipe area.

No other value moves, and nothing is tuned after a result.

## The verdict rule, frozen

Applied to the medians over the five seeds. Five seeds cannot read R4's interquartile limit, so this probe does not read it. [inferred]

- **Go:** a candidate has all six metrics within 5% of the reference at both ages, and cycle seconds at or under 50 ms at age 80. A later child builds the dial.
- **Worth tuning:** no candidate goes, but one has cycle seconds at or under 50 ms at age 80 and all six metrics within 25% at both ages. These values are untuned, so a miss this size does not settle the route. It is reported to the owner as unresolved, with the metrics that miss.
- **No-go:** neither. The route is written as failed for this transformation, with the numbers.
- **Unresolved:** the probe has no reading for every candidate by 18:30 Zurich.

Every verdict but Go ends the run `NEEDS_HUMAN`, since the goal's stop falls before any build could finish. A Go does too, and names the child to build next. These limits are a judgment: 5% is R4's, 50 ms is the goal's Route stop at age 80, and 25% is the host's.

What this cannot show: the cycle's cost only, with no field, Wasm or browser cost, so a Go is not R2 met. The probe has one species, so a Go is not R4 met for all presets.

## Boundaries

- No shipped tree, preset table, shader or package entry changes. [paraphrase]
- No neural network, no closure. [paraphrase]
- Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. Generated data stays in ignored `raw/` directories. [paraphrase]

## Decision Context

Settled by the goal and not reopened: one grower with the fidelity as its dial and the full cycle as its finest setting; the coarse path on the CPU for everyone; a fast tree held to the full cycle's statistics, not its exact tree at that seed; the bar stays if a route fails. [paraphrase]

fn-323's evidence on this route: order 4 and deeper hold 93% of the apex-cycles at age 80 on seed 1, and the shed share is 0.80 to 0.82, so most cost is short-lived fine wood. Fewer laterals per unit could fall geometrically with order. Whether light and shedding answer the same way when each node carries more leaf area is unknown. [paraphrase]

## Open Questions

None.
