# Tree space: wander bends over a length

## Goal

The owner judged the oak (fn-195, round 8) "structurally ok but very blob like and unnatural in subtle ways. Branches are a bit chaotic ... the reference looks calmer and more natural" (owner, 2026-10-05). Cause, checked in code: `form.wander` drew an independent turn at every node (`geometry/lay.rs`), a memoryless random walk that can only jitter at internode scale, never bend slowly. A real oak's limbs make a few large, slow turns while fine wood stays even; raising wander for "tortuous" limbs made every order jittery. The beech's "fewer, heavier limbs" gap may share the cause (host, 2026-10-05). Geometry only; the oak's values stay fn-195's.

## Decisions

- Design (host, 2026-10-05): each axis carries a curvature vector, a lineage-keyed mean-reverting process relaxing over a correlation length; the turn is curvature × internode. `form.wander` stays the amplitude; new `form.bend_length` (m) is the correlation length, 0 reproducing today's per-node draw exactly. A lateral starts with zero curvature; a continuation or relay carries its bearer's on.

## Requirements

- **R1:** `bend_length` neutral at 0, every existing tree byte-identical — met for beech, spruce, oak at seeds 1 and 7 (the palm was not on this branch; its check falls to the palm's branch).
- **R2:** At a long `bend_length` successive turns correlate as expected with the neutral spread; red first on the per-node draw — met (`tests/bend.rs`: lag-one correlation −0.011 red vs exp(−0.1/2) = 0.951; spread within 10% of wander²/3).
- **R3:** Walks on `bend_length` and `wander` within bound 30 — met (steepest 0.16).
- **R4:** The oak re-rendered with slow limb bends and low fine-wood wander, viewed by the host beside round 8 and S1 — met, recorded in fn-195 ("fn-207 R4 trial").
- **R5:** Workspace gate; Codex review — met (1,063 passed; Codex SHIP on review 2).

## Result

`Form::bend_length` (neutral 0, max 1,000): curvature relaxes by exp(−internode / bend_length) and is kicked by the keyed draw scaled by √(1 − keep²). Codex review 1's two P1s fixed (curvature updated at bend length 0; relaxation runs whenever bend length > 0, without kick at zero wander), with two red-first walks across a continuation.

## Open

R1's palm check on the palm's branch.

## Friction

- Gate ran before Codex review, whose fixes forced a second 9-min gate -> on tasks needing both, review before the gate.
