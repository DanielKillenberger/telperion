# The field reads the plan, not placed leaves

## Goal

A voxel consumer (the killenberger.com voxel tree) asks the core where a tree's wood and foliage are, and that answer cost more than the full mesh: a field request placed and culled every leaf, then built a box tree over them (master `3fdfb440`, 2026-09-21, field-only via Wasm: oak 1.0 s / 96 MB, birch 3.5 s / 32 MB, spruce 8.9 s / 668 MB, against 124, 163 and 58 ms for the solved tree alone). The information exists before any leaf is placed: the solved tree's wood segments and fn-91's prepared station segments. This spec moves the field onto those inputs so the strategy's "sample trees without paying to construct a surface mesh" holds for the expensive stage too.

Owner quotes:
> "We can just use the generated field and make a voxel tree no? one that fits the black white aesthetic. But for that we'd need the generated field to be lightweight enough to be included in killenberger.com We shouldn't drag in the full rendering thing."
> "no we don't want voxels output in telperion-core. I thought we'd have a step that outputs an intermediary step that could inform voxels that happens before generating the full mesh. A field informing where branches/twigs and foliage are before it's actually placed."
> "How can the field be slower than the full mesh build?"
> "but clearly a minecraft style/voxel consumer or whatever it is should be able to consume information from before the mesh gets built to build its own mesh otherwise it's completely pointless and overly complex."
> "this pipeline restructure's performance must be faster or at worst equal to current performance."
> On dropping clumps being consumer styling: "i agree with that last bit."

## Decisions

- The field is built from the solved tree and a foliage descriptor layer (leaf plan): one descriptor per leaf-bearing wood segment with endpoints, pre-cull station count, owning limb system and the family's leaf reach. It never calls placement or the cull. The renderer's station preparation is built on the same descriptors (one definition of leaf-bearing wood).
- Wood occupancy unchanged (tapered sphere sweeps). Foliage is a capsule sweep with a conservative reach (larger radius times seating factor plus the element's farthest vertex at the largest scale), tested with the cell circumsphere as wood is.
- A cell's leaf-count estimate is station count times the share of descriptor length inside the cell (pre-cull, stated); limb id from `foliage/clumping.rs` `systems`; two systems in a cell -> the larger estimate, tie -> lower id.
- Families with no plan form (short shoots, limb clumping, no twig layer, numeric-range fallback; among presets only the beech) keep the placed-leaf path and report it.
- Rejected: a `voxels` output in the core (owner); deleting the field in favour of the structure export (every consumer would reimplement the segment test and guess the reach). Dropping clumps is consumer styling (owner).
- Readiness (host, 2026-09-22): the circumsphere sweep covers 1.40 / 1.64 / 1.16 times the base field's foliage cells at 0.25 m (oak, birch, spruce), so R5's factor of two holds. The spruce's low crown against a bare 15 m leader is a preset matter, not a field defect.
- R9-R11 added by the owner on 2026-09-22 after judging the first field sheet worse for oak and birch (wood specks, crowns breaking into islands at low keep) and better for the spruce.

## Acceptance Criteria

- **R1:** For a plan-supported family a field-only build runs no placement, cull or contact surface; stages reported per preset — met (every preset but the beech).
- **R2:** Field-only build plus 64-cell batch query faster than base and than the same commit's surface-plus-foliage build; peak memory lower — met.
- **R3:** Wood answers unchanged: field tests pass unedited, wood bits byte-identical against base — met on eight single trees (the 8-tree forest no longer exists).
- **R4:** Every vertex of every retained leaf lies in a reported foliage cell at 0.1/0.25/1 m — met (`tests/field_plan.rs`).
- **R5:** Foliage cell count at 0.25 m at most twice the base's — met (1.39 / 1.63 / 1.16).
- **R6:** Estimates sum within 1% of the plan total; two-limb fixture reports distinct ids — met.
- **R7:** Descriptor layer builds with surface and placement compiled out — met (`geometry` default feature, `core-plan` CI job; also gates the specimen's aged reads).
- **R8:** The owner accepts a 64-cell voxel sheet of oak, birch and spruce from the new field as no worse than the base structure-export sheet — met (owner, 2026-09-22).
- **R9:** Per-cell largest wood radius in the batch answer (`Occupancy.wood_radius`, binding slot 25, `FieldQuery.woodRadius`) — met.
- **R10:** Selectable limb order (`"field": {"limbOrder": n}`, default `clump_system_order`, default answers unchanged) — met.
- **R11:** The voxel prototype culls wood under a fifth of a cell and clumps by limb id; new sheet produced — met (order 3).

## Result

Field-only median of five with a 64^3 batch query, base -> candidate (seeds 1 / 7): oak 940 -> 288 / 1,071 -> 330 ms, birch 2,362 -> 345 / 2,598 -> 579 ms, spruce 9,002 -> 233 / 8,646 -> 223 ms. Peak linear memory lower on every tree (spruce 853 -> 75 MB). Field bytes ratio 0.33-0.34 oak, 0.52-0.53 birch, 0.044-0.045 spruce. The batch query is about twice as slow per cell (it tallies every reaching sweep). Clump counts at 64 cells: oak base coin 67, field order 1/2/3+ = 21/129/281.

Owner verdict (2026-09-22), on a comparison page and a field explorer: "looks good to me, seems like we have all the info". Owner notes: coin-based clump dropping loses the crown's structure; how foliage is thinned is the consumer's problem; the wood cutoff belongs in metres of radius, about 2 cm for broadleaves and 1 cm for the spruce.

## Open

- A flags-only batch query could restore the placed path's early exit (not built).

## Friction

- R7's feature build reached into the growth path (~40 min, ~20 `cfg` attributes) -> readiness checks should name the modules a direct build pulls in.
- The "8-tree forest" named in R3 no longer exists (~10 min) -> noted; eight single trees used.
- Local `dcg` hook blocked scratch redirects -> local setup, reported only.
