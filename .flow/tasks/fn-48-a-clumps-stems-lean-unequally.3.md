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

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
