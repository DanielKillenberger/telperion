# fn-361: what the row moved (R1 to R3)

Builder (Opus), 13 Nov 2026. The row `/space/ages/6/form/dominance = 0.5` with its comment, and the host's amendment (spec, Boundaries): `crates/telperion-space/src/oak.rs` mirrors it, the limb's form `dominance: 0.5` and the bough's, which is built `..limb.form`, `dominance: 0.0`. No other row inherits the limb's form (`..limb.form` appears once).

## Generated

- `src/browser/presets.generated.ts`: one line, the oak's limb `"dominance": 0.5`, by `TELPERION_WRITE_REFERENCE=1 node scripts/build-wasm.mjs`. `docs/parameters.md` and `parameters.generated.ts` unchanged (`parameter_reference` passes).

## Pins that moved, each with its tree

| pin | tree | was | is |
|---|---|---|---|
| `catalogue_identity::every_preset_is_the_family_it_was` | every preset's family together | 16_645_187_557_545_691_998 | 17_200_827_603_434_776_716 |
| `catalogue_identity::every_walk_is_the_walk_it_was` | every walk together | 16_404_614_338_522_828_711 | 14_407_390_098_334_827_746 |
| `catalogue_identity::every_override_reads_and_writes_as_it_did` | every override together | 9_054_662_260_167_386_261 | 3_108_744_754_618_608_219 |
| `pipeline::cycle_cost_tests` PINNED, row 2 | oak at 16, seeds 1, 1234, 5678, finest and coarsest | (14944671683796001899, 475426537269313102) | (12276229172433221013, 16700210903344882314) |
| `telperion-space` `grow::growing_tests::a_growing_phytomer_is_read_as_it_was` | `oak()` at 20, seed 1 | 10364051823450299320 | 6244642234542684519 |
| same | `oak()` at 20, seed 7 | 10572993160139470314 | 5226649160101445206 |

## The other six presets unchanged (measured)

The catalogue digests hash every preset together, so they are read through `cycle_cost_tests`' per-preset rows: with the row, the run read every pair but the oak's equal to its pin (catalogue order ordinary, oak, spruce, beech, birch, palm, Telperion, Laurelin): ordinary (13986120611573680668 both ends), spruce (12665486350558508414, 8154765959244887056), beech (13924919106548380920 both), birch (2054489571488262802, 9663260454690218178), palm (4799502948138528708 both), Telperion (3392261673279751331 both), Laurelin (16456616023369512333 both). Each is a digest of that preset grown through the cycle at both ends of the dial. The beech and spruce pins in `growing_tests` are also unchanged.

## Narrow suites, green with the row (measured)

- `telperion-heavy 12 --threads 8 -- cargo test --profile ci -p telperion-core --no-fail-fast --test preset_values --test space_presets --test catalogue_identity --test capability --test walk_ends --test walk_law --test walk_midpoint --test walk_flying --test space_walks --test parameter_reference --test walk_sizes`: rc 0.
- `... cargo test --profile ci -p telperion-core --no-fail-fast --lib -- pipeline::cycle_cost_tests family::tests suite::drop suite::sag suite::strands`: rc 0.
- `... cargo test --profile ci -p telperion-space --no-fail-fast`: rc 0.
