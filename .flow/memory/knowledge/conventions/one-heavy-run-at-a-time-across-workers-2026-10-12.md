---
title: One heavy run at a time across workers; 8 GB free first
date: "2026-10-12"
track: knowledge
category: conventions
tags: [memory, owner, workers]
applies_when: dispatching workers that build trees or run gates
decision_status: accepted
related_to: [knowledge/conventions/reset-review-round-caps-without-asking-2026-10-09]
---

The owner (2026-10-12), with the desktop lagging: "are you hogging my drive or smth>? pc lagging like crazy". Two workers ran full-tree CPU builds at once (oak 80 at 3 GB, Telperion toward 5 GB) beside another project's 7 GB process; free memory fell to 2 GB and the machine swapped.

**Why:** the owner works on this desktop; a parallel heavy run makes it unusable.

**How to apply:** every worker brief says: before a heavy run, check `free -g`; start only with at least 8 GB available and no other telperion example or test binary running; one tree at a time across all workers. The host runs at most one heavy worker phase at a time.
