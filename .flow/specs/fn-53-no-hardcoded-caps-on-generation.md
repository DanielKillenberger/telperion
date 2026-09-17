# No hardcoded caps on generation

## Conversation Evidence

> user (2026-09-15): "why do we have a node cap?"
> user (2026-09-15): "shouldn't we avoid having arbitrary caps? but just make sure the generator produces expected results without assertions of any kind by keeping parameter ranges clean and accepting that some trees may need some more performance. We rather improve performance than arbitrarily cap things that have impact on generation."
> user (2026-09-15): "if the node count is a parameter that can be set i guess that's fine but not hardcoded separately from ranges for params that are configurable. Makes sense?"
> user (2026-09-15): "ok can you capture another spec that cleans up those caps/constants and makes sure the project follows this."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 15% [user], 70% [paraphrase], 15% [inferred] -->

While fn-31 was being judged, the owner asked why the ten-year oak test carried a node cap. The answer showed a pattern: limits that have nothing to do with a tree's form had been steering the growth rule. They included count gates in tests, a build-time ceiling, and constants inside the generator. Several form fixes stalled against them. The owner set the rule on 2026-09-15, and CLAUDE.md now carries it: a count or limit that changes what the generator grows is a configurable parameter with a validated range, never a constant hardcoded apart from those ranges. "We rather improve performance than arbitrarily cap things that have impact on generation." [paraphrase]

fn-31 removes the count gates in its own tests and the cap in its own shedding rule. This spec takes the rest of the project and makes sure it follows the rule from here on. [paraphrase]

A host inventory on 2026-09-15 found these hardcoded limits in the generation core: [paraphrase]
- A 250,000-node ceiling that silently overrides the configurable node budget, and that caps the internal node estimate.
- A 4,096-unit limit on the steps of one scaffold axis.
- A twelve-generation limit on twig depth.
- A bend-resolution floor that silently clamps the writhe wavelength and the spiral rate for a given step size.

The inventory also found limits the rule already allows, because each is a configurable setting with a default: [paraphrase]
- the one-shot colonization node budget;
- the foliage instance budget;
- the chronicle's history retention;
- the age range.

## Architecture & Data Models
<!-- scope: technical -->

- **A limit that shapes the tree is a parameter.** Every limit that changes what the generator grows is a named numeric parameter. It has a validated range, every family carries it, the blend walks it, and presets author it where it binds. Its bounds live in the parameter table, in one place. [paraphrase]
- **A value inside its range takes effect as given.** No separate constant clamps, shortens or overrides it. A value outside its range is refused, naming the field, as parameters are refused today. [paraphrase]
- **Resource safety is a range bound, not a hidden ceiling.** A build that reaches a limit the caller set stops with the existing capped diagnostic, so the limit is visible and deliberate. [paraphrase]
- **Numeric tolerances are not caps.** Floating-point epsilons, convergence thresholds and similar arithmetic guards do not change what grows, so they stay as constants. [inferred]
- **Tests check form, not counts.** A test may assert that a parameter is honoured, for example that a set node budget stops growth there. It never gates a preset's result on a node, leaf or population count. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **fn-11 is closed.** The spec's growth-over-time contract says the node ceiling stops growth with a capped diagnostic. This spec keeps that behaviour, but for a caller-set node budget rather than a hidden constant. [inferred]
- **The largest mature tree is near the old ceiling.** The mature oak reaches about 185,000 nodes, and fuller crowns under discussion in fn-31 reach about 216,000. Any preset whose default node budget binds must author its own value, rather than being cut off by a separate ceiling. [inferred]
- **Removing a cap can make a build slower.** The owner accepts that cost in principle, but it is measured and the path is optimized rather than capped again. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every constant and hardcoded limit in the generation core and the Wasm bindings is inventoried and classified as one of three kinds: a validated range bound of a named parameter, a numeric tolerance that does not change what grows, or a cap on generation. The classified inventory is recorded, and no cap on generation remains at the end. [paraphrase] Errors: an item that cannot be classified is recorded as a cap and handled as one.
- **R2:** The node budget is one configurable parameter whose validated range carries its upper bound. No separate node ceiling overrides it. Presets author the budget wherever the default would bind. A build that reaches a caller-set budget stops with the existing capped diagnostic. [paraphrase] Errors: a budget outside the range is refused, naming the field.
- **R3:** The scaffold step limit, the twig depth limit and the bend-resolution floor each either become a named parameter with a validated range or are removed. A value inside a parameter's range takes effect as given. [paraphrase] Errors: a value outside the range is refused, naming the field.
- **R4:** Every preset builds either the same tree as before or a documented change. Where removing a cap changes a preset's output, the record names the preset, the change and the reason. The pins move once, after convergence is recorded. Determinism and native-to-Wasm parity hold. [inferred]
- **R5:** Build cost for every preset at its derived mature age is measured before and after, with the machine's load recorded beside each sample. Where removing a cap raises the cost, that path is optimized rather than capped again, and the record states the remaining difference. [paraphrase]
- **R6:** Tests in the core assert form and parameter contracts, not a preset's node, leaf or population count. Every count-gated test outside fn-31's own is rewritten to check the form it was protecting, or removed. Each rewrite or removal carries its reason. [paraphrase]
- **R7:** A guard keeps the rule in force. A test fails when the generation core gains a new limit that is neither a range bound in the parameter table nor a declared numeric tolerance. [inferred] Errors: the failure names the new limit and the two places it could legitimately live.

## Boundaries
<!-- scope: business -->

- fn-31's own shedding cap and its own count gates are fn-31's to remove, not this spec's. [paraphrase]
- No new generation behaviour. Removing a hidden clamp may change a preset's output, and R4 records that change, but no parameter gains a new meaning. [inferred]
- The history retention setting, the age range and numeric tolerances are not caps, so they stay as they are. [paraphrase]
- Renderer and GPU-side limits are out of scope. The rule covers what the generator grows. [inferred]

## Decision Context
<!-- scope: both — conditionally substructured -->

### Motivation
<!-- scope: business -->

The owner prefers performance work over caps: "We rather improve performance than arbitrarily cap things that have impact on generation." [user] A count or limit is acceptable when it is a parameter a preset can set. It is not acceptable "hardcoded separately from ranges for params that are configurable". [user] The rule exists because limits unrelated to form had been steering the growth rule in fn-31, and the fixes to real visual defects stalled against them. [paraphrase]

## Strategy Alignment

STRATEGY.md's approach already asks that "every generator parameter is a numeric trait that acts on every tree". Its Growth and botanical fidelity track notes that "counts and continuity tests alone cannot catch" structural mistakes. This spec removes the constants that stand outside that parameter space, and it moves tests from counts to form.
