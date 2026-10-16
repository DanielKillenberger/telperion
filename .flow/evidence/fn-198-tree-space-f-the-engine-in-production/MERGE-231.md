# fn-231 merged into F4 (host merge, 2026-10-09)

F4's head `fa1b8ebf` took fn-231 in merge `297b25b4` (`cde559eb`, 32 commits on base `ce3b9dc5`, done, review SHIP). fn-231 makes the engine's walk continuous at its ends: a fractional bud place, a shedding fade and a lifespan-cut unit each count at their share, so the size a blend is solved to moves. It leaves every preset's rows as they were.

## Conflict

The only conflict was `crates/telperion-core/tests/catalogue_identity.rs`. Both sides had re-pinned digests there: F4 for fn-217's rows (MERGE-217.md), and fn-231 for the walk.

- **Header:** both histories are kept. F4's fn-217 paragraphs come first, then fn-231's. A new paragraph records this merge's re-pin.
- **Walk digest:** both sides had changed it. It was computed from the merged tree and re-pinned (below).
- **Family, override and refusal digests:** fn-231 left them at the base value, so F4's values auto-merged. The merged tree gives F4's values for all three.

Every other file auto-merged. That includes the `telperion-space` walk, the fine WGSL kernels, and `space_presets.rs` and `space_walks.rs`.

## Neutral rows give F4's trees (`space_hash`)

"Before" is `raw/merge-231/hash-before.txt`, built at `fa1b8ebf` with `crates/telperion-render/examples/space_hash.rs`. "After" is `raw/merge-231/hash-after.txt`, built on the merged tree. All 16 match exactly, node counts included. They also equal MERGE-217.md's:

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

## Re-pin (`catalogue_identity`)

The walk digest was re-pinned only after the hashes above matched. It is the only digest that moved.

- **Walk:** F4's `2_827_795_343_174_730_974` and fn-231's `8_472_664_931_003_817_987` became `4_263_024_408_501_985_780`. The merged walk is fn-231's walk run over F4's rows, which include fn-217's, so neither side's value can hold.
- **Family, override and refusal:** F4's values, unchanged.

## Generated files

fn-231 changes no preset, catalogue row or browser source.

- **`parameters.generated.ts` and `docs/parameters.md`:** these were rewritten with `TELPERION_WRITE_REFERENCE=1 cargo test --profile ci -p telperion-core --test parameter_reference`. The output was byte-identical.
- **`presets.generated.ts`:** `npm test`'s pre-test Wasm build left it unchanged.

## Gate and npm (raw output in `raw/merge-231/`)

The gate ran with `RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`.

- **Gate, render half under the GPU lock:** 205 passed, 0 failed, 35 ignored (`gate-render.log`).
- **Gate, the rest of the workspace:** 1,064 passed, 0 failed, 47 ignored (`gate-rest.log`).
- **`npm test`:** 13 files and 143 tests pass. The tests took 75.7 s (`npm.log`).
