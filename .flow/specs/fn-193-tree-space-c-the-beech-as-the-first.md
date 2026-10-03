# Tree space C: the beech as the first point

## Goal & Context

The first species in the tree space: a mature open-grown European beech that reads as a beech, built from sourced architecture data on the engine of A and B. Today's beech is the baseline to beat; the owner judged it poor. [user]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## Acceptance Criteria

- **R1:** Sourced architecture for the beech: Troll's model in *Fagus* (LITERATURE.md; Millet 1998, Nicolini, Letort's GreenLab beech), every value cited; where mature open-grown data is missing, that is stated and the value marked as estimated. [user, checked]
- **R2:** The beech as a point (values only) at several ages and mature, seeds 1 and 7, bare and in leaf, beside `.flow/references/european-beech/` and today's beech (pinned baseline). [user]
- **R3:** The gate: the host is confident the mature sheet reads as a beech beside the photographs, and Astra independently agrees on the same sheet; each verdict names the traits it judged. [user]
- **R4:** Time, fine wood and memory reported against the pinned baseline. [user]
- **Wrong-path stop:** two rejections (host or Astra) naming the same trait after a change aimed at it. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
