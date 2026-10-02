---
title: "Judge a generator change by its mechanism on every preset, not one tree's look"
date: "2026-10-02"
track: knowledge
category: decisions
module: pipeline/branching
tags: [verification, catalogue, owner]
applies_when: proposing or shipping any change to how trees are generated
decision_status: accepted
---

A generator change is judged by its mechanism, measured on every catalogue preset, never by one tree's look.

fn-188 measured deleting the scaffold tip's continuation shoot on the beech only, and the host claimed it "helps every tree". Across the catalogue it stripped the whole twig layer from telperion at seed 7 (18,140 nodes and 128,852 leaves to 363 and none) and 25 to 41% of the leaves from ordinary, laurelin and telperion at seed 1: the shoot was compensating for the twig layer's thickness gate. Earlier the same day an unchecked spec claim (removing inner twigs thickens limbs) and an experiment framed through one hypothesis (the horns as missing twigs) each cost a round.

Owner (2026-10-02): "figure out what it was and make sure that we have the elegant right solution. Don't just accept looks good without understanding why."

How to apply: before proposing or shipping a generator change, trace why it works (file:line), run it on every catalogue preset at two seeds with node and leaf counts, and state what it depends on. A beech-only result is a hypothesis.
