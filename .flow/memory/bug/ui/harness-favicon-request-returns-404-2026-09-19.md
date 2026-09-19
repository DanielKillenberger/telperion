---
title: Harness favicon request returns 404
date: "2026-09-19"
track: bug
category: ui
module: harness
tags: [qa, favicon, pre-existing]
problem_type: ui
symptoms: Harness favicon request returns 404
root_cause: (unspecified)
resolution_type: fix
---

## Problem
P2, confidence 100, pre_existing. A fresh browser loading the local harness logs a missing resource. The favicon endpoint returns HTTP 404; the scene still renders and controls work.
## Reproduction
Open http://127.0.0.1:42444/ in a fresh Chromium session, wait for scene draws, and inspect console. Repeated in two isolated sessions; direct GET /favicon.ico returns 404.
## Expected and actual
Expected clean runtime/resource console during QA. Actual: one 404 console entry, no JavaScript exceptions, no failed renderer module or application request.
## Evidence
.flow/tmp/qa-fn-42-bark-depth-level-and-a-fair-capture/live-console.log and oak-settled.png. No favicon or harness page asset was changed by fn-42.
## Resolution
Open cosmetic issue, no fix applied in this bark change.
