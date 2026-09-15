---
title: Viewer sees the ten-year oak stem flare into a bottle at the ground
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core growth radius / presets oregon-white-oak
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: 10 y oak base flares below breast height on seeds 1 and 7
root_cause: (observed via live QA — unconfirmed)
resolution_type: fix
related_to: [bug/ui/a-second-subject-on-stage-makes-every-2026-09-04, bug/ui/viewer-sees-norway-spruce-primaries-as-2026-09-15, bug/ui/viewer-sees-the-141-year-spruce-as-a-2026-09-15]
---

## Problem
The ten-year Oregon white oak's stem flares into a bottle at the ground in the live harness, on both seed 7 and seed 1. The owner rejected a bell-shaped trunk base in round 6 as "an awful regression".

## Steps to reproduce (cold)
1. http://localhost:5173/?species=oregon-white-oak&seed=7
2. Set age (years) to 10 in "whole tree" view; press "reframe".
3. Observe the stem between the ground and the first laterals.

## Expected
R1: the oak at 10 years reads as a sapling; the owner's round-6 rejection of the trunk-base bell stands.

## Actual
A thick flared base tapering sharply into the stem. The breast-height diameter is 4.8 cm, on the round-8 target; the flare is below breast height. Present on seed 1 in the round-8 native strips too.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-s7-whole-10.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-young-montage.png; .worktrees/fn-31-round5-grok/.flow/evidence/fn31/round8/oregon-white-oak-strips.png (seed 1 row)
- url: http://localhost:5173/?species=oregon-white-oak&seed=7

## Traceability
- R-IDs: [R1, R2]   scenario: S1   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
