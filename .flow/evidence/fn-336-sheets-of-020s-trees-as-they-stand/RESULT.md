# fn-336 result: sheets of 0.2.0's trees as they stand

Nineteen retained JPEG sheets beside this file, 4.1 MiB in all: eighteen of the six real presets (three views each) and one of the old oak. Every tree is the full cycle: no preset sets `/fidelity`, so each grows at 0 and the tuned `/space/coarse` rows stay dormant. Base `a755efe26`. No rule, row, value or crate changed.

## How they were made

- Stills: `probes/stills.sh`, one serial GPU job (`telperion-heavy 10 --gpu 2 --threads 4`), one process per tree through fn-330's `peak.py`. It draws with the space still examples (`crates/telperion-render/examples/space_<species>.rs`, `space/still.rs`): 960x720, sun at azimuth 115 and elevation 60, camera fitted to each tree. Raw PNGs and logs are in ignored `raw/`.
- Old-oak numbers: fn-332's probe (`crates/telperion-core/examples/trunk.rs`) in a separate CPU-only job (`telperion-heavy 8 --threads 4`), logs in `raw/trunk/`.
- Sheets: `probes/sheets.py` (ImageMagick), at most 2,000 px wide, JPEG quality 82, metadata stripped.

## Views (worker's mapping, open to the host)

The space still example names seven shots. The sheets use four of them:

- **Whole:** `whole`, the hero pose.
- **Mid:** `crown`, the upper crown in leaf, from three quarters of the tree's height away.
- **Close:** two rows. `spray` is a limb from 9 m in leaf, and `low` is the trunk base from 6 m in leaf.

Reference photographs from `.flow/references/` stand on the first row of the matching view: S1 and S2 on whole, S3 on mid, S4 and S5 on close. The palm's references are P-WHOLE (whole) and P-TRUNK and P-BASE (close). Its mid sheet says the repository has no reference at that scale. The ordinary tree's three sheets say there are none at all. The oak's set is mostly *Q. robur* standing in for *Q. garryana* (its README).

**The grove is not drawn.** The voxel style exists only as `src/field/voxelize.ts`, which "draws nothing", and no example under `crates/*/examples` renders cubes. Per the dispatch and R6, nothing was built. FRICTION.md names what would remove the gap.

## The sheets

| Sheet | Holds |
|---|---|
| `sheet-oak-whole.jpg`, `sheet-oak-mid.jpg`, `sheet-oak-close.jpg` | Oregon white oak, age 40, seeds 1, 7, 42, with its references |
| `sheet-spruce-whole.jpg`, `-mid`, `-close` | Norway spruce, age 40 |
| `sheet-beech-whole.jpg`, `-mid`, `-close` | European beech, age 40 |
| `sheet-birch-whole.jpg`, `-mid`, `-close` | Silver birch, age 40 |
| `sheet-palm-whole.jpg`, `-mid`, `-close` | Date palm, age 40 |
| `sheet-ordinary-whole.jpg`, `-mid`, `-close` | The ordinary tree, age 40, no references |
| `sheet-oak-old-whole.jpg` | The oak at 120, 200 and 300, seeds 1, 7, 42, each with fn-332's probe printed under it and the known faults stated |

## Seen before kept (R3)

The worker viewed every sheet. None is empty or unlit, and none is mislabelled. On the first old-oak sheet, two stills were clipped: at seed 7 the hero pose cut off the trunk base at age 300 and grazed the flare at age 200. Both were remade with the existing headless still: the age laid over the preset (`--family`, `{"age": N}`), the same size and sun, and `--camera {"fill":0.72,"targetHeight":0.45}`. Each remake's leaf count matches the space example's (526,125 and 542,997), so it is the same tree. Both are labelled "camera pulled back" on the sheet. No other still was remade.

## Screening notes (observations, never a verdict)

- **Oak, 40:** The crowns are narrow and upright, taller than wide. The references are wider than tall, with a short bole. The foliage reads dark. A seed-42 limb hangs low beside the trunk. The base close-up shows a horizontal band in the bark about 1 m up (seeds 1 and 7) and a flare like a pedestal.
- **Spruce, 40:** The trees are conical, with a single straight leader. The branches are sparse and tiered, with see-through gaps near the top. The references are denser and fuller. At the base, low branches arch down to the ground.
- **Beech, 40:** The trees are lollipop-shaped: a bare, straight stem 4 to 5 m high under a round, compact crown. The references have a short bole and a broad, spreading crown. The leaves read olive-brown.
- **Birch, 40:** The stems are white and some lean or curve (seeds 1 and 7). The crowns are narrow and weeping, with long hanging twigs and a pale, sparse leaf. At the base, some twigs hang down to near the ground.
- **Palm, 40:** The trees are 21.5 m tall with a rosette of about thirty fronds and a straight trunk. The trunk shows a regular diamond lattice of leaf bases, and its foot ends in a smooth, rounded bulb. The fronds read nearly black on their shaded side. The three seeds look nearly identical (7,605 nodes each).
- **Ordinary, 40:** The trees are upright and open, with sparse bright-green leaves on thin, straight twigs.
- **Old oak:** At 200 and 300 the trunks bend near horizontal. At 300, seed 42 loops back on itself near the ground. The crowns sit 3 to 20 m off the base (probe). At 120, seed 7 carries long bare upturned whips on one side. Heights at 300 are 35.6 to 36.7 m.

## Time and memory per tree (R5)

Grow is `executor::grow` with expansion, rayon at 4 threads. Wall and peak are the whole process (grow, dress, GPU, every shot), from `peak.py`. One desk with an RTX 3080, beside the owner's session.

| Tree | Seed | Grow ms | Nodes | Leaves | Height m | Wall s | Peak GiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| oak 40 | 1 | 717 | 275,902 | 45,085 | 10.7 | 5.1 | 0.73 |
| oak 40 | 7 | 1,216 | 384,593 | 72,112 | 11.2 | 2.3 | 0.82 |
| oak 40 | 42 | 839 | 285,930 | 47,577 | 10.1 | 1.8 | 0.71 |
| spruce 40 | 1 | 3,327 | 994,902 | 4,024,568 | 11.7 | 13.0 | 1.91 |
| spruce 40 | 7 | 3,047 | 966,173 | 3,925,480 | 11.9 | 9.3 | 1.82 |
| spruce 40 | 42 | 3,310 | 1,005,581 | 4,084,402 | 11.7 | 9.7 | 1.91 |
| beech 40 | 1 | 206 | 114,700 | 33,427 | 10.1 | 4.2 | 0.47 |
| beech 40 | 7 | 309 | 205,400 | 66,854 | 10.5 | 1.2 | 0.55 |
| beech 40 | 42 | 394 | 251,816 | 76,640 | 10.0 | 1.3 | 0.62 |
| birch 40 | 1 | 552 | 242,450 | 112,288 | 16.6 | 4.7 | 0.65 |
| birch 40 | 7 | 1,109 | 355,645 | 163,502 | 16.5 | 2.1 | 0.79 |
| birch 40 | 42 | 712 | 300,786 | 137,681 | 16.6 | 1.6 | 0.71 |
| palm 40 | 1 | 2.8 | 7,605 | 6,554 | 21.5 | 3.9 | 0.35 |
| palm 40 | 7 | 3.5 | 7,605 | 6,554 | 21.7 | 0.7 | 0.29 |
| palm 40 | 42 | 3.3 | 7,605 | 6,554 | 21.7 | 0.7 | 0.29 |
| ordinary 40 | 1 | 239 | 86,361 | 7,849 | 12.5 | 3.6 | 0.43 |
| ordinary 40 | 7 | 400 | 130,355 | 11,703 | 13.2 | 1.2 | 0.45 |
| ordinary 40 | 42 | 409 | 152,178 | 13,780 | 12.4 | 1.2 | 0.49 |
| oak 120 | 1 | 28,447 | 1,534,716 | 271,551 | 23.4 | 30.2 | 2.77 |
| oak 120 | 7 | 56,823 | 3,508,486 | 632,587 | 29.2 | 60.2 | 5.93 |
| oak 120 | 42 | 24,857 | 2,476,044 | 510,413 | 27.1 | 27.4 | 3.87 |
| oak 200 | 1 | 60,069 | 1,281,121 | 205,559 | 25.0 | 61.7 | 2.55 |
| oak 200 | 7 | 126,456 | 3,092,882 | 542,997 | 33.2 | 129.5 | 5.81 |
| oak 200 | 42 | 57,989 | 1,042,776 | 180,577 | 23.8 | 59.5 | 3.21 |
| oak 300 | 1 | 76,987 | 1,006,957 | 159,161 | 36.9 | 78.4 | 2.29 |
| oak 300 | 7 | 147,412 | 2,588,952 | 526,125 | 35.8 | 150.1 | 5.07 |
| oak 300 | 42 | 75,137 | 1,710,009 | 308,866 | 36.0 | 77.0 | 3.38 |

Each species' first wall time carries the process's first GPU start. Height is the dressed mesh's bounds, leaves included. The remade stills (headless, the whole tree built once more) took 110.1 s and 5.25 GiB at 200 and 145.4 s and 4.63 GiB at 300.

Trunk probe, CPU only, grow with census, wall s and peak GiB: 120: 26.0/2.11, 55.5/4.60, 22.2/2.06; 200: 54.9/2.00, 124.2/4.97, 59.3/3.11; 300: 77.2/2.01, 174.2/4.36, 90.3/2.74 (seeds 1, 7, 42). Seed 7 needs about twice seed 1's time and memory at every age.

| Old oak | Seed | Top m | Trunk path ends: high m | out m | Crown mean off base m |
|---|---:|---:|---:|---:|---:|
| 120 | 1 | 23.24 | 15.11 | 10.34 | 8.97 |
| 120 | 7 | 29.08 | 7.02 | 7.84 | 3.87 |
| 120 | 42 | 26.93 | 16.77 | 14.57 | 3.23 |
| 200 | 1 | 24.89 | 22.87 | 15.65 | 15.65 |
| 200 | 7 | 32.99 | 9.12 | 12.43 | 8.52 |
| 200 | 42 | 23.63 | 18.03 | 5.70 | 11.91 |
| 300 | 1 | 36.74 | 33.53 | 17.40 | 19.73 |
| 300 | 7 | 35.57 | 18.93 | 14.44 | 12.47 |
| 300 | 42 | 35.78 | 28.89 | 5.66 | 6.16 |

## R4: the question for the owner (draft for the goal's log)

What reads wrong on each sheet, in your own words? One line per sheet is enough. Each sheet's path is under `.flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand/`:

- Oak, whole / mid / close: `sheet-oak-whole.jpg`, `sheet-oak-mid.jpg`, `sheet-oak-close.jpg`
- Spruce, whole / mid / close: `sheet-spruce-whole.jpg`, `sheet-spruce-mid.jpg`, `sheet-spruce-close.jpg`
- Beech, whole / mid / close: `sheet-beech-whole.jpg`, `sheet-beech-mid.jpg`, `sheet-beech-close.jpg`
- Birch, whole / mid / close: `sheet-birch-whole.jpg`, `sheet-birch-mid.jpg`, `sheet-birch-close.jpg`
- Date palm, whole / mid / close: `sheet-palm-whole.jpg`, `sheet-palm-mid.jpg`, `sheet-palm-close.jpg`
- Ordinary tree, whole / mid / close: `sheet-ordinary-whole.jpg`, `sheet-ordinary-mid.jpg`, `sheet-ordinary-close.jpg`
- The old oak at 120, 200, 300: `sheet-oak-old-whole.jpg`

The grove on the website's voxel style is not on any sheet: no headless entry draws it yet.
