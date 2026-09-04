---
title: "Interpenetrating junctions: contain the whole ring against the surface as drawn"
date: "2026-09-04"
track: bug
category: runtime-errors
module: src/mesh/surface.ts
tags: [procgen, mesh, geometry, invariants]
problem_type: runtime-error
symptoms: "A child branch's start ring and back cap protruded through the parent's skin at a fork, showing as a flat crescent seam"
root_cause: "Containment was asserted on the ring's centre and measured against the parent's mean analytic radius, not every vertex against the lobed, finitely-sampled surface actually drawn"
resolution_type: fix
related_to: [bug/runtime-errors/zero-width-is-not-no-constraint-the-2026-09-04]
---

## Problem
The swept surface joins a child branch to its parent by interpenetration:
the child's first ring starts inside the parent's solid and emerges through
its skin, so there is no seam to stitch. Containment was asserted on the
ring's CENTRE, which is not the claim - at a balanced fork each child is
1/sqrt(2) of the parent, swollen by the fillet to about 0.95 of it, so most
of the starting ring and its flat back cap protruded through the parent's
surface as a visible crescent at exactly the junction the socketing exists
to hide.

## What Didn't Work
Two clamps that were each still measuring against the wrong surface:

1. Ring radius <= parentRadius * sqrt(1 - forkSocket^2). Correct for
   circles, wrong here: both sections are lobed, so the child's widest
   vertex is (1 + lobeDepth) of its mean and the parent is only solid out
   to (1 - lobeDepth) of its own.
2. Adding the lobe factors but still comparing against the analytic
   section. The parent as DRAWN is a polygon of `segments` sides, whose
   inscribed radius is another cos(pi/segments) in - and a vertex was
   still outside the actual trunk mesh.

## Solution
Measure containment against the parent as it is drawn, and put every
conservatism under the same root:

    inscribed  = parentRadius * (1 - lobeDepth) * cos(pi / segments)
    socketDepth = min(forkSocket * parentRadius, MAX_FORK_SOCKET * inscribed)
    contained  = sqrt(inscribed^2 - socketDepth^2) / (1 + lobeDepth)

in `src/mesh/surface.ts`. Capping the sink as a fraction of the
inscribed radius rather than of the mean is what keeps `contained` strictly
positive - a ring clamped to zero is a band of degenerate triangles, which
trades a seam for a pinch.

## Prevention
Assert the geometric claim on EVERY vertex of the ring, never on its
centre, and state the bound in terms of the drawn surface (lobes and
segment count included). A test that checks a centre is testing the
placement, not the containment. `src/mesh/surface.test.ts`
"sockets a child inside its parent" is that test.
