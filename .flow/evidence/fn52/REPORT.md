# fn-52: a leaf mass lit as a canopy

The leaf-on pairs drew the leaf mass far darker than the photographs. On
S-WHOLE (sun) the centre read 35 against 83. On B-WHOLE (overcast) it read 45
against 80. One per cent of the leaf pixels sat above half brightness, where
the photographs have a fifth. This record has two parts. R1 is why a leaf in
the matched light got so little of it. The rest is the four rows that now
light a leaf as part of its crown, the pairs rendered again, and the frame
cost.

## R1: the diagnosis, term by term

### Method

A temporary patch, `diagnose.patch` here, feeds a debug mode into the uniform
block's one reserved float, `leaf_colour_detail.w`, from `TELPERION_DEBUG_TERM`.
The foliage fragment then writes one term in place of the lit colour. Mode 0
is the production frame, and it reproduces round 8c's S-WHOLE and B-WHOLE
stills byte for byte (`11764aee…`, `41e8a77b…`). Mode 1 paints leaves magenta
and wood cyan over a black sky and ground, so MSAA resolves each pixel to its
leaf and wood coverage. A fully covered leaf pixel (leaf coverage above 0.9)
is a leaf pixel.

Every term was rendered at the matched shot, the first fixed seed and the
1440 rig, with the camera, sun and overcast built as `tests/species.mjs`
builds them. Radiance terms are written linear and divided by four, then
decoded from the sRGB target, so they are read before the exposure and the
tone curve. `terms.py` reads them, and its outputs are `terms-S-WHOLE.json`
and `terms-B-WHOLE.json`. `bins-*.txt` holds the same terms by sixths of the
crown. Brightness is the mean of the three sRGB channels, 0..255, and "above
half" is that mean above 127.5, as the compare script's centre mean reads it.

### Per-term means over the leaf pixels

Linear radiance in the green channel. The first figure is over all fully
covered leaf pixels, the second over those inside the centre crop, the middle
two fifths of the tree box.

| Term | S-WHOLE (sun) | B-WHOLE (overcast) |
|---|---|---|
| Albedo, as drawn | 0.295 / 0.296 | 0.254 / 0.247 |
| Sun, before the shadow map | 0.093 / 0.060 | 0.0081 / 0.0097 |
| Sun, as drawn (shadowed) | 0.023 / 0.0095 | 0.0015 / 0.0006 |
| Sky, no occlusion | 0.058 / 0.060 | 0.049 / 0.052 |
| Sky, as drawn (fn-29's crown-depth occlusion and interior darkening) | 0.053 / 0.053 | 0.041 / 0.039 |
| Transmission, as drawn (forward lobe) | 0.0066 / 0.0017 | 0.0035 / 0.0005 |
| Transmission if diffuse, at the row's strength and thickness | 0.082 / 0.041 | 0.0137 / 0.0030 |
| Sun specular (cuticle) | 0.0058 / 0.0023 | 0.0004 / 0.0001 |
| Total before tone | 0.089 / 0.066 | 0.046 / 0.040 |

The leaf pixels as drawn read 39.7 on S-WHOLE, with 2.4% above half, and 21.1
on B-WHOLE, with none above half.

### The shading normal, the back face and the leaves facing the sun

- **The flip is right.** The shading normal faces the eye at every leaf pixel
  of both stills. Its dot product with the face normal taken from the
  fragment's own screen derivatives averages 0.99 and is positive everywhere.
  So there is no sign or handedness fault between the winding, the vertex
  normal and `front_facing`. The placement's basis is right-handed (side ×
  axis = face) with one uniform scale, so no leaf is mirrored.
- **The eye sees undersides.** Both cameras stand below the crown, at −12° and
  −15°. Of the leaf pixels, 66% on S-WHOLE and 79% on B-WHOLE show the back
  face. That face's normal turns toward the ground half of the hemisphere,
  whose light is the ground's 0.11 times the sky.
- **Few leaves face the sun and see it.** On S-WHOLE the seen face turns
  toward the sun at 42% of leaf pixels. Their shadow visibility is 0.27, so
  16% of leaf pixels are both sun-facing and unshadowed. On B-WHOLE the
  figures are 32%, 0.20 and 7%.
- **Where the sun reaches.** By sixths of the crown on S-WHOLE, visibility is
  0.74 at the sunward edge and 0.88 along the top row. It is 0.21 to 0.26
  across the middle columns and 0.04 along the bottom row. The shot puts the
  sun 90° to the camera's left, 55° high, with the eye 12° below the aim. The
  crown's outward direction at the middle of the visible disc turns 0.17 away
  from the sun, so that middle stands just past the crown's own terminator.
- **The shadow's normal offset is not the cause.** Offsetting the receiver
  toward the sun rather than toward the eye moves visibility from 0.314 to
  0.325 (0.353 to 0.368 on back-seen leaves). The shadow map is left as it
  is, as the boundary asks.

### Exposure and tone against wood and sky

One `tone()` in `common.wgsl` maps the sky, the ground, the wood and the
leaves alike: an exposure of 0.6, then Narkowicz's ACES fit. The foliage
takes no curve of its own and no unit of its own.

| | S-WHOLE | B-WHOLE |
|---|---|---|
| Sky, top of frame (still / photograph) | 165 / 147 | 189 / 255, clipped |
| Wood pixels, as drawn | 40.7 | 62.3 |
| Leaf pixels, as drawn | 39.7 | 21.1 |

Under this curve a birch leaf's front row, square to the unshadowed sun,
reads 150, and at a cosine of 0.5 about 110. Only unshadowed faces turned
nearly square to the sun cross half brightness, and the card model gives 16%
of leaf pixels any sun at all. The photograph of B-WHOLE is exposed brighter
against its sky than the renderer's fixed exposure: its overcast sky clips to
white where ours reads 189. This is recorded, and the instrument is left
unchanged.

### How the overcast row reaches foliage

`sceneOf` in `tests/species.mjs` dims the sun to 1 − 0.8·overcast, which is
0.28 at B-WHOLE's 0.9, and draws the zenith toward the horizon colour. Both
reach the leaves through the same `u.sun`, `u.sky_zenith` and `u.sky_horizon`
that `ambient()` and `key()` read for the wood and the ground. On B-WHOLE the
dimmed sun gives the leaves 0.0015 of their 0.046. The row reaches foliage as
it reaches wood, and it is not the cause.

### Centre-crop composition

MSAA coverage over the centre crop:

| | Leaf | Wood | Background |
|---|---|---|---|
| S-WHOLE | 0.478 | 0.507 | 0.015 |
| B-WHOLE | 0.606 | 0.331 | 0.063 |

Half of S-WHOLE's centre is the curtain's wood, at 41, so the centre mean is
capped by wood no leaf row reaches. That wood is fn-47's and fn-51's.

### Findings

No fault was found: no sign, no handedness, no unit and no term read from
the wrong uniform. Nothing was fixed as a bug, and no pin moves. The darkness
is what the card model leaves out, in order of size:

1. **Transmission leaves only along the sun's line.** `transmitted()` scales
   by the square of the eye's cosine to the sun's line. At S-WHOLE's 90° sun
   that factor is 0.17² = 0.03. The sunward leaves face the sun and show the
   camera their backs, so they get almost nothing, where a thin leaf
   transmits nearly evenly. Diffuse transmission alone would carry 0.082,
   more than the sun on the faces (0.023).
2. **No sky passes through a leaf or reflects off one.** Under overcast a
   leaf's only light is the hemisphere over its seen face. The eye sees
   undersides, so that hemisphere is mostly the ground.
3. **Each card is lit alone.** Its own orientation decides whether it sees
   the sun, and only 16% (S) and 7% (B) of leaf pixels do, so the mass reads
   as a dark speckle and not as a volume lit on one side.
4. **On S-WHOLE the wood and the light's geometry cap the centre**:
   half the crop is curtain wood at 41, and the visible middle stands just
   past the crown's terminator.

The spec's canopy terms answer 1 to 3: a lighting normal bent toward the
crown's outward direction, the sun wrapped past the terminator, a diffuse
share of transmission that carries the sun and the sky through the leaf, and
a sheen that returns the sky toward grazing.

## R2: the canopy rows

Five rows on the material, each refused by name outside its rail, carried
on the wire, walked linearly by the blend, present in the regenerated
browser metadata and covered by the harness's panel test. Each is inert at
zero. The WGSL terms live in `canopy.wgsl` as pure functions, concatenated
after the transmission term, so the native and browser renderers compile
the same source. Each term sits behind a uniform branch, and the clay view
reads none of them.

| Row (wire) | Rail | What it does |
|---|---|---|
| `canopyNormal` | 0..1 | Bends the lighting normal from the leaf's face toward the crown ellipsoid's outward direction at the fragment, so the sunward shell is lit whichever way its leaves turn. A leaf with no crown around it is not bent. |
| `lightWrap` | 0..1 | Wraps the sun's cosine past the terminator, raised by the wrap and divided by one plus it. A face square to the sun takes exactly what it took before. |
| `diffuseTransmission` | 0..1 | The share of the leaf's transmission that leaves it evenly rather than on the forward lobe. The diffuse share carries both the sun, gated by the shadow map, and the sky behind the leaf. |
| `leafSheen` | 0..0.5 | The cuticle's reflectance of the sky at normal incidence, rising by Schlick's Fresnel to the whole sky at grazing, on the face's own normal. |
| `crownShade` | 0..1 | The share of the sky that one crown radius of leaves takes from each leaf's sky reads (over it, behind it, in its sheen), by the chord of crown standing straight over the leaf. |

The bend reads light arriving through the mass: sky, sun and transmission.
What the face reflects, the sun's glint and the sheen, stays on the face
normal. A first version took the sheen on the bent normal. It matched
B-WHOLE's share above half brightness (13.5% against 12.4%), but on the pair
the ellipsoid became a mirror along its silhouette, and every frond at the
lower rim returned the horizon sky, grey and ghostly, where the photograph's
lower crown is dark green. The face-normal sheen replaced it.

The fifth row came from reading round 10's first rows by sixths of the
crown. The leaves lit, but they did not fall off. B-WHOLE read 93 at the top
and 69 at the bottom, where the photograph falls from 125 to 52. The diffuse
share carried the whole sky into the dome's underside, since fn-29's
crown-depth occlusion is nought at the shell.

### Neutral is inert

`neutral-before.sha` and `neutral-after.sha` hold 25 stills: the seven
presets whole, bare and leaf at seed 1, and the four matched stills B-WHOLE,
B-BARE, S-WHOLE and S-BARE. They were rendered by the base commit and by the
final code. With the beech's and birch's rows at zero, all 25 hash the same.
With them set, six move, exactly the six the rows reach: the beech's and
birch's whole and leaf stills, B-WHOLE and S-WHOLE. The four bare and
close-up matched stills are byte-identical to round 8c's records. No bug was
fixed, so no pin moved.

## R3: the rows set, and round 10

| Row | Beech | Birch |
|---|---|---|
| canopyNormal | 0.8 | 0.8 |
| lightWrap | 0.4 | 0.5 |
| diffuseTransmission | 1.0 | 1.0 |
| leafSheen | 0.08 | 0.06 |
| crownShade | 0.15 | 0.2 |

Photograph / round 8c → round 10, on the leaf-on pairs:

| | S-WHOLE | B-WHOLE |
|---|---|---|
| Centre mean | 83.0 / 35.3 → 58.5 | 79.7 / 44.9 → 65.6 |
| Centre crop above half | 21.8% / 1.8% → 3.1% | 12.4% / 7.0% → 7.2% |
| Leaf pixels above half | — / 2.4% → 17.7% | — / 0.0% → 1.0% |
| Leaf pixel mean | — / 39.7 → 87.7 | — / 21.1 → 67.9 |
| Centre-crop leaf pixel mean | 83.0 (whole crop) / 33.7 → 71.2 | 79.7 (whole crop) / 17.9 → 53.1 |
| Leaf brightness by sixths, top to bottom | 153 111 97 68 57 38 / 48 40 38 36 37 36 → 156 114 89 67 55 45 | 125 100 81 75 55 52 / 24 19 17 20 25 27 → 91 70 54 52 55 55 |

The photograph's figures are over its whole centre crop, or its tree box's
middle three fifths by sixths with pixels above 200 left out as sky. It has
no leaf mask. The still's leaf pixels come from the diagnosis's coverage
mask; the geometry did not move, so the mask holds. `bright.py` and `rows.py`
compute these figures, and `bright-round10.json` holds them.

- **S-WHOLE.** The leaf pixels now sit where the photograph's do. About a
  fifth are above half (17.7% against the crop's 21.8%), the centre's leaf
  pixels read 71, and the fall from top to bottom follows the photograph's
  within ten at every sixth. The centre mean, 58.5 against 83, misses by
  24. Half of that crop is the curtain's wood at about 41, and no leaf row
  reaches it (fn-47, fn-51).
- **B-WHOLE.** The centre mean is 65.6 against 79.7, 14 short. The hue is
  the photograph's (sRGB 57/78/62 against 71/94/74), and the fall from top
  to bottom has the photograph's shape. The level is short at the top two
  sixths, and so is the share above half, 1% of the leaf pixels. The
  photograph's bright centre pixels are a pale, sky-lit green (mean
  149/180/152, 93% green-led, 7% near-white sky). The photograph's camera
  exposed its overcast sky to a clipped white, where the renderer's fixed
  exposure draws it at 189. The beech's leaf rows (front green 0.105) under
  this exposure do not reach that level. Neither the leaf colour nor the
  instrument is this spec's to move.

## R4: the frame

The oak's native hero frame, seed 7, 1600 by 1000, is the command fn-29's R6
used. The base commit's binary was interleaved with this branch's, in two
sessions on the shared RTX 3080. The oak ships neutral rows. "Every term on"
drives the oak with all five rows set (0.8, 0.5, 1.0, 0.08, 0.2) through a
temporary material override, not committed.

| Total p50 (ms), median of the session | Base commit | Shipped oak | Every term on |
|---|---|---|---|
| Session 1, 5 rounds (`timing/f-*.json`) | 4.0100 | 3.9990 | 4.0074 |
| Session 2, 7 rounds (`timing/g-*.json`) | 3.9785 | 3.9798 | 3.9836 |
| fn-29's accepted, `.flow/evidence/fn29/oak-native-timing.json` | 3.9823 | | |

Session 1 was contended: the unchanged base binary itself read 4.01 in it.
In session 2 the shipped oak's frame is 3.9798 ms, at fn-29's accepted
3.9823. With every term on it is 3.9836, 0.0013 ms above the accepted figure
and 0.005 ms above the base in the same session, inside that session's own
spread of 3.96 to 4.20. The terms cost nothing at neutral and nothing
measurable when on.

The browser orbit uses `orbit.mjs`, which is `tests/browser/render.mjs`'s own
timing session run on its own. It orbits the hero pose on a bare canvas at
1600 by 1000 in hardware Chromium (Vulkan), seed 7, on a 100 Hz display.

| Preset | Wall p50 / p95 / worst (ms) | Frames | GPU total p50 (ms) |
|---|---|---|---|
| Oak (neutral rows) | 10.0 / 10.1 / 10.7 | 999 | 4.05 |
| Silver birch (rows on) | 10.0 / 10.1 / 20.2 | 988 | 4.90 |
| European beech (rows on) | 10.0 / 10.1 / 20.1 | 992 | 6.81 |

fn-29 recorded the oak's orbit at 10.00 / 10.10 / 10.10 with a GPU total p50
of 4.10. Every orbit holds 60 fps: p95 under 16.7 ms and no frame past
33 ms.

## R5: the tests

- `crates/telperion-render/tests/canopy_terms.rs` puts the production terms
  to a synthetic leaf on the GPU. The wrap is exact at zero, lights a face
  just past the terminator and leaves a face square to the sun alone. The
  canopy normal turns a face that looks away from the sun toward a sunward
  shell, and is the face's own with no bend or no outward. Diffuse
  transmission reaches a leaf lit from behind at the sun's cosine plus the
  sky, and only the sky when the leaf faces the sun or is shadowed. The
  sheen runs from its reflectance to all of the sky at grazing, and is
  nothing at zero. The crown chord runs under, at the centre, at the top
  and beside the crown, and the shade takes its share per radius.
- `crates/telperion-render/tests/canopy_light.rs` covers real frames. On a
  synthetic shell of 6,000 seeded leaves, the canopy normal makes the
  sunward side lead the far side, and the crown shade darkens the
  underside more than the top. One leaf facing and facing away from the sun
  gains from diffuse transmission more when it faces away, and gains from
  the wrap and the sheen. The canopy normal leaves a lone leaf byte for
  byte. The clay view and the bare view are byte-identical with every row
  on.
- `crates/telperion-core/tests/material_detail.rs` covers the rows. Both
  rail ends are refused by name on the wire, the endpoints round-trip, the
  walk hits the interior point, and older documents gain inert zeros. Every
  shipped canopy row crosses the page's wire as the native still reads it,
  and only the beech and the birch state one; this is the native and
  browser agreement, since both hand the parsed material to one renderer.
  `material.rs`'s refusal table covers each row's name.
- `harness/material-detail.test.ts` carries the five controls through the
  panel.
- `crates/telperion-render/tests/conformance.rs` now also scans
  `canopy.wgsl` for family and anatomy names.
- Neutral byte identity: the 25-still record above, and the pinned clay
  still in `look.rs`.

## What the implementer saw on the pairs

The four-image rule was kept per capture: two pairs of the first round-10
rows, two comparisons of the sheen's normal, and the two final pairs.

- **S-WHOLE: yes, as a lit mass.** The upper shell and the sunward side read
  a bright yellow-green, the crown falls into shade toward its base, and
  the dark speckle of round 8c is gone. What still reads wrong is the band
  of red-brown curtain wood through the upper middle, which is half the
  centre crop (fn-47, fn-51); the hem stopping at the crown base (fn-51);
  and a softer, more even texture than the photograph's clustered
  highlights.
- **B-WHOLE: lit, but not one mass.** The leaves are the photograph's
  mid-green, lighter on the sky-lit top and darker toward the bottom. The
  crown is still an umbrella of sprays at the limb ends, with grey limbs
  and sky through its lower half (fn-50's short shoots and the vase pass),
  so it cannot read as the photograph's single dense mass. The mass is also
  a little flatter and dimmer than the photograph's: no dark interior
  clumps under bright tops, and 66 against 80.
