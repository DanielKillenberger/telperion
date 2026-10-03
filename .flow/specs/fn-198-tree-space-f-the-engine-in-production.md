# Tree space F: the engine in production

## Goal & Context

The tree-space engine replaces stage 2 of the one pipeline for every species that passed C to E; today's generator keeps the rest until each passes. The PR carries every passed species for the owner's verdict. [user]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## Acceptance Criteria

- **R1:** The engine inside the pipeline's stage 2 for the passed presets, as values; the twig layer, its thickness gate, the tip shoot, girth patches and the foliage cull are removed for those presets' path (`docs/pipeline.md`). [user]
- **R2:** Growth time per passed preset within 1.5x of today's skeleton time at equal or greater fine wood, against the pinned baseline (BASELINE.md); full build no slower than today; memory reported. [user]
- **R3:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; the Codex review passes; every shipped artifact within its CI size budget. [AGENTS.md]
- **R4:** The PR (per `docs/pr-format.md`) shows every passed species' sheet beside today's and the photographs, for the owner's verdict; it is not merged without it. [user]
- **Wrong-path stop:** the speed bar missed after measured optimisation. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
