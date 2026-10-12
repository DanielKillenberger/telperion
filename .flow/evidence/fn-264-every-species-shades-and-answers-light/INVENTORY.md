# fn-264 R1: the rows the lattice and the light response read (worker, 2026-10-12)

Checked against the code at `c1aef3f38` (fn-256 task 12, carrying fn-255's cycle and the device executor). Values are from `crates/telperion-core/presets/*.values`; a row a file does not state is the neutral of `telperion_space::chain::passed()`. Laurelin and Telperion carry `telperion_space::beech()` (`tests/space_presets.rs`), so their rows are the beech's.

## The site's light: environment, not species

| Row | Meaning | Consumer (CPU cycle / device) | Today |
|---|---|---|---|
| `/space/light/extinction` | Beer-Lambert k | `light/field.rs` `sweep` (skips at k = 0, line 237); device `build_light.rs:60`, `light_device.rs:94` | oak 0.5, ordinary 0.16667 (the centroid's mean of 0.5, 0, 0), every other preset unstated: 0 |
| `/space/light/sky` | the sky's radiance shape | `Light::directions` (`light.rs`) | 0.5 (neutral) on every preset |

The walk interpolates both (`blend.rs:161-162`); COUNCIL.md says environment is not interpolated by a walk.

## The species' rows

| Row (wire) | Field | Consumer (CPU cycle / device) | Dormant where |
|---|---|---|---|
| `leafArea` | `leaf_area` | the deposit: each phytomer laid this cycle lends scale x size x leaf_area at its tip (`grow/draw.rs:253`); device `capture_layout.rs:109,310`; also `couple.rs:141`, `light.rs:117` (frozen-tree tools) | extinction 0 |
| `shadeHazard` | `shade_hazard` (phi) | survival: viability^(light^-phi) (`grow/advance.rs:72`, `contract::decide::shaded`); device `capture_units.rs:140` | viability 1, or no leaf shades |
| `shadeSize` | `shade_size` (psi) | a unit's size by its vigour per presence, PA normalised (`grow/cycle/passes.rs:102`); device `capture_hierarchy.rs:156` | no leaf shades |
| `apicalControl` | `apical_control` (lambda) | the vigour split at ordered attachments (`passes.rs:105`); device `capture_hierarchy.rs:155` | shade_size 0, or no leaf shades |
| `upkeep` | `upkeep` | the carbon balance's wood cost (`passes.rs:35`) | balance_hazard 0 |
| `balanceHazard` | `balance_hazard` | shedding by remembered balance (`passes.rs:61`, `contract/shed.rs:52`) | 0 |
| `tolerance` | `tolerance` | the balance below which shedding starts (`passes.rs:62`, `contract/shed.rs:52`) | balance_hazard 0 |
| `leafGirth` | `leaf_girth` (chi) | a pipe by its leaves' light against the tree's mean (`girth.rs:161-277`, `girth/disused.rs:158-249`); device `capture_settled.rs:246-253` | no leaf shades |
| `retained` | `retained` | a shed branch's pipe kept in its bearer (`girth/disused.rs:295`); reads no light, listed as the response's neighbour | 0 |
| `viability` | `viability` | the base shade_hazard raises (`advance.rs:72`) | 1 |

## Per preset

| Preset | leafArea | shadeHazard | shadeSize | apicalControl | upkeep | balanceHazard | tolerance | leafGirth | retained |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| norway-spruce | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| silver-birch | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| european-beech | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| date-palm | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| laurelin (beech) | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| telperion (beech) | 0 | 0 | 0 | 0.5 | 0 | 0 | 0 | 0 | 0 |
| oregon-white-oak | 0.00181 | 1.0 | 1.5 (PA 2-4, 8), 0.5 (6-7), 1.0 (9, 12-14) | 0.45 | 0.35 | 2.0 | 0 | 1.0 | 0 |
| ordinary (centroid) | 0.000167 | 0.333 | 0.5 | 0.483 | 0.00605 | 0.0486 | 0 | 0.333 | 0.167 |

- **Viability** (the base shade hazard raises; 1 where unstated): spruce PA 8 0.998, 9 0.998, 10 0.99, 11 0.99, 12 0.9; birch 6 0.998, 7 0.995, 8 0.97, 9 0.92, 14 0.92; beech and the Two Trees 6-8 0.995, 9 0.95, 12 0.95, 14 0.9; palm 1 throughout; oak 7 0.999, 8 0.95, 9 0.98, 12 0.93, 13 0.98, 14 0.9; ordinary 6 0.998, 7 0.997, 8 0.981, 9 0.976, 10 0.973, 11 0.957, 12 0.927, 13 0.933, 14 0.9.
- The oak's rows are on PAs 2, 3, 4, 6, 7, 8, 9, 12, 13, 14; its viability is 1 on PAs 2-4 and 6, so shade_hazard is dormant there. The ordinary's are on all 15 PAs.
- **Provenance of the oak's rows** (`telperion_space::oak`, `oak.rs:83-105`): `leaf_area` 18.1 cm² a node is sourced (one leaf a node, Go Botany; the mean area of 616 *Q. robur* leaves, Visakorpi et al. 2020, Table 1). phi, psi, lambda, upkeep, balance_hazard and chi are the oak's own values from fn-195 round 5 (host decision 29), not literature. The preset is named Oregon white oak (*Q. garryana*) but carries the *Q. robur* engine species.
- **The oak's 0.5 extinction** was "cited from memory" (fn-197 STEP3.md: spherical leaf-angle distribution, Ross 1981, Campbell & Norman 1998) and never checked against a source; R1a checks it.
- **The ordinary's rows** are written by `examples/centroid.rs` from the beech, oak and spruce: any change to those three's rows is a rerun of that example, never a hand edit.
- `Species::reads_light` still gates the closed form and the coarse path (`closed_form.rs:106`, `fine/coarse.rs:75,83`); COUNCIL.md lists it for deletion once replaced. Not this spec's work.
