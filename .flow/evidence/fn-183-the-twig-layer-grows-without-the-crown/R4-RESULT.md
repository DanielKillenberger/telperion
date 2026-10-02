# fn-183 task 2: the twig layer asks the crown nothing (R3 to R6), 2026-10-02

Branch `fn-183-the-twig-layer-grows-without-the-crown`, measured at `3b798e95` (generator final; later commits touch only the leaf box, tests and pins). Base is `f1ec68be`: master's generator plus the profiler, which compiles out for timing.

## What changed

- **The wall is gone from the direct build** (`local/planner.rs`, `local/advance.rs`). The per-stride outline check, the 40-step bisection and the terminal and leaf-twig admission now run only when `growing_envelope` is set. The growth path keeps its wall unchanged. No code became dead on every path, because the growth path still uses `admitted`, `rejected` and `Curtain::admits`.
- **The curtain floor is exact** (`Curtain::crossing`, `Planner::cut`). A curtain stride that crosses the floor plane is cut there in closed form: one height compare and one interpolation. The forced test curtain's lowest node is back above its 1 m clearance; under rung 1 it reached 0.961 m.
- **The hang row fades in** (`pendant.rs`). Two terms switched on in full at any hang above zero. The sag's turn ignored hang, which cost about 21k of the birch's nodes at hang 0.01. The floor jumped from none to near the ancestor's tip, about 7k more. Both now scale with `hang.min(1)`, and the floor rises from the ground. Birch node counts across hang 0, 0.01, 0.1 and 0.2 are 70,238, 70,434, 71,960 and 71,902. Every shipped table hangs at 1 or more, so no shipped tree moves by this.
- **Leaves are boxed by the grown tree** (`foliage/reference.rs`, `stage.rs`, `executor.rs`). On the direct build, both the CPU stage and the GPU expansion size the quantisation box from the grown tree. Each node is grown by the reach of a station seated on that node's own widest radius, and never less than the authored reach. The parameter box stays for family validation and the growth path. No direct-build consumer reads the box before the tree exists: `input.rs:99` only computed it early, and its readers hold the tree.
- **The species gate reads the tree.** The oak's width was stuck at 26.64 to 26.67 m on all 12 seeds. The cause was that the gate placed leaves against the authored box (`suite/species.rs:402`), and the width metric is the retained leaves' bounds (`examples/species_metrics/mod.rs:106`). Oak leaves clamped onto the box wall at ±13.2 m. On the grown box the oak's width varies from 27.6 to 31.1 m, and every profile check passes.

## Tests

- **New:** `suite::twig_extent::the_twig_layer_asks_the_crown_nothing_and_builds_one_tree`. On every catalogue preset it checks zero `twig_stride`, `twig_bisection`, `terminal_admission` and `curtain_band` queries, and that two builds give one tree. To make this possible, the query counters now compile into the crate's own tests.
- **Curtain floor:** `drop::no_shoot_falls_below_the_clearance…` holds the floor. Its second half now compares the tree built with a clearance above the base against the tree built at the base: wood that no curtain floors may dip under the base.
- **Deleted, owner (2026-10-02):** the containment parts of `species::fixed_*`, `growth`, `habit`, `drop` and the shortened-limb `limb_tests`. The scaffold-in-share check stays.
- **Changed by the host's decision:** the drop walk's lowest tenth may rise by up to 1 cm per step. The generator-born collapse fixture is retired: no tree drops triangles in seeds 1 to 64 of any catalogue preset, the beech, or the fixture's own beech. The palm's bases test compares decoded stations within the boxes' steps. The beech at spread 1e300 now builds. The beech's round-trip bound moves from 0.26 to 0.30 mm.
- **Re-pinned (Generator evolution):** species digests, identity pins, the catalogue `pins.json` records (including the palm's) and pages, the drop, sag and strands neutral pins, the oak and spruce audit pins, and the fn-24 clay still. Two limit-inventory entries were added.

## R4: paired runs (counters on for queries, off for time)

Noise policy: 3 interleaved rounds per preset and seed, 5 samples each, cold first sample dropped, so 12 warm samples per side, on an AMD Ryzen 9 5950X. A preset counts as "slower" when the branch's warm median falls outside the base's warm range.

| Preset | Seed | Queries base → new | Δ | ms base median [range] | ms new median [range] | Δ | Nodes | Leaves |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| Oak | 1 / 7 | 184k → 26k / 198k → 33k | −86% / −83% | 46 [44–55] / 61 [59–62] | 42 [40–46] / 45 [44–51] | −10% / −26% | 125k→134k / 139k→144k | 715k→767k / 869k→903k |
| Beech | 1 / 7 | 294k → 36k / 294k → 37k | −88% / −87% | 96 [92–102] / 97 [93–105] | 69 [66–73] / 69 [67–75] | −28% / −29% | 188k→213k / 191k→215k | 4.93M→5.83M / 5.00M→5.86M |
| Birch | 1 / 7 | 1268k → 20k / 1291k → 21k | −98% / −98% | 183 [180–188] / 187 [184–194] | 23 [22–32] / 24 [24–31] | −87% / −87% | 80k→86k / 87k→92k | 232k→258k / 261k→279k |
| Spruce | 1 / 7 | 259k → 169k / 245k → 160k | −35% / −35% | 36 [34–40] / 37 [32–42] | 38 [33–40] / 31 [31–40] | +5% / −15% | 95k→96k / 90k→91k | 7.35M→7.41M / 7.01M→7.07M |
| Telperion | 1 / 7 | 467k → 239k / 73k → 41k | −49% / −44% | 383 [379–420] / 54 [51–58] | 364 [360–415] / 52 [49–55] | −5% / −4% | 76k→77k / 17k→18k | 535k→550k / 118k→129k |
| Ordinary | 1 / 7 | 116k → 69k / 118k → 71k | −40% / −40% | 45 [44–49] / 45 [44–49] | 40 [39–48] / 45 [44–47] | −11% / +1% | 12k / 12k | 41k / 42k |
| Laurelin | 1 / 7 | 152k → 87k / 245k → 134k | −43% / −46% | 102 [99–111] / 184 [180–196] | 98 [94–102] / 172 [168–182] | −4% / −6% | 53k→54k / 75k | 387k→393k / 542k→544k |
| Date palm | 1 / 7 | 14.9k → 15.1k / 15.4k → 15.4k | +2% / +0% | 1.585 [1.567–1.656] / 1.710 [1.619–1.815] | 1.679 [1.622–2.009] / 1.642 [1.624–1.933] | +6% / −4% | 552 / 553 | 6.6k / 6.6k |

- **Targets met:** queries are 83 to 86% lower on the oak (target 80), 87 to 88% on the beech (85), 98% on the birch (95), 35% on the spruce (30) and 44 to 49% on Telperion (40). Growth time is 28 to 29% lower on the beech (target 20) and 87% on the birch (80).
- **Miss: "no preset slower" fails on the date palm at seed 1.** The branch's warm median is 1.679 ms, outside the base range of 1.567 to 1.656 ms: 0.09 ms, or 6%, on a 552-node build whose twig layer made only 41 queries. Its tree hash moved, and shedding queries rose from 0 to 272. Seed 7 keeps its tree hash and is not slower.
- **Remaining queries:** with the twig layer at zero, what is left is the scaffold's containment and sampling plus shedding. Shedding is 230k of Telperion's 239k at seed 1 and 46k of the ordinary's 69k.
- **Nodes and leaves rise on every preset,** by 7 to 18% on the oak, beech and birch, so no crown was emptied.

## Whole-run excursion past the lobed outline (report only, share of widest radius)

| Preset | Seed | Axes outside | Median | p95 | Max | Worst at top / below base / beside | Band nodes |
|---|--:|--:|--:|--:|--:|---|--:|
| Oak | 1 / 7 | 7.08% / 3.85% | 0.065 / 0.040 | 0.394 / 0.281 | 0.780 / 0.677 | 4-572-3599 / 10-148-2519 | 0 |
| Beech | 1 / 7 | 13.55% / 13.10% | 0.101 / 0.101 | 0.488 / 0.490 | 0.877 / 0.832 | 759-0-11615 / 875-0-11214 | 0 |
| Birch | 1 / 7 | 4.28% / 3.58% | 0.227 / 0.265 | 0.473 / 0.498 | 0.750 / 0.681 | 0-6-1637 / 0-3-1466 | 14,769 / 11,548 |
| Spruce | 1 / 7 | 1.11% / 0.92% | 0.022 / 0.030 | 0.177 / 0.225 | 0.331 / 0.432 | 5-6-537 / 5-9-417 | 0 |
| Telperion | 1 / 7 | 3.62% / 8.49% | 0.132 / 0.089 | 0.482 / 0.356 | 0.757 / 0.456 | 47-0-777 / 0-0-448 | 0 |
| Ordinary | 1 / 7 | 0.03% / 0.12% | 0.007 / 0.106 | 0.007 / 0.209 | 0.007 / 0.209 | 0-0-1 / 0-0-4 | 0 |
| Laurelin | 1 / 7 | 1.91% / 0.17% | 0.126 / 0.316 | 0.890 / 0.356 | 1.005 / 0.425 | 82-0-222 / 23-2-14 | 0 |
| Date palm | 1 / 7 | 100% of 42 / 37 | 3.68 / 2.85 | 4.78 / 4.75 | 5.20 / 5.38 | all above the top | 0 |

The method is R1's `ladder_stats`, run from a scratch copy that was never committed. The oak, spruce, beech and Telperion rows equal rung 1's. The birch moved slightly from the floor cut: 86,326 nodes against 86,549 at seed 1. The palm's few twig-layer axes sit above its thin smooth outline, which is why every one of them counts as outside.

## R5: stills

All are 960x720 headless at the hero pose, on the final code (`2ce984c8`), under `raw/stills/` (gitignored):
- `oregon-white-oak-s{1,7}-final-{whole,bare}.png`
- `european-beech-s{1,7}-final-{whole,bare}.png`
- `silver-birch-s{1,7}-final-{whole,bare}.png`
- `norway-spruce-s1-final-{whole,bare}.png`
- `telperion-s1-final-{whole,bare}.png`

Two were opened to check they are not blank: the birch at seed 1 whole, and Telperion at seed 1 bare.

## R6: gates and costs

- **`cargo test --profile ci --workspace --no-fail-fast`:** 1,103 passed, 2 failed, 22 ignored, in 307 s.
  - `telperion-jev` `replay::the_recorded_beech_replays_through_tunes_first_revision` fails because its tape lacks the shot-look request (key `4ff9e13b…`). The beech's candidate renders changed, and re-recording needs a live Jev and network run (`species european-beech --record`). The test is GPU-only and skips in CI.
  - `objectives::the_palm_s_materials_track_leads_with_its_own_objective` is a pre-existing race: both tests in the file share `fn119-objectives-<pid>` (`objectives.rs:28`). It passes 3 of 3 runs alone.
- **`npm test`:** green, 14 files and 141 tests, in 136 s, after `npm ci` in the worktree.
- **Build time:** release `growth_profile` rebuilds in 19 to 22 s on both base and branch. `npm run build` on the base took 55 s.
- **Peak RSS of one growth run (base → branch):** oak 44.7 → 46.3 MB, beech 67.2 → 73.4 MB, birch 31.9 → 34.2 MB, Telperion 66.8 → 68.9 MB.
- **Artifacts** (`node scripts/artifact-budgets.mjs` after `npm run build`; ceilings only, since this is not a PR run):

  | Artifact | Base | Branch | Growth | Ceiling |
  |---|--:|--:|--:|--:|
  | `telperion.wasm` | 1,420,078 | 1,421,083 | +0.07% | 1,600,000 |
  | `telperion-render.wasm` | 1,993,654 | 1,996,108 | +0.12% | 2,200,000 |
  | `telperion-field.wasm` | 369,691 | 369,935 | +0.07% | 400,000 |
  | `telperion.js`, `field.js`, `voxelize.js` | 119,870 / 2,720 / 4,526 | unchanged | 0 | 150,000 / 16,000 / 16,000 |

Raw output is under `raw/` and in the session scratchpad. Neither is committed.
