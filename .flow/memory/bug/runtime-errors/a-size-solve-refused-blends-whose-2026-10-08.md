---
title: A size solve refused blends whose target lies below what laterals can reach
date: "2026-10-08"
track: bug
category: runtime-errors
module: crates/telperion-core/src/blend/size.rs
tags: [blend, solver, tree-space]
problem_type: runtime-error
symptoms: blend::families refused a first-year walk between two species
root_cause: "the solve treated a target below the no-lateral growth as an error, though a first year grows no laterals"
resolution_type: fix
---

## Problem
fn-230 sized every blend between engine species by solving one factor on the lateral probabilities until the predicted size was the ends' geometric mean. The bracketed solve refused, by name, any blend whose target it could not reach. At age 1 the beech-oak midpoint grows 7.5 phytomers against a target of 7.48. A first year grows no laterals, so no factor can lower it, and a blend that had always built was refused.

## What Didn't Work
Treating an unreachable target as an error. The solver's floor (a factor near e^-64) looked like a pathological case, but it is ordinary wherever the tree's growth without laterals already passes the target.

## Solution
Where the bracket reaches its floor with the prediction still above the target, the factor is 0: no laterals, the nearest the factor comes (`crates/telperion-core/src/blend/size.rs`, `solve`). The grown count's solve starts at the kept factor and returns 0 when that is 0. A test covers first-year walks from the beech to the oak and from the beech to the spruce (`space_walks.rs`).

## Prevention
A solve on a control parameter needs a case where the target lies below what the parameter can reach. Test the smallest age the input allows, not only the ages the evidence sheet uses.
