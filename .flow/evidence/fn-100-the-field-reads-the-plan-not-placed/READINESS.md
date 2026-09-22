# fn-100 readiness measurements (2026-09-22, master 3fdfb440, native ci profile, seed 1)

Two parked unknowns were settled before the spec was marked ready. Sources: `reach-probe.rs` (copy into `crates/telperion-core/examples/`, `cargo run --profile ci --example reach-probe -- <preset> <cell>`) and `bounds-probe.mjs` (`node`, reads `src/browser/telperion.wasm`).

## R5: over-coverage of a descriptor sweep against the base field

Reach per bearing segment = the segment's larger radius (times the seating factor when `surface_contact > 0`) plus the element's farthest vertex at `size * (1 + size_variation)`. Bearing segments follow `placement::bearing_runs`: twig nodes, plus wood slender enough for `shoot_radius`. `circ` tests the cell's circumsphere as the wood query does; `boxed` tests the sphere at the nearest segment point against the cube.

| preset | cell | base foliage cells | circ | boxed | ratio circ | ratio boxed | base cells not covered (circ / boxed) |
|---|---|---|---|---|---|---|---|
| oregon-white-oak | 0.25 | 152,137 | 212,339 | 190,269 | 1.40 | 1.25 | 1 / 186 |
| oregon-white-oak | 1.0 | 5,197 | 6,112 | 5,440 | 1.18 | 1.05 | 0 / 1 |
| silver-birch | 0.25 | 38,444 | 62,942 | 56,798 | 1.64 | 1.48 | 0 / 8 |
| silver-birch | 1.0 | 1,298 | 1,639 | 1,450 | 1.26 | 1.12 | 0 / 0 |
| norway-spruce | 0.25 | 12,995 | 15,095 | 13,293 | 1.16 | 1.02 | 0 / 57 |
| norway-spruce | 1.0 | 376 | 455 | 382 | 1.21 | 1.02 | 0 / 0 |

The factor of two in R5 holds with the circumsphere test on all three presets. Base cells are leaf AABB overlaps, so a base cell the sweep misses is not by itself a real vertex outside the sweep; R4 is judged against vertices. Blade reach: oak 0.135 m, birch 0.093 m, spruce 0.023 m; the spruce's seating factor is 2.84.

Field-only builds through the Wasm binding on the same commit, reproduced today: oak 1,014 ms, birch 3,517 ms, spruce 8,832 ms; placement and cull dominate.

## Spruce bounds

Nothing stretches the spruce's box. Its solved tree runs 8.0 x 15.0 x 8.2 m; the leader reaches y = 15.00 at node 50 with a 0.3 mm radius, and the twig nodes sit low: by height decile the node counts are 2412, 35993, 26951, 13785, 7761, 3120, 1038, 245, 14, 12. The prototype's half-filled grid is the tree. The field box also reaches y = -trunk radius through the root sphere (oak 0.43 m, birch 0.25 m, spruce 0.22 m below ground).

Oak: 25.6 x 22.9 x 25.3 m, 138,510 nodes. Birch: 13.9 x 16.5 x 12.2 m, 73,337 nodes.
