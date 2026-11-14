---
title: "Wood hung after the solve bore cauline leaves: node kind does not exclude it"
date: "2026-10-19"
track: bug
category: integration
module: crates/telperion-core/src/pipeline/foliage/plan.rs
tags: [foliage, clothing, cauline, fn-283]
problem_type: integration
symptoms: Frond axes and leaf bases gained cauline leaves once cauline and shootRadius were above zero
root_cause: bearing_of bears any slender wood; the clothing's tail was not marked
resolution_type: fix
---

## Problem
fn-283 hung each frond's axis as wood of the leaf bases' kind (`NodeKind::Branch`, `stem: false`), and the spec held that this kind keeps foliage off it. It does not: `plan::bearing_of` bears cauline leaves on any wood within the slender bound, and the axes taper to FINEST. A palm with cauline 1 and shootRadius 0.1 went from 6,554 leaves to 77,518 once the axes were drawn. The retained bases had the same exposure at shootRadius 1.

## Solution
The tree records where its clothing tail begins (`Tree::clothing`, set by `branching::clothe` when it hangs or recognises the tail). `bearing_of` and placement's slender shoots bear nothing at or past it.

## Prevention
Wood appended after the solve is not leaf-bearing by its kind. Check every bearing predicate, on both executors, before a spec claims new wood is bare. A red test: a family with the wood drawn and cauline above zero keeps the leaf count it has without that wood.
