## Conversation Evidence

> user (2026-09-15, on the round-5b beech): "Also just much more thick core trunks for almost the entire height of the tree. Our tree thins out too quickly"
> user (2026-09-15, on the round-5c winter pair): "well we need to figure out how to get there. It's just not there. It's not appearing strong/large enough. Too many small branches. It does seem like there's less density on the reference actually. Fewer larger branches compared to ours which has many more thinner ones directly attached to the trunk"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 45% [user], 35% [paraphrase], 20% [inferred] -->

The beech in the winter photograph is a thick trunk that runs up through the crown, a few large limbs leaving it steeply, and fine twigs on those limbs. The generated beech, after rounds 5b and 5c on its habit rows, has the trunk and the steep limbs but thins out too quickly and carries too many small branches straight off the trunk. The owner wants the photograph's architecture: fewer, larger branches, and a core that stays thick for almost the whole height. [user]

The value tier cannot reach it. The row that keeps a leader thick is the fork exponent of the pipe model; raising it toward Murray's law gives exactly the core the owner asked for on the pair, and every protocol seed hits the 250,000-node ceiling, because the thicker wood stays above the twig threshold for one more generation and the twig layer spends the whole budget there. Halving the laterals per station, which is what the photograph asks for anyway, frees nothing: the twig layer takes what the limbs gave up. The twig rows that should bound it (twig diameter, bearing diameter, the limb handoff share) leave the fifth generation and the cap in place. The wood budget follows the radius, so any table that asks for girth is refused by the ceiling. [paraphrase]

This spec decouples them: the twig layer's depth and count are bounded by rows a table can state, independent of how thick the pipe model makes the wood, so the beech can have thick limbs and a bounded crown. The beech is the specimen judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Where the budget goes.** Seed 1 of the beech at fork exponent 1.8: 115,255 branches over four generations, 115,080 twigs, 193,836 nodes. At 2.8: 149,743 branches with a fifth generation of 35,655, 82,843 twigs, capped at 250,000. At 2.8 with two laterals a station: 60 first-order limbs instead of 106, a fifth generation of 73,627, capped. The twig law keeps branching a lateral while its radius is above the twig diameter and its length above the internode; the pipe model with a high exponent leaves the outer wood above that diameter for one more generation, and the `is_twig` test in `advance.rs` is the only thing that stops it. [inferred]
- **A generation cap that is a row.** `skeleton.twigs.generations` (1 to 6, neutral today's `MAX_LEVELS` behaviour): the twig-law generation at which a lateral is a twig whatever its radius. A table that states 4 gets four generations and its twigs at the fourth, however thick the pipe model leaves them. Neutral reproduces every shipped tree to the byte. [inferred]
- **Girth without a fifth generation.** With the generation row set, the beech's fork exponent may rise toward Murray's law and the twig layer stays where it was; the radius solve is untouched. The node estimate in `branching.rs` (`nodes_for`) learns the cap so the planner's budget is honest. [inferred]
- **Fewer, larger limbs are rows already.** `laterals_per_station`, `lateral_spacing` and `lateral_length_ratio` give the beech its few big limbs; this spec only makes them affordable. [paraphrase]
- **The ceiling stays.** `NODE_CEILING` is the owner's cost line and does not move here. [user]

## API Contracts
<!-- scope: technical -->

- **Family row** `skeleton.twigs.generations` validated by name, on the wire, blended as a count, in the browser metadata, in the sweep's moved or held lists. [inferred]
- **Species metrics** already report branches per order; the report records the beech's before and after. [paraphrase]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Every shipped preset at the neutral value is byte-identical, asserted by the pins. [paraphrase]
- **A generation cap never strands wood.** A lateral that becomes a twig by the cap still bears its twig stations and leaves; nothing is left bare. [inferred]
- **The planner's estimate agrees.** `nodes_for` counts generations to the cap so the node budget the planner reserves matches what grows. [inferred]
- **Determinism and blend.** Same seed and rows, same tree; a walk of the count row changes the tree at integer steps the way leaf lobe counts do. [paraphrase]
- **File sizes.** `advance.rs` is at 367 lines; the cap is one comparison beside the `is_twig` test and the row lives in `twigs.rs`. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `skeleton.twigs.generations` is a twig row with a rail of 1 to 6, neutral at today's behaviour, refused by name outside it, on the wire, blended as a count, in the regenerated browser metadata and on the harness dial; every shipped preset is byte-identical at neutral. Errors: a value outside the rail is refused naming the field. [inferred]
- **R2:** With the row set, no twig-law lateral above that generation exists on any fixed seed, whatever the pipe model's radii; the planner's node estimate counts to the cap. Errors: a branch beyond the cap fails the test naming the seed. [paraphrase]
- **R3:** The beech's table states the generation cap, two laterals a station further apart and longer, and a fork exponent near Murray's law; the 24-seed protocol passes with no seed capped and the DBH gate held; the matched pairs are rendered again with the numbers and the branches-per-order table beside round 5c's in `.flow/evidence/fn34/REPORT.md`; the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity on every preset, the rail, the cap on a synthetic family whose radii would otherwise branch past it, the estimate against the count, determinism per seed and a blend walk of the count. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- The node ceiling does not move. [user]
- No change to the radius solve or the pipe model; the exponent is a row the beech sets. [paraphrase]
- No shedding or self-pruning by shade; fn-16 and fn-21 own those. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's loop: a species exposed a coupling the value tier cannot undo, and the generator gains a row that bounds it. [user]

### Implementation Tradeoffs

- A generation cap over a radius-independent twig threshold: the twig law already keys its kind on generation for the level cap; a row on that path is one comparison, and it leaves the radius model whole. [inferred]
- A row over raising the ceiling: the ceiling is a cost line for every consumer, the row is a species' own choice. [user]

## Resolved via Codebase

- The twig kind test: `crates/telperion-core/src/branching/local/advance.rs:149-153` (`is_twig`: terminal, bearing lateral, radius at or under `twig_radius`, or length under the internode); the level cap at `:160-163` (`MAX_LEVELS`, `level_capped`).
- The twig radius: `advance.rs:35` (`t.twig.diameter / 2.0`); twig anatomy rows and defaults `crates/telperion-core/src/twigs.rs:7-24` (diameter 5 mm, bearing diameter 50 mm), rails at `:111-118`.
- The node estimate: `crates/telperion-core/src/branching.rs:165-180` (`nodes_for`, `child_radius`); the ceiling `branching.rs:23` (`NODE_CEILING` 250,000) and its clamp at `:157`.
- The pipe model: `crates/telperion-core/src/radius.rs:60-76` (a parent is the n-th root of the sum of its children's n-th powers, normalised at node zero), incremental form `radius/incremental.rs:113-116`.
- The beech's table: `crates/telperion-core/src/presets/species.rs` (habit rows, `twigs.laterals` 5, `length_ratio` 0.40, `fork_exponent` 1.8, `length_taper` 0.3).
- The round-5d attempts and their branch-order counts: `.flow/evidence/fn34/REPORT.md`, "Round 5d, attempted".
