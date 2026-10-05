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

## 2026-10-05, worker, task 1

- **Doing:** rendering the spruce at 80 years (R4 and the seed check).
- **Slowed by:** wgpu Out of Memory on three of six spruce launches (about 200 million wood triangles), each retried after 30 s under the GPU lock.
- **Cost:** about 3 minutes of retries.
- **Would remove it:** a still runner that frees or budgets GPU memory for a mesh this size; the lock serialises renders but not other GPU users.

## 2026-10-05, worker, task 1

- **Doing:** R3, the walk from the oak to the palm.
- **Slowed by:** no species-to-species walk exists, and the two species' settings do not line up (10 PAs against 1, integer settings, different presets); stopped at a design question (RESULT.md, R3).
- **Cost:** about 10 minutes of reading; R3, the gate, the review and `done` wait.
- **Would remove it:** the host's mapping between reference axes of different lengths.

## 2026-10-05, worker, task 1

- **Doing:** the workspace gate for fn-196.
- **Slowed by:** `suite::species::fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` failed on its peak-resident ceiling (2.75 GB against 2.57 GB) with the machine at a load of 20 from other workers. Run alone twice on the same binary right after: it passed, then failed (8.85 GB against 8.69 GB). The ceiling is derived from the machine's free memory, so it moves with other workers' builds. The beech preset has no leaf-base rows, so fn-204's clothing returns before it does any work for it (`clothe_leaf_bases`, `bearing`).
- **Cost:** two reruns, about 1 minute; the gate's record carries one flaky failure.
- **Would remove it:** a ceiling that does not move with other processes' memory, or the test kept off a shared machine's gate.
