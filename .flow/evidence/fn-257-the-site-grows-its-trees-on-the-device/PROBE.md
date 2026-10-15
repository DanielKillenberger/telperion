# fn-257 early probe: a field grown on the device, per tree, warm

2026-10-15, branch `fn-257-device-field` from F4 `a07bf2732`, RTX 3080, native (`crates/telperion-render/src/generation/resident/field_probe_tests.rs`, `cargo test --profile ci -p telperion-render field_probe -- --ignored --nocapture`). Each tree is the second of two device draws in the process. The CPU columns build the same family's leaf plan, field and a whole 36³ query over its bounds natively, the CPU tree standing in for the read-back one (the same node count); in a browser they would run in Wasm, slower.

## Verdict: STOP (NEEDS_HUMAN)

The spec's route (R1/R1a: grow on the device, read the drawn records back once, answer the unchanged query on the CPU) misses 0.1.4 on every site species, and two of its costs are structural:

1. **The device's growth alone is over 0.1.4's whole call** for the birch (cycle 377 ms against 0.1.4's 346-357 ms end to end), the spruce (453 ms against 240-244) and the ordinary tree (289 ms of cycle plus 101 ms of tables). Only the oak's growth (181 ms) is under its bar (278-319). No change to the field reaches the birch or the spruce; that is growth speed.
2. **Reading the drawn tree back is the size of the tree:** 13-115 MB for the oak, birch, beech and ordinary, 217-226 MB for the spruce, 275-358 MB for Telperion and Laurelin; 118-213 ms for the oak, 566-745 ms for the spruce. The field then rebuilds its plan, its index and its query over all of it (oak +180-340 ms, spruce +1.7-2.2 s). The site's cells are 0.3-0.5 m; nearly all of that data is finer than one cell.

## The table (ms unless stated)

| Preset | Seed | Age | Device drawn | Read | Read MB | Nodes | CPU grow (reference) | Plan | Field | Query 36³ | Device route total | 0.1.4 end to end |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| ordinary | 1 | 40 | 442 | 35 | 18.1 | 86,360 | 298 | 3 | 13 | 37 | 530 | not served |
| ordinary | 1234 | 40 | 461 | 38 | 23.9 | 117,099 | 336 | 5 | 21 | 44 | 569 | |
| ordinary | 5678 | 40 | 437 | 22 | 18.9 | 90,114 | 290 | 3 | 16 | 34 | 512 | |
| oak | 1 | 40 | 232 | 118 | 63.5 | 275,902 | 845 | 30 | 64 | 87 | 531 | |
| oak | 1234 | 40 | 263 | 213 | 115.1 | 397,023 | 1,569 | 75 | 113 | 150 | 814 | 278 |
| oak | 5678 | 40 | 249 | 133 | 86.7 | 321,105 | 1,066 | 41 | 71 | 83 | 577 | 319 |
| spruce | 1 | 40 | 399 | 652 | 222.0 | 994,901 | 3,486 | 322 | 502 | 819 | 2,694 | |
| spruce | 1234 | 40 | 407 | 589 | 226.2 | 1,009,245 | 3,889 | 357 | 552 | 1,119 | 3,024 | 240 |
| spruce | 5678 | 40 | 415 | 566 | 217.1 | 987,349 | 3,572 | 330 | 522 | 981 | 2,814 | 244 |
| beech | 1 | 40 | 168 | 22 | 23.5 | 114,699 | 158 | 6 | 16 | 48 | 260 | not served |
| beech | 1234 | 40 | 150 | 14 | 13.0 | 59,575 | 104 | 3 | 8 | 47 | 222 | |
| beech | 5678 | 40 | 155 | 15 | 18.8 | 85,098 | 157 | 5 | 12 | 25 | 212 | |
| birch | 1 | 40 | 360 | 50 | 47.2 | 242,452 | 547 | 19 | 55 | 178 | 662 | |
| birch | 1234 | 40 | 371 | 94 | 60.4 | 269,350 | 790 | 25 | 58 | 237 | 785 | 346 |
| birch | 5678 | 40 | 398 | 42 | 41.0 | 168,500 | 486 | 12 | 30 | 155 | 637 | 357 |
| palm | 1 | 40 | 75 | 0 | 0.1 | 480 | 2 | 1 | 1 | 13 | 90 | not served |
| Telperion | 1 | 82 | 952 | 1,124 | 358.2 | 1,985,622 | 7,499 | 221 | 531 | 255 | 3,083 | not served |
| Laurelin | 1 | 80 | 828 | 930 | 336.4 | 1,859,571 | 7,097 | 211 | 513 | 233 | 2,715 | not served |

0.1.4's end to end is fn-245 PROFILE.md's (grow, grid, query and voxelize in Wasm, cut 1,024, n = 36); 0.1.4 serves only the oak, birch and spruce. "Device route total" is drawn + read + plan + field + query, natively; the voxelizer (3-13 ms) and Wasm's overhead on the CPU stages would add to it in a browser.

Where the device's draw goes (`CYCLE_TRACE`, seed 1): oak tables 9, grown 190, settled 211, drawn 227 ms; birch 11 / 388 / 413 / 425; spruce 20 / 473 / 514 / 537; ordinary 101 / 390 / 415 / 428. The cycle is most of it; settle and dress are about 35-65 ms.

## What would close the gap (each is a design choice, not this spec's)

- **The field reduced on the device.** A kernel that writes the site's cells (wood flag, thickest radius, leaf estimate, owning limb) from the drawn records on the device, read back as about 36³ × 16 B ≈ 0.75 MB. This removes read, plan, field and query (oak about 300-550 ms, spruce about 2.3-2.6 s). It answers a grid, not R1a's arbitrary cubes: R1a's "no new grid" would have to go.
- **Growth speed.** With the field on the device, the oak would fit its bar (about 230-260 ms drawn plus a reduction). The birch, the spruce and the ordinary tree would not: their cycle alone is over 0.1.4's whole call. That needs the generator faster at these trees (fn-245 and fn-285's ground), or a coarser grown tree for the site (the cut route, which fn-257's spike already measured as too lossy or as slow as the whole tree), or a different bar for these species.
- **The read-back of a whole tree in a browser** (217-358 MB for the spruce, Telperion and Laurelin) is a memory risk on its own, apart from time.
