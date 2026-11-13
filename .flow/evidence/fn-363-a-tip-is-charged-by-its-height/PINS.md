# fn-363 R2 to R4: tests, pins and bounds (builder, Opus, 13 Nov 2026)

Each suite ran once through `telperion-heavy 12 --threads 8` in this worktree's own target. The raw logs are in the ignored `raw/` folder.

## R2: red, then green

- `1d555c155` commits the two tests on their own: `a_tip_is_charged_the_square_of_its_height_whatever_its_path` and `a_tip_below_the_collar_or_without_a_height_is_charged_nothing` (`grow/cycle/balance_tests.rs`). On that commit they are red because they do not compile: `passes::charged`, the charge of a tip by its height, does not exist yet. On the base the charge is reached only through `Grower::balance_one`'s closure, and no hand-built tree reaches that closure.
- `bb9e120e4` adds the change, and all six `balance_tests` pass (`raw/r2-balance.log`). The arithmetic test is renamed `a_charge_is_the_square_of_the_height`. Its assertions are unchanged.
- No second form exists. Both engines charge through `balance_one`, which calls `charged` (R1).

## R3: year-independence

`every_year_is_the_horizons_prefix`, run by name with `--ignored --exact`, passes in 17.3 s (`raw/r3-every-year.log`). The four others pass in the space crate's full run (`raw/r4-space.log`): `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`, `the_tree_at_a_year_is_the_horizons_prefix`, `a_year_is_a_state_not_a_horizon`, `a_growth_goes_on_past_its_first_request`.

## R4: pins that moved, each with its tree

Each pin was reread by its own test. Every value below is the one its test measured on this branch, and each is the value the pin held before fn-361's row (`8f079717c`). Only removing the row moved them. The charge moved none of them, because none of these trees has shed by the balance at its pinned age.

| pin | tree | was (fn-361) | is (measured; = before fn-361) |
|---|---|---|---|
| `catalogue_identity::every_preset_is_the_family_it_was` | every preset's family together | 17_200_827_603_434_776_716 | 16_645_187_557_545_691_998 |
| `catalogue_identity::every_walk_is_the_walk_it_was` | every walk together | 14_407_390_098_334_827_746 | 16_404_614_338_522_828_711 |
| `catalogue_identity::every_override_reads_and_writes_as_it_did` | every override together | 3_108_744_754_618_608_219 | 9_054_662_260_167_386_261 |
| `pipeline::cycle_cost_tests` PINNED, row 2 | oak at 16, seeds 1, 1234, 5678, finest and coarsest | (12276229172433221013, 16700210903344882314) | (14944671683796001899, 475426537269313102) |
| `grow::growing_tests::a_growing_phytomer_is_read_as_it_was` | `oak()` at 20, seed 1, unlaid, in light | 6244642234542684519 | 10364051823450299320 |
| same | `oak()` at 20, seed 7 | 5226649160101445206 | 10572993160139470314 |

`raw/r4-repin-*.log` holds the reruns: all green.

**Unchanged to the bit (measured):** in `cycle_cost_tests`, every row but the oak's (ordinary, spruce, beech, birch, palm, Telperion, Laurelin, each at 16 at both ends of the dial). In `growing_tests`, the beech at 30 and the spruce at 30, seeds 1 and 7. These pins are at young ages, where the balance sheds nothing yet, so they cannot show the charge. R5 measures the presets at 40 and 80.

## A bound that moved: the inverse-height walk's stretch

`tests/walk/mod.rs`, `inverse_heights()`: the walk unit is now the walk tree's tallest living tip above the root collar, where it was the longest path to a living tip (R1). The value range (0 to 0.1 a metre), the steps and the bound are unchanged. `light_changes_the_tree_by_degree` passes with the new unit (461.6 s, `raw/r4-space.log`). The old and new stretch values on the walk tree grown at none are **unknown**. The read is written (an uncommitted scratch test, `crates/telperion-space/tests/zz_stretch.rs` in `.worktrees/fn-363-before`) but was not run before the yield watcher's pause.

## A test that fails: the browser's ceiling (not loosened)

`pipeline::space::ceiling_tests::every_shipped_preset_keeps_under_the_wasm_ceiling` fails on this branch. It passes on the top (`fn-351-phase-top`, `raw/ceiling-before.log`). Phytomers kept at each preset's own seed 42 and age, against the 4,000,000 ceiling:

| preset | top | this branch |
|---|---|---|
| Telperion, 82 | 3,614,135 (margin 9.6%) | **6,120,872 (over by 53.0%)** |
| Laurelin, 80 | 3,382,695 (margin 15.4%) | **5,422,804 (over by 35.6%)** |
| oak, 40 | 349,024 | 355,333 (the row removed) |
| ordinary, spruce, beech, birch, palm, all at 40 | 159,293; 1,143,268; 935,092; 347,917; 480 | the same, to the count |

Under the new charge, Telperion and Laurelin keep 60 to 70% more phytomers, and the browser would refuse both. The test is left as it is. This goes to the host with R5's numbers (MEASURES.md).

## Guards

`generation_limit_guard` and `compile_size` pass (`raw/guard-*.log`). The generation-limit inventory did not ask for anything.
