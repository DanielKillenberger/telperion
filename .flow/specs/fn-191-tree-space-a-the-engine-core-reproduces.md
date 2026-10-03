# Tree space A: the engine core reproduces GreenLab

## Goal & Context

Build the tree space's growth engine core and prove it computes the botany correctly against oracles, before anything new is built on it. [user]

The core: buds carrying a physiological age (PA) that sets their growth and branching; the PA's sequence along the species' reference axis (AmapSim's term); annual growth units of phytomers with each lateral's PA set by its position on the parent; death and pruning; minimal geometry (lengths, angles). It lives in a new crate, written to product standard (typed Rust, tests, files under about 400 lines), not wired into the pipeline until phase F. The fn-190 probe (`crates/telperion-render/examples/growth_law/`, branch fn-190) is frozen and nothing is copied from it. [user, inferred]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## Acceptance Criteria

- **R1:** A new crate holds the engine core: buds with a physiological age, a reference-axis automaton, growth units of phytomers with position-dependent lateral PA, death and pruning, minimal geometry. Errors: an input the engine cannot draw is refused by name. [user]
- **R2:** GreenLab's closed-form counts (structural factorisation; de Reffye et al. 2021, `greenlab2021.md`) are reproduced exactly: organ counts per PA and per cycle equal the formula for at least three parameter sets in a test. [checked]
- **R3:** Véronique Letort's GreenLab simulator (`github.com/VeroniqueLC/PlantStructureFactoryStochastic`, live at `veroniquelc.github.io/PlantStructureFactoryStochastic`, and its *Acer* variant), run unchanged in a headless browser, is the stochastic oracle: for the same parameters, our structures match its outputs in distribution (means and spread of counts per PA and age over many seeds) and deterministic runs match exactly; the comparison is a test with its fixtures recorded. The repository has no licence: it is run and compared against, never copied. [checked]
- **R4:** The whole-tree 3D reference for later phases is chosen with reasons recorded: L-Py's shipped models (`openalea/lpy`), Pałubicki 2009's figures, or another runnable source. [inferred]
- **R5:** A side-by-side sheet of our structures against the oracle's, viewed by the host. Time per structure measured and reported. [user]
- **Wrong-path stop:** the oracle cannot be run, or cannot be matched after three recorded attempts. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
