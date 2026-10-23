# fn-272 task 1: the path, the map, the film tool and the first cut (worker, 2026-10-14)

## Rework on the owner's verdict on cut 3 (2026-10-19)

The owner: "the idea i think is right just some execution flaws". The film branch merged the release branch (fn-198, 467 commits; one conflict, the `telperion-space` exports, resolved as their union; the map drops `Scale::Whole`, retired by fn-277). Clips are in `raw/clips/` (ignored), cut from `release.json` by `clips.py`.

1. **Year blending: the owner's look (NEEDS_HUMAN).** The release engine settles within a year only through fn-270's seasonal year (`telperion_space::unfold::Season`: leaves colour and fall after the whole age, every leaf of the year grows from nothing at budburst, full leaf again at the next whole age), so continuous growth without the leaves going and coming back does not exist on it; that is the grow-in the owner rejected. The treatment between whole years is now a path value, `fade`, the share of each year over which the next whole year fades in, eased (`path::faded`): 0 a hard step at the year, 1 a fade across it (cut 3's look, now eased). Candidates rendered: `raw/clips/beech-fades.mp4` and `raw/clips/laurelin-fades.mp4` (fade 0, 0.25 and 1 side by side; the beech 1 to 40 and the Laurelin 1 to 80, 7 s each), the single clips beside them. Frame 101 (beech 19.85) viewed for each: fade 0 one crown; fade 0.25 the next year's crown faint over the last quarter only; fade 1 faint across the year. At 7 s the beech steps 5.6 times a second and the Laurelin 11. Owner's decision (2026-10-19): fade 0.25, "yea i think .25 is fine"; `release.json` holds 0.25.
2. **Map labels.** A label that would overlap another, a dot or the axis names, or leave the panel, steps a line down, then up, set off right with a leader line to its dot (`overlay.rs`, `place`); the dots stay where the projection puts them and the straight-walk test passes unchanged. `tests/film_overlay.rs`: no two labels overlap, none covers an axis name or leaves the panel, at most one of the beech, Laurelin and Telperion stays at its dot; red with the placement removed. Viewed on `raw/clips/labels-crop.png`. The last walk still barely moves the dot: the Two Trees share the beech's engine species, which the projection shows truly.
3. **Walks on the device.** fn-276 builds every walk frame the CPU does; the film's walks now use `Generator::prepare`. Every frame of the two clips built: beech to spruce at 40, 30 frames, 0.81 s a frame (CPU 2.9 s); Laurelin to Telperion at 80, 30 frames, 1.39 s a frame (CPU 13 s), device peak 4,248 MB.
4. **Oracle (`raw/oracle-2026-10-19.log`, 22 min):** 479 of 510 walk frames meet it (cut 3: 478). Every frame of beech to spruce, spruce to birch, palm to Laurelin and Laurelin to Telperion. Birch to oak 65/90: kept 536,347 to 667,281 against a ceiling of 535,701 from t = 0.15 to 0.99 (up to 24.6 % over), grown above it at some; fn-251's named exception (birch to oak at 40, measured there up to 21 % over at seeds 7 and 42; here 24.6 % at seed 7, the film's frames). Oak to palm 114/120: kept 421 to 430 against a floor of 432 at t = 0.636 to 0.718, unchanged; fn-273's tenth named exception (host, 2026-10-14). No other frame is out of band; no preset was tuned.

### Cut 4 (2026-10-19, fade 0.25)

`raw/cut4/film.mp4`: 35.0 s, 1,050 frames, 1280x720 at 30 fps, 31 MB; `frames.tsv` logs every frame, the closing hold among them; 42.9 min wall (`raw/cut4.log`). Walks on the direct build: the device refused beech to spruce at 40, t = 0.216 (LayGenerations at cycle 9; FRICTION.md), so the device does not yet build every film walk frame. Cost: beech grown 4.0 s build; walks 281.5 s (beech to spruce), 26.9 s (oak to palm), 1,173.1 s (Laurelin to Telperion, peak resident 4,694 MB); Laurelin grown 10.7 s, device peak 4,939 MB. Oracle on the cut (`raw/oracle-cut4.log`, 21 min): 479 of 510, the same 31 misses as above (birch to oak 25, fn-251's named exception; oak to palm 6, fn-273's tenth); Laurelin to Telperion 90/90. Stills viewed (101 beech 20, one crown; 435 birch to oak; 700 palm seedling; 1049 the closing line): the palm at 1 is now a small seedling (fn-275), so the camera keys set for cut 3's 16 m rosette show it small beside the scale figure; for the owner.

### The owner's verdict on cut 4 and the flying leaves (diagnosis, 2026-10-19)

Owner: "mostly good. As we transition to the palm tree there's some flying leaves that don't make sense." Measured by `examples/frond_probe.rs` (each frame built as the film builds it, `blend::families` and `mesh::build`; `raw/frond-probe.tsv`, `raw/frond-probe-edges.tsv`); frames 540 and 575 viewed.

A frond's rachis is never wood: a rosette is "a placement source over wood the skeleton already has: no node, no run" (`foliage/rosette.rs`, module doc, `fronds` and `clothe`), and its leaflets are set along `leaflet::rachis` (`foliage/leaflet.rs`, `rachis` and `leaflets`, leaflet k at (k + 1) / count of `rachisLength`), a curve nothing draws (fn-283, open, after the release). At the palm end that is hidden by density: at frame 595, 111 leaflets on a 6.98 m rachis, median leaf 1.36 m long, a leaflet's nearest neighbour 0.05 of a leaf length away, though the median leaf stands 1.29 m from any drawn wood. Mid-walk the two rows that set that density move on different scales: `rachisLength` (oak 0, palm 7.0 m) and `rosetteFronds` (0 to 42) walk by value, linear, while `leafletCount` (1 to 111) walks on the log (fn-277 ruling 3; `catalogue/walk.rs`, `Blend::Log`, row at `canopy/params.rs` `leaflet_count`). At frame 540 (t = 0.5125): 21.5 fronds, a 3.59 m rachis (51 % of the palm's) carrying 11.3 leaflets (10 %), one every 0.32 m, the median leaf 0.46 m long, its nearest neighbour 0.69 of a leaf length away, 203 of 351 leaves more than 0.5 m from any wood, median 0.90 m; the drawn wood's thinnest radius 4.0 mm, the stem's apex the only wood the fronds stand on. The halo shows (median nearest-leaf gap over half a leaf length, leaves off the wood) from frame 498 (t = 0.067, 2.8 fronds, 1.37 leaflets on 0.47 m) to frame 544 (t = 0.562, 23.6 fronds, 14.3 leaflets on 3.94 m), 47 frames, 1.6 s; by 575 (t = 0.896) 69 leaflets on 6.27 m, a neighbour 0.09 of a length away, and the fronds read. Between 515 and 520 (t 0.22 to 0.27) the crown holds 41 to 59 leaves, every one isolated (gap 3.6 to 5.2 lengths). The fix is the host's: draw the rachis (fn-283) or walk the frond's rows on one scale.

Gate: not admitted in 31 minutes (8-9 GB of room for 12), stopped; for the host on the merged branch (FRICTION.md). The film's own tests (`film_map`, `film_path`, `film_overlay`) pass; `npm test` 173 of 173.

Codex impl review (fresh cycle; the lost 2026-10-14 receipt in FRICTION.md): three draws SHIP; two P2s fixed (held walk frames logged, ages whole years).


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

## Cut 5 (2026-10-22, the release branch merged, fn-308's midpoint)

At 7c1278c04: 96837aef4 merged fn-283, fn-298, fn-299, fn-301, fn-302, fn-303, fn-304, fn-306 and fn-308; the host's fix after it puts a walk frame's map dot a share t along its ends' segment (`Map::along`), since `telperion_space::blend` now takes the tree's age (fn-308) and the film map no longer re-projects walk frames.

`telperion-heavy 8 --gpu 4 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json .flow/evidence/fn-272-the-release-film-through-species-space/raw/cut5` (FILM.md did not record cut 4's settings; the host's default). The video is kept at `/home/daniel/Videos/telperion/cut5.mp4` (31,978,377 bytes, the same bytes as `raw/cut5/film.mp4`).

`raw/cut5/film.mp4`: 35.0 s, 1,050 frames, 1280x720 at 30 fps, 32 MB; 39.7 min wall, the build already cached (`raw/cut5-render.log`, 16:33 to 17:13). No frame refused. Walks on the direct build, as cut 4.

| segment | frames | trees | build | draw | peak resident | device |
|---|--:|--:|--:|--:|--:|--:|
| 0 beech grown 1-40 | 210 | 40 | 8.4 s | 7.4 s | 818 MB | 1,519 MB |
| 1 beech to spruce | 90 | 90 | 272.3 s | 10.5 s | 2,232 MB | 1,583 MB |
| 2 spruce to birch | 90 | 90 | 381.6 s | 12.5 s | 2,248 MB | 1,583 MB |
| 3 birch to oak | 90 | 90 | 156.5 s | 8.8 s | 1,808 MB | 1,327 MB |
| 4 oak to palm | 120 | 120 | 46.2 s | 8.5 s | 1,799 MB | 1,327 MB |
| 5 palm 40 to 1 | 90 | 39 | 1.8 s | 4.8 s | 1,705 MB | 1,329 MB |
| 6 palm to Laurelin at 1 | 30 | 30 | 0.1 s | 0.5 s | 1,705 MB | 1,073 MB |
| 7 Laurelin grown 1-80 | 210 | 80 | 10.7 s | 7.9 s | 1,711 MB | 4,628 MB |
| 8 Laurelin to Telperion | 120 | 90 | 1,353.2 s | 21.5 s | 4,643 MB | 2,615 MB |

**Oracle** (`telperion-heavy 8 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json --oracle`, `raw/oracle-cut5.log`, 24 min): 481 of 510 walk frames meet it; "29 walk frames miss the oracle". Every frame of beech to spruce, spruce to birch, oak to palm (120/120; cut 4 missed 6, fn-273's tenth exception; the band's floor is now 480 kept), palm to Laurelin and Laurelin to Telperion. Birch to oak 61/90 (cut 4: 65/90): kept 537,078 to 685,562 against a ceiling of 487,001 from t = 0.20 to 0.9985, up to 40.8 % over (cut 4: up to 24.6 % over a ceiling of 535,701), grown above its ceiling of 1,278,945 at ten of them. fn-251's named exception covers this walk; it measured up to 21 % over. For the host: the band is narrower and the overshoot larger than at cut 4.

**Oak to palm, `frames.tsv`:** the oak's leaves fall away over the first fifth: 71,438 at t = 0, 63,598 at 0.024, 10,105 at 0.081, 2,346 at 0.27, 1,964 at 0.39, then up to 3,639 at 0.51, 3,250 at 0.75 and 6,554 at the palm. At cut 4, measured by the probe, frame 540 held 351 leaves and frames 515 to 520 held 41 to 59.

**Frames viewed:**

- **520** (t = 0.27, 2,346 leaves): an oak-like bare branch skeleton on a straight stem, small leaves spread thinly through it. A few leaves sit above and beside the crown's outer twigs with no wood visible under them. A sparse scatter, not the halo of single leaves that cut 4 had at this point.
- **540** (t = 0.51, 3,639 leaves): a dense mop of narrow leaves around fronds that radiate from the stem's apex, on a palm-barked stem. The crown reads as one mass. At its top edge a few narrow leaves stand apart from the rest. No cloud of isolated leaves round the crown.
- **560** (t = 0.75): a palm. Fronds with brown rachis lines drawn from the apex, leaflets along them, and a palm-barked stem.
- **575** (t = 0.90): a full palm crown, rachises visible, leaflets dense along them.
- Cut 4's flying leaves are not there in 540 or 560. In 520 a thin scatter of leaves stands off the oak's twigs.
- **105** (beech, age label 21): a young slender beech with layered laterals and leaves clustered at the top and on the lower limbs. The scale figure stands large in the foreground.
- **255** (beech to spruce, t = 0.52): a conical conifer with dense needles to the ground and drooping lower limbs.
- **345** (spruce to birch, t = 0.52): a tall narrow tree with weeping fine branches and a pale stem. The top touches the frame's top edge.
- **435** (birch to oak, t = 0.52): a leaning brown stem dividing into white-barked limbs, a spreading fine-twigged crown and bare low branches.
- **645** (palm shrinking, age 20): a broad rosette of fronds with brown rachises fills the frame's middle. The stem and the scale figure are mostly hidden behind the lower fronds.
- **705** (palm to Laurelin at 1, t = 0.55): the scale figure fills the left of the foreground and a small seedling of a few narrow leaves stands beside it.
- **825** (Laurelin, age label 41): a round crown of green leaves and hanging orange-gold flower clusters on a dark stem. A small tuft of leaves and flowers hangs off the stem below the crown.
- **975** (Laurelin to Telperion, t = 0.52, age 80): a broad spreading crown of pale silver-white blossom over dark limbs, the scale figure small at the foot.
- The map dot moves along the segment between its ends' dots in every walk frame viewed.

## Cut 6 (2026-10-23, fn-309 merged)

At c4bc857c2 (1d34ddd3c merged the release branch with fn-309: a compound leaf never spreads along a rachis nothing draws). Same command as cut 5, into `raw/cut6`: `telperion-heavy 8 --gpu 4 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json .flow/evidence/fn-272-the-release-film-through-species-space/raw/cut6`. The video is kept at `/home/daniel/Videos/telperion/cut6.mp4` (31,324,321 bytes, the same bytes as `raw/cut6/film.mp4`).

`raw/cut6/film.mp4`: 35.0 s, 1,050 frames, 1280x720 at 30 fps, 31 MB; 36.2 min wall including an 18 s rebuild (`raw/cut6-render.log`, 01:08 to 01:44). No frame refused.

| segment | frames | trees | build | draw | peak resident | device |
|---|--:|--:|--:|--:|--:|--:|
| 0 beech grown 1-40 | 210 | 40 | 8.1 s | 7.3 s | 836 MB | 1,263 MB |
| 1 beech to spruce | 90 | 90 | 257.3 s | 9.9 s | 2,349 MB | 1,775 MB |
| 2 spruce to birch | 90 | 90 | 340.6 s | 11.1 s | 2,349 MB | 1,519 MB |
| 3 birch to oak | 90 | 90 | 141.7 s | 8.1 s | 1,892 MB | 1,263 MB |
| 4 oak to palm | 120 | 120 | 43.3 s | 7.0 s | 1,887 MB | 1,263 MB |
| 5 palm 40 to 1 | 90 | 39 | 1.7 s | 4.7 s | 1,710 MB | 1,263 MB |
| 6 palm to Laurelin at 1 | 30 | 30 | 0.1 s | 0.5 s | 1,710 MB | 1,007 MB |
| 7 Laurelin grown 1-80 | 210 | 80 | 10.4 s | 7.8 s | 1,718 MB | 4,228 MB |
| 8 Laurelin to Telperion | 120 | 90 | 1,214.7 s | 20.3 s | 4,918 MB | 2,890 MB |

**Oracle** (`telperion-heavy 8 --threads 4`, `raw/oracle-cut6.log`, 22.5 min): 481 of 510, "29 walk frames miss the oracle". Every walk frame's kept and grown counts are the same as cut 5's. Birch to oak 61/90: kept 537,078 to 685,562 against the ceiling of 487,001 from t = 0.20 to 0.9985, up to 40.8 % over, the same as cut 5; grown above 1,278,945 at ten frames. Every other walk meets it on every frame.

**Oak to palm, `frames.tsv`:** leaf counts are the same as cut 5's at every frame viewed; the bounds are smaller (frame 536: 13.60 m tall and 7.46 m wide, cut 5 15.36 m and 8.26 m; frame 526: 12.00 m and 6.71 m, cut 5 13.46 m and 8.48 m).

**Frames 506, 516, 526 and 536, each viewed whole and its crown enlarged three times:**

- **506** (t = 0.13, 3,936 leaves): a bare oak skeleton; the leaves sit in clumps among the twigs of the two upper limbs. At the crown's left edge a handful of single dark leaves sit at fine twig tips. Enlarged, a twig runs to each, but at full size the twigs are hairlines and those leaves read as dark specks just off the crown. No leaves on the lower limbs.
- **516** (t = 0.23, 4,292 leaves): the same; the leaves are clumped in the upper crown among twigs. A few single leaves stand at the crown's edge: a pale green one at the top left beside the twig tips with no twig visibly under it, and dark ones at the right edge at twig ends. One dark leaf sits beside the stem below the crown.
- **526** (t = 0.34, 860 leaves): leaves do stand off the tree. A star of bare frond stems radiates from the apex, with a small tuft of leaves above it. Above the tuft five or six single dark leaves stand apart against the sky; some are at the ends of hairline twigs, and two or three have no wood visible under them even enlarged. Below the apex, beside the stem, two small pale tan leaves and a faint grey-green one hang with nothing under them. Near the ground a thin bent bare stick stands out to the right of the stem.
- **536** (t = 0.46, 2,396 leaves): a palm-like star of drawn frond stems with two leaf tufts, one above the apex and one hanging below it down the stem. Along the bare fronds sit single dark leaflets, about six, each alone on its frond, spaced far apart. They are on drawn wood but read as lone leaves on bare stalks. Below the apex two small pale tan leaves stand beside the stem with nothing visibly under them.
- In short, some leaves stand off the tree in all four frames. In 506 and 516 they are a few edge leaves on hairline twigs. In 526 and 536 a few leaves have no wood visible under them, and in 536 the leaflets stand singly on otherwise bare fronds.

**Birch to oak at 30, 50 and 70 % (frames 417, 435, 453) against its ends (390 birch, 479 oak):** heights (`frames.tsv`) fall steadily, 16.5 m (birch), 15.9, 14.8, 12.7, 11.2 m (oak). Widths 8.2 m (birch), 8.9, 10.4, 8.2, 7.5 m (oak), so the crown at 50 % is wider than either end, 27 % wider than the birch and 39 % wider than the oak. Leaves 162,736 (birch), 99,073, 64,548, 50,034, 72,112 (oak), so at 70 % there are fewer than at either end. Seen (the camera closes in over the segment, so on-screen sizes are not comparable): 390 a weeping white birch; 417 a narrower upright birch crown, still white-stemmed; 435 a broad open fine-twigged crown on white limbs over a leaning brown stem with bare low branches; 453 a rounder, denser crown on a darker stem, shorter; 479 the oak's dense dark-green crown on a thick stem.
