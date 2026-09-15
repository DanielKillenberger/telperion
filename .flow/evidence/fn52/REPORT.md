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
