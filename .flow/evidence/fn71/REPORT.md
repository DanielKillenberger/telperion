# fn-71: bark relief that reads the same at a distance

Implemented in-host on the session model (Claude Fable 5.1); the routed
`gpt-6-astra` bridge refused on quota until 2026-09-19 16:27, as for fn-55.

## The cause, measured first

A probe decomposed the 2x2 box error of the oak's 4x distance series into
the part a perfectly linear renderer would still show (the tone curve and
sRGB over the sub-pixel variance) and the rest, and took the signed mean.
At 4x the error of 3.134/255 was 0.85 nonlinear floor and 2.60 in linear
light, and the linear part was a bias of -2.50: the far draw is lighter than
the near draw reduced, on the lit side (-2.32) and the shaded side (-3.23)
alike. Plain wood with no relief reads 0.24 and no bias. By term, removing
the cavity took the bias to -1.56 and removing the tints to -2.35; the plate
network and its identity took about 0.6 more. Two mechanisms, both a
nonlinear reading of a filtered height: the slopes a footprint removes still
tilt the wood inside the pixel and the sun's cosine over those tilts
averages below the filtered normal's, and the rectified tints (crest
`max(t, 0)`, fissure `max(-t, 0)`) read at a cell's mean height understate
their mean on every cell the mean crossing runs through. The birch's
close-up, by contrast, has almost no bias (-0.06) and its 2.998 is edge
disagreement from fn-40's plates, peel and tints.

## R1: the distance series

Two renderer terms in `wood.wgsl`, no row and no species branch:

- `facets = inverseSqrt(1 + variance)` scales the sun's cosine and the
  sky's affine share by the mean cosine over the lost slope variance the
  shader already estimated for roughness, and that estimate now counts the
  ridge shoulder's own box filtering: the edge integral keeps the pixel's
  width, so the slope variance it retains falls as sqrt(3/5) of the shoulder
  over the footprint, well before the band fades.
- `bark_rectified(t, spread)` replaces the two kinks with the box mean of a
  ramp across the shading cell, `spread` being half the height the cell's
  own differences span, in the range the tints are read over; the
  constant-height shortcut passes nought and is unchanged.

Oak, mean/255 (2x, 4x): base 2.463 / 3.134; facets alone 2.360 / 2.564;
with the rectifier 2.342 / 2.532. The bias at 4x fell from -2.50 to -0.84,
what is left on the shaded side. Near trunk 1.825 (base 1.813), grazing oak
2.911 (2.941), spruce 2.707 (2.783). The 4x bound is 3.0 again with the
3.25 interval's reason recorded beside it in `bark_distance.rs`.

The spruce is not in the series. Its bare crown throws twig shadows over
the trunk at every height the fixed strip can frame: plain wood with no
relief reads 6.2/255 at 2x about y=0.9, 13.6 about y=2, and the p95 is a
shadow edge's 26 to 42. The spruce's bark is held by the grazing fixture at
2.707; a distance fixture for it needs a pose or mask clear of its own
shadows, which is a follow-up, not a bound this task can honestly claim.

## R2: receipts

Every resolution test writes its fixtures' mean, p95 and bound to
`resolution/<test>.json` on a green run through `common/resolution.rs`,
which also holds the clay-room wood mask the smooth-bark test and the
distance series now share; a receipt that cannot be written fails the test.

## R3: the grain rows

Beech: `barkGrainScale` 2 mm, `barkGrainStrength` 0.3. B-BASE reads 2.37
against 3.0 (1 mm cells read 2.76, on the half-size still's Nyquist edge),
and at 1440 tall a cell is three pixels. Birch: held off, refused by the
bound. With no grain at all S-BARK reads 2.999 against 3.0, its margin
taken by the plates, peel and tints; 1 mm cells are under a pixel at
S-BARK's 1440 and show nothing, 2 mm cells at 0.3 read 3.96. The grain
itself still converges by an amplitude fade rather than a box prefilter,
which is why a cell of one to two pixels cannot agree with the reduction;
an exact box integral of the value noise would make its colour term agree
by construction and is the next step for the birch.

## R4: the close-ups, near and at 4x

`stills/`, 2160 by 1440, seed 1: B-BASE and S-BARK at their fn-34 distance
and at four times it, and both wholes at the hero pose and at four times
its distance. The implementer, having looked at the four close-ups: the
beech's base at 1.2 m reads as a matte, finely grained grey with the lichen
patches and dashes over it, no longer clay-smooth; at 4.8 m the grain has
gone into the tone and the same grey, lichen and dashes read as the same
bark. The birch at 1.5 m is white with the grey-black peel marks and a fine
crackle relief; at 6 m the marks and the vertical relief bands read as the
same material, the crackle gone into the tone. The owner judges in the
harness on the mature path.

## R5: pins and cost

Every redraw is byte-identical (the trunk, grazing and smooth-bark tests
assert it). The one pinned still, fn-24's clay room, does not read the
material and is unchanged. Native hero frames, seed 7, 1600 by 1000, three
rounds each, `timing/`, fn-55's `after` beside:

| Preset | fn-55 total p50 (ms) | fn-71 |
|---|---|---|
| Oregon white oak | 5.217, 5.208, 5.208 | 5.259, 5.252, 5.244 |
| Silver birch | 3.381, 3.389, 3.384 | 3.409, 3.407, 3.422 |
| European beech | 10.531, 10.530, 10.531 | 10.636, 10.638, 10.659 |

About one per cent: the two terms and the beech's grain, now on.

# Round 2: detail leaves at the pixel

The owner rejected round one's R4 in the harness: "it still looks much less
detailed from a distance." The bounds had measured a whole strip's mean
difference at two multiples and could not see the step.

## The sweep

`sweep.py` measures, for six factors from 1.5 to 8, the structure that
survives against the near still reduced by the same factor: the standard
deviation of the luminance minus its box mean at 2, 4, 8 and 16 pixels,
inside the near clay's wood, far over near. One is the near draw minus what
the pixel cannot hold; well under one at a band the pixel still resolves is
detail that left early. Two ways to walk: the camera to f times the distance
(`sweep/round1..3.md`), which turned out to measure the framing as well (a
crop of the same pixel size at f times the distance sees more of a
cylinder's lit side: with the relief off entirely the far draw read 7 to
13% brighter), and the footprint alone (`sweep/res*.md`): the same camera
drawn at a fth of the size, registered pixel for pixel. The footprint walk
is the measure below; the walked stills stay on disk, the curves are
committed.

Beech B-BASE, band ratio far/near at 2 / 4 / 8 / 16 px:

| factor | round 1 (cccd1260) | round 2 (5beb0818) |
|---|---|---|
| 1.5 | 0.93 / 0.94 / 0.94 / 0.95 | 0.94 / 0.94 / 0.95 / 0.96 |
| 2 | 0.93 / 0.94 / 0.94 / 0.95 | 0.95 / 0.95 / 0.95 / 0.96 |
| 3 | 0.81 / 0.80 / 0.82 / 0.86 | 0.90 / 0.90 / 0.92 / 0.94 |
| 4 | 0.62 / 0.61 / 0.70 / 0.78 | 0.85 / 0.85 / 0.90 / 0.93 |
| 6 | 0.62 / 0.63 / 0.74 / 0.82 | 0.82 / 0.83 / 0.89 / 0.92 |
| 8 | 0.70 / 0.72 / 0.81 / 0.85 | 0.79 / 0.81 / 0.88 / 0.91 |

The step at 3x to 6x, where the beech's 2 mm ridges and 2 mm grain crossed
the pixel and were cut to their mean an octave early, is gone; the mean
holds within 2% along the walk (round one held it within 0.5%, round two
darkens the far draw by up to 2.2% at 8x). The birch S-BARK reads 1.00 to
1.03 at 1.5x to 3x and 0.91 to 0.95 at 8x, its mean flat. A relief-free
beech (`ridgeScale` 0) reads 0.71 to 0.76 at the 2 px band from 4x on, so
what is left of the beech's loss at 4x to 8x is not the relief: the lichen
patches and lenticel dashes are integrated across their rim as one edge,
which stands for a disc only while the box is narrower than the spot, and
their small members leave early through that. A disc's own box integral is
the next step there.

## What changed

- The grain is box-averaged over the pixel's extent on each axis,
  `bark_noise2_box` in `common.wgsl`: the exact integral of the lattice
  noise's basis over the box (sixteen values at most), so a half-size draw
  reads the mean of the four full-size samples it stands for and the grain's
  colour agrees across resolution by construction; the little that a box
  over a cell and a half leaves goes to the mean by two cells. Its fade an
  octave before the mottle is gone.
- The ridge band's fade runs from one ridge width a pixel to two
  (`bark_pass(0.5 * scale_pixel)`), its edge integral being its box filter;
  the constant-height shortcut moves with it. The lichen fades from two
  pixels across to one; a lenticel dash leaves by its length, never its
  thickness, its rim across the thin axis being the edge integral.
- The tints and the cavity read over the range the band's fade took out of
  the height, `lost` in `wood.wgsl`, scaled by the furrow row: a furrowed
  field stands at floor or face and spans its range, one without furrows
  carries its plate faces and flakes and spans half of it (the beech's near
  fissure reads 0.14; a uniform half-range of one gave 0.25 and darkened the
  far draw 5%). The cell ramp's half-spread is half the height the cell's
  differences span.

## R3: the grain rows again

Beech 2 mm at 0.3: B-BASE reads 1.40 against 3.0 (round one 2.37; the box
integral agrees where the fade did not). Birch 2 mm at 0.15: S-BARK reads
2.90 against 3.0, 2.75 with no grain; 0.2 read 2.999 and 0.3 read 3.24, the
grain's tilt of the normal being the nonlinear part the reduction cannot
match, and 3 mm at 0.2 read 2.92. Every resolution test is green with its
receipts under `resolution/`; the oak's series reads 2.30 and 2.52.

## R5: pins and cost

Every redraw byte-identical, the clay room unchanged. Native hero frames,
seed 7, 1600 by 1000, three rounds, `timing/round2-*`:

| Preset | fn-55 | round 1 | round 2 |
|---|---|---|---|
| Oregon white oak | 5.217, 5.208, 5.208 | 5.259, 5.252, 5.244 | 5.917, 5.904, 5.889 |
| Silver birch | 3.381, 3.389, 3.384 | 3.409, 3.407, 3.422 | 3.976, 3.981, 3.989 |
| European beech | 10.531, 10.530, 10.531 | 10.636, 10.638, 10.659 | 11.358, 11.357, 11.358 |

Seven to seventeen per cent over round one: the grain's three box integrals
a fragment (the birch's grain is on now), the ridge band drawn out to two
widths a pixel, and the dashes and patches drawn further out. The grain's
gradient can come from one integral rather than three, a follow-up if the
owner sets a bound.

## R4

The owner judges the walk in the harness on the mature path; the browser
modules were rebuilt for the dev server on port 5174. The implementer,
having looked at the registered 3x pair of the beech base (near reduced
against far drawn): round one's far draw was smoother and lighter, its grain
gone; round two's carries the same fine grain as the reduction and reads as
the same bark.
