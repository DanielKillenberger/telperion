---
title: Viewer sees the ten-year oak as a leafy bush with no visible stem after round 9
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core branching scaffold station reach / lowLimbReach
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: "10 y oak 2.3 m bush with foliage from near the ground, 651 nodes"
root_cause: (observed via live QA — unconfirmed) lowLimbReach applies to every station of a sapling
resolution_type: fix
---

## Problem
After round 9, the ten-year Oregon white oak reads as a leafy bush with almost no visible stem on seeds 1 and 7. The owner rejected this form in the round-5 verdict: "the ten-year oak is a shrub with no visible stem and nothing leans". The round-6 owner decision asked for "a ten-year oak with a visible leaning stem and woody laterals".

## Steps to reproduce (cold)
1. http://localhost:5173/?species=oregon-white-oak&seed=7 (and seed=1)
2. Set age (years) to 10 in the "whole tree" view; press "reframe".

## Expected
The owner's round-6 decision: a ten-year oak with a visible leaning stem and woody laterals.

## Actual
A 2.3 m bush, 2.2 m wide, with foliage from near the ground. It has 651 nodes and 4,878 leaves at seed 7, against round 8's 315 nodes and 2,631 leaves. Round 9's lowLimbReach 0.6 caused it: every station of a ten-year oak stands below the mature crown base, so every limb reaches for mature-crown room. The existing test ten_year_oak_is_a_stemmed_sapling still passes and does not catch it.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r9/oak-s7-whole-10.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r9/oak-s1-whole-10.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r9/oak-whole-montage.png
- native: growth_curve at seed 7, 10 y: height 2.30 m, x from -1.12 to 1.10 m, 651 nodes
- console: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r9/console.log (no page errors)

## Traceability
- R-IDs: [R1]   scenario: S1   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
