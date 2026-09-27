---
title: "A CI size check's skip path must be the one sanctioned absence, not any error"
date: "2026-09-27"
track: bug
category: integration
module: scripts/artifact-budgets.mjs
tags: [ci, size-budget, thresholds, fail-open, fn-178]
problem_type: integration
symptoms: Growth limit skipped on API errors and old PRs; exactly 5 percent rejected
root_cause: "Catch-all fail-open, one-page lookup, float ratio comparison"
resolution_type: fix
---

## Problem
The first cut of the per-PR size-growth check (scripts/artifact-budgets.mjs) had three defects codex found: the master-run lookup read one page of 30 runs, so an older PR's base fell off the window and growth went unchecked; a catch-all turned every API, download or PR-body failure into a ceiling-only warning, so undeclared growth passed on a transient error; and `size / from - 1 > 0.05` rejected exactly 5 percent (2,720 to 2,856 bytes computes 0.050000000000000044).

## What Didn't Work
A fail-open catch "so a GitHub outage never blocks a PR" - it widened the spec's one sanctioned skip (no base exists) into any error.

## Solution
Page through runs until an ancestor is found; stop at the first ancestor whose package expired (retention is by age). Only a confirmed absent base warns and skips; every other failure fails the job. The share is an integer `growthPercent` compared as `100 * (size - from) > growthPercent * from`.

## Prevention
For any threshold, test exactly-at and one-over. For any skip path, name the one condition that permits it and let everything else throw.
