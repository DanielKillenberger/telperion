# FN22: hero tree through a Rust wgpu renderer

Native GPU timing and the hero stills the owner judges, from the headless target
on this machine. The renderer draws one tree in two passes: the room first, then
the vegetation in a pass of its own. The timestamp pair goes around the second
pass alone, so the numbers below are the wood and the foliage, not the ground
disc, the background or the scale figure standing beside the trunk.

## Protocol

Eight conditioning frames run untimed, eight timed warmup frames are thrown away,
and the percentiles are taken over the 120 measured frames that follow; nothing
outside those 120 is counted. Each measured frame resolves its own timestamp pair
and reads it back before the next frame is drawn, so a sample is one frame's
vegetation pass and not an average over a queue. Only the base timestamp feature
is used - never the native-only inside-pass ones - and the readback is driven by
the mapping callback, so the same session runs in a browser.

A session is judged before it is believed. `valid` means every sample was a
duration and the p95 sat inside twice the median. `unavailable` means there was
nothing to measure with: no timestamp feature, or a software adapter. `disjoint`
means a sample was not a duration - non-finite, or a clock that ran backwards.
`contended` means the tail ran away from the median and something else had the
GPU. A record that is not `valid` carries no percentile at all: the fields are
absent from the JSON, not zero, so an invalid session has no number in it that
can be read as a pass.

## Measured

Rendered at 1600x1000, seed 7, whole view, at the hero pose. Milliseconds are the
vegetation pass on the GPU.

| Species | Adapter | Backend | p50 | p95 | Verdict |
|---|---|---|---:|---:|---|
| Oregon white oak | NVIDIA GeForce RTX 3080 | vulkan | 17.87 ms | 18.08 ms | valid |
| Norway spruce | NVIDIA GeForce RTX 3080 | vulkan | 94.18 ms | 94.19 ms | valid |

Driver NVIDIA 610.57.04. The raw records are [oak](oak-native-timing.json) and
[spruce](spruce-native-timing.json); the stills beside them are
[oak-hero.png](oak-hero.png) and [spruce-hero.png](spruce-hero.png).

What each frame draws, in three draw calls either way:

| Species | Wood vertices | Wood triangles | Foliage instances | Triangles drawn |
|---|---:|---:|---:|---:|
| Oregon white oak | 2,725,520 | 5,280,080 | 555,204 | 154,075,392 |
| Norway spruce | 3,575,758 | 6,911,320 | 7,895,664 | 449,069,144 |

Both sessions were clean: the p95 sits within a quarter of a percent of the
median on either species, which is what an idle desktop GPU with nothing else on
it looks like. The spread the contention verdict exists to catch is two orders of
magnitude wider than anything observed here. Performance is reported and not
gated in this spec; the 2 ms hero target belongs to the fast-hero spec, and
spruce at 94 ms is the distance to it, measured rather than estimated.

## The same trees in the browser

The same session protocol, the same seed and the same canvas size, run on the
page's own module through Chrome's WebGPU. Both trees are the ones the native
rows measured: seed 7 through the panel's own dial composition, and the counts
below match the native table exactly, so these are two measurements of one tree
and not two trees.

| Species | Adapter | Backend | p50 | p95 | Verdict |
|---|---|---|---:|---:|---|
| Oregon white oak | nvidia ampere, as WebGPU reports it | webgpu | 17.86 ms | 17.88 ms | valid |
| Norway spruce | nvidia ampere, as WebGPU reports it | webgpu | 62.91 ms | 62.93 ms | valid |

Chromium 153.0.8010.12, launched with `--no-sandbox --enable-unsafe-webgpu
--enable-features=Vulkan --use-angle=vulkan --disable-vulkan-surface
--ignore-gpu-blocklist`. The raw records are
[oak](oak-browser-timing.json) and [spruce](spruce-browser-timing.json).

Two things about a browser number. Chrome quantizes WebGPU timestamps to 100
microseconds unless developer features or the unsafe-WebGPU flag are on; these
percentiles are finer than that step, so this session was not quantized, and each
record says so from its own numbers rather than from an assumption. And the
module cannot name the hardware: WebGPU tells it nothing about the adapter, so
the Rust record's adapter and driver fields are empty and the page's own view of
the adapter is recorded beside them.

Oak agrees with the native measurement to within a tenth of a percent. Spruce
does not: 62.9 ms in the browser against 94.2 ms native, on the same tree, the
same canvas and the same GPU. Both sessions are `valid` and both are tight, so
this is not noise or contention; something differs between the two pipelines on
the instance-heavy tree, and nothing here establishes what. The native rows stay
the reference the later specs compare against, and the gap is a question for the
fast-hero spec rather than an answer from this one.

## The page, driven

`npm run test:render` drives the page the way the owner does, on the hardware
adapter, and its scenarios are the browser half of R1 and R6. Every shipped
preset renders: Ordinary, Oregon white oak, Norway spruce, Telperion and
Laurelin all come back with wood, foliage and a canvas that reads back as a
picture rather than a blank, and no two of them draw the same one. Eight notches
of the height dial rebuild the tree and the panel's counts move with it
(4,313,760 to 4,381,024 wood triangles on Laurelin). The bare view drops the
foliage and keeps the wood (0 instances, 4,381,664 triangles), the leaf view is
one placed element (1 instance, 16 triangles), and the whole view is both. A
second browser launched with `--disable-gpu` never draws: the panel says "no
hardware GPU adapter; the only one offered was the software fallback", which is
the renderer's own sentence.

The five minute soak (`SOAK=1`, [soak.json](soak.json)) held one canvas and one
live device across ten polls with no rebuild the page asked for itself, and the
orbit answered a drag at the end. It ran on a display that belongs to a person:
344 keystrokes and pointer events arrived from the desktop during those five
minutes, which is why the canvas hash moves between polls while the tree does
not. The record counts them rather than hiding them, and the claim it makes is
unaffected - one device, one canvas, no rebuild.

## A note on the framing, for the owner's eye

The tree occupies roughly half the frame height in both stills, which is looser
than the framing margin's own words suggest. This is faithful, not broken: the
Rust `hero_pose` is a port of the harness's rule at `harness/stage.ts:667-680`,
stand-off included, so a browser frame and a headless still of the same tree are
the same picture by construction.

The looseness is in the rule itself. The margin solves a distance that fits the
subject with a quarter again its extent, and then adds half the subject's own
diagonal on top as orbit clearance. On the oak fixture the camera tests use
(18.2 x 22.6 x 17.8 m) that is a 42.7 m solve plus a 17.0 m stand-off, and the
tree's height then covers 55% of the frame rather than the 77% the margin alone
would give. Most of the stand-off is load-bearing: an all-angle fit on the
subject's bounding sphere needs 52.3 m with no margin left, so tightening to the
orbit-safe floor would recover 55% to 63% and no more, at the cost of the
clearance the orbit test pins. Framing the tree larger means changing the rule
for both renderers - a decision about the composition, not a defect in the port -
so nothing was changed here.

## Owner verdict

One slot per species, in the owner's eye sense of the strategy (R7). Left empty
deliberately; the stills above are what is being judged.

### Oregon white oak

> _verdict:_

### Norway spruce

> _verdict:_
