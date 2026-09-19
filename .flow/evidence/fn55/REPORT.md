# fn-55: a physical highlight and a pixel grain

Implemented in-host on the session model (Claude Fable 5.1); the routed
`gpt-6-astra` bridge refused on quota until 2026-09-19 16:27.

## R1: the highlight

One term per material, `highlight()` in `canopy.wgsl`: Schlick's Fresnel at
the half vector on a reflectance row (`barkReflectance`, `leafReflectance`,
default 0.04) over a Blinn-Phong term normalised to the hemisphere,
`(n+2)/8 (n.h)^n`, width from `barkRoughness` on wood (with the existing
roughness-detail and lost-variance terms) and from `1 - cuticleGloss` on the
front face, exponent `2/alpha^2 - 2` with `alpha = roughness^2` floored at
0.01, the implicit geometry term, nothing toward an eye below the surface's
horizon. What the term mirrors is taken from the sun's diffuse,
`sun * (1 - F)`. Native and browser share the shader; the browser preset
mirror is regenerated. `canopy_terms.rs` integrates the term over the
hemisphere on the GPU: a dielectric returns between 1% and 8% of the sun
(chalk and smooth, three views), never more than the sun toward the eye,
nothing at zero reflectance, and the smooth term peaks above the chalk one.
The only pinned still, fn-24's clay room, does not read the material and is
unchanged.

## R2: the grain

Rows `barkGrainScale` (metres) with `barkGrainStrength`, and
`bladeGrainScale` (cells per leaf length) with `bladeGrainStrength`; both a
factor on the colour and a tilt of the normal through `relief_normal`
(`bark_normal` moved into the prelude for both materials), fading an octave
before the mottle. `grain.rs`: the oak trunk at 0.5 m moves 665,919 channels
with the grain on and at 4x distance one channel by one; a leaf at 12 cm
moves 91,468 and at 6 m none.

The bark tables hold the grain off. At the shipped 2 mm and 0.4 the birch's
S-BARK close-up read 4.00 to 4.46 against the smooth-bark resolution bound of
3.0 (base 2.987), and 1 mm cells read 3.40 on B-BASE: a pixel-scale grain
that tilts the normal cannot agree with a 2x2 box reduction at a framing
where a cell is one to two pixels, whatever its fade. The rows stay for the
owner to set once the bound has an answer.

The 4x distance-series test is red: mean 3.134 against 3.0, base 2.904
(2x: 2.463 against 3.0, base 2.385). It is not the highlight: with the
highlight removed entirely the same frame reads 3.167. The old sheen put
15% of the sun on the lit trunk, which sat the frame on the tone curve's
shoulder where the relief's own cross-resolution error compressed into fewer
code values; without it the same linear error reads 0.23/255 more. The
grazing tests hold: oak 2.941 (base 2.946), spruce 2.783 (base 2.754); the
smooth barks hold: beech 1.251 (base 1.249), birch 2.998 (base 2.987);
every redraw is byte-identical.

## R3: the frame

Native hero frames, seed 7, 1600 by 1000, three rounds each, `timing/`,
same session as the base binary, run in sequence:

| Preset | Base total p50 (ms) | After |
|---|---|---|
| Oregon white oak | 5.217, 5.223, 5.228 | 5.208, 5.217, 5.208 |
| Silver birch | 3.368, 3.419, 3.375 | 3.384, 3.381, 3.389 |
| European beech | 10.593, 10.593, 10.596 | 10.531, 10.531, 10.530 |

Inside each preset's spread; the term costs nothing measurable. fn-52
recorded the oak at 3.98 on an uncontended session; this session shared the
RTX 3080 with another live checkout.

## R4: the close-ups

`stills/`: B-BASE, B-BARE, B-WHOLE, S-BARK, S-BARE, S-WHOLE at 1440 tall.
The implementer, having looked: the beech's B-BASE has no sheen band and
reads matte, but smooth like clay rather than grained like the photograph,
the grain being held off. The birch's S-BARK is far less glossy than round
16; the peel strips' lifted edges still carry bright rims down the lit side
of the front stem. Both leaf-on wholes read matte with no white sparkle.
The owner judges.
