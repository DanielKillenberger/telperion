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
