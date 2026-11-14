---
title: Draw-on-change left the canvas blank after an unchanged-size resize
date: "2026-10-09"
track: bug
category: ui
module: src/browser/render.ts
tags: [webgpu, canvas, redraw, resize]
problem_type: ui
symptoms: An idle canvas went blank after a resize notice that kept its pixel size
root_cause: "Writing canvas.width or height clears the picture even at the same value, and the redraw gate saw no change"
resolution_type: fix
---

## Problem
fn-246 made the browser renderer draw only when the picture changed (`redraw.rs`). The page's ResizeObserver callback still wrote `canvas.width` and `canvas.height` on every notice. A write clears a WebGPU canvas even at the same value, and Rust's `resize` returned early on an unchanged size without asking for a frame. So a fractional CSS resize could leave an idle canvas blank until the next camera or scene change. Under the old continuous frame loop this was invisible, because the next animation frame repainted it.

## What Didn't Work
The redraw unit tests covered resize as "a changed size asks for a frame". They never covered a size notice that changes nothing, which is the case that clears the picture.

## Solution
`src/browser/render.ts` writes a canvas dimension only when it differs from the current one. `src/browser/render.test.ts` stubs the observer and counts the writes on an unchanged notice.

## Prevention
When a loop changes from always drawing to drawing on change, list every caller that can invalidate the picture without going through the dirty flag: canvas size writes, surface reconfiguration, a lost context. Test each with an unchanged value.
