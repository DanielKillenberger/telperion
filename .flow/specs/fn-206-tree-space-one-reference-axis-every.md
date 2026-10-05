# Tree space: one reference axis, every setting by degree

## Goal & Context

The programme's first claim is that every species is a point in one continuous space (`docs/tree-space.md`). Each species so far was written on its own list of physiological ages (the beech 6, the spruce 6, the oak 10, the palm 1), with whole-number settings (lifespan, next age, zone count, node counts, buds per node) that step rather than slide. fn-196's worker found that a walk from the oak to the palm therefore has no midpoint (fn-196 RESULT.md, R3, checked against the code). Every species spec's R3 walk (fn-194, fn-195, fn-196) depends on this. AmapSim's reference axis is the botanical frame: one ordered chain of physiological ages that every axis travels along at a species' own rate (host, 2026-10-05).

## Design (host, 2026-10-05; to be refined by the proposal below)

- **One shared chain.** Every species is written on the same ordered list of N physiological ages. An age a species does not use is dormant (its laterals' probabilities 0, its lifespan 0 so it is passed through), and "next" is always the next age on the chain.
- **Every whole-number setting gets a continuous form.** Lifespan carries a fractional last unit (the share machinery of fn-202). Node counts take a continuous mean. Buds per node become the probability of each further bud. A zone count becomes a fixed number of zones per age, with unused zones at zero nodes. Each continuous form is neutral where it reproduces today's integer.
- **Structure walks judge bare stills.** The dressing (presets, organ counts) is not walked here; continuity of the dressing is fn-198's organ contract. A species-to-species walk is rendered with one dressing.
- A species is then exactly a vector of settings on the shared chain, and the midpoint of two species is their midpoint.

## Requirements

- **R0:** A proposal (`PROPOSAL.md`): every setting in `species.rs` listed, with its continuous form, its neutral value and its effect on the closed form; the chain length N and how the four species map onto it; the cost. Reviewed by the host before R1.
- **R1:** The shared chain and the continuous forms, every passed species unchanged in look (byte-identical where the continuous form is exactly neutral).
- **R2:** The closed form and A's oracle still hold; walk tests for every new continuous form within the bound of 30.
- **R3:** Bare still strips walking beech → spruce → oak → palm at a fixed seed, viewed by the host: no visible pop.
- **R4:** Workspace gate; Codex review.

## Boundaries

Structure only. Dressing continuity is fn-198's.
