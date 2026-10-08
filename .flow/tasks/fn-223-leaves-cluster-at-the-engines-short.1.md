---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-223-leaves-cluster-at-the-engines-short.1 Implement leaves cluster at the engine's short shoots

## Description
### Carry plan (R1, stated before code; checked against the code at 891c3bd0)

**Rows (R2).** `PaState` gains `cluster_leaves` (whole leaves, 0 to 8) and `cluster_spread` (radians, 0 to pi/2), both 0 in every species and test species. They are listed in `species/scale.rs` (`settings`, `SCALES`, linear), so presets read them as `/space/ages/N/clusterLeaves` and `/clusterSpread` with no new parsing, and `species/validate.rs` refuses each by name outside its bounds or, for leaves, when not whole. The device reads them from `PaGeometry`: the geometry table's per-PA block grows from 37 to 39 words (`laid.rs`, `dress.wgsl`, `geometry.wgsl`).

**CPU carry, from `pipeline/space.rs::convert`.** Each axis has one PA (`axis.pa`), and `PaRow` gains that PA's cluster. In an axis whose PA has `cluster_leaves > 0`, every node the axis draws is marked clustered (`Tree::clustered`, per node, empty when no PA clusters). The axis's last drawn node, the `parent` its phytomer loop ends on once `made`, is its tip: `Tree::clusters` holds `{node, leaves, spread}`. An axis that draws no node of its own (all its phytomers merged, thinner than `FINEST`, or shed) gets no cluster. A continuation or relay is its own axis with its own PA, so one axis owns one cluster at its own tip. No cluster is placed at a run end or at a seam.

**Device carry, from `fine/dress.wgsl::draw` and `fine/sweep.wgsl::gather`.** The axis record carries its PA (`ax(i, PA)`), and `draw` reads that PA's two cluster words from the geometry table. Every MADE record of a clustering axis is flagged `CLUSTERED` (32). The axis's last made record, the `parent` at the end of the loop, is flagged `TIP` (64), with its leaves in bits 8 to 11 of the flags word. A new record word 26 carries the spread (CV 26 to 27). `gather` already copies the flags to swept word 9; the spread goes to swept word 27 (NW 27 to 28).

**Seating (both executors).** A clustered node never bears stations: `plan::bearing_of` and `stations.wgsl::bear` both leave it out. The short shoot's evenly spaced leaves are therefore gone, and a long shoot's run continues past the short shoots it bears. Each tip becomes a run of its own, with one station segment of `leaves` stations marked as a cluster (CPU `StationSegment::cluster`, GPU `Segment` with a negative span). It goes into `plan::runs` totals, the plan's descriptors, `placement::leaves_on` and `place_on`, so the instance budget counts cluster leaves and refuses an over-budget tree by name.

Cluster leaf j sits at the tip node's position. Its radial is the level bearing of the tip's laid heading (`Tree::frames` on the CPU, swept words 15 to 17 on the device; +X where the heading is vertical), turned about +Y by spread x (2j/(n-1) - 1), and 0 for a lone leaf. Its axis is `station::axis` with the heading as tangent. The CPU's `rosette::fan` and the GPU's `pack.wgsl::station` mirror each other.

**Identity (R4).** The registry gains `CLUSTER` (under a node, `lineage_keys`, mirrored in `lineage.wgsl`). A cluster's stream is `leaf_stream(seed, cluster_key(tip key))`, where `cluster_key(k) = mix32(k ^ CLUSTER)` and the tip key is `Tree::key(tip)` (device swept word 25). Leaf j draws at the stream's ordinals for station j, so both executors draw by (tip lineage, index).

**R3.** The canopy's `shortShoot*` rows hang clusters on limb wood by their own spacing, and only where `shortShootSpacing > 0`. No shipped preset sets the spacing. The beech states the other four rows dormant, so nothing they describe is drawn today. R1 covers their purpose for engine species, so the rows, `short_shoots.rs`, their suite tests and their dial, harness and generated entries are deleted. The beech's dormant rows go without R2 values, because they drew nothing, and its stills stay byte-identical.
## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
