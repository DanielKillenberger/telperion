# Friction: fn-196

## 2026-10-05, worker, task 1

- **Doing:** finding the date palm's reference photographs to put beside the sheet.
- **Slowed by:** there is no `.flow/references/date-palm/` folder, unlike the spruce, beech, oak and ash. The only local copies are under `.worktrees/fn-80-the-gap-loops-first-live-run/.refs/fn80/date-palm/` (whole.jpg, trunk.jpg, base.jpg), found by a filesystem search; `catalogue/date-palm/packet/references.json` names `.refs/fn80/date-palm/` relative to a checkout that no longer holds it.
- **Cost:** about 5 minutes and three searches.
- **Would remove it:** a `.flow/references/date-palm/` folder with resized copies and a README, as the spruce has.

## 2026-10-05, worker, task 1

- **Doing:** carrying the palm's organs through `tree::convert` and `executor::expand`.
- **Slowed by:** the retained leaf bases are hung in the pipeline's skeleton stage (`pipeline::skeleton`, `branching::clothe_leaf_bases`), which `executor::expand` skips, and `branching` is private, so an engine tree cannot reach them. Found by reading; it stops the build at a design question (RESULT.md).
- **Cost:** about 15 minutes of reading; the round-1 sheet is not rendered.
- **Would remove it:** the host's answer to RESULT.md's question D1.
