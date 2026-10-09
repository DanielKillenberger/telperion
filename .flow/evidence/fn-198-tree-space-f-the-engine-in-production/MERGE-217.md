# fn-217 merged into F4 (host merge, 2026-10-08)

F4's head `61178732` took fn-217 (`ffa8c33e`, 31 commits on base `1f88d724`). fn-217 brings the `/supernatural` group: its rows, its scale, the leaf reflectances, the rim, and the bloom rows `petalCurl`, the eye (`bloomEye*`) and the stamens. F4 keeps fn-223's clusters, fn-229's florets on the GPU, fn-230's blends and fn-226.

## Conflicts

The two sides changed disjoint rows. Each conflict was a count or a list that both sides had edited, and each was resolved as the union of the two.

- **`crates/telperion-core/src/catalogue/tests.rs`:** the row count and the wire ranks were 278 on the base. F4 retired five `/canopy/shortShoot*` rows (273), and fn-217 added 60 `/supernatural` and bloom rows (338). The merge has 333 rows, ranked `0..333`.
- **`crates/telperion-core/src/suite/sweep.rs`:** `HELD` was 99. F4 dropped `/canopy/shortShootSpacing` (98), and fn-217 added its 60 rows with its comment (159). The merge has 158: F4's list and comment, plus fn-217's rows and comment. The retired row stays out.
- **`crates/telperion-core/tests/catalogue_identity.rs`:** the header keeps both histories, fn-223's paragraph followed by fn-217's. A new paragraph records this merge's re-pin. All four digests are re-pinned (below).
- **`docs/generation-limits-inventory.json`:** F4's five blend sites (`blend.rs`, `blend/size.rs`) and fn-217's `supernatural.rs` flowering-age walk are both kept. F4's deletions (the short-shoot and `catalogue/walk.rs` entries) auto-merged and stay deleted. The file has 267 entries.
- **Wire ranks (no textual conflict, found by the gate):** fn-217 numbered its 60 rows `wire: 278..337` after the base's 278. F4 had retired five rows and compacted the ranks to `0..273`, so the merged wire skipped 273 to 277, and `every_row_has_its_own_rank_on_the_wire` failed. fn-217's ranks were shifted down by five to `273..332` in `supernatural.rs`, `supernatural/overrides.rs`, `supernatural/flowering.rs`, `material/params.rs` (the eye) and `foliage/bloom_params.rs` (curl and stamens). No rank is duplicated, and the highest is 332. The shift moved no digest and no generated file.

Every other file auto-merged, including `pipeline/space.rs`, `foliage/bloom_params.rs`, `fine/dress.rs` and `dress.wgsl`, and the generated browser files.

## Neutral rows give F4's trees (`space_hash`)

"Before" is `raw/merge-217/hash-before.txt`, built at `61178732` and copied out of `target/`. "After" is `raw/merge-217/hash-after.txt`, built on the merged tree. All 16 match exactly, node counts included, and equal MERGE-223.md's:

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

## Re-pins (`catalogue_identity`)

These were re-pinned only after the hashes above matched. Each digest changes because the family's text and the wire now carry both sides' rows.

- **Family:** `15_341_796_280_686_330_418` (F4) became `6_657_907_224_534_109_834`.
- **Walk:** `10_261_846_770_402_034_553` became `2_827_795_343_174_730_974`.
- **Override:** `7_934_268_765_783_179_966` became `17_412_911_408_925_295_970`.
- **Refusal:** `1_536_851_553_539_686_348` became `159_807_827_090_449_713`. It pairs rows by index, so it re-pairs whenever the wire changes.

## Generated files

- **`parameters.generated.ts` and `docs/parameters.md`:** these were rewritten with `TELPERION_WRITE_REFERENCE=1 cargo test -p telperion-core --test parameter_reference`. The output was byte-identical to the auto-merged files.
- **`presets.generated.ts`:** `npm test`'s pre-test Wasm build left it as merged.

## Gate and npm (raw output in `raw/merge-217/`)

The first run, on the merge before the rank fix (`*-run1.log`), failed in two places. The render half passed. The rest failed only `every_row_has_its_own_rank_on_the_wire`, which the rank fix above resolved. `npm test` failed one test: `field.test.ts`'s "grows every shipped preset" timed out at 60 s. Load was 58 on 32 threads, and the tests took 153 s against about 75 s in earlier merges. The second run, on the final tree under load 4 to 9, passed it.

- **Gate, render half under the GPU lock:** 205 passed, 0 failed, 35 ignored (`gate-render.log`).
- **Gate, the rest of the workspace:** 1,061 passed, 0 failed, 46 ignored (`gate-rest.log`).
- **`npm test`:** 13 files, 143 tests pass, tests 63.8 s (`npm.log`).
- **`space_hash`** was re-run on the final tree after the rank fix: the 16 still match.
