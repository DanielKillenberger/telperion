---
title: A trunk-region guard on the parent node let the child step in anyway
date: "2026-09-04"
track: bug
category: runtime-errors
module: src/skeleton/colonize.ts
tags: [procgen, space-colonization, invariants, guards]
problem_type: runtime-error
symptoms: "A limb grew one step below the bare-trunk height, into the part of the envelope with no width"
root_cause: "The no-branching-below-trunkHeight check tested the growing node, never the position its step would land in"
resolution_type: fix
---

## Problem
Space colonization refused to grow *from* any node below the bare-trunk
height, and that read as "nothing grows in the trunk region". It is not
the same rule. A node just above the line could still take one step
downward and put its child below it, into the part of the envelope that
has no width at all - a limb outside the authored silhouette.

It stayed invisible while the generator had no directional persistence,
because the bias field's gravitropism snapped a downward step back up
within one node. Adding a turn limit removed that accident: a branch
that arrives heading down now keeps heading down for a few steps, and
the graze appeared - one node, four centimetres under the line, in one
of eighteen swept configurations.

## What Didn't Work
Loosening the invariant test to "a node below the crown base must climb
only if its parent is also below it", on the argument that one step of
overshoot is what a step length means. That is true and it is still the
wrong fix: it documents the leak instead of closing it, and the review
called it exactly that.

## Solution
The guard belongs on the candidate position, not only on the parent.
src/skeleton/colonize.ts, in the progress pass: a step whose
child would land below `config.trunkHeight` is not progress, whatever it
closes on, so the tip stops. The original invariant assertion in
grow.test.ts was restored unchanged.

## Prevention
When a guard is stated over a node, ask whether the thing it protects is
the node or the step. A rule about a region wants testing at both ends
of every edge that crosses into it. And when a new mechanism makes a
dormant invariant fire, the first hypothesis is that the invariant was
always being violated and something was hiding it - not that the
invariant is too strict.
