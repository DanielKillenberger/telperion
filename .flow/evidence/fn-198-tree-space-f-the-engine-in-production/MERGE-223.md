# fn-223 merged into F4 (worker, 2026-10-08)

F4's head `4ceaeb00` (fn-230 and fn-226 merged) took fn-223 (`ce17b621`, base `571af291`). Twelve files conflicted. None came from fn-226's re-lay deletion or fn-230's blends: eleven came from fn-229 (florets on the device), which F4 holds, and one from fn-230's re-pin. Both sides appended words to the same three GPU records and the same geometry table. No conflict needed a design judgment.

## Record layouts: both sides' words, fn-229's first

| record | base | F4 (fn-229) | fn-223 | merged |
|---|---|---|---|---|
| geometry table PA block `GG` | 37 | +1: 37 flowering | +2: 37 leaves, 38 spread | 40: 37 flowering, 38 `cluster_leaves`, 39 `cluster_spread` |
| drawn node `CV` (`dress`) | 26 | +3: 26-27 phytomer key, 28 flowering | +1: 26 cluster spread | 30: 26-28 as F4, 29 cluster spread |
| swept node `NW` (`sweep`, `stations`) | 27 | +3: 27-28 key, 29 flowering | +1: 27 cluster spread | 31: 27-29 as F4, 30 cluster spread |

- **`laid.rs`, `geometry.wgsl`, `dress.rs`/`.wgsl`, `fold.wgsl`, `sweep.rs`/`.wgsl`, `stations.wgsl`:** the constants are as tabled. fn-223's reads and writes moved with them: `laid::table` writes the cluster rows at 38 and 39, `dress.wgsl` reads them from there and puts the spread at drawn word 29, `sweep.wgsl`'s gather copies drawn words 26 to 29 into node words 27 to 30, and `stations.wgsl`'s cluster segment reads node word 30. `blooms.wgsl` reads 27 to 29, which are unchanged.
- **`stations.rs` `source()`:** F4's is kept (contract, bloom, stations, bloom kernels). The contract already carries `lineage.wgsl`, so fn-223's extra `KEYS` prefix is dropped: two copies would redefine every key. `KEY_CLUSTER` reaches the shader through the contract.
- **`stations.wgsl` header:** both paragraphs are kept. **A merge fix:** `tip()` now also requires `cfg.leaves != 0`. F4 lets the stations run for a tree that flowers without leaves. fn-223's tip runs bypass `bear`, so without the check a leafless flowering tree would emit cluster leaves that the CPU, which prepares none, never draws. Before F4, the early return covered this case.
- **`telperion-space/src/fine/geometry.rs`:** `PaGeometry` carries `flowering` and both cluster fields.

## The CPU side

- **`telperion-core/src/pipeline/foliage.rs`:** F4's bloom exports are kept (`now`, `place_in`, `reach`). fn-223's deletion of the `short_shoots` module is kept, along with its exports and `MAX_SHORT_SHOOT_LEAVES`/`SHORT_SHOOT_SPACING`. F4 had only reordered those lines, and no F4 code uses them.
- **Lineage registry:** `keys.rs` (`CLUSTER`, `ALL` of 24) and `lineage.wgsl` auto-merged. The mirror and distinctness tests pass in the render gate.

## Re-pin: `catalogue_identity`, family and walk digests

- **Family:** `7_479_809_179_514_851_386` (F4) and `16_505_628_211_544_797_342` (fn-223) became `15_341_796_280_686_330_418`.
- **Walk:** `4_247_907_124_043_262_165` (F4) and `11_066_511_611_635_173_183` (fn-223) became `10_261_846_770_402_034_553`.
- **Override and refusal:** both auto-merged to fn-223's values, which F4 had not moved, and both pass.

The two changes touch disjoint rows: fn-230 changes the blend's sizing and the ordinary preset, and fn-223 retires the `shortShoot*` rows and adds neutral `cluster_*` rows. The text-level undo that fn-223 did was not repeated here. The check on trees is below. The test header records the re-pin.

## Neutral rows give F4's trees (`space_hash`)

"Before" is `raw/merge-f230-f226/hash-after.txt`, taken at `efe8444b`. `4ceaeb00` changes no code after it. "After" is `raw/merge-223/hash-after.txt`. All 16 match exactly, node counts included:

```
beech 40 1 102846 af1e34d92f5de418    spruce 40 1 1147168 b32c14b9a41e5405
beech 40 7 181014 42bfbabd8debdc39    spruce 40 7 1107132 4d7fe97d251a607e
beech 80 1 1540442 582d621737d6ba3c   spruce 80 1 4635552 35e51826f380cf43
beech 80 7 2609576 c0d3301359e16721   spruce 80 7 4608273 38eb1907944fb770
oak 40 1 167123 9be2dec8e67d5cce      palm 40 1 993 da01c8cbf52dd468
oak 40 7 291579 4d2787641bc450dc      palm 40 7 993 de27cb076da06ca5
oak 80 1 947012 cee692955a1f5ddd      palm 80 1 1473 7c90dd11f427e83a
oak 80 7 2222419 3178df24b438dc29     palm 80 7 1473 84ca7b46a037c49d
```

## Clusters and florets on the merged layout (`raw/merge-223/seam.log`)

- **`clusters_match_across_the_seam`:** 128 clusters stand above the cut and 195 in fine roots. Tips are 9.90e-7 m apart and headings 5.20e-4 rad apart, as in fn-223's RESULT.md.
- **`florets_on_the_device_match_the_whole_tree`:** passes on the beech and the spruce.

## Gate and npm (raw output in `raw/merge-223/`)

- **Gate, the render half under the GPU lock:** 196 passed, 0 failed, 31 ignored (`gate-render.log`). That is F4's 195 plus fn-223's seam test.
- **Gate, the rest of the workspace:** 1,046 passed, 0 failed, 46 ignored (`gate-rest.log`).
- **`npm test`:** 13 files and 143 tests pass (`npm.log`). Its pre-test Wasm build left the generated browser files as merged.
