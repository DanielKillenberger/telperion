# fn-249: the site's request after R1 to R4, against 0.1.4

All four changes are in, and no tree changed. The oak is still 2.5 to 4.7 times 0.1.4 at the site's request. The spruce is 1.3 to 1.5 times. The birch is 2.3 to 2.8 times on the first tree a Wasm instance grows, and 0.5 to 1.1 times on every later tree of the same species and age. **The site never sees that warm birch today.** `growField` makes a new Wasm instance for every tree (`src/field/index.ts:88`), so R1's rows are dropped after each tree. The cold column is what the site gets now.

## Per change, Wasm, one thread

| Change | What it does | Measured gain |
|---|---|---|
| R1 (31766be5) | Keeps the closed-form rows for the 4 most recently used (species, age, mode) keys in the process | Birch warm grow 752-833 → 112-192 ms. Spruce gains 40-50 ms warm; the oak's warm gain is within noise. Cold is unchanged. |
| R2 (473fae95) | `I256::quotient` uses Knuth's algorithm D on 32-bit limbs, not a 256-step bitwise division | Oak 125-250 ms. None for birch or spruce. |
| R4 (3a473741) | `Grid::corners` uses fixed arrays, not two `Vec`s per call | Oak shade span 157 → 132 ms and 121 → 93 ms (instrumented build, best of 5). Lost in whole-grow noise otherwise. The gathering buffers in `grow/stepped/feedback.rs:81` were not changed: they are allocated 40 times per tree, not per leaf. |
| R3 (3cc60b48) | The unbent lay leaves empty, childless laterals unlaid; the bent lay frames them | Spruce 22-27 ms, birch 4-12 ms. Oak is within noise. |

R3 measurement, taken first: with every empty, childless axis skipped in both lays, `place` fell by about 51 ms for the spruce (54+78 → 29+52 ms, unbent + bent), about 30 ms for the oak and 10 to 21 ms for the birch (`raw/r3-measure.txt`). That is the ceiling for all placeholder framing. The unbent half is the part that is a duplicate. Astra's estimate of 50 to 160 ms was too high: the placeholders are 80 to 96 % of the axes, but they take no phytomer steps.

## End to end at the site's request, ms, best of 3

End to end is grow, the grid, the query and voxelize. "Cold" is the first grow in a fresh instance; "warm" is the second, with the other seed. Each seed was measured both ways, in two processes. Load was 3.8 to 5.3 on 32 cores (Node v26, one thread). 0.1.4 is killenberger.com's `node_modules/telperion/dist/telperion-field.wasm`, called with its two-argument `grow`. F4 used cut 1,024.

| Tree | n | F4 before, cold | now, cold | now, warm | 0.1.4, cold | 0.1.4, warm | now ÷ 0.1.4, cold / warm |
|---|--:|--:|--:|--:|--:|--:|--:|
| oak 1234 | 24 | 1,434 | 1,123 | 978 | 240 | 189 | 4.7× / 5.2× |
| oak 1234 | 36 | 1,476 | 1,166 | 1,021 | 272 | 222 | 4.3× / 4.6× |
| oak 1234 | 48 | 1,550 | 1,239 | 1,090 | 330 | 284 | 3.8× / 3.8× |
| oak 5678 | 24 | 1,067 | 827 | 712 | 280 | 231 | 3.0× / 3.1× |
| oak 5678 | 36 | 1,097 | 863 | 747 | 311 | 271 | 2.8× / 2.8× |
| oak 5678 | 48 | 1,159 | 912 | 798 | 366 | 318 | 2.5× / 2.5× |
| birch 1234 | 24 | 922 | 912 | 223 | 323 | 276 | 2.8× / 0.8× |
| birch 1234 | 36 | 988 | 979 | 299 | 348 | 313 | 2.8× / 1.0× |
| birch 1234 | 48 | 1,110 | 1,099 | 412 | 400 | 374 | 2.7× / 1.1× |
| birch 5678 | 24 | 818 | 809 | 133 | 317 | 273 | 2.6× / 0.5× |
| birch 5678 | 36 | 852 | 838 | 170 | 346 | 307 | 2.4× / 0.6× |
| birch 5678 | 48 | 912 | 900 | 229 | 396 | 358 | 2.3× / 0.6× |
| spruce 1234 | 24 | 364 | 327 | 235 | 223 | 174 | 1.5× / 1.3× |
| spruce 1234 | 36 | 373 | 337 | 249 | 239 | 196 | 1.4× / 1.3× |
| spruce 1234 | 48 | 397 | 359 | 271 | 264 | 226 | 1.4× / 1.2× |
| spruce 5678 | 24 | 352 | 318 | 228 | 222 | 170 | 1.4× / 1.3× |
| spruce 5678 | 36 | 362 | 327 | 240 | 240 | 191 | 1.4× / 1.3× |
| spruce 5678 | 48 | 383 | 350 | 264 | 269 | 218 | 1.3× / 1.2× |

n = 30 sits between 24 and 36 in every row (`raw/final2.jsonl`; `bench/table.py` prints all four).

Grow alone, n-independent:

| Tree | F4 before, cold / warm | now, cold / warm | 0.1.4, cold / warm |
|---|--:|--:|--:|
| oak 1234 | 1,374 / 1,276 | 1,062 / 924 | 167 / 124 |
| oak 5678 | 1,020 / 943 | 781 / 672 | 195 / 154 |
| birch 1234 | 862 / 813 | 852 / 172 | 259 / 222 |
| birch 5678 | 784 / 741 | 773 / 105 | 263 / 227 |
| spruce 1234 | 338 / 281 | 302 / 215 | 148 / 108 |
| spruce 5678 | 328 / 273 | 294 / 208 | 152 / 104 |

## The remaining gap

- **Oak:** 550 to 910 ms over 0.1.4 cold. Its stepped cycles (balance, allotment, rough layout, lattice) are still about 2× slower in Wasm than natively, and the light model was out of scope (spec boundary).
- **Birch:** 490 to 700 ms over 0.1.4 cold. That is the closed form, recomputed in every instance. It closes only if the caller reuses an instance across trees: a change to `growField`'s contract, since the slim engine holds one tree at a time. That is the host's decision.
- **Spruce:** 80 to 105 ms over 0.1.4 cold.

## Byte identity

- **Field answers:** for every species, seed and n above, the SHA-256 over the bounds and all four answer arrays (`bench/bench.mjs`) is the same before (4b0f7b01) and after each change (`raw/*.jsonl`; `bench/summ.py` prints IDENTICAL for 24 of 24).
- **`space_hash`:** all 16 trees are the same before and after (`crates/telperion-render/examples/space_hash.rs`, `--profile ci`):

```
beech 40 1 nodes 102954 clothed f69922a372b7f3dc
beech 40 7 nodes 181199 clothed d7527f62c0e3f446
beech 80 1 nodes 1541615 clothed 80728e206bf237d7
beech 80 7 nodes 2611352 clothed 86eb0cf4269dfd82
spruce 40 1 nodes 1147168 clothed b32c14b9a41e5405
spruce 40 7 nodes 1107132 clothed 4d7fe97d251a607e
spruce 80 1 nodes 4635552 clothed 35e51826f380cf43
spruce 80 7 nodes 4608273 clothed 38eb1907944fb770
oak 40 1 nodes 280038 clothed e4c61b383bd3045c
oak 40 7 nodes 371248 clothed 5b346063c56d2d8a
oak 80 1 nodes 1405483 clothed 60a103c038ce1f46
oak 80 7 nodes 2193140 clothed 8247c6ebeab36727
palm 40 1 nodes 993 clothed da01c8cbf52dd468
palm 40 7 nodes 993 clothed de27cb076da06ca5
palm 80 1 nodes 1473 clothed 7c90dd11f427e83a
palm 80 7 nodes 1473 clothed 84ca7b46a037c49d
```

## Reproduce

Build the slim binary (`cargo build --release --target wasm32-unknown-unknown -p telperion-field --config profile.release.package.telperion-field.strip=true`) and copy it beside `bench/`. Then run `bench/sweep.sh <wasm> <out.jsonl>`, followed by `python3 bench/table.py <before> <after> <v014>`.
