---
title: Viewer sees a separate tuft above the 20-year oak's crown
date: "2026-09-15"
track: bug
category: ui
module: crates/telperion-core branching scaffold / oak crown
tags: [qa, fn-31-growth-rule-sapling-form-thickening-by, harness]
problem_type: ui
symptoms: "20 y oak: tuft above an oval crown with a gap, seeds 7 and 42"
root_cause: (observed via live QA — unconfirmed) the crown widens suddenly near 20 y; the umbrella invariant checks only 24-30 y
resolution_type: fix
---

## Problem
After round 11, the 20-year Oregon white oak carries a separate tuft of foliage above its main crown, with a gap between them, on seeds 7 and 42. It is the same kind of crown gap as the umbrella the owner rejected in round 6, at an earlier age than the umbrella invariant checks. The round-11 worker also measured the crown widening suddenly here: on seed 7 its widest reach goes from 0.77 to 1.04 of the height in one year.

## Steps to reproduce (cold)
1. http://localhost:5173/?species=oregon-white-oak&seed=7 (and seed=42)
2. Set age (years) to 20 in the "whole tree" view; press "reframe".

## Expected
The owner's round-6 decision: the oak crown rounded and broad, with no umbrella. The round-9 invariant forbids a nearly empty band between crown tiers, but it checks only 24 to 30 years.

## Actual
An oval crown with a small tuft above it and a visible gap, at 60,589 leaves on seed 7 and 57,325 on seed 42. By 26.7 years the crown is a broad dome with no gap.

## Evidence
- screenshots: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r11/oak-s7-whole-20.png, .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r11/oak-s42-whole-20.png, montage .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r11/oak-young-r11.png
- console: .flow/tmp/qa-fn-31-growth-rule-sapling-form-thickening-by/r11/console.log (no page errors)

## Traceability
- R-IDs: [R1]   scenario: S1   driver_rung: playwright (headed chromium, Vulkan WebGPU)   viewport: 1400x1000
