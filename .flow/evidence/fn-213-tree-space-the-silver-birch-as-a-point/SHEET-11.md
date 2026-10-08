# fn-213 sheet 11: round 11, the leaves (worker, 2026-10-08)

The owner's verdict on round 10 (2026-10-08): "structurally the birch looks almost right though the leaves look strange? weird color and are they small and numerous to appear like they just fill the twigs length."

Round 11 changes the leaf rows only: colour, size, shape, and leaves per metre. No wood row moved, and node counts are identical to round 10's.

## What moved (against round 10, `bdfe3ff7`)

| Row | Round 10 | Round 11 | Source |
|---|---|---|---|
| `/skeleton/twigs/twig/internodeLength` (leaf station spacing) | 0.012 m | 0.06 m | One leaf a node of the strand's 0.06 m internode: 4 to 6 per year's growth, inside the 3 to 7 leaves a long shoot bears (SU03) |
| `/element/length`, `width` | 0.065, 0.052 m | 0.05, 0.037 m | Flora of China, *Betula pendula*: "leaf blade triangular-ovate or rhombic-ovate, 3-7.5 × 1.5-6 cm" (`raw/lit/foc-betula-pendula.txt`); the middles of both ranges |
| `/canopy/sizeVariation` | 0.2 | 0.3 | 35 to 65 mm, inside FoC's 30 to 75 mm |
| `/element/connectorLength` (petiole) | 0.022 m | 0.025 m | FoC: "Petiole slender, 2-3 cm" |
| `/element/tipSharpness` | 1.1 | 1.5 | FoC: "apex acuminate or caudate-acuminate". `widestAt` 0.2 and `baseFullness` 0.3 stay: widest near a truncate base (FoC: "base truncate, broadly cuneate"), which is triangular |
| `/material/leafFront*` | 0.16 / 0.26 / 0.035 | 0.134 / 0.145 / 0.066 | Measured (below) |
| `/material/leafBack*` | 0.25 / 0.33 / 0.09 | 0.134 / 0.145 / 0.066 | The same measured colour (below) |
| `/material/transmission*` | 0.28 / 0.55 / 0.10 (blue the default) | 0.134 / 0.145 / 0.066 | The same measured colour (below) |

`catalogue_identity.rs`'s three preset digests are re-pinned for these rows, as in every earlier round.

## The colour, measured (`leaf_colour.py`; output in `raw/leaf-colour.txt`)

Leaf pixels are picked by a mask: a region, green above red and blue, and on the close-ups either in focus or lit above the black ground. The masks are in `raw/sheet11/s2-oslo-mask.png` and `s5-hanging-mask.png`. Each pixel is converted from sRGB to linear, then averaged.

| Photograph | Leaf pixels | Linear mean | R:G:B |
|---|--:|---|---|
| S2-Oslo sunlit crown (August) | 596,290 | 0.149 / 0.160 / 0.068 | 0.93 : 1 : 0.42 |
| the same, on the exposure of its sunlit white stem | | **0.134 / 0.145 / 0.066** | |
| S5-hanging spray, in focus (August) | 599,151 | 0.359 / 0.423 / 0.265 | 0.85 : 1 : 0.63 |
| S2-Uusimaa crown (late September, yellowing) | 173,695 | 0.147 / 0.152 / 0.117 | 0.97 : 1 : 0.77 |
| S5-short-shoots (April, young leaves) | 710,142 | 0.231 / 0.302 / 0.024 | 0.76 : 1 : 0.08 |

The rows take the S2-Oslo value on its stem's exposure. Its sunlit stem reads 0.866 / 0.839 / 0.719 linear; the crown is scaled by the bark rows' 0.78 / 0.76 / 0.70 over that reading. The others were left out for these reasons:

- **S5-hanging:** the close-up is exposed for its bokeh and has no white reference. It reads 0.42 green, past the 0.2 at which colour rows clip.
- **S2-Uusimaa:** yellowing in mist (the README's caveat).
- **S5-short-shoots:** spring flush. Its 0.76:1:0.08 is close to round 10's hue (0.62:1:0.13), which is why round 10 read as a bright lime.

**Back and transmission:** no reference shows an underside or a backlit leaf that code or the worker could tell apart. A k-means split of S5-hanging's leaf pixels separates them by lighting, not by face. FoC says of the underside only "abaxially densely resinous punctate". So both rows take the measured leaf colour. Round 10's back (0.33 green) was above the clip line.

**Rendered, the same mask on the stills** (`raw/leaf-colour-rendered.txt`):

| Still | Linear mean | R:G:B |
|---|---|---|
| R10 crown @40 / @80 (seed 1) | 0.228/0.323/0.116, 0.271/0.372/0.125 | 0.71:1:0.36, 0.73:1:0.34 |
| R11 crown @40 / @80 (seed 1) | 0.207/0.230/0.142, 0.203/0.223/0.138 | 0.90:1:0.62, 0.91:1:0.62 |
| S2-Oslo crown (photo) | 0.149/0.160/0.068 | 0.93:1:0.42 |

R11's red-to-green now matches the photograph. Its blue is higher (0.62 against 0.42), and the stills read grey-olive.

## Measures (`birch_measure.rs`, seed 1; raw in `raw/measure-r10-leaves.txt`, `measure-r11-leaves.txt`)

| Measure | R10 @40 | R11 @40 | R10 @80 | R11 @80 |
|---|--:|--:|--:|--:|
| Leaves | 297,544 | 81,257 | 2,480,185 | 633,342 |
| Leaves per metre of hanging strand | 78.4 | 22.1 | 82.6 | 22.6 |
| Leaves per short shoot (runs < 5 cm), nearest | 2.27 | 0.23 | 2.38 | 0.30 |
| Leaf blade length p10 / p50 / p90, mm | 55 / 65 / 75 | 38 / 50 / 62 | 55 / 65 / 75 | 38 / 50 / 62 |
| Blade width p50 / petiole p50, mm | 52 / 22 | 37 / 25 | 52 / 22 | 37 / 25 |
| Leaf colour, linear: front | 0.160/0.260/0.035 | 0.134/0.145/0.066 | same | same |
| back | 0.250/0.330/0.090 | 0.134/0.145/0.066 | same | same |
| transmission (at strength 0.6) | 0.280/0.550/0.100 | 0.134/0.145/0.066 | same | same |

**Leaves per short shoot.** The 0.23 is an artefact of the measure:

- A run's first station sits at its base (`station.rs`), and a run under 6 cm gets exactly one station.
- So a short shoot's one leaf now sits where it leaves the strand, and the measure counts it on the strand.
- SU03's two or three leaves at the short shoot's tip are fn-223's clustering. They are not faked here.

## Cost (native, `--profile ci`, limits 8; `raw/sheet11/render.log` against `raw/sheet10/render.log`)

| | Round 10 | Round 11 |
|---|--:|--:|
| Nodes at 40, seeds 1 / 7 / 42 | 280,885 / 535,595 / 335,228 | the same |
| Nodes at 80, seeds 1 / 7 / 42 | 1,845,346 / 2,155,356 / 2,108,348 | the same |
| Leaves at 40 | 297,544 / 539,471 / 347,936 | 81,257 / 158,268 / 98,811 |
| Leaves at 80 | 2.48M / 2.68M / 2.79M | 633,342 / 698,262 / 714,250 |
| Dressed at 40, ms | 299 / 607 / 359 | 168 / 322 / 174 |
| Dressed at 80, ms | 2,468 / 2,807 / 2,714 | 1,161 / 1,337 / 1,360 |

Leaves fall to about a quarter, and dressing takes about half the time.

## The stills

- **Compare sheets:** `raw/compare/compare11-40.png` and `compare11-80.png` (`compare_sheet.sh 10 11`). For each of seeds 1, 7 and 42 there are three rows: the old birch, round 10 and round 11 (bare, in leaf, crown, limb).
- **Leaf close-ups:** `raw/compare/leaf11.png`. S5-hanging and S2-Oslo crops sit beside round 10 and round 11 at 40 and 80 (seed 1). The camera is 0.9 m off the outermost leaf at mid-crown, from the hero camera's side; the tool is `leaf_closeup.rs`.
- **Round 11's own stills:** `raw/sheet11/`; the spray and base shots are in `raw/sheet11/rest.png`.

The worker viewed every still.

**The worker's reading.**

- **Close-ups:** round 10's strands were dressed like fern fronds, a leaf every 12 mm on alternate sides. Round 11's leaves stand one by one along the strands, triangular, with a visible stalk and a pointed tip.
- **Colour:** grey-olive. It is greyer and duller than S2-Oslo's crown, which reads a yellow-green because sunlit highlights sit among dark gaps.
- **Distant views (in leaf, crown):** round 11 reads nearly as bare as the bare view at both ages. The strands and limbs dominate, the leaf mass of S2-Oslo is gone, and the crown is see-through. Round 10 was a dense lime mass.
- **Structure:** unchanged.

## For the host and the owner

- **Leaf mass.** The leaf count follows the rows: about 22 a metre of strand, one a node, by SU03. At that count, and with no clustering, the crown does not read as S2-Oslo's leafy mass. The leaves real birches carry on short shoots are fn-223, so the engine places one leaf where the photograph shows a cluster of two or three. Whether to raise the count by value meanwhile is the host's call; that would be the evenly lined strand again.
- **Colour.**
  - **Method:** the rows are the photograph's mean leaf pixel, as asked. That mean includes the leaves' own shade, and the renderer shades again, so the rendered crown reads greyer than the photograph. Its rendered green, 0.23, is above the photograph's 0.16, and its blue is about 1.5 times the photograph's.
  - **What the measure cannot settle:** whether the rows should be the sunlit pixels' colour instead. That is a method question for the host.
  - **Back and transmission:** no reference separates the underside or the backlit leaf, so both rows take the front's colour.
- **Size.** The blade is now 50 mm, smaller than round 10's 65. It is botanical (FoC, JRC); the owner's "small" read the fern-like rows.

## Gate (round 11)

- **Render half first, under the GPU lock:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0.
- **Then the workspace:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0 after the digest re-pin.
- **npm test:** 13 files, 143 tests passing.
- **Logs:** limits at 8; logs in the ignored `raw/gate11-*.log`.
