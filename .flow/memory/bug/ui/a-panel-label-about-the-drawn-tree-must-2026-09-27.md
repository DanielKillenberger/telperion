---
title: "A panel label about the drawn tree must be derived from params, not stored"
date: "2026-09-27"
track: bug
category: ui
module: harness/LookControls.tsx
tags: [harness, react, state, fn-166]
problem_type: ui
symptoms: kept-tree indicator and seed box stale after other controls moved the dials
root_cause: indicator and seed held in state that only the look panel wrote
resolution_type: fix
related_to: [bug/ui/a-second-subject-on-stage-makes-every-2026-09-04]
---

## Problem
fn-166's look panel stored which tree was shown ("kept tree" / preset) in its own state and loaded the look's seed into params without the seed box, so a dial, a preset button or reset left "kept tree" pressed on a tree that was not the run's, and the drawn seed could differ from the seed box.

## What Didn't Work
A `showing` state set only by the panel's own buttons: every other control that writes params bypassed it.

## Solution
Derive the indicator from the params (`showing(opened, params)` in harness/look.ts compares families ignoring seed) and route every look write through one callback that sets params and the seed text together (`applyLook` in harness/GrowerDev.tsx).

## Prevention
In the harness, any label that describes the tree on the dials is computed from `params`, never stored beside it; a control that writes the seed also writes `seedText`.
