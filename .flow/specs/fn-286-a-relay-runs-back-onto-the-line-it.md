# A relay runs back onto the line it takes over

## Goal & Context

The owner's species review (2026-10-15) on the Oregon white oak at seed 7: "Lower branches that grow to long and zigzag weirdly." The cause (fn-236 `CAUSE.md`, branch `fn-236-oak-seed7`, 8d358fcb2): a relay at blend 1 heads where a lateral bud would, at its age's full insertion angle (`geometry.rs:212-226`, `turned`), and nothing turns it back towards the line of the axis it replaces; the oak's ordinary branches turn 49-57° at each relay against their 57° insertion. Switching relays off removed every turn of 40° or more on every seed and the hook from the still, at a fifth of the leaves. In a sympodial axis the relay takes over its parent's course: it leaves at an angle and straightens into the line as it grows (Troll's take-over), so the axis reads as one tortuous limb, not a zigzag.

The owner (2026-10-15), on whether the six species with relays (beech, birch, ordinary, Telperion, Laurelin, oak) may change: "I don't need them byte identical. If they improve that sounds good?" A general change is wanted; each changed species is judged on its stills.

## Requirements

- **R1 (the mechanism, by degree):** a new per-PA row, `takeover`, how far a relay's course runs back onto the line it carries on (the parent's running direction at its tip) as it grows, over the same scaled length its straightening fades over (`geometry::reaches`); 0 is today's relay exactly (to the bit), 1 runs fully onto the line. Dormant without relays; no species branch. It composes with straightening and erection as `bend` does; the CPU, the device cycle and the Wasm entries draw the same tree (the existing agreement tests).
- **R2 (values):** each species with relays gets a value read from its stills: the oak's ordinary branches with no turn of 45° or more left at seeds 1-50 at 40 that is not a main limb's designed kink (`oak.rs:196-201`); the others where it improves their limbs, else 0. Catalogue text, the parameter reference and the preset table regenerated; catalogue digests re-pinned with a history comment.
- **R3 (proof):** fn-236's seed-7 measures (`seed7/oak_lines.rs`) before and after on seeds 1-50; stills of every changed species at its preset age, seeds 1, 7, 42, beside today's, viewed; build time and node count per changed preset against today's; gate and npm test green; the owner's verdict per changed species.

## Boundaries

Release item (fn-237). The length of the oak's low limbs is fn-236's (a sourced sapling segment); low branches dying under the crown is fn-282, after the release.

## The owner's verdict (2026-10-15)

On the sheets of all six changed species (`sheets/`, today beside new at seeds 1, 7, 42; take-over 0.75 oak and ordinary, 0.5 beech, birch, Telperion, Laurelin; the oak's sapling segment 0.031 m from Hibbs & Yoder 1993): "definitely better". Approved, pending the gate (the device sizing refusal and the within-year presentation test, both being fixed).
