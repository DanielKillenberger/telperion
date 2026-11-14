---
title: "Every spec runs through /flow-next:flow, not hand-dispatched agents"
date: "2026-10-08"
track: knowledge
category: decisions
tags: [process, flow-next, review]
applies_when: "Every spec runs through /flow-next:flow, not hand-dispatched agents"
decision_status: accepted
---

Every spec is driven through `/flow-next:flow` (plan review, work through flow-next's workers, implementation review, completion), never by hand-dispatched general-purpose agents with host-written prompts and `flowctl start/done` alone.

Owner (2026-10-08): "yes it should have always been going through /flow-next:flow ..." after the host admitted that on 2026-10-07/08 it had captured specs and tasks with flowctl but dispatched its own agents, skipping plan review, impl review, plan-sync and the completion review.

Why: the skipped stages catch what gates miss. fn-226 started without the requirement "light inline in the growth step, no added pass", which a plan review would have asked for; one hand-run Astra review of fn-212 found nine real defects the flow's impl review would have found per task.

How to apply: start or resume any spec with `/flow-next:flow <spec-id>` (attended) or `/flow-next:flow --auto <spec-id>` for unattended loops; host decisions still escalate at the flow's human stops. Workers already running when this rule took effect finish their current task; the next task enters the flow.
