# Tree space: one reference axis, every setting by degree

## Goal & Context

The programme's first claim is that every species is a point in one continuous space (`docs/tree-space.md`). Each species so far was written on its own list of physiological ages (the beech 9, the spruce 9, the oak 10, the palm 1; counts corrected by the R0 proposal, checked), with whole-number settings (lifespan, next age, zone count, node counts, buds per node) that step rather than slide. fn-196's worker found that a walk from the oak to the palm therefore has no midpoint (fn-196 RESULT.md, R3, checked against the code). Every species spec's R3 walk (fn-194, fn-195, fn-196) depends on this. AmapSim's reference axis is the botanical frame: one ordered chain of physiological ages that every axis travels along at a species' own rate (host, 2026-10-05).

## Design (host, 2026-10-05; to be refined by the proposal below)

- **One shared chain.** Every species is written on the same ordered list of N physiological ages. An age a species does not use is dormant (its laterals' probabilities 0, its lifespan 0 so it is passed through), and "next" is always the next age on the chain.
- **Every whole-number setting gets a continuous form.** Lifespan carries a fractional last unit (the share machinery of fn-202). Node counts take a continuous mean. Buds per node become the probability of each further bud. A zone count becomes a fixed number of zones per age, with unused zones at zero nodes. Each continuous form is neutral where it reproduces today's integer.
- **Structure walks judge bare stills.** The dressing (presets, organ counts) is not walked here; continuity of the dressing is fn-198's organ contract. A species-to-species walk is rendered with one dressing.
- A species is then exactly a vector of settings on the shared chain, and the midpoint of two species is their midpoint.

## Host decisions on the proposal (2026-10-05)

The R0 proposal is `.flow/evidence/fn-206-tree-space-one-reference-axis-every/PROPOSAL.md`: a shared chain of N = 15 ages and every setting's continuous form.

1. Each axis is keyed by its chain position. Byte identity is a preference, not a requirement (AGENTS.md, "Generator evolution"), so R1 is unchanged in look, with a pixel diff recorded.
2. A stop is a probability of continuing at the end of the lifespan (1 where an age moves on today, 0 where it stops): the abortion machinery if it expresses this exactly, else a `continuation` share. One mechanism.
3. Zones carry fixed roles: 0 bare base, 1 medial, 2 top whorl, 3 spare. Each species maps its zones onto them.
4. The oak's low fork and the beech's top fork stay separate ages.

5. (host, 2026-10-05) "Unchanged in look" means the same species law, not the same individual: re-keyed draws give each seed a new individual, accepted where the seed statistics match within spread (AGENTS.md, "Generator evolution").
6. (host, 2026-10-05) A lifespan between whole cycles ends with its last unit grown at the share f and the next age's first unit in the same cycle at 1 - f, as fn-202's woken buds' first units are; growth units are keyed by the cycle they grow in, not by the units counted; the abortion hazard counts the real time spent in the age; the closed form and `Living` follow, and dormant buds age through real lifespans.

7. (host, 2026-10-05) A species is built in canonical form: an age it does not use carries its nearest used age's settings, with no lifespan and no laterals.
8. (host, 2026-10-05) "Never moves on" is continuation 0 on a finite lifespan; every positive magnitude mixes on a log scale, angles, probabilities and shares linearly, from one scale table that every setting must appear in.
9. (host, 2026-10-05) The spruce's 0.14 m height change from continuous node bounds is accepted.

## Requirements

- **R0:** A proposal (`PROPOSAL.md`): every setting in `species.rs` listed, with its continuous form, its neutral value and its effect on the closed form; the chain length N and how the four species map onto it; the cost. Reviewed by the host before R1.
- **R1:** The shared chain and the continuous forms. The beech, spruce, oak (round 4) and palm re-written on the chain and unchanged in look: before-and-after stills at seeds 1 and 7, viewed, with a pixel diff recorded. The renumbering is its own commit, apart from the continuous forms, for the merge with phase E's work on `grow.rs` and `species.rs`.
- **R2:** The closed form and A's oracle still hold; walk tests for every new continuous form within the bound of 30, real-valued lifespan among them, red first on the count-keyed form.
- **R3:** Bare still strips walking beech → spruce → oak → palm at a fixed seed, one tree per process, viewed by the host: no visible pop. Then a before-and-after five-seed bare sheet for the beech: its passed `final21` seeds against the new individuals.
- **R4:** Workspace gate; Codex review.

## Boundaries

Structure only. Dressing continuity is fn-198's.
