---
title: Viewer sees Norway spruce primaries as straight rays rising at every crown heigh
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core presets norway-spruce / harness bare view
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: "lower and mid-crown primaries straight, +21 to +28 deg above horizontal at 36.9 and 65 y"
root_cause: (observed via live QA — unconfirmed)
resolution_type: fix
related_to: [bug/ui/a-second-subject-on-stage-makes-every-2026-09-04]
---

## Problem
A viewer judging the Norway spruce in the live harness finds the primary branches drawn as straight rays that rise from the trunk at every crown height, from 26.6 years to maturity. The owner's words: "At all ages the branching looks weird and too straight. Especially mature the branches grow upwards."

## Steps to reproduce (cold)
1. http://localhost:5173/?species=norway-spruce&seed=7
2. Set view to "bare branches"; set age (years) to 65, then 36.9, then 26.6.
3. Wait for the frontier and the geometry line to update; press "reframe".
4. Observe the first-order branches off the leader.

## Expected
R1: "every later age reads as a continuation of the same tree" of a Norway spruce. Norway spruce carries ascending branches in the upper crown, near-horizontal branches mid-crown, and drooping lower branches with upturned tips and hanging branchlets.

## Actual
Primaries are straight and ascend at all heights. A native probe at seed 7 measures mean chord elevation of first-order branches from the leader: +24.9°, +25.6°, +21.2° in the lower three fifths of the crown at 65 y; +27.9°, +24.8°, +24.3° at 36.9 y. Only the top fifth reaches horizontal. Reproduced on a second run with identical geometry counts (4,561,160 wood triangles at 65 y) and 97 of 1.4 million pixels different.

## Evidence
- console: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/run-console.log (no page errors; one 404 resource load)
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-s7-bare-{26.6,36.9,65}.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-s7-bare-r2-65.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-montage.png
- url: http://localhost:5173/?species=norway-spruce&seed=7
- probe: crates/telperion-core/examples/qa_branch_posture.rs (temporary, removed after the pass)

## Traceability
- R-IDs: [R1]   scenario: S2   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
