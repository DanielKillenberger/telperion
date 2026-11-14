# The `telperion/field` path killenberger.com imports, 0.1.4 against fn-198

The owner asked on 2026-10-07: "The tree field that we import in killenberger.com with the telperion npm package. How is perf impacted for that path?" This file measures that path. It changes nothing.

## The site's request

The site's code is in `killenberger.com/src/lib/grove/` and `src/components/grove/`, and was only read. A worker calls `growField(species, seed, { source: compiled })` with no limb order, so the family's own limb order applies. It then runs `grid(bounds, n)`, `field.query(cells)` once, and `voxelize(..., { woodCutoff: cell * wood, thin })`. There is no age option, so the preset's default applies. On the branch that is `/age = 40` for the oak, spruce and palm. 0.1.4 has no age: it ran the direct build.

| Caller | Species | n |
|---|---|---|
| Portrait, specimen card (`growTree` default) | any of `SPECIES` (oak, birch, spruce, palm) | 48 |
| Grove, layers far to near | oak, birch, spruce (`GROVE_SPECIES`) | 24, 30, 36 |
| `warmGrove` | silver-birch | 12 |

Per-species dials come from `species.ts`: oak 0.08 / tuft 0.4, birch 0.08 / tuft 0.32, spruce 0.1 / density 0.8, palm 0.08 / density 0.5. Seeds 1234 and 5678 stand in for the site's four-digit seeds.

## Builds

- **0.1.4**: the `v0.1.4` tag, commit `c79bb590`. It was built from `git archive` with the package's own command (`cargo build --release --target wasm32-unknown-unknown -p telperion-field`, strip on). It is the same size as the `telperion-field.wasm` installed in the site's `node_modules` (344,798 B; the hash differs). The installed file gives the same cubes (oak 1234: 634 wood, 10,265 foliage) and the same timing.
- **branch**: fn-198 at `b65c0750`, the same command.
- **Wasm under Node**: Node v26.8.1, one thread (`wasm32-unknown-unknown` has none). One process per tree. The site's own `dist/field.js` and `dist/voxelize.js` from 0.1.4 were used for both builds; the wrapper differs between the builds only in a comment. Memory is the instance's linear memory after `growField` and after the queries. Linear memory never shrinks, so the larger figure is the peak.
- **Native**: the slim ABI called from a Rust bin that counts the heap peak in a global allocator, with `RAYON_NUM_THREADS=1`. Growth and query only; the voxelizer is TypeScript.

## Load on the machine

Another worker's binary ran on 26 cores throughout. Load average was 76 to 131 on 32 cores during the smoke runs, 27 to 55 during the three-repeat sweep, and 12 to 15 during the final pass. Times move with load: one oak 0.1.4 smoke run took 2.3 s at load 76 against 0.17 to 0.21 s at load 15. The table therefore gives the quietest pass (load 12 to 15). The three-repeat medians at load 27 to 55 are listed under it. Cube counts, memory and module sizes do not depend on load.

## Result, n = 48, Wasm, one thread, quietest pass

| Species, seed | Build | growField ms | query ms | voxelize ms | cubes (wood + foliage) | Wasm memory MB | height m |
|---|---|---:|---:|---:|---:|---:|---:|
| oak 1234 | 0.1.4 | 166 | 147 | 25 | 10,899 (634 + 10,265) | 72.6 | 24.5 |
| oak 1234 | branch | 1,351 | 242 | 11 | 6,930 (3,898 + 3,032) | 304.9 | 12.5 |
| oak 5678 | 0.1.4 | 206 | 164 | 25 | 10,723 (649 + 10,074) | 115.6 | 24.1 |
| oak 5678 | branch | 515 | 95 | 8 | 3,650 (1,999 + 1,651) | 123.9 | 13.5 |
| birch 1234 | 0.1.4 | 258 | 136 | 25 | 8,045 (712 + 7,333) | 59.7 | 16.9 |
| birch (any seed) | branch | refused in 2 to 3 ms: `Field core: invalid input: no engine species` | | | | | |
| birch 5678 | 0.1.4 | 267 | 121 | 23 | 7,526 (714 + 6,812) | 50.6 | 17.0 |
| spruce 1234 | 0.1.4 | 156 | 125 | 6 | 5,818 (250 + 5,568) | 87.0 | 15.3 |
| spruce 1234 | branch | 2,942 | 608 | 6 | 4,883 (421 + 4,462) | 1,014.5 | 12.3 |
| spruce 5678 | 0.1.4 | 154 | 115 | 6 | 5,798 (255 + 5,543) | 87.3 | 15.3 |
| spruce 5678 | branch | 2,853 | 578 | 7 | 4,878 (433 + 4,445) | 1,020.3 | 12.2 |
| palm 1234 | 0.1.4 | 14 | 28 | 5 | 3,719 (262 + 3,457) | 8.7 | 26.1 |
| palm 1234 | branch | 18 | 34 | 8 | 5,364 (302 + 5,062) | 8.9 | 22.1 |
| palm 5678 | 0.1.4 | 14 | 28 | 5 | 3,448 (255 + 3,193) | 8.7 | 27.0 |
| palm 5678 | branch | 18 | 33 | 7 | 5,409 (304 + 5,105) | 8.9 | 21.9 |

A tree end to end (grow, query and voxelize at n = 48) takes:

| Species | 0.1.4 | branch | Change |
|---|---|---|---|
| Oak | 0.34 to 0.40 s, 73 to 116 MB | 0.62 to 1.60 s, 124 to 305 MB | ×1.6 to ×4.7 in time, ×1.1 to ×4.2 in memory. Strongly seed dependent. |
| Spruce | 0.28 to 0.29 s, 87 MB | 3.4 to 3.6 s, about 1 GB | About ×12 in time, ×11.7 in memory. |
| Palm | 0.05 s, 9 MB | 0.06 s, 9 MB | About the same. |
| Birch | 0.41 to 0.42 s | refused | |

**Medians of three repeats at load 27 to 55** (min..max in `raw/field-path/`, gitignored):

| Species | 0.1.4 growField ms | branch growField ms |
|---|---:|---:|
| Oak 1234 | 211 | 2,017 |
| Oak 5678 | 207 | 749 |
| Birch | 299 to 332 | refused |
| Spruce | 188 to 222 | 3,451 to 3,534 |
| Palm | 15 | 20 to 21 |

The spruce query on the branch is 730 to 850 ms at every n, against 90 to 175 ms on 0.1.4.

**Grove grid sizes, quietest-pass shape from the sweep.** The branch's query cost barely falls with n for the spruce: about 0.65 to 0.8 s at n = 24 and n = 48 alike. That looks like a fixed cost per query; its cause was not measured. On 0.1.4 the query falls from about 170 ms at n = 48 to about 80 to 105 ms at n = 24. Voxelize is under 30 ms everywhere and lower on the branch because the oak's foliage is thinner. Cube counts at n = 24 / 30 / 36 / 48:

- Oak 1234: 1,703 / 3,123 / 5,115 / 10,899 on 0.1.4, against 1,298 / 2,257 / 3,507 / 6,930 on the branch.
- Spruce 1234: 901 / 1,629 / 2,660 / 5,818 on 0.1.4, against 945 / 1,594 / 2,466 / 4,883 on the branch.

The oak's mix turns from foliage-heavy (6 % wood) to wood-heavy (about 55 % wood).

## Native reference, one thread, n = 48 (load 17 to 20, two runs agree within 10 %)

| Species, seed | 0.1.4 grow / query ms, heap peak MB | branch grow / query ms, heap peak MB |
|---|---|---|
| Oak 1234 | 111 to 122 / 96 to 97, 72 | 1,276 to 1,394 / 175 to 178, 305 |
| Oak 5678 | 131 / 110 to 112, 110 | 468 to 477 / 65 to 73, 122 |
| Birch | 232 to 254 / 81 to 92, 50 to 56 | refused |
| Spruce 1234 | 98 to 99 / 75, 83 | 2,459 to 2,546 / 479 to 487, 985 |
| Spruce 5678 | 96 to 102 / 73 to 75, 83 | 2,333 / 462, 963 |
| Palm | 5 / 15 to 16, 7 | 5 / 21 to 22, 7 |

The spruce on the branch with every core (rayon's default pool) grows in 1.31 to 1.34 s against 2.46 to 2.55 s on one core; the site's Wasm has one. One sweep run at a load spike gave the branch spruce 7.9 to 8.1 s native and was discarded for this table; it stays in `raw/field-path/sweep.jsonl`.

## The field module's size

| | 0.1.4 | branch | Change |
|---|---:|---:|---:|
| `telperion-field.wasm` raw | 344,798 B | 816,675 B | ×2.37 |
| gzip -9 | 131,051 B | 298,461 B | ×2.28 |
| brotli -q 11 | 107,758 B | 235,446 B | ×2.18 |

The branch's slim core now carries `telperion-space`, the engine and its closed form, as `telperion-core`'s dependency.

## What the silver birch does on the branch

`growField("silver-birch", …)` rejects in 2 to 3 ms with `Field core: invalid input: no engine species`, natively and in Wasm, at every seed. The preset has no `/space` row. The site's consequences, read from its code:

- The grove (`Grove.tsx`) grows its trees with `Promise.all(plants.map(growTree …))` and swallows a rejection. Each plant draws its species uniformly from oak, birch and spruce, and a 1,280 px strip has 39 plants (20 + 12 + 7). The chance that a grove holds no birch is (2/3)^39, about 10^-7, so on the branch the grove never draws.
- `warmGrove` warms each worker with a birch at n = 12, so on the branch it warms nothing. Its `.catch` hides that.
- The portrait and specimen card draw from all four species, so about one tree in four shows a failure.

## Exploratory: the field from the structure above fn-209's cut only

This needs code, so it stops here.

- The slim entry has no option for it. Its request is a species id, a seed and a limb order (`crates/telperion-field/src/grow.rs`), and the limb order only names limb systems in the answer. The repository's own test `the_same_species_and_seed_answer_identically_and_the_order_names_limbs` asserts that the order changes no flag.
- `pipeline::Request.field` takes only the limb order.
- `pipeline/space.rs::grow` always calls `telperion_space::grow`, which returns the whole tree. For the spruce and the palm, `telperion_space::grow` already runs `grow_fine` at N = 1,024 internally but assembles every fine root back in, "to the bit". For the oak it grows whole, because the oak reads light and keeps retained pipes (`retained 0.5`, `leaf_girth 1.0`), and `grow_fine` refuses both cases (fn-211).

What it would take: a request option carried from the TS `GrowOptions` through the slim ABI, `grow::field` and `pipeline::Request` into `GrowInput`, under which `space::grow` calls `grow_fine` (or the settled stage before `assemble`) and converts only the axes above the cut, without the fine roots' axes. That would make the field answer a coarse tree; whether that is acceptable is a design question for the host. It would not reach the oak without fn-211's stepped executor. SPLIT.md's figure for what it would save is that the CPU keeps 1.1 % of the spruce's nodes above the cut. That was not measured here on this path.

## Reproduce

The scratch harness sits in the gitignored `raw/field-path/`, not committed. It is `run.mjs`, which reproduces the worker's `grow` with the package's `dist/field.js` and `dist/voxelize.js` and takes the Wasm path, species, seed, list of n and dials, plus a native bin, `field-native`, on the slim ABI with a counting allocator. Build each Wasm with `cargo build --release --target wasm32-unknown-unknown -p telperion-field --config profile.release.package.telperion-field.strip=true` in a checkout of the tag and of the branch.
