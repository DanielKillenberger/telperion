---
satisfies: [R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.3 A clean fork where a stem parts from a stem

## Description
fn-48.2 added `skeleton.habit.stem_fork_height`, so a clump's later stems
can leave the first stem above the ground. At any positive height the fork
shows a ledge: the trunk's full girth ends in a ring at the fork and the
narrower upright stem rises from inside it. The owner asked for it on
2026-09-15 ("ok capture the follow up task then pls") after the birch was
set back to parting at the ground.

The cause, from fn-48.2's worker: the surface's trunk run follows the
wider, leaning stem through the fork, and the upright stem is socketed at
only the girth the trunk can contain. A fix rendered clean in that task's
experiment (reverted, not committed): the trunk run carries on into the
straighter stem and eases its girth into it over the fork's diameter, and
the other stem leaves as the socketed one.

It reaches past fn-48.2's surfaces for two reasons, which are this task's
scope:
- Telling a stem from a limb needs the bud's fate, which the specimen's
  shoot-less reads drop: `specimen/history.rs`, the sparse interval trees in
  `specimen/interval.rs`, and the browser view tree rebuilt from `RunNode`
  in `specimen/view.rs`. In those trees the rule took every station for a
  fork and `sparse_spruce_projects_only_changed_contact_paths` failed. The
  stem-or-limb fact must survive into those trees (a flag on the node or
  the run, carried through every read), not be re-derived from fates.
- The contact query's run choice in `specimen/contacts.rs` must mirror the
  surface's, so a trunk run that follows the straighter stem is the same
  run for contacts.

## Acceptance
- [x] **R1** at a positive fork height the trunk run continues into the
      straighter stem and eases its girth into it over the fork's diameter;
      the other stem leaves as a socketed run; no ledge, no seam, on a
      synthetic clump and on the birch.
- [x] **R2** the stem-or-limb fact survives every read (full, shoot-less,
      sparse interval, browser view); the sparse and packed-read tests hold,
      including `sparse_spruce_projects_only_changed_contact_paths`.
- [x] **R3** the contact query chooses the same run as the surface.
- [x] **R4** every shipped preset byte-identical at fork height 0; the
      birch's fork height is then set against S-WHOLE and S-BARE (the
      photographs disagree: S-WHOLE parts at the ground, S-BARE forks about
      a third of its visible height up), its pairs rendered again, visual QA
      before returning, and the owner judges in fn-34.

## NEEDS_HUMAN

R4's last clause is the owner's verdict in fn-34, and this task cannot award
it. The round is REPORT.md's round 20; its pairs are recorded by sha256 in
`round20-fn48c/stills.json` with `visual_status: unassessed`, and the work is
merged into `fn-34-integration` (`64a3aea4`).

The birch states the fork at half the bole, the rail's top, which puts the
fork node at 0.96 m. The two photographs disagree and the row cannot satisfy
both: S-WHOLE's pair leaves the ground as two stems, and S-BARE's stands on
one trunk that forks at its lowest limbs, about a third of its visible
height up. The owner asked for the stem to move up, and 0.5 is as far toward
S-BARE as the row reaches.

The host's read on the pairs: the trunk narrows into the upright stem with no
ring and no step, and the leaning stem leaves its left side in a plain crotch,
so there is no ledge and no seam. S-BARE shows one white trunk to about a
metre and then the pair, which is its photograph's habit, though the
photograph forks well above anything the rail reaches. S-WHOLE stands on a
metre of one trunk under the curtain where its photograph parts at the ground,
and the lean still barely reads.

All forty-eight protocol cases pass (`measure/protocol-fn48c/`), none capped.
Every shipped table is byte-identical with its fork at the ground: wood,
normals, coordinates, indices and leaves, at two seeds and a scrubbed view.
The specimen snapshot moves to schema 2 for the stem flag, and the browser's
wire decoder reads the extra byte.

A consequence found later, in round 21 on the integration branch: the S-BARK
shot block aims its camera at 0.06 of the tree's height, and the merged birch
stands 16.43 m, so the camera now sits at 0.986 m, just above this fork. The
close-up frames the crotch rather than the bole it framed before.

## Done summary
The fork where a stem parts from a stem is swept clean, the trunk run carrying on into the upright stem; the specimen snapshot moved to schema 2. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: 55dc2ddc
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: