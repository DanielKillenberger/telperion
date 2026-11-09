# What a fine lateral really grows, by its light

## Conversation Evidence

> goal log, owner, 3 Nov 2026: "I think it's problematic that we're making core settings per preset. This should be a generalized solution that works across the tree space. This shouldn't be per species."
> goal log, owner, 9 Nov 2026: "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast."
> goal log, owner, 9 Nov 2026: "unless we can generate the twigs and stuff with a NN?"
> goal log, 9 Nov 2026: "The derivation from the tables alone is refuted: no preset holds at 1.0 m, because the tables give a twig's unshaded potential, 10 to 1,000 times what the tree realises."

## Goal & Context

Child of goal fn-321, serving R2, R4 and R11: a coarse dial that needs no row per preset. fn-334 showed that what a coarse tree's missing laterals would have carried cannot be computed from the species' tables, since the tables hold no light. Its row-free rule for which ages coarsen held and carried the speed. What is missing is a lateral's realised wood given its light, and this child measures whether that quantity can be supplied from what the grower already holds at a node. It is the go or no-go for a small model, and it trains none. [paraphrase]

Premise, not yet run: for a lateral of a fine age, the share of its tables' potential that its subtree really grows follows the light at its birth and its age closely enough that a curve or a model of a few local inputs can stand in for it. The council's evidence against (fn-245 COUNCIL.md) is that shedding and light depend on the twigs a stand-in replaces. [inferred]

Checked 9 Nov 2026: the grower has a working set that retires nothing (`Working::reference()`), used by `grow_cycle_is_the_reference`, and each phytomer records the light its bud grew in. Unknown: whether the tree that working set settles still holds the axes the settle would drop, and no public entry returns it. [inferred]

<!-- Source: 20% user / 50% [paraphrase] / 30% [inferred] -->

## Acceptance Criteria

- **R1 (frozen after a dry run):** the record's fields, the fine ages read, the light and age bins, the seeds and the verdict rule are committed in evidence before any read that counts. Before the freeze, the measurement runs on one tree per preset and its output is checked for sense; that dry run chooses nothing but the protocol's wording. [inferred]
- **R2 (the record):** for every lateral axis of a fine age that a full cycle ever grew, the shed ones included, one record: its age on the reference axis, the cycle it was born, the light at its birth, the cycles its apex lived, and what its whole subtree grew by age (phytomers, leaf area, section, wood length), beside the tables' potential for a bud of its age over the same span (fn-334's expectation, kept in its patch). A lateral whose record cannot be read is counted and named, never dropped silently. [inferred]
- **R3 (the read):** the oak, spruce, birch and beech at their preset ages, full cycle, on at least twelve seeds from 1001 up. Seeds 1 to 50 are not grown. [paraphrase]
- **R4 (how tightly it follows):** per preset and fine age, the realised share of the potential against the light at birth: its median and its spread within each light bin, how much of its variance light and age account for, and the same with the bearer's age or height added. The result says whether one curve fits all four presets or each needs its own. [inferred]
- **R5 (the verdict):** by the frozen rule, go, no-go or unresolved for a stand-in read from local state, with the numbers; no model is trained and no product row or behaviour changes. [paraphrase]
- **R6 (verification):** where code outside evidence stays in the tree, the gate is green and the codex review of the child's diff ends SHIP; code that only measures leaves as a patch in evidence, as fn-334's did. [paraphrase]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published.
- Heavy work runs through `telperion-heavy`.
- No value per preset or per species is fitted here; the read describes, it does not tune. [paraphrase]
- The grown tree is unchanged: a recorder may read the grower and may not alter what it grows. [inferred]

## Decision Context

Settled by the goal: one grower, the full cycle as the finest setting, a fast tree held to the full cycle's statistics. The base generator comes before any network (owner, 9 Nov 2026), and a small model is brought in only for what a calculation cannot hold. The host conducts and an Opus worker builds, escalating design questions (owner, 9 Nov 2026). [paraphrase]

From fn-334: the tuned rows hold the oak, spruce and birch within 3 to 5% at 5 to 10 times, so rows that work exist; the tables' potential overstates by 10 to 1,000 times; a protocol's formulas run on one tree per preset before they are frozen. [paraphrase]

## Open Questions

None.

## Result

Closed 9 Nov 2026, measurement only, built by an Opus worker through the work stage. Verdict by the frozen rule: no-go for a stand-in read from local state. With each species' own averages by age, span and light, the standing leaf, section and wood of the fine laterals of six unseen trees come out within 0.2% on the spruce, 5% on the oak and birch and 10 to 11% on the beech, with the worst position group at 10 to 12% on the broadleaves; span alone misses by up to 19%, and averages do not carry between species (54 to 75%). Found on the way: a lateral grows 0.43 to 1.21 of its tables' potential over its own span, and the settle sheds 55 to 66% of the finest; fn-334's "10 to 1,000 times" compared against a bud grown for the tree's whole age. The first read used three span bins by mistake and read near; it was repeated on the protocol's ten. No product code stays (the recorder is a patch in evidence); no gate or review, since the crates are unchanged. Evidence: RESULT.md, NEXT.md and cells.csv under .flow/evidence/fn-335-what-a-fine-lateral-really-grows-by-its/.

## Friction

- The recorder printed 134 MB of records for one spruce. Disposition: fixed in the child; the reader reduces the pipe to cell sums and no record reaches the disk.
- The shell hook refused a heredoc append and a checkout of the crates; the formatter check flags files already on the branch (3 minutes). Disposition: the hook is local setup; the format drift is the one fn-334 named, to ride with the next child that changes the core crate.
- The host's protocol wrote "1 to 8 each" for the span bins and the worker read it as one bin, found only when the host read the cell table (one repeat, about 5 minutes). Disposition: a protocol lists its bins one by one.

