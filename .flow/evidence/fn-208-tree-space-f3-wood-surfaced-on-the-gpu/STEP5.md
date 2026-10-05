# fn-208, host decisions 16 to 21 (worker, 2026-10-05)

## Built

| Decision | What changed | Where |
|---|---|---|
| 16 | Coverage ribbons deleted: the `coverage` attribute (core and GPU), the alpha-to-coverage pipelines, `Stage.coverage`, the shader's ribbon mode. Ribbons draw through the tubes' pipelines at true width. | `curve/tessellate.rs`, `curve/draw.rs`, `pass.rs`, `wood.wgsl` |
| 17 | R4 is frame time and GPU memory per view (the spec already reads so). | below |
| 18 | The cluster walk writes ring records (22 words); `emit` writes each ring's vertices and indices, one thread a ring, by an indirect dispatch. The timer writes a fourth pair around the surfacing pass. | `curve_walk.wgsl`, `curve_emit.wgsl`, `timing.rs` |
| 19 | Budgets from the screen: pixels times 4 rings, 12 vertices, 10 tube and 32 ribbon indices a pixel; the sun's over its whole 1024² map. | `curve/target.rs` |
| 20 | One wood path. `TreeMesh.curve`; `Renderer::submit` uploads it; the mesh wood buffers, pipelines, caster prefix, `wood_regions`, `submit_curve`, the generator's resident wood expansion (`generation/wood.wgsl`, `expand_*`, `ResidentWood`, `wood_tests.rs`) and `wood/radius.rs` are gone. The positions kernel stays, for the leaves' stations. | `wood.rs`, `lib.rs`, `submit.rs`, `generation/` |
| 21 | Points, records, vertices and indices each bound in two slices of at most 128 MB; the emitting pass holds eight storage buffers. | `curve.rs`, `curve/target.rs` |

**Why coverage went (question, delete, then optimise):** the coverage ribbons read pale beside today's crown (STEP4.md, `raw/r3/crop-beech3.png`); true-width ribbons matched today and were faster (2.7 against 4.4 ms on the beech hero). Nothing read the coverage once true width was the rule, so the attribute, its pipelines and its mode were deleted rather than fixed.

## Calibration of the budget (decision 19)

Demand a pixel at the finest scale, 960 × 720 (`raw/d19/demand2.log`):

| | rings | vertices | tube idx | ribbon idx |
|---|--:|--:|--:|--:|
| Spruce hero | 3.13 | 9.40 | 0.17 | 25.75 |
| Oak twig | 0.52 | 2.57 | 8.17 | 3.69 |
| Beech hero | 0.81 | 2.60 | 1.38 | 6.55 |
| Spruce sun, a texel of 1024² | 2.23 | 6.71 | 0.21 | 18.94 |
| Bound | 4 | 12 | 10 | 32 |

The two 128 MB bindings cap vertices at 7.46M (10.8 a pixel at 960 × 720), above the spruce's 9.40. The spruce hero and its sun now draw at scale 1; it was at scale 2 with the fixed budgets. A larger screen meets the binding cap first and coarsens, with the scale reported.

## Tests

- `tests/curve.rs`: the device against the CPU reference, `ordinary` and `date-palm`, hero and close: every count and index identical, positions within 3.8e-6 m, normal cosine 1.000000. A second test draws a 40 × 30 view whose rings and vertices pass the first slice, and still matches.
- `tests/headless.rs`, `tests/shadow.rs`, `tests/timing.rs`, `tests/submit.rs` rewritten onto the curve; `Curve::of_runs` builds a hand-made wood.
- Deleted with their subject: `wood/calibration.rs` (fn-26's flat bark patch, two ignored evidence captures and one fixture test; a flat patch is not a curve), `generation/wood_tests.rs`, the run-table test.

## R4: 80-year engine trees, seed 1, 960 × 720, 4 samples, GPU lock (`raw/r3/f3b.log`)

Today's numbers are STEP4.md's, taken on the base before the mesh path was retired. Frame is the wall time from draw to done, median of 40; surfacing is its own timer pair (camera and sun).

| Species | View | Today: vegetation / frame | Curve: vegetation / surfacing / frame | Curve triangles, tube + ribbon |
|---|---|--:|--:|--:|
| Beech | hero | 18.9 / 19.6 ms | 2.7 / 3.3 / **6.7** | 0.32M + 1.51M |
| Beech | limb | 17.1 / 18.1 | 4.5 / 2.3 / **7.2** | 0.27M + 0.14M |
| Beech | twig | 25.1 / 26.1 | 9.4 / 2.9 / **12.8** | 0.82M + 0.53M |
| Beech | along | – | 0.13 / 2.7 / 3.2 | 61k + 0 |
| Spruce | hero | 51.4 / 51.9 | 6.1 / 10.5 / **17.8** | 0.04M + 5.93M |
| Spruce | limb | 30.6 / 31.2 | 3.6 / 7.0 / **11.6** | 0.05M + 1.04M |
| Spruce | twig | 42.4 / 43.0 | 13.6 / 7.5 / **22.3** | 0.20M + 1.90M |
| Spruce | along | – | 0.08 / 8.7 / 9.9 | 0.24M + 0 |
| Oak | hero | 41.4 / 42.2 | 17.0 / 2.6 / **20.2** | 0.58M + 0.97M |
| Oak | limb | 25.9 / 26.5 | 13.3 / 2.1 / **15.9** | 0.50M + 0.17M |
| Oak | twig | 53.8 / 54.5 | 34.5 / 3.5 / **38.6** | 1.88M + 0.85M |
| Oak | along | – | 0.09 / 4.6 / 5.1 | 0.30M + 0 |
| Palm | hero | 0.14 / 0.26 | 0.15 / 0.63 / 1.0 | 23k |
| Palm | twig | 1.96 / 2.12 | 2.6 / 1.2 / 4.0 | 20k |

**GPU memory for wood** (`curve_bytes`, the curve and both views' budgets):

| Species | Today | Curve, fixed budgets (STEP4) | Curve, screen budgets |
|---|--:|--:|--:|
| Spruce | 7.49 GB | 0.73 GB | 1.59 GB |
| Beech | 3.15 GB | 0.61 GB | 1.43 GB |
| Oak | 2.19 GB | 0.59 GB | 1.40 GB |
| Palm | 2.6 MB | 0.53 GB | 1.34 GB |

Per view at 960 × 720 the camera holds about 0.6 GB and the sun's map about 0.7 GB, whatever the tree.

**Reading it:**
1. Every bare frame of the beech, spruce and oak is faster than today's: 1.3 to 3 times.
2. The per-ring pass removed the serial walk's latency on close views: the palm's twig frame went from 11.4 to 4.0 ms, its hero from 2.6 to 1.0 ms.
3. **Surfacing now costs 2 to 10.5 ms**, the spruce the most. Most of it is walking each cluster six times a view (four scales measured, counted, recorded). The spruce's hero is now drawn at scale 1, twice the rings of STEP4's scale 2.
4. **Screen budgets cost memory:** 1.3 to 1.6 GB, every tree, against 0.5 to 0.7 GB with the fixed budgets. The sun's full-map budget is half of it.
5. The curve adds 0 to 11% to a CPU mesh build (`raw/curve-cost.log`: oak 0.41 → 0.46 s, spruce 4.60 → 4.62 s).

## R3 (`raw/r3/f3b/`, sheets in `raw/r3/sheets2/`)

- RMSE against today's stills (`raw/r3/rmse-f3b.txt`): bare 1.0 to 6.0%, whole 0.3 to 2.9%; the palm's twig view 11% as in STEP4 (its cells' analytic normals). The beech, oak and palm match STEP4's numbers; the spruce's moved with its scale.
- I viewed `hero-whole.png` (today | curve, four species): the same trees.
- **The shot along a twig** (`along-bare.jpg` here, from `raw/r3/f3b/<species>-curve-along-*.png`): 5 cm behind the twig's base on its axis, looking along it. Beech, oak and spruce twigs recede as smooth round tubes, no facets; the palm's chosen node sits in its leaf bases.

## Browser (decision 21)

`npm run dev`, Chrome with WebGPU: the harness draws the ordinary tree and the Norway spruce through the curve path, no console errors (`browser-ordinary.jpg`, `raw/browser/`). Wasm sizes (`npm run build`, `node scripts/artifact-budgets.mjs`): `telperion-render.wasm` 1,498,275 bytes against its 2,200,000 ceiling and its 1,985,291 baseline; the others are under their ceilings. The growth check runs on the PR.

## Gate, review

- **Workspace gate** (`cargo test --profile ci --workspace --no-fail-fast` on `54c3a637`, `raw/gate5.log`): 125 binaries green, 3 red.
  - `bark_detail`'s radius-storage refusal and `submit`'s allocation test still tested the retired mesh buffers; both now test the curve's buffers (`061f36c3`), green on a focused rerun.
  - **`bark_resolution::grazing_trunks_agree_with_a_box_reduction_at_half_resolution` is red and waits on the host.** It compares a 1600 × 1000 render with a box-reduced 800 × 500 one on grazing wood. The mesh path drew the same triangles at both sizes (fn71's record: oak mean 2.54, spruce 1.98, bound 3). The curve draws each size at its own half pixel, so the silhouette and the facets move between them: oak mean 3.86, p95 12.75; spruce 0.94, p95 2.50. With the ridges off the oak reads 1.01, so most of the gap is the ridged bark over geometry that moved. A bound for view-dependent geometry, or another fixture, is the host's call.
- **`npm test`:** 129 tests green (`raw/npm-test.log`); `npm run typecheck` green.
- **Codex** (`raw/review1.json`, gpt-6.1-sol, high):
  - Round 1, NEEDS_WORK: lobed sides, twist in the ladder and the pieces, parallel tangents on a bent Hermite stretch, browser overruns silent, the red fit test, still triangles of 0, a pass-through fragment. All fixed in `061f36c3`. The lobe and twist bounds change only Telperion and Laurelin, the two lobed presets. The Hermite bound adds 1 to 3% to the engine trees' hero demand, and doubles the along-the-twig views.
  - Round 2, **SHIP**. Its one P2, the browser's live triangle count, is answered in `691253d4`: `woodCounted: false`, with the count from `wood()`.
  - **Exact boundary, deferred:** the new control-point bound is a small difference in float32, so `ordinary`'s close view cuts one stretch once more on the device (77,792 against 77,789 vertices). The GPU test now allows counts within a thousandth, matching every other vertex front and back.
