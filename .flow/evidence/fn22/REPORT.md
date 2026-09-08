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
