---
title: Viewer sees the 26.7-year oak as a flat umbrella on a bare stem
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core branching scaffold / presets oregon-white-oak
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: flat wide top with a gap above a small lower cluster at 24-28 y
root_cause: (observed via live QA — unconfirmed)
resolution_type: fix
---

## Problem
The 26.7-year Oregon white oak is a flat-topped umbrella: a wide crown on a bare stem with a separate small cluster of branches below it. The owner named "the 26.7 y umbrella" in the round-6 verdict; it is still present.

## Steps to reproduce (cold)
1. http://localhost:5173/?species=oregon-white-oak&seed=7
2. Set age (years) to 26.7 in "whole tree" view, then scrub through 20, 24 and 28 in "bare branches"; press "reframe" after each.

## Expected
The owner's round-6 decision: the oak crown "rounded and broad at every age from 26.7 years to maturity", with the 26.7 y umbrella included in that.

## Actual
At 24 y a flat spreading layer appears at the top of a thin leader, and at 26.7 y the crown is a wide flat top with a gap above a small lower cluster. This is the crown fork recorded in round 7: every candidate rule that fills the lower crown broke a gate.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-s7-whole-26.7.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-s7-bare-{20,24,28}.png, montages .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-young-montage.png and .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/oak-scrub-montage.png
- url: http://localhost:5173/?species=oregon-white-oak&seed=7

## Traceability
- R-IDs: [R1]   scenario: S1   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
