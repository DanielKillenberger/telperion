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
