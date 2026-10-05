# Leaf-base clothing runs in expansion

## Goal & Context

The date palm's persistent leaf bases (the diamond lattice) are clothed by `branching::clothe_leaf_bases`, which runs inside the pipeline's skeleton stage (`pipeline.rs:166-174`, checked by fn-196's worker). A tree from the tree-space engine enters the pipeline at expansion (`executor::expand`), after that stage, so its leaf bases are never clothed; fn-196's palm cannot meet today's palm without them. The principle is one pipeline with no stage run outside it (`docs/principles.md`), so the step moves to where every tree passes (host, 2026-10-05).

## Design (host, 2026-10-05)

- Leaf-base clothing moves from the end of the skeleton stage to the start of expansion. For the direct build the input it reads is the same solved skeleton, so every shipped preset's artifacts are expected byte-identical; this is checked, not assumed.
- Nothing is re-exported and no second entry is added: a handed-in tree reaches the step by entering expansion, as it already does.

## Requirements

- **R1:** The move, with every shipped preset's artifacts byte-identical (the existing catalogue and pipeline tests, plus a recorded digest comparison for the date palm before and after).
- **R2:** A test that a tree handed to `executor::expand` with leaf-base rows set comes out with its leaf bases clothed, red first on the base.
- **R3:** Workspace gate; Codex review.

## Boundaries

The step's position only; its behaviour is unchanged.
