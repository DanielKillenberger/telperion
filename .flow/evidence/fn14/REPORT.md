# FN14: bark and foliage appearance, as the machine drew it

The oak and the spruce lit, at the seed and the pose fn-24 drew them at, beside
the neutral room they used to be drawn in. What changed in this spec is the
renderer, not the trees: the same generator grows the same two specimens, and
they now stand under a sun with a shadow map, take their colour from a material
row on the family, vary leaf by leaf inside the ranges that row states, darken
towards the middle of the crown, and resolve out of a four-sample target through
a filmic tone map. The numbers below sit beside fn-24's for that reason, and one
of them is over its bound.

## Protocol

fn-22's rules, as fn-23 and fn-24 ran them, with two additions this spec makes.
Eight conditioning frames run untimed, eight timed warmup frames are thrown
away, and the percentiles are taken over the 120 measured frames that follow. A
session is judged before it is believed, on the vegetation samples, by fn-22's
verdicts: `valid`, `unavailable`, `disjoint`, `contended`. A record that is not
`valid` carries no percentile at all.

The additions: a **third** timestamp pair now goes around the depth pass the sun
writes its map in, so a record carries `shadow_p50_ms` and `shadow_p95_ms`
beside the vegetation and selection pairs, and `total_*` is the **three** added
frame by frame and then ranked. That total is the number R12 is judged on, and
it is not comparable term for term with fn-24's `total_*`, which added two. A
record also carries `multisample`, the samples a pixel the frame was actually
drawn at, because the renderer asks the adapter and falls back to one where
either the colour format or the depth format refuses four. Every record here
says 4.

Every session below came back `valid` and was run once. None was contended, so
none was repeated. The machine is the one fn-22, fn-23 and fn-24 measured on:
NVIDIA GeForce RTX 3080, driver NVIDIA 610.57.04, vulkan natively and WebGPU
through Chrome, on an idle desktop at 1600 by 1000 and native pixel ratio.

Two things about this run are departures a reader may want to reverse:

- The browser session wrote its records into this spec's evidence directory
  (`RENDER_EVIDENCE=.flow/evidence/fn14`) rather than rewriting fn-23's, which
  is where the rig writes by default. fn-23's four records are untouched by this
  run and still hold the numbers that spec was closed on.
- The spruce's native clock was not re-run, exactly as in fn-24. R12 names the
  oak. The spruce's browser numbers come free with the oak's because the browser
  suite measures both, and they are recorded here without a gate.

## The references

fn-9's manifest catalogues six photographs and states the command that retrieves
them. The two whole-tree images R11 judges against were fetched with that
command, unchanged, into the ignored `.refs/fn9/` directory. No pixel of either
is committed here, and neither is redistributed.

| ID | File | Source | SHA-256 | Bytes |
|---|---|---|---|---|
| O-WHOLE | `quga788B.jpg` | [Quercus garryana, OSU Landscape Plants](https://landscapeplants.oregonstate.edu/plants/quercus-garryana), image at `sites/plantid7/files/plantimage/quga788B.jpg` | `01bed8875ac98d92926404ad1dfd967a319509ad4f735731064ba2b1452407a4` | 429,804 |
| S-WHOLE | `piab977.jpg` | [Picea abies, OSU Landscape Plants](https://landscapeplants.oregonstate.edu/plants/picea-abies), image at `sites/plantid7/files/plantimage/piab977.jpg` | `49df5c91efafdf76d356a21a0adcac6c1d1d93cb6d86ca5cc889317cb5b8ec90` | 482,724 |

Both returned HTTP 200 and both decode: `quga788B.jpg` is a 1350 by 900 baseline
JPEG whose own EXIF description reads `Quercus garryana, Oregon White Oak, Garry
Oak, near Vet. Med., from a slide`, and `piab977.jpg` is a 1172 by 898 baseline
JPEG. Copyright is OSU's and reuse rights were not inferred from download
access, as fn-9 required. The four close-up references, O-BARE, O-LEAF, S-BRANCH
and S-NEEDLE, were not fetched: the close-up scales are fn-26's to judge.

## The stills

`./target/release/examples/headless --preset <id> --seed 7 --size 1600x1000
--out .flow/evidence/fn14/<id>-hero.png`, whole view, the renderer's own hero
pose, default scene row. The clay still adds `--view clay` and nothing else, so
the only difference between it and the lit oak is which room the tree stands in.

| Still | Preset | View | Wood triangles | Foliage instances |
|---|---|---|---:|---:|
| [oregon-white-oak-hero.png](oregon-white-oak-hero.png) | Oregon white oak | whole, lit | 8,255,000 | 869,310 |
| [norway-spruce-hero.png](norway-spruce-hero.png) | Norway spruce | whole, lit | 5,580,040 | 7,012,326 |
| [oregon-white-oak-clay.png](oregon-white-oak-clay.png) | Oregon white oak | clay | 8,255,000 | 869,310 |

The counts are the same trees fn-24 drew, down to the instance. Both lit stills
lay over fn-24's
[oak](../fn24/oregon-white-oak-hero.png) and [spruce](../fn24/norway-spruce-hero.png)
at the same preset, seed, view, pose, size and machine, so the pair is the light
and nothing else.

**What the pixels say about the clay view, as an aid and not as a verdict.** The
neutral room is the one place where this spec is supposed to have changed
nothing, so it is the one pair worth measuring. ffmpeg's own filters over
fn-24's oak hero still, which was drawn in that room before this spec existed,
and today's clay still:

| Pair | PSNR | SSIM | Mean absolute difference | Mean luminance |
|---|---:|---:|---:|---|
| fn-24 oak (clay, 1 sample) against fn-14 oak (clay, 4 samples) | 27.29 dB | 0.867 | 3.62 of 255 | 162.09 -> 162.54 |

Read that difference as the resolve and not as a leak. The oak's crown is
869,310 leaves of a few pixels each, so almost every pixel of the picture is a
silhouette, and four samples move every silhouette. The measurement that
separates the two is in the suite rather than here:
`tests/look.rs::the_clay_view_draws_the_still_the_room_always_drew` judges the
flat paint of the room apart from its edges, and over the 91.76% of the pinned
picture whose neighbourhood is uniform the mean channel error is 0.009. A
material, a sun or a tone map reaching the clay view would move that number, and
it has not moved. The whole frame's mean luminance moves by 0.45 of 255.

## The materials

What the look costs, pass by pass, on the oak at 1600 by 1000. The 1-sample
column is fn-14.4's run at commit `5fdbf0a`, one commit before multisampling,
which is the only run this adapter will give at one sample because the sample
count is device-driven and no switch forces it.

| Pass | fn-24, no sun | 1 sample, lit | 4 samples, lit | What the last column adds |
|---|---:|---:|---:|---|
| vegetation | 2.218 ms | 1.993 ms | **3.050 ms** | the resolve, +1.057 ms |
| selection | 0.087 ms | 0.087 ms | 0.087 ms | nothing |
| shadow | - | 1.799 ms | **1.813 ms** | nothing; the map stays single-sample |
| **total p50** | **2.304 ms** | **3.877 ms** | **4.949 ms** | |

The material row, the per-leaf offset, the crown-depth term, the sky triangle
and the tone map together cost about 0.04 ms: fn-14.3 measured 3.86 ms with the
sun and no look, and fn-14.4 measured 3.877 ms with both. Everything else in the
1.573 ms between fn-24 and the lit 1-sample frame is the sun's depth pass. The
resolve is the whole of the 4-sample column's rise, and all of it lands in the
vegetation pass, which is where the resolve attachment is.

**Samples.** 4 a pixel on colour and depth, on the headless target and on the
page, resolved into the single-sample target. The adapter is asked once, in
`Renderer::new`, whether the colour format and `Depth32Float` both take four,
and the renderer draws at one where either refuses. Every record in this report
says `"multisample": 4`, and the headless log line says `at 4 samples a pixel`.
The 1-sample path is what the renderer did before fn-14.5 and is pinned as a
rule by a unit test without a device, but it is not walked on this machine,
which offers four.

**Memory, nominal, as the buffers and textures are sized.** Driver padding is
not counted, and nothing here was read back off the device.

| What | Oak | Spruce |
|---|---:|---:|
| Wood coordinate buffer, 2 floats a vertex | 34.10 MB | 23.11 MB |
| The same tree's positions, normals and indices, for scale | 201.35 MB | 136.28 MB |
| Leaf element coordinate buffer, one element, uploaded once | under 4.2 kB | under 1.2 kB |
| Shadow map, 1,024 by 1,024, `Depth32Float`, single-sample | 4.19 MB | 4.19 MB |
| Colour and depth targets at 4 samples plus the resolve, 1600 by 1000 | 57.60 MB | 57.60 MB |
| The same targets at 1 sample | 12.80 MB | 12.80 MB |

So this spec adds 83.09 MB to the oak's frame at this size: 34.10 MB of surface
coordinates, 4.19 MB of shadow map and 44.80 MB of extra target. The coordinate
buffer is a sixth again on top of the wood's own arrays, and it is the price of
seam-free bark and leaf patterns in fn-26, which reads it. The shadow map is
fixed and does not scale with the frame; the targets scale with it, and a page
at a smaller canvas pays proportionally less.

## The oak on the clock

Both targets, beside fn-24's numbers, at the same pose and size. `total p50` is
the number R12 names. The fn-24 rows added two passes and the fn-14 rows add
three, so the columns are stated separately rather than differenced.

| Target | Session | Selection p50 | Vegetation p50 | Shadow p50 | Together p50 | Together p95 | Verdict |
|---|---|---:|---:|---:|---:|---:|---|
| native, fn-24 | plain | 0.087 ms | 2.218 ms | - | **2.304 ms** | 2.306 ms | valid |
| native, fn-14 | plain | 0.087 ms | 3.050 ms | 1.813 ms | **4.949 ms** | 5.412 ms | valid |
| native, fn-24 | orbit | 0.087 ms | 2.049 ms | - | 2.136 ms | 2.149 ms | valid |
| native, fn-14 | orbit | 0.087 ms | 3.081 ms | 1.813 ms | **4.981 ms** | 5.413 ms | valid |
| browser, fn-24 | plain | 0.092 ms | 2.016 ms | - | 2.108 ms | 2.309 ms | valid |
| browser, fn-14 | plain | 0.092 ms | 3.060 ms | 1.813 ms | 4.985 ms | 5.302 ms | valid |
| browser, fn-24 | orbit | 0.092 ms | 2.038 ms | - | 2.130 ms | 2.329 ms | valid |
| browser, fn-14 | orbit | 0.092 ms | 3.104 ms | 1.813 ms | 5.011 ms | 5.456 ms | valid |

The raw records are [native plain](oak-native-timing.json),
[native orbit](oak-native-orbit.json), [browser plain](oak-browser-timing.json)
and [browser orbit](oak-browser-orbit.json). The browser sessions ran on the
same crown as the native ones under the Chromium flags the records carry, and
neither was quantized: the percentiles are finer than Chrome's 100 microsecond
step.

**The native oak is over its budget by 1.15 ms.** R12 asks for 3.8 ms and this
run measured 4.9487 ms with a valid verdict. Per R12's own error clause that
stops the spec with the number rather than another attempt, and nothing was
tuned to chase it. The one paragraph the clause asks for is the next section.

## Why the number is 4.95 ms, and what the lever is

The frame is 3.050 ms of vegetation, 1.813 ms of shadow and 0.087 ms of
selection. The shadow pass is the half that does not pay its way. It draws the
same 8,255,000 wood triangles the vegetation pass draws, with no fragment stage
at all, and costs 1.81 ms where the vegetation pass costs 3.05 ms with shading,
a resolve and 869,310 instanced leaves on top. fn-14.3 measured the pass at
2.30 ms with a 2,048 texel map and 1.81 ms at 1,024, so halving the map bought
0.49 ms and halving it again cannot buy 1.3 ms: what is left is geometry
submission and not raster. The wood has no level ladder, only the crown does, so
the sun has no coarser caster to draw. That is the lever, and building one is
not in this spec.

The 1.5 ms the owner allowed for the sun was spent before the look was written.
Of the 2.645 ms between fn-24's 2.304 ms and this run's 4.949 ms, the shadow
pass is 1.813 ms, the resolve is 1.057 ms shared across four samples of every
pixel, the look itself is about 0.04 ms, and the vegetation pass is otherwise
0.225 ms cheaper than fn-24's. The owner's options are to accept 4.95 ms as the
price of a lit tree at four samples, to give the sun a coarse wood caster, to
drop to one sample and take 3.88 ms with the edges fn-24 had, or to move the
oak's foliage density, which is the row fn-24's own stop already pointed at.
This task takes none of them.

## The orbit, on the clock

`--orbit` turns the camera once about the vertical axis through the subject at
the hero pose's own elevation and distance. In the browser the wall clock is the
page's own frame-to-frame interval over ten seconds, which is the second half of
R12's question.

| Target | Species | Wall p50 | Wall p95 | Wall worst | Frames |
|---|---|---:|---:|---:|---:|
| native, fn-14 | Oregon white oak | 5.32 ms | 5.69 ms | 5.72 ms | 119 |
| browser, fn-24 | Oregon white oak | 10.00 ms | 10.10 ms | 10.20 ms | 999 |
| browser, fn-14 | Oregon white oak | **10.00 ms** | **10.10 ms** | **10.20 ms** | 999 |
| browser, fn-24 | Norway spruce | 10.10 ms | 30.10 ms | 30.10 ms | 612 |
| browser, fn-14 | Norway spruce | 30.00 ms | 30.20 ms | 30.40 ms | 349 |

**The oak's browser orbit holds R12's bounds exactly where fn-24 left them**:
999 frames in ten seconds on a 100 Hz display, a wall p95 of 10.10 ms against
the 16.7 ms tail and a worst frame of 10.20 ms against the 33 ms ceiling, valid.
The sun and the four samples cost the page 2.88 ms of GPU time a frame and cost
its frame rate nothing, because the loop is still waiting for the monitor. The
native row is the loop's own pace and not a frame rate claim: the session
resolves and maps a timestamp readback between measured frames.

The spruce is recorded and gated nowhere, and it went the other way: 349 frames
in ten seconds where fn-24 drew 612, a wall p50 of 30.00 ms and a GPU frame of
28.49 ms of which **13.51 ms is the shadow pass**. 7,012,326 needles drawn
twice, once for the sun and once for the camera, at four samples. Records:
[spruce browser plain](spruce-browser-timing.json) and
[spruce browser orbit](spruce-browser-orbit.json). The same coarse-caster lever
applies to it with far more to gain, and the aggregation spec the needles are
already waiting on applies to it too.

## What each level drew

Median instance count over the 120 measured frames of the native sessions,
coarsest level first. The last row is the bucket of leaves no level drew.

| Level | Tolerance | Instances, plain | Instances, orbit |
|---|---:|---:|---:|
| 0 (coarsest) | 0.010157 m | 861,024 | 855,086 |
| 1 | 0.005079 m | 8,286 | 14,087 |
| 2 | 0.002539 m | 0 | 0 |
| 3 | 0.000317 m | 0 | 0 |
| 4 | 0.000040 m | 0 | 0 |
| 5 (the whole element) | 0.000000 m | 0 | 0 |
| unseen | - | 0 | 0 |

Identical to fn-24's table, to the instance, in both sessions. The level ladder
did not notice the light, which is what an additive appearance change should
look like from here. The sun's own pass ignores the ladder by design and draws
every placement at level 0.

## Verdicts

**R1: awaiting the owner on the eye's half.** The oak and the spruce take bark
colour, roughness, leaf front and back colour, per-leaf hue and brightness
ranges and interior darkening from the material row on their own family, and
surface coordinates for fn-26 to draw relief and veins along. Translucency is
not here and is fn-26's by the spec's own boundary, so R1's translucency clause
is not claimed. Whether each still reads as its species under light is the
owner's, in the slots below.

**R2: partly met, and honest about the part that is not.** The trunk and crown
scales are exposed in the two lit hero stills under a stated scene row with a
stated sun. The leaf scale is not judged here: the close-up scales are fn-26's
by the spec's boundary, and the four close-up references were not fetched for
that reason. No seam or stretched coordinate can be judged before fn-26 draws
anything along the coordinates, since the bark and the blade are flat colours
under light in this spec. The unsupported-effect fallback R2 asks for exists and
is stated: a device that refuses four samples draws at one and the record says
so.

**R3: met.** The clay view is selectable from the page and the headless command,
draws no material, no sun and no tone map, and its still is
[oregon-white-oak-clay.png](oregon-white-oak-clay.png). The pixel table above
puts it against the room's own picture from fn-24, and the flat-paint half of
the `look.rs` pin holds at a mean channel error of 0.009. Material GPU and
memory costs are reported in the materials section: 1.813 ms of shadow, 1.057 ms
of resolve, 83.09 MB of new buffers and targets on the oak at 1600 by 1000. No
structural regression is concealed, because the clay view still draws the
structure with nothing over it.

**R11: awaiting the owner.** Both whole-tree references are on disk under
`.refs/fn9/` with their sources and checksums in the table above, and the two
lit hero stills are beside them. Two slots in the spec are the owner's, and the
spec closes only on accepting verdicts for both.

**R12: half met, and the spec stops on the other half.** The browser orbit
holds: 999 frames in ten seconds, a wall p95 of 10.10 ms against 16.7 ms and a
worst frame of 10.20 ms against 33 ms, valid, on the same page and machine as
fn-24's. The native total p50 does not: 4.9487 ms against a 3.8 ms bound, valid,
1.15 ms over, for the reason the section above gives.

## Owner verdict

Two slots, in `.flow/specs/fn-14-bark-and-foliage-appearance.md` under
`## Owner verdict`: the lit oak and the lit spruce, each beside its photograph.
Left empty here deliberately. R12's number is a separate stop that no verdict
clears.

## Follow-ups this run found and did not take

- **`src/browser/render.ts` does not type `shadow_p50_ms`, `shadow_p95_ms` or
  `multisample`.** The browser records in this directory carry all three,
  because they come from the Rust report, and the page's own type declares the
  older fields optionally and never mentions the new ones. Nothing breaks; a
  reader of that file is told less than the record says. It is outside this
  task's declared surface and was left alone.
- **The headless usage line still offers `--view whole|bare|leaf`.** `clay` is
  accepted and works, and `View::NAMES` has four entries, but
  `examples/headless/walk.rs` was not updated when the view was added, and
  `src/view.rs`'s own header still says "the same three the harness offers".
  Both are outside this task's declared surface and were left alone.
- **The sun has no coarse wood caster.** The lever named twice above, worth
  1.8 ms on the oak and 13.5 ms on the spruce.
