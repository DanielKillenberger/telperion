# FN23: fast hero, the oak inside the frame budget

GPU timing for the oak and the spruce after level selection, native and in the
browser, on the same machine fn-22 measured, with the stills the owner judges
the oak on and the sweep that holds every preset to the one path. The renderer now draws one
tree in three passes: the selection compute pass that decides which level each
leaf is drawn at, the room, then the vegetation. One timestamp pair goes around
the selection pass and one around the vegetation pass, so the two numbers below
are attributable separately and their sum is what the tree costs the GPU. The
room, the background, the ground disc and the scale figure are in neither.

## Protocol

Unchanged from fn-22, and a plain record's existing fields are byte-compatible
with fn-22's: same keys, same order, same four decimal places. Eight
conditioning frames run untimed, eight timed warmup frames are thrown away, and
the percentiles are taken over the 120 measured frames that follow. Each
measured frame resolves both its pairs and reads them back before the next
frame is drawn, so a sample is one frame's passes and not an average over a
queue. Only the base timestamp feature is used - pass-boundary writes on a
render pass and on a compute pass, never the native-only inside-encoder ones -
so the same session runs in a browser.

A session is judged before it is believed, on the vegetation samples, by the
rules fn-22 set: `valid`, `unavailable`, `disjoint`, `contended`. Everything
this spec adds follows the same rule and then one more. A record that is not
`valid` carries no percentile of any kind - not the vegetation pass, not the
selection pass, not the per-level counts, not the wall clock; the keys are
absent, not zero. And a selection pair that no pass wrote resolves to a pair of
zeroes rather than to a duration, so it is refused on its own account even when
the frame around it was measured cleanly. A timed frame therefore opens the
selection pass even in a view that selects nothing, so the pair is always
written and no readback waits on the device for a query that never happened.

Two things are new in the record:

- `selection_p50_ms` and `selection_p95_ms`, the compute pass on its own, and
  `total_p50_ms` and `total_p95_ms`, the two passes added **frame by frame** and
  then ranked. The total is a frame's own cost, not the sum of two separately
  ranked tails.
- `levels`, one entry per level of the crown's element, coarsest first, with
  the tolerance that level accepts in metres and the median instance count over
  the measured frames. The last entry is the bucket of leaves no level drew,
  and it has no tolerance of its own - it approximated nothing, it was not
  shown - so its `deviation_m` is `null`. The counters are read back once per
  measured frame inside a timing session and nowhere else; the plain frame path
  never stalls on them.

An orbit session adds `wall_p50_ms`, `wall_p95_ms` and `wall_max_ms`, and in
the browser `wall_frames` beside them, because how many frames a ten-second
window came to is the display's answer and not a constant.

## Measured

Rendered at 1600x1000, seed 7, whole view, hero pose, native pixel ratio, on an
idle desktop. Milliseconds are GPU time. Each session was run once; none came
back contended, so none was repeated.

| Species | Session | Selection p50 | Vegetation p50 | Together p50 | Together p95 | Verdict |
|---|---|---:|---:|---:|---:|---|
| Oregon white oak | plain | 0.057 ms | 1.693 ms | **1.750 ms** | 1.901 ms | valid |
| Oregon white oak | orbit | 0.058 ms | 1.854 ms | 1.912 ms | 1.942 ms | valid |
| Norway spruce | plain | 0.744 ms | 31.875 ms | 32.619 ms | 32.876 ms | valid |

NVIDIA GeForce RTX 3080, driver NVIDIA 610.57.04, vulkan backend. The raw
records are [oak plain](oak-native-timing.json), [oak orbit](oak-native-orbit.json)
and [spruce](spruce-native-timing.json).

Against fn-22 on the same machine and the same trees: the oak's vegetation pass
was 17.87 ms and is 1.69 ms, a factor of 10.6; the spruce's was 94.18 ms and is
31.88 ms, a factor of 3.0. Selection costs the oak 0.057 ms for 555 thousand
leaves and the spruce 0.744 ms for 7.9 million - fourteen times the leaves for
thirteen times the cost, which is the flat per-thread pass the design says it is
and not something that grows with the crown's shape.

The orbit is 0.16 ms slower at the median than the still pose. Different sides
of the crown put different amounts of leaf in front of different amounts of
leaf; the number moves with the view, not with the session.

## What each level drew

Median instance count over the 120 measured frames. The oak's element carries
six levels, the spruce's three; the triangle counts beside each tolerance are
the element's own, from the core's doubling ladder.

Oregon white oak, hero pose (plain session):

| Level | Tolerance | Triangles each | Instances |
|---|---:|---:|---:|
| 0 (coarsest) | 0.012694 m | 4 | 131,919 |
| 1 | 0.006347 m | 12 | 422,928 |
| 2 | 0.000793 m | 28 | 357 |
| 3 | 0.000397 m | 56 | 0 |
| 4 | 0.000099 m | 128 | 0 |
| 5 (the whole element) | 0.000000 m | 268 | 0 |
| unseen | - | - | 0 |

Norway spruce, hero pose (plain session):

| Level | Tolerance | Triangles each | Instances |
|---|---:|---:|---:|
| 0 (coarsest) | 0.000740 m | 14 | 7,895,664 |
| 1 | 0.000093 m | 30 | 0 |
| 2 (the whole element) | 0.000000 m | 56 | 0 |
| unseen | - | - | 0 |

The oak's medians sum to 555,204, which is the whole crown: at the hero pose the
tree is inside the frustum and nothing lands in the unseen bucket. So the crown
draws 5,612,808 foliage triangles where the finest level alone would draw
148,794,672 - a factor of 26.5 - and the whole frame comes to 10.9 million
triangles against fn-22's 154.1 million. The spruce's needles are all under the
coarsest tolerance, so every one is drawn at 14 triangles: 110.5 million against
442.2 million, a factor of 4. Its element has no level below a needle, which is
the aggregation spec this one deliberately does not do.

The orbit session's per-level medians are 122,201 / 432,668 / 4 and do **not**
sum to the crown. They are not meant to: each column is a median over frames
taken from different sides of the tree, and the split between the first two
levels moves as the camera comes closer to or further from the near face of the
crown. The still pose's medians sum because every frame in it counted the same
crown.

## In the browser

The same tree on the same GPU through Chrome 153, WebGPU backend, 1600x1000 at
native pixel ratio, still pose. The page runs the identical selection pass and
the identical indirect draws; nothing in the browser path knows a level from a
species. Task 5 recorded these; they are repeated here so both targets read from
one page.

| Species | Selection p50 | Vegetation p50 | Together p50 | Together p95 | Verdict |
|---|---:|---:|---:|---:|---|
| Oregon white oak | 0.063 ms | 2.212 ms | **2.275 ms** | 2.509 ms | valid |
| Norway spruce | 0.752 ms | 29.434 ms | 30.186 ms | 30.341 ms | valid |

The raw records are [oak](oak-browser-timing.json) and
[spruce](spruce-browser-timing.json), with the Chromium flags and the
quantization note each session ran under. Neither session was quantized: the
percentiles are finer than Chrome's 100 microsecond step.

Against fn-22 in the same browser: the oak's vegetation pass was 17.86 ms and is
2.21 ms, a factor of 8.1; the spruce's was 62.91 ms and is 29.43 ms, a factor of
2.1. Selection costs the browser 0.063 ms on the oak against the native target's
0.057 ms, so the compute pass carries across at the same order.

The browser draws the oak's vegetation in 2.21 ms where the native target takes
1.69 ms, half a millisecond more on the same tree and the same GPU. Both are
inside the budget and neither R1 nor R2 turns on the difference, so it was
recorded and not chased. fn-22's much larger spruce gap ran the other way, the
browser faster than native; on these frames the browser is the slower of the two
for both species, which is a cleaner pair of numbers for the needle spec to ask
its question about.

## The orbit, on the clock

`--orbit` runs the same protocol while the camera makes one full revolution
about the vertical axis through the subject, at the hero pose's own elevation
and distance; conditioning and warmup run at the start of the turn and the 120
measured frames divide it evenly. The still beside a session is always the hero
pose - the orbit is what is measured, not what is judged.

| Target | Species | Wall p50 | Wall p95 | Wall worst | Frames |
|---|---|---:|---:|---:|---:|
| native headless | Oregon white oak | 2.17 ms | 2.24 ms | 2.25 ms | 120 |
| browser | Oregon white oak | **10.00 ms** | **10.10 ms** | **10.10 ms** | 999 |
| browser | Norway spruce | 30.00 ms | 44.50 ms | 60.10 ms | 330 |

Read this as the native loop's own pace and not as a frame rate claim. The
native session resolves and maps a timestamp readback between every pair of
measured frames, so the wall time is draw plus readback plus the host's turn,
which is why it sits 0.31 ms above the GPU number rather than at it. The 60 fps
question is R2's, and R2 is judged in the browser on the page's own loop, in
task 5. What this row does establish is that nothing in the native orbit stalls:
the worst frame of a full turn is 0.08 ms above the median.

The browser rows are the R2 evidence, from [oak](oak-browser-orbit.json) and
[spruce](spruce-browser-orbit.json). Ten seconds of orbit on this machine's
100 Hz display gave the oak 999 frames at a p50 of 10.00 ms, a p95 of 10.10 ms
and a worst frame of 10.10 ms, against R2's 16.7 ms tail and its 33 ms ceiling.
The tail sits on the display's own interval, so the loop is waiting for the
monitor rather than for the tree. Both browser clocks are coarsened to Chrome's
100 microsecond step, which is why the numbers land on a 0.1 ms grid. The GPU
numbers beside them come from the measured frames of the same session, which
carry a timestamp readback each and are therefore not the frames the wall clock
was read from; the split is deliberate and task 5 defends it.

The spruce is recorded and gated nowhere. Its 30.00 ms median is its 29.4 ms
vegetation pass and almost nothing else, so it renders at about 33 frames per
second and misses the ceiling once in a turn at 60.10 ms. Needles have no level
below a needle, and the aggregation that would give them one is a later spec.

## Every preset through the one path

`cargo test --release -p telperion-render` runs the sweep task 3 wrote, on the
final path with selection and the indirect draws in place. It rendered all five
shipped families through one submit and one frame, then 20 parameter sets
nobody wrote by hand, drawn from a pinned generator seed by scaling every number
in a shipped family by a factor between 0.8 and 1.25: 20 rendered, 5 refused by
the generator, 25 tried. Every set that rendered put more triangles on the
screen than its own wood and drew exactly the crown it submitted; a still that
came out as one flat colour would have failed. A refusal carries the generator's
own words out through the renderer's error, asserted separately on a negative
envelope height and a negative leaf length, each of which has to name the
parameter it refused.

Two further tests hold the no-branch rule structurally rather than by
inspection: no file in the renderer's source may contain the family table, and
`select.rs`, `foliage.rs`, `select.wgsl` and `foliage.wgsl` may not name a
shipped family or any of six anatomy words. Selection reads sections,
deviations and a matrix, and nothing else.

`npm run species:qa` ran both profiles over the protocol's 24 seeds each: 48
numeric cases, all pass, and 48 stills across the whole, bare and single-leaf
views of every required case plus the three supplementary presets, all captured.
The runner exits 1 by design while the visual inspection is unassessed, which is
its standing contract and not a failure of this run; no image was missing and
none failed.

## The stills

[oak-hero.png](oak-hero.png) and [spruce-hero.png](spruce-hero.png) are seed 7,
whole view, hero pose, 1600x1000, written by the native headless target on the
RTX 3080, the same command, pose and size as fn-22's stills so the two can be
laid over each other.

The per-frame statistics line the headless target prints still reports the
crown as if every leaf were drawn at the finest level, because no count comes
back from the device outside a timing session. The 5.6 million foliage triangles
above are the measured truth; the 154 million on that line are the upper bound
the CPU can name without stalling.

## Verdicts

**R1: met.** The Oregon white oak at seed 7, whole view, hero pose, 1600 by 1000
at native pixel ratio, renders its vegetation pass including selection at a p50
of **1.750 ms** on the RTX 3080 through the native headless target, with a valid
verdict, every level and the unseen bucket in place. The budget is 2 ms. The
stop rule was not reached and no levers were added.

**R2: met.** The same oak in the browser page on the same machine held a
frame-to-frame wall p50 of **10.00 ms**, a p95 of **10.10 ms** and a worst frame
of **10.10 ms** over 999 frames of a ten-second orbit at 1600 by 1000, native
pixel ratio, under hardware WebGPU. R2 asks for a p95 under 16.7 ms and no frame
above 33 ms. The flags Chromium ran under and the 100 microsecond quantization
are in the record.

**R5: met.** All five shipped families and 20 pinned random parameter sets
render through the identical submit and frame, with no species or template
branch anywhere in renderer code, held by two tests that read the source for a
family name or an anatomy word. Generator refusals carry the generator's own
message. The Norway spruce runs that same path and reports 32.619 ms native and
30.186 ms in the browser, both valid, with a browser orbit wall p50 of 30.00 ms
and one frame of the turn at 60.10 ms. Every spruce number here is recorded and
gated nowhere.

**R4: accepted by the owner (2026-09-09).** [oak-hero.png](oak-hero.png) is committed beside
[fn-22's](../fn22/oak-hero.png) for the comparison. The slot below is the
owner's, and the spec closes only on an accepting verdict.

**Parked question, partly answered.** fn-22 left the spruce's browser-native gap
open (62.9 ms browser against 94.2 ms native) and asked whether task 4's
selection timing would explain it. It does not: selection is 0.744 ms of a 32.6
ms spruce frame, two per cent, so nothing about the compute pass accounts for a
thirty-millisecond gap. Task 3 observed the browser spruce at 29.4 ms against
this run's 31.9 ms native, which is a far smaller gap than fn-22's on a much
faster frame; whether the original difference survives at all is now a question
for the needle spec, with a new pair of numbers to ask it about.

## Owner verdict

One slot, the owner's, against the fn-22 oak still. Left empty deliberately.
Compare [.flow/evidence/fn23/oak-hero.png](oak-hero.png) with
[.flow/evidence/fn22/oak-hero.png](../fn22/oak-hero.png): same preset, same seed,
same pose, same size, same machine, one rendered at the finest level throughout
and one at whatever level each leaf's projected error allowed.

What the pixels say, as an aid and not as the verdict: 4.6 per cent of the frame
differs at all between the two, 1.0 per cent by more than 32 of 255 in some
channel, the mean absolute difference over the whole frame is 0.83 of 255, and
the mean luminance moves from 170.83 to 170.88. The differences are inside the
crown, where a leaf a few pixels tall is now drawn from 4 or 12 triangles
instead of 268. Whether that is a change worth having is the question the slot
below answers.

### Oregon white oak, hero pose

> _verdict (owner, 2026-09-09):_ side by side they look exactly the same so we achieved the goal
