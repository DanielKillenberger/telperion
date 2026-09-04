---
title: An AC that enumerates the default state is the test; a comment is not a licence
date: "2026-09-04"
track: bug
category: ui
module: harness/stage.ts
tags: [acceptance-criteria, three.js, lighting, clay-render, scope]
problem_type: ui
symptoms: Default clay scene carried a directional fill the AC did not name; review flagged it NEEDS_WORK
root_cause: Craft instinct added a light beyond the enumerated default and a comment argued for it instead of removing it
resolution_type: fix
---

## Problem
The clay harness's acceptance is explicit: "Clay is the default and only
judging mode; any lighting extras are behind an explicit toggle." The
first implementation put a hemisphere sky light *and* a weak directional
fill in the default scene, with a comment explaining that the fill "only
keeps the shaded side from going to a single flat value." Review caught
it. The comment was the tell: it argued for the deviation instead of
removing it.

## What Didn't Work
Reasoning about what makes a good clay render (DCC clay setups do use a
dome plus a soft key) instead of reading what the acceptance criterion
actually enumerates. The AC named one light. Craft instinct added a
second and then narrated why it was fine.

## Solution
`harness/stage.ts`: the default scene holds the
hemisphere and nothing else; the directional fill moved behind
`setLightingCheck` alongside the shadow key, and the hemisphere's
intensity was raised to cover what the fill had been doing. The toggle
is now a proper key-and-fill pair, and "off" is exactly the AC's light.

## Prevention
When an AC enumerates what the default state contains, the default state
contains exactly that and the enumeration is the test. A code comment
justifying an addition to an enumerated default is a review finding, not
a licence - if the extra is genuinely needed, the AC is wrong and gets
changed, not narrated around. This spec has six more tasks under the
same clay constraint (fn-11.2 through .7); each is one "harmless helper"
away from the same finding.
