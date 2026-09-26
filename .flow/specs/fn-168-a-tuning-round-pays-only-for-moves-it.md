## Conversation Evidence

> owner (2026-09-26), on the host's proposal from fn-157's friction that a tuning round asks only about the dials its failing traits reach and checks candidates against the gating ranges before any paid look: "yes"

## Goal & Context
<!-- scope: business -->

A tuning revision should spend its Jev and vision budget on moves that can be kept. On the beech's first live revision (fn-157, 2026-09-26) every round asked Jev about all 227 rows of the dial table, the palm's fronds and spines among them, and from round 3 every candidate widened the crown past a gating range and was refused only after it had been proposed, rendered and looked at. The revision spent about 2.04 million Jev tokens over 63 calls and 7 visual passes to keep one round; a capped rerun spent about 0.86 million tokens on three rounds. A round asks only about the dials that can move what is failing, and a candidate that breaks a gating range never reaches a paid look. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, from fn-157's FRICTION.md and its run report (2026-09-26).** [checked]
- With no dials named in the tuning config, a revision offers every row of the dial table (227); the first proposal call asked all 227 in one request (302 KB) and was refused with a 400, after which the recorded configs set `max_questions_per_call` 32.
- From round 3 of the beech's revision every bundle failed the gating crown width; the runaway rule stopped the revision only after those rounds had been proposed, rendered and judged.

**Shape.** [inferred]
- **Dials a round asks about.** A round's proposal questions cover the dials that reach its failing traits: the rows the reviewer's findings map to, and the live dials of the species' current tree (dormant rows, such as a frond count on a broadleaf, are never offered). The config may still name dials; absent a list, this set is the default.
- **Numeric check before paid looks.** Each candidate is measured against the gating ranges before it is rendered for the reviewer; a candidate outside a gating range is refused and logged with the range it broke, with no vision call.
- **A round that can keep nothing ends the revision.** When every candidate of a round fails the same gate, the revision stops and names the gate, instead of proposing further bundles in the same direction.

**Unknown.** [unknown]
- How findings map to dials when a finding names a trait no row states; the implementer measures on the recorded beech and reports the offered set per round.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** On the recorded beech, each round offers only dials that reach a failing trait and none that are dormant for the species; the offered count per round is reported against 227. [inferred]
- **R2:** A candidate outside a gating range makes no vision call and is logged with the range it broke. [inferred]
- **R3:** A round whose every candidate fails the same gate ends the revision with that gate named. [inferred]
- **R4:** Replaying the beech's recorded revision costs no more Jev tokens and no more vision calls than today and keeps at least the same round; the saving is reported. [inferred]
- **R5:** The workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not what tuning aims at or how the reviewer judges, not the gating rules (fn-157), not the generator. The cost of a revision only.

## Strategy Alignment

- Serves "Minimalist af, efficient af and beautiful": a revision pays only for moves it can keep. [strategy:Our approach]
