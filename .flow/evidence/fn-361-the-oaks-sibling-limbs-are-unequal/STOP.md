# fn-361 task 1: stopped at R3, 13 Nov 2026 (builder, Opus)

The row cannot pass the narrow suites without a code change, and the spec and brief forbid one. The row is reverted; its diff is kept in `row.patch` (the values row with its comment, and the one line `scripts/build-wasm.mjs` regenerated in `src/browser/presets.generated.ts` under `TELPERION_WRITE_REFERENCE=1`).

## What failed, with the row applied (measured)

`telperion-heavy 12 --threads 8 -- cargo test --profile ci -p telperion-core --no-fail-fast --test preset_values --test space_presets --test catalogue_identity --test capability --test walk_ends --test walk_law --test walk_midpoint --test walk_flying --test space_walks --test parameter_reference`

- `space_presets::every_engine_preset_carries_the_engines_species` fails for `OregonWhiteOak`. It asserts the preset's `/space` species equals `telperion_space::oak()` (`crates/telperion-space/src/oak.rs`). The two species differ in one field: `dominance: 0.5` (preset) against `0.0` (`oak()`). The birch's 0.2 is mirrored in `crates/telperion-space/src/birch.rs:314`; the oak's would need the same, under `crates/*/src`.
- `catalogue_identity` (expected repins, not made): `every_preset_is_the_family_it_was` 16645187557545691998 -> 17200827603434776716; `every_override_reads_and_writes_as_it_did` 9054662260167386261 -> 3108744754618608219; `every_walk_is_the_walk_it_was` 16404614338522828711 -> 14407390098334827746. These digests cover every preset together, so they cannot by themselves show that only the oak moved.
- Passed: `preset_values` 5, `capability` 10, `parameter_reference` 2, `space_walks` 10, `walk_ends` 1, `walk_law` 2, `walk_midpoint`, and `space_presets`' other 4. `walk_flying` has 1 ignored test.
- Not run: the lib pin tables (`pipeline::cycle_cost_tests` PINNED, `family::tests`, `suite::{drop,sag,strands}`) and `walk_sizes`.

## The stop check's base, drawn (measured)

The oak at 40 and 80 without the row, on `b13c0d1d0`, seeds 1001 to 1012, no stills: `scoreboard/fn361-norow/measures.jsonl` under fn-352's evidence (`run.sh fn361-norow panel` with `PRESETS="" OAK_AGES="40 80" OAK_EXTRA="" OAK_SHOTS=none`). Medians:

| tree | height m | trunk 1.3 m, m | leaf m2 |
|---|---|---|---|
| oak 40 | 10.65 | 0.329 | 187.3 |
| oak 80 | 19.83 | 0.944 | 1,434.6 |

The trees with the row were not drawn, so the 25% check was not made.

## Second stop, 13 Nov 2026 (after the host's amendment)

The host allowed one mirroring line, `dominance: 0.5` in the limb's `Form` in `crates/telperion-space/src/oak.rs`. With it, `space_presets::every_engine_preset_carries_the_engines_species` still fails, now at the bough: the bough's form is built `..limb.form` (`oak.rs`, the `let bough` state, `primary: 0.004`, `bend_length: 1.5`), so the bough inherits 0.5 while the preset's bough (`/space/ages/7/`) has none, 0.0. Equality needs a second line, `dominance: 0.0` in the bough's `Form`, beyond the one line allowed, so the run stopped again.

With the row and that one line (measured, same narrow command plus `--test walk_sizes`, and `--lib -- pipeline::cycle_cost_tests family::tests suite::drop suite::sag suite::strands`):

- `catalogue_identity`: the same three new digests as above.
- `pipeline::cycle_cost_tests::every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` fails at the oak's row alone, catalogue order ordinary, oak, spruce, beech, birch, palm, Telperion, Laurelin: the oak's pair (finest, coarsest) moves from (14944671683796001899, 475426537269313102) to (12276229172433221013, 16700210903344882314); the other seven pairs equal their pins. The cycle grows each preset from its `/space` rows, so this reads the preset, not `oak()`, and the bough leak does not enter it.
- `cargo test --profile ci -p telperion-space --no-fail-fast`: one failure, `grow::growing_tests::a_growing_phytomer_is_read_as_it_was`, "oak seed 1": 10364051823450299320 pinned, 10796496750175493632 read. This one grows `oak()`, so it carries the bough's leaked 0.5 and would read differently again with the bough line. Every other space suite passed (lib 132 passed, 4 ignored).
- The row, the generated line and the one mirroring line are kept in `row-and-mirror.patch`; all three are reverted, so the crates and `src` are as at the base.
- Passed: `family::tests`, `suite::drop`, `suite::sag`, `suite::strands` (22 lib tests passed, 7 ignored, in the filtered run), `walk_sizes` 1 (1 ignored), and the suites listed above.
