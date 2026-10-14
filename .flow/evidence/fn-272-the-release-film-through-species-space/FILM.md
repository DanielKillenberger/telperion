# fn-272 task 1: the path, the map, the film tool and the first cut (worker, 2026-10-14)

Raw output is in `raw/` (ignored): `raw/cut3/` (1,050 stills, `frames.tsv`, `film.mp4`), `raw/cut3.log`, `raw/oracle.log`.

## The tool

- **Path (R1, R1a):** `crates/telperion-render/examples/film/release.json`, read by `examples/film.rs` (`film/path.rs`). Nine segments over 35 s at 30 fps, 1280x720, seed 7: beech grown 1 to 40 (7 s); walks at 40 beech to spruce, spruce to birch, birch to oak (3 s each), oak to palm (4 s); the palm grown 40 to 1, played backwards (3 s); palm to Laurelin at 1 (1 s); Laurelin grown 1 to 80 (7 s); Laurelin to Telperion at 80 (3 s, then a 1 s hold under "Every frame grown live by the engine"). The camera is declared beside them: azimuth 20 degrees turning 4 a second, 8 degrees down, a 38-degree lens, and keys of distance and aim height eased between (monotone cubic on the log of the distance, `film/camera.rs`). Key values were set from the trees' measured bounds.
- **Growth (host ruling, 2026-10-14):** settled at whole ages only, where R5c stands the tree at full leaf, never within a year; a frame between two whole ages is both years drawn from its own pose and crossfaded by the share of the year (`film.rs`, `grow`). The ruling followed the owner's verdict on fn-270's four-draws-a-year grow-in ("leaves flickering in and out") and drops R1a's "leaves unfolding each spring" (spec R1a updated). Cut 2 sampled the nearest whole age with no crossfade; cut 3 is the crossfaded one.
- **Growth path:** `Generator::{begin_growth, grow_to, settle}`, one tree per segment, ages in rising order (a shrink's frames are drawn last to first).
- **Walks:** each frame's family `blend::families(a, b, smoothstep(t))` built by `mesh::build` and put up with `submit_at`, as `headless --walk` does. The device build was tried first and refuses walk frames (Defects, 1).
- **Map (R1b):** `film/map.rs`, tested by `tests/film_map.rs`. Every catalogue row a walk moves on its own (Linear, Weighted, Degrees and the rounded counts; age, kept and coupled rows left out) and every engine-species setting, each on the scale `telperion_space::blend` mixes it on (log above its floor for magnitudes, kept share for delays, the relaying share for relays, value otherwise), less its mean and over its standard deviation across the eight catalogue presets; the projection onto the first two principal components (Gram matrix, Jacobi). Not log-odds for probabilities: the walk mixes them by value (fn-251 rejected natural-scale walks), and only the walk's own scales keep a walk straight on the map.
- **Overlay:** drawn into the still's pixels after rendering (`film/overlay.rs`): a translucent panel at the lower right with the eight dots and names, the trail of the path so far, the current point, the axis names; `age N` at the top left; the closing line fading in over the hold. Letters are the public-domain 8x8 bitmap font (`font8x8`, dev-dependency).
- **Oracle (R3):** `film --oracle` grows every walk frame by the build's cycle and judges it by fn-251's seed-spread band (`film/oracle.rs`).

## The map

63.1 % of the variance on the two axes. x: zone node counts (`zone.nodes.min`, `zone.nodes.max`); y: `zone.lateral`, `shade_hazard`. Dots (x, y): date palm 50.1, -3.8; spruce 2.5, 5.6; ordinary 0.8, -0.6; oak -6.2, 12.1; birch -9.8, 30.8; beech -12.3, -14.4; Telperion -12.5, -14.8; Laurelin -12.6, -15.1. The palm alone spans most of x; the beech, Laurelin and Telperion overlap (the Two Trees share the beech's engine species), so their names collide on the panel and the last walk barely moves the point.

Test: every pair of the eight at 40 and at 1, t = 0, 0.25, 0.5, 0.75, 1: the ends land on the dots (1e-12) and every frame within 1 % of the segment's length of its point a share t along. The worst is 0.45 % (oak to birch, t = 0.5), from whole-count rows rounding at each frame. With magnitudes mapped by value instead of their log the test fails at 2 % (checked).

## Cost (cut 3, 1,050 frames, 39.7 min wall)

| segment | frames | trees | build | draw | peak resident | device |
|---|--:|--:|--:|--:|--:|--:|
| 0 beech grown 1-40 | 210 | 40 | 3.9 s | 9.2 s | 727 MB | 1,228 MB |
| 1 beech to spruce | 90 | 90 | 266.1 s | 10.1 s | 2,902 MB | 1,548 MB |
| 2 spruce to birch | 90 | 90 | 312.4 s | 9.7 s | 2,773 MB | 2,060 MB |
| 3 birch to oak | 90 | 90 | 157.3 s | 8.7 s | 1,798 MB | 1,804 MB |
| 4 oak to palm | 120 | 120 | 30.8 s | 5.3 s | 1,671 MB | 1,804 MB |
| 5 palm 40 to 1 | 90 | 40 | 2.1 s | 7.5 s | 1,182 MB | 1,804 MB |
| 6 palm to Laurelin at 1 | 30 | 30 | 0.1 s | 1.7 s | 1,182 MB | 1,548 MB |
| 7 Laurelin grown 1-80 | 210 | 80 | 11.6 s | 10.3 s | 1,187 MB | 4,630 MB |
| 8 Laurelin to Telperion | 120 | 90 | 1,203.4 s | 19.5 s | 6,436 MB | 2,578 MB |

Draw is render (twice for a crossfaded frame), overlay and PNG. Device memory is what `nvidia-smi` saw the process hold, sampled four times a second; resident is the process's peak since the segment began. Overlay map points take about 1.5 min before the first frame; ffmpeg about 10 s.

## R3

- **Walks, fn-251's oracle on every frame (`raw/oracle.log`, 31 min):** 510 frames, 478 meet it. Beech to spruce 90/90, spruce to birch 90/90, palm to Laurelin at 1 30/30, Laurelin to Telperion 90/90. Birch to oak 64/90: kept above the band's ceiling (0.537M) from t = 0.15 to 0.99, up to 0.670M (25 % over 0.537M), grown above 1.474M at nine frames; fn-251's named exception at t = 0.5 and judged by the owner's eye (fn-251 decisions). Oak to palm 114/120: kept 421 to 430 below the floor of 432 at t = 0.64 to 0.72, up to 2.5 % under; these frames are not among fn-251's named exceptions (it measured t = 0.75 at 440).
- **Growth, pop:** no frame settles within a year, so R5a's in-year blend is not what carries the motion; a year's growth comes in as a crossfade of the two whole years over the frames between them (5.25 frames a year for the beech, 2.6 for the Laurelin, 2.2 for the palm's shrink). The largest year-to-year steps (cut 2's `frames.tsv`, whole ages): the beech's first year, 0.49 m to 1.05 m and its first 4 leaves; the Laurelin's, 1.28 m to 2.73 m; the palm 3 to 2, 24.7 % in height. Seen in cut 3: frame 100 (beech between 19 and 20) shows both years' crowns, the later faint.
- **Joins (last frame of one segment, first of the next, `frames.tsv`):** beech settled at 40 (66,779 leaves, 10.59 m) to the first walk frame (69,437, 10.60 m); oak-to-palm's end (6,554 leaves, 21.65 m) to the palm settled at 40 (6,554, 21.56 m); the palm settled at 1 (1,017 leaves, 7.29 m) to the walk's first frame (1,017, 7.17 m); the walk's Laurelin at 1 by the direct build (0 leaves, 1.14 m) to the Laurelin settled at 1 (0 leaves, 1.28 m, 12 % taller); the Laurelin settled at 80 (863,268 leaves, 46.87 m) to the walk's first frame (863,263, 46.87 m).

## Stills viewed (cut 2, and cut 3's crossfades: 100 beech 19-20, 640 palm 22-21, 760 Laurelin 16-17)

60 (beech 12), 209 (beech 40), 255 (beech to spruce, t about 0.5), 345 (spruce to birch), 435 (birch to oak), 505 (oak to palm, t 0.08), 540 (oak to palm, t 0.5), 650 (palm 18, shrinking), 689 (palm 1), 705 (palm to Laurelin at 1), 800 (Laurelin 31), 929 (Laurelin 80), 975 (Laurelin to Telperion), 1049 (Telperion, closing line).

## Defects seen, for the host (none hidden by camera or cut)

1. **The device build refuses walk frames:** beech to spruce at 40, t = 0.00037, 0.01 and 0.1 at seed 7, and at seed 1: "the build passed its sizes at cycle N: UnitRounds (0x1)"; t = 0.5, 0.9 and both ends build. The Laurelin at 20 alone: "Given (0x1)". Other walks' probed frames built.
2. **Oak to palm: the leaves go at once.** 871,168 leaves at t = 0.0098, 4 at t = 0.0127 (`frames.tsv`, frames 486 to 487); frame 505 (t = 0.08) is a bare oak-shaped crown on a palm-barked stem.
3. **The palm seedling is a full-size crown.** The palm settled at 1 is 7.3 m tall and 16.3 m across with 1,017 leaves; frames 650 and 689 show fronds at mature length on a stump, so the shrink to "a seedling" ends on a stump under a 16 m rosette. The direct build at 1 agrees (7.29 m, 16.3 m).
4. **The Laurelin at 1:** the direct build stands 1.14 m and the settled growth 1.28 m, a 12 % step at the join; both have no leaves at the whole age.
5. **Visible but not judged:** a broken-looking hanging limb on the beech at 40 (frame 209); a hanging brown skirt mid spruce-to-birch (frame 345); leaf-count steps of 25-38 % between consecutive frames of spruce to birch (t = 0.16) and birch to oak (t = 0.83).

## For the host to decide

- Camera keys: the palm at 40 is cut off at the top (frame 540); the palm seedling's 16 m rosette overfills the frame (689) and the walk to the Laurelin seedling closes in on the scale figure (705). The scale figure stands in every frame (the renderer's default).
- Overlay look: the bitmap font, panel place and size, the colliding beech, Laurelin and Telperion names, the axis names as raw row names.
- Timings: R1a's as written.
