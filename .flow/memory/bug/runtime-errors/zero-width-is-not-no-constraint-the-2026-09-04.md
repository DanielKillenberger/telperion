---
title: "Zero width is not no constraint: the tree forked below its own envelope"
date: "2026-09-04"
track: bug
category: runtime-errors
module: src/skeleton/colonize.ts
tags: [procedural-geometry, space-colonization, envelope, invariants, grower]
problem_type: runtime-error
symptoms: Branches grew from ~4 m on a tree whose authored crown starts at 7.2 m; the containment test called them inside
root_cause: "The trunk stopped climbing on 'an attractor is in reach' rather than 'the crown is reached', and containment was a radius-only test that is vacuous where the profile has zero radius"
resolution_type: fix
---

## Problem
The generator's whole premise is that the silhouette is authored and only
the branching is generated. The envelope has zero width below its
`crownBase`, so a branch down there is outside the authored shape however
plausible it looks. The first implementation grew branches from about
4 m on a tree whose crown starts at 7.2 m, and no test noticed.

Two separate mistakes, one shared cause: a region of the shape where the
width is zero, and code that read zero width as no constraint.

## What Didn't Work
The trunk climbed "until some attractor comes into reach" and that was
taken to mean "until the trunk reaches the crown". It is not the same
condition. The influence radius is nine times the growth step, so the
lowest attractors come into range of the trunk long before the trunk has
climbed anywhere near them - the tree started forking at roughly half its
intended trunk height, out in the open.

The containment test that should have caught it was written as a radius
comparison alone: `hypot(x, z) <= radiusAt(y)`. Where the profile has no
width that reads `0 <= 0`, so the entire trunk axis answered "inside" -
including the ground beneath the tree and the sky above its tip.

## Solution
`src/skeleton/colonize.ts`: the bare-trunk height is now an
explicit `GrowthConfig.trunkHeight`, and the reach loop climbs to it
before any branching is allowed - `y < trunkHeight || !anyInReach()`.
`grow.ts` fills it from the envelope's own `height * crownBase`, so the
authored shape states the constraint rather than the growth distances
happening to satisfy it.

`src/envelope.ts`: `envelopeContains` rejects points outside
`[0, height]` before it compares radii.

## Prevention
Where a shape has a zero-width region, a predicate written only in terms
of that width is vacuous exactly there - which is the region least likely
to be tested and the one where "inside" is most obviously wrong. Check
the extent as well as the width.

And a reach condition is not a goal condition. When an authored value
(`crownBase`, and later a taper limit or a foliage band) is supposed to
be respected, pass it in and stop on it; do not let a tuning distance
happen to produce it. The tuning distance changes, and then the
constraint is silently gone.

fn-11 has five tasks left, all of them growing more geometry inside this
same envelope: torsion, radii, the swept mesh, foliage frames, and the
two authored silhouettes. Each is one vacuous predicate away from
repeating this.
