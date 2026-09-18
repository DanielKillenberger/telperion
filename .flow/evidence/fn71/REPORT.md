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

# Round 3: the relief leaves by its box integral alone

The owner rejected round two: "there's still a jump where the relief
visibly disappears. It's a bit further away now but still there", and sent
two harness frames of the silver birch one wheel notch apart (`PUSH_PER_NOTCH`
1.12 in `harness/orbit.ts`), the trunk 180 to 250 pixels tall: at the closer
notch the dark lenticel dashes and peel marks are on the trunk, one notch
out they are gone. At that footprint (sixty to eighty millimetres of bark a
pixel, the sum of both derivatives) round two's fades were crossing: the
dash's, from thirty to sixty, and the plate network's, from twenty-five to
fifty. A fade window is a jump wherever the walk crosses it.

## What changed

Every amplitude fade is gone from the relief, the plates, the dashes and
the grain; each term leaves by the box integral of its own shape over the
footprint, or by more reads of it inside the pixel.

- The ridge band and the plate network are read only at footprints their
  edge integrals stand for, under a ridge width and under half a plate, and
  a wider pixel is shaded as more cells sharing a lattice of heights: two a
  side under a ridge width, three under three, four beyond, each cell's
  footprint capped at a width and the cells standing apart past four.
  `survives`, the plate band's `bark_pass`, `lost` and the constant-height
  shortcut at two widths are gone. The lost-slope roughness and facet term
  now takes the cell's own footprint, the home for the shading the height
  integral removes.
- A lenticel dash is a rounded bar, its box integral the product of one
  span on each axis (`bark_span`, the exact box integral of a rimmed
  interval), summed over the cells the box reaches, which grow with it;
  dashes that overlap cover as independent covers do. Its groove keeps to
  the cells of its own row, the relief reading it a dozen times a pixel.
- The grain's box window grows with the footprint to six cells, and its
  gradient comes from the same integral (`bark_noise2_box` returns the
  value and its two slopes), one integral a field instead of three.
- Wood reads its means where no read can resolve anything: a pixel spanning
  six ridge widths or three plates, where four cells stand a width and a
  half apart and their estimate's own noise is above the box's residue (a
  sixth of the relief's deviation), a twig whose pixel spans its own
  radius, and dashes on wood whose pixel spans half its radius, a quarter
  of its circumference. These are the cost bounds, not fades; the residue
  they drop is under two code values.
- The lichen keeps its fade from two pixels across to one, a follow-up: its
  patches are spheres cut by the surface, read in twenty-seven cells, and a
  disc's box integral with a growing window is the next step there.

Four tests pinned the fade design and were re-pinned on the box-integral
contract: `bark_filter` (an unresolved axis keeps under a tenth of the
relief, not exactly none), `grain` (the far draw's move is under a fifth of
the near one's, not exactly none, at eight times the distance), and
`smooth_means` (the mean of far reads at every probe against the near mean,
not one constant against it; the strip's box read carries a five per cent
higher relief mean than its point read, a product of two edge integrals a
box does not commute with, present in every mid-distance read before and
now measured; the groove's one-row window misses six per cent of a groove a
millimetre deep).

## The sweep

Footprint walk (`sweep/res6.md`, `sweep/hero6.md`), band ratio far/near at
2 / 4 / 8 / 16 px; the seam would be a kink between two factors.

| factor | beech B-BASE round 2 | beech B-BASE round 3 | birch S-BARK round 3 |
|---|---|---|---|
| 1.5 | 0.94 / 0.94 / 0.95 / 0.96 | 0.94 / 0.95 / 0.96 / 0.97 | 1.03 / 1.03 / 1.01 / 1.00 |
| 2 | 0.95 / 0.95 / 0.95 / 0.96 | 0.97 / 0.97 / 0.98 / 0.99 | 1.03 / 1.03 / 1.00 / 0.99 |
| 3 | 0.90 / 0.90 / 0.92 / 0.94 | 0.96 / 0.97 / 0.98 / 0.99 | 1.01 / 1.01 / 0.98 / 0.98 |
| 4 | 0.85 / 0.85 / 0.90 / 0.93 | 0.95 / 0.96 / 0.97 / 0.98 | 0.99 / 0.98 / 0.96 / 0.97 |
| 6 | 0.82 / 0.83 / 0.89 / 0.92 | 0.94 / 0.95 / 0.97 / 0.98 | 0.96 / 0.95 / 0.95 / 0.96 |
| 8 | 0.79 / 0.81 / 0.88 / 0.91 | 0.94 / 0.95 / 0.97 / 0.98 | 0.94 / 0.93 / 0.93 / 0.95 |

No adjacent pair of factors differs by more than 0.03 at any band; the
beech's mean holds within 1.3% along the walk and the birch's within 1.6%.
The birch at the hero pose (1350 by 900, factors 1.12, 1.25, 1.5, 2, 3, 4,
the owner's notch first) reads 1.00 to 1.08 at every band with the mean
within 0.4%: the dashes and marks stay through the notch. The implementer,
having looked at the registered pairs: the birch trunk at the hero pose one
notch out and at twice the footprint carries the same dashes and peel marks
as the near draw reduced, dimmer and wider where the pixel is wider, and the
beech base at four times reads as one bark.

## R3 and receipts

Beech B-BASE 1.04 against 3.0, birch S-BARK 2.54 (its 2 mm grain at 0.15
kept), oak 2x 2.23, 4x 2.84 (the facet term now takes the cell's footprint,
half the pixel's, and gives back a third of the round-one bias; still under
the bound), grazing 2.89 and 2.70, every redraw byte-identical.

## R5: cost

Native hero frames, seed 7, 1600 by 1000, three rounds, `timing/round3-*`:

| Preset | round 2 | round 3 |
|---|---|---|
| Oregon white oak | 5.917, 5.904, 5.889 | 9.98, 10.02, 10.12 |
| Silver birch | 3.976, 3.981, 3.989 | 14.96, 15.22, 14.96 |
| European beech | 11.358, 11.357, 11.358 | 19.56, 19.48, 19.97 |

The first no-fade build cost 18, 38 and 236 ms; the twig and sparse-wood
means and the groove's row brought it here. What remains, by switching rows
off: the birch's dashes about 8 ms (a square window of cells filtered to
the ring, read once a fragment on every branch wider than two pixels), its
grain 3 ms, the cells 2 ms; the oak's plates 4 ms (the network read sixteen
times a pixel on limbs seen at grazing angles). A dash window that walks the
arc, the groove read once a fragment and shared by the cells, and a grain
window capped at four cells are the reductions, follow-ups if the owner
sets a bound.

# Round 4: cost, under the accepted picture

The owner accepted round three ("yea much better"). This round changes
cost only; the invariant is the footprint sweep flat within 0.03 between
adjacent factors on the beech base, the birch bark and the birch hero notch,
the resolution receipts green, redraws byte-identical, no fade.

## What changed

- The groove is read once a fragment at the pixel's footprint and shared by
  every cell whose footprint across the wood is over twice the groove's
  depth; a narrower cell reads its own. The bowl is under the cell either
  way.
- The dash window reads exactly the rows a box can touch (a row's dashes
  lie within their site's wander and their half-height of the row's middle,
  so a box under two thirds of a pitch reads one row, where the round-three
  window read three or five) and the cells across the arc within the box's
  half-extent and a dash's reach, whole cells up.
- The grain's window is capped at three cells (round three: six), the
  residue past it a third of the noise's deviation, which the sweep does not
  see.
- The cells are sized per axis, two or three on each, so a grazing pixel,
  long one way and a fraction of a width the other, takes its cells only
  along its length; four cells a side are gone (the sparse mean stands at
  six widths, unchanged).

The sweep (`sweep/res8.md`) holds the invariant: the widest step between
adjacent factors is 0.030, on the beech's 2px band from 1.5 to 2, and that
is round three's own widest step, in the same cell. Against round three
(`sweep/res6.md`) the curve moves in five of its seventy-two cells, by 0.01
each: the beech at 8x on the 8px and 16px bands, the birch at 3x on 4px, the
hero walk at 1.5 on 2px and at 3 on 8px. Every receipt is green: oak 2x
2.23, 4x 2.84, beech 1.06, birch 2.54, grazing 2.89 and 2.70.

## Cost

Native hero frames, seed 7, 1600 by 1000, three rounds, `timing/round4-*`:

| Preset | round 2 | round 3 | round 4 |
|---|---|---|---|
| Oregon white oak | 5.917, 5.904, 5.889 | 9.98, 10.02, 10.12 | 8.91, 8.92, 8.91 |
| Silver birch | 3.976, 3.981, 3.989 | 14.96, 15.22, 14.96 | 7.00, 7.08, 7.12 |
| European beech | 11.358, 11.357, 11.358 | 19.56, 19.48, 19.97 | 14.59, 14.63, 14.57 |

By term, a row switched off with `--family` (the per-term cost line the
friction entry asked for is this table by hand; a counter in the timing
receipt is not cheap, the shader would have to count its own hashes):

| Preset | all | no grain | no plates | no lenticels | no relief |
|---|---|---|---|---|---|
| Oregon white oak | 8.9 | 9.0 | 5.2 | 9.0 | 3.5 |
| European beech | 14.6 | 13.4 | 14.7 | 12.8 | 13.7 |
| Silver birch (round-4 first build, 9.4) | 9.4 | 7.4 | 8.6 | 5.2 | 8.1 |

What holds the rest, and why it stays: the oak's 3 ms over round two is the
plate network, twenty-seven hashes a read, read nine to sixteen times a
fragment on limbs whose pixel spans two to six ridge widths, where round two
read it once and drew the mean; that band is exactly the relief the owner
asked to keep, and the sparse mean at six widths is the invariant's edge.
The birch's 3 ms over round two is the dashes on every branch wider than two
pixels (about 2 ms, their box integral summed over the cells the box
reaches) and the grain's box window (about 1 ms); the beech's 3 ms is the
same two terms on its twigs. A cheaper dash would need its cells walked
along the arc with an exact dedupe, which is not a smaller read of the same
integral but a different search; the grain's window is at three cells, the
floor before its residue shows in the sweep.

## Gates

Re-run on this tree after the round's edits, all green: `cargo fmt --all --
--check` (its only diffs are the three files master's fn-58 tooling commit
left unformatted, `telperion-core/examples/species_measure.rs`,
`telperion-core/src/params/tests.rs` and `telperion-jev/src/bin/jev.rs`,
untouched by this branch), `cargo clippy --release --workspace --all-targets
-- -D warnings`, `cargo test --release --workspace` (the resolution tests
rewrote the four receipts above to the same values), `npm test` and `npm run
typecheck`.
