# FN34: beech, ash and birch as real species

2026-09-14. Packets, value tables and the fixed/fresh numeric protocol for
European beech and silver birch. European ash has a complete profile and
reference set; its template waits for fn-33. Visual verdicts are unassessed.
`owner_feedback` is null. This report does not award a visual pass.

Raw 24-seed receipts live on disk under `.flow/evidence/fn34/measure/`
(about 106 MB) and are not committed. The compact table is
`numeric-summary.json`. Reference rasters are under ignored `.refs/fn34/`.

## What changed

Two catalogue presets, `european-beech` and `silver-birch`, are value tables
in `crates/telperion-core/src/presets/species.rs`. Materials sit beside the
oak and spruce rows. Growth traits are the oak's; age is 120 yr (beech,
32 m envelope) and 70 yr (birch, 18 m envelope) so the profile's mature
reference height is the envelope. No generator or renderer code changed.

Ash is gathered in `.flow/evidence/fn34/european-ash/`. No ash preset
exists in the source; the template is written once fn-33 ships compound
leaves. A simple-blade ash is not a pass (R4).

## Numeric protocol

Seeds drawn once with `--draw-seeds` to `.flow/evidence/fn34/seeds.json`
at commit `3468ef710a6f67009c912ffd8c6f7076c5f3771b`, before the full
48-case run. Fixed list is the fn-9 twelve. Fresh seeds were not resampled.

`npm run species:measure -- --profiles .flow/evidence/fn34/profiles.json --seeds .flow/evidence/fn34/seeds.json --output .flow/evidence/fn34/measure`

Capture half (`npm run species:qa -- --capture-only` on the same profiles, seeds and output): every required still wrote a PNG. The runner exited 1 with every `visual_status` unassessed, which is the designed exit. Extra protocol stills (leaf views, first three fresh seeds, ordinary/telperion/laurelin at seed 1) stay under ignored `measure/` and are not the judging set.

| Species | Cases | Numeric | Height m | DBH m (proxy) | Crown width m | Retained failures |
|---|---|---|---|---|---|---|
| european-beech | 24 | 24 pass | 29.98–31.85 | 0.874 | 31.23–32.03 | none |
| silver-birch | 24 | 24 pass | 14.09–15.45 | 0.345 | 12.48–13.72 | none |
| european-ash | not run | unready | — | — | — | waits for fn-33 |

Gating metrics that passed on every seed: height, foliage length, foliage
width; beech also gates DBH. Crown width and crown-base stay contextual.

## Stills

Owner judging set: fixed seeds 1, 2, 3; views `whole` and `bare`; 960×720;
headless on an RTX 3080. Files in `.flow/evidence/fn34/stills/`, hashes in
`stills.json`. Every `visual_status` is `unassessed`. `owner_feedback` is
null. The implementer did not open the rasters (four-images reading rule).

| File | Preset | Seed | View |
|---|---|---|---|
| stills/european-beech-1-whole.png | european-beech | 1 | whole |
| stills/european-beech-1-bare.png | european-beech | 1 | bare |
| stills/european-beech-2-whole.png | european-beech | 2 | whole |
| stills/european-beech-2-bare.png | european-beech | 2 | bare |
| stills/european-beech-3-whole.png | european-beech | 3 | whole |
| stills/european-beech-3-bare.png | european-beech | 3 | bare |
| stills/silver-birch-1-whole.png | silver-birch | 1 | whole |
| stills/silver-birch-1-bare.png | silver-birch | 1 | bare |
| stills/silver-birch-2-whole.png | silver-birch | 2 | whole |
| stills/silver-birch-2-bare.png | silver-birch | 2 | bare |
| stills/silver-birch-3-whole.png | silver-birch | 3 | whole |
| stills/silver-birch-3-bare.png | silver-birch | 3 | bare |

## Ash

Profile, references and a closed-schema `species.json` are complete.
Required capability `pinnate-compound` is unmet. The fixture is not a
catalogue species and was not measured or rendered as a passing tree.

## Undecided / owner

- Visual verdicts on the twelve stills (R3). The spec closes only on
  accepting owner verdicts.
- Ash template after fn-33 lands: leaflet count 7–13, whole-leaf length
  0.20–0.30 m, placements vs instances.
- Growth-trait calibration through time waits for fn-30's method. Height
  by age is recorded on each ready profile.
- `tests/crown_reference.rs` is an ignored FN6 pin list and does not
  include oak or spruce; it was not extended.

## Round 2: matched stills (2026-09-14, fn-36)

The owner rejected round 1 because the fixed views made the comparison unfair
before it was made. Round 2 renders the first fixed seed of each species
through the camera, sun, foliage state and aspect recorded on every whole,
bare and base reference (the `shot` block in `references.json`), without the
scale figure, and pairs each still with its photograph at one height. The
compare script measures both images the same way. Pairs and stills stay on
disk under the ignored `measure/pairs/` directory; `round2/stills.json` records
each by sha256 and `round2/*-compare.json` carries the numbers.

Photograph / still, per reference:

| Reference | Foliage | Width over height | Crown base | Fill | Occupied | Centre order |
|---|---|---|---|---|---|---|
| B-BARE | hidden | 0.74 / 0.69 | 0.25 / 0.26 | 0.90 / 0.96 | 0.28 / 0.31 | RGB / BGR |
| B-BASE | hidden | 0.60 / 0.67 | 1.00 / 0.04 | 1.00 / 1.00 | 0.05 / 0.10 | BRG / BGR |
| B-WHOLE | leaf-on | 0.70 / 0.69 | 0.12 / 0.09 | 0.91 / 0.96 | 0.46 / 0.55 | GBR / GBR |
| S-BARE | hidden | 0.70 / 0.72 | 0.15 / 0.25 | 0.95 / 0.93 | 0.46 / 0.22 | RGB / BGR |
| S-BARK | hidden | 1.07 / 0.90 | 1.00 / 0.00 | 1.00 / 1.00 | 0.47 / 0.43 | GRB / GRB |
| S-WHOLE | leaf-on | 0.85 / 0.88 | 0.08 / 0.21 | 0.96 / 0.94 | 0.44 / 0.40 | GRB / BGR |

How to read the columns: width over height and crown base come from the
photograph's tree box and crown base read by eye, and from the still's tree
mask (what stands out of the row background under the shot's sun and under
the twin's horizon sun). Fill is the tree's height over the frame's; the
still overfills by a few percent because the camera frames the crown's
inscribed ellipsoid, not its silhouette. Occupied is the dark-pixel share of
the photograph's box against the mask's share of the still's, so it is a
density proxy that the photograph's background contaminates on the bare and
base shots. Centre order is the fn-29 channel order of the tree's middle.

What the pairs show, read by the host on the two whole pairs (the four-image
rule): the framing now agrees to within a few percent on proportion, so what
is left is the tree. The beech's crown reads as a scatter of small leaves on
visible straight branches where the photograph is one dense continuous mass;
its trunk is thicker and shorter than the beech's and reads blue-grey under
the overcast sky. The birch is a single upright stem with a rounded crown
where the photograph is two leaning stems with a weeping curtain of long
shoots reaching almost to the ground; the crown base measures 0.21 against
the photograph's 0.08 for that reason. Neither species has the density or
the habit of its photograph yet. No visual pass is awarded here; the owner
judges the pairs.

## Round 3: value tables tuned against the pairs (2026-09-14)

The owner asked for the easy flaws fixed before judging again. The host tuned
the two value tables in two passes, reading one whole pair per species per
pass, and left the engine alone. Beech: laterals at 1.4 m instead of 1.8, a
longer lateral ratio, five twig laterals, a fuller and slightly wider
envelope (fullness 0.62, spread 0.52) with crookedness 14, an 8 by 5 cm blade,
a smaller root flare, a warmer bark row and less interior darkening. Birch:
secondary rise at -0.85 with a 62 degree lateral pitch, the crown base at a
tenth of the height, eight twig laterals with longer thinner twigs, and
foliage leaning forward and down in clumps of eight. Two overshoots were
walked back: a beech dense enough to hit the node cap, and blades on both
species that left their profile's length gate. The variation regression for
the beech now judges whichever crown dimension the seeds move more, because a
crown that fills its envelope pins both.

Photograph / round 2 → round 3, per reference:

| Reference | Foliage | Width over height | Crown base | Occupied | Centre mean |
|---|---|---|---|---|---|
| B-BARE | hidden | 0.74 / 0.69 → 0.70 | 0.25 / 0.26 → 0.25 | 0.28 / 0.31 → 0.39 | 100 / 156 → 147 |
| B-BASE | hidden | 0.60 / 0.67 → 0.67 | 1.00 / 0.04 → 0.16 | 0.05 / 0.10 → 0.10 | 130 / 94 → 83 |
| B-WHOLE | leaf-on | 0.70 / 0.69 → 0.69 | 0.12 / 0.09 → 0.08 | 0.46 / 0.55 → 0.64 | 80 / 83 → 50 |
| S-BARE | hidden | 0.70 / 0.72 → 0.68 | 0.15 / 0.25 → 0.22 | 0.46 / 0.22 → 0.31 | 91 / 170 → 160 |
| S-BARK | hidden | 1.07 / 0.90 → 0.58 | 1.00 / 0.00 → 0.00 | 0.47 / 0.43 → 0.52 | 92 / 182 → 183 |
| S-WHOLE | leaf-on | 0.85 / 0.88 → 0.92 | 0.08 / 0.21 → 0.20 | 0.44 / 0.40 → 0.47 | 83 / 111 → 72 |

What the host read on the two whole pairs, within the four-image rule: the
beech now branches low, fills its outline and reads as one mass; it is still
darker and bluer than the photograph and its outline is smoother than a
beech's. The birch is denser and its shoots hang at the edge, but the crown
is a rounded mop where the photograph is an irregular weeping silhouette on
two leaning stems; the hanging curtain and the second stem are not value
changes and are recorded as gaps. Identity pins re-recorded once for both
species with the reason stated. No visual pass is awarded; the owner judges
the round-3 pairs.

## Round 4, fn-37: the curtain as rows (2026-09-14)

The gap analysis after round 3 named the birch's hanging curtain as the first
thing a value table could not reach. fn-37 turned that curtain into four twig
rows — how strongly a shoot hangs, how far it runs before it stops, below what
fraction of the trunk's radius it hangs at all, and how many degrees apart
neighbouring shoots stand — and widened the canopy's four orientation rails to
their signed ranges so a leaf may hang under the shoot it is strung along.

The birch's table is the first to state them: a full hang, shoots running three
and a half metres unbranched where the twig's own anatomy gave them a quarter
of one, nine degrees between neighbours, leaves hung under their shoots, over
an envelope with almost no bare trunk beneath it and a shell retained to less
than half its depth. The beech is untouched and its round-3 pairs still stand.

Photograph / round 3 → round 4, per reference:

| Reference | Foliage | Width over height | Crown base | Occupied | Centre mean |
|---|---|---|---|---|---|
| S-BARE | hidden | 0.70 / 0.68 → 0.67 | 0.15 / 0.22 → 0.15 | 0.46 / 0.31 → 0.41 | 91 / 160 → 161 |
| S-BARK | hidden | 1.07 / 0.58 → 1.33 | 1.00 / 0.00 → 0.00 | 0.47 / 0.52 → 0.27 | 92 / 183 → 168 |
| S-WHOLE | leaf-on | 0.85 / 0.92 → 0.88 | 0.08 / 0.20 → 0.15 | 0.44 / 0.47 → 0.57 | 83 / 72 → 86 |

Five of the eight numbers that compare moved toward the photograph. The crown
base closed on both references — S-BARE is now within 0.004 of the photograph
and S-WHOLE halved its distance — and the whole tree's colour under the
matched sun moved from 72 to 86 against the photograph's 83. The leaf-on
crown is denser than the photograph's (occupied 0.57 against 0.44), which is
where the curtain costs what it buys: three and a half metres of unbranched
shoot is a great deal of leaf-bearing wood, and the retained shell is already
carrying less than half its depth.

What the host read on the whole pair, within the four-image rule: the crown now
reaches nearly to the ground, sky shows through it, and the pale trunk reads
against it, where round 3 was a rounded mop on a bare stem. What it is not yet
is a weeping silhouette. The shoots read as bristles standing out of the crown
rather than as strands hanging from it, and the reason is in the spec: the
curtain's droop is capped at the value the constant already carried — about
nineteen degrees below horizontal — and the hang row only scales that cap down,
never up. A shoot can now be long and it can be set, but it cannot point
further down than it ever could. Closing the rest of that gap needs the cap
itself to become reachable, which is a row this spec did not open.

The cost, at the first fixed seed: 32,752 nodes and 448,981 retained leaves
against round 3's 25,091 and 415,879 — a third more wood for eight per cent
more leaf, which is what a curtain of long unbranched shoots is. The measured
crown base fell from 2.43 m to 1.20 m on a tree of the same height. No growth
hit the node cap, and the fixed and fresh numeric protocol passes all
forty-eight cases.

No visual pass is awarded. The round-4 pairs are recorded by sha256 in
`round4-fn37/stills.json` with `visual_status: unassessed`; the owner judges
them and records the verdict here.

### Round 4b: the droop may exceed the hidden mode's constant (2026-09-15, host QA on fn-37)

fn-37 as specified scaled today's droop cap down with `hang` and never up, so
the birch's shoots were long and settable but could not point further down
than the hidden mode allowed, and the crown read as bristles on a dome. The
host widened the hang rail to 3, so that above 1 a shoot droops past the old
constant, set the birch at 2.4, and re-rendered its pairs. The one image read
(S-WHOLE) shows curtains all around the crown. The fine hanging shoots take
the bark row's white, so the curtain reads frosted where the photograph's
twigs are dark; young-shoot colour by radius is a bark row for fn-40.

Photograph / round 4 → round 4b:

| Reference | Width over height | Crown base | Occupied | Centre mean |
|---|---|---|---|---|
| S-BARE | 0.70 / 0.67 → 0.67 | 0.15 / 0.15 → 0.14 | 0.46 / 0.41 → 0.44 | 91 / 161 → 160 |
| S-BARK | 1.07 / 1.33 → 1.33 | 1.00 / 0.00 → 0.00 | 0.47 / 0.27 → 0.28 | 92 / 168 → 168 |
| S-WHOLE | 0.85 / 0.88 → 0.88 | 0.08 / 0.15 → 0.13 | 0.44 / 0.57 → 0.58 | 83 / 86 → 77 |

## Round 5, fn-39: the crown outline learns a bearing (2026-09-15)

The gap analysis after round 3 named the smooth oval both crowns fill as the
second thing a value table could not reach. fn-39 gave the envelope two rows,
an outline amplitude as a fraction of the radius and a lobe wavelength as a
fraction of the height, keyed by the family's seed so one seed is one outline,
with growth, the scaffold and the twig planner rejecting against the lumpy
shell while the two-dimensional profile keeps driving shedding and the crown
index. The compare script gained the outline's radial deviation from its own
fitted ellipse, read on the photograph's box and the still's mask alike.

The beech states an amplitude of 0.18 at a wavelength of 0.7 of its height,
four or five broad lobes; the birch 0.15 at 0.45, finer and shallower. The
beech at five laterals a station stood within a percent of the node ceiling
on most protocol seeds, and the lobes' per-seed noise tipped seed 89 over it
at any amplitude tried, so its twig length ratio went from 0.42 to 0.40; the
heaviest seed now sits at 224,436 nodes against the 250,000 ceiling, and the
ceiling itself is untouched, a cost the owner sets. Both identity pins are
re-recorded once with the reason stated. The fixed and fresh protocol passes
all forty-eight cases (`measure/protocol-fn39/`).

Photograph / previous round → round 5, per reference (previous is round 3 for
the beech and round 4b for the birch); the last column is the new outline
statistic, photograph / still:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.70 → 0.70 | 0.25 / 0.25 → 0.24 | 0.28 / 0.39 → 0.38 | 100 / 147 → 148 | 0.29 / 0.13 |
| B-BASE | 0.60 / 0.67 → 0.67 | 1.00 / 0.16 → 0.03 | 0.05 / 0.10 → 0.09 | 130 / 83 → 91 | — / 0.14 |
| B-WHOLE | 0.70 / 0.69 → 0.69 | 0.12 / 0.08 → 0.08 | 0.46 / 0.64 → 0.65 | 80 / 50 → 52 | 0.23 / 0.10 |
| S-BARE | 0.70 / 0.67 → 0.67 | 0.15 / 0.14 → 0.13 | 0.46 / 0.44 → 0.43 | 91 / 160 → 159 | 0.17 / 0.11 |
| S-BARK | 1.07 / 1.33 → 1.33 | 1.00 / 0.00 → 0.00 | 0.47 / 0.28 → 0.27 | 92 / 168 → 167 | 0.13 / — |
| S-WHOLE | 0.85 / 0.88 → 0.91 | 0.08 / 0.13 → 0.14 | 0.44 / 0.58 → 0.58 | 83 / 77 → 78 | 0.22 / 0.11 |

How to read the outline column: a smooth oval of revolution sits near zero
and the self-test's five-lobed disc reads above 0.06; the photograph's number
is contaminated by whatever the box holds besides the tree (a fence, a house,
neighbouring crowns), so it bounds the target from above rather than naming
it. Both stills moved off the oval and stay under half the photograph's
figure. The proportion numbers barely moved, which is what a perturbation of
the shell about its own mean should do.

What the host read on the two whole pairs, within the four-image rule: the
beech's silhouette now carries lobes, a dent on the left and a shoulder top
right, where round 3 was an oval; what it is not yet is one dense lit mass,
and it stays darker and bluer than the photograph, which the gap table
already assigns to appearance work. The birch's dome is ragged rather than
round; it is still one upright stem under a frosted curtain, which are
fn-38's and fn-40's. No visual pass is awarded. The round-5 pairs are
recorded by sha256 in `round5-fn39/stills.json` with `visual_status:
unassessed`, and a local judging page beside them
(`measure/judge.html`, ignored) lays every round's pair, the numbers and the
references out for the owner, who records the verdict in fn-34.

## Round 5b: the beech stands up (2026-09-15, owner's round-5 verdict)

The owner judged the round-5 pairs on the judging page and did not accept
either species. On the beech: "I still have an issue with the beech which is
clearly not structurally sound. The reference grows relatively straight up
and out. Our generation bends too much. Second trunk is missing but that's
coming later that's fine." On the birch: "I think the worst part is the
hanging curtains are not affected by gravity or smth. It's clearly wrong."
The birch's verdict is fn-37's and became fn-44 (pendulous shoots hang under
gravity); the second stem is fn-38's, in progress. The beech's structure is
rows, so this is a value round on the beech alone.

The winter pair shows what the owner saw: the photograph's trunk runs up
through the crown and its limbs leave it steeply and keep rising; the
round-5 beech split low into a few limbs that arced out nearly flat. The
habit rows moved: apical dominance 0.2 to 0.5, lateral pitch 50 to 38
degrees with variation 12 to 10, primary rise 0.08 to 0.3, crookedness 14 to
6. Nothing else in the table changed. The beech pin is re-recorded with the
reason stated; the 24-seed protocol passes with the heaviest seed at 210,706
nodes.

Photograph / round 5 → round 5b:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.70 → 0.70 | 0.25 / 0.24 → 0.27 | 0.28 / 0.38 → 0.38 | 100 / 148 → 141 | 0.29 / 0.13 → 0.07 |
| B-BASE | 0.60 / 0.67 → 0.67 | 1.00 / 0.03 → 0.00 | 0.05 / 0.09 → 0.75 | 130 / 91 → 78 | — / 0.14 → 0.23 |
| B-WHOLE | 0.70 / 0.69 → 0.69 | 0.12 / 0.08 → 0.22 | 0.46 / 0.65 → 0.60 | 80 / 52 → 46 | 0.23 / 0.10 → 0.08 |

What the host read on the bare and the whole pair: the trunk now continues
through the crown and the limbs ascend nearly straight, which is the
photograph's architecture. The cost is a crown that starts higher (base 0.22
against 0.12) and is narrower at the shoulder, a goblet where the beech is
broad low down; that is one more value pass (a lower first lateral, a touch
more pitch low down), listed as round 5c. The base shot's occupied figure
jumps because the steeper limbs now cross the base frame. The outline
statistic fell as the ascending limbs fill the shell more evenly. The
leaf-on crown remains a scatter of dark leaves with sky through it, which the
gap table assigns to appearance. No visual pass is awarded; the pairs are in
`round5b-beech/stills.json` and on the judging page.

## Round 5c: a thick core and a clean still (2026-09-15, owner's notes on 5b)

The owner on the round-5b pairs: "ok that is much closer. Is AA off? why is
there this dithering effect again? ... Also just much more thick core trunks
for almost the entire height of the tree. Our tree thins out too quickly."

Anti-aliasing was on, at four samples a pixel, but the matched stills were
rendered at the pair's own height of 720, so a twig thinner than a pixel
dithered whatever the sampler did. The runner now renders a matched still at
1440 tall (`MATCHED_HEIGHT` in `tests/species.mjs`) and the compare script's
Lanczos step to 720 is the rest of the supersample; the measured numbers are
fractions of the still and are unchanged by it. The fixed protocol stills
keep their 960 by 720.

The beech's radius rows moved for the first time: the per-metre taper from
the default 0.6 to 0.3 and the fork exponent from 2.0 to 1.8, so the leader
keeps its girth up through the crown instead of shedding it at every fork;
the DBH proxy stays inside its gate at 0.885 m. The lower limbs leave at 42
degrees instead of 38 so the crown starts lower and reads broader at the
shoulder. The beech pin is re-recorded with the reason; the 24-seed protocol
passes with the heaviest seed at 195,502 nodes.

Photograph / round 5b → round 5c:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.70 → 0.70 | 0.25 / 0.27 → 0.27 | 0.28 / 0.38 → 0.35 | 100 / 141 → 142 | 0.29 / 0.07 → 0.08 |
| B-BASE | 0.60 / 0.67 → 0.67 | 1.00 / 0.00 → 0.00 | 0.05 / 0.75 → 0.72 | 130 / 78 → 112 | — / 0.23 → 0.38 |
| B-WHOLE | 0.70 / 0.69 → 0.69 | 0.12 / 0.22 → 0.08 | 0.46 / 0.60 → 0.60 | 80 / 46 → 44 | 0.23 / 0.08 → 0.10 |

No visual pass is awarded; the pairs are in `round5c-beech/stills.json` and
on the judging page, and the owner judges.

### Round 5d, attempted: the fork exponent against the node ceiling (2026-09-15)

The owner on 5c: "doesn't seem that much thicker?" Right: 5c had moved the
fork exponent the wrong way (1.8 makes the parent thicker relative to its
leader, so the base is the same and the leader thins faster). The row that
keeps a leader thick is a higher exponent, near Murray's 3. At 2.8 the bare
pair showed exactly the core the owner asked for, and every one of the 24
protocol seeds hit the 250,000-node ceiling: seed 1 went from 115,255
branches with four orders and 115,080 twigs to 149,743 branches with a fifth
order of 35,655 and 82,843 twigs, capped. The thicker wood stays above the
threshold at which a limb becomes a twig for one more order, and that order
is what the ceiling cannot hold. Exponents 2.0, 2.2, 2.3 and 2.5 with the
halved taper capped the same three heavy seeds; raising the twig bearing
diameter (0.03 to 0.045), the limb handoff share (0.1 to 0.16) and the twig
diameter (5 to 8 mm) each left the fifth order and the cap in place. The
table returns to 5c's values and the 5d stills are not recorded.

What this needs is the owner's: the beech at five twig laterals a station
sits within a percent of the ceiling before any of this, and a core as thick
as the photograph's costs on the order of a third more nodes. Either the
ceiling rises for the catalogue (a cost decision the gap table already
names) or the beech gives up twig density for girth. Neither is a value the
table may spend on its own.

The owner's read on the 5c winter pair, after this: "Too many small
branches. It does seem like there's less density on the reference actually.
Fewer larger branches compared to ours which has many more thinner ones
directly attached to the trunk." Two laterals a station, 2.2 m apart and
0.6 long, at exponent 2.8: 60 first-order limbs instead of 106, and a fifth
twig generation of 73,627 branches, capped on every heavy seed. The twig
layer spends whatever the limbs give up, so the coupling is the
generator's: fn-45.

## Round 6, fn-38: two stems (2026-09-15)

The first thing the gap analysis named after round 3 was the one no value
could reach: the birch in the S-WHOLE and S-BARE photographs stands on two
stems that part at the ground, one leaning well out of the pair, and the
generator grew one leader from the origin with every stage below it assuming
one. fn-38 makes the clump three habit rows — how many stems leave the root,
how far apart in bearing they part about a bearing the seed alone decides, and
how far out of vertical the outermost of them lean — and grows them as
order-zero axes born at the root. They share the base through the pipe model
that was already there, each is swept as a trunk run with its own buried root
and flare and no fork socket, the canopy and the twig layer measure against
the thickest stem rather than the root's combined pipe, and the profile's
diameter proxy stops calling several stems ambiguous and reports the largest
with a count of how many crossed breast height.

The birch's table declares two stems, a hundred and ten degrees apart in
bearing and twenty-two degrees out of vertical apiece. Twelve degrees was
tried first and read as one flared trunk that divided: this birch's curtain
reaches to within a fraction of a metre of the ground, leaving only the first
couple of metres of stem bare, and over that stretch a dozen degrees left the
two of them still inside one another's bark. Every other table leaves the
count at one stem, where the other two rows reach nothing, and is
byte-identical; the birch's identity pin is re-recorded once with the reason
stated. The fixed and fresh protocol passes all forty-eight cases
(`measure/protocol-fn38/`), no birch seed is node-capped, and the heaviest
birch seed stands at 94,838 nodes against the 250,000 ceiling. At seed 1 the
birch went from 41,103 nodes and 3.6 s in round 5 to 88,244 nodes and 10.5 s:
two stems are two crowns' worth of laterals inside one shell, and the cost is
close to double.

Photograph / round 5 → round 6, per reference; the last column is the
outline's departure from its own fitted ellipse, photograph / still:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| S-BARE | 0.70 / 0.67 → 0.68 | 0.15 / 0.13 → 0.13 | 0.46 / 0.43 → 0.45 | 91 / 159 → 156 | 0.17 / 0.11 → 0.08 |
| S-BARK | 1.07 / 1.33 → 1.33 | 1.00 / 0.00 → 0.00 | 0.47 / 0.27 → 0.33 | 92 / 167 → 160 | 0.13 / — → 0.61 |
| S-WHOLE | 0.85 / 0.91 → 0.92 | 0.08 / 0.14 → 0.15 | 0.44 / 0.58 → 0.51 | 83 / 78 → 49 | 0.22 / 0.11 → 0.10 |

Two of those moved for the clump and not for noise. S-BARK is the close-up on
the base, and its outline statistic went from unreadable — a mask that touched
the frame on every side — to 0.61, far above the self-test's five-lobed disc
at 0.06: the bark shot is no longer one column of wood but two parting, which
is exactly what the statistic is measuring. S-WHOLE's centre mean fell from 78
to 49 against the photograph's 83, and the clump is why: the middle of the
leaf-on box used to hold the white trunk running up it, and with the two stems
leaning apart there is foliage there instead. The proportions barely moved,
which is what taking one axis and making it two about the same shell should
do; the leaf-on crown grew a little less dense, its occupied share falling
from 0.58 toward the photograph's 0.44.

What the host read on the two pairs, within the four-image rule. On S-BARE the
still now plainly stands on two stems: one flared foot at the ground dividing
immediately into two white stems that lean apart, both carrying bark and both
flared where they meet the ground. Against the photograph the parting is
lower and more symmetric — the photograph's two stems run up together for a
couple of metres before one leans away, and ours part at the root and open
into a V — and the photograph's leaning stem travels much further out before
it rises. On S-WHOLE the clump does not read at all: the curtain covers the
base to within a fraction of a metre of the ground and only a stub of white
shows under it, where the photograph's two stems are visible under a canopy
lifted on that side. That is the curtain's reach rather than the clump, and it
belongs to fn-44; the fine hanging shoots still take the trunk's white bark,
which is fn-40's. What else reads wrong and is nobody's spec yet: the parting
is a symmetric V because the two stems are spread about one bearing and lean
by the same angle, so neither of them is the near-vertical stem the
photograph's pair has. No visual pass is awarded. The round-6 pairs are
recorded by sha256 in `round6-fn38/stills.json` with `visual_status:
unassessed`, and the owner records the verdict in fn-34.

## Round 6b, fn-44: the curtain hangs (2026-09-15)

The owner rejected the round-5 birch on one thing: "I think the worst part is
the hanging curtains are not affected by gravity or smth. It's clearly wrong."
fn-37 had made the curtain reachable as rows, but a curtain lateral departed in
one fixed direction - across the crown and down by a droop - and then ran
straight, so every hanging shoot was a rod pointing sideways-and-down. fn-44
gives the twig table a fifth row, `sag`: how far toward straight down a hanging
shoot's course has turned by the end of its pendulous length, spread along the
run as a cubic ease, steepest where the shoot leaves the wood that bears it.
Weight is not a turn a growing tip steers, so the branch law's own turn limit
does not bound it. Neutral is the straight rod, and every table but the birch's
leaves the row there and is byte-identical.

### What had to give way before a shoot could hang

The first pass at this round was rejected too - "clearly this doesn't look
right", a dome of white bristles with nothing hanging - and the measurement
said why. Of the 5,761 hanging runs the first fixed seed plans, 4,226 (73%)
ended at the length their own wood would hold out, a median of 1.00 m against
the 3.5 m the table allowed; 1,099 (19%) at the tip of the limb they hung
under; 436 (8%) at the shell. A shoot that spends a third of its arc is still
pointing outward when it stops.

Two of the three are not what ends a strand hanging by its own weight, and the
sag row now carries both, walked by the row rather than switched by it so that
no value of it is the frame where a shoot changes kind:

- **The run.** The allometry that gives a self-supporting limb its length by
  its own thickness has nothing to say about a strand hanging from one, so the
  sag carries the run from what the branch law allows out to the whole
  pendulous length.
- **The floor.** The tip of the first descending ancestor is where a shoot held
  out by its own wood comes to rest; a weeping one falls past it, so the sag
  carries the floor down to the crown's own base.

The third stays: the shell is what the envelope contains, every post-crossover
node is inside it by an invariant this repository has held since the beginning,
and a curtain is no exception. Measured both ways, letting the curtain out of
the shell bought nothing the form needed and cost the invariant - the shoots
ran 2.42 m rather than 2.37 m of 2.50 m, and the same 96% hung their lower half
within 15 degrees either way - while the crown fell past the photograph's own
crown base. So the shell holds and the curtain hangs inside it.

### The birch's table

A sag of 1 over a pendulous length shortened from 3.5 m to 2.5 m, which is what
the photograph's strands measure against the tree. At the first fixed seed its
4,266 hanging shoots run 2.37 m on average; their angle to straight down is
6.1 degrees at half the run and 1.3 degrees at its end, and the whole lower
half of 96% of them lies within 15 degrees of vertical.

Then the crown had to be opened. A shoot that runs two and a half metres
instead of one carries two and a half times the leaf-bearing wood, and the
first render of this round put 1,139,792 leaves in the crown against round 5's
591,007: the form hung, and the mass hid it. The lever is the twig's own
internode length, 12 mm, which is where the leaves sit on a table that has a
twig layer - the canopy's `spacing` row places leaves only on wood the twig
layer has not marked, and on this table that is none of it. At 36 mm the crown
carries 389,795 leaves, a third of what it did and a third fewer than round 5,
and the sky comes back through it. The retained shell depth was tried too and
put back: at 0.35 it culled the crown's interior and carried the leaf-on
brightness to 91 against the photograph's 83, past it rather than up to it,
where the leaf spacing alone lands on 79.5. The skeleton hash is unchanged by
any of this - the wood is the same tree, and the 96% is the same 96%.

Photograph / round 5 → round 6b:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| S-BARE | 0.70 / 0.673 → 0.674 | 0.15 / 0.132 → 0.122 | 0.46 / 0.430 → 0.469 | 91 / 159 → 163 | 0.17 / 0.110 → 0.115 |
| S-BARK | 1.07 / 1.335 → 1.335 | 1.00 / 0.00 → 0.00 | 0.47 / 0.274 → 0.301 | 92 / 167 → 167 | 0.13 / — → 0.872 |
| S-WHOLE | 0.85 / 0.906 → 0.906 | 0.08 / 0.141 → 0.120 | 0.44 / 0.578 → 0.569 | 83 / 78 → 80 | 0.22 / 0.114 → 0.108 |

Every number that compares is closer to its photograph than round 5's, on both
the bare and the leaf-on reference: the winter crown fills 0.469 of its box
against the photograph's 0.459, where round 5 filled 0.430; the leaf-on crown
base falls from 0.141 to 0.120 against 0.08; the leaf-on crown is thinner
(0.569 against 0.443) and brighter under the matched sun (80 against 83, from
78). What is still furthest out is the leaf-on occupied figure, and it is the
one number that a denser or thinner crown cannot fix alone: the wood itself
fills 0.469 of the winter box, so a crown of strands hanging the length of the
tree is a wide mask however few leaves it carries. The bark reference's outline
figure is noise - its mask fills the whole frame, which is what an outline
statistic cannot read.

The cost at the first fixed seed: 4,385,080 wood triangles and 389,795 retained
leaves against round 5's 2,443,040 and 591,007 - a third more wood for a third
less leaf. No growth hit the node cap and the fixed and fresh numeric protocol
passes all forty-eight cases (`measure/protocol-fn44d/`).

### What the host read on the pairs, and what is left

Within the four-image rule: the shoots hang, and the crown is now airy enough
to see that they do. Strands fall from every limb down the lower half of the
tree, sky shows between them, and the pale trunk reads through the crown the
way the photograph's does - which is the photograph's architecture, and was in
no earlier round.

Three things still read wrong and each is somebody's row:

- **The strands are all the same length.** 85% of the birch's hanging shoots
  run exactly 2.50 m; the tenth percentile is 1.79 m and every metre of that
  spread comes from the shell or the floor cutting a shoot short, not from a
  row. `vigourVariation` varies the length the branch law asks for, and the
  sag discards that length, so at sag 1 it reaches nothing. No twig row varies
  a pendulous run per shoot. That is fn-34's gap against the photograph, whose
  strands run every length from a hand's width to three metres, and closing it
  is a row fn-44 did not open.
- **The shoots are white.** Thinning the crown made the bark row's white more
  visible, not less: fn-40's young-shoot colour by radius.
- **One stem.** fn-38's.

No visual pass is awarded. The round-6b pairs are recorded by sha256 in
`round6b-fn44/stills.json` with `visual_status: unassessed`; the owner judges
them on the judging page and records the verdict in fn-34.

## Round 6, fn-45: few big limbs (2026-09-15)

### What the fifth generation really was

The round-5d note above read the beech's cap as a fifth twig-law generation
that a higher fork exponent grew. It is not. The twig law's depth is shallow
and it falls, not rises, with the exponent: at 1.8 the beech branched three
generations from the structural crown, at 2.8 it branched two, and
`MAX_LEVELS` (12) was never approached. `branch_order` in
`examples/species_metrics` is not that depth - it is a Gravelius order over
every node, twig stations included - so the "fifth order" 5d recorded is one
more level of *runs*, not one more generation.

What the exponent moves is the branch-or-twig decision itself. `is_twig` in
`advance.rs` compares an absolute radius against `twig.diameter / 2`, and the
pipe model rescales every radius in the tree: at 1.8, 72,166 of the beech's
first-generation laterals on seed 1 were single twig nodes; at 2.8 only 361
were, and the other sixty thousand became multi-node branch runs that each
hang a twig at every internode. That is the ceiling, and no depth cap reaches
it. So the row this spec adds, `skeleton.twigs.generations` (1 to 6, neutral
at 6), is real and general - the twig layer's depth is a table's choice
rather than a consequence of how thick the wood is - but it is not what
pays for a thick core; twig density is. The beech states two generations,
which its shipped radii happen to reach on their own; with its twig and
bearing thresholds thinned, the row is what holds it there.

### What the pairs asked for, and what moved

Three readings drove the table. The owner on 5c: a thick core and fewer,
larger limbs. The first round-6 render: limbs sweeping out and down, a core
that thinned above two thirds, a crown widest low. And the host on round 6b's
leaf-on pair: a flat umbrella of leaves on the top third over bare limbs, the
trunk forking into five or six co-dominant stems at a third of the height,
and foliage that read as fern fronds.

Round 6b answered the first two with steep limbs (32 degrees) and made the
bare pair read while the leaf-on pair failed: the scaffold ended its leader
at three fifths of the height, so the crown above that was carried by limbs,
and a limb leaving low at 32 degrees ran beside the leader to the crown's
top. Every limb tip - and every leaf, since leaves follow twig wood and twig
wood grows from tips - ended in the upper third, and the long low limbs
carried enough wood to rival the leader. On seed 1, seven stems crossed a
third of the height at more than half the thickest one's radius, 72 per cent
of the leaves sat in the top third, and the lowest twentieth of the leaves
began at 47 per cent of the height. Round 6c then opened the limbs to 58
degrees, which fixed the leaf-on pair and turned the bare crown into a round
head with the leader still stopping at three fifths.

The row that serves both pairs is apical dominance. The scaffold ends its
leader at `crownBase + (1 - crownBase) * apicalDominance` of the height; the
beech's 0.9 runs it to nine tenths, so the leader, not the limbs, carries
the crown to the top. With that, the limbs leave at 48 degrees and bend up
over their run (rise 0.45, climbing about 61 degrees over their length)
without the low ones ending at the crown's top; their side branches are held
nearly level (0.1). The crown starts at a twentieth of the height, is widest
at 0.48 of its depth and rounds at the top (shoulder 1.5). The fork exponent
is 2.6: at 2.8 the wood sat on a cliff, where a few degrees of limb angle
collapsed the local layer from about 150,000 nodes to 55,000 and the crown
to a skeleton. Four twig laterals a station fill the crown and a twig length
ratio of 0.30 pays for them and for the longer leader. Local shoots are
two-ranked (divergence 180, not 137.5), so a spray is flat rather than a
bottlebrush. Leaves lean along their shoots (0.45), lift toward the light
(0.3), scatter 30 degrees and are a tenth larger with less spread, keeping
the largest leaf inside the sourced 4 to 10 cm. Shoots under a twentieth of
the trunk's radius carry leaves (`canopy.shootRadius` 0.05), which leafs the
inside of the crown at no node cost. The species test's bound on the beech's
apical dominance moves from 0.6 to 0.95, which still refuses the spruce's
excurrent 1.0.

Seed 1, round 6b -> final: stems at a third of the height 7 -> 1; the
leader reaches 0.60 -> 0.90 of the height and at three quarters of it is
0.88 of the thickest wood crossing there; leaves by third of the height
1/27/72 -> 6/50/44 per cent; lowest twentieth of the leaves at 47 -> 32 per
cent of the height; leaf crown's radius at its top tenth over its widest
0.61 -> 0.42, a rounded top.

Budget: every one of the 48 protocol cases passes, the heaviest beech seed
is 197,872 nodes of the 250,000 ceiling, none is node-capped, and neither is
any of a further 120 random seeds (heaviest 206,066). No protocol or random
seed fails the surface build on this table. The DBH proxy holds at 0.889 m.

Photograph / round 5c -> round 6:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.70 -> 0.73 | 0.25 / 0.27 -> 0.29 | 0.28 / 0.35 -> 0.47 | 100 / 142 -> 126 | 0.29 / 0.08 -> 0.09 |
| B-BASE | 0.60 / 0.67 -> 0.67 | 1.00 / 0.00 -> 0.78 | 0.05 / 0.72 -> 0.10 | 130 / 112 -> 75 | - / 0.38 -> 0.16 |
| B-WHOLE | 0.70 / 0.69 -> 0.74 | 0.12 / 0.08 -> 0.09 | 0.46 / 0.60 -> 0.59 | 80 / 44 -> 47 | 0.23 / 0.10 -> 0.09 |

Seed 1, round 5c -> round 6: 193,836 -> 188,636 nodes; 115,255 -> 87,670
branch axes; 115,080 -> 86,700 twigs; branches per order 5c's four orders ->
{1: 51, 2: 1,048, 3: 9,061, 4: 33,713, 5: 43,797}. Leaf instances at seed 7:
1,520,948 -> 2,254,119, inside the fidelity band.

### Read on the pairs, without a verdict

B-BARE: one leader, thick at the base, runs up through the crown to near its
top, with limbs leaving it along the whole height and rising steeply - the
photograph's architecture, grown straight up and out. The crown is an
upright oval rather than round 6c's round head. What still reads wrong: the
crown is fuller and wider in its upper half than the photograph's, which
narrows toward the top; the trunk above the first limbs is slimmer than the
photograph's heavy column, and the leader thins to twig girth in its top
fifth; occupied is 0.47 against the photograph's 0.28, most of which is the
resolution of the two measurements (a dark-pixel fraction off a 267 by 360
crop against a full mask at 961 by 1366), the rest genuinely more wood.

B-WHOLE: a full rounded crown on a central trunk, the leader visible running
up into the leaf mass, the limbs rising under it. The umbrella and the vase
are gone. The leaf mass sits higher than round 6c's (top third 44 per cent
against 28) because steeper limbs carry their tips up, and below it the
lower third of the crown is open, the rising limbs showing between sprays.
Two faults are left for the generator spec the host is capturing and are not
chased here: the leaf mass stops at about a third of the height where the
photograph's reaches about two metres above the ground - leaves follow twig
wood, and the lower crown holds only the trunk and the thick first metres of
the lowest limbs - and the sprays at the crown's edge read as fern fronds,
because a twig is a fixed 25 cm with a leaf every 2 cm and shorter twigs
cost a twig node per 15 cm of spray, which the node ceiling refuses (twig
length 0.15 m node-caps seed 1 even at three laterals a station). The
foliage is darker and bluer than the photograph's (centre mean 47 against
80); that is appearance, owned elsewhere.

### A robustness finding for the owner

`surface.rs:351` refuses a wood triangle whose three float32 positions come
out collinear with "surface triangle collapsed in float32", and the refusal
is a hard `InvalidInput`: the seed grows no tree at all. Round 6 met it at
apical dominance 0.59 (protocol seeds 2181184680 and 2779011501), but it is
not an apical-dominance effect: during round 6c one candidate table failed
protocol seed 55 at 0.58, and the same table failed three of 120 random
seeds at 0.9 and none of the protocol's. It tracks geometry, about one seed
in a hundred, wherever it falls. The final table is clean on all 24
protocol seeds and 120 random ones; that is a miss, not a cure. It belongs
to the surface builder and is becoming its own bug spec.

No visual pass is awarded; the pairs are in `round6-fn45/stills.json` and on
the judging page, and the owner judges.

The merge of fn-45 into fn-34's integration branch reproduced this round: the
beech's three matched stills rendered from the merged branch
(`measure/pairs-merge45/`) are byte-identical to the sha256 records in
`round6-fn45/stills.json`, and so are their compare numbers.

## Round 7: the birch on two stems under a hanging curtain (2026-09-15)

fn-38 and fn-44 each fixed one thing on its own branch, so round 6 showed two
stems under a curtain of rods and round 6b showed a curtain that hangs from one
stem. The fn-34 integration branch merges fn-44 into fn-38, which also brings
round 5b's beech rows from fn-44's base. The birch's table now declares both
the clump (two stems, 110 degrees apart in bearing, 22 degrees out of vertical)
and the curtain (sag 1, a 2.5 m pendulous run). Three files conflicted and
each kept both sides: the sweep's comment above the HELD list, the identity
pins' doc paragraphs, and this report's round sections. The birch re-pins once
for the merge, and the beech's pin is round 5b's. The sag test's cooked family
now stands on one stem, because the clump splits the birch's wood between two
and wood that thin bears its hanging shoots in one or two chords, leaving no
arc to read. The birch's neutral-sag pin is re-recorded.

### What the host changed after looking

The first render of the merged table failed on sight. The curtain fell to the
crown base of 0.015 of the height, 0.27 m, so on S-WHOLE the two stems showed as
one white stub under a single mass, and on S-BARE as a short V under a white
fog. Two stems carry two crowns' worth of shoots inside one shell, and at a leaf
every 36 mm the seed-1 crown carried 848,801 leaves, more than twice round 6b's
389,795, with no sky through it. Two rows of the birch's own table reach both
problems:

- **The crown base, 0.015 to 0.10 of the height.** Under a sag of 1 the crown's
  base is the curtain's floor (fn-44), so lifting it lifts where the strands
  stop. Measured off the still's own mask, the bare stem below the curtain went
  from 7% to 16% of the tree's height on both references, about the first two
  metres, which is what the photographs show.
- **The twig internode, 36 mm to 60 mm.** This is the leaf spacing on a table
  with a twig layer, the same lever round 6b used. The seed-1 crown carries
  444,750 leaves.

The sag test's cooked family keeps the 36 mm internode its arc was read on. At
60 mm a run's stations fall unevenly, so a longer stride turns further in one
step and the per-step check stops holding. The turn per metre still eases off
the whole way down on all 228 whole runs, which is the law the row states.

Photograph / round 6 / round 6b → round 7:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| S-BARE | 0.70 / 0.68 / 0.67 → 0.69 | 0.15 / 0.13 / 0.12 → 0.20 | 0.46 / 0.45 / 0.47 → 0.46 | 91 / 156 / 163 → 167 | 0.17 / 0.08 / 0.12 → 0.09 |
| S-BARK | 1.07 / 1.33 / 1.33 → 1.03 | 1.00 / 0.00 / 0.00 → 0.00 | 0.47 / 0.33 / 0.30 → 0.38 | 92 / 160 / 167 → 154 | 0.13 / 0.61 / 0.87 → 0.72 |
| S-WHOLE | 0.85 / 0.92 / 0.91 → 0.86 | 0.08 / 0.15 / 0.12 → 0.21 | 0.44 / 0.51 / 0.57 → 0.46 | 83 / 49 / 80 → 89 | 0.22 / 0.10 / 0.11 → 0.12 |

The merged table before the two QA rows read 0.91, 0.13, 0.50, 69 and 0.10 on
S-WHOLE and 0.68, 0.11, 0.50, 166 and 0.10 on S-BARE. Width over height on
S-WHOLE is the closest any round has come (0.86 against 0.85). Both occupied
figures sit within 0.02 of their photographs. S-BARK's box stopped filling the
frame, because the curtain no longer hangs across the base shot, and its width
over height reads 1.03 against 1.07. Two numbers moved away. The crown base on
both references is now above the photograph's, 0.21 against 0.08 on S-WHOLE,
because that photograph's figure is its left-hand curtain reaching to about a
metre while the stems stand clear in the middle, and one floor all round the
tree can do one or the other. The leaf-on centre went from 14 darker than the
photograph to 6 brighter, and the white fine shoots showing through the thinner
crown account for part of that.

The cost at the first fixed seed is 152,330 nodes, 9,143,160 wood triangles and
444,750 leaves in 3.8 s, against round 6b's 4,385,080 triangles and 389,795
leaves. The heaviest birch seed stands at 154,583 nodes against the 250,000
ceiling. The fixed and fresh protocol passes all forty-eight cases
(`measure/protocol-round7/`), and no seed is node-capped.

### What the host read on the pairs, and what is left

Within the four-image rule, two images before the QA rows and two after. On
both pairs the tree now stands on two white stems that part at the ground and
carry a curtain whose strands fall nearly vertical from the limbs. Sky shows
through the leaf-on crown at its edges and in gaps through the middle. Neither
round 6 nor round 6b had both.

It still does not read as the photograph's tree, for these reasons:

- **A ball on a V.** The curtain stops at one height all round, so the crown is
  a round mass with a level lower edge. The photograph's curtain comes down to
  about a metre over the leaning stem's side and lifts over the stems. Part of
  that is the uniform strand length (the named gap; round 6b measured 85% of
  strands at the full 2.5 m), and part is that the floor is one height for the whole tree.
- **The stems are a symmetric V** parting at the root. The photograph's pair
  runs up together before one leans away, and neither of ours is its
  near-vertical stem. This is the named gap after fn-38.
- **The winter crown is a white fog.** On S-BARE the fine hanging wood fills the
  crown in the bark's white. The photograph's winter crown shows its limbs dark
  against the sky with a brown haze of shoots. The colour is fn-40's.

No visual pass is awarded. The round-7 pairs are recorded by sha256 in
`round7-birch/stills.json` with `visual_status: unassessed`, and the owner
records the verdict in fn-34.

## Round 8, fn-46: young shoots take their own colour (2026-09-15)

Every piece of wood took the trunk's colour, so the birch drew its hanging
curtain in white bark and two worker reads named the white shoots the loudest
fault on the pairs. fn-46 gives the material row a young wood: `shootRed`,
`shootGreen` and `shootBlue` (0 to 1) and `shootRadius` (0 to 0.1 m). Wood
thinner than the radius takes the shoot colour and gives it up to the bark
colour on a smoothstep of the radius it already carries, reaching the bark by
twice the radius. The wood fragment mixes the two before mottle, cavity and
occlusion act, next to the relief's own `maturity`, which carries relief by
radius in ridge widths where the new term carries colour by radius in shoot
radii. A row with no radius selects the bark colour alone. The five neutral
presets' whole and bare stills hash the same before and after, ten of ten, and
the render tests' pinned stills hold. The rows are validated by name, on the
wire, walked linearly, in the regenerated browser metadata and in the
harness's panel. Colour changes no geometry, so the birch's skeleton, its
leaves and every mesh pin are round 7's.

### The rows

- **Silver birch: 0.095/0.07/0.055 below 1 cm radius, white by 2 cm.** The
  literature gives the twigs as "slender, reddish brown" and the bark as
  "reddish brown ... when very young, later turning white" (VT Dendrology;
  Atkinson 1992). The S-BARE photograph's winter haze, read numerically off
  the non-sky pixels of its crown, is linear 0.061/0.042/0.030 in its darker
  band on the same exposure that reads its trunk at 0.78/0.78/0.71, which is
  the bark row.
- **European beech: 0.14/0.13/0.085 below 4 mm, grey by 8 mm.** Twigs "slender,
  zigzag, light brown" (VT Dendrology), olive-brown stems (OSU). B-BARE's winter
  haze reads 1:0.86:0.52 in its darker band. The beech's table is being
  reworked on another branch, so this row is stated but not rendered here;
  the host renders B-WHOLE and B-BARE after the merge.
- **Oak, spruce, Ordinary and the Two Trees stay neutral**, with a shoot colour
  equal to their own bark so a walk from one of them starts at no change.

### What the worker changed after looking

The first render at 0.10/0.06/0.045 turned the winter fog into a haze of
strands, and it was too red. S-BARE's darker haze band read sRGB 72/51/41
(linear 1:0.51:0.35) against the photograph's 70/58/48 (1:0.69:0.49), and a
maroon cast showed through the middle of the leaf-on crown. The matched still
reddens a row, so the row moved to the greyer 0.095/0.07/0.055, and the band
now reads 71/58/49 (1:0.67:0.49). The beech's unrendered row takes the same
correction toward green.

### fn-34's value pass on the birch's leaves

The host's read of the first round-8 S-WHOLE: the first birch that reads as a
weeping birch, but a leaf mass far too dark and heavy, 32 against the
photograph's 83 at the centre. That pass was authorised on the birch's leaf
rows and its canopy density rows only, with the shot and the sun held as the
instrument. The leaf rows moved to a lighter yellow-green blade, front
0.10/0.19/0.06 and back 0.20/0.28/0.14, from S-WHOLE's own leaf band. That band
reads R:G:B 1:1.38:0.79 linear (sRGB 92/108/82), where the old row drew it at
1:1.93:0.79, and the new row draws it at sRGB 83/105/73. Interior darkening went
from 0.4 to 0.15. The centre moved from 32 to 35. The target was within about
10 of 83, and no row the pass may touch reaches it.

These are the numbers behind that, each read off S-WHOLE's centre crop with
one row moved at a time:

- **The curtain's wood covers 74% of the centre crop.** That is the share of
  pixels that change when only the shoot colour does. The same state with
  white young wood reads 87, and with this dark wood 35. In the photograph the
  strands hide behind the leaves.
- **The leaves there are in shadow.** Even leaves of reflectance 1 reach only
  78, or 81 with the crown's sky occlusion off as well. Sunlit leaf pixels,
  brighter than half of full scale, are 1% of the crop against the
  photograph's 20%. The shadow instrument is not the cause. Casting every leaf
  rather than one in four reads 31.8 against 32.0, and quadrupling the
  shadow's normal offset reads 32.4. The leaves' own light terms do not reach
  it either: full transmission reads 35.9, and leaves turned up instead of
  hanging read 34.8, against 35.3.
- **Thinning the leaves bares the interior.** A retained shell of 0.3 reads
  39, and one of 0.2 reads 44 on 226,452 leaves (46 with the new leaf rows).
  The one image taken of 0.2 showed the upper crown as a bare brown tangle,
  winter strands with leaves only at the rim. A twig internode of 120 mm reads 41 on 221,857 leaves, and
  the internode also sets the stations of the hanging runs that fn-47 owns.
  Both were put back.

Photograph / round 7 → round 8:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| S-BARE | 0.70 / 0.69 → 0.70 | 0.15 / 0.20 → 0.22 | 0.46 / 0.46 → 0.52 | 91 / 167 → 71 | 0.17 / 0.09 → 0.09 |
| S-BARK | 1.07 / 1.03 → 1.12 | 1.00 / 0.00 → 0.00 | 0.47 / 0.38 → 0.34 | 92 / 154 → 153 | 0.13 / 0.72 → 0.73 |
| S-WHOLE | 0.85 / 0.86 → 0.94 | 0.08 / 0.21 → 0.21 | 0.44 / 0.46 → 0.42 | 83 / 89 → 35 | 0.22 / 0.12 → 0.13 |

The first render's shoot row read 68 and 31 for the two centre means, the
corrected shoot row 71 and 32 before the leaf pass, and the same figures for
the rest. The winter centre went from 75 brighter than the
photograph to 20 darker, and its leading channel is now red, as the
photograph's is (round 7 led with blue). The leaf-on centre went from 6
brighter to 48 darker. The white wood had been carrying the leaf-on crown's
brightness, and with it gone the curtain's wood and the shaded leaves read far
darker than the photograph's sunlit crown.

The mask figures moved although no geometry did. The still's tree mask is
what stands out of the row's background, and white fine wood against a pale
sky fell under that threshold where dark strands do not. So the curtain's
thin tips now count. S-WHOLE's width over height reads 0.94 against 0.85,
where round 7's 0.86 had been measuring a crown whose white edges the mask
could not see. S-BARE's occupied figure went from 0.46 to 0.52 for the same
reason, and S-WHOLE's fell from 0.46 to 0.42 because its box widened more than
its mask filled.

The cost is one smoothstep, one mix and one select per wood fragment. The
oak's hero frame at seed 7, 1600 by 1000, measures 3.59 ms p50 and 3.93 ms p95,
beside fn-29's recorded 3.60 ms and 4.18 ms for the same command. The birch at
seed 1 measures 5.06 ms p50. The fixed and fresh numeric protocol passes all
forty-eight cases unchanged, after the shoot rows (`measure/protocol-fn46/`)
and again after the leaf pass (`measure/protocol-fn46b/`).

### What the worker read on the pairs, and what is left

Within the four-image rule, two images before the shoot row's correction and
two after. The leaf pass was a fresh capture and took two more: the 0.2 shell,
and the committed rows. S-BARE is byte-identical across the leaf pass, since
it draws no leaves. On S-BARE the winter crown is no longer a white fog. It is a grey-brown
haze of hanging strands, with the two white stems running up into white
scaffold limbs inside it, which is how the photograph's winter tree is built.
On S-WHOLE the curtain reads as dark strands under green leaves on two white
stems. After the correction no red cast shows through, and after the leaf
pass the green is the photograph's yellower green. No hard line
between white and brown was visible at the pairs' scale. The ramp test holds
the blend continuous: no neighbouring pixel columns differ by more than 0.06
of the full step.

What still reads wrong:

- **The leaf-on crown is too dark and heavy**, 35 against 83 at the centre.
  The pass above measured why: three quarters of the centre is curtain wood,
  and the leaves over it get almost no sun. Both are outside the leaf and
  density rows. The first is how much wood the curtain draws, which is its
  strand count and girth (the twig and pendulous rows). The second is that
  the leaves the camera sees stand in the crown's own shade under a side sun,
  where the photograph's catch it. The leaf rows can only move the crown
  between 32 and 35.
- **The winter haze is a shade cooler than the photograph's.** Its darker band
  matches, but the photograph's sunlit haze reads more coppery. That is a
  second, sunlit band, and the scene's low sun does not warm the strands the
  way the photograph's does.
- **The uniform strand length and the level hem** (fn-47) and **the symmetric V
  of the stems** (fn-48), unchanged from round 7.

No visual pass is awarded. The round-8 pairs are recorded by sha256 in
`round8-fn46/stills.json` with `visual_status: unassessed`, and the owner
records the verdict in fn-34.

## Round 8c: the merge of fn-45 and fn-46 (2026-09-15)

The integration branch now carries both species' latest work in one tree. The
beech is fn-45's final round-6 table, and fn-46 adds its young-shoot colour, an
olive-brown (0.14/0.13/0.085 linear) below a 4 mm radius. The birch is round
7's table with fn-46's round-8 colours: red-brown young wood below 1 cm, the
yellow-green blade, and the lighter interior darkening. Neither merge conflicted
on this step. Every identity pin reproduces exactly, because colour does not
move geometry. All forty-eight protocol cases pass
(`measure/protocol-merge46/`), and no seed is node-capped.

Photograph / previous → merged. The previous round is fn-45's round 6 for the
beech and fn-46's round 8 for the birch:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.73 → 0.73 | 0.25 / 0.29 → 0.29 | 0.28 / 0.47 → 0.49 | 100 / 126 → 121 | 0.29 / 0.09 → 0.09 |
| B-BASE | 0.60 / 0.67 → 0.67 | 1.00 / 0.78 → 0.78 | 0.05 / 0.10 → 0.10 | 130 / 75 → 75 | — / 0.16 → 0.16 |
| B-WHOLE | 0.70 / 0.74 → 0.74 | 0.12 / 0.09 → 0.09 | 0.46 / 0.59 → 0.58 | 80 / 47 → 45 | 0.23 / 0.09 → 0.08 |
| S-BARE | 0.70 / 0.70 → 0.70 | 0.15 / 0.22 → 0.22 | 0.46 / 0.52 → 0.52 | 91 / 71 → 71 | 0.17 / 0.09 → 0.09 |
| S-BARK | 1.07 / 1.12 → 1.12 | 1.00 / 0.00 → 0.00 | 0.47 / 0.34 → 0.34 | 92 / 153 → 153 | 0.13 / 0.73 → 0.73 |
| S-WHOLE | 0.85 / 0.94 → 0.94 | 0.08 / 0.21 → 0.21 | 0.44 / 0.42 → 0.42 | 83 / 35 → 35 | 0.22 / 0.13 → 0.13 |

The three birch stills are byte-identical to round 8's sha256 records, and so
is the beech's B-BASE close-up, where no wood is thin enough to take the shoot
colour. The shoot colour moves only B-BARE and B-WHOLE: the winter crown's
centre darkens from 126 to 121 against the photograph's 100, and the leaf-on
centre from 47 to 45 against 80.

### What the host read on the four pairs

The host looked at B-WHOLE, B-BARE, S-WHOLE and S-BARE and changed nothing,
since fn-50, fn-51 and fn-52 start from this merge.

- **B-WHOLE.** The dome's outline and proportion sit near the photograph's. The
  crown itself does not match it. The photograph is one dense, lit, mid-green
  mass with the trunk hidden to the ground. Ours is a dark green umbrella of
  feathery leaf sprays at the limb ends, with grey limbs bare along their
  length and sky through the whole lower half of the crown. The bare limbs are
  fn-50's short shoots, and the darkness (45 against 80) is fn-52's canopy
  lighting.
- **B-BARE.** The fine twig haze at the crown's top reads as a twig haze now,
  and the young-shoot colour gives it a faint warmth. The wood is still grey
  and paler than the photograph (121 against 100). The trunk divides into
  several co-dominant limbs at about a quarter of the height and spreads into
  a round vase, where the photograph's leader runs up past half its height
  under a narrower, upright crown.
- **S-WHOLE.** Two white stems part at the ground under a curtain of hanging
  strands, and the leaves are the photograph's yellow-green in hue. The whole
  mass is far too dark, 35 against 83, so it reads as a dark ball rather than
  the photograph's sunlit curtain. Red-brown young wood shows in streaks
  through the interior. The curtain stops at the crown's own base, so the tree
  is a round ball on a V where the photograph's curtain falls below the crown
  over the leaning stem's side. That is fn-51's gap, and the darkness is
  fn-52's.
- **S-BARE.** This is the closest pair so far. The hanging shoots draw a brown
  winter haze that reads like the photograph's weeping twigs, where round 7 drew
  a white fog. The veil is uniform, though, and dense enough to swallow the
  white stems about a third of the way up. In the photograph the white main
  stems run up through the crown, and sky shows between hanging clusters of
  unequal length. The uniform length is fn-47's, the symmetric V fn-48's.

No visual pass is awarded. The round-8c pairs are recorded by sha256 in
`round8c-merge/stills.json` with `visual_status: unassessed`, and the owner
records the verdict in fn-34.
