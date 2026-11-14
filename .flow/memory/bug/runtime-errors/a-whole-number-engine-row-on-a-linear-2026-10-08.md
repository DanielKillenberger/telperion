---
title: A whole-number engine row on a linear scale breaks walks between species
date: "2026-10-08"
track: bug
category: runtime-errors
module: crates/telperion-space/src/species/scale.rs
tags: [engine, blend, scale, validation]
problem_type: runtime-error
symptoms: blend::families refused a walk between two valid families
root_cause: cluster_leaves mixed linearly to 1.5 and validation requires whole
resolution_type: fix
---

## Problem
fn-223 added `cluster_leaves`, an engine PA row whose validation requires a whole number, and listed it in `species/scale.rs` on the `Linear` scale. A walk between two valid species mixes every setting. Walking from 0 leaves to 3 at t = 0.5 gave 1.5, the final `Species::validate` refused it, and `blend::families` failed between two families that were each valid.

## Solution
A `Whole` scale mixes by value and rounds to the nearest whole (`crates/telperion-space/src/species/scale.rs`). `cluster_leaves` uses it. Fractional input is still refused, and every walked value is valid. The regression test `a_walk_between_cluster_counts_stays_whole` walks 0 to 3 in 21 steps.

## Prevention
When a new engine row's validation is stricter than its scale's range (whole numbers, a closed set), check what `Scale::mix` produces between two valid ends. Add a walk test across differing values of the row, not only one at its neutral, because neutral-to-neutral walks pass whatever the scale.
