# fn-198 task 2: grown-tree hashes, the examples' handed-in path (baseline)

Recorded at a9b11055 (before any task-2 change), `cargo build --profile ci`, by `crates/telperion-render/examples/space_hash.rs`: `telperion_space::grow` (budget 20M; the oak under light 0.5, 0.5 as BASELINE.md, the others neutral), `space/tree.rs::convert` with each species' trunk ages, then `executor::expand` on the preset overlaid by the example's ROWS. `grown` hashes the converted tree, `clothed` the expansion's tree (`space/hash.rs`: every node field by its bits). Load 18 to 20 on 32 threads; times indicative.

```
beech 40 1 nodes 102256 grown bb772650e550d010 clothed bb772650e550d010 ms 193
beech 40 7 nodes 176628 grown 604f6958c75d8d43 clothed 604f6958c75d8d43 ms 341
beech 80 1 nodes 1525191 grown 0bd447b27abf9b94 clothed 0bd447b27abf9b94 ms 2793
beech 80 7 nodes 2532841 grown fac452f044b6525c clothed fac452f044b6525c ms 3775
spruce 40 1 nodes 1147615 grown 4c3eabec018c280d clothed 4c3eabec018c280d ms 2166
spruce 40 7 nodes 1107566 grown b9bc15960a8ef98c clothed b9bc15960a8ef98c ms 1562
spruce 80 1 nodes 4637482 grown 8a1b693f1fbf25b6 clothed 8a1b693f1fbf25b6 ms 7438
spruce 80 7 nodes 4610322 grown ae19e1efae7a4b0c clothed ae19e1efae7a4b0c ms 7420
oak 40 1 nodes 157595 grown 48d2be5036279f17 clothed 48d2be5036279f17 ms 882
oak 40 7 nodes 281160 grown 41c7b8949ebbd487 clothed 41c7b8949ebbd487 ms 923
oak 80 1 nodes 868966 grown f62f128ffdb32c47 clothed f62f128ffdb32c47 ms 6113
oak 80 7 nodes 2203466 grown c209da3a3e149284 clothed c209da3a3e149284 ms 14173
palm 40 1 nodes 993 grown 748cb5c0c374292e clothed d7e5916bfdd8537a ms 3
palm 40 7 nodes 993 grown fc6c0f17deb01f35 clothed e7f7c3b375270962 ms 3
palm 80 1 nodes 1473 grown 2ceb1bc193488db4 clothed 398ec484fa22ce8b ms 4
palm 80 7 nodes 1473 grown 2002fb292fb839c0 clothed 8401a8d5b6fcbd40 ms 5
```

## Through the pipeline, after the handed-in path closed

The same example, rewritten to the pipeline alone: each engine preset at the age and seed, `executor::grow(&family)` then `expansion()`, the expansion's tree hashed. Every `clothed` hash equals the handed-in path's above (16 of 16; compared by script). For the beech, spruce and oak the handed-in path's `grown` and `clothed` hashes are equal, so their converted trees equal before clothing too; the palm's clothing hangs its leaf bases from the preset's unchanged canopy rows. Load 17 to 18.

```
beech 40 1 nodes 102256 clothed bb772650e550d010 ms 115
beech 40 7 nodes 176628 clothed 604f6958c75d8d43 ms 162
beech 80 1 nodes 1525191 clothed 0bd447b27abf9b94 ms 1333
beech 80 7 nodes 2532841 clothed fac452f044b6525c ms 2203
spruce 40 1 nodes 1147615 clothed 4c3eabec018c280d ms 1370
spruce 40 7 nodes 1107566 clothed b9bc15960a8ef98c ms 1336
spruce 80 1 nodes 4637482 clothed 8a1b693f1fbf25b6 ms 7636
spruce 80 7 nodes 4610322 clothed ae19e1efae7a4b0c ms 4427
oak 40 1 nodes 157595 clothed 48d2be5036279f17 ms 403
oak 40 7 nodes 281160 clothed 41c7b8949ebbd487 ms 567
oak 80 1 nodes 868966 clothed f62f128ffdb32c47 ms 4035
oak 80 7 nodes 2203466 clothed c209da3a3e149284 ms 9096
palm 40 1 nodes 993 clothed d7e5916bfdd8537a ms 2
palm 40 7 nodes 993 clothed e7f7c3b375270962 ms 2
palm 80 1 nodes 1473 clothed 398ec484fa22ce8b ms 3
palm 80 7 nodes 1473 clothed 8401a8d5b6fcbd40 ms 3
```

## After the canonical axis order (host, 2026-10-07)

The pipeline now puts every grown structure into one canonical axis order before conversion (parents first, siblings by lineage, `pipeline/space.rs::canonical`, the engine's own order: `the_order_is_the_engines_canonical_order`), so the conversion never depends on how the tree was assembled (`a_tree_assembled_in_another_order_converts_to_the_same_tree`, which also shows the unsorted order reaches the tree). The proof above was taken before the sort. Node counts are unchanged; the trees' hashes move once, the palm's (one axis) not at all:

```
beech 40 1 nodes 102256 clothed 945218a946ce4064 ms 86
beech 40 7 nodes 176628 clothed eb0feb34ee35e272 ms 126
beech 80 1 nodes 1525191 clothed 1fce76e4df125173 ms 1102
beech 80 7 nodes 2532841 clothed 56da1c879b1bfde7 ms 1738
spruce 40 1 nodes 1147615 clothed 3deadd87ba252e29 ms 1092
spruce 40 7 nodes 1107566 clothed af0c3b84aaae7efc ms 1019
spruce 80 1 nodes 4637482 clothed 8ae9105fe2bfe7ae ms 4381
spruce 80 7 nodes 4610322 clothed 945721486c0682b7 ms 4297
oak 40 1 nodes 157595 clothed 8cb290d46d27e1fd ms 374
oak 40 7 nodes 281160 clothed 48ad8b55d0773760 ms 564
oak 80 1 nodes 868966 clothed 973b2a55cc12a29e ms 3729
oak 80 7 nodes 2203466 clothed 762ecf06c6cc21bc ms 8233
palm 40 1 nodes 993 clothed d7e5916bfdd8537a ms 2
palm 40 7 nodes 993 clothed e7f7c3b375270962 ms 2
palm 80 1 nodes 1473 clothed 398ec484fa22ce8b ms 3
palm 80 7 nodes 1473 clothed 8401a8d5b6fcbd40 ms 3
```

**Before/after stills**, age 40 seed 1, whole view, 960x720 (`raw/sort/{before,after}/`, gitignored; the before built with the sort bypassed): the oak differs on 5,611 pixels (0.81 %), the beech on 5,820 (0.84 %) (`magick compare -metric AE`), within fn-211's 0.5 to 1.3 %. Viewed (the oak pair): the same tree, crown, limbs and shadow; the difference is scattered leaf placement inside the crown.

## After a node's wander is drawn apart from its places (host, 2026-10-07)

A node's wander was keyed `node.child(1)`, which is also its slot-1 bud place's key, so its wander and that place's draw were one stream. `lineage_keys::WANDER` moves to the reserved `u64::MAX - 13`, and the registry now lists the places (`PLACE`, `MAX_BUDS` values under the node key), so `keys_under_one_parent_are_distinct` fails on the old value ("PLACE and WANDER meet"). Every wandering tree changes. Base is the fn-199 merge (49773335), whose 16 hashes equal fn-199's own; `space_hash`, `node count / clothed hash` before, then after:

```
beech 40 1 102840 9ce782a7682af851 102841 5811567f7f3670bf
beech 40 7 181015 599b1dbde4424241 181015 11f2c0b5a799ac06
beech 80 1 1540448 6fd6aed2f0a431e1 1540449 e12f497857fd4d8a
beech 80 7 2609681 9e658469509a53c9 2609652 cff3dcd09da8775a
spruce 40 1 1147615 65b3a0577b13b3b8 1147615 3324e54ed90088b7
spruce 40 7 1107566 89fd1c097108b239 1107566 cda21916e9c0abf6
spruce 80 1 4637482 af8f7512c34502b1 4637482 18d1e715d3968286
spruce 80 7 4610322 13ebdaaffdc70f37 4610322 7ab24d5670ac9ed6
oak 40 1 164533 dae6a94a9ae44f02 167177 1369b2a64d812568
oak 40 7 298514 b275fbff908fec91 296176 78cfd0ebb356b793
oak 80 1 970257 69d1139498de1807 950317 04b0a548c5595683
oak 80 7 2311842 1eb9d7984e9e53f8 2333490 ecc67a4291310e96
palm 40 1 993 8c5c37c4741272e2 993 061987de19837971
palm 40 7 993 19a541c6262b7258 993 8d2ce3067c21f86e
palm 80 1 1473 3f8c871219dc6b9b 1473 025e167fec51c301
palm 80 7 1473 158cecedcd5bab6c 1473 333ad5c2de1e903a
```

The spruce and the palm keep their node counts. The beech changes by up to 29 nodes, and the oak by up to 2.1 % (80/1: 970,257 to 950,317), because a different wander moves shoots in the light.

**Hero stills**, age 80 seed 1, 1024x1024 (`headless --family {"age": 80}`, `raw/wander/{before,after}/`, gitignored). Differing pixels (`magick compare -metric AE`): beech 4.1 %, spruce 2.4 %, oak 5.7 %, palm 1.7 %. Viewed side by side: the beech, the spruce and the palm are the same tree, with differences in twig and frond placement. The oak is visibly another specimen of the same species: the same height, spread and open, irregular crown, but its leader leans differently and its main fork is placed differently. No defect is visible in any of the four. The four species' walk tests (`telperion-space` `walks`, 10 tests) pass.
