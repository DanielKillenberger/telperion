# FN31: native renderer prerequisite blocker

The required first checkpoint is blocked. All four renderer targets fail
natively, but three failures differ from the brief's missing-geometry
diagnosis. No growth rule, preset, test, pin or production renderer code has
changed. No implementation checkpoint is complete.

## Protocol

Re-anchored on branch `fn-31-growth-rule-sapling-form-thickening-by` at
`ccb44eaf609c15e4df7170385f81b372b0d5f532`. Read the task, fn-31 and fn-30 specs,
fn-11's complete research section, CLAUDE.md, the strategy's growth section,
and fn-30's report and composed-curve script. Flow state and task files remain
unchanged. No agents, bridges, review verdict, push or history rewrite.

Ran all four renderer targets with `--no-fail-fast`. Native GPU tests executed
without skips. Viewed zero images and made no capture. A temporary Rust
example queried the exact capped fixtures and full oak and spruce. Its source
is retained as `fixture_probe.rs`, outside production and test directories.
To reproduce, copy it to `crates/telperion-core/examples/fn31_probe.rs`, run
`cargo run --release -p telperion-core --example fn31_probe`, then remove the
copy. The source prints geometry summaries and never renders an image.

All five requested gate strings run unchanged. The workspace command retains
Cargo's normal fail-fast behavior; the separate renderer command covers all
four targets independently.

## The prerequisite failures

| Target | Native result | Evidence |
|---|---|---|
| `bark_detail` | 2 changed channels; requires more than 100 | Ordinary with `max_nodes=400` stops at age 9 with 349 nodes, 204 structural nodes, 17,920 wood triangles and 241 placements. Root radius is 0.126443 m; the tested ridge scale is 0.16 m. `bark.wgsl:139` returns zero relief when radius is no greater than ridge scale. The capped juvenile does not exercise the intended relief. |
| `bark_distance` | 4x mean 3.139683333333333/255; limit 3.0 | p95 is 9.25/255 against 12. The 2x mean is 1.097725/255 and p95 3.5/255. Full oak has 11,564,840 wood triangles and 1,175,265 placements. This failure measures resolution agreement on populated geometry. |
| `bark_resolution` | Grazing mask has 0 pixels; requires more than 1,000 | Full oak's first surface run has 1,320 indices and spans y=-0.096 to 2.3964474 m. The fixed camera and run-zero mask select no grazing pixels. The other trunk test passes at mean 0.946909/255, p95 3.25/255. The exact cause of zero mask coverage has not been isolated beyond these measurements. |
| `conformance` | Set 6 from Ordinary has 1,440 wood triangles and 0 placements | This confirms missing foliage. The other five tests in the target pass. |

The initial progress hypothesis of a root-only rollback for the 400-node
fixture was disproved by the probe. Its actual stop is age 9 with 349 nodes.
The 20-node fixture separately retains 14 nodes at age 5, with 760 triangles
and no placements; its radius-storage assertion passes.

The user explicitly requires a gate believed wrong to be reported as a
blocker, never edited around. The relief fixture cannot test its stated bark
scale at the retained juvenile radii, and the grazing fixture cannot measure
resolution agreement with an empty mask. Those fixture/gate problems remain
unresolved. The distance failure additionally needs a cause and fix beyond
the missing-geometry diagnosis. No assertion, tolerance, camera, fixture or
shader was changed to obtain a pass.

`logs/renderer-baseline.log` records exit 101 for the focused run.
`logs/fixture-probe.log` records the successful geometry probe.

## The references

Reused fn-30's five local source files without fetching or changing them.
Recomputed SHA-256 values match its report exactly. Absolute source paths and
hashes are in `logs/checksums.log`. No seedling reference was added.

| ID | File | Source | SHA-256 | Bytes |
|---|---|---|---|---:|
| E1 | `ertragstafeln.pdf` | [Bavarian yield-table extracts, Jüttner 1955 oak and Wiedemann 1936/42 spruce](https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf) | `c6c7d6558fe6f8157c2fea3a67e8aaed133902b18446eb32275972fc2a5f9885` | 351,827 |
| G1 | `gould2011.pdf` | [Gould, Harrington and Devine 2011](https://cascadiaprairieoak.org/wp-content/uploads/2014/07/Gould-P.J.-C.A.-Harrington-and-W.D.-Devine.-2011.-Growth-of-Oregon-White-Oak-Quercus-garryana.pdf) | `3bac24bce07da85b47b280a7a78c0e6a220de2e86930e5ddbe56249d63916341` | 605,406 |
| S1 | `silvics-oregon-white-oak.html` | [Stein 1990, Silvics](https://research.fs.usda.gov/silvics/oregon-white-oak) | `21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f` | 103,163 |
| V1 | `pmc2987550.html` | [Vospernik, Monserud and Sterba 2010](https://pmc.ncbi.nlm.nih.gov/articles/PMC2987550/) | `3193a0d83020a90fa21da2e2f49b0bab6bfde3304b04fbaf2de1ce5214bb06ad` | 314,453 |
| U1 | `utd-gtr253.pdf` | [Urban Tree Database report](https://research.fs.usda.gov/download/treesearch/52933.pdf) | `0765570cf3513a8cc91ba07b4abd7702611b5e765b07e93cde67555b4097b1c7` | 10,558,386 |

The unchanged `../fn30/curves.py` SHA-256 is
`9beda29216a153a3ef5a882c1ca16fd938a60e1d457ed0c8912bfcf7e0568dc4`.
Composition and limitations remain those of fn-30. Oak height uses a
stand-grown Quercus robur/petraea proxy; DBH integrates Gould from the chosen
7.2 cm age-30 anchor. Spruce DBH doubles the stand mean diameter. These are
compositions, not direct observed open-grown age curves.

## Ratio and diameter evidence inherited from fn-30

This table is copied from fn-30's report. It contains base measurements,
not post-fix results or new calibration. No R2 acceptance is claimed.

| Species / point | Age y | Reference height m | Reference DBH m | Reference H/DBH | Measured height m | Measured DBH m | Measured H/DBH | Height error % | DBH error % | Provenance at this age |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| oak / young | 26.7 | 8.00 | 0.064 | 125.0 | 7.575 | 0.266618 | 28.4 | -5.3 | +316.6 | E1 II linear extension to age 30; E1 I DBH anchor linear extension |
| oak / middle | 56.1 | 16.00 | 0.112 | 142.8 | 16.603 | 0.580425 | 28.6 | +3.8 | +418.2 | E1 II 50–60 interpolation; G1 open-grown integration from E1 I anchor |
| oak / envelope | 112.0 | 24.00 | 0.236 | 101.8 | 22.605 | 0.789969 | 28.6 | -5.8 | +234.7 | E1 II 110–120 interpolation; G1 integration from E1 I anchor |
| oak / derived-mature | 432 | 39.48 | 0.902 | 43.8 | 23.730 | 0.835595 | 28.4 | -39.9 | -7.4 | E1 II extrapolation beyond 200; G1 integration from E1 I anchor |
| spruce / young | 14.1 | 5.00 | 0.106 | 47.3 | 4.800 | 0.122191 | 39.3 | -4.0 | +15.3 | E1 I linear extension to age 20; E1 I DBH ×2 from V1 |
| spruce / middle | 26.6 | 10.00 | 0.203 | 49.3 | 10.500 | 0.299797 | 35.0 | +5.0 | +47.7 | E1 I 20–30 interpolation; E1 I DBH ×2 from V1 |
| spruce / envelope | 36.9 | 15.00 | 0.285 | 52.6 | 12.900 | 0.369860 | 34.9 | -14.0 | +29.8 | E1 I 30–40 interpolation; E1 I DBH ×2 from V1 |
| spruce / derived-mature | 158 | 40.08 | 1.144 | 35.0 | 15.000 | 0.421595 | 35.6 | -62.6 | -63.1 | E1 I height and DBH extrapolation beyond 120; DBH ×2 from V1 |

## Derived ages, crown retention and cost

Values below are inherited unless marked as remeasured. The growth rule has
not changed, so there are no newly derived maturity ages.

| Preset | Maturity y | Seed-7 live nodes | Threshold | Evidence |
|---|---:|---:|---:|---|
| Oregon white oak | 432 | 196,901 | 0 | Nodes remeasured by fixture probe |
| Norway spruce | 158 | 76,386 | 0 | Nodes remeasured by fixture probe |
| Ordinary | 173 | 44,235 | 0 | fn-30 report |
| Telperion | 173 | 190,340 | 0 | fn-30 report |
| Laurelin | 173 | 194,517 | 0 | fn-30 report |

Authored threshold restoration, the vigour floor, mature-crown tests and
shaded-shoot death-stamp evidence remain pending. The zero-threshold
workaround is still present.

| Species | fn-30 median build ms | Task ceiling ms | This session |
|---|---:|---:|---|
| Oak | 2463.322165 | 2463 | No post-change cost measurement |
| Spruce | 867.473470 | 867 | No post-change cost measurement |

fn-30 measured three-build medians of `Specimen::build`, seed 7, excluding
meshing and foliage materialization. No timing here establishes the
half-second dial target.

## Convergence and pins

Full-species probe counts match the base exactly. Oak is 196,901 nodes /
5,394 crossover and spruce is 76,386 / 21,442. Mesh bounds and placement
counts also match fn-30's production identity table, as printed in
`logs/fixture-probe.log`. No pin moved. Post-change convergence for all five
presets remains pending.

## The strips and stills

No fn-31 strip, hero or beside-fn-30 montage was generated. The required
first checkpoint has not cleared. No image was viewed in this session.

Inherited artifacts remain at:

- `../fn30/strips/oregon-white-oak-strip.png`
- `../fn30/strips/norway-spruce-strip.png`
- `../fn30/oregon-white-oak-hero.png`
- `../fn30/norway-spruce-hero.png`
- `../fn30/oregon-white-oak-beside-fn14.png`
- `../fn30/norway-spruce-beside-fn14.png`

The seven-frame fn-31 protocol, year-one framing parameter and new mature
montages remain pending. No new owner judgment exists.

## Gates

| Command | Exit code | Absolute log path |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/clippy.log` |
| `cargo test --release --workspace` | 101 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/workspace-test.log` |
| `npm test` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/npm-test.log` |
| `npm run typecheck` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/typecheck.log` |

The workspace command exits at `bark_detail` with the same two-channel
failure as the focused run. npm test passes all 77 tests, including native
to Wasm parity. Formatting, clippy and typecheck pass.

## Blocked

Checkpoint 1 has four failing renderer targets. The relief fixture tests a
bark scale larger than its retained juvenile radius, and the grazing
fixture selects zero pixels from populated geometry. This session reports
these faulty fixture/gate conditions under the explicit instruction against
editing around a gate. Distance agreement also exceeds its limit by
0.139683333333333/255, and conformance set 6 lacks foliage. R1-R6,
implementation, red/green regression tests, post-change numerical evidence,
new captures, documentation updates and the single re-pin remain pending.

## Owner verdict

- R1, oak strip and mature comparison:
- R1, spruce strip and mature comparison:
- R2, reference composition and diameter deviations by age:
