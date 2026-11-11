# fn-346: the host's answers to OLDTREE.md, 11 Nov 2026

Each is the host's design decision; none is the owner's ruling.

## What the measurement settles

- **Question 1: yes.** The balance decides on a whole lateral subtree with one number, and a shed takes every apex in it (`contract/shed.rs`). 66 of 78 shed boughs and 3 of 4 shed limbs on seed 1001 (176 of 198 and 8 of 13 on 1007) had at least one paying child, and about a third of a shed line's children were paying. The largest case was shed at a balance of −0.004 with 27,222 tips and 9 of 12 boughs paying. That is a defect of the unit of decision, not of the charge: a limb a hair under break-even is dropped whole, although shedding only what does not pay would leave it well above.
- **Question 2: both.** A limb whose tip has stopped lays no nodes, so its fine wood dies of age within a few years and it ends by abortion with 3 to 5 tips. That is the renewal gap again, on one limb.
- **Question 4: yes, renewal near a loss is part of this goal.** Rule B could not show what it does, because buds died with bearers that were shed whole.
- **Question 3: the load is out of scale first.** Limbs 21 to 52 m long are not an oak's; sag then does what sag does. Girth and sag stay untouched in this probe. The reach is looked at again once limbs are no longer lost whole.
- **Question 5: yes,** a same-session pair.

## The rule: the balance counts what would remain

A part is shed only when it would not pay after everything inside it that does not pay has gone.

For every lateral subtree, in place of today's sums:

    Q* = own light  + the sum of Q* over its child laterals that pay (Q* >= C*)
    C* = own charge + the sum of C* over those same children
    balance = (Q* - C*) / (Q* + C*)        and 0 where both are 0

- "Own" is what the subtree holds outside its child laterals: the tips on its own line, as today's sums take them.
- A child that does not pay adds nothing to its bearer, light or charge: it is judged alone, by its own balance, and shed by the existing hazard.
- The memory, the hazard, `kept`, tolerance and the idle rule read this balance exactly as they read today's. Nothing else changes.
- The charge is Q's: size × path / `L_max`, `L_max` twice the potential height, dead wood nothing, the main line tested as a lateral is.

This is the measured cause and no variation of an earlier rule: it changes which unit a decision is taken on, where P, A and B changed what was charged or what was born.

## The run

Variants: **QR** (Q with the rule above) and **QRB** (QR with rule B as built: sleeping buds on the limb, bough and branch rows, never ageing, waking at the existing rate × max(0, 1 − cost / light)). Rule A is not used. Probe kept as `probes/remain.patch`; the crates end as they began.

1. **Census to 300,** oak seeds 1001 and 1007, QR and QRB, stopping itself as before: height, spread, held phytomers, living tips by row, framework axes held, lowest living limb, leaves, income and charge, births over deaths, buds held and woken, lines shed by row with the share that had a paying child, every 10 years. A variant that is stopped is reported with its curve.
2. **Stills,** leafless and leafed, today's beside Q, QR and QRB: the oak at 40, 80, 120, 200 and 300 on seed 1001, and at 120, 200 and 300 on seed 1007 (Q's stills from the last round are reused where they exist).
3. **The other presets** under QR and QRB at each one's preset age and the beech at 80: one sheet beside today's, with the measures.
4. **Cost, measured as a pair in one session:** today's build and QR, one thread, the oak at 40, 80, 120, 200 and 300 on seed 1001, with peak memory.
5. **Each fork limb's shape** under QR at 160, 200, 240 and 300 as in OLDTREE.md (length, reach, tip height against base, lowest point), so the reach can be read.
6. **Your plain read per tree and variant:** better, the same or worse than today's, and what is wrong where it is worse. Look in particular at whether limbs are still lost whole, whether the crown is clothed through its depth at 200 and 300, whether a gap refills, how far the limbs run and whether they lie on the ground, and whether the oak at 40, 80 and 120 has moved from Q's.

About 2.5 hours. Result as `REMAIN-RESULT.md` with the sheets. Return then. The host views every sheet before anything else happens.
