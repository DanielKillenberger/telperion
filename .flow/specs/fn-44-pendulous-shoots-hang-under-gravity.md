## Conversation Evidence

> user (2026-09-15, on the round-5 S-WHOLE pair of the silver birch): "I think the worst part is the hanging curtains are not affected by gravity or smth. It's clearly wrong."
> user (2026-09-15, on the beech's second stem): "Second trunk is missing but that's coming later that's fine."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 40% [paraphrase], 20% [inferred] -->

The birch's curtain, as fn-37 made it, is a set of rods. A curtain lateral departs its limb in one fixed direction, across the crown and down by a droop, and then runs straight for its pendulous length; the owner looked at the round-5 pair and saw shoots pointing sideways-and-down that no weight has ever pulled on. A real weeping shoot is a slender stem bent by its own weight: it leaves the limb outward, its tangent turns toward vertical with every metre, and its lower half hangs straight down with the leaves strung along it. [paraphrase]

This spec adds that bend as one row, a sag, so that a table can say how far toward vertical a hanging shoot has turned by the end of its run, with the turn spread along the run as a smooth arc. Neutral is today's straight rod, so every shipped tree is untouched until its table says otherwise. The birch is the first specimen judged on the matched pairs; the spruce, which states the constants the hidden mode carried, stays byte-identical. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The sag is a turn along the run, not a departure angle.** `Curtain::direction` today returns the one direction a curtain lateral departs in; the shoot then advances straight (the run is planned once from `wanted` in `advance.rs`). With a sag, each step of a hanging shoot rotates its heading toward straight down by a share of the angle still between the heading and the down vector, the share set so that a shoot of its pendulous length has turned `sag` of the way to vertical by its end, eased so the bend is steepest near the attachment and the lower run is nearly straight, the shape a weighted slender stem takes. The turn lives in the pendant module beside the droop; the run planner asks the curtain for the heading of every step of a hanging run rather than once. [inferred]
- **The departure droop stays.** fn-37's droop is where the shoot starts; the sag is where it goes. At hang 1 and sag 0 the tree is fn-37's to the byte. [paraphrase]
- **The floor still binds.** A shoot bent to vertical spends its length against the clearance rule, which already stops a step short of the floor; a vertical shoot simply stops sooner. No new floor logic. [inferred]
- **Leaves hang along the shoot.** The canopy's orientation rails are signed since fn-37, so a leaf may hang under its shoot; nothing new is needed for the leaves to follow. [paraphrase]
- **Row.** `skeleton.twigs.sag`, 0 to 1, neutral 0: the fraction of the way to vertical a hanging shoot's tangent has turned by the end of its pendulous length. Railed and refused by name, on the wire, blended linearly, in the regenerated browser metadata, on the harness dial. [inferred]

## API Contracts
<!-- scope: technical -->

- **Family row** as above, validated by name, on the wire, blended, in the browser metadata, in the sweep's moved or held lists. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Sag zero is byte-identical to today on every shipped preset, asserted by the pins; the spruce and the birch at sag 0 are fn-37's trees. [paraphrase]
- **Monotone and smooth.** Along one hanging shoot the angle to the down vector never increases, no step turns more than the one before it, and a shoot never points upward. [inferred]
- **A curtain that is short.** A shoot whose run is cut by the floor or the shell has turned less than the row asks, which is correct: the sag is a rate over the pendulous length, not a promise about the end. [inferred]
- **Determinism and blend.** Same seed and rows, same curtain; a walk of the row from 0 to 1 bends the shoots continuously. [paraphrase]
- **File sizes.** `pendant.rs` is at 114 lines and `advance.rs` at 367; the per-step heading lands in the pendant module, and `advance.rs` may not grow past about 400. [paraphrase]
- **Cost.** A bent shoot is the same number of nodes as a straight one; the birch's node count is recorded beside round 5's. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `skeleton.twigs.sag` is a twig row with a rail of 0 to 1, neutral 0, refused by name outside it, on the wire, blended linearly and in the regenerated browser metadata and on the harness dial; every shipped preset is byte-identical at neutral. Errors: a value outside the rail is refused naming the field. [inferred]
- **R2:** With a positive sag a hanging shoot's heading turns toward straight down along its run, monotonically and as a smooth arc steepest at the attachment, so that a shoot of its full pendulous length has turned the stated fraction of the way to vertical by its end; the departure droop, the floor and the separation are fn-37's. Errors: a shoot that points upward or whose angle to the down vector grows along its run fails the test naming the seed. [paraphrase]
- **R3:** The silver birch's table states a sag against S-WHOLE and S-BARE, its matched pairs are rendered again with the numbers beside the previous round's in `.flow/evidence/fn34/REPORT.md`, and the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity on every preset, the rail, the monotone turn and the end angle on a synthetic hanging shoot at sag 1 and at sag 0.5, no step turning more than the previous, the floor still holding, determinism per seed and a blend walk of the row. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No wind, no motion, no dynamics; fn-15 owns movement. [paraphrase]
- No change to how many shoots hang, how long they run or how far apart they stand; those are fn-37's rows. [paraphrase]
- No second stem; fn-38 owns it. No shoot colour; fn-40 owns it. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's loop: species expose gaps, gaps become generator specs, the comparison runs again. The round-5 birch showed the curtain's rods; fn-37's rows made the curtain reachable and this term makes it hang. [user]

### Implementation Tradeoffs

- A turn per step over a curved run planned once: the run planner already handles a heading per step for twigs, and a per-step turn composes with the floor and the shell rejection without a new geometry. [inferred]
- One row, a fraction, over degrees per metre: a fraction of the way to vertical reads the same on a short curtain and a long one and blends like the other twig rows. [inferred]

## Strategy Alignment

- Follows "The catalogue": a species exposed an unsupported form and the generator gains a row, never a branch.
- Follows "Growth and botanical fidelity": the form is judged against the photograph on the matched pairs.

## Resolved via Codebase

- The departure direction of a curtain lateral: `crates/telperion-core/src/branching/local/pendant.rs:89-104` (`Curtain::direction`, the droop clamped to `DROOP_CAP` 0.35 times hang, `DROOP_SLOPE` 0.5).
- The straight run: `crates/telperion-core/src/branching/local/advance.rs:175-183` asks the curtain once for `wanted`; `:224-233` plans the run from it; twigs take a heading from `planner.heading` at `:185-191`.
- The floor and clearance: `pendant.rs:56-70` (`below`, `clear`, `CLEARANCE` 0.8); the stop-at-floor deferral in `advance.rs:199-220`.
- The rows and rails today: `crates/telperion-core/src/twigs.rs:39-48` (`hang` 0 to 3, `pendulous_length`, `pendulous_radius`, `curtain_separation`), rails at `:111-118`.
- The birch's curtain values: `crates/telperion-core/src/presets/species.rs:135-138` (hang 2.4, pendulous length 3.5, radius 1.0, separation 9); the spruce states the constants (hang 1).
- Signed canopy orientation rails since fn-37: `canopy.forwardLean`, `upward`, `leanRise`, `outward`.
- File sizes: `pendant.rs` 114, `advance.rs` 367, `planner.rs` 156, `seed.rs` 200.
