---
title: A mid-task instruction that adds a requirement has not withdrawn the AC
date: "2026-09-04"
track: bug
category: ui
module: harness/params.ts
tags: [acceptance-criteria, scope, art-direction, parameterization, grower]
problem_type: ui
symptoms: "Review filed P1: the control the AC names was deleted when the field it drove was split into five named parameters"
root_cause: A mid-task refinement was read as superseding the written acceptance rather than adding to it
resolution_type: fix
related_to: [bug/ui/an-ac-that-enumerates-the-default-state-2026-09-04]
---

## Problem
fn-11.3 was dispatched with an acceptance criterion naming one control -
"torsion slider moves the tree from straight to visibly writhing" - and
then amended mid-task by two coordinator messages asking for the field to
be split into five independently named parameters, because one lumped
number cannot be art-directed. The five were built and the `torsion` dial
was deleted as the thing they replaced. The review caught it as a P1: the
written AC still named a control that no longer existed, and no single
move took the tree from straight to writhing any more.

## What Didn't Work
Treating the mid-task instruction as superseding the AC. It refined *how*
the field is parameterized; it never withdrew the acceptance the task is
judged against. The coordinator's own wording even left the panel surface
open ("whether each of those gets its own panel dial is your call") while
requiring only that the *library config* expose the terms individually -
so the deletion was never asked for.

## Solution
Both, composed rather than chosen between. The library keeps the five
named terms (`src/torsion.ts`: gravitropism, lean,
writheAmplitude, writheWavelength, spiralRate) as the preset shape a later
task authors two trees in. The panel keeps a `torsion` master that scales
the three which are departures from vertical
(`harness/skeleton-view.ts:toSkeletonParams`), so zero
is dead straight and one is the terms as dialled. Gravitropism stays
outside the master, or torsion 0 would mean "no opinion about direction"
rather than "straight".

## Prevention
When a mid-task instruction reshapes a surface the AC names, re-read the
AC before deleting anything from that surface: an instruction that adds a
requirement has not removed one. A named control in an AC is satisfied by
composition - keep it as a view over the new structure - not by
substitution. Cheapest check: grep the task's Acceptance section for every
identifier being deleted in the diff.
