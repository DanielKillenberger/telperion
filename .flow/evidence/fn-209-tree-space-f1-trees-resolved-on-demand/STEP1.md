# fn-209 step 1: the envelope built, and three design questions (worker, 2026-10-05)

A dispatched worker wrote this report. Step 1 is partly built; it stops at the design questions in section 3, under the host's rule "any design question: stop and report". One of them corrects a claim in PROPOSAL.md.

## 1. What is built (`787db420`)

- **Where every event stands.** The closed form's events moved to `closed_form/events.rs`, and each event now carries where it stands along its axis (`closed_form/along.rs`):
  - zones follow the order `ALONG` gives;
  - a place-0 bud stands at the rank its own draw gives it, since that draw also orders the nodes (`grow/unit.rs:363-369`): rank 1 + Binomial(N − 1, 1 − u);
  - every other place stands at an even rank;
  - a sleeping bud stands half an internode below its node;
  - a relay stands at its PA's `relay_at` share of the unit it stands on (the unit before a failed one);
  - a continuation stands at the tip.

  Each event also carries its insertion angle, place shares and dominance moments.
- **The envelope** (`closed_form/envelope.rs`, `StandIns::of(pa, cycles left, carried, first)`). It gives Σ l, Σ l (x · heading) and Σ l |x|² of a bud's expected wood about its base, per unit base scale. From these come the centre along the heading and the root-mean-square reach. Axes are taken as straight and azimuths as even.
- **Counts and lengths are unchanged to the bit.** Every species at 80 years, seeds 1 and 7, neutral and lit, hashes exactly as F2 recorded (`measures hash`): beech `f1eede48`/`9eb6e2a8`, spruce `109d37db`/`0ffab055`, oak `905a271a`/`573bed1e`, lit `ebad1976`/`0ec24cfd`, palm `d2cd1637`/`c3dee328`.
- **Tests:** the `telperion-space` suite passes, all binaries.

**Oracle results:**
- **Deterministic tree:** exact to 1e-9. The tree has opposite pairs, a trunk relayed in line at half its unit, and a continuation into a zoned leader.
- **Stochastic tree:** 600 seeds in all. The tree has Poisson and uniform node laws, 1.5 and 2.5 bud places, a lifespan of 3.5, abortion with relays, failed units with relays, sleeping buds and even roll.
  - Passes within 4.5 standard errors per group of 300 or more subtrees.
  - **There is a consistent shortfall of 2 to 6 %**: the realized subtree is smaller (question C).

**Not built:** the expected section, the mass, and the extinction curve. All three wait on questions A and B.

## 2. The envelope against the production species (measured, not a test)

The setup: beech and spruce at 30 years, three seeds. The geometry was straightened: tropism, wander, sag, straightening, erection and epitony all 0, dominance 0, roll even. Each kept lateral subtree, by PA and birth, is compared against its envelope:

| Group | Realized wood / expected | Centre, realized vs expected | Reach, realized vs expected |
|---|--:|--:|--:|
| Spruce PA 12 (twigs), 1 to 5 years | 0.997 to 1.17 | within 0.001 m | within 0.001 m |
| Spruce PA 10 (branchlets), 1 to 6 years | 1.00 to 1.01 | within 0.001 m | within 0.001 m |
| Spruce PA 10, 7 to 21 years | **0.22 to 0.86** | 0.37 vs 0.30 m | 0.42 vs 0.35 m |
| Beech PA 12, 1 to 5 years | 1.00 to 1.05 | within 0.001 m | within 0.002 m |
| Beech PA 14, 2 to 4 years | **1.11 to 1.21** | within 0.001 m | within 0.001 m |

- **Young subtrees: the envelope holds.** Centre and reach are within a few millimetres.
- **The large departures are shedding,** which the closed form leaves out (`closed_form.rs:13`).
  - An old spruce branchlet has shed its inner twigs: it keeps 22 to 60 % of the wood it grew, and that wood stands farther out.
  - The beech's young twigs read **more** than expected because only the kept ones are counted. A subtree that died and was shed is missing from the mean (survivorship).
- So every **kept** quantity a stand-in carries (wood, envelope, leaves, section, mass) needs shedding in the closed form, not only the fade (question B).

## 3. Design questions

### A. One-level look-ahead does not make junctions continuous (it corrects PROPOSAL.md)

PROPOSAL.md says a parent and child "sum the same thing", so the joint is continuous. That is wrong. Let parent P bear child C, which bears G:
- For P to count C's base section without growing G, C's girth must count G as the closed form expects it, E_G. C's base is then own_C + Σ E_G, which P can sum.
- But G, once drawn, sums its own chain exactly: own_G + Σ E_H, which is not E_G.

So every drawn junction meets one level of mismatch. A finite look-ahead cannot close it: making C count G exactly makes P need G's chain, and so on down.

**The options:**

| Option | What | Cost | Look |
|---|---|---|---|
| **(1) Exact topology, lazy geometry** | Grow the whole tree's topology at once (no geometry; growth reads no girth). Then girth, retained pipes and fades are today's, exactly. Sag needs today's unbent lay as well; otherwise it is a stand-in moment, which is not drawn and so needs no consistency. Only the lay and the surfacing are lazy. | One-core growth plus settle: beech 1.0 s, spruce 3.6 s, palm 0. 32 cores: about 0.5 to 1.5 s (F2's growth column). The oak's coupling pass is unchanged. | Beech, spruce and palm unchanged to the bit if sag stays exact. Streamed equals one-pass trivially. |
| **(2) Coupling tree exact, fine subtrees resolved whole** | Chains above the coupling scale get today's exact girth among themselves, and the expected section (closed form) for each fine root. A fine subtree (under 0.5 m) is resolved whole and is exact inside. Its radii are scaled once, at its root, to the section the coupling tree reserved for it: one factor, so no drift down the tree. | Needs section, mass and extinction in the closed form (question B). A fine subtree is about 70 phytomers on average (oak), 50 to 500 µs. | One scale factor per fine subtree. Its spread is about the junction spread measured in R0: beech p5 0.33 to p95 1.34, spruce 0.71 to 1.20, oak 0.09 to 2.07 in radius, against the mean of its kind (a proxy for the expectation). |
| (3) Accept the mismatch | Each chain draws its own sum, and its parent counts the expectation. | As (2). | A junction off by the same spread, at every level. |
| (4) Rescale every chain to its parent's reservation | | | Drift: the factors multiply down the depth. Not recommended. |

- **The worker's lean, as data only:** (1) for beech, spruce and palm, which have no light coupling. Nothing about them changes, the cost is the growth pass F2 already sped up, and laziness still saves the lay, the surfacing and every chain off screen.
- **The catch:** (1) brings back a whole-tree pass before the first frame. Decision 2(b)'s frame 1 can still draw closed-form stand-ins (envelope and leaves) while that pass runs.
- **The oak** needs (2) or (3) in any case below its coupling tree.

### B. Shedding in the closed form is a deep change

Shedding is needed under (2) and (3), and for every kept quantity a stand-in draws (section 2).
- **What it takes:** shedding drops a lateral once its whole subtree has been idle past the delay (`shed.rs:314-426`). In expectation that is the chance a subtree still has a living apex at each time: a branching-process extinction recursion on probabilities. It runs through node-count laws (probability generating functions of the uniform and Poisson laws), bud places, relays, continuations and sleeping buds that wake on the bearer's chain (`dormant.rs:105-140`).
- **Its size (estimate):** a second recursion beside the counts, about 300 to 500 lines, with its own oracle.
- **Then the kept quantities:** they also need the fade's presence-weighted living time (`shed.rs:382-416`), which is not a plain expectation.
- **Under (1),** none of this is needed for beech, spruce and palm. Frame 1's stand-ins could draw grown wood, conditioned only by F2's certain-shed bound (`shed.rs:270-303`).

### C. The closed form counts drawn elements whole; trees grow some in part

The grower grows an element whose draw lies inside its presence window at a share (`presence.rs:118-134`). In the stochastic oracle the most frequent cases are nodes near a uniform law's bound (`lineage.rs:120-133`). The closed form counts every drawn element whole, as the counts always have.
- On the oracle species, the realized subtrees fall 2 to 6 % short.
- In production young subtrees, they are within about 1 %.
- **The options:**
  - Accept it and document it: the oracle tolerance stays statistical.
  - Make the closed form presence-aware: E[presence^p] for each draw from its window, in closed form. That means mirroring the grower's window logic inside the closed form, a second copy of that logic to keep in step.

### On decision 5, from reading the code

Light enters a unit's survival as viability^(light^−φ) (`grow/advance.rs:47-52`). Under a constant I, the closed form's survival per unit takes the same substitution: one term in `events`, which is shallow. Size by light (ψ) and the carbon balance's shedding are not shallow: the first needs r̄ and the second is shedding (question B).

## 4. What is next, once the questions are settled

- **Under (1):** step 2 is the chain grown alone, with today's girth, fade and sag from the global topology pass, and a chain-by-chain lay. The topology test is bit-identity, which section 1's hashes already show for the closed form.
- **Under (2) or (3):** first the section, mass and extinction tables, then step 2.
- **F2 (#158)** was still open when checked, so master is not merged into this branch.
