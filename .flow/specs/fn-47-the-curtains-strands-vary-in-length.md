## Conversation Evidence

> user (2026-09-15): "If you can see it's clearly not there and you have a way to fix it you should do that."
> worker read, fn-44 (2026-09-15): "85% of hanging shoots run exactly 2.50 m ... No twig row varies a pendulous run per shoot."
> worker read, round 7 (2026-09-15): "A round ball on a V. The curtain stops at one height all the way round, where the photograph's curtain hangs lower on the leaning side ... it comes from the uniform strand length and the single floor height."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 50% [paraphrase], 25% [inferred] -->

Since fn-44 a weeping shoot runs its whole pendulous length and hangs to the crown's base, so every strand in the birch's curtain is the same 2.5 m and the curtain ends in one level hem all the way round: a ball on a stick. The photograph's strands run everything from a hand's width to three metres, and the hem is ragged, lower where the heavier limbs lean out. [paraphrase]

This spec varies the pendulous run per shoot, as a row a table can state, seeded so one seed is one curtain, with zero reproducing today's uniform curtain. The birch is judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **A variation row.** `skeleton.twigs.pendulous_variation`, 0 to 1, neutral 0: a hanging shoot's run is the pendulous length times one minus the variation times a per-shoot draw in 0 to 1, keyed by the shoot's own identity and the family seed, so a shoot's length never depends on the order the tree grows in. `Curtain::length` in `pendant.rs` applies it where the sag makes the pendulous length the run. [inferred]
- **The hem follows the wood.** A shoot hangs from its limb; a shorter one ends higher. With the variation set, the curtain's lower edge is as ragged as the draws and as low as the lowest limbs, so a leaning stem's heavier, lower limbs carry a lower hem by construction; no per-side term. [paraphrase]
- **Sag spends over the run it has.** The sag's arc is over the shoot's own run, so a short strand still ends near vertical. [inferred]

## API Contracts
<!-- scope: technical -->

- **Family row** validated by name, on the wire, blended linearly, in the regenerated browser metadata, on the harness dial, in the sweep's moved or held lists. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Every shipped preset at 0 is byte-identical, asserted by the pins. [paraphrase]
- **Determinism.** A shoot's draw is a function of its identity and the seed, not of growth order; the monthly and the one-shot build agree. [inferred]
- **Cost.** Shorter strands carry fewer nodes; the birch's node count is recorded. [inferred]
- **File sizes.** `pendant.rs` and `twigs.rs` have room; `advance.rs` may not grow past about 400 lines. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `skeleton.twigs.pendulous_variation` is a twig row with a rail of 0 to 1, neutral 0, refused by name outside it, on the wire, blended, in the regenerated browser metadata and on the harness; every shipped preset byte-identical at neutral. Errors: a value outside the rail is refused naming the field. [inferred]
- **R2:** With a positive variation, hanging shoots' runs spread between the pendulous length and that length times one minus the variation, seeded per shoot identity, with the sag's arc spent over each shoot's own run. Errors: a shoot longer than the pendulous length or a run that depends on growth order fails the test naming the seed. [paraphrase]
- **R3:** The silver birch's table states a variation against S-WHOLE and S-BARE, its matched pairs are rendered again with the numbers beside the previous round's in `.flow/evidence/fn34/REPORT.md`, the implementer does visual QA before returning, and the owner judges the pairs in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity, the rail, the spread of runs on a synthetic curtain, determinism per identity across build orders, the end angle of a short strand under sag 1, and a blend walk. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No change to the sag law, the floor or the separation; fn-44's and fn-37's. [paraphrase]
- No shoot colour; its own spec. No per-stem lean; its own spec. [paraphrase]

## Resolved via Codebase

- The run: `crates/telperion-core/src/branching/local/pendant.rs:84-90` (`Curtain::length`: at sag above 0 the run walks to `pendulous_length`).
- The twig rows and rails: `crates/telperion-core/src/twigs.rs` (`vigour_variation` at `:37`, default 0.15; the curtain rows beside it).
- The birch's curtain: `crates/telperion-core/src/presets/species.rs` (hang 2.4, sag 1, pendulous length 2.5).
- The round-7 read and numbers: `.flow/evidence/fn34/REPORT.md`, "Round 7".
