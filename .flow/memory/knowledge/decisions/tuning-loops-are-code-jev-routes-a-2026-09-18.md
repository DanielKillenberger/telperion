---
title: Tuning loops are code; Jev routes a verdict to a dial direction
date: "2026-09-18"
track: knowledge
category: decisions
module: "experiments/fn58-tuning-loop, crates/telperion-jev"
tags: [tuning, jev, typesafe, loop, fn-58, fn-68]
applies_when: Tuning loops are code; Jev routes a verdict to a dial direction
---

A species tuning round is a code loop: measure the candidate, render its matched stills, read the compare script's numbers against the photograph's, step one dial, and let the lowest score stand. Jev's only part is one Choice per dial over up, down and hold, read from the owner's verdict notes, which proposes the four moves code evaluates first; a full single-step sweep is the fallback. The probe of 2026-09-18 on the beech (`experiments/fn58-tuning-loop/`) decided it: the sweep reached 0.145 from 0.245 in 74 evaluations, the direction framing 0.154 in 13, and the two framings that asked Jev to judge moves or compare measured candidates did not move the score. The loop lives in fn-68, beside fn-58, which keeps its finish line on fn30 validation.

## Considered Options

- Jev returns a parameter set: it has no channel for numbers and no model of a dial's effect.
- Jev picks among measured candidates: flat between 0.15 and 0.28 over 22 options.
- A vision model as the judge: breaks the model-swap test and the owner's-eye Boundary; kept only as a note source that never scores.

## Consequences

- Every shipped value stays one code proposed and a render measured.
- The direction question needs a labelled set from fn-34's and fn-62's rounds before it is trusted.
- New measurements, not new judges, are the lever for the fidelity the five still numbers miss.
