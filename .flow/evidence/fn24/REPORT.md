# FN24: continuous tree space, as the machine drew it

One still per shipped preset at the hero pose, the oak-to-spruce transition the
continuous space was asked to prove, and fn-23's oak clock re-run on the tree
the one builder now grows. Nothing in the renderer changed in this spec: the
same selection pass, the same room and the same vegetation pass draw these
frames as drew fn-23's. What changed is the trees themselves. Every preset is
now a row of continuous traits grown by one builder, so the oak and the spruce
in these stills are not the oak and the spruce fn-23 measured, and the numbers
below are not expected to repeat fn-23's - they are expected to be judged
against them.

## Protocol

Unchanged from fn-22 and fn-23, and the records are byte-compatible with
theirs: same keys, same order, same four decimal places. Eight conditioning
frames run untimed, eight timed warmup frames are thrown away, and the
percentiles are taken over the 120 measured frames that follow. One timestamp
pair goes around the selection compute pass and one around the vegetation pass;
`total_*` is the two added frame by frame and then ranked, so it is a frame's
own cost and not the sum of two separately ranked tails. A session is judged
before it is believed, on the vegetation samples, by fn-22's rules: `valid`,
`unavailable`, `disjoint`, `contended`. A record that is not `valid` carries no
percentile at all. An orbit session adds `wall_p50_ms`, `wall_p95_ms` and
`wall_max_ms`, and in the browser `wall_frames` beside them.

Every session below came back `valid` and was run once. None was contended, so
none was repeated. The machine is the one fn-22 and fn-23 measured on: NVIDIA
GeForce RTX 3080, driver NVIDIA 610.57.04, vulkan natively and WebGPU through
Chrome 153.0.8010.12, on an idle desktop at 1600 by 1000 and native pixel
ratio.

Three things about this run are worth stating before the numbers, because they
are departures from the task's written approach and each one is a choice
somebody may want to reverse:

- The five stills were rendered straight from the headless target at 1600 by
  1000 rather than through `npm run species:qa`, whose capture half renders at
  960 by 720. The acceptance asks for fn-23's and fn-22's framing, and 960 by
  720 is not it. The numeric half of the species protocol was run in full and
  is reported below; its capture half, 57 stills at the QA rig's own size, was
  not run - the same binary drew the five hero stills, and nothing in this task
  judges the QA rig's images.
- The spruce's native clock was not re-run. The spec's boundaries say so in as
  many words: R6 is the oak, as fn-23 was, and there is no spruce timing re-run
  in this spec. The spruce's browser numbers came free with the oak's, because
  the browser suite measures both, and they are recorded here.
- The 240 walked frames are ignored by git through
  `.flow/evidence/fn24/transition/.gitignore`, so the sequence stays on disk
  and only `transition.json` and `transition.mp4` are committed. The ignore
  file sits inside the evidence directory rather than in the repository root
  because this task's declared surface is `.flow/evidence/fn24/**`.

## The stills

`./target/release/examples/headless --preset <id> --seed 7 --size 1600x1000
--out .flow/evidence/fn24/<id>-hero.png`, once per preset, whole view, the
renderer's own hero pose. The oak and the spruce are therefore the same preset,
seed, view, pose, size and machine as
[fn-23's oak](../fn23/oak-hero.png) and [fn-22's spruce](../fn22/spruce-hero.png),
which are also 1600 by 1000, so the pairs lay over each other.

| Preset | Still | Wood triangles | Retained leaves | Element at the finest level |
|---|---|---:|---:|---:|
| Oregon white oak | [oregon-white-oak-hero.png](oregon-white-oak-hero.png) | 8,255,000 | 869,310 | 172 triangles |
| Norway spruce | [norway-spruce-hero.png](norway-spruce-hero.png) | 5,580,040 | 7,012,326 | 48 triangles |
| Ordinary | [ordinary-hero.png](ordinary-hero.png) | 622,880 | 41,827 | - |
| Telperion | [telperion-hero.png](telperion-hero.png) | 1,224,272 | 118,329 | - |
| Laurelin | [laurelin-hero.png](laurelin-hero.png) | 3,115,712 | 542,357 | - |

The oak's element triangle count is corroborated twice: the still's own
statistics line minus its wood comes to 172 triangles per leaf, and the species
protocol's `prototype_triangles` for the oak is 172. The spruce's needle is 48
by the same two readings. Both elements are smaller than the ones fn-23
measured (268 and 56): they come out of one outline routine now rather than two
hand-written ones.

The three colonizing presets are recorded, not gated. The spec says their
scaffolds change with everything else, and fn-10 owns any later verdict on the
Two Trees.

**What the pixels say, as an aid and not as a verdict.** Computed with ffmpeg's
own filters over each pair, so both sides of a comparison use one convention:

| Pair | PSNR | SSIM | Mean absolute difference | Mean luminance |
|---|---:|---:|---:|---|
| fn-23 oak against fn-24 oak | 20.29 dB | 0.680 | 10.53 of 255 | 162.50 -> 162.09 |
| fn-22 spruce against fn-24 spruce | 25.73 dB | 0.884 | 3.09 of 255 | 168.59 -> 169.14 |

These are large differences by fn-23's standard, where the same comparison
moved 0.83 of 255 on the mean. They are supposed to be: fn-23 changed which
level each leaf was drawn at and nothing about the tree, and this spec rebuilt
the tree. The frame's overall brightness barely moves in either pair, so
neither crown lost or gained mass on the scale of the whole picture; the
difference is in where the wood and the foliage went. Whether each still still
reads as its species is R3's question and the owner's alone.

## The oak on the clock

Both targets, beside fn-23's numbers, at the same pose and size. `total p50` is
the number R6 names: vegetation plus selection, ranked frame by frame.

| Target | Session | Selection p50 | Vegetation p50 | Together p50 | Together p95 | Verdict |
|---|---|---:|---:|---:|---:|---|
| native, fn-23 | plain | 0.057 ms | 1.693 ms | **1.750 ms** | 1.901 ms | valid |
| native, fn-24 | plain | 0.087 ms | 2.218 ms | **2.304 ms** | 2.306 ms | valid |
| native, fn-23 | orbit | 0.058 ms | 1.854 ms | 1.912 ms | 1.942 ms | valid |
| native, fn-24 | orbit | 0.087 ms | 2.049 ms | 2.136 ms | 2.149 ms | valid |
| browser, fn-23 | plain | 0.063 ms | 2.212 ms | 2.275 ms | 2.509 ms | valid |
| browser, fn-24 | plain | 0.092 ms | 2.016 ms | 2.108 ms | 2.309 ms | valid |
| browser, fn-24 | orbit | 0.092 ms | 2.038 ms | 2.130 ms | 2.329 ms | valid |

The raw records are [native plain](oak-native-timing.json),
[native orbit](oak-native-orbit.json), [browser plain](oak-browser-timing.json)
and [browser orbit](oak-browser-orbit.json). The browser sessions ran on the
same crown as the native ones - 869,310 foliage instances, 8,255,000 wood
triangles in both - under the Chromium flags the record carries, and neither
browser session was quantized: the percentiles are finer than Chrome's 100
microsecond step.

**The native oak is over its budget by 0.30 ms.** R6 asks for a 2 ms native
p50 and this run measured 2.3043 ms with a valid verdict. The browser is
*faster* than it was in fn-23 on the same measurement, so this is not a
renderer regression and not a browser one. It is the crown: fn-23's oak carried
555,204 retained leaves and this one carries 869,310, fifty-seven per cent
more. Per leaf the frame got cheaper - 1.693 ms over 555,204 leaves is 3.05
nanoseconds each, 2.218 ms over 869,310 is 2.55 - so a leaf costs sixteen per
cent less than it did and there are simply more of them. The oak's leaf count
is a preset row, and moving it is the owner's decision, not this task's; see
the stop note under the verdicts.

## What each level drew

Median instance count over the 120 measured frames of the native sessions,
coarsest level first. The last row is the bucket of leaves no level drew; it
approximated nothing, so it has no tolerance of its own.

| Level | Tolerance | Instances, plain | Instances, orbit |
|---|---:|---:|---:|
| 0 (coarsest) | 0.010157 m | 861,024 | 855,086 |
| 1 | 0.005079 m | 8,286 | 14,087 |
| 2 | 0.002539 m | 0 | 0 |
| 3 | 0.000317 m | 0 | 0 |
| 4 | 0.000040 m | 0 | 0 |
| 5 (the whole element) | 0.000000 m | 0 | 0 |
| unseen | - | 0 | 0 |

The plain session's medians sum to 869,310, which is the whole crown: at the
hero pose the tree is inside the frustum and nothing lands in the unseen
bucket. The ladder itself moved with the element - fn-23's coarsest tolerance
was 0.012694 m and this one is 0.010157 m - and the split between the first two
levels moved with it: fn-23 drew 131,919 leaves at level 0 and 422,928 at level
1, and this crown draws almost everything at level 0. The per-level triangle
counts of the new ladder were not extracted, so no measured triangle total is
claimed here; the still's own statistics line reports the crown as if every
leaf were drawn at the finest level, which is the upper bound the CPU can name
without stalling and not what the GPU drew.

## The orbit, on the clock

`--orbit` turns the camera once about the vertical axis through the subject at
the hero pose's own elevation and distance. In the browser the wall clock is
the page's own frame-to-frame interval over ten seconds, which is R2's
evidence in fn-23 and the second half of R6's question here.

| Target | Species | Wall p50 | Wall p95 | Wall worst | Frames |
|---|---|---:|---:|---:|---:|
| native, fn-23 | Oregon white oak | 2.17 ms | 2.24 ms | 2.25 ms | 120 |
| native, fn-24 | Oregon white oak | 2.37 ms | 2.44 ms | 2.45 ms | 119 |
| browser, fn-23 | Oregon white oak | 10.00 ms | 10.10 ms | 10.10 ms | 999 |
| browser, fn-24 | Oregon white oak | **10.00 ms** | **10.10 ms** | **10.20 ms** | 999 |
| browser, fn-23 | Norway spruce | 30.00 ms | 44.50 ms | 60.10 ms | 330 |
| browser, fn-24 | Norway spruce | 10.10 ms | 30.10 ms | 30.10 ms | 612 |

The oak's browser orbit is where it was: 999 frames in ten seconds on a 100 Hz
display, a p95 of 10.10 ms against the 16.7 ms tail and a worst frame of 10.20
ms against the 33 ms ceiling. The loop is waiting for the monitor, not for the
tree. The native rows are the loop's own pace, not a frame rate claim: the
session resolves and maps a timestamp readback between measured frames, which
is why the native wall sits about 0.23 ms above its GPU number.

The spruce is recorded and gated nowhere, and it moved a long way. Its browser
vegetation pass was 29.43 ms in fn-23 and is 13.39 ms here, and its orbit went
from 330 frames in ten seconds with a 60.10 ms worst frame to 612 frames with a
30.10 ms worst. It carries 7,012,326 needles where fn-23's carried 7,895,664,
and each needle is 48 triangles where fn-23's was 56 - about a third less
foliage geometry, for rather more than a third of the time. Its native clock
was not re-run, so there is no fn-24 native spruce number to put beside
fn-23's 32.619 ms.

## The transition

One run, one seed, no second attempt:

```
./target/release/examples/headless --preset oregon-white-oak --to norway-spruce \
  --seed 7 --frames 240 --size 1600x1000 --out .flow/evidence/fn24/transition/frame.png
```

It walked the oak row to the spruce row in 240 linear steps, built and rendered
a whole tree per frame, and wrote `frame-0001.png` through `frame-0240.png`
plus the record and the video. 353 seconds on the RTX 3080, about 1.5 seconds a
frame, of which the mesh build is nearly all. The record it left:

| Field | Value |
|---|---|
| from | `oregon-white-oak` |
| to | `norway-spruce` |
| seed | 7 |
| size | 1600 by 1000 |
| frames | 240 |
| fps | 24 |
| encoder | `.flow/evidence/fn24/transition/transition.mp4` |

The system encoder was present, so the video is beside the record at
[transition.mp4](transition/transition.mp4), 13.3 MB, ten seconds at 24 frames
per second. Had ffmpeg been absent the record would carry the notice `no ffmpeg
on the path; the frames stand` in that same `encoder` field, the frames would
stay, and the run would still exit 0; task 5 proved that branch separately.
Nobody has opened the video or the frames, here or in task 5 - the project's
evidence rules forbid it, and R4's verdict on the passage is the owner's eye,
not an agent's.

What holds underneath the video is a test, not a viewing:
`every_pair_of_presets_grows_a_tree_at_every_step` and
`the_oak_to_spruce_walk_has_no_switch_frame` in
`crates/telperion-core/tests/sweep.rs` walk every pair of shipped presets in ten
steps and assert every step validates, generates and moves continuously. They
are green in the run recorded below.

## The fidelity band

Retained leaves at seed 7, whole tree, as the stills drew them, against the
band `crates/telperion-core/tests/sweep.rs` holds each preset to:

| Preset | Retained leaves | Band | Inside |
|---|---:|---|---|
| Ordinary | 41,827 | 10^4 to 10^6 | yes |
| Oregon white oak | 869,310 | 10^5 to 10^7 | yes |
| Norway spruce | 7,012,326 | 10^5 to 10^7 | yes |
| Telperion | 118,329 | 10^5 to 10^7 | yes |
| Laurelin | 542,357 | 10^5 to 10^7 | yes |

One number in that table is not where the strategy's fidelity track would put
it. **Ordinary carries 41,827 leaves on a twenty-four metre envelope**, a decade
below the 10^5 to 10^7 the track names, on the same envelope the oak fills with
869,310. The band in the test records where the count stands, not where the
owner has said it should stand, and the comment in the test says so. This is a
property of the colonizing rows after the one builder - the same movement task
1 recorded when Telperion's retained rail fell from a million to four hundred
thousand - and no preset was touched to produce it. Telperion's 118,329 sits
just inside the band for the same reason. Whether the colonizing rows want more
foliage is the owner's call; nothing in this spec proposes it.

## The species protocol, numerically

`npm run species:measure` ran the frozen fn19 protocol over 24 seeds for each
of the two species - twelve fixed, twelve fresh - and gated every case against
its frozen profile: **48 of 48 pass**, in 360 seconds, with no GPU in the path.
The per-case record is [species-numeric.json](species-numeric.json); the full
84 MB output stayed on disk. Across the 24 oaks the retained leaf count runs
674,193 to 962,559 with a median of 871,598, and across the 24 spruces
6,503,412 to 7,481,473 with a median of 7,158,192. So the seed-7 trees in the
stills are ordinary members of their own distributions, not outliers the
framing flattered.

## Verdicts

**R3: awaiting the owner.** [oregon-white-oak-hero.png](oregon-white-oak-hero.png)
and [norway-spruce-hero.png](norway-spruce-hero.png) are rendered at fn-23's and
fn-22's framing beside [fn-23's oak](../fn23/oak-hero.png) and
[fn-22's spruce](../fn22/spruce-hero.png). The question is whether each still
reads as its species at least as well as before, never whether it is identical.
Two slots in the spec are the owner's.

**R4: met on the machine's half, awaiting the owner on the eye's half.** Ten
linear steps between every pair of shipped presets validate and generate, and
the oak-to-spruce walk carries no switch frame, both asserted in
`crates/telperion-core/tests/sweep.rs` and green in this run. Every shipped
preset's leaf count is inside its band, with the Ordinary caveat above. The
transition rendered once at seed 7 into 240 frames and a video. Whether that
video reads as a smooth passage through one tree space is the third slot in the
spec.

**R6: half met, and the spec stops on the other half.** The browser orbit holds:
999 frames in ten seconds, a wall p95 of 10.10 ms against 16.7 ms and a worst
frame of 10.20 ms against 33 ms, valid, on the same page and the same machine
as fn-23's. The native p50 does not: 2.3043 ms against a 2 ms budget, valid,
0.30 ms over. Per R6's own error clause this stops the spec with the number
rather than another attempt, and nothing was tuned to chase it. The one
paragraph the clause asks for is this: the oak's vegetation pass costs 2.218 ms
where fn-23's cost 1.693 ms because the one builder grows this oak 869,310
retained leaves where the three builders grew 555,204, and a leaf is if
anything cheaper than it was - 2.55 nanoseconds against 3.05 - so no renderer
change and no level-ladder change recovers the budget; only the crown's own
foliage count would, and that is a preset row. The owner's options are to
accept 2.3 ms as the price of the tree these stills show, to move the oak row's
foliage density and re-measure, or to open the aggregation spec the spruce's
needles are already waiting on. This task takes none of them.

## Owner verdict

Three slots, in `.flow/specs/fn-24-continuous-tree-space-habit-and-leaf-as.md`
under `## Owner verdict`: the oak still, the spruce still, and the transition
video. Left empty here deliberately; the spec closes only on three accepting
verdicts, and R6's number above is a separate stop that no verdict clears.
