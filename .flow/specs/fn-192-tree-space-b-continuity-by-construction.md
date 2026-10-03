# Tree space B: continuity by construction

## Goal & Context

Make the tree space continuous by design: walking any setting at a fixed seed changes the tree by degree, never with a pop. This is the programme's first genuinely new piece. [user]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## Acceptance Criteria

- **R1:** Every random draw is keyed to the bud's lineage (its path from the root), never to an index, so adding a branch anywhere reshuffles nothing else. [user, checked: the fn-190 probe keyed draws to node index and lateral parity]
- **R2:** Birth and death are gradual: when a setting crosses a seed's draw, the branch enters at vanishing length and radius and grows in; death and abortion fade out the same way. [user]
- **R3:** Discrete botany becomes continuous settings, each with its neutral value and dormant where its structure is absent: sympodial growth an abortion rate, rhythm a strength, Corner's unbranched stem branching readiness at zero, relay readiness and base straightening for Troll. [user]
- **R4:** A test walks every setting at fixed seeds; no step changes the shape measures (total length, silhouette, branch positions) by more than a stated multiple of the step. Phase A's oracle tests still pass at their exact points. [user]
- **R5:** Still strips of representative walks, viewed by the host. [user]
- **Wrong-path stop:** a jump that cannot be removed without breaking the oracle. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
