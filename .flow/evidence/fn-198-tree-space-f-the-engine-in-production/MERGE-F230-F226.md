# fn-230 and fn-226 merged into F4 (worker, 2026-10-08)

F4's head `1f88d724` took fn-230 (`b369a9dc`, merge `72f0c48d`), then fn-226 (`b3fd236d`, merge `efe8444b`).

- **fn-230** brings `blend::centroid`, the blend's size factor, the ordinary tree re-derived and the space walks at 40 and 80.
- **fn-226** brings the periodic re-lay's deletion, the named `ROUGH_LAYOUT` walk exception (`tests/walk/degree.rs`, bound 33 for the two `apical_control` light walks) and its evidence. Its outline-light code was added and deleted on the branch, so the net code change is the deletion.

## Conflicts

- **fn-230, one file: `docs/generation-limits-inventory.json`.** fn-230's five blend sites (`blend.rs` weights, `blend/size.rs` solve) are kept. The flowering-refusal entry (`executor.rs`) that fn-230's base still carried stays deleted, as fn-229 deleted the site on F4. The result is F4's file plus fn-230's 36 added lines; `generation_limits_are_classified` passes.
- **fn-226: none.** `tests/walk/mod.rs` auto-merged. The workspace type-checks with every target.

## Generated files and re-pins

- **No re-pin.** F4 changed no preset row since fn-230's base (`571af291`), so `catalogue_identity`'s four digests are fn-230's values unchanged and pass. fn-226 touches no catalogue digest.
- **`presets.generated.ts`:** `npm test`'s pre-test Wasm build left it as merged. No parameter row changed, so `parameters.md` and `parameters.generated.ts` were not regenerated.
- **The centroid example** (`cargo run --profile ci -p telperion-core --example centroid`) rewrote `presets/ordinary.values` byte-identically (sha256 `d678f60b…` before and after).

## The 16 clothed hashes (`space_hash`; before at `1f88d724`, after at `efe8444b`)

```
beech 40 1   102846 af1e34d92f5de418 -> af1e34d92f5de418
beech 40 7   181014 42bfbabd8debdc39 -> 42bfbabd8debdc39
beech 80 1  1540442 582d621737d6ba3c -> 582d621737d6ba3c
beech 80 7  2609576 c0d3301359e16721 -> c0d3301359e16721
spruce 40 1 1147168 b32c14b9a41e5405 -> b32c14b9a41e5405
spruce 40 7 1107132 4d7fe97d251a607e -> 4d7fe97d251a607e
spruce 80 1 4635552 35e51826f380cf43 -> 35e51826f380cf43
spruce 80 7 4608273 38eb1907944fb770 -> 38eb1907944fb770
oak 40 1     167162 611cb08d00036381 -> 167123 9be2dec8e67d5cce
oak 40 7     296157 616d85aadf3289bd -> 291579 4d2787641bc450dc
oak 80 1     950009 7236dea7302e4782 -> 947012 cee692955a1f5ddd
oak 80 7    2332830 1992d9445d185fed -> 2222419 3178df24b438dc29
palm 40 1       993 da01c8cbf52dd468 -> da01c8cbf52dd468
palm 40 7       993 de27cb076da06ca5 -> de27cb076da06ca5
palm 80 1      1473 7c90dd11f427e83a -> 7c90dd11f427e83a
palm 80 7      1473 84ca7b46a037c49d -> 84ca7b46a037c49d
```

- **Twelve hold; the four oaks move.** The oak is the one shipped preset grown under site light (`oregon-white-oak.values`: `/space/light/extinction = 0.5`), so it grows a rough layout and the re-lay ran for it. Its deletion moves the oak's light, and with it the node counts (−0.02 % to −4.7 %).
- **fn-230 cannot move them:** it changes only the blend and the ordinary preset, which `space_hash` does not grow.
- **This corrects fn-226's R2 "Shipped trees" paragraph,** which inferred (without running `space_hash`) that no shipped preset sets light. On F4 the oak does. The move is the change the host accepted in R5/R6 (today's model, no re-lay), which R2 measured on the lit oak at 40 and 80; no stills were taken here.

## Walks, gate and npm (raw output in `raw/merge-f230-f226/`)

- **Filtered walk** (`WALK=apical_control`, `light_changes_the_tree_by_degree`): passes (`walks-apical.log`).
- **Full walks once** (`walks.log`): `telperion-space` `walks` 11 passed, 1 ignored (the slow suite); `telperion-core` `space_walks` 6 of 6, `catalogue_identity` 4 of 4, `space_presets` 5 of 5.
- **Gate, render half under the GPU lock:** 195 passed, 0 failed, 31 ignored (`gate-render.log`).
- **Gate, the rest of the workspace:** 1,048 passed, 0 failed, 46 ignored (`gate-rest.log`).
- **`npm test`:** 13 files, 143 tests pass (`npm.log`).
