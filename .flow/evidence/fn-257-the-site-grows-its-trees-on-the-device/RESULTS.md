# fn-257: the field on the device, measured (worker, 2026-10-16)

Warm, RTX 3080, one GPU job at a time (`measure.sh`, `raw/measure/`). Natively through `Generator::grow_field` and `query_field`, the calls `telperion/field`'s `growField` and `query` make in a browser; a browser's own times are the owner's probe page (`field-probe.html`, in a worker, as the site runs it).

## R3': per tree, grow and a 36-cell query, against 0.1.4

| Species | Seed | Grow ms | Query 36³ ms | Total ms | 0.1.4 ms (TEST5.md) | Field MB |
|---|--:|--:|--:|--:|--:|--:|
| oak | 1 | 240 | 12.6 | 253 | 290 | 18.0 |
| oak | 1234 | 293 | 12.9 | 306 | 273 | 31.7 |
| oak | 5678 | 257 | 15.5 | 273 | 332 | 23.9 |
| birch | 1 | 209 | 28.5 | 238 | 292 | 15.4 |
| birch | 1234 | 229 | 21.1 | 250 | 352 | 18.7 |
| birch | 5678 | 208 | 11.9 | 220 | 354 | 12.1 |
| spruce | 1 | 448 | 62.4 | 510 | about 240 | 100.2 |
| spruce | 1234 | 447 | 76.0 | 523 | about 240 | 102.3 |
| spruce | 5678 | 431 | 77.7 | 509 | about 240 | 98.9 |
| palm | 1, 1234, 5678 | 76-80 | 2.4-3.1 | 79-83 | 17-18 | 0.4 |
| ordinary | 1 | 432 | 5.9 | 438 | none | 5.0 |
| beech | 1 | 168 | 14.3 | 182 | none | 6.6 |
| Telperion | 1 | 938 | 79.4 | 1,017 | none | 101.8 |
| Laurelin | 1 | 871 | 76.4 | 947 | none | 95.6 |

- **Oak:** at or under 0.1.4 at two seeds of three, 33 ms over at 1234 (1,234's tree is the largest of the three, 31.7 MB of field).
- **Birch:** under 0.1.4 at every seed, by 54-134 ms.
- **Spruce:** about 0.51 s against the owner's "about 0.35 s or better": growth is 431-448 ms of it (its kernels', after the release), the query 62-78 ms.
- **Palm:** about 80 ms against 0.1.4's 17 ms, all of it growth's fixed cost; the palm is the site's portrait, one tree.
- The device's start and its programs' compile are once a page (R3-0) and are not in these numbers; the probe page reports them.

## The harness's build (`build_time`, warm), F4 → this branch

| Preset | Seed 1 | Seed 7 | Seed 42 | Device peak MB |
|---|--:|--:|--:|--:|
| ordinary | 430 → 436 | 457 → 461 | 481 → 473 | 184 → 194 |
| oak | 243 → 240 | 254 → 262 | 245 → 244 | 838 → 910 |
| spruce | 447 → 439 | 440 → 432 | 467 → 445 | 1,595 → 1,625 |
| beech | 168 → 167 | 182 → 182 | 215 → 208 | 207-312 → 220-325 |
| birch | 356 → 205 | 420 → 248 | 398 → 223 | 425-548 → 615 |
| palm | 79 → 76 | 78 → 75 | 79 → 76 | 35 → 35 |
| Telperion | 947 → 915 | 1,284 → 1,248 | 1,328 → 1,290 | 2,605-2,665 → 2,740 |
| Laurelin | 855 → 865 | 1,158 → 1,123 | 1,217 → 1,189 | 2,396-2,451 → 2,520 |

The birch builds 151-172 ms faster (task 3: no more moves); every other preset within run-to-run noise, Telperion 30-40 ms faster (its lattice no longer moves). The device peaks rise 2-14 % where the tables now hold their room from the start (task 3's headroom: the oak's +72 MB, the birch's table sized once at 615 MB where its moves had peaked at 425-548).

## Sizes (`npm run build`)

| Artifact | F4 | This branch | Ceiling |
|---|--:|--:|--:|
| `telperion.wasm` | 1,727,052 | 1,549,918 | 1,600,000 |
| `telperion-render.wasm` | 3,680,668 | 3,601,258 | 3,700,000 (raised, owner 2026-10-16) |
| `telperion-field.wasm` | 1,165,252 | gone | gone |
| `telperion.js` | 255,929 | 194,685 | 200,000 (raised) |
| `wasm-source.js` (the chunk both entries share: the loader and the renderer's glue) | | 64,564 | 70,000 (new) |
| `field.js` | | 2,780 | 16,000 |

`telperion.wasm` is back under its ceiling (the coarse cut's code is gone); the field entry no longer ships a module of its own.
