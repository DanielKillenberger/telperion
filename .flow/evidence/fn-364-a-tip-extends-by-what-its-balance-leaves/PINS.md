# fn-364 R2 to R4: tests, pins and bounds (builder, Opus, 14 Nov 2026)

Every suite ran through `telperion-heavy`. The space crate and the guards ran in `.worktrees/fn-364-suites` at `6c7093f96`, its own target. Those commits change the space crate no further. The core suites reran on the commits named. Raw logs are in the ignored `raw/` folder.

## R2: red, then green

- **The share and the lay** (`grow/cycle/surplus_tests.rs`, five tests). `fe3593951` commits them alone. They are red because they do not compile: the share, its record and the settled stretch do not exist. `f4ce1ded4` adds the change, and all five pass (408 s, a PAUSE inside).
  - The hand-built tip lays its unit at 1 − (h/H)²/L of its length, with its leaf whole.
  - A tip past its limit lays zero-length internodes and keeps its nodes and leaves.
  - Without a potential height, the oak is fn-363's `fine::digest` to the bit. The pin was taken on `49ec1f379`.
  - The settle lays every length growth laid, every node within 0.1 mm (9.5e-6 m on the base).
- **Stations under a stretch** (`pipeline/space/stretch_tests.rs`, the host's design of 14 Nov). The first version (`d35f985b0`) was red because it did not compile. On `f6c00cc49`, three variants on the oak at 12, seed 7 (every phytomer at 0.5 and relaid; each axis's last two at 0; every third at 0):
  - The leaf-bearing wood, own and handed, equals the tree laid whole's to 1e-12 m.
  - No node's own kind moves. Keys two nodes share are left out.
  - Stations are 1,795, 1,805 and 1,796 against 1,794. None is fewer, and each stays within one twig internode's stations per handed run (bound `per × handed runs`; 54, 1,846 and 639 handed runs) and within 2%.
  - At 0.5 the spacing is 0.50 of the whole. Handed stations stand at one point, several to a segment.
- **R8** (`supernatural/height_tests.rs`). `47c13470b` is red by assertion ("0.025 against 0.025"). `be4357b7e` is green, as is `supernatural::` (17 passed), with `the_two_trees_with_the_group_off_are_the_beech` among them.

## R3

- `every_year_is_the_horizons_prefix`, run by name with `--ignored --exact`, passes in 17.3 s. The four others pass in the space crate's full run: `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`, `the_tree_at_a_year_is_the_horizons_prefix`, `a_year_is_a_state_not_a_horizon`, `a_growth_goes_on_past_its_first_request`.
- The preset oak at 80, seed 1001, grown at once equals the oak paused at 40 and grown on: `tree_digest` `b6f1d607a86ea1c6` both ways. At 40, direct and paused are both `980c7b3fe04dffc4`.

## R4: pins that moved, each with its tree

"Text only" means the pinned tree has no phytomer whose stretch is not 1 (counted on this branch). A tree with every stretch at 1 lays and settles as fn-363's to the bit (R2's no-potential-height test). The digest moves only because `tree_digest` hashes the `Debug` text, which now names the stretch. "Tree moved" means stretched phytomers are in it (their count given).

| pin | tree | was (fn-363) | is | moved |
|---|---|---|---|---|
| `grow::growing_tests::a_growing_phytomer_is_read_as_it_was` | oak 20, seed 1 (whole-engine sketch) | 10364051823450299320 | 1764865460666631849 | text only (0 stretched) |
| same | oak 20, seed 7 | 10572993160139470314 | 15961503032430280732 | text only |
| same | beech 30, seed 1 | 2408497720773702210 | 10230782299803655335 | text only |
| same | beech 30, seed 7 | 11778063828277882196 | 14322777518951312446 | text only |
| same | spruce 30, seed 1 | 10160230377130830586 | 16217216416435493051 | text only |
| same | spruce 30, seed 7 | 13617150108993612047 | 14584903475260592403 | text only |
| `pipeline::cycle_cost_tests` PINNED (age 16, seed 7, finest / coarsest) | ordinary | (13986120611573680668, same) | (4262522910082062681, same) | text only (0) |
| same | oak | (14944671683796001899, 475426537269313102) | (13115723846314570422, 722241139789091176) | tree moved (39,251 / 3,627) |
| same | spruce | (12665486350558508414, 8154765959244887056) | (7598300905195022402, 12629625233296773812) | tree moved (85,237 / 4,659) |
| same | beech | (13924919106548380920, same) | (12081055634519087481, same) | tree moved (24,149) |
| same | birch | (2054489571488262802, 9663260454690218178) | (3196652887491806787, 3427627273596663911) | tree moved (48,108 / 9,760) |
| same | palm | (4799502948138528708, same) | (1911908266268488713, same) | text only (0) |
| same | Telperion | (3392261673279751331, same) | (9858385408055298977, same) | tree moved (23,140), R8 in |
| same | Laurelin | (16456616023369512333, same) | (4713123428946290432, same) | tree moved (22,905), R8 in |

Each was reread by its own test after the repin (`raw/` repin log). `catalogue_identity` passes unchanged: it hashes the catalogue's text, and no row changed. The two guards (`generation_limit_guard`, `compile_size`) pass, and the generation-limit inventory asked for nothing.

**Compactness:** `an_axis_and_its_draws_stay_compact` passes unchanged (`Axis` 232 B, `Draws` 144 B). The settled phytomer stays 136 B and a growth unit's record 8 B (R1).

## A test that fails: a promise for the host

`telperion-space/tests/light.rs`, `light_no_bud_reads_leaves_every_passed_species_to_the_bit`: "beech seed 1: Light { extinction: 0.5, sky: 0.0 } changed the tree". The test zeroes every light response a bud has (`shade_hazard`, `shade_size`, `balance_hazard`, `shade_short`) and holds that the tree is then the same under any light. It keeps `inverse_height`. The surplus share reads the tip's light whatever the hazard: a tip charged against its potential height is laid shorter in less light. So under this rule, a species with a potential height has buds that read light even when every other response is zero. Whether the share should read light where the balance's hazard is zero, or the test's promise should name the potential height as a light response, is a question about the form of s, so the test is left as it is and goes to the host.

## The browser's ceiling test (R8): fails, not loosened

`pipeline::space::ceiling_tests::every_shipped_preset_keeps_under_the_wasm_ceiling` on `8e22c8a17` (`raw/r8-ceiling.log`). Phytomers kept at seed 42 and each preset's age, against the 4,000,000 ceiling:

| preset | top (fn-363's PINS.md) | fn-363 | this, R8 |
|---|---|---|---|
| Telperion, 82 | 3,614,135 | 6,120,872 | **9,066,716 (over by 127%)** |
| Laurelin, 80 | 3,382,695 | 5,422,804 | **8,227,011 (over by 106%)** |
| oak, 40 | 349,024 | 355,333 | 348,790 |
| ordinary, spruce, beech, birch, palm, 40 | 159,293; 1,143,268; 935,092; 347,917; 480 | the same | 159,293; 1,142,638; 917,858; 316,396; 480 |

The test fails on Telperion and Laurelin, as it failed on fn-363's branch. Its ceiling is untouched.
