## Conversation Evidence

> user (2026-09-16): "The tree has clear regular outline/border that doesn't look natural."
> user (2026-09-16): "We still haven't achieved the reference for the bare one either. It still looks off."
> user (2026-09-16, on the review's path): "ok let's do that."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 30% [paraphrase], 50% [inferred] -->

STRATEGY.md measures the generator's coverage against the Hallé and Oldeman architectural models. The European beech grows on Troll's model: its axes lean outward and relay one another, so its limbs spread wider low in the crown than high, and its crown edge is ragged where each limb system ends at its own reach. Telperion's scaffold builds every tree as one pitch from one band, with each primary axis running straight until it leaves the shell, which draws a symmetric fan under a smooth dome. The path review of 2026-09-16 found this is the first cause of the beech's bare and leaf-on mismatch. [inferred]

This spec adds the two scaffold rows the beech's bare photograph needs and nothing more: a lateral pitch that changes with height, and a primary reach that stops short of the shell by a share each limb draws for itself. A true sympodial relay is named here and left for a later spec if the beech's value round shows the rows cannot read as it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Pitch by height.** A habit row for the lateral pitch at the crown's top, beside `lateral_pitch` at its base, interpolated over the station's height in the crown. Neutral is the base pitch everywhere, which is today's tree to the byte. `scaffold.rs:263` draws the pitch with no height term today. [inferred]
- **A ragged reach.** A habit row for how far short of the shell a primary axis may stop, as a share of its room, drawn per axis from its own key. `reach` (`scaffold.rs:208`) measures the straight-line room to the shell; the row shortens it before the axis grows. Neutral 0 reaches the shell as now. [inferred]
- **Rows, not species.** Both are general habit rows with validated rails that every table may state. No count or angle cap is hardcoded. [paraphrase]
- **Relay, named not built.** Troll's model builds each axis by relay: a lateral bud takes over the extension, so the line zigzags. `crookedness` is a smooth wave about an axis's intent (`scaffold.rs:293`). If the beech's value round shows the wave cannot read as the photograph's zigzag, a relay spec follows. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Both rows exist with rails, are neutral byte-identical on every shipped table at two seeds and in the specimen views, and are refused by name off their rails, on the wire, blended, in the regenerated browser metadata and on the harness. Errors: a value off a rail is refused naming the field. [inferred]
- **R2:** On a synthetic family, a lower station's lateral leaves at the base pitch and a top station's at the top pitch, with the stations between graded; a ragged reach leaves each primary axis short of the shell by a share within the row, different axes by different shares, and the same seed the same shares. Errors: each property is its own failing test. [inferred]
- **R3:** The beech states both rows in the beech spec's next value round; this spec's own check is that the bare pair is rendered once with them and the implementer answers whether the lower limbs spread wider than the upper and the crown edge is ragged. Errors: none beyond the record; the owner judges in the beech spec. [paraphrase]

## Boundaries
<!-- scope: business -->

- No sympodial relay, no leaf-load bending (fn-59), no envelope change. [inferred]
- No species branch; the beech is a value table. [paraphrase]

## Resolved via Codebase

- Primary reach against the shell: `crates/telperion-core/src/branching/scaffold.rs:208-226` (`reach`, `planning.contains`).
- One pitch for every station: `crates/telperion-core/src/branching/scaffold.rs:263-266` (`lateral_pitch + pitch_variation * ...`).
- Crookedness as a wave: `crates/telperion-core/src/branching/scaffold.rs:291-294`.
- Coverage target: `STRATEGY.md:41` (the 23 Hallé and Oldeman models).
