# Surplus probe: "a tip extends by what its balance leaves" (13 Nov 2026)

Worktree `.worktrees/census-old` at `d9228bdd8`. Nothing committed or pushed. The worktree is clean again (`git status` empty, all patches reversed). The patches, logs and scripts are in this folder.

## Stopped or not run

- **V2 was not built.** It stopped at the escalation the brief asked for. On the oak, "this axis goes on" is not carried by `continuation` alone. The limb and the bough have `continuation = 0` but carry on by relays and a long lifespan. The branch carries on by `continuation = 1` and also relays. If c = `continuation`, V2 would slow branches fully and leave limbs and boughs untouched. The rows are below; choosing between them is the host's call.

  | row (index) | lifespan | continuation | relay / relayEnded / relayFailed | abortion | viability |
  |---|---|---|---|---|---|
  | trunk (2) | 12 | 1.0 | 0 / 0 / 0 | 0 | 1.0 |
  | low fork (3) | 2 | 1.0 | 0 / 0 / 0 | 0 | 1.0 |
  | leader (4) | 6 | 1.0 | 0 / 0 / 0 | 0 | 1.0 |
  | **limb (6)** | 500 | **0.0** | 1 / 1 / 1 | 0.15 | 1.0 |
  | **bough (7)** | 60 | **0.0** | 1 / 1 / 1 | 0.10 | 0.999 |
  | sprig (8) | 10 | 1.0 | 0 / 0 / 0 | 0 | 0.95 |
  | **branch (9)** | 12 | **1.0** | 1 / 1 / 1 | 0 | 0.98 |
  | shoot (12) | 4 | 0.0 | 0 / 0 / 0 | 0 | 0.93 |
  | twig (13) | 5 | 0.0 | 0 / 0 / 0 | 0 | 0.98 |
  | short (14) | 3 | 0.0 | 0 / 0 / 0 | 0 | 0.90 |

  Source: the preset `oregon-white-oak.values` plus the defaults in `oak.rs` `state(lifespan, continuation, zones)`. Bold rows are the structural tips T2 reads.
- **Nothing broke, panicked or was killed.** All 20 grows (V0 and V1 × seeds 1001 and 1007 × 5 ages) exited 0. The stop-early condition did not apply, because V2 was not built and V1 at 40 to 120 was not broken.
- **Timing.** No PAUSE fell inside any run: the last pause was 14:05:34 to 14:08:12, and the runs went from 14:08:25 to 14:32:41. Every row is timed.
- **Method.** One grow per age, each a fresh process. No snapshots inside a run.
- **Generated TS file.** `presets.generated.ts` was reverted with fn-361's own hunk, the one line `"dominance": 0.5`. The generator (`scripts/build-wasm.mjs`, a release wasm build) was not rerun. The grower runner does not read this file.

## Code reading (before running)

- **Where `size` goes.** `allot_one` writes `Pencil::size`. The unit then reads it through `Reads::size` (`shoot.rs`). Both `advance.rs` (a unit grown) and `stop.rs` (a failed unit) push it into `Draws::shade`, one entry per growth unit. Two places read it from there:
  - **During growth:** `draw.rs` `draw_bent` sets `p.size` and lays each phytomer at `scale × size`. So size scales the internode length and the path (`path_end`). During growth, leaf area is `light::leaf_area(scale, size, area)`, so size also scales the deposit the light field reads and the leaf that `girth::lay` turns into sapwood demand.
  - **At the settle:** `presence::assign_one` sets `phytomer.size`, and `geometry::scale` multiplies it into the phytomer's final `scale`, that phytomer only and not inherited by what it bears. So in the settled tree it scales internode length, leaf area (`leaf_area × scale`) and, through the leaf, girth.

  Size is not in a unit's presence (`Unit::size` is the grown share, a different field), so it does not change node counts or survival directly.
- **Is size 0 legal?** Yes, already today. `allocation::drawn` returns 0 for a bud with no vigour and ψ > 0. `invariants.rs` accepts `p.size >= 0`. `stand.rs` guards `scale > 0` before dividing. `Layer::step` divides by nothing that scales with it. The settle refuses (`Error::Collapsed`) only when every phytomer of the tree has scale 0. `draw.rs`'s `image()` asserts that the size is an f32's image, so the rule multiplies in f32 and widens the result. Measured: V1 gave sizes of exactly 0 (T2's s = 0 column, up to 63% of structural tips at seed 1001, age 300) and nothing failed.
- **Does `allot_one` run in every mode?** It runs in every production grow. `grow` → `grow_cycle` → `Paused`, which sets `one = true`, and `step` calls `balance_one` and `allot_one` every cycle unconditionally. The test-only whole-tree engine (`grow::run`, `whole`, `sketch`, `one = false`) uses `sketch.rs`'s `allot`, not `allot_one`, and balances only where `balances()` holds. The rule placed in `allot_one` alone therefore does not reach that engine.
- **Test promises the rule would touch.** None were run; this list comes from reading only.
  - **Year-independence** (`grow/cycle/paused/tests.rs`: `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`, `the_tree_at_a_year_is_the_horizons_prefix`, `every_year_is_the_horizons_prefix`, `a_year_is_a_state_not_a_horizon`, `a_growth_goes_on_past_its_first_request`). The rule reads only that cycle's pencil state, so these should still hold. Unverified.
  - **One cycle against its references** (`grow/cycle/tests.rs` `the_working_set_and_retirement_leave_the_tree_as_it_was` and its siblings). Both sides go through `allot_one`. Unverified.
  - **Whole engine against the one cycle.** Any test that holds `whole`/`run` trees to `grow_cycle` trees would now differ. `grow/tests.rs` `without_what_later_years_add_the_rough_layout_is_the_final_one` compares `whole` with `sketch`, both on the old engine, so it is not touched. I did not find which other tests compare across the two engines: unknown.
  - **Pins.**
    - `grow/growing_tests.rs` `tree_digest` pins (the oak at seeds 1 and 7 and the other presets).
    - `telperion-core/tests/catalogue_identity.rs` digests.
    - The oak's both-ends pair in `pipeline/cycle_cost_tests.rs`.
    - Every preset whose sketch exists, which is any species with shade or laid girth, because s < 1 wherever path > 0 and the inverse height is non-zero.
  - **Closed form** (`closed_form/predictor/tests.rs` `counts_at_40_and_80`, `the_trees_count_is_the_seed_row`, and expected lengths in `tests/closed_form.rs`). Counts may hold; expected lengths would move wherever a sketch exists. Unknown.
  - **Walks** (`tests/walks.rs` "by degree" and the `walk_*` tests in telperion-core). The rule is continuous in path, light and inverse height (the clamp at 0 is continuous), but the walks' pinned bounds would move. Unknown.
  - **Balance unit tests** (`balance_tests.rs`). The balance itself is untouched.

## What was run

- **Patches** (apply in order, reverse in reverse order):
  - `reader.patch`: the old-oak census reader plus this probe's printers (`surplus_probe.rs`, `SP|` lines).
  - `v0.patch`: the limb dominance row removed from the values file, `oak.rs` and `presets.generated.ts`.
  - `v1.patch`: the rule. In `allot_one`, `size = sized as f32 × surplus(path_end, inverse_height, light)`.
- **Commands:**
  - `build.sh <v>`: `TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --threads 4 -- cargo build --profile ci`, with the runner at `oldoak/runner`.
  - `runs.sh <v> "<seed>:<age>:<GB> ..."`: `OLDOAK=1 SURPLUS=1 TELPERION_HEAVY_RESERVE=8 telperion-heavy <GB> --threads 4 -- python3 peak.py bin/heldrun-<v> oregon-white-oak <age> <seed>`. Admission was 8 GB for seed 1001, 16 GB for seed 1007 up to 120, and 24 GB for seed 1007 at 200 and 300.
  - `python3 -I tables.py v0 v1` produces `tables.md`, which has the full tables including per-row T2.

## T1

Measured on the settled tree:
- **Height:** the top tip above the root collar's base.
- **Spread:** the x and y extent of all phytomer tips.
- **Radius at 1.3 m:** the first phytomer of the seed axis or its continuations whose tip reaches 1.3 m.
- **Leaf:** the reader's formula, leaf_area × scale × clamp(leaf_years − age), on open axes.
- **Living apexes:** the grower's live set in the last cycle's allotment.

| seed | age | var | height m | spread x / y m | r 1.3 m (m) | leaf m² | leafed ph | phytomers | axes | living apexes | ph / m² leaf | mean / gmean scale, leafed | peak GiB | s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1001 | 40 | V0 | 10.65 | 7.2 / 8.2 | 0.131 | 113.5 | 73,982 | 326,166 | 51,271 | 24,838 | 2,872 | 0.848 / 0.600 | 0.2 | 1 |
| 1001 | 40 | V1 | 10.34 | 6.8 / 8.0 | 0.122 | 98.7 | 76,653 | 336,150 | 52,053 | 25,348 | 3,406 | 0.711 / 0.486 | 0.2 | 1 |
| 1001 | 80 | V0 | 18.00 | 20.2 / 15.8 | 0.345 | 501.4 | 374,421 | 1,852,188 | 266,935 | 125,073 | 3,694 | 0.740 / 0.438 | 1.1 | 7 |
| 1001 | 80 | V1 | 16.24 | 16.6 / 13.2 | 0.289 | 326.8 | 399,122 | 2,044,675 | 287,302 | 138,020 | 6,257 | 0.452 / 0.261 | 1.2 | 7 |
| 1001 | 120 | V0 | 27.10 | 25.2 / 23.0 | 0.537 | 738.6 | 596,108 | 3,100,796 | 401,667 | 181,771 | 4,198 | 0.685 / 0.371 | 1.7 | 22 |
| 1001 | 120 | V1 | 22.01 | 18.8 / 17.9 | 0.441 | 508.7 | 620,563 | 4,083,487 | 546,473 | 252,907 | 8,028 | 0.453 / 0.229 | 2.3 | 26 |
| 1001 | 200 | V0 | 32.55 | 30.2 / 29.1 | 0.666 | 199.3 | 180,635 | 988,917 | 122,500 | 50,583 | 4,962 | 0.610 / 0.193 | 1.3 | 43 |
| 1001 | 200 | V1 | 26.17 | 21.7 / 21.4 | 0.671 | 537.8 | 693,524 | 6,049,818 | 811,689 | 363,152 | 11,250 | 0.428 / 0.220 | 3.5 | 84 |
| 1001 | 300 | V0 | 7.69 | 1.9 / 0.8 | 0.673 | 0.9 | 14,292 | 84,740 | 9,562 | 4,268 | 96,186 | 0.034 / 0.015 | 1.3 | 47 |
| 1001 | 300 | V1 | 27.74 | 22.9 / 20.9 | 0.828 | 290.8 | 472,766 | 5,673,749 | 754,906 | 331,536 | 19,513 | 0.340 / 0.166 | 3.5 | 165 |
| 1007 | 40 | V0 | 10.38 | 8.0 / 7.4 | 0.167 | 181.0 | 111,343 | 476,669 | 84,660 | 40,157 | 2,634 | 0.898 / 0.620 | 0.3 | 1 |
| 1007 | 40 | V1 | 10.08 | 7.8 / 7.1 | 0.155 | 152.3 | 117,554 | 500,295 | 86,701 | 41,459 | 3,284 | 0.716 / 0.474 | 0.3 | 1 |
| 1007 | 80 | V0 | 18.62 | 24.2 / 21.9 | 0.468 | 908.7 | 651,508 | 3,268,122 | 510,907 | 229,776 | 3,596 | 0.771 / 0.372 | 1.9 | 14 |
| 1007 | 80 | V1 | 16.79 | 20.1 / 18.4 | 0.392 | 624.3 | 870,262 | 4,615,265 | 707,246 | 327,112 | 7,392 | 0.396 / 0.184 | 2.6 | 17 |
| 1007 | 120 | V0 | 26.94 | 32.4 / 31.1 | 0.692 | 1,113.7 | 881,160 | 4,582,520 | 634,298 | 276,682 | 4,115 | 0.698 / 0.295 | 2.8 | 40 |
| 1007 | 120 | V1 | 21.62 | 22.9 / 23.9 | 0.645 | 926.4 | 1,170,021 | 9,796,359 | 1,406,671 | 640,019 | 10,575 | 0.437 / 0.174 | 5.7 | 67 |
| 1007 | 200 | V0 | 30.92 | 29.6 / 25.7 | 0.834 | 315.2 | 409,792 | 2,257,943 | 290,873 | 130,194 | 7,163 | 0.425 / 0.125 | 2.0 | 76 |
| 1007 | 200 | V1 | 26.22 | 25.8 / 29.5 | 0.995 | 1,246.6 | 1,747,883 | 14,210,952 | 2,046,543 | 864,208 | 11,400 | 0.394 / 0.175 | 8.4 | 219 |
| 1007 | 300 | V0 | 22.17 | 18.2 / 16.2 | 0.906 | 219.0 | 314,375 | 1,625,427 | 203,332 | 91,387 | 7,422 | 0.385 / 0.207 | 2.0 | 99 |
| 1007 | 300 | V1 | 27.91 | 31.2 / 26.5 | 1.242 | 761.3 | 1,829,273 | 15,115,522 | 2,035,509 | 875,642 | 19,856 | 0.230 / 0.117 | 9.0 | 480 |

## T2: living limb, bough and branch tips, in the age's own cycle

Values are p10 / p50 / p90. "Size before" is the allotment's size; "size given" is that size after the rule. In V0 the share s is computed but not applied. Per-row splits are in `tables.md`.

| seed | age | var | n | s = 0 | path_end m | light | s | size before | size given |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 40 | V0 | 870 | 0 | 7.40 / 8.79 / 10.03 | 0.14 / 0.33 / 0.79 | 0.63 / 0.81 / 0.91 | 0.40 / 0.84 / 1.72 | same |
| 1001 | 40 | V1 | 869 | 0 | 7.26 / 8.54 / 9.73 | 0.16 / 0.34 / 0.80 | 0.68 / 0.82 / 0.91 | 0.43 / 0.85 / 1.71 | 0.30 / 0.70 / 1.53 |
| 1001 | 80 | V0 | 3,724 | 418 | 11.52 / 15.12 / 18.13 | 0.16 / 0.29 / 0.76 | 0 / 0.43 / 0.69 | 0.43 / 0.84 / 1.71 | same |
| 1001 | 80 | V1 | 3,890 | 298 | 10.75 / 13.72 / 16.47 | 0.14 / 0.33 / 0.72 | 0.14 / 0.57 / 0.74 | 0.38 / 0.94 / 1.64 | 0.05 / 0.53 / 1.14 |
| 1001 | 120 | V0 | 5,012 | 1,612 | 15.80 / 21.51 / 28.03 | 0.28 / 0.42 / 0.78 | 0 / 0.12 / 0.41 | 0.51 / 0.93 / 1.60 | same |
| 1001 | 120 | V1 | 6,870 | 2,018 | 14.17 / 18.00 / 22.57 | 0.09 / 0.54 / 0.84 | 0 / 0.45 / 0.65 | 0.18 / 0.97 / 2.02 | 0 / 0.48 / 1.21 |
| 1001 | 200 | V0 | 1,558 | 862 | 22.03 / 29.83 / 34.12 | 0.52 / 0.71 / 0.90 | 0 / 0 / 0.38 | 0.61 / 0.95 / 1.49 | same |
| 1001 | 200 | V1 | 10,921 | 5,026 | 20.88 / 25.49 / 29.18 | 0.17 / 0.64 / 0.99 | 0 / 0.25 / 0.56 | 0.21 / 0.92 / 2.09 | 0 / 0.26 / 0.93 |
| 1001 | 300 | V0 | 94 | 2 | 18.36 / 18.98 / 19.24 | 0.37 / 0.45 / 0.64 | 0.19 / 0.34 / 0.53 | 0.66 / 0.87 / 1.65 | same |
| 1001 | 300 | V1 | 10,421 | 6,555 | 25.65 / 27.90 / 31.80 | 0.18 / 0.37 / 0.99 | 0 / 0 / 0.38 | 0.30 / 0.74 / 2.22 | 0 / 0 / 0.70 |
| 1007 | 40 | V0 | 1,729 | 0 | 6.97 / 9.01 / 10.19 | 0.09 / 0.26 / 0.76 | 0.50 / 0.77 / 0.89 | 0.33 / 0.83 / 1.85 | same |
| 1007 | 40 | V1 | 1,725 | 0 | 6.81 / 8.76 / 9.88 | 0.11 / 0.28 / 0.75 | 0.58 / 0.79 / 0.90 | 0.36 / 0.85 / 1.79 | 0.21 / 0.67 / 1.60 |
| 1007 | 80 | V0 | 7,895 | 1,705 | 8.51 / 15.98 / 19.64 | 0.12 / 0.22 / 0.70 | 0 / 0.29 / 0.60 | 0.37 / 0.73 / 1.78 | same |
| 1007 | 80 | V1 | 10,626 | 1,300 | 9.14 / 14.09 / 17.22 | 0.13 / 0.27 / 0.62 | 0 / 0.43 / 0.75 | 0.38 / 0.87 / 1.73 | 0 / 0.39 / 1.10 |
| 1007 | 120 | V0 | 8,900 | 3,063 | 10.25 / 22.64 / 29.85 | 0.20 / 0.42 / 0.79 | 0 / 0.11 / 0.58 | 0.43 / 0.83 / 1.60 | same |
| 1007 | 120 | V1 | 19,709 | 8,533 | 11.92 / 18.77 / 23.33 | 0.08 / 0.32 / 0.74 | 0 / 0.20 / 0.65 | 0.20 / 0.91 / 2.03 | 0 / 0.18 / 1.13 |
| 1007 | 200 | V0 | 3,473 | 795 | 13.25 / 15.60 / 32.95 | 0.20 / 0.36 / 0.80 | 0 / 0.31 / 0.63 | 0.37 / 0.76 / 1.70 | same |
| 1007 | 200 | V1 | 30,161 | 13,552 | 17.40 / 25.46 / 29.35 | 0.13 / 0.54 / 0.98 | 0 / 0.26 / 0.56 | 0.22 / 0.96 / 2.06 | 0 / 0.29 / 0.94 |
| 1007 | 300 | V0 | 2,431 | 836 | 21.08 / 25.43 / 27.15 | 0.40 / 0.55 / 0.77 | 0 / 0.09 / 0.38 | 0.61 / 0.95 / 1.55 | same |
| 1007 | 300 | V1 | 27,968 | 12,387 | 19.32 / 28.14 / 31.96 | 0.18 / 0.65 / 0.99 | 0 / 0.13 / 0.56 | 0.30 / 0.95 / 1.94 | 0 / 0.15 / 0.69 |

Limb tips with s = 0 in V1:

| seed | 120 | 200 | 300 |
|---|---|---|---|
| 1001 | 2 of 10 | 8 of 24 | 28 of 41 |
| 1007 | 14 of 43 | 49 of 102 | 50 of 87 |

## T3: apexes shed by the balance, cycles age−19 to age

| seed | age | var | limb | bough | branch | shoot | twig | short | total |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 40 | V0 / V1 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| 1001 | 80 | V0 / V1 | 0 / 0 | 0 / 0 | 217 / 0 | 31 / 0 | 1,214 / 5 | 4,705 / 45 | 6,167 / 50 |
| 1001 | 120 | V0 / V1 | 0 / 1 | 14 / 20 | 3,207 / 1,991 | 407 / 304 | 18,448 / 11,474 | 99,576 / 55,763 | 121,652 / 69,553 |
| 1001 | 200 | V0 / V1 | 6 / 2 | 61 / 54 | 4,187 / 6,991 | 355 / 1,367 | 21,895 / 49,357 | 131,125 / 322,464 | 157,629 / 380,235 |
| 1001 | 300 | V0 / V1 | 0 / 11 | 1 / 171 | 56 / 12,298 | 5 / 2,016 | 312 / 81,000 | 1,858 / 520,411 | 2,232 / 615,907 |
| 1007 | 40 | V0 / V1 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| 1007 | 80 | V0 / V1 | 2 / 0 | 17 / 0 | 2,942 / 39 | 396 / 3 | 15,369 / 256 | 62,465 / 1,056 | 81,191 / 1,354 |
| 1007 | 120 | V0 / V1 | 5 / 10 | 56 / 60 | 8,463 / 6,193 | 1,027 / 1,109 | 48,344 / 38,639 | 269,211 / 226,911 | 327,106 / 272,922 |
| 1007 | 200 | V0 / V1 | 3 / 25 | 54 / 406 | 3,335 / 33,605 | 302 / 4,868 | 17,537 / 206,663 | 101,720 / 1,219,292 | 122,951 / 1,464,859 |
| 1007 | 300 | V0 / V1 | 1 / 64 | 8 / 652 | 2,026 / 38,405 | 358 / 6,093 | 12,581 / 239,209 | 76,423 / 1,491,948 | 91,397 / 1,776,371 |

The trunk, fork, leader and sprig shed 0 in every run.

## T4: longest path from the root collar, m

| seed | age | V0: living tip (settled) | V0: living tip (grower) | V0: leafed ph | V1: living tip (settled) | V1: living tip (grower) | V1: leafed ph |
|---|---|---|---|---|---|---|---|
| 1001 | 40 | 11.12 | 10.94 | 11.12 | 10.75 | 10.59 | 10.75 |
| 1001 | 80 | 20.38 | 20.10 | 20.38 | 18.25 | 18.06 | 18.25 |
| 1001 | 120 | 31.21 | 30.96 | 31.21 | 24.74 | 24.62 | 24.74 |
| 1001 | 200 | 38.15 | 37.98 | 38.15 | 30.25 | 30.16 | 30.25 |
| 1001 | 300 | 8.43 | 30.72 | 8.43 | 32.20 | 32.16 | 32.20 |
| 1007 | 40 | 11.27 | 11.06 | 11.27 | 10.90 | 10.71 | 10.90 |
| 1007 | 80 | 23.36 | 22.89 | 23.36 | 19.53 | 19.72 | 19.81 |
| 1007 | 120 | 32.36 | 32.16 | 32.36 | 25.96 | 25.89 | 25.99 |
| 1007 | 200 | 35.39 | 35.16 | 35.39 | 30.27 | 30.24 | 30.27 |
| 1007 | 300 | 28.26 | 34.81 | 28.26 | 32.77 | 32.75 | 32.77 |

"Settled" is summed along the settled tree's segments. "Grower" is the largest `path_end` over the grower's live set in the last cycle.

## Against the paper expectations

- **E1:**
  - At 40, V1's height is 10.34 m against V0's 10.65 m on seed 1001 (−2.9%) and 10.08 m against 10.38 m on seed 1007 (−2.9%).
  - V1's tallest at any age is 27.91 m (seed 1007, 300) and 27.74 m on seed 1001, against 34.8 m.
  - V0 reached 32.55 m.
- **E2:**
  - V1's leaf at 300 over leaf at 120 is 290.8 / 508.7 = 0.57 on seed 1001 and 761.3 / 926.4 = 0.82 on seed 1007.
  - V0's is 0.9 / 738.6 = 0.001 and 219.0 / 1,113.7 = 0.20.
- **E3:**
  - V1's leaf as a share of V0's is 0.87 at 40 and 0.65 at 80 on seed 1001.
  - On seed 1007 it is 0.84 at 40 and 0.69 at 80.
- **E4:**
  - V1's phytomers per m² of leaf are 11,250 at 200 and 19,513 at 300 on seed 1001, and 11,400 and 19,856 on seed 1007.
  - V0's at 120 are 4,198 (seed 1001) and 4,115 (seed 1007).

## Unknowns and oddities (recorded, not judged)

- **V0 at seed 1001, age 300.** The grower's live set holds tips with a path up to 30.72 m: 94 structural tips, one bough at 19.26 m. The settled tree's longest path to a living tip is 8.43 m. So the settle drops the subtrees that carry those living tips. The same happens on seed 1007 at 300 (34.81 m against 28.26 m). Why is not traced.
- **The earlier no-dominance census** (PHASE3 background) gives leaf 757, 206 and 4 m² at 120, 200 and 300. This V0 gives 738.6, 199.3 and 0.9 m², close but not equal. The cause (commit or reader differences) is not traced. Age 240 was not grown here.
- **V1's tree has more phytomers than V0's at every age from 80 on, and more living apexes.** V1 over V0 phytomers:
  - seed 1001: 1.10 at 80, 1.32 at 120, 6.1 at 200, 67 at 300;
  - seed 1007: 1.41 at 80, 2.14 at 120, 6.3 at 200, 9.3 at 300.

  In V1 the total peaks at 6.05 M (seed 1001, 200) and 15.1 M (seed 1007, 300). Smaller sizes mean shorter internodes, which means more tips with s > 0 surviving the balance (T3 at 80 shows 50 against 6,167 sheds). Cost follows: seed 1007 at 300 took 480 s and 9.0 GiB against 99 s and 2.0 GiB.
- **The test list above comes from reading only.** No test was run against V1.
- **V2 is open:** which row(s) stand for "this axis goes on" (see the table at the top).
