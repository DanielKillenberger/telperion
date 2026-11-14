# The dial's rows are derived from the species

## Conversation Evidence

> owner, 3 Nov 2026: "I think it's problematic that we're making core settings per preset. This should be a generalized solution that works across the tree space. This shouldn't be per species."
> owner, 9 Nov 2026: "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast."
> owner, 9 Nov 2026: "yea i think we need a base generator that works before we can think of traininga a networK?"
> owner, 9 Nov 2026, on running the derived dial first and training a small model only on what it cannot hold: "alright"

## Goal & Context

Child of goal fn-321, serving R1, R2, R4 and R11. The dial coarsens a species through seven rows per preset (`species/coarse.rs`: first, factor, leaf, pipe, bud, hazard, shedding), found by search in fn-325, fn-326 and fn-328. The owner ruled against rows per preset. Presets without rows gain nothing from the dial, and Laurelin and the beech run slower at the coarsest than in full (fn-329). [paraphrase]

Read from code, 9 Nov 2026: `Species::coarsened` lengthens a coarsened age's internodes by a factor s and divides its units' nodes by s, with lateral probability per node unchanged. So a coarse unit bears 1/s of the laterals, and every subtree those laterals would have grown is missing. The `leaf` and `pipe` rows stand in for that missing wood (the test rows pair factor 2 with leaf 16). `closed_form` already computes the expected phytomers of a bud by age and cycle from the species' tables (`expected_counts`), with shedding outside the formula. [paraphrase]

Premise, not yet run: what the missing laterals would have carried is an expectation over the species' own tables, so the rows can be computed. For each coarsened age k with factor s_k:

- s_k is the requested metres over the age's internode length, at least 1 and at most the unit's mean node count, so no age needs a `first` row. [inferred]
- a node's leaf area becomes s_k times its own plus (s_k - 1) times the expected living leaf area of the laterals one of its nodes bears. [inferred]
- a phytomer's pipe keeps its unit's section (the existing s^(1/exponent)) and adds the missing laterals' expected section by the same sum. [inferred]
- a metre's upkeep adds the missing laterals' expected wood length, so the carbon balance sees the wood that is not grown. This replaces the `hazard` row with a conserved quantity. [inferred]

Unknown: whether shedding, which the expectation leaves out and light decides, moves the statistics past the limit; the `shedding` and `bud` rows have no derivation here and stay at 1.

## Acceptance Criteria

- **R1 (frozen first):** the derivation's formulas, the fidelity values (0.25, 0.5, 1.0 m), the seeds (1001 to 1012), the timing protocol and the verdict rule below are committed in evidence before any derived tree grows. Seeds 1 to 50 choose nothing. [inferred]
- **R2 (a probe, no product change):** a measurement inside the space or core crate builds the derived species for a preset and a fidelity from its tables alone, with no value tuned for any preset, and grows it through the one cycle. [inferred]
- **R3 (all eight presets):** for every preset at its preset age and each fidelity, a results file gives R4's six medians over the frozen seeds as ratios to the full cycle's on the same seeds, the worst departure, one-thread seconds, axes and apex-cycles, beside the tuned rows' read where a preset has them. A preset whose derived tree fails to grow is listed with the refusal. [inferred]
- **R4 (the verdict):** "holds" where a preset's worst departure at the coarsest is at most 5%, "near" at most 10%, "misses" past it, each miss naming its statistic. The result says for which statistics a general correction or a small trained model is needed, and for which none is. No setting may be slower than the full cycle; one that is, is named. [inferred]
- **R5 (the next step):** the result ends with the proposal: the derivation replacing `/space/coarse` rows in the product, a correction to it, or the gap a model would learn. No product row changes in this child. [inferred]
- **R6 (verification):** the gate is green and the codex review of the child's diff ends SHIP where code outside evidence changed. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- No value per preset or per species enters the derivation; a constant in it holds for every point of the tree space and is stated with its reason. [paraphrase]
- No network is trained here. [paraphrase]
- Heavy work runs through `telperion-heavy`. [paraphrase]

## Decision Context

Settled by the goal: one grower, the full cycle as the finest setting, a fast tree held to the full cycle's statistics. The route of 9 Nov 2026 puts this child first. Timing protocol as fn-328: one thread, pinned, native `--profile ci`, five repeats less the first, a grow over 0.5 s not repeated. The first profile of the cycle (`REVIEW-FABLE-2026-11-09.md` in the goal's evidence) puts no pass above 26%, so the saving has to come from growing less. [paraphrase]

Evidence against the premise: hand-written means failed the oak by 14 to 17% with half the sheds (fn-245 COUNCIL.md, forms (a) and (c)). Those replaced whole coupled years; this replaces only laterals inside a year's unit while every apex still grows year by year. Untested either way. [paraphrase]

## Open Questions

None.

## Result

Closed 9 Nov 2026, measurement only. The derivation as frozen is refuted: at 1.0 m under D1 no preset holds or comes near; five miss (leaf area worst on four, limbs on the beech) and three are refused before growth (the oak and spruce on upkeep, the palm on leaf area). The tables give the wood a bud could grow unshaded, far more than the tree realises (the oak's age 8: 1,012 times a node's leaf), so the missing quantity is a lateral's realised wood given its light. The rule for which ages coarsen held and carries the speed (the oak 55 times faster without the upkeep term, height within 10% on every preset). The tuned rows hold at 3 to 5% on the oak, spruce and birch. No product code stays; the probe is `probes/derived.patch`. Built by an Opus worker on the host's brief; no gate or review, since the crates are unchanged. Evidence: `.flow/evidence/fn-334-the-dials-rows-are-derived-from-the/RESULT.md`.

## Friction

- `cargo fmt -p telperion-core -- --check` is red on the base in `examples/early.rs`, `premises.rs` and `trunk.rs` (a minute). Disposition: an obvious fix, to ride with the next child that changes the core crate, since a commit there runs the whole gate; a format check in the gate is proposed to the owner.
- The host froze a protocol whose derived values left the engine's bounds on three presets, found only by the full read. Disposition: the next protocol runs its formulas on one tree per preset before it is frozen.

