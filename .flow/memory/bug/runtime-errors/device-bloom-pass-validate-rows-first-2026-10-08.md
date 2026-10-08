---
title: "Device bloom pass: validate rows first, narrow time deltas in f64, share readbac"
date: "2026-10-08"
track: bug
category: runtime-errors
module: crates/telperion-render/src/generation/fine/resident.rs
tags: [gpu, fine, florets, f32, readback]
problem_type: runtime-error
symptoms: GPU counted with unchecked rows; short window lost on device; extra round trips
root_cause: check after dispatch; absolute times narrowed separately; per-block summary readback
resolution_type: fix
---

## Problem
fn-229 put the device's floret pass into fn-212's device path. The review found three defects:
- The bloom rows reached the GPU counting kernel before `bloom_element`'s range check. A `florets` of 1e9 would have looped a billion times per node.
- The absolute time `now` and the window's opening `opens` were each narrowed to f32 separately. A window a billionth of a year long then lost its midpoint, and the device drew no florets.
- The floret summary was read back separately, and its mass grid waited on its own completion. That added round trips that R1 forbids.

## Solution
- Build the organ, and with it the check, before the stations read its rows (`resident.rs`: `Organ::of` before `station_rows`).
- Upload `(now - opens).rem_euclid(1.0)`, computed in f64 (`data::flower`).
- Record every placement into one encoder, copy all summaries into one buffer, read it back once, and record both mass grids into one submission (`resident.rs`: `read`; `compute.rs`: `record_mass`).

## Prevention
- Validate a row before any kernel reads it.
- Narrow differences in f64, never two absolute values separately.
- When a spec says "no added readback", count `read_async` and `io::complete` calls per build before and after the change.
