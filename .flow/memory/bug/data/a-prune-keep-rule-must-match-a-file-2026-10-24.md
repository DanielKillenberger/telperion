---
title: A prune keep rule must match a file name cited beside its spec
date: "2026-10-24"
track: bug
category: data
module: .flow/evidence
tags: [prune, evidence, flowctl]
problem_type: data
symptoms: A prune keep rule must match a file name cited beside its spec
root_cause: keep rule matched full paths only
resolution_type: fix
---

fn-312 pruned closed specs' task files and evidence. A keep rule that matched only full paths (`fn-188-…/R3-PROBE.md`) missed records that cite evidence the way specs usually do, by file name beside the spec number ("fn-188's R3-PROBE.md"); codex caught 27 such files after both prune commits had landed. A second miss: a kept task's dependencies were pruned, so `flowctl validate` lost them.

When pruning `.flow` records, keep a file when a live record names it by path or by its file name on a line that also names its spec (`fn-188`, `fn188`), keep a kept task's dependency chain, and keep the inputs of any kept script or JSON reader. Run the full candidate list against those rules before the first `git rm`.
