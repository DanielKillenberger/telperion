---
title: A second subject on stage makes every 'what is on screen' read a call site
date: "2026-09-04"
track: bug
category: ui
module: harness/GrowerDev.tsx
tags: [react, state, harness, grower, scale-reference]
problem_type: ui
symptoms: reframe placed the 1.8 m scale figure by the dial height while the comparison showed two 130 m presets
root_cause: a new compare branch was applied in the build effect but not at the other call site reading params.height
resolution_type: fix
related_to: [bug/ui/a-mid-task-instruction-that-adds-a-2026-09-04]
---

## Problem
The grower harness gained a comparison mode: a toggle that puts two library
presets on the stage instead of the tree the sliders describe. The build effect
correctly asked which subject was standing there before choosing the height to
frame it by (`compare ? tallestPresetHeight() : params.height`). The `reframe`
button, twenty lines further down, did not - it passed `params.height`
unconditionally. With the panel at its defaults that placed the 1.8 m scale
figure at a 24 m tree's foot while two trees over a hundred metres tall stood
in front of it, so the one thing establishing the trees' size was decided by
dial state that was not driving the subject at all.

## Solution
Both call sites ask the same question and get the same answer, in
`harness/GrowerDev.tsx`. The condition is duplicated rather
than hoisted because the two call sites read the state at genuinely different
moments - the effect after the subject is built, the button before it - but the
expression is now identical in both.

## Prevention
When a component grows a second SUBJECT (a compare mode, a preview, an
alternate source), every place that answers "what is on screen" becomes a call
site of the same new conditional, and the ones outside the effect that builds
it are the ones that get missed. The cheap sweep at authoring time is to grep
the component for every existing read of the old subject's state
(`params.height` here) and check each one against the new branch - the compiler
cannot help, because the stale value has exactly the right type.
