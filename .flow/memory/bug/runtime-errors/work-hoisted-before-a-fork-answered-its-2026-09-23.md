---
title: Work hoisted before a fork answered its error ahead of earlier stages
date: "2026-09-23"
track: bug
category: runtime-errors
module: crates/telperion-core/src/pipeline.rs
tags: [pipeline, concurrency, error-order, fn-102]
problem_type: runtime-error
symptoms: Same request errors differently native vs Wasm
root_cause: Shared ring sweep ran before the fork and returned via ? ahead of the plan stage
resolution_type: fix
---

## Problem
fn-102's pipeline runs wood and leaves side by side on native targets. For a family whose leaves sit on the wood, the shared ring sweep ran before the fork, and its `?` returned the sweep's error at once. Run in turn (Wasm), the element and leaf plan ran first, so the same bad request answered "surface parameters" natively and "leaf segments" in Wasm.

## What Didn't Work
The first error-order test only failed wood and leaves (no contact), so the pre-fork sweep path was never exercised.

## Solution
`crates/telperion-core/src/pipeline.rs`: when the early sweep fails, run the plan stage and return its error if it has one, else the sweep's. The test `the_earliest_failing_stage_answers` covers contact + bad element under both schedules.

## Prevention
Any work hoisted ahead of a fork for concurrency must keep its error behind every stage that precedes it in stage order. Test error precedence per schedule, with every hoisted artifact made to fail.
