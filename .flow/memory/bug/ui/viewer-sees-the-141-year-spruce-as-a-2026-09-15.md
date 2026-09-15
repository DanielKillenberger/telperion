---
title: "Viewer sees the 14.1-year spruce as a different, gangly tree between 5 and 26.6 "
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core presets norway-spruce / harness
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: 14.1 y spruce sparse with long straight laterals; 5 y laterals straight stubs
root_cause: (observed via live QA — unconfirmed)
resolution_type: fix
related_to: [bug/ui/a-second-subject-on-stage-makes-every-2026-09-04, bug/ui/viewer-sees-norway-spruce-primaries-as-2026-09-15]
---

## Problem
The 14.1-year Norway spruce does not read as a continuation between the 5-year and 26.6-year trees: it is a sparse, gangly tree with long, thin, straight laterals, and the 5-year sapling carries short straight stubs like a bottle brush.

## Steps to reproduce (cold)
1. http://localhost:5173/?species=norway-spruce&seed=7
2. Set age (years) to 5, then 14.1, then 26.6, in "whole tree" and "bare branches" views; press "reframe" after each.
3. Compare the three frames.

## Expected
R1: "the spruce at 1, 5 and 14.1 years ... read as actual saplings ... branched saplings carrying foliage, and every later age reads as a continuation of the same tree".

## Actual
At 14.1 y the tree is 4.29 m with 33 first-order branches averaging 1.69 m long in the second fifth of its height (probe), sparse needles (179,770 foliage instances against 3,064,506 at 26.6 y), and a silhouette unlike the dense cone at 26.6 y. Reproduced on a second run with identical counts.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-s7-whole-{5,14.1,26.6}.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-s7-bare-{5,14.1}.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-s7-bare-r2-14.1.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/spruce-montage.png
- url: http://localhost:5173/?species=norway-spruce&seed=7

## Traceability
- R-IDs: [R1]   scenario: S2   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
