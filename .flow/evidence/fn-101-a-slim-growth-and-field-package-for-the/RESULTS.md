# fn-101 results (2026-09-22)

Base: the fn-100 tip `b8c94d03` (PR #55), in its own worktree. The full
binding on that tip, built here before any edit, is 1,261,063 bytes raw and
306,093 bytes brotli-11 (`raw/full-base-b8c94d03.wasm`, ignored); the spec's
figure for master `3fdfb440` is 1,242,383 raw and 344,254 brotli-11. Every
size is `brotli -q 11 -c <file> | wc -c` with brotli 1.2.0; every timing is
Node v26.8.1 on the 32-core desk (AMD Ryzen 9 5950X, 31 GB), the load
average recorded beside it.

## The build

The slim binding is its own crate, `crates/telperion-field`, rather than a
feature of `telperion-wasm`: its artifact has its own name, so the two
builds never overwrite each other in `target/`; its dependency on the core
carries `default-features = false`, so no feature unification with the full
crate can pull the geometry back in; and `cargo tree` answers R2 directly.
The chain it runs is `presets::by_identity` (moved out from under the `json`
feature with the catalogue tables, `params` re-exporting them), the seed set
on the skeleton, `branching::generate`, `build_element`, `plan::plan` and
`Field::planned` (`crates/telperion-field/src/grow.rs`). A family the plan
cannot describe is refused with `family without a leaf plan`; no placement
is linked.

## R1: size

| file | raw bytes | brotli-11 |
|---|---|---|
| `telperion_field.wasm` (slim) | 340,213 | 101,417 |
| `dist/field.js` (the entry point, Vite library build) | 2,690 | 1,067 |
| slim entry point download, Wasm + JavaScript | 342,903 | **102,484** |
| `dist/voxelize.js` (the example, only if imported) | 4,526 | 1,670 |
| full `telperion_wasm.wasm`, this tip | 1,261,063 | 306,093 |
| full, master `3fdfb440` (spec) | 1,242,383 | 344,254 |

Target: half of 344,254 is 172,127; the slim download is 30 % of the spec's
figure and 33 % of this tip's. The raw module still carries a 35,781-byte
function-names section (`raw/slim-twiggy-top.txt`, `twiggy top`); the
release profile does not strip it and it compresses to a few KB, so it is
left alone. Not measured against: Vite's library mode inlines every asset
as a base64 data URL, which made the first `dist/field.js` 456,306 bytes
(142,551 brotli); the entry point now names the Wasm through a constant so
the bundler leaves the URL to run time, and `scripts/build-wasm.mjs` places
the file beside `dist/field.js`.

## R2: what the slim build retains

`cargo tree -p telperion-field --target wasm32-unknown-unknown` is the core,
`libm` and `slotmap` (`raw/slim-cargo-tree.txt`); no serde, serde_json or
bincode. Of the 436 symbols `twiggy` lists (`raw/slim-symbols.txt`), the
core's are 85 in `branching`, 35 in `field`, 23 in `foliage`, 19 in `tree`,
5 in `radius`, 4 in `envelope`, 3 each in `presets`, `noise`, `math` and
`bias`, 1 each in `twigs`, `colonization` and `surface`
(`raw/slim-modules.txt`). The one `surface` symbol is
`SurfaceParams::validate`, the plan validating a value table; there is no
surface builder, no `placement`, `short_shoots`, `station`, `prepared`,
`mesh`, `footprint`, `material` or specimen-API symbol, and no JSON. The
`branching::specimen` symbols are the direct build's grower, not the
specimen API; the only `Placement` traces are that grower's drop glue for
its timeline map type. `dist/field.js` imports nothing from
`dist/telperion.js` (Vite emits it as its own entry, 2,690 bytes).

## R3: from species and seed to a queried grid, in Node

`measure.mjs`, rows in `raw/r3-timings.json`; the grid is the voxelizer's,
64 cells a side with 62 across the longest axis. The full binding was
measured in the same two minutes with fn-100's `measure.mjs` (its grid is
64 cells across the longest axis, over the bounds exactly;
`raw/r3-full-timings.json`). Load average 3.9 to 4.1 (`raw/r3-load.txt`); an
earlier run at load 15.8 is in FRICTION.md and discarded.

| tree | cold ms (compile + grow + query) | warm median ms (grow + query) | full binding, same minute |
|---|---|---|---|
| oak, seed 1 | 388 (17 + 178 + 211) | 349 (146 + 204) | 328 (139 + 189) |
| oak, seed 7 | 389 (1 + 166 + 224) | 388 (165 + 222) | 386 (165 + 220) |
| birch, seed 1 | 407 (1 + 217 + 190) | 406 (216 + 190) | 1,065 (626 + 367) |
| birch, seed 7 | 648 (1 + 406 + 242) | 649 (407 + 243) | 634 (404 + 232) |
| spruce, seed 1 | 292 (1 + 135 + 157) | 289 (135 + 154) | 275 (126 + 148) |
| spruce, seed 7 | 273 (0 + 126 + 147) | 273 (126 + 147) | 264 (120 + 143) |

**R3 is missed on every tree**: the 250 ms ceiling is met by none of the
six. The slim entry point runs at parity with the full binding on the same
code in the same minute (the birch at seed 1 caught a spike on the full
run; every other pair is within 7 %), so the owner's constraint that the
restructure be no slower than today holds, and the binding layer has
nothing to trim: compile and instantiation are 1 to 17 ms, and the rest is
`branching::generate` and the plan query fn-100 measured at 288, 330, 345,
579, 233 and 223 ms on this same code the day before. The 250 ms figure was
inferred from the prototype's structure-export path (93 to 205 ms), which
never ran the plan query. A faster query is a core change (fn-100's own
note: a flags-only query could keep the placed path's early exit, at the
cost of the leaf counts and limb ids the voxel rules read); that is the
host's call, not this task's. Recorded as `NEEDS_HUMAN` in the task file.

## R4: the full build is unchanged

The rebuilt full module's export list is identical to the pre-edit build's
(names and kinds compared in Node); the bytes differ by one, from the
catalogue tables moving between modules. `tests/browser/bindings.mjs` runs
unedited and green (`raw/browser-bindings.log`), `npx vitest run` is 123
tests green after `scripts/species-profiles.test.mjs` was pointed at the
table's new file (`raw/vitest.log` holds the run that found it), and the
workspace gate `cargo test --profile ci --workspace --no-fail-fast` is 119
suites, 891 passed, 0 failed, 21 ignored (`raw/gate-cargo-test-ci.log`).

## R5: the smokes

`src/field/field.test.ts` loads the slim entry point in Node from the
module's bytes, grows the ordinary family at seed 5 with limb order 1,
answers a 12^3 grid, and holds every answer array byte for byte to the full
`TreeEngine`'s field on the same family, seed and order; it also pins the
refusals (unknown and in-work species, a fractional or out-of-range seed, a
negative limb order, an unpacked batch, a negative half extent, a NaN
centre) and the released handle. `tests/browser/field.mjs`, run by
`scripts/test-wasm.mjs`, imports the entry point into a module worker from
the served origin, lets it fetch the Wasm from beside itself (no source
passed, no bundler-resolved URL), and answers one whole-tree cell.

## R6: the sheets

`experiments/voxel-field/voxels.mjs` now grows through `growField` and
draws through `grid` and `voxelize`; the structure-export path and the
`FIELD` switch are gone. At the accepted dials:

- `LIMB_ORDER=3 node experiments/voxel-field/voxels.mjs 64` (keeps 0.5,
  0.3, 0.15, the coin rule, wood cutoff 0.2 of a cell, seed 42) is byte for
  byte the owner's order-3 sheet, sha256 `66bbd3ad…`
  (`raw/sheet-64-field-order3.png` against
  `raw/accepted-sheet-64-candidate-field-order3.png`).
- `WOOD_ONLY=1 CUT_METRES=1 node experiments/voxel-field/voxels.mjs 64
  0.03,0.02,0.015` is byte for byte the wood-only sheet, sha256
  `5bf0a5db…`, 2 cm in the middle row.

The clump counts match fn-100's table (oak 281, birch 204, spruce 2,974 at
order 3).

## R7: the example voxelizer

`src/field/voxelize.ts` is typed, imports only types from the entry point,
and is tested on a four-cell fixture grid in `src/field/voxelize.test.ts`:
the grid's geometry, the wood cutoff at three values, each of the five
rules at a keep that separates them, the coin's whole-limb behaviour, the
clump count, and the refusals. `docs/field-package.md` documents the entry
point first and names the voxelizer an example beside it.

## Friction

Three entries in FRICTION.md: the accepted sheets' location, the loaded
machine during the first R3 run, and R3's unchecked ceiling.
