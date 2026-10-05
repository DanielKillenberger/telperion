# Tree space E: light and the carbon balance

## Goal & Context

Feedback on top of the passed species: light intercepted from above and a carbon balance shape density, the clear bole and size. The 2026-10-03 probe (fn-190 R1 round 4) showed this can form a bole by shedding and bound size; here it is reproduced first against a published light model, then added by degree. [user, checked]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## Design (host, 2026-10-05)

The options and the reasons are in `.flow/evidence/fn-197-tree-space-e-light-and-the-carbon/DESIGN-OPTIONS.md`, with these decisions in its section 8. [host]

- **Light:** Beer–Lambert through a coarse leaf-area lattice anchored at the world origin.
  - Deposits and reads use trilinear weights.
  - The field is swept once per cycle along a few sky directions, and every bud reads the previous cycle's field.
  - Oracles: the leaf slab exp(−k·LAI), a uniform-density sphere (exp(−k·ρ·chord)), and GreenLab's production formula Q = Sp·(1 − e^(−k·S/Sp)) where it applies. [host]
- **Reading:** a rough layout built as the tree grows (P1), and a full lay with sag every K cycles (K an engine constant chosen by measured cost). Light reads the latest full lay plus the rough layout of growth since then. [host, decision 8]
- **Sky:** one continuous site setting, `sky`, from overhead-only (0) to a uniform overcast sky (1), with the standard overcast sky as the reference. Every species is judged under one value, chosen at step 3 on the oak and recorded with its reason. [host]
- **Stop:** ln(survival) = ln(viability) · I^(−φ), neutral φ = 0. The shed hazard rises continuously as the remembered balance falls.
  - Both are walk-tested.
  - A red-first test pits the multiplicative form viability · I^φ against the walk bound. [host]
- **Fill by light (Borchert–Honda):**
  - Each cycle a basipetal pass sums the light each subtree collects. An acropetal pass splits the base's vigour at every branching point by per-PA apical control λ (Pałubicki 2009; 0.5 is the unbiased split that gives each bud its own light).
  - A bud's unit is sized by its vigour per presence against the tree's presence-weighted mean, to ψ, normalised exactly so the mean unit is whole. The size is the unit's own (internodes and girth) and never passes to what it bears.
  - ψ = 0 is neutral. α cancels and is deleted. The balance bounds size and acts only through shedding; there is no phototropism. [host, decisions 4, 11, 14, 15]
- **Shedding on the balance:** a lateral subtree's balance, (light − upkeep) / (light + upkeep), is remembered with an engine constant `MEMORY` = 0.5 (estimated, unsourced). A yearly shed hazard rises continuously below the PA's tolerance. A limb that dies on it is dropped after its PA's shedding delay, the persistence of dead branches. [host, decisions 5, 18, 20]
- **Apical control** is walked within Pałubicki's published 0.45 to 0.55; the trunk and leader PAs may sit higher (about 0.55 to 0.6) so the leader keeps its share. [host, decisions 17, 21]
- **Walk scope:** the gate walks the light settings and a sample of structural ones in leaf with the full lay; every setting in leaf is a slow suite outside the gate. [host, decision 12]
- **Calibration:** leaf area and k stay plausible on this branch; the oak's leaf area is sourced on its own branch. [host, decision 16]
- **Girth:** `retained`, the share of a shed branch's pipe kept in its bearer (Shinozaki's disused pipes), neutral 0, built in step 5 beside thickening from leaves. [host]
- **Walks:** refine depth 6 for every walk; a jump does not shrink under refinement, a steep crossing does. [host, decision 9]
- **Foliage:** sag's foliage term and light's `leaf_area` are two estimates of one thing, kept apart for neutrality and unified per species at R3. [host, decision 10]
- **Neutral:** every new setting at neutral leaves the beech, the spruce and the oak byte-identical. [host]
- **Timing:** the measures example reports growth, rough layout, light sweep and final lay per stage. [host]
- **Steps:**
  1. The grid and its oracle tests.
  2. The rough layout at neutral, measured and timed.
  3. φ on the oak, and the walks on `sky`, k and φ.
  4. The balance and the shed draw.
  5. Girth from leaves, and `retained`. [host]

## Acceptance Criteria

- **R1:** A light pass (Beer-Lambert from above, or Pałubicki 2009's shadow propagation, chosen with reasons) reproduced against its published description, then added as a setting whose neutral value leaves every passed species unchanged. [user]
- **R2:** Shedding on each branch's remembered balance, and growth bounded by it. Light must not widen the seed-to-seed spread, in leaf count, compared with the same species at neutral light. [host, decision 19, 2026-10-05: the earlier "seeds within 1.5x in node count" was inferred, and round 4 itself spreads about 4x in leaves]
- **R3:** The gate: every passed species re-rendered and judged as in phase C, none looks worse; walks stay continuous. [user]
- **Wrong-path stop:** a passed species regresses and cannot be restored. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
