---
title: Rank speed work by gain; byte identity is only a free verification aid
date: "2026-09-27"
track: knowledge
category: best-practices
module: crates/telperion-core
tags: [performance, byte-identity, generator-evolution]
applies_when: "Proposing, ranking or speccing a generator performance change"
---

Rank speed candidates by measured gain. Byte identity is not a ranking axis, a reason to split specs, or something that needs the owner's word. Pick the fastest design (prepared tables, cheaper approximations, analytic derivatives) and state its evidence plan: before/after timings, the suite's correctness checks, and the owner's visual check where the tree changes. Keep byte identity as a requirement only where it costs nothing and eases verification. This applies AGENTS.md "Generator evolution" (2026-09-20).

On 2026-09-27 the owner corrected a skeleton-speed investigation that labelled every fix "byte-identical" or "changes bytes", preferred exact variants with smaller gains, and carried over fn-124's "byte-changing options need the owner's word". The owner said: "i have said many times as we build the engine we don't need things byte identical."
