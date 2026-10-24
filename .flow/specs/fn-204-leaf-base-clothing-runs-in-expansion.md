# Leaf-base clothing runs in expansion

## Goal

The date palm's persistent leaf bases are clothed by `branching::clothe_leaf_bases` inside the pipeline's skeleton stage. A tree from the tree-space engine enters the pipeline at expansion (`executor::expand`), after that stage, so its leaf bases were never clothed and fn-196's palm could not meet today's palm. The principle is one pipeline with no stage run outside it (`docs/principles.md`), so the step moves to where every tree passes (host, 2026-10-05). The step's position only; its behaviour unchanged.

## Decisions

- Design (host, 2026-10-05): clothing moves from the end of the skeleton stage to the start of expansion; nothing re-exported, no second entry.
- Host, 2026-10-05 (Codex P2): a tree already ending in exactly the bases its table hangs is left as it is, so a clothed tree handed back to `expand` is not clothed twice.

## Acceptance Criteria

- **R1:** The move, with every shipped preset's artifacts byte-identical and a recorded digest comparison — met.
- **R2:** A tree handed to `executor::expand` with leaf-base rows comes out clothed, red first — met.
- **R3:** Workspace gate; Codex review — met (1,044 passed; Codex SHIP).

## Result

`pipeline::clothe` runs after the skeleton in `pipeline::build` and in `Grown::expansion`. All eight shipped presets' artifact digests (`crates/telperion-core/examples/artifact_digest.rs`, sha256 of debug text) were identical before (`140ffc61`) and after, and again after the P2 fix; date palm `8d7a61d2bdc8c9d2`. `a_clothed_tree_is_not_clothed_again` was red first (1,064 nodes against 552).

## Open

None.

## Friction

- `cargo fmt -p telperion-core` reformatted 19 unrelated files (crate not rustfmt-clean) -> fmt-clean tree or a CI fmt check; rustfmt touched files only.
- telperion-core's fixed-beech memory ceiling failed once under machine load -> shared-machine pressure, nothing in the repository.
