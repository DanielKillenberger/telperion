# A limb holds its angle where its reaction wood can

## Conversation Evidence

> goal, owner, on the full oak at 200: "yea that's bad"
> goal, owner, 3 Nov 2026: "But then are we able to realistically grow 300 year old trees? That's really the goal here."
> goal R4: "At 200 and at 300 the crown's mean sits within 3 m of the trunk's base on the median seed."
> goal R4: "Each rule added for this is a rule of the one cycle for every species, its rows sourced, with what it does to every preset at its preset age measured and shown."
> fn-337's sourced note, quoting Alméras and Fournier 2009: "without this control process, the increasing mass of growing trees would make the stem bend more and more ... trees would adopt a weeping habit"; "At the critical steady limit Pg=1, the tree maintains its angle when it grows."

## Goal & Context

Child of goal fn-321, serving R4. It replaces fn-337, whose rule its own sources refuted before any code: bending each year's added load at that year's girth gives 10 to 18 times today's bend on a self-similar limb, since a radius only grows (fn-337 R1-NOTE.md). The same sources name what holds a real limb up: reaction wood, whose maturation strain turns the limb back as it thickens. Alméras and Fournier (2009) measure it as gravitropic performance, the curvature the reaction can give over the curvature the added weight gives, and a limb at performance 1 or more keeps its angle as it grows. [paraphrase]

The engine has the weight's half and not the reaction's. Its sag bends every phytomer by its whole moment over its radius to the fourth, at the settle, from the finished tree (`sag.rs`, `grow/settle.rs`). On the old oak that moment outgrows the wood: the crown's mean sits 11.91 m off the base at 200 and 12.47 m at 300 on the median seed (fn-337 BEFORE.md). [paraphrase]

Premise, not yet run: the reaction's capacity can be read from a phytomer's present state. If the weight's curvature goes as the moment over the radius to the fourth, and the reaction's as the wood's growth over the radius squared, their ratio over a limb's life goes as the radius cubed over the moment, with the limb's age cancelling. Then the bend is the sag of the moment left over once the wood has held what it can, a share of its radius cubed, and nothing where the wood holds it all. That reads only what the settle already holds, keeps no yearly state, and leaves a tree at any age the same however it was reached. Whether Alméras and Fournier's expression supports this reading, and what sets the share, is this child's first question. [inferred]

## Acceptance Criteria

- **R1 (sourced first, and back to the host):** a note in evidence gives Alméras and Fournier's gravitropic performance as their text states it (its expression, its terms, the values they report for real trees and any they give by species or wood type), each claim checked against the text, and says whether the premise's reading follows from it, under which assumptions, and what it leaves out. It also lists what the engine already has that acts as posture control (`straightening`, `erection`, `tropism`, with what each reads). No rule is built before the host answers this note. [inferred]
- **R2 (a probe at the settle, before any product rule):** with the host's answer, a probe changes only how the settle reads a phytomer's moment, and grows the oak at 40, 80, 120, 200 and 300 on seeds 1, 7 and 42 across a stated sweep of the reaction's share: fn-332's measures for each, beside fn-337's BEFORE.md, and stills of 120, 200 and 300 on one retained sheet. The sweep's values and what counts as holding are written down before the first tree grows. [inferred]
- **R3 (every preset under the probe):** every preset at its preset age, seeds 1, 7 and 42, at the sweep's values: height, spread, trunk and the whole-tree still on retained sheets, the birch's hanging twigs and the spruce's low branches named in the notes, since a rule that holds every limb up would take a weeping habit away from a species that has one. [inferred]
- **R4 (the proposal, the host's to accept):** the result ends with what the probe shows: whether the reaction's share can be one sourced value for every species or needs a row with sourced values, what remains of the old oak's bend, and what it costs the cycle. The product rule is built only on the host's word, in this child if it is small, with the gate green and the codex review SHIP. [inferred]
- **R5 (a tree at any age):** under the probe and under any rule kept, a tree grown to an age in one call equals one paused and resumed on the way, and with sag at nothing every tree is unchanged to the bit. [paraphrase]

## Boundaries

- No value fitted per species. A row is allowed only as a botanical trait with sourced values, and only on the host's word after R4. [paraphrase]
- This child does not make limbs share the crown and does not limit height; fn-332 found the crown 7.9 m off the base with sag off entirely, so the goal's 3 m is not expected here.
- Nothing merges to master, nothing is tagged or published. Heavy work runs through `telperion-heavy`.

## Decision Context

Settled by the goal: one grower; a rule is general and sourced; the host conducts and an Opus worker builds, stopping at any question about how the system should work. fn-337's evidence stays in use: its R1-NOTE.md (the sources read so far), its DESIGN-FINDINGS.md (where state could live, with option C, nothing kept, the one this premise needs) and its BEFORE.md (the old oak before any change). [paraphrase]

The engine's `sag` value is far larger than an elastic beam's (the host's arithmetic, unchecked: a wood of 800 kg/m3 and 10 GPa gives about a millionth per metre against the oak's 0.0003), so it already stands for a limb's long droop, not its spring. That is why the reaction belongs beside it. [inferred]

## Open Questions

None.
