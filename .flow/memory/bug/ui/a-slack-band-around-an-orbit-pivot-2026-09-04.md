---
title: A slack band around an orbit-pivot clamp left the pivot off the subject
date: "2026-09-04"
track: bug
category: ui
module: harness/stage.ts
tags: [three-js, camera, orbit-controls, invariants, tolerances]
problem_type: ui
symptoms: "After a 400 m to 24 m shrink the orbit pivot sat ~48 m up, above the new tree; orbiting lost the subject"
root_cause: "The clamp targeted the subject box expanded by a full span - slack sized by the pan case it protected, not by the failure it corrected"
resolution_type: fix
---

## Problem
The orbit pivot of the grower's dev stage was left pointing at a tree that
was no longer there. `fitRoom` runs on every rebuild, and after a drag from
400 m down to 24 m the pivot still sat where the old crown's centre had
been, 200 m up. Orbiting from there swings a 24 m tree clean off the frame.

## What Didn't Work
The first fix clamped the pivot into the new subject's box EXPANDED BY A
FULL SPAN, on the reasoning that panning off the tree to inspect one limb
is the owner moving the camera and the room does not get a vote on it. The
slack was real but it was sized by the thing it was protecting rather than
by the failure it was correcting: at 24 m a full span is another 24 m, so
the pivot landed near y=48 - still above the tree, still losing the subject
on the next orbit. A tolerance wide enough to swallow the defect is not a
tolerance, and review caught it (P2, confidence 75).

## Solution
`pivotOn(box, pivot)` in harness/stage.ts: pure, returns a
new point, clamps to the subject's ACTUAL bounds. The pan case needs no
slack at all - the whole subject is inside its own bounds, so panning across
a crown is already unchanged, and a subject that grew already contains the
pivot. Only a subject that shrank away from the pivot moves anything.

## Prevention
The suite already asserted the room's invariant (`solveRoom` never writes an
orbit limit on the wrong side of the camera) and it passed throughout - it
tested the numbers next to the defect rather than the corrected state. When
a fix is "correct X before measuring from it", the test asserts X's post-
condition (the pivot is inside the new bounds), not just that the
measurement downstream of it is consistent. The corrective half of a change
needs its own pure, exported seam so it can be asserted at all; both new
pivot tests were confirmed red against the previous behaviour before the
fix was accepted.
