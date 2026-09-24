# fn-144 design, as landed

Recorded after the fact: the host minted fn-144 at capability assessment (2026-09-24) and dispatched one strong-tier agent that designed and built it in one pass (branch fn-144-retained-leaf-bases-pack-into-a-lattice, 965b0403; PR 110).

- **Interface.** Two canopy rows, `leafBaseWidth` (0 to 2, share of the cell the rosette spiral gives a base; 1 = neighbours touch) and `leafBaseFlatness` (0 to 1, ellipse through the cell's corners to the flat-sided diamond cell). Both 0 draw today's round peg, byte-identical.
- **Route.** The base's section is a table on the tree (`Tree::sections`, never serialized) that the existing sweep reads: no new rings, vertices or triangles; wood build 0.83 to 1.03 ms at 256 bases. A tree with shaped bases takes the CPU sweep (the GPU ring format carries round sections only).
- **Invariants.** One lattice per stem, laid on its mean girth and the centreline smoothed over its girth; a base on thinner wood is raised to meet the bark with its neighbours; packed bases lean at least 15 degrees; a packed base's length is measured out of the bark.
- **Verification.** R1 `leaf_base_lattice.rs` at 32, 96, 256 bases (cover 1.000, gap and crowding below 1e-5 of a cell on a straight trunk; 0.12 and 0.25 on the shipped crooked palm); R2 presets byte-identical; R3 P-TRUNK still; gate 1,060 passed.
- **Unknown.** The reviewer's `trunk-leaf-base-diamond-pattern` cell on a tuned palm, which tuning revision 2 records.
