---
satisfies: [R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.3 A clean fork where a stem parts from a stem

## Description
At any positive `stem_fork_height` the fork showed a ledge. Owner (2026-09-15): "ok capture the follow up task then pls". The trunk run carries on into the straighter stem and eases its girth into it; the stem-or-limb fact must survive every read (`specimen/history.rs`, `specimen/interval.rs`, `specimen/view.rs`) and the contact query (`specimen/contacts.rs`) must choose the same run.

## Acceptance
- [x] R1 no ledge, no seam on a synthetic clump and the birch.
- [x] R2 stem-or-limb fact survives full, shoot-less, sparse interval and browser view reads; `sparse_spruce_projects_only_changed_contact_paths` holds.
- [x] R3 contact query chooses the surface's run.
- [x] R4 presets byte-identical at fork height 0; birch fork height set, pairs rendered, visual QA; owner verdict in fn-34.

## Done summary
The fork is swept clean; specimen snapshot moved to schema 2 for the stem flag. Birch states 0.5 (fork node 0.96 m), the rail's top; S-WHOLE and S-BARE disagree on fork height. Round 20 (`round20-fn48c` rows in `.flow/evidence/fn34/rounds.tsv`), 48/48, none capped; merged into fn-34-integration (64a3aea4). The owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Evidence
- Commits: 55dc2ddc
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
