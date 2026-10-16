---
title: Reset review round caps without asking during autonomous runs
date: "2026-10-09"
track: knowledge
category: conventions
tags: [review, autonomy, owner]
applies_when: a review hits its round cap in an autonomous run
decision_status: accepted
---

The owner (2026-10-09), when the host stopped to ask before resetting a spec's plan-review round counter: "you are running this session autonomously just reset if needed".

**Why:** the round cap is a backstop against non-convergence, not an owner gate; asking stalls an autonomous run on a mechanical step.

**How to apply:** when plan-review or impl-review hits the cap and the last findings were fixed (each one specific and addressed), reset with `flowctl spec reset-review-rounds` and re-review without asking. Escalate only when findings keep moving or need a design decision.
