# fn-235 merged into F4 (host merge, 2026-10-09)

F4's head `7190eac5` took fn-235 (`1a79ffdb`, 19 commits on base `8d19611b`, done, review SHIP). `scratch-fn-235-birch` was not merged.

## Conflict: `apex.wgsl`

fn-235 restructured the apex loop so that `run`, `grow_unit` and `abortion_after` each have one call site: `advance` now grows a failing unit and draws its abortion threshold, and `fail` keeps only its tail. F4's fn-231 had added fractional elements as making draws: the place presence (`stands`, `place_q`), the partial unit laying its share of nodes (`laid_nodes`, the `LAID` draw, `U_LAID`), and the `PLACED`/`LAID` lineage keys.

The resolution puts fn-231's semantics into fn-235's structure:

- **`fail`:** fn-235's tail, unchanged. F4's copy grew the lived part with `grow_unit(.., lived.x, 0u, 0u)`, a unit no lifespan cuts.
- **`advance`:** fn-235's single `grow_unit` call, with fn-231's cut. `span_of` is computed as in fn-231 but held at 0 for a failing unit, as F4's `fail` held it, and is written to `U_LAID` (0 is what `push_unit` already leaves). The call is `grow_unit(a, pa, unit, cycle, select(grows, span_of, span_of != 0u), grows, span_of)`. For a unit that does not fail, `grows` is fn-231's `share`.
- Each of `run`, `grow_unit` and `abortion_after` still has exactly one call site.

## Cold compile: two slimmings, bit-identical

The resolved merge made `grow` cold in 21.4 s (twice) and failed the cold-build test at 22.3 s; fn-235 alone made it in 11.7 s on the same machine. A bisection with `make_grow` attributed the extra time to fn-231's two `lead_q(0, t, d, ..)` copies in `grow`'s inlined body, about 3 to 5 s each: the `LAID` draw in `grow_unit` and `place_q` in `sleep`. Hoisting the laid draw out of the node loop and merging the loops in `laid_nodes` both measured slower and were dropped.

- **The woken bud's place presence leaves `grow`.** `sleep` stores the place key in the sleeper's words 10 and 14 (pend 12 and 13) in place of its presence. `take_ask` multiplies it into the woken bud's first ask, over that ask's own window (the same `words[table[zat + 13] + cycle * npa + pa]`), at the call site the lateral place already used. `made_q` loses its woken branch. So `place_q` has one call site, in `asks`.
- **`under_lead`** (`apex.wgsl`) is `lead_q(0, t, d, ..)` to the word, its four `log2q` from one call site, which the laid draw uses.

The result is `grow` cold at 14.3 to 15.2 s, `asks` 0.9 to 1.0 s (0.4 s on fn-235; fn-231's lateral `place_q`), `assign` 0.14 s, and the beech at 20 cold-built in 19.0 to 20.0 s, 4.7 s besides `grow`. That is inside fn-235's 5 s, but by only 0.3 s: the first tree's bound is the tight one now.

## Neutral rows give F4's trees (`space_hash`)

`raw/merge-235/hash-before.txt` (at `7190eac5`) and `hash-after.txt` (merged) match in all 16 rows, node counts included, and equal MERGE-231.md's.

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

## Gate and npm (raw output in `raw/merge-235/`)

The gate ran with `RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`.

- **Render, under the GPU lock:** 215 passed, 0 failed, 39 ignored (`gate-render.log`). This includes the chain, realized and stepped parity tests with fn-231's fractional buds, cut units and fractional lifespans, the key and selector mirror tests, and fn-235's cold-build test.
- **The rest of the workspace:** 1,066 passed, 0 failed, 47 ignored (`gate-rest.log`).
- **`npm test`:** 13 files and 143 tests pass; generated files were unchanged (`npm.log`).
