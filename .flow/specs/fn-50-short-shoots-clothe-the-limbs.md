## Conversation Evidence

> user (2026-09-15, on the beech winter pair): "Too many small branches. It does seem like there's less density on the reference actually. Fewer larger branches compared to ours which has many more thinner ones directly attached to the trunk"
> host read, fn-45 round 6b (2026-09-15): "The leaf mass is a flat-topped umbrella on the top third; the lower two thirds of the crown are bare grey limbs ... The foliage reads as feathery fronds."
> worker, fn-45 round 6c (2026-09-15): "The leaf mass stops at about a quarter of the height; the photograph's reaches about 2 m ... twig wood only grows at limb tips and on thin wood ... A twig is a fixed 25 cm with a leaf every 2 cm, which is a fern's pinna; a beech's shoots are nearer 10 cm ... A shoot that carries a cluster of leaves without a node per leaf would be generator work, not a row."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 45% [paraphrase], 30% [inferred] -->

A beech, like most broadleaves, carries most of its leaves on short shoots: spurs a few centimetres long that grow a cluster of two to five leaves each year, all along its older limbs and deep inside the crown. The long shoots at the ends make the crown's extent; the short shoots make it full. Telperion grows only long shoots: leaves sit on twig wood, and twig wood grows at limb tips and on thin wood, so a few big limbs, the architecture the owner asked for, leave the inside of the crown bare, and each twig is a long even pinna with a leaf every two centimetres. After six value rounds the beech has the photograph's structure bare and its silhouette leaf-on, and no row can leaf it down through the crown. [paraphrase]

This spec adds short shoots: leaf clusters along limb and branch wood below a radius, spaced and sized by rows, costing one placement per cluster and not one node per leaf. Neutral grows none. The beech is judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Short shoots are placements, not skeleton.** A short shoot is a point on existing wood with a direction, a length of a few centimetres, and a cluster of leaves at its tip; it adds no node to the skeleton and no run to the surface, so the node ceiling is untouched. The foliage placement already places leaves along twig wood (`crates/telperion-core/src/foliage/placement.rs`); short shoots are a second placement source over structural and branch wood. [inferred]
- **Rows.** On the canopy: `short_shoot_spacing` (metres between short shoots along the wood; 0 neutral, none grow), `short_shoot_radius` (the wood thicker than this fraction of the stem radius carries none), `short_shoot_length` (metres), `short_shoot_leaves` (leaves per cluster, 1 to 8, blended as a count), and the cluster's spread angle. Railed and refused by name, on the wire, blended, in the browser metadata, on the harness. [inferred]
- **Light and depth.** Short shoots on wood deep inside the crown follow the same retention as leaves do today (`shell_depth`, the crown index), so the interior fills as far as the table's retention lets it and no further. [inferred]
- **Seeded and stable.** Each short shoot's position along its wood, its bearing and its cluster are drawn from the wood's own identity and the seed, so the monthly and one-shot builds agree and a blend walks them continuously. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Spacing 0 grows no short shoot; every shipped preset is byte-identical, asserted by the pins. [paraphrase]
- **Budget.** Short shoots cost foliage instances, not nodes; the beech's instance count is recorded and must stay inside the foliage fidelity band the protocol already checks. [inferred]
- **Trunk bare where it should be.** The trunk below the crown base and wood thicker than the radius row carry none. [inferred]
- **File sizes.** `placement.rs` and `advance.rs` are near 400 lines; short shoots land in their own module. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The short-shoot rows exist with rails, neutral at spacing 0, refused by name outside them, on the wire, blended, in the regenerated browser metadata and on the harness; every shipped preset byte-identical at neutral. Errors: a value outside its rail is refused naming the field. [inferred]
- **R2:** With a positive spacing, clusters of leaves grow on short shoots along every piece of limb and branch wood under the radius row, above the crown base, retained by the canopy's existing rules, adding no skeleton node. Errors: a short shoot on wood thicker than the row or below the crown base fails the test naming the seed. [paraphrase]
- **R3:** The beech's table states short shoots against B-WHOLE and B-BARE, its matched pairs are rendered again with the numbers and the leaf share by third of the height beside the previous round's in `.flow/evidence/fn34/REPORT.md`, the implementer does visual QA before returning (a full oval leafed nearly to the ground, the foliage reading as a beech's layered mass and not as fronds), and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity, the rails, placement only on eligible wood, no node added, determinism across build orders, the cluster count, the instance budget, and a blend walk. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No new skeleton nodes, no epicormic sprouts from the trunk, no seasonal growth of short shoots over time (fn-11 and fn-21 own development). [paraphrase]
- No leaf-mass lighting; appearance work owns it. [paraphrase]

## Resolved via Codebase

- Foliage placement over twig wood: `crates/telperion-core/src/foliage/placement.rs` (seeds from every child of node zero at about `:200-209`; root-radius scaling at `:160`, `:246`); `canopy.shoot_radius` clothes shoots under a share of the trunk's radius.
- Twig anatomy: `crates/telperion-core/src/twigs.rs` (twig length 0.25 m default, internode 0.02 m).
- The beech's table and its round-6c numbers: `crates/telperion-core/src/presets/species.rs`; `.flow/evidence/fn34/REPORT.md`, round 6 (fn-45).
