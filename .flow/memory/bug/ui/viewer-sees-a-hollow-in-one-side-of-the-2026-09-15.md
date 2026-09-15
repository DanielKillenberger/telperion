---
title: Viewer sees a hollow in one side of the 20-to-22-year oak crown
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core branching scaffold / oak crown
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: "C-shaped flank at 20 and 22 y on seed 7, hole at 22 y on seed 42, gone by 26.7 y"
root_cause: (observed via live QA — unconfirmed)
resolution_type: fix
---

## Problem
After round 12, the oak's crown at 20 and 22 years has a hollow in one side under a single reaching limb: on the left of seed 7, and smaller in the middle-left of seed 42 at 22 years. It shrinks by 24 years and is gone by 26.7. The no-gap invariant measures height bands, so it cannot see a hollow in a side.

## Steps to reproduce (cold)
1. http://localhost:5173/?species=oregon-white-oak&seed=7 (and seed=42)
2. Set age (years) to 20, then 22, in the "whole tree" view; press "reframe".

## Expected
The owner's round-6 decision: a rounded, broad oak crown.

## Actual
A C-shaped flank at 20 and 22 years on seed 7, and a hole at 22 years on seed 42. Minor: the crown is otherwise continuous.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r12/oak-s7-whole-20.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r12/oak-s7-whole-22.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r12/oak-s42-whole-22.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r12/oak-teen-r12.png
- console: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r12/console.log (no page errors)

## Traceability
- R-IDs: [R1]   scenario: S1   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
