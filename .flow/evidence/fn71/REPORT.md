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
