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

---

# Round 2 (13 Nov 2026): V3, year-independence, the settle trace, V4 to V6

## Broken, killed or cut

- **No grow panicked, and every finished run exited 0.** The space invariants (`telperion_space::invariants`), now run on every settled tree, failed 0 times.
- **V6 is partial.** Seed 1007 at 120 was frozen by the owner's PAUSE (16:50) and cancelled on your instruction. Seed 1007 at 200 and 300 were not run. V6 has seed 1001 at all five ages and seed 1007 at 40 and 80.
- **Pauses.** The desk paused the V3 build for 27 minutes (14:38 to 15:05) and V4 to V6 for 65 minutes (15:44 to 16:49). Rows whose run spanned a PAUSE are marked "not timed": V5 at seed 1001 age 300, and V6 at seed 1001 age 200.
- **Test literals.** The V3 patch had to add `stretch: 1.0` to five test literals of `Phytomer` (`geometry/lay_tests.rs`, `geometry/tests.rs`, `girth/tests.rs` ×2, `shed/tests.rs`) so the lib tests compile. This is plumbing only; no assertion changed.
- **Clean up.** The worktree is clean (`git status` empty, `d9228bdd8`). The patches, logs, binaries and tables are in this folder.

## V3: the surplus share shortens internodes only

### What was built (`v3.patch`, on reader + V0)

- **The rule.** `allot_one` computes the same s that V1 used and stores it as `Pencil::stretch`. The allotment's `size` is left as `sizes` gave it.
- **Growth.**
  - At each unit, `advance.rs` and `stop.rs` push the stretch into a new per-unit record, `Draws::stretch`, beside `Draws::shade`. `Reads::stretch` reads it like `size`.
  - `draw_bent` lays each phytomer with `scale = bare × size × stretch` (in f32) and builds the pull's `total` from the same lengths. So `path_end`, which the balance reads, is the shortened path.
  - `p.size` stays the allotment's size. The growing leaf (`light::leaf_area(scale, size, area)`) therefore uses the unstretched size, and so do the light deposit and `girth::lay`'s sapwood demand.
- **The settle.**
  - `presence::assign_one` copies the stretch onto a new settled field, `Phytomer::stretch: f64`.
  - `Node::stretch()` is a default method: 1 for `Growing`, the field for `Phytomer`.
  - Every settled reader of length multiplies by it: `Layer::step` (internode length, tropism, run), `geometry::reaches` (straightening reach), and `stand.rs` (`length_of`, `length_to`, `node_x`, `zone_node`, `span_in`).
  - Settled `scale` keeps the size, so leaf (`leaf_area × scale`) is unshortened.
  - `fine::canonical` hashes the stretch too.
- **`retire.rs`** lets go of `stretch` with `shade`.

### What a product build would need

- **The settled tree cannot hold a length different from its leaf without a new field.** The settled `scale` serves both, so V3 adds `Phytomer::stretch`.
- **Bytes per record:**
  - settled `Phytomer`: +8 B per phytomer (one f64);
  - `Draws`: +24 B per axis (a Vec header), plus 8 B per growth unit on the heap wherever a rough layout is grown;
  - `Pencil`: +8 B per axis;
  - `Growing`: unchanged, because the stretch is folded into `scale` only for the moment of the step.
  - `an_axis_and_its_draws_stay_compact` pins `Draws`, so it would likely fail (not run).
- **Functions that change:**
  - `allot_one`;
  - `Reads::stretch` (new);
  - `advance.rs` and `stop.rs` (the push);
  - `Draws::stretch` (new);
  - `presence::assign_one`;
  - the retire let-go;
  - `draw_bent`;
  - `Node` (a new method);
  - `Layer::step`;
  - `geometry::reaches`;
  - five functions in `stand.rs`;
  - `fine::canonical`.
- **Consumers that read length from `scale`:**
  - `telperion-core/src/pipeline/space.rs:373` computes each internode's piece as `internode * p.scale * internode_factor(...)` for the drawing's join splitting. It would not follow the shortened length. It also cannot read the field, which is `pub(crate)`, so a product build needs a public accessor and this line changed.
  - Node positions come from the laid tips, so they do follow.
- **What silently follows the shortened length, and what does not:**
  - Girth does not: sections come from the growing leaf, which keeps the size.
  - The growth-time `stand.rs` on `Growing` axes reads the bare scale. It already excluded `size` before this change and still excludes the stretch.
  - Sag levers were not checked: unknown.
  - Light deposits sit at the shortened tips with unshortened area.
- **Length-0 internodes:**
  - Settled phytomers with scale > 0 and length 0 reach 842,770 at seed 1007 age 120, of which 183,917 are leafed.
  - The space invariants pass 0 failures.
  - The pipeline's conversion, expansion and wood mesh (`executor::grow` → `expansion()` → `wood()`, no drawing) ran on seed 1007 at 80, which has 514,905 such phytomers: 2,412,470 nodes and a wood mesh of 283 chunks, 176,093 runs, no error.
  - Nothing clamped and nothing failed.

### Year-independence on V3 (measured, no test edited)

- The five tests in `grow/cycle/paused/tests.rs` were run with V0 and V3 applied (`cargo test --profile ci -p telperion-space --lib`, through the queue):
  - `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`: ok;
  - `the_tree_at_a_year_is_the_horizons_prefix`: ok;
  - `a_year_is_a_state_not_a_horizon`: ok;
  - `a_growth_goes_on_past_its_first_request`: ok;
  - `every_year_is_the_horizons_prefix`: ignored by default ("slow"); run by name with `--ignored`, ok in 17 s.
- Their species include `crate::oak()`: inverse height 1/34.8, light extinction 0.5, 24 years. So the rule is exercised.
- **Runner check** (seed 1001, the preset): a tree grown at once to 80 and one paused at 40, settled there and grown on to 80 have equal `tree_digest` (`476114bd6a93687c`). At 40, the direct grow and the paused settle are also equal. `tree_digest` is the Debug form of the tree, so it includes `stretch`.

### V3 tables

The full tables are in `tables-all.md`.

T1 (V3):

| seed | age | height m | spread x/y m | r 1.3 m (m) | leaf m² | leafed ph | phytomers | living apexes | ph/m² leaf | peak GiB | s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1001 | 40 | 10.36 | 6.8/8.2 | 0.129 | 110.2 | 71,219 | 316,379 | 24,312 | 2,872 | 0.2 | 1 |
| 1001 | 80 | 16.38 | 18.0/13.5 | 0.327 | 435.3 | 329,673 | 1,671,527 | 115,738 | 3,840 | 1.0 | 7 |
| 1001 | 120 | 22.12 | 19.4/18.3 | 0.478 | 510.3 | 457,056 | 2,477,813 | 150,920 | 4,856 | 1.5 | 21 |
| 1001 | 200 | 23.51 | 19.4/20.5 | 0.573 | 160.4 | 201,632 | 1,164,348 | 62,568 | 7,259 | 1.1 | 38 |
| 1001 | 300 | 20.41 | 9.6/9.8 | 0.603 | 35.2 | 78,420 | 458,156 | 22,961 | 13,014 | 1.1 | 47 |
| 1007 | 40 | 10.10 | 8.0/7.3 | 0.164 | 174.3 | 106,398 | 460,000 | 39,282 | 2,639 | 0.3 | 1 |
| 1007 | 80 | 17.18 | 21.8/20.6 | 0.435 | 751.7 | 550,958 | 2,873,211 | 209,031 | 3,822 | 1.8 | 13 |
| 1007 | 120 | 21.77 | 23.5/24.4 | 0.607 | 746.1 | 671,448 | 3,764,981 | 246,739 | 5,046 | 2.5 | 37 |
| 1007 | 200 | 19.32 | 19.7/20.3 | 0.752 | 384.7 | 530,468 | 3,128,382 | 185,091 | 8,132 | 2.3 | 75 |
| 1007 | 300 | 18.93 | 14.6/20.1 | 0.844 | 198.6 | 382,243 | 2,202,663 | 128,364 | 11,090 | 1.8 | 113 |

**T2** (V3, all structural tips, median; "length factor" is size × stretch):

| seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| 1001: tips | 870 | 3,718 | 4,339 | 1,543 | 477 |
| 1001: s = 0 | 0 | 504 | 1,077 | 317 | 55 |
| 1001: s | 0.80 | 0.40 | 0.20 | 0.41 | 0.47 |
| 1001: length factor | 0.64 | 0.33 | 0.16 | 0.41 | 0.56 |
| 1007: tips | 1,734 | 7,894 | 8,337 | 5,345 | 3,482 |
| 1007: s = 0 | 0 | 1,910 | 2,674 | 834 | 663 |
| 1007: s | 0.76 | 0.25 | 0.13 | 0.43 | 0.28 |

**T3** (V3, total balance sheds in the last 20 cycles):

| seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| 1001 | 0 | 8,134 | 71,076 | 24,362 | 5,099 |
| 1007 | 0 | 74,275 | 164,598 | 26,613 | 36,293 |

**T4** (V3, longest path to a living tip, settled):

| seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| 1001 | 10.78 | 18.41 | 24.86 | 27.06 | 25.91 |
| 1007 | 10.91 | 21.30 | 26.07 | 25.95 | 26.34 |

**E1 to E4 for V3:**
- **E1:** height at 40 is −2.7% against V0 on both seeds (10.36 vs 10.65, 10.10 vs 10.38). The maximum at any age is 23.51 m.
- **E2:** leaf at 300 over leaf at 120 is 0.069 (seed 1001) and 0.266 (seed 1007).
- **E3:** leaf against V0's is 0.97 and 0.87 at 40 and 80 on seed 1001, and 0.96 and 0.83 on seed 1007.
- **E4:** phytomers per m² of leaf at 200 and 300 are 7,259 and 13,014 (seed 1001), and 8,132 and 11,090 (seed 1007). V0's at 120 are 4,198 and 4,115.

## The settle trace: V0, seed 1001, at 300

**The settle drops none of them.** It draws them at a fifth of their size.

- **Nothing is removed.** All 94 living limb, bough and branch tips in the grower's live set, and all 4,334 living axes, survive `shed_past` (`TR|tips|live structural 94|kept by the settle 94`). No ancestor of any of them meets `lost`, the parent-lost or `fade <= 0.0` test.
- **What shrinks them** is the applied fade in `crates/telperion-space/src/shed.rs`, `shed_past`:

  ```rust
  let own = bearer.min(q(fade[i]));
  axis.vigour *= real(own / bearer);
  ```

  The fade comes from `fades` → `faded` → `contract::presence::fade(delay = q(shedding + 1), birth, (lived, alive), age)` = `clamp(delay − ((age − birth) − lived), alive, 1)`. Here `lived` is the subtree's presence-weighted living time carried up through `carry(before, weight, gap, l) = before + weight·(gap + l)`, with weight = vigour × the bearing node's presence. `alive` is the presence of the subtree's most present living apex, relative to the axis.
- **Where it lands for each traced tip:**
  - The bough tip (grower path 19.31 m, relay born 300, light 0.59) sits under limb lateral axis 2 (row 6, born 13). There `lived` = 222 of 287 cycles, so idle = 65 against a delay of 5 + 1. The fade clamps to alive = 0.2219.
  - The three longest branch tips (grower paths 30.73, 24.44 and 24.39 m, light 0.94 to 0.96) sit under limb lateral axis 4 (row 6, born 14). There `lived` = 274 of 286, so idle = 12 against 6. The fade clamps to alive = 0.1320.
  - Every other ancestor on these walks has fade 1, except one branch run on tip 1's walk at 0.217.
  - The applied fade multiplies `vigour`, so `geometry::scale`'s base scale carries it down the whole limb subtree. Settled lengths along tip 1's lineage are 0.13 times the grower's laid lengths. For example, axis 6 is 1.736 m in the grower's layout and 0.229 m settled.
- **The grower never sees this fade.** The grower's pencils keep base scale 0.789 and 0.948 on axes 4 and 2, against settled vigour 0.104 and 0.222. Its layout, light read and balance run on the unfaded subtree. In the last 20 cycles the traced tips' presence (base × running × kept) is 0.08 to 0.21, kept = 1, never closed.
- **The leaf gap at 300:** the grower's own leaf deposit (`OO|meta deposit`) is 4.08 m², against 0.88 m² settled, a factor of 4.6 ≈ 1/0.22. At 120 the two are 757.5 and 738.6 m².
- **This resolves the earlier unknown.** The earlier census's 757, 206 and 4 m² match the grower's deposit at 120, 200 and 300 (757.5, 205.7, 4.08); this report's figures are the settled tree's (738.6, 199.3, 0.88).
- **How long before 300 the settled tree already shows it** (fresh grows of V0, seed 1001; applied fade of the living axes and of the living structural tips):

  | age | living structural tips | of them faded below 1 / below 0.5 | median fade (all living axes) | longest living path: settled / grower m | leaf: settled / grower deposit m² |
  |---|---|---|---|---|---|
  | 120 | 5,225 | 495 / 327 | 1.00 | 31.21 / 30.97 | 738.6 / 757.5 |
  | 200 | 1,672 | 116 / 70 | 1.00 | 38.15 / 37.98 | 199.3 / 205.7 |
  | 240 | 374 | 76 / 39 | 1.00 | 35.47 / 35.35 | 38.2 / 41.3 |
  | 260 | 208 | 179 / 134 | 0.22 | 33.84 / 34.87 | 8.3 / 12.4 |
  | 280 | 132 | 124 / 101 | 0.22 | 33.76 / 34.42 | 4.0 / 7.5 |
  | 300 | 94 | 94 / 94 | 0.22 | 8.43 / 30.72 | 0.9 / 4.1 |

  The longest settled path holds until one unfaded tip remains. At 300 every living axis is under a fade of 0.22 or less, and the drawn height falls to 7.69 m.

## V4 to V6: the charge's form

**How each variant was built.** Each changes only `charge`'s inputs or form, on the V0 base, with no surplus rule (`v4.patch`, `v5.patch`, `v6.patch`):
- **V4** reads height for path, exponent 2.
- **V5** reads path, exponent 1.
- **V6** reads height, exponent 1.

**Where V4's height comes from.** It is at hand in `balance_one`, with no new record:
- It is the z of the tip's last laid node (`Growing::tip[2]`, written by last cycle's rough layout), or the axis base where no node is laid.
- The collar is axis 0's base z, and height is never below 0.
- It is read through `self.axes`, which the `own` closure already borrows.

**T2's s** is 1 − charge/earn under each variant's own charge.

### T1: added columns, all variants that ran

**Height** (m):

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 10.65 | 18.00 | 27.10 | 32.55 | 7.69 |
| 1001 | V1 | 10.34 | 16.24 | 22.01 | 26.17 | 27.74 |
| 1001 | V3 | 10.36 | 16.38 | 22.12 | 23.51 | 20.41 |
| 1001 | V4 | 10.65 | 18.01 | 27.42 | 36.84 | 36.05 |
| 1007 | V0 | 10.38 | 18.62 | 26.94 | 30.92 | 22.17 |
| 1007 | V1 | 10.08 | 16.79 | 21.62 | 26.22 | 27.91 |
| 1007 | V3 | 10.10 | 17.18 | 21.77 | 19.32 | 18.93 |
| 1007 | V4 | 10.38 | 18.82 | 28.44 | 37.12 | 38.79 |

**Crown spread, x / y** (m):

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 7.2 / 8.2 | 20.2 / 15.8 | 25.2 / 23.0 | 30.2 / 29.1 | 1.9 / 0.8 |
| 1001 | V1 | 6.8 / 8.0 | 16.6 / 13.2 | 18.8 / 17.9 | 21.7 / 21.4 | 22.9 / 20.9 |
| 1001 | V3 | 6.8 / 8.2 | 18.0 / 13.5 | 19.4 / 18.3 | 19.4 / 20.5 | 9.6 / 9.8 |
| 1001 | V4 | 7.2 / 8.2 | 20.3 / 15.8 | 26.4 / 24.7 | 35.5 / 37.3 | 38.7 / 30.3 |
| 1007 | V0 | 8.0 / 7.4 | 24.2 / 21.9 | 32.4 / 31.1 | 29.6 / 25.7 | 18.2 / 16.2 |
| 1007 | V1 | 7.8 / 7.1 | 20.1 / 18.4 | 22.9 / 23.9 | 25.8 / 29.5 | 31.2 / 26.5 |
| 1007 | V3 | 8.0 / 7.3 | 21.8 / 20.6 | 23.5 / 24.4 | 19.7 / 20.3 | 14.6 / 20.1 |
| 1007 | V4 | 8.0 / 7.4 | 25.4 / 23.7 | 37.1 / 35.3 | 52.5 / 53.7 | 48.7 / 51.7 |

**Trunk diameter at 1.3 m** (m, twice the radius):

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 0.262 | 0.690 | 1.074 | 1.332 | 1.346 |
| 1001 | V1 | 0.244 | 0.578 | 0.882 | 1.342 | 1.656 |
| 1001 | V3 | 0.258 | 0.654 | 0.956 | 1.146 | 1.206 |
| 1001 | V4 | 0.262 | 0.696 | 1.154 | 1.804 | 2.196 |
| 1007 | V0 | 0.334 | 0.936 | 1.384 | 1.668 | 1.812 |
| 1007 | V1 | 0.310 | 0.784 | 1.290 | 1.990 | 2.484 |
| 1007 | V3 | 0.328 | 0.870 | 1.214 | 1.504 | 1.688 |
| 1007 | V4 | 0.334 | 0.984 | 1.646 | 2.590 | 3.126 |

**Mean spread over diameter:**

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 29.3 | 26.0 | 22.4 | 22.2 | 1.0 |
| 1001 | V1 | 30.2 | 25.8 | 20.8 | 16.0 | 13.2 |
| 1001 | V3 | 29.2 | 24.1 | 19.7 | 17.4 | 8.0 |
| 1001 | V4 | 29.3 | 26.0 | 22.2 | 20.2 | 15.7 |
| 1007 | V0 | 23.1 | 24.7 | 22.9 | 16.6 | 9.5 |
| 1007 | V1 | 24.0 | 24.5 | 18.2 | 13.9 | 11.6 |
| 1007 | V3 | 23.2 | 24.3 | 19.7 | 13.3 | 10.3 |
| 1007 | V4 | 23.1 | 24.9 | 22.0 | 20.5 | 16.1 |

**Height over diameter:**

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 40.8 | 26.1 | 25.2 | 24.4 | 5.7 |
| 1001 | V1 | 42.2 | 28.1 | 24.9 | 19.5 | 16.7 |
| 1001 | V3 | 40.3 | 25.0 | 23.1 | 20.5 | 16.9 |
| 1001 | V4 | 40.8 | 25.9 | 23.8 | 20.4 | 16.4 |
| 1007 | V0 | 31.0 | 19.9 | 19.5 | 18.5 | 12.2 |
| 1007 | V1 | 32.5 | 21.4 | 16.8 | 13.2 | 11.2 |
| 1007 | V3 | 30.7 | 19.8 | 17.9 | 12.8 | 11.2 |
| 1007 | V4 | 31.0 | 19.1 | 17.3 | 14.3 | 12.4 |

**Leaf per leafed phytomer** (cm²):

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 15.35 | 13.39 | 12.39 | 11.03 | 0.62 |
| 1001 | V1 | 12.88 | 8.19 | 8.20 | 7.75 | 6.15 |
| 1001 | V3 | 15.47 | 13.20 | 11.16 | 7.96 | 4.49 |
| 1001 | V4 | 15.35 | 13.45 | 12.57 | 12.53 | 11.62 |
| 1007 | V0 | 16.25 | 13.95 | 12.64 | 7.69 | 6.97 |
| 1007 | V1 | 12.96 | 7.17 | 7.92 | 7.13 | 4.16 |
| 1007 | V3 | 16.38 | 13.64 | 11.11 | 7.25 | 5.20 |
| 1007 | V4 | 16.25 | 14.15 | 12.92 | 12.70 | 9.48 |

**Dead-standing phytomers as a share of all** (apex ended, nothing living below):

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 0.478 | 0.491 | 0.529 | 0.550 | 0.530 |
| 1001 | V1 | 0.482 | 0.501 | 0.517 | 0.515 | 0.523 |
| 1001 | V3 | 0.475 | 0.474 | 0.497 | 0.496 | 0.526 |
| 1001 | V4 | 0.478 | 0.484 | 0.507 | 0.528 | 0.537 |
| 1007 | V0 | 0.442 | 0.469 | 0.515 | 0.501 | 0.538 |
| 1007 | V1 | 0.448 | 0.464 | 0.508 | 0.508 | 0.517 |
| 1007 | V3 | 0.438 | 0.446 | 0.451 | 0.442 | 0.472 |
| 1007 | V4 | 0.442 | 0.437 | 0.480 | 0.509 | 0.504 |

**Leaf over crown ground area,** taken as the ellipse of the two spreads (π/4 · x · y), not a hull:

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V0 | 2.47 | 2.00 | 1.62 | 0.29 | 0.74 |
| 1001 | V1 | 2.31 | 1.90 | 1.92 | 1.48 | 0.77 |
| 1001 | V3 | 2.51 | 2.28 | 1.83 | 0.51 | 0.48 |
| 1001 | V4 | 2.47 | 2.04 | 1.88 | 1.00 | 0.64 |
| 1007 | V0 | 3.87 | 2.18 | 1.41 | 0.53 | 0.94 |
| 1007 | V1 | 3.52 | 2.15 | 2.16 | 2.09 | 1.17 |
| 1007 | V3 | 3.82 | 2.14 | 1.66 | 1.22 | 0.86 |
| 1007 | V4 | 3.87 | 2.25 | 1.92 | 0.99 | 0.56 |

**V4's leaf, phytomers and cost** (the other three variants are in the round-1 T1 and the V3 table above):

| seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| 1001: leaf m² | 113.5 | 513.8 | 967.8 | 1,041.4 | 589.9 |
| 1001: phytomers | 326,166 | 1,885,934 | 3,984,042 | 4,311,571 | 2,625,032 |
| 1001: s, peak GiB | 1, 0.2 | 7, 1.1 | 25, 2.2 | 75, 2.6 | 121, 2.0 |
| 1007: leaf m² | 181.0 | 1,062.7 | 1,974.5 | 2,198.1 | 1,111.1 |
| 1007: phytomers | 476,669 | 3,796,021 | 8,025,988 | 9,068,912 | 6,324,353 |
| 1007: s, peak GiB | 1, 0.3 | 16, 2.3 | 55, 4.7 | 171, 5.7 | 287, 4.4 |

V4 at 40 is identical to V0 on both seeds: the balance has shed nothing by then in either.

### V5 and V6

**V5, all of T1:**

| seed | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| 1001: height m | 10.64 | 17.55 | 25.59 | 27.73 | 7.69 |
| 1001: leaf m² | 102.4 | 301.5 | 334.8 | 48.3 | 0.0 |
| 1007: height m | 10.35 | 17.49 | 24.98 | 29.94 | 7.34 |
| 1007: leaf m² | 151.6 | 474.1 | 505.3 | 0.0 | 0.0 |

V5 already sheds at 40: 4,460 balance sheds on seed 1001 and 12,657 on seed 1007. At seed 1001 age 300 it has 4 axes, 172 phytomers and no living apex.

**V6, partial.** Seed 1001: height 10.64, 17.69, 26.05, 36.85 and 32.12 m; leaf 105.0, 355.3, 493.4, 356.8 and 109.2 m². Seed 1007: 40 and 80 only (height 10.35 and 17.66 m; leaf 158.4 and 590.5 m²).

### T2 to T4 for V4 to V6

**T2:** all structural tips, median s, with the count of tips at s = 0:

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 0.85 (0) | 0.59 (164) | 0.32 (1,188) | 0.11 (2,638) | 0 (2,625 of 4,831) |
| 1001 | V5 | 0.38 (171) | 0.16 (620) | 0 | 0 | none living |
| 1001 | V6 | 0.43 (151) | 0.25 (541) | 0.09 (1,204) | 0 (1,543) | 0.02 (298 of 728) |
| 1007 | V4 | 0.83 (0) | 0.41 (1,197) | 0.23 (3,696) | 0.07 (6,711) | 0.19 (2,735 of 10,486) |
| 1007 | V5 | 0.36 (323) | 0.10 (1,365) | 0 (2,710) | 0.06 (66 of 182) | 0.22 (0 of 11) |
| 1007 | V6 | 0.45 (289) | 0.21 (1,266) | cancelled | not run | not run |

**T3:** total balance sheds in the last 20 cycles:

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 0 | 852 | 68,302 | 292,088 | 314,626 |
| 1001 | V5 | 4,460 | 67,398 | 154,713 | 30,720 | 0 |
| 1001 | V6 | 3,507 | 52,534 | 138,429 | 217,764 | 35,687 |
| 1007 | V4 | 0 | 19,238 | 229,484 | 787,984 | 329,036 |
| 1007 | V5 | 12,657 | 181,035 | 376,011 | 14,178 | 0 |

**T4:** longest path to a living tip (settled) / tallest living tip / widest spread, all in m:

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 11.12 / 10.65 / 8.2 | 20.39 / 18.01 / 20.3 | 31.47 / 27.42 / 26.4 | 48.81 / 36.84 / 37.3 | 58.64 / 36.05 / 38.7 |
| 1001 | V5 | 11.11 / 10.64 / 8.0 | 19.89 / 17.55 / 17.3 | 29.50 / 25.59 / 22.9 | 38.25 / 27.73 / 28.7 | none living / – / 0.2 |
| 1001 | V6 | 11.11 / 10.64 / 8.1 | 20.05 / 17.69 / 18.1 | 29.92 / 26.05 / 23.3 | 47.07 / 36.85 / 36.6 | 57.61 / 32.12 / 32.0 |
| 1007 | V4 | 11.27 / 10.38 / 8.0 | 24.60 / 18.82 / 25.4 | 40.66 / 28.44 / 37.1 | 54.09 / 37.12 / 53.7 | 64.89 / 38.79 / 51.7 |
| 1007 | V5 | 11.24 / 10.35 / 7.4 | 20.20 / 17.49 / 19.4 | 29.66 / 24.98 / 29.2 | 16.07 / 6.43 / 19.8 | 5.47 / 5.18 / 0.4 |

V5 at seed 1007 age 200 shows the settle-fade split again: the grower's path is 35.59 m against 16.07 m settled.

### F1 to F5 (numbers placed, no judgement)

- **F1, height under 34.8 m at every age:**
  - V0: maximum 32.55 m;
  - V1: 27.91 m;
  - V3: 23.51 m;
  - V4: 36.84 and 36.05 m on seed 1001 (200, 300), 37.12 and 38.79 m on seed 1007;
  - V5: maximum 29.94 m;
  - V6: 36.85 m at seed 1001 age 200.
- **F2, leaf at 300 over leaf at 120** (seed 1001, seed 1007):

  | V0 | V1 | V3 | V4 | V5 | V6 |
  |---|---|---|---|---|---|
  | 0.001, 0.197 | 0.572, 0.822 | 0.069, 0.266 | 0.610, 0.563 | 0.000, 0.000 | 0.221, not run |

- **F3, leaf over V0's** at 40 / 80 / 120:

  | seed | V1 | V3 | V4 | V5 | V6 |
  |---|---|---|---|---|---|
  | 1001 | 0.87 / 0.65 / 0.69 | 0.97 / 0.87 / 0.69 | 1.00 / 1.02 / 1.31 | 0.90 / 0.60 / 0.45 | 0.92 / 0.71 / 0.67 |
  | 1007 | 0.84 / 0.69 / 0.83 | 0.96 / 0.83 / 0.67 | 1.00 / 1.17 / 1.77 | 0.84 / 0.52 / 0.45 | 0.88 / 0.65 / – |

- **F4, phytomers per m² of leaf over V0's at 120,** at 40 / 80 / 120 / 200 / 300:

  | seed | V0 | V4 | V1 | V3 | V5 | V6 |
  |---|---|---|---|---|---|---|
  | 1001 | 0.68 / 0.88 / 1.00 / 1.18 / 22.9 | 0.68 / 0.87 / 0.98 / 0.99 / 1.06 | up to 4.65 | up to 3.10 | 1.03 at 200, no leaf at 300 | 0.69 / 0.84 / 0.89 / 0.87 / 1.19 |
  | 1007 | 0.64 / 0.87 / 1.00 / 1.74 / 1.80 | 0.64 / 0.87 / 0.99 / 1.00 / 1.38 | up to 4.83 | up to 2.70 | 1,286 at 200, 2,349 at 300 | – |

- **F5, mean spread over diameter** at 120 / 200 / 300:

  | seed | V0 | V1 | V3 | V4 | V5 | V6 |
  |---|---|---|---|---|---|---|
  | 1001 | 22.4 / 22.3 / 1.0 | 20.8 / 16.0 / 13.2 | 19.7 / 17.4 / 8.0 | 22.2 / 20.2 / 15.7 | 27.2 / 26.7 / 0.14 | 24.6 / 27.7 / 18.7 |
  | 1007 | 22.9 / 16.6 / 9.5 | 18.2 / 13.9 / 11.6 | 19.7 / 13.3 / 10.3 | 22.0 / 20.5 / 16.1 | 28.0 / 16.5 / 0.39 | – |

## Unknowns (round 2)

- **Sag levers under V3:** whether they read a phytomer's length from `scale` was not checked.
- **V3's `an_axis_and_its_draws_stay_compact`** and the other pins were not run.
- **V4's height** is last cycle's laid tip, the same lag as `path_end`. Whether the grower's rough layout and the settled tree stand the tip at the same height was not compared.
- **The settle's applied fade** (trace above) also acts on V1, V3 and V4 trees. In every variant, the settled-against-grower gaps it causes are inside the tables' settled figures.

---

# Round 3 (13 Nov 2026): V7, V8 on V4's height charge; seed 1012

## Broken, killed or cut

- **V7, seed 1007 at 300, was killed at the 24 GB cap** (exit 143, 18:14). It finished its last cycle's allotment, printing T2 with 83,171 structural tips of which 36,417 have s = 0, and was killed during the settle. Its peak is at least about 24 GB; no T1 row.
- **Cut by the host:** V7 at seed 1012 age 300 (cancelled mid-run at 19:33) and V8 on seed 1012 (not run). The V4-alone test check was skipped (the fn-363 build runs those tests).
- **Freezes:** the desk paused the queue 13 times between 17:21 and 19:33. Rows whose run spanned a PAUSE say "not timed" in the seconds column.
- **No panics; invariants pass.** No grow panicked, and the space invariants failed 0 times on every settled tree.
- **Clean up:** the worktree is clean (`git status` empty, `d9228bdd8`) and the queue holds none of these jobs.

## What was run

- **Base: reader + V0 + V4** (`v4.patch`) **+ `extras.patch`.** The extras print the count of living axes with an applied settle fade below 1 and their median fade.
  - **V0** = reader + V0 + V4's census part (`v4-census.patch`, for the tallest living tip) + extras, without V4's charge.
  - **V4** was rebuilt with the extras (binary `heldrun-v4x`). Its settled leaf equals round 2's on every compared row.
- **V7** (`v7.patch`): in `allot_one`, each living apex's allotment size is multiplied by s = max(0, 1 − charge(1, height, h⁻¹)/light). The charge and height are V4's.
- **V8** (`v3-plumbing.patch` + `v8.patch`): V3's plumbing, with `stretch` set to the same height s. Leaf and the rest keep the allotment's size.
- **One grow per age.** Seeds 1001 and 1007 for all four variants; seed 1012 for V0, V4 and V7 up to 200.

## T1

- **dbh** is twice the radius at 1.3 m, shown beside White's open-grown range.
- **Leaf** is the settled tree's leaf / the grower's own last leaf deposit.
- **Fades** are the living axes whose applied settle fade is below 1, with their median fade.

| seed | age | variant | tallest living tip m | mean spread m | dbh m | White dbh m | leaf settled / deposit m² | leaf per leafed ph cm² | ph per m² leaf | spread/dbh | height/dbh | dead-standing share | living axes faded <1 (median) | peak GiB | s |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1001 | 40 | V0 | 10.65 | 7.7 | 0.261 | 0.28–0.40 | 113.5 / 113.9 | 15.35 | 2872 | 29.3 | 40.8 | 0.478 | 245 of 26454 (0.534) | 0.2 | 1 |
| 1001 | 40 | V4 | 10.65 | 7.7 | 0.261 | 0.28–0.40 | 113.5 / 113.9 | 15.35 | 2872 | 29.3 | 40.8 | 0.478 | 245 of 26454 (0.534) | 0.2 | 1 |
| 1001 | 40 | V7 | 10.35 | 7.6 | 0.248 | 0.28–0.40 | 101.9 / 102.3 | 13.42 | 3275 | 30.4 | 41.7 | 0.481 | 250 of 26903 (0.534) | 0.2 | 1 |
| 1001 | 40 | V8 | 10.37 | 7.7 | 0.258 | 0.28–0.40 | 110.7 / 111.1 | 15.47 | 2868 | 29.8 | 40.2 | 0.475 | 246 of 25930 (0.534) | 0.2 | 1 |
| 1001 | 80 | V0 | 18.00 | 18.0 | 0.691 | 0.56–0.79 | 501.4 / 509.2 | 13.39 | 3694 | 26.0 | 26.1 | 0.491 | 11617 of 129311 (0.380) | 1.1 | 7 |
| 1001 | 80 | V4 | 18.01 | 18.0 | 0.695 | 0.56–0.79 | 513.8 / 521.7 | 13.45 | 3671 | 26.0 | 25.9 | 0.484 | 11747 of 133607 (0.384) | 1.1 | 7 |
| 1001 | 80 | V7 | 16.53 | 16.5 | 0.609 | 0.56–0.79 | 372.6 / 379.3 | 9.31 | 5355 | 27.2 | 27.2 | 0.495 | 11351 of 138214 (0.424) | 1.1 | not timed |
| 1001 | 80 | V8 | 16.64 | 17.7 | 0.669 | 0.56–0.79 | 465.8 / 473.8 | 13.46 | 3742 | 26.5 | 24.9 | 0.471 | 11883 of 125511 (0.384) | 1.1 | 7 |
| 1001 | 120 | V0 | 27.10 | 24.1 | 1.074 | 0.83–1.09 | 738.6 / 757.5 | 12.39 | 4198 | 22.4 | 25.2 | 0.529 | 25841 of 189672 (0.375) | 1.7 | 23 |
| 1001 | 120 | V4 | 27.42 | 25.6 | 1.153 | 0.83–1.09 | 967.8 / 989.9 | 12.57 | 4117 | 22.2 | 23.8 | 0.507 | 32146 of 254633 (0.407) | 2.2 | 25 |
| 1001 | 120 | V7 | 22.99 | 20.7 | 0.959 | 0.83–1.09 | 625.0 / 641.8 | 8.25 | 7101 | 21.6 | 24.0 | 0.504 | 35778 of 284854 (0.421) | 2.3 | 27 |
| 1001 | 120 | V8 | 23.39 | 23.0 | 1.057 | 0.83–1.09 | 741.4 / 765.3 | 11.91 | 4484 | 21.8 | 22.1 | 0.484 | 35968 of 216316 (0.374) | 2.0 | 23 |
| 1001 | 200 | V0 | 32.55 | 29.7 | 1.333 | 1.21–1.52 | 199.3 / 205.7 | 11.03 | 4962 | 22.2 | 24.4 | 0.550 | 6479 of 55499 (0.298) | 1.3 | 47 |
| 1001 | 200 | V4 | 36.84 | 36.4 | 1.804 | 1.21–1.52 | 1041.4 / 1064.6 | 12.53 | 4140 | 20.2 | 20.4 | 0.528 | 28823 of 263978 (0.330) | 2.6 | 74 |
| 1001 | 200 | V7 | 29.90 | 26.8 | 1.599 | 1.21–1.52 | 1028.0 / 1055.1 | 7.86 | 8939 | 16.8 | 18.7 | 0.508 | 68704 of 586122 (0.351) | 5.2 | 105 |
| 1001 | 200 | V8 | 26.80 | 25.9 | 1.474 | 1.21–1.52 | 548.5 / 572.6 | 10.71 | 5155 | 17.6 | 18.2 | 0.494 | 37432 of 167051 (0.207) | 1.8 | 59 |
| 1001 | 300 | V0 | 7.17 | 1.3 | 1.347 | 1.56–1.92 | 0.9 / 4.1 | 0.62 | 96186 | 1.0 | 5.7 | 0.530 | 4334 of 4334 (0.222) | 1.3 | not timed |
| 1001 | 300 | V4 | 36.05 | 34.5 | 2.196 | 1.56–1.92 | 589.9 / 603.1 | 11.62 | 4450 | 15.7 | 16.4 | 0.537 | 16896 of 160989 (0.312) | 2.0 | 121 |
| 1001 | 300 | V7 | 31.71 | 28.9 | 2.213 | 1.56–1.92 | 1073.3 / 1102.5 | 5.78 | 12106 | 13.1 | 14.3 | 0.516 | 199863 of 847618 (0.000) | 7.3 | 264 |
| 1001 | 300 | V8 | 26.06 | 24.8 | 1.711 | 1.56–1.92 | 252.9 / 270.0 | 8.92 | 6261 | 14.5 | 15.3 | 0.507 | 27020 of 87133 (0.156) | 1.5 | 87 |
| 1007 | 40 | V0 | 10.38 | 7.7 | 0.335 | 0.28–0.40 | 181.0 / 181.2 | 16.25 | 2634 | 23.1 | 31.0 | 0.442 | 265 of 43999 (0.364) | 0.3 | 1 |
| 1007 | 40 | V4 | 10.38 | 7.7 | 0.335 | 0.28–0.40 | 181.0 / 181.2 | 16.25 | 2634 | 23.1 | 31.0 | 0.442 | 265 of 43999 (0.364) | 0.3 | 1 |
| 1007 | 40 | V7 | 10.11 | 7.6 | 0.317 | 0.28–0.40 | 160.0 / 160.2 | 13.81 | 3088 | 24.1 | 31.9 | 0.446 | 134 of 45115 (0.392) | 0.3 | 1 |
| 1007 | 40 | V8 | 10.13 | 7.8 | 0.330 | 0.28–0.40 | 175.8 / 176.0 | 16.41 | 2630 | 23.6 | 30.7 | 0.438 | 259 of 43006 (0.458) | 0.3 | 1 |
| 1007 | 80 | V0 | 18.62 | 23.1 | 0.935 | 0.56–0.79 | 908.7 / 919.1 | 13.95 | 3596 | 24.7 | 19.9 | 0.469 | 27959 of 241146 (0.344) | 1.9 | 30 |
| 1007 | 80 | V4 | 18.82 | 24.5 | 0.985 | 0.56–0.79 | 1062.7 / 1074.1 | 14.15 | 3572 | 24.9 | 19.1 | 0.437 | 34351 of 294549 (0.356) | 2.2 | 15 |
| 1007 | 80 | V7 | 17.14 | 21.6 | 0.856 | 0.56–0.79 | 768.1 / 777.4 | 8.74 | 5741 | 25.2 | 20.0 | 0.451 | 32493 of 334733 (0.347) | 2.5 | 17 |
| 1007 | 80 | V8 | 17.47 | 24.2 | 0.942 | 0.56–0.79 | 949.9 / 961.5 | 14.04 | 3700 | 25.7 | 18.5 | 0.422 | 37053 of 277783 (0.346) | 2.2 | 15 |
| 1007 | 120 | V0 | 26.94 | 31.7 | 1.383 | 0.83–1.09 | 1113.7 / 1135.3 | 12.64 | 4115 | 22.9 | 19.5 | 0.515 | 43721 of 295913 (0.242) | 2.7 | 45 |
| 1007 | 120 | V4 | 28.44 | 36.2 | 1.646 | 0.83–1.09 | 1974.5 / 2014.2 | 12.92 | 4065 | 22.0 | 17.3 | 0.480 | 88069 of 544332 (0.274) | 4.7 | 56 |
| 1007 | 120 | V7 | 23.66 | 31.7 | 1.400 | 0.83–1.09 | 1394.5 / 1431.6 | 7.39 | 8152 | 22.7 | 16.9 | 0.467 | 113438 of 800040 (0.311) | 6.5 | 71 |
| 1007 | 120 | V8 | 23.61 | 35.5 | 1.494 | 0.83–1.09 | 1473.7 / 1519.9 | 12.00 | 4583 | 23.7 | 15.8 | 0.445 | 101397 of 465154 (0.239) | 4.3 | 53 |
| 1007 | 200 | V0 | 30.92 | 27.7 | 1.669 | 1.21–1.52 | 315.2 / 328.8 | 7.69 | 7163 | 16.6 | 18.5 | 0.501 | 27904 of 132423 (0.235) | 2.0 | not timed |
| 1007 | 200 | V4 | 37.12 | 53.1 | 2.589 | 1.21–1.52 | 2198.1 / 2242.4 | 12.70 | 4126 | 20.5 | 14.3 | 0.509 | 81822 of 578561 (0.259) | 5.6 | 166 |
| 1007 | 200 | V7 | 30.47 | 37.4 | 2.474 | 1.21–1.52 | 2483.9 / 2553.6 | 9.19 | 8625 | 15.1 | 12.3 | 0.492 | 269260 of 1455856 (0.096) | 12.5 | not timed |
| 1007 | 200 | V8 | 26.88 | 36.8 | 2.125 | 1.21–1.52 | 1123.4 / 1191.1 | 9.66 | 6006 | 17.3 | 12.8 | 0.451 | 126376 of 415259 (0.183) | 4.6 | 143 |
| 1007 | 300 | V0 | 22.17 | 17.2 | 1.811 | 1.56–1.92 | 219.0 / 226.9 | 6.97 | 7422 | 9.5 | 12.2 | 0.538 | 13897 of 95809 (0.150) | 2.0 | 103 |
| 1007 | 300 | V4 | 38.79 | 50.2 | 3.126 | 1.56–1.92 | 1111.1 / 1149.5 | 9.48 | 5692 | 16.1 | 12.4 | 0.504 | 65409 of 384053 (0.306) | 4.3 | 284 |
| 1007 | 300 | V7 | killed or cancelled | | | | | | | | | | | | | |
| 1007 | 300 | V8 | 26.11 | 34.2 | 2.534 | 1.56–1.92 | 805.8 / 857.4 | 7.40 | 7825 | 13.5 | 10.3 | 0.449 | 113965 of 388268 (0.249) | 4.2 | 234 |
| 1012 | 40 | V0 | 10.46 | 8.2 | 0.282 | 0.28–0.40 | 125.8 / 127.1 | 13.63 | 3325 | 29.0 | 37.1 | 0.474 | 906 of 33189 (0.411) | 0.2 | 1 |
| 1012 | 40 | V4 | 10.46 | 8.2 | 0.282 | 0.28–0.40 | 125.8 / 127.1 | 13.63 | 3325 | 29.0 | 37.1 | 0.474 | 906 of 33189 (0.411) | 0.2 | 1 |
| 1012 | 40 | V7 | 10.17 | 8.2 | 0.270 | 0.28–0.40 | 114.4 / 115.7 | 12.18 | 3714 | 30.3 | 37.6 | 0.478 | 900 of 33514 (0.414) | 0.2 | 1 |
| 1012 | 80 | V0 | 17.95 | 19.5 | 0.763 | 0.56–0.79 | 608.3 / 615.5 | 13.09 | 3849 | 25.5 | 23.5 | 0.484 | 14921 of 162404 (0.402) | 1.3 | 10 |
| 1012 | 80 | V4 | 17.97 | 19.6 | 0.771 | 0.56–0.79 | 631.8 / 639.1 | 13.05 | 3837 | 25.4 | 23.3 | 0.476 | 14907 of 172095 (0.389) | 1.4 | 10 |
| 1012 | 80 | V7 | 16.41 | 17.8 | 0.673 | 0.56–0.79 | 452.6 / 458.4 | 8.74 | 5654 | 26.5 | 24.4 | 0.486 | 14409 of 178853 (0.406) | 1.4 | 10 |
| 1012 | 120 | V0 | 25.40 | 29.4 | 1.179 | 0.83–1.09 | 886.6 / 905.3 | 12.06 | 4271 | 25.0 | 21.5 | 0.514 | 33145 of 244453 (0.315) | 2.2 | 36 |
| 1012 | 120 | V4 | 25.97 | 30.7 | 1.336 | 0.83–1.09 | 1388.6 / 1414.4 | 12.39 | 4133 | 23.0 | 19.4 | 0.487 | 50471 of 389855 (0.314) | 3.2 | 38 |
| 1012 | 120 | V7 | 22.25 | 26.5 | 1.115 | 0.83–1.09 | 938.5 / 964.6 | 8.20 | 7192 | 23.7 | 20.0 | 0.487 | 54632 of 460053 (0.341) | 3.8 | 44 |
| 1012 | 200 | V0 | 31.68 | 30.4 | 1.511 | 1.21–1.52 | 399.4 / 413.3 | 7.80 | 6865 | 20.1 | 21.0 | 0.504 | 30844 of 171947 (0.173) | 1.7 | not timed |
| 1012 | 200 | V4 | 39.35 | 48.4 | 2.274 | 1.21–1.52 | 1887.2 / 1926.0 | 10.48 | 5073 | 21.3 | 17.3 | 0.499 | 82714 of 628579 (0.281) | 5.7 | 137 |
| 1012 | 200 | V7 | 29.10 | 37.5 | 2.083 | 1.21–1.52 | 2081.9 / 2125.8 | 8.71 | 9046 | 18.0 | 14.0 | 0.499 | 140376 of 1236380 (0.315) | 10.8 | 206 |
| 1012 | 300 | V0 | 23.74 | 18.7 | 1.707 | 1.56–1.92 | 308.9 / 313.2 | 7.22 | 6935 | 10.9 | 13.9 | 0.543 | 13487 of 140968 (0.124) | 1.6 | 94 |
| 1012 | 300 | V4 | 38.24 | 53.9 | 2.903 | 1.56–1.92 | 1370.8 / 1400.5 | 6.75 | 7725 | 18.6 | 13.2 | 0.496 | 140950 of 703956 (0.021) | 6.3 | 288 |
| 1012 | 300 | V7 | killed or cancelled | | | | | | | | | | | | | |

V4's figures equal V0's at 40 on all three seeds. V0's tallest living tip at seed 1001 age 300 (7.17 m) is lower than its top tip of any phytomer (7.69 m).

## T2: all structural tips

The median of s under the height charge, with the count of tips at s = 0. In V0 the charge reads path, so s is not shown for it.

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 0.85 (0 of 870) | 0.59 (164 of 3,910) | 0.32 (1,188 of 6,868) | 0.11 (2,638 of 7,219) | 0 (2,625 of 4,831) |
| 1001 | V7 | 0.86 (0 of 869) | 0.66 (107 of 3,891) | 0.54 (1,447 of 7,744) | 0.38 (4,929 of 15,922) | 0.23 (8,930 of 24,987) |
| 1001 | V8 | 0.84 (0 of 870) | 0.56 (234 of 3,904) | 0.32 (1,032 of 6,281) | 0.35 (728 of 4,392) | 0.43 (114 of 2,014) |
| 1007 | V4 | 0.83 (0 of 1,729) | 0.41 (1,197 of 10,266) | 0.23 (3,696 of 16,891) | 0.07 (6,711 of 17,112) | 0.19 (2,735 of 10,486) |
| 1007 | V7 | 0.85 (0 of 1,728) | 0.56 (346 of 10,790) | 0.46 (4,989 of 24,901) | 0.33 (19,318 of 46,284) | 0.27 (36,417 of 83,171), killed in settle |
| 1007 | V8 | 0.83 (0 of 1,733) | 0.38 (1,262 of 10,327) | 0.28 (3,259 of 15,725) | 0.40 (2,000 of 12,246) | 0.39 (1,126 of 11,401) |
| 1012 | V4 | 0.85 (0 of 1,089) | 0.63 (60 of 5,241) | 0.38 (1,388 of 11,568) | 0.20 (5,719 of 19,478) | 0.13 (7,321 of 21,851) |
| 1012 | V7 | 0.86 (0 of 1,086) | 0.70 (0 of 5,223) | 0.55 (3,116 of 13,862) | 0.22 (16,242 of 38,135) | cancelled |

## T3: balance sheds in the last 20 cycles (all rows)

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 0 | 852 | 68,302 | 292,088 | 314,626 |
| 1001 | V7 | 0 | 0 | 40,703 | 301,294 | 964,715 |
| 1001 | V8 | 0 | 1,633 | 48,391 | 58,839 | 16,838 |
| 1007 | V4 | 0 | 19,238 | 229,484 | 787,984 | 329,036 |
| 1007 | V7 | 0 | 5 | 119,108 | 1,353,809 | 2,837,715 (cycles before the kill) |
| 1007 | V8 | 0 | 19,889 | 140,834 | 158,843 | 37,122 |
| 1012 | V4 | 0 | 596 | 99,134 | 730,492 | 735,834 |
| 1012 | V7 | 0 | 0 | 30,454 | 675,789 | cancelled |

## T4: longest path from the collar to a living tip (settled), m

The tallest living tip is in T1.

| seed | variant | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | V4 | 11.12 | 20.39 | 31.47 | 48.81 | 58.64 |
| 1001 | V7 | 10.76 | 18.61 | 26.09 | 35.71 | 43.21 |
| 1001 | V8 | 10.79 | 18.74 | 26.77 | 37.38 | 38.94 |
| 1007 | V4 | 11.27 | 24.60 | 40.66 | 54.09 | 64.89 |
| 1007 | V7 | 10.94 | 21.48 | 33.89 | 39.34 | killed |
| 1007 | V8 | 10.95 | 23.18 | 37.55 | 39.36 | 47.43 |
| 1012 | V4 | 11.10 | 23.10 | 34.94 | 54.49 | 78.58 |
| 1012 | V7 | 10.75 | 20.86 | 28.96 | 39.23 | cancelled |

## G1 to G7 (numbers placed, no judgement)

- **G1, tallest living tip** at 40 / 80 / 120 / 200 / 300, m:

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 10.65 / 18.00 / 27.10 / 32.55 / 7.17 | 10.65 / 18.01 / 27.42 / 36.84 / 36.05 | 10.35 / 16.53 / 22.99 / 29.90 / 31.71 | 10.37 / 16.64 / 23.39 / 26.80 / 26.06 |
  | 1007 | 10.38 / 18.62 / 26.94 / 30.92 / 22.17 | 10.38 / 18.82 / 28.44 / 37.12 / 38.79 | 10.11 / 17.14 / 23.66 / 30.47 / – | 10.13 / 17.47 / 23.61 / 26.88 / 26.11 |
  | 1012 | 10.46 / 17.95 / 25.40 / 31.68 / 23.74 | 10.46 / 17.97 / 25.97 / 39.35 / 38.24 | 10.17 / 16.41 / 22.25 / 29.10 / – | not run |

- **G2, leaf at 300 over leaf at 120:**

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 0.001 | 0.610 | 1.717 | 0.341 |
  | 1007 | 0.197 | 0.563 | – | 0.547 |
  | 1012 | 0.348 | 0.987 | – | not run |

- **G3, leaf over V0's** at 40 / 80 / 120:

  | seed | V4 | V7 | V8 |
  |---|---|---|---|
  | 1001 | 1.00 / 1.02 / 1.31 | 0.90 / 0.74 / 0.85 | 0.97 / 0.93 / 1.00 |
  | 1007 | 1.00 / 1.17 / 1.77 | 0.88 / 0.85 / 1.25 | 0.97 / 1.05 / 1.32 |
  | 1012 | 1.00 / 1.04 / 1.57 | 0.91 / 0.74 / 1.06 | not run |

- **G4, phytomers per m² of leaf over V0's at 120** (4,198 / 4,115 / 4,271), at 40 / 80 / 120 / 200 / 300:

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 0.68 / 0.88 / 1.00 / 1.18 / 22.91 | 0.68 / 0.87 / 0.98 / 0.99 / 1.06 | 0.78 / 1.28 / 1.69 / 2.13 / 2.88 | 0.68 / 0.89 / 1.07 / 1.23 / 1.49 |
  | 1007 | 0.64 / 0.87 / 1.00 / 1.74 / 1.80 | 0.64 / 0.87 / 0.99 / 1.00 / 1.38 | 0.75 / 1.40 / 1.98 / 2.10 / – | 0.64 / 0.90 / 1.11 / 1.46 / 1.90 |
  | 1012 | 0.78 / 0.90 / 1.00 / 1.61 / 1.62 | 0.78 / 0.90 / 0.97 / 1.19 / 1.81 | 0.87 / 1.32 / 1.68 / 2.12 / – | not run |

- **G5, leaf per leafed phytomer over V0's at 40** (15.35, 16.25, 13.63 cm²), at 40 / 80 / 120 / 200 / 300:

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 1.00 / 0.87 / 0.81 / 0.72 / 0.04 | 1.00 / 0.88 / 0.82 / 0.82 / 0.76 | 0.87 / 0.61 / 0.54 / 0.51 / 0.38 | 1.01 / 0.88 / 0.78 / 0.70 / 0.58 |
  | 1007 | 1.00 / 0.86 / 0.78 / 0.47 / 0.43 | 1.00 / 0.87 / 0.79 / 0.78 / 0.58 | 0.85 / 0.54 / 0.45 / 0.57 / – | 1.01 / 0.86 / 0.74 / 0.59 / 0.46 |
  | 1012 | 1.00 / 0.96 / 0.88 / 0.57 / 0.53 | 1.00 / 0.96 / 0.91 / 0.77 / 0.50 | 0.89 / 0.64 / 0.60 / 0.64 / – | not run |

- **G6, mean spread over diameter** at 120 / 200 / 300:

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 22.4 / 22.2 / 1.0 | 22.2 / 20.2 / 15.7 | 21.6 / 16.8 / 13.1 | 21.8 / 17.6 / 14.5 |
  | 1007 | 22.9 / 16.6 / 9.5 | 22.0 / 20.5 / 16.1 | 22.7 / 15.1 / – | 23.7 / 17.3 / 13.5 |
  | 1012 | 25.0 / 20.1 / 10.9 | 23.0 / 21.3 / 18.6 | 23.7 / 18.0 / – | not run |

- **G7, seconds and peak GiB** at 200 and at 300:

  | seed | V0 | V4 | V7 | V8 |
  |---|---|---|---|---|
  | 1001 | 47 s, 1.3 · not timed, 1.3 | 74 s, 2.6 · 121 s, 2.0 | 105 s, 5.2 · 264 s, 7.3 | 59 s, 1.8 · 87 s, 1.5 |
  | 1007 | not timed, 2.0 · 103 s, 2.0 | 166 s, 5.6 · 284 s, 4.3 | not timed, 12.5 · killed at the 24 GB cap | 143 s, 4.6 · 234 s, 4.2 |
  | 1012 | not timed, 1.7 · 94 s, 1.6 | 137 s, 5.7 · 288 s, 6.3 | 206 s, 10.8 · cancelled | not run |

## Unknowns (round 3)

- V7's memory at seed 1007 age 300 is known only to exceed what the 24 GB cap allowed.
- The V4-alone year-independence and balance-test check was not run. By reading only: `balance_tests.rs` calls `charge(size, quantity, inverse)` directly, and V4 changes only the quantity passed in `balance_one`, not `charge`. So those tests would compile, and their asserts check `charge`'s square form, which V4 keeps. `a_tip_is_charged_the_square_of_its_path` names and documents the argument as a path; its asserts do not read a tree.
