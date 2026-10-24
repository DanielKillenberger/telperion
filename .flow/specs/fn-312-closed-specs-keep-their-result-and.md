# Closed specs keep their result and verdicts, not their working records

## Goal & Context

Owner (2026-10-24), on PR #159 adding about 255k lines: "it's still 250k loc added which is a bit crazy. can we do another cleanup?". They chose to prune closed specs: keep open specs whole, and keep each closed spec's short result and verdicts. Closed specs' task files, FRICTION logs and bulky evidence go. Git history keeps everything.

Measured against origin/master on 2026-10-24: `.flow/evidence` adds 41,579 lines, `.flow/specs` 33,283 and `.flow/tasks` 11,891.

## Unknown, check before the plan

- What flowctl needs from a closed spec to keep working: `list`, `show`, `deps`, the review state, and a spec closing that other specs depend on. Can a closed spec's task `.md` and `.json` go, or only the `.md`? Test it on a scratch copy before deleting anything.
- Whether any test, script or doc reads files under `.flow/evidence` or `.flow/tasks`. Grep for it. Anything read stays, or the reader moves.
- What `docs/evidence-retention.md` already says. The prune follows it and amends it.

## Requirements

- **R1 (the rule, written into `docs/evidence-retention.md`):** a spec that is done, or closed, keeps:
  - its spec `.md` and `.json`;
  - its `RESULT.md`, or the one summary file that stands for it;
  - the compressed sheets the owner's verdicts rest on.

  Everything else goes: the remaining evidence files (logs, tables, scripts, patches, raw notes, FRICTION.md), and whatever task files flowctl does not need. An open spec, and every spec fn-237 depends on until fn-237 closes, keeps everything.
- **R2:** apply the rule to every closed spec, in one commit per hundred specs or so, so the diff can be read.
- **R3 (proof):**
  - flowctl `list`, `show` and `deps` on the release work.
  - `flowctl validate`, if it exists, is clean.
  - The gate and `npm test` are green.
  - The lines removed are reported against origin/master, before and after.
  - A codex review, dispatched inside telperion-heavy.

## Boundaries

- No product code, and no open spec's records.
- The friction reviews of closed specs were done at their builds; deleting FRICTION.md does not drop an open friction item. Check each closed spec's FRICTION against its build's host review, and carry any unreviewed entry to the host before deleting it.
- Joins the release (fn-237).
