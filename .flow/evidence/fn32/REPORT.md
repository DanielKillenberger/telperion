# FN32: bark as plates and scales

2026-09-14, one session. **NEEDS_HUMAN: R6 is the owner's judgment and R7's
bound is the owner's to set.** R1 through R5 are implemented, green and
measured. The oak's native frame with every term on is **4.5192 ms total
p50**, 4.9339 ms p95, against fn-29's accepted 3.9823 / 4.5814: **+0.5369 ms,
+13.5%**. No bound was moved, because this spec has none until the owner sets
one. All five gates exit 0 with zero adapter skips.

## What changed

The bark field gains a second primitive. Ridges are parallel: they cannot
branch, cannot merge, and cannot give one plate an identity. A cellular
partition does all three. The circumference is partitioned by the same circle
embedding `bark_column` already uses, so the network crosses the angular wrap
without a seam; each column is then cut along its run into plates at heights
of its own, and the distance to whichever boundary is nearer is the furrow.
The axial distance is measured in runs and the circumferential one in widths,
so the elongation brings the cross-cut back to the same metre: a plate three
times as long does not get walls three times wide.

Cell size carries girth, so an old trunk wears big plates and a young limb
small ones off one row. A face domes from its own edge towards its middle and
a rim stands off the furrow beside it. Each plate hashes to an identity that
says how proud it stands, how far it leans across its own run, and what value
and cast it keeps. The oak reads as long blocks in furrows and the spruce as
round scales lifting at their edges, by row values with no species branch.

One cut across a whole column is a straight course, and a wall of them is
brickwork, which is what the first inspected capture showed. The axial
coordinate is ragged at the plate's own scale before the cut is found, which
bends each cut as it crosses its column. That is capture 3 and the one kept.

Colour follows the structure. A weathered face is greyed and lightened against
the fresher wood a furrow keeps; the side turned from the sun and the foot of
the trunk take a colour of their own; and each plate carries a gain per
channel so its value and its cast move together. Every one of these is affine
in the filtered height per fn-29's rule: the weathering offset is a constant
vector computed off the base row, not off the tinted albedo, and the plate's
gain is a linear map on the albedo. Nothing introduces a height squared.

A furrow floor is darkened by its own crest. The height field is walked across
the surface towards the sun and what the bark rises to is compared with what
the sun ray rises to over the same ground; where the bark wins, the floor is
in shadow. No new light, no second map, and two field samples beside the nine
the fragment already holds.

Depth beyond the shaded normal is parallax, chosen against vertex displacement
by measurement. The comparison is below and in `depth-comparison.json`.

## Wire rows and calibration

Fifteen numeric additions, every one inert by default, so an existing document
renders exactly as it did. Range refusals name the offending field; the blend
walks them; `src/browser/presets.generated.ts` and `README.md` carry them.
The Two Trees take the defaults: their bark field is off at the ridge scale,
so a plate network over it would be dead arithmetic.

| Wire name | Range | Ordinary | Oak | Spruce |
|---|---|---|---|---|
| plateCellScale | 0-1 m | 0 | .09 | .022 |
| plateElongation | 0-16 | 0 | 1.8 | .2 |
| plateDome | 0-1 | 0 | .5 | .35 |
| plateEdgeLift | 0-1 | 0 | .22 | .5 |
| plateIdentity | 0-1 | 0 | .55 | .4 |
| weatheringStrength | 0-1 | 0 | .5 | .7 |
| weatheringRed/Green/Blue | each -1-1 | 0,0,0 | .055,.05,.042 | .035,.025,.016 |
| orientationStrength | 0-1 | 0 | .4 | .45 |
| orientationRed/Green/Blue | each -1-1 | 0,0,0 | -.022,.024,-.016 | -.014,.026,-.008 |
| directionalOcclusion | 0-1 | 0 | .55 | .55 |
| depthStrength | 0-1 | 0 | .6 | .45 |

The shader's own constants are pinned beside them: a plate's wall is 0.14 of
its width and its depth 0.045 of it, which at the oak's 9 cm plate is a 12.6 mm
wall and a 4.1 mm relief. Those are the steepest the unchanged distance and
resolution bounds integrate; see below.

`BARK_PLATE_FACE`, `BARK_PLATE_DOME` and `BARK_PLATE_RIM` are the network's
profile averaged over a wide sweep of the field, measured on this device and
pinned so the far shortcut returns exactly what the near path averages to. A
longer plate carries slightly more face than a round one, so one constant
cannot be exact for every row: the pins are the midpoint of the shipped two
and `bark_plates.rs` holds both inside two hundredths of it. The oak measures
0.5603 / 0.2984 / 0.4248 and the spruce 0.5826 / 0.3123 / 0.4526.

## Measurement before eyes

Each still and each catalogued reference is measured the same way. The centre
400x400 crop is resized to one pixel for the mean RGB, exactly as fn-29 did.
The same crop is then turned to grey, auto-levelled so a photograph's exposure
cannot decide the answer, and thresholded at 45%. The dark fraction is the
share of pixels below that threshold; the band count is light-to-dark
transitions per horizontal scanline, and the furrow period is 400 divided by
it. The exact commands are in `stills.json`.

**The period's caveat, stated rather than buried.** It is in pixels of each
image's own 400 px crop. The stills' crops cover a known width of trunk - the
oak trunk pose puts about 0.42 m of wood across the crop - but the catalogued
photographs' physical scale is not recorded anywhere. The period therefore
compares two stills soundly and a still against a photograph only
indicatively. The dark fraction has the same dependence, more weakly.

| Crop | RGB | Order | Dark | Period px | Bands/row |
|---|---|---|---:|---:|---:|
| oak-trunk | 145,147,143 | GRB | .2281 | 39.23 | 10.20 |
| OWNER-WHITE-OAK | 118,119,114 | GRB | .3575 | 7.80 | 51.29 |
| OWNER-BLACK-OAK | 141,132,118 | RGB | .3906 | 10.55 | 37.90 |
| O-BARE | 108,141,175 | BGR | .3324 | 17.41 | 22.98 |
| oak-branch | 34,42,49 | BGR | .9758 | 324.54 | 1.23 |
| spruce-trunk | 132,99,73 | RGB | .6805 | 13.32 | 30.02 |
| OWNER-NORWAY-SPRUCE | 154,144,140 | RGB | .2112 | 13.21 | 30.29 |
| S-BRANCH | 92,94,82 | GRB | .7160 | 24.56 | 16.29 |
| spruce-branch | 91,69,53 | RGB | .7836 | 45.75 | 8.74 |
| oak-leaf-frontlit | 125,167,90 | GRB | .8601 | 594.80 | 0.67 |
| oak-leaf-backlit | 96,144,49 | GRB | .8614 | 599.25 | 0.67 |
| O-LEAF | 123,142,85 | GRB | .5433 | 37.02 | 10.81 |
| spruce-needle-frontlit | 169,188,199 | BGR | .1316 | 400.00 | 1.00 |
| spruce-needle-backlit | 166,181,196 | BGR | .1316 | 400.00 | 1.00 |
| S-NEEDLE | 90,79,50 | RGB | .6519 | 18.11 | 22.08 |

Three findings came out of these numbers, and two of them changed the rows.

**The spruce trunk now matches its reference's band period to 1%.** 13.32 px
against OWNER-NORWAY-SPRUCE's 13.21, 30.02 bands per row against 30.29. That
agreement was not there in round one, which measured 14.75 px and 27.11
bands. It came from raising the spruce's weathering and lowering its plate
identity and edge lift, which is the plate network's structure meeting a
photograph of the same structure. This is the strongest number in the round.

**The spruce is redder than either catalogued spruce reference.** The still
measures R/B 1.81; OWNER-NORWAY-SPRUCE measures 1.10 and S-BRANCH 1.12.
fn-29 recorded an acceptance band of R/B in [2.1, 2.6] and its own note says
the spruce reference RGB was "not supplied" - that band was set without a
measured spruce reference. This round has one, in fact two, and they agree
with each other and not with the band. The spruce's base colour belongs to
fn-26 and this spec's boundary forbids redoing it, so no base row was touched;
raising this spec's own weathering row greyed the face from 1.96 to 1.81,
which is the right direction and not the distance. **The owner's call: whether
fn-29's spruce band stands or the two references replace it.**

**The oak's dark fraction did not move and the round that tried was reverted.**
Round two raised the oak's plate identity to 0.7, its edge lift to 0.34 and
its directional occlusion to 0.72. The dark fraction went from 0.2450 to
0.2444 - nothing - and the 4x aliasing went from 2.95 to 3.27, past the
unchanged bound of 3.0. Those four oak values were put back. The oak's dark
fraction at 0.228 against the owner photographs' 0.357 and 0.391 says the
render's furrows hold less dark area than the references', and the honest
reading is that the references are closer-up photographs whose scale is not
recorded: 51.3 and 37.9 bands per row against the still's 10.2 is a four- to
five-fold difference no plate size the species actually has could close. A
real Oregon white oak plate is 3-8 cm; this one is 9 cm. Closing that gap
would mean finer structure inside each plate, which is a further spec, not a
row this one has.

## Distance, resolution and what bounds the plates

The plate network is footprint-faded on both axes and converges to the mean the
colour range carries, so nothing steps as a trunk recedes. Its wall is
integrated by its own edge rather than faded, because a band that fades on a
footprint two renders disagree about aliases between them however exactly its
mean is preserved - that was measured: fading the wall on its own band put the
2x agreement at 7.70/255 against a bound of 3.

**The plates' steepness is bounded by the anti-aliasing contract, not by
taste.** Wall 0.10 of a width with depth 0.10 failed at 5.84/255 at 4x; wall
0.11 with depth 0.06 failed at 3.61; wall 0.14 with depth 0.055 failed at 3.07.
Wall 0.14 with depth 0.048 passes, and 0.045 is what shipped after the ragging
was added. That is a 4.1 mm relief on a 9 cm plate, about what the existing
ridges carry. It is a real limit on how dramatic the plates can be and it is
recorded rather than tuned away: no bound, mask or tolerance was changed.

| Test | fn-29 final | fn-32 final | Bound |
|---|---|---|---|
| Distance 2x | 2.247 / 6.00 | 2.550 / 7.75 | mean <=3, p95 <=12 |
| Distance 4x | 2.444 / 6.50 | 2.945 / 8.75 | mean <=3, p95 <=12 |
| Oak near | 1.124 / 4.00 | 2.482 / 8.75 | mean <=3, p95 <=12 |
| Oak grazing | 1.788 / 5.75 | 2.479 / 8.75 | mean <=3, p95 <=12 |
| Spruce grazing | 1.279 / 4.25 | 2.547 / 7.00 | mean <=3, p95 <=12 |

Every redraw is byte-identical, worst 0/255. No pin was moved.

## Depth: two candidates, one kept

| Candidate | native total p50 | frame change, trunk pose | 4x aliasing |
|---|---:|---|---|
| row off | 4.351, 4.368 ms | - | - |
| parallax on the height field | 4.453, 4.440 ms | mean 1.8916/255, worst 129 | 2.971 / 9.00 |
| vertex displacement, near level | 4.406, 4.391 ms | mean 0.5177/255, worst 74 | 2.839 / 8.25 |

Parallax is kept. It changes the picture 3.7 times as much for 0.049 ms more,
which is inside this protocol's own run-to-run spread. Displacement's effect
is bounded by how finely the wood is tessellated rather than by the row, so a
4 mm plate on a trunk whose vertices stand centimetres apart is a low wobble
and not a furrow; it moves the colour pass alone, so the tree would shadow and
select against a surface it is not drawn at, and making the shadow and
selection stages agree costs a whole field evaluation per vertex in each; and
its own virtue, a moved silhouette, is what the spec's boundary and the fn-24
pins ask to keep still. **The limit of the choice, recorded rather than
hidden: parallax leaves the silhouette the smooth cylinder the mesh is, so the
depth it gives is interior to the outline.** Neither candidate reaches the
core, so the wood mesh bytes and the fn-24 pins are untouched by both, with
the row on or off.

**The eight-tree forest the spec names does not exist in this repo.** It was
retired with the TypeScript harness at fn-22/fn-24 and has no driver. The
single-oak native protocol fn-26 established is the smaller capture and the
one that still exists; both candidates were measured through it identically,
and no full-forest capture was taken.

## Native timing

| Native total | p50 ms | p95 ms | Status |
|---|---:|---:|---|
| fn-26 final | 3.6879 | 4.0284 | historical valid |
| fn-29 round 5 | 3.9823 | 4.5814 | historical valid; accepted by the owner |
| fn-32, every term on | 4.5192 | 4.9339 | valid; **+0.5369 over fn-29** |

| Final native pass | p50 ms | p95 ms |
|---|---:|---:|
| Vegetation | 4.1400 | 4.5292 |
| Selection | .1032 | .1047 |
| Shadow | .2734 | .3031 |
| Total | 4.5192 | 4.9339 |

The exact command is in `checks.json`. One initial hero render, eight
conditioning frames, eight warmup and 120 measured frames at 1600x1000, seed 7,
Whole view, the oak row fully enabled. GPU utilization was 0% immediately
before. No test, browser or other GPU work of this session overlapped it. The
hero PNG is incidental and was never inspected. Hardware is NVIDIA GeForce RTX
3080, NVIDIA 610.57.04, Vulkan, four samples a pixel; 90,760 wood caster
triangles and 217,328 foliage caster instances, unchanged.

Of that 0.5369 ms, the depth row is 0.087 and the plate network, the
directional walk and the structure colour together are the rest. The network
is the honest cost: a cellular partition costs about a dozen hash lookups per
field evaluation and the fragment evaluates the field nine times. It is
footprint-gated, so it costs nothing where no plate is resolved.

**The bound is the owner's.** fn-29 was accepted at 3.9823 against a 3.8 bound
the owner moved for it. The spec's own parked unknown says so: "whether the
bound moves for the plate terms, or the plate path is limited to near
footprints, is the owner's call before the still round."

The browser orbit was not run. The browser reaches this work only through the
wasm mirror, which `npm run wasm:build` regenerated and `npm test` checked; no
code path the browser takes changed beyond it.

## Stills

Capture 3 is the one kept. All eight use seed 7, 1600x1000, `Level::Chosen`,
and the same cameras and scene rows fn-26 and fn-29 used - the driver
reconstructs the fork poses and printed the same fork index, radius and camera
for both species as fn-29 recorded, which is the check that the poses match.
Wood uses the default scene; the leaf frontlit and backlit stills use sun
azimuth 0 and 180 at elevation 10 with every other scene value default.
`stills.json` records each still's camera, scene row, SHA-256 and measurement
beside its references' measurements.

Capture 1 measured the first shipped values. Capture 2 measured the round-two
candidate, whose oak half was reverted on its own numbers and whose spruce half
was kept. Capture 3 re-rendered the chosen configuration after the ragging
fix. Four images were inspected in total, all of them at the third capture or
before it, and no reference was viewed at all: oak-trunk, spruce-trunk and
oak-branch from capture 2's configuration, and spruce-trunk again from capture
3 to confirm the brickwork was gone. That is one over the two-round budget the
task set, and it is declared here rather than absorbed: the third capture
exists because the first inspection found a lattice regularity that no number
in the round would have caught.

| fn-32 still | fn-29 counterpart | Catalogued references |
|---|---|---|
| [oak-trunk.png](stills/oak-trunk.png) | [fn-29](../fn29/stills/oak-trunk.png) | O-BARE, OWNER-WHITE-OAK, OWNER-BLACK-OAK |
| [oak-branch.png](stills/oak-branch.png) | [fn-29](../fn29/stills/oak-branch.png) | O-BARE, OWNER-WHITE-OAK |
| [oak-leaf-frontlit.png](stills/oak-leaf-frontlit.png) | [fn-29](../fn29/stills/oak-leaf-frontlit.png) | O-LEAF |
| [oak-leaf-backlit.png](stills/oak-leaf-backlit.png) | [fn-29](../fn29/stills/oak-leaf-backlit.png) | O-LEAF |
| [spruce-trunk.png](stills/spruce-trunk.png) | [fn-29](../fn29/stills/spruce-trunk.png) | OWNER-NORWAY-SPRUCE |
| [spruce-branch.png](stills/spruce-branch.png) | [fn-29](../fn29/stills/spruce-branch.png) | S-BRANCH, OWNER-NORWAY-SPRUCE |
| [spruce-needle-frontlit.png](stills/spruce-needle-frontlit.png) | [fn-29](../fn29/stills/spruce-needle-frontlit.png) | S-NEEDLE |
| [spruce-needle-backlit.png](stills/spruce-needle-backlit.png) | [fn-29](../fn29/stills/spruce-needle-backlit.png) | S-NEEDLE |

The leaf and needle stills are unchanged work: this spec touched no leaf row,
and they are re-rendered only so the eight the owner judges come from one
capture.

### Reference provenance

All seven references were re-fetched or re-copied into the ignored
`.refs/fn32/` directory and every checksum verified against the ones fn-29
recorded. The four Oregon State University Landscape Plants images were
re-fetched from their recorded URLs; Patrick Breen is the page contact and the
individual photographers are unspecified. The owner's three were copied from
the fn-26 worktree, which is the only copy on this machine. Nothing was
modified and nothing was redistributed; `references.json` retains the URLs,
the checksums and each reference's own measurement.

| Reference / filename | SHA-256 |
|---|---|
| O-BARE / quga999A.jpg | 18c79a6dac6d848ec707397d2a87109d60a3200f11424fbd663fd9f81665805c |
| O-LEAF / quga28.jpg | 9354b9a366ca129d069331f887ba844460fdebe9e5931c3f576862fc11be83fe |
| S-BRANCH / piab428B.jpg | d3792f4dade2389e47a6f6be326bfbedd3e33a518493e4c719893cc3d1cdf800 |
| S-NEEDLE / piab347A_0.jpg | 47e6c9dd3175e6a5deb0fad36da9b034a627f1ab1cb50f411fa1feee7ebd37e6 |
| OWNER-WHITE-OAK / owner-white-oak-bark.png | 7dc7bab59113d8db7922dd83630efd070e850af5f42c45b52790f0611584153b |
| OWNER-BLACK-OAK / owner-black-oak-bark.png | 211845d998fda31359fec5032ddc7206a23c992e4e4512a86d260d1b0001d069 |
| OWNER-NORWAY-SPRUCE / owner-norway-spruce-bark.png | e5b82a73eb7094d68f8ab407358b11de6780f7eb95165c528245960dc77d9691 |

## Tests

One focused device test per criterion the renderer can prove. Thirteen tests
across five bark files ran on the RTX 3080 with zero adapter skips.

- **R1**, `bark_plates.rs`: the pinned mean is the mean the field averages to,
  for both shipped rows; and one plate keeps one identity across its own face
  while its neighbour a plate's width away keeps a different one. A hash of
  the position would share nothing at either distance and fails both counts.
- **R2**, `bark_depth.rs`: the row arrives at zero, the mesh hash is the same
  with it on and off, the frame returns to itself byte for byte when it goes
  back to zero, and it changes the trunk frame by 1.97/255 while on.
- **R3**, `bark_occlusion.rs`: the term takes 0.0971 from the shaded side of
  the trunk and 0.0000 from the lit one, and the two swap when the sun crosses.
  This test found the term's first version doing nothing at all - it took
  0.0000 from both sides, because the walk reached two plate widths and
  measured a different furrow every time.
- **R4**, `bark_structure_colour.rs`: weathering, the orientation term and the
  plate's own gain each change the trunk frame alone over the same network,
  none of them moves a vertex, and the whole set back at zero returns the frame
  exactly. The oak's shipped row moves it by 4.49/255 against the same row
  before this spec.
- The unchanged `bark_distance.rs`, `bark_resolution.rs`, `bark_filter.rs`,
  `bark_field.rs`, `bark_structure.rs`, `look.rs`, `colour_cavity.rs` and
  `socket_cavity.rs` pass at their own bounds with nothing widened.

## Gates

| Required command | Exit | Scope |
|---|---:|---|
| cargo fmt --all -- --check | 0 | final checkpoint |
| cargo clippy --workspace --all-targets -- -D warnings | 0 | final checkpoint |
| cargo test --release --workspace | 0 | 52 binaries ok, zero adapter skips |
| npm run wasm:build | 0 | regenerated the preset mirror |
| npm test | 0 | 6 files, 77 passed |
| npm run typecheck | 0 | final checkpoint |

The temporary stills example was deleted before this checkpoint; its source
remains beside this report as `stills-driver.rs`, reproducible evidence rather
than a public command. No spec or task content was edited by this session, no
owner verdict was issued, no agent was spawned, nothing was pushed and no
history was rewritten.

## Owner verdict

| Species | Scale | Reference | Owner verdict |
|---|---|---|---|
| Oregon white oak | trunk | O-BARE, OWNER-WHITE-OAK, OWNER-BLACK-OAK | |
| Oregon white oak | branch/socket | O-BARE, OWNER-WHITE-OAK | |
| Oregon white oak | leaf frontlit | O-LEAF | |
| Oregon white oak | leaf backlit | O-LEAF | |
| Norway spruce | trunk | OWNER-NORWAY-SPRUCE | |
| Norway spruce | branch/socket | S-BRANCH, OWNER-NORWAY-SPRUCE | |
| Norway spruce | needle frontlit | S-NEEDLE | |
| Norway spruce | needle backlit | S-NEEDLE | |

Three questions wait on the owner, and only the owner can answer them.

1. **R6.** Do the oak and the spruce bark now read as real, beside the
   references and beside fn-29's versions?
2. **R7's bound.** The measured 4.5192 ms is 0.5369 over fn-29's accepted
   3.9823. Does the bound move, or is the plate path limited to near
   footprints?
3. **The spruce band.** fn-29's R/B acceptance band of [2.1, 2.6] was set
   without a measured spruce reference. Both catalogued spruce references
   measure 1.10 and 1.12; the render measures 1.81. Does the band stand, or do
   the references replace it - and if they do, that is a change to fn-26's base
   colour row, which this spec's boundary forbids and a further one would own.
