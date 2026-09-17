---
title: Measure every preset candidate before rendering; the photograph's numbers pick t
date: "2026-09-17"
track: knowledge
category: best-practices
module: crates/telperion-core/src/presets/species.rs
tags: [presets, tuning, measurement, matched-shots, fn-34]
applies_when: "Any round that moves rows in a species preset against a reference photograph, and fn-58's tuning route"
---

## Practice

A value-table round measures every candidate before it renders any, and picks by the photograph's numbers, not by eye. Code changes the rows, the species measurement example counts limbs, branches, twigs, leaves and leaf area at the fixed seed, and only the two or three candidates whose counts sit in the right range are rendered and compared against the photograph's centre brightness and occupied share. The candidate whose whole-tree centre lands on the photograph's is the one that ships; the lighter ones that overshoot are the evidence for stopping there.

## Why

fn-34 round 26 had a proposal taken at its word, four twig laterals at 0.4 of the parent's length, which measured as a third of the leaf the tree had. Nine candidates were measured, four rendered. The two lighter rendered ones overshot the photograph's centre of 83 by 13 and 6 levels; seven laterals at 0.55 landed on 82.2. The round took one worker session instead of the three or four the earlier rounds needed, because nothing was rendered that the counts had already ruled out.

## Applies when

Any round that moves rows in a species preset against a reference photograph, and any tuning route fn-58's pipeline automates: code proposes, the measurement decides which candidates are worth a render, the render decides which ships.
