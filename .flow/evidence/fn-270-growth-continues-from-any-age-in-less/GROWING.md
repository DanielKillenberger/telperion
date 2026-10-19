# fn-270 task 5: the page grows the tree (worker, 2026-10-14)

R6 and R7. Raw output is in `raw/t5/` (ignored).

## What was built

- **The growth entry on the device (R7).** `Generator::{begin_growth, grow_to, settle}` (`resident/growing.rs`) hold the device's growth state (`GrowthState`) for a family, begun for the family's age as its horizon and its seed. `settle` draws the tree at an age in its last year and prepares it as a resident tree through the placement a tree built at once takes (`build::placed`, split out of `crowned`), so there is one route from a drawn tree to the canvas. `Prepared` carries the tree's autumn colouring, and the submit sets it.
- **Across the binding (R7).** `WebRenderer::{beginGrowth, growTo, drawGrowth}` (`web/growing.rs`) and `render.ts`'s `Growth`: the state crosses as a handle the page holds; one call at a time on a tree, a second refused as `growth.busy`; each call holds the page's one generation request, so a build and a growth never overlap. A browser without WebGPU gets `createRenderer`'s named "WebGPU is unavailable" refusal; no growth route has a CPU fallback.
- **Tests.** `growing_tests`: the oak at 8 and the palm at 6, grown a year at a time, drawn within each year and put up, end leaf for leaf and bound for bound as the tree built at once; past the horizon, a non-finite age and a draw outside the last year are refused by name, and nothing moves. `render.test.ts`: a growth waits on the programs, grows and draws through its handle, and rejects by name where the programs failed.

## Cost per preset (device, native, `growing_cost`, steps a year as stated)

Grow is `grow_to` over every year divided by the years. Draw is `settle`: the device's settle, the presentation within the year, the dress and the placement, to a prepared tree. Peak is the ledger's over the run, the frame loop's whole tree included.

| Preset (age) | Grow / year | Draw, whole year: mean (max) | Draw within the year: mean (max) | Peak |
|---|--:|--:|--:|--:|
| date-palm (40) | 12 ms | 18 (22) ms | 19 (24) ms | 35 MB |
| ordinary (40) | 27 ms | 34 (59) ms | - | 239 MB |
| european-beech (40) | 30 ms | 54 (83) ms | - | 390 MB |
| silver-birch (40) | 38 ms | 54 (80) ms | - | 689 MB |
| oregon-white-oak (40) | 46 ms | 87 (124) ms | 106 (142) ms | 980 MB |
| norway-spruce (40) | 73 ms | 142 (200) ms | - | 2,414 MB |
| laurelin (80) | 116 ms | 241 (438) ms | - | 3,917 MB |
| telperion (82) | 105 ms | 218 (437) ms | 270 (572) ms | 4,145 (one a year), 4,649 (four) |

The draw costs about twice a year's growth on every preset: the settle is the next lever (an incremental settle is the spec's later spec).

## Frame time while growing (R6)

A page's frame loop draws while its growth is awaited, on the same queue. Natively: one thread grows and draws the tree as the page does (each draw held until the next), another draws the preset's whole tree at the hero pose frame after frame (the dearest frame of the run), each frame timed from its submit to the queue completing it, at 1280 x 720. The device's growth state cannot cross threads (`Rc` in the cycle's plan), so the frame thread draws its own whole tree rather than the drawn one.

| Preset | Idle p50 | While growing p50 / p95 / max | Frames |
|---|--:|--:|--:|
| date-palm | 0.5 ms | 0.6 / 1.6 / 4.3 ms | 1,816 |
| ordinary | 0.7 ms | 0.7 / 3.3 / 15.9 ms | 2,515 |
| european-beech | 1.6 ms | 1.7 / 4.4 / 16.3 ms | 1,612 |
| silver-birch | 1.3 ms | 1.4 / 4.7 / 12.7 ms | 1,922 |
| oregon-white-oak | 2.8 ms | 3.0 / 6.1 / 15.9 ms (four a year: 21.5) | 1,552 |
| norway-spruce | 4.1 ms | 4.2 / 8.7 / 30.6 ms | 1,729 |
| laurelin | 6.9 ms | 7.2 / 19.3 / 104.7 ms | 2,964 |
| telperion | 5.9 ms | 6.2 / 18.7 / 130.2 ms (four a year: 19.2 / 247.3) | 2,985 |

Every preset up to the spruce holds a 60 Hz frame at p95 while it grows; the Two Trees hold it at p50 and miss it at p95 (19 ms), with single frames waiting 100 to 250 ms behind one long submission of the cycle or the settle. The browser's own frame time is the owner's to measure (the timing session, `gpu timing`, while a tree grows).

## The recording (viewed)

`examples/grow_in.rs` grows a preset through the same entry, four draws a year, each put up and taken as a still from the camera that frames the grown tree: `raw/t5/oregon-white-oak-grows-in.mp4` (157 stills) and `raw/t5/telperion-grows-in.mp4` (325 stills), 24 fps. Stills viewed: the oak at 20 (in leaf, young crown), at 39.5 (winter, bare) and at 40 (full leaf); Telperion at 41 (in silver flower, about half its height) and at 82 (the grown tree). A seedling stands only from its first budburst, near the end of year 1: a draw before it has no wood and is refused by the pipeline as "surface height", so a grow-in starts at year 1's whole age.

## Consumers

- **fn-257's field query:** no device consumer exists on this branch (fn-257 has only its two spikes done; the field reads the CPU's tree through `telperion-field`). Whether it takes the state is unknown until fn-257 builds it.
- **The CPU Wasm entry (`telperion-wasm`):** takes no growth state; the page grows on the device. The CPU state still borrows its parameters (`GrowthState<'p>`); no consumer needs it owned yet.

## The harness (R6; host rulings, 2026-10-14)

`harness/grow-in.ts`, `GrowerDev.tsx`. Opening, a preset, a look, the seed (typed or rerolled), reset and the age grow the tree in from year 1, a whole year a draw, as fast as the device allows and each drawn year up at least 150 ms; the trait dials build the grown tree at once, as before. "grow again" replays the grow-in; "play the year" steps the tree through its last year in 12 draws (budburst, full leaf, autumn, fall, winter) and ends at the whole age, on the growth already held where it is this tree's and grown whole. The canvas keeps the last drawn tree while the next one grows; a newer request abandons a run between its draws and lets its state go (no queue). The panel says "year 23 of 82" while a run draws. A stage still waiting to be framed (opening, a preset, a seed) follows the tree as it grows. There is no `?growth=1` flag: the old growth path and its flag went in fn-181, and growth is now how a tree appears. Tests: `harness/grow-in.test.ts` (years and floor, the year played on the same tree, a trait build letting go, abandonment).

## Open (for the settle lever)

- The Two Trees miss a 60 Hz frame at p95 while growing (19 ms), and single frames wait 100 to 250 ms behind one long submission of the cycle or the settle (host, 2026-10-14: an open item for the settle lever, not a blocker).

## The owner's steps in the harness

1. In the worktree: `npm run dev`, then open the printed address in Chrome.
2. The beech opens growing in: "year 1 of 40" to "year 40 of 40", last year's tree staying drawn while the next grows.
3. Choose telperion: it grows in to 82 (about 30 s). Drag the camera while it grows; the frame should stay smooth apart from the stalls above.
4. Press "play the year": the grown tree goes through budburst, full leaf, autumn colour, leaf fall and winter, and ends in full leaf at 82. Do the same on the oak (brown autumn) and the date palm (new fronds at the crown's centre).
5. Press "grow again" to replay; move a trait dial: the grown tree is built at once, with no grow-in.
6. While a tree grows, press "gpu timing" and note the vegetation p50/p95: the browser's frame time while growing.
7. A link grows in too: `?species=oregon-white-oak&age=40&seed=7`.

## Gate, tests and review

- Gate (`telperion-heavy 9 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast`, at 917af11e1): 1,370 passed, 0 failed, 90 ignored, 23 minutes.
- The review's fixes (e9b187578) came after the gate: `growing_tests` (2 of 2, a failed placement spending the tree among them) and `harness/grow-in.test.ts` (4 of 4) cover them.
- `npm test` (`telperion-heavy 10 --threads 4`), after the fixes: 168 of 168.
- Codex review: NEEDS_WORK on a placement failure leaving the tree usable, the floor timed from before the growth, and a one-year tree's played year; all three fixed; the field-query finding declined under the host's dispatch (fn-257 has no consumer here); re-review SHIP.

## Owner (2026-10-14)

- The first recordings (four draws a year): "the video isn't very appealing with leaves flickering in and out". Host ruling: recordings and the film's growth segments draw whole years at full leaf, crossfaded (`raw/t5/whole/*-whole-years.mp4`); the seasons stay in the harness's "play the year".
- Chrome refused the cycle hierarchy module (`|` mixed with `<<` unparenthesised, which Tint refuses and naga accepts); fixed by 6945bf72b's cherry-pick, caught from now on by fn-237 R14's Tint probe.
- After the fix, the harness grow-in: "yea it grows now. nice".
