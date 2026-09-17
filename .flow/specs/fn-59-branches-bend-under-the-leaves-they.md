## Conversation Evidence

> user (2026-09-16, on round 22 and the round-23 beech trials): "ok i think i know the problem. We have trees where the branches point upwards when there are no leaves. But they droop under the weight of leaves. That's one problem."
> user (2026-09-16): "But the bigger issue is with the hwhole tree that doesn't match the reference at all."
> user (2026-09-16, on the host's proposal to spec branches that bend under leaf weight): "yes"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 30% [paraphrase], 40% [inferred] -->

A broadleaf in leaf is not its winter skeleton with leaves added. The leaves weigh the limbs down, so a crown that stands upright and open when bare arches out and hangs low in summer. Telperion grows one skeleton and draws the same wood in both states, so its leaf-on beech is a vase of upright limbs holding the leaves high over a bare lower crown, where B-WHOLE's crown hangs almost to the ground. [paraphrase]

fn-54 reached for the photograph's low leaf mass by lowering the beech's crown base from 0.12 to 0.06 of the height, which pulled the winter crown down with it; B-BARE now forks into a broom a metre above the ground where the photograph stands on one trunk to about two fifths. A load that bends the wood in leaf lets the crown base go back to the bare photograph's while the summer crown still reaches down. [inferred]

This spec makes a tree's wood bend under the foliage it carries, as one general row every table can state, in the leaf-on state only. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **An in-leaf build.** The core builds a tree in leaf or bare. `mesh::build` (`crates/telperion-core/src/mesh.rs:72`) takes the state; bare is today's tree to the byte. The renderer's Bare view (`crates/telperion-render/src/view.rs`) keeps hiding foliage, and the runner builds the bare still from the bare tree, so the matched shots' `foliage: hidden` records draw unloaded wood and `leaf-on` records loaded wood. [inferred]
- **The load.** Each node carries the leaf area of every placement borne on it and its descendants, gathered once from the placement pass the way the pipe model gathers radius (`crates/telperion-core/src/radius.rs`). The leaf area is the element's own area times the placements, so a heavier or denser crown bends more with no species constant. [inferred]
- **The bend.** From the root outward, each node's subtree turns about its parent toward straight down, in the vertical plane of the branch, by an angle that grows with the carried load times its horizontal lever and falls with the wood's stiffness, taken as the fourth power of its radius (a cantilever). Angles accumulate down a limb, so a long laden limb arches. A vertical axis has no lever and does not bend; no node turns past straight down. [inferred]
- **Foliage follows the wood.** Placements are positioned on the bent wood, so a leaf keeps its place on its shoot. The curtain's `sag` (`crates/telperion-core/src/branching/local/pendant.rs`) is unchanged and applies after. [inferred]
- **One row.** `skeleton.leafLoad`, a stiffness scale with a validated rail, neutral 0: at 0 nothing bends and every shipped table is byte-identical in both states. No count or angle cap is hardcoded; the clamp at straight down is geometry, not a limit on growth. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The core builds a tree in leaf or bare, and at `leafLoad` 0 both states are today's tree to the byte on every shipped table, full builds at two seeds and the specimen views. Errors: a byte difference at neutral fails naming the table. [inferred]
- **R2:** At a positive row, on a synthetic family, a horizontal limb carrying leaves ends lower in leaf than bare; a vertical axis does not move; a thicker limb under the same load bends less; doubling the load bends further; no node passes straight down; and the bare tree is unchanged. Errors: each property is its own failing test. [inferred]
- **R3:** The row is refused by name off its rail, on the wire, blended, in the regenerated browser metadata and on the harness. Errors: a value off the rail is refused naming the field. [paraphrase]
- **R4:** The runner builds each matched still in its record's foliage state, and the build cost of the load pass is measured and reported beside the round's protocol. Errors: none beyond the record; cost is not a gate on form. [paraphrase]
- **R5:** The beech states the row and its crown base goes back to B-BARE's reading, the matched pairs are rendered through the full runner, the 48-case protocol passes, the implementer answers after looking whether B-BARE stands on one trunk and B-WHOLE hangs low like its photograph, and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words. [user]

## Boundaries
<!-- scope: business -->

- Leaf-on bending only; no wind, no snow, no seasonal growth of the wood itself. [inferred]
- No species branch in the generator; the beech is a value table that states the row. [paraphrase]
- The birch's curtain rows are not retuned here. [inferred]

## Resolved via Codebase

- One build per tree: `crates/telperion-core/src/mesh.rs:72` (`build` grows the skeleton, plaits the surface, places and culls the foliage).
- Bare is a renderer view over the same mesh: `crates/telperion-render/src/view.rs:10` (`View::{Whole, Bare, Leaf, Clay}`), set in `crates/telperion-render/examples/headless.rs:47`.
- The only droop today is the curtain's: `crates/telperion-core/src/branching/local/pendant.rs` (`sag`, `DROOP_CAP`).
- Which wood bears a leaf is known where limb systems clump: `crates/telperion-core/src/foliage/clumping.rs:141`, `foliage/placement.rs:197`.
- The beech's lowered crown base: `crates/telperion-core/src/presets/species.rs` (`crown_base: 0.06`, fn-54 WIP `fe9ebd4e`).
