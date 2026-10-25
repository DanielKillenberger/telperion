---
title: "Rows hoisted before the expansion read the raw family, not Family::applied"
date: "2026-10-20"
track: bug
category: runtime-errors
module: crates/telperion-render/src/generation/resident/boots.rs
tags: [device, supernatural, fn-283]
problem_type: runtime-error
symptoms: A palm with a supernatural size override drew no crown on the device
root_cause: Frond rows moved ahead of Expansion read family.canopy instead of family.applied()
resolution_type: fix
---

## Problem
fn-283 task .2 moved the device's frond rows (`rosette_rows`, `organ_rows`, the blade) from the stations stage to the boots stage, which runs before the pipeline's `Expansion` exists. The moved code read `family.canopy` and `family.element` directly. The stations had read `x.leaves().canopy`, which `Inputs::of` takes from `Family::applied()`. A palm whose supernatural size override lifts a natural size of zero drew no crown, and a size override gave young fronds the wrong strap ratio.

## Solution
`boots.rs`' `frond_rows` resolves `family.applied()` before it reads any row. `agree_tests.rs`' `every_crown_the_leaf_base_stage_draws_is_placed_as_the_cpu_places_it` places a palm with natural size 0 and override size as the CPU does.

## Prevention
Any rows hoisted ahead of `executor::resident` (or any stage that reads `Inputs::of`) read the family through `Family::applied()`, never its raw fields. The byte-equivalence digest did not catch it: no shipped preset enables the supernatural group.
