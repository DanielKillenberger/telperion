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

Round 6b answered the first two with steep limbs (32 degrees) and a rising
second order, which made the bare pair read well and the leaf-on pair fail:
a limb leaving low at 32 degrees runs beside the leader to the top of the
crown, so every limb tip - and every leaf, since leaves follow twig wood and
twig wood grows from tips - ended in the upper third, and the long low limbs
carried enough wood to rival the leader. On seed 1, seven stems crossed a
third of the height at more than half the thickest one's radius; 72 per cent
of the leaves sat in the top third; the lowest twentieth of the leaves began
at 47 per cent of the height.

Round 6c reads the leaf-on pair first. The limbs leave at 58 degrees, so a
low limb ends in the low crown; their side branches are held nearly level
(rise 0.1, not 0.4); apical dominance 0.58; the crown starts at a twentieth
of the height, is widest at 0.48 of its depth and rounds at the top (shoulder
1.5). The fork exponent comes down from 2.8 to 2.6: at 2.8 the wood sat on a
cliff, where a few degrees of limb angle collapsed the local layer from about
150,000 nodes to 55,000 and the crown to a skeleton. Four twig laterals a
station fill the crown and a twig length ratio of 0.36 pays for them. Local
shoots are two-ranked (divergence 180, not 137.5), so a spray is flat
rather than a bottlebrush. Leaves lean along their shoots (0.45), lift toward
the light (0.3), scatter 30 degrees and are a tenth larger with less spread,
which keeps the largest leaf inside the sourced 4 to 10 cm; 1.3 times larger
failed that gate. Shoots under a twentieth of the trunk's radius carry
leaves (`canopy.shootRadius` 0.05), which leafs the inside of the crown at no
node cost.

Seed 1, round 6b -> round 6c: stems at a third of the height 7 -> 1; leaves
by third of the height 1/27/72 -> 10/62/28 per cent; lowest twentieth of the
leaves at 47 -> 28 per cent of the height; leaf crown's radius at its top
tenth over its widest 0.61 -> 0.43, a rounded top.

Budget: every one of the 48 protocol cases passes, the heaviest beech seed
is 202,342 nodes of the 250,000 ceiling, none is node-capped, and neither is
any of a further 120 random seeds (heaviest 211,653). The DBH proxy holds at
0.889 m.

Photograph / round 5c -> round 6:

| Reference | Width over height | Crown base | Occupied | Centre mean | Outline |
|---|---|---|---|---|---|
| B-BARE | 0.74 / 0.70 -> 0.72 | 0.25 / 0.27 -> 0.25 | 0.28 / 0.35 -> 0.48 | 100 / 142 -> 126 | 0.29 / 0.08 -> 0.07 |
| B-BASE | 0.60 / 0.67 -> 0.67 | 1.00 / 0.00 -> 0.00 | 0.05 / 0.72 -> 0.10 | 130 / 112 -> 107 | - / 0.38 -> 0.15 |
| B-WHOLE | 0.70 / 0.69 -> 0.74 | 0.12 / 0.08 -> 0.09 | 0.46 / 0.60 -> 0.58 | 80 / 44 -> 41 | 0.23 / 0.10 -> 0.07 |

Seed 1, round 5c -> round 6: 193,836 -> 178,851 nodes; 115,255 -> 84,180
branch axes; 115,080 -> 83,303 twigs; branches per order 5c's four orders ->
{1: 50, 2: 1,055, 3: 9,074, 4: 34,309, 5: 39,692}. Leaf instances at seed 7:
1,520,948 -> 2,106,977, inside the fidelity band.

### Read on the pairs, without a verdict

B-WHOLE: the crown is one full rounded oval on a central trunk, dense from
about a quarter of the height to the top. The flat umbrella and the vase are
gone. Three things still read wrong, and none of them has a row left that
reaches it.

The leaf mass stops at about a quarter of the height where the photograph's
reaches about two metres above the ground; below it our lowest limbs are bare
against the sky. Leaves follow twig wood, and twig wood grows only at
scaffold tips and on wood under `limbRadius` of the trunk's; the lowest
quarter of the crown holds only the trunk and the thick first metres of the
lowest limbs, which leave at 58 degrees and end higher up. `limbRadius` from
0.1 to 1.0 moved the lower third's share of the leaves from 4 to 6 per cent
at a quarter of the twig layer's nodes; hanging the second order (rise -0.4)
took the lowest twentieth of the leaves to 22 per cent of the height but
hung every spray like a cedar's; limbs wider than 60 degrees collapsed the
local layer.

The sprays at the crown's edge still read as fern fronds. A twig is a fixed
25 cm with a leaf every 2 cm - a pinna - and a beech's shoots are nearer 10
cm. Shorter twigs cost a twig node per 15 cm of spray instead of per 25:
twig length 0.15 m node-caps seed 1 even at three laterals a station and
loses a fifth of the leaves. Larger leaves, which would close the comb, are
held by the sourced leaf-size gate. A shoot that bears a cluster of leaves
without a twig node for each would be generator work, not a row.

The foliage is darker and bluer than the photograph's (centre mean 41
against 80). That is appearance, owned elsewhere.

B-BARE: a central leader thick at the base with limbs leaving it along its
height, under a round crown. It reads less like the photograph than round
6b's did: the photograph's limbs leave steeply and its crown is an upright
oval, where ours leave at 58 degrees under a round head, and the leader ends
at three fifths of the height. The pitch is a trade between the two pairs:
32 degrees reads right bare and makes a vase leaf-on, 58 degrees the
reverse. The row that would give both is apical dominance, which ends the
scaffold's leader at `crownBase + (1 - crownBase) * apicalDominance` of the
height: at 0.9 the leader reaches 91 per cent of the height with the same
leaf-on shape. It is held under 0.59 by the host's rule, for the float32
finding below, and by the species test's bound under 0.6.

### A robustness finding for the owner

`surface.rs:351` refuses a wood triangle whose three float32 positions come
out collinear with "surface triangle collapsed in float32", and the refusal
is a hard `InvalidInput`: the seed grows no tree at all. Round 6 met it at
apical dominance 0.59 (protocol seeds 2181184680 and 2779011501), but round
6c's exploration shows it is not an apical-dominance effect: at 0.58, one
candidate table failed protocol seed 55; at 0.9, the same table failed three
of 120 random seeds and none of the protocol's. It tracks geometry, one seed
in a hundred or so, wherever it falls. The shipped table is clean on all 24
protocol seeds and 120 random ones; that is a miss, not a cure. It belongs
to the surface builder, and it is not fn-45's to fix.

No visual pass is awarded; the pairs are in `round6-fn45/stills.json` and on
the judging page, and the owner judges.
