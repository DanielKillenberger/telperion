# The candidate species merged into F4 (host merge, 2026-10-09)

F4 at `7dc60f82` took three species branches so that the release branch holds what the owner will test. Their tasks stay open for the owner's verdicts. `scratch-fn-235-birch` was not merged. Raw output is in `raw/merge-species/` (ignored).

| Merge | Branch | Head |
|---|---|---|
| `8fbd0fa7` | `fn-219-tree-space-telperion` (Laurelin's fn-218 under it) | `6889ed20` |
| `51c2edfc` | `fn-213-tree-space-the-silver-birch-as-a-point` (birch round 14) | `b5248f96` |
| `965b94dd` | `fn-236-tree-space-the-oregon-white-oaks-crown` (oak round 3) | `1c091dfb` |

fn-219 carried F4 up to `7190eac5`, not `6831fdb0` as briefed: its own F4 merge `50958721` took `7190eac5`. fn-235's commits came in from F4's side; the fn-219 merge had no conflicts.

## Conflicts

With the birch, Laurelin and Telperion on the engine, every preset carries an engine species. Each conflict came from that or from a digest.

- **`tests/space_presets.rs`:** `engine()` lists all seven engine presets with their species, stems and light. `a_preset_without_an_engine_species_carries_none` had no member left and was deleted.
- **`harness/parity.test.ts`:** `ENGINE` lists all eight presets. The refusal loop over `NONE` had no member left and was deleted.
- **`suite/outline.rs`:** the envelope-walk test kept F4's birch-to-itself walk. A walk between two engine species still walks the envelope rows linearly, and the test passes.
- **`suite/sweep.rs`:** `HELD` was recomputed from the merged walk, not merged by hand. It went from 144 to 118 rows: the birch moves the envelope, clump, peel, curtain, sag and twig-length rows, and the doubled supernatural switch was removed.
- **`tests/catalogue_identity.rs`:** the family, walk and override digests were read from the merged tree after each merge, and the refusal digest is F4's. The final values are family `2_333_297_806_083_657_236`, walk `17_905_568_364_190_521_403` and override `6_556_638_518_150_561_013`.
- **`src/browser/presets.generated.ts`:** git merged it without conflict. `npm run wasm:build` after each merge reported the metadata current.
- **`src/field/field.test.ts`** (after the merges, `npm test`): Telperion is no longer refused, so its refusal line and the every-preset test's no-species branch were removed. The every-preset field test now grows eight fields. It took 65.4 s unloaded against vitest's 60 s default, so it has its own 180 s timeout, like the parity tests' per-test timeouts. **The host should check this timeout.**

## Checks

- **Values:** `laurelin.values` and `telperion.values` equal fn-219's, and `laurelin.values` equals fn-218's. `silver-birch.values` and `telperion-space/src/birch.rs` equal fn-213's, and `oregon-white-oak.values` and `telperion-space/src/oak.rs` equal fn-236's. No other `.values` file moved from F4.
- **`space_hash`** (`raw/merge-species/hash-after.txt`): the 12 rows for beech, spruce and palm at 40 and 80, seeds 1 and 7, equal MERGE-235.md's, node counts included.
- **The oak's new hashes** (round 3, which differs from MERGE-235 by design):

```
oak 40 1 280342 7bf8be4d40c11dc4
oak 40 7 371391 951edeeccad6af19
oak 80 1 1407033 2d613f2790db59c7
oak 80 7 2197974 b40e0f886a90db11
```

  fn-236 recorded no `space_hash` of its own, so these were not compared with a branch run.
- **The birch's DBH** (`raw/merge-species/dbh-after.txt`, from fn-213's `birch_dbh.rs`, copied in to run and then removed): 30.7, 33.0 and 30.3 cm at 40, and 59.7, 60.3 and 62.1 cm at 80, for seeds 1, 7 and 42. These are round 14's values, and the node counts equal fn-213's MERGE-F4 run.
- **Laurelin and Telperion, natively and in Wasm** (`raw/merge-species/parity.log`): the node buffers are byte-identical. Laurelin's is 159,416,700 bytes and Telperion's 224,126,280, under the browser's cap. Every other preset is also byte-identical, and both fields grow in the field test.

## Gate (`RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`)

- **Render half, under the GPU lock:** 214 passed, 1 failed and 39 ignored (`gate-render.log`). The failure was fn-235's cold-build test, where `grow` was made cold in 24.6 s against 20 s while the workspace half shared the CPU. Run again alone under the lock at load 0.4 (`gate-render-cold-rerun.log`), it passed in 21.6 s. The merge changes no shader or `generation/` file.
- **Workspace half** (`--exclude telperion-render`): 1,065 passed, 0 failed and 50 ignored (`gate-rest.log`). The two new ignored tests are fn-218's and fn-219's leaf-count measurements, at 80 and at 92.
- **`npm test`:** 13 files and 143 tests passed after the field-test fix, and the generated files were unchanged (`npm.log`). The first run failed the two field tests described above.
