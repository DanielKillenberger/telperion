# Closed specs keep their result and verdicts, not their working records

## Goal & Context

Owner (2026-10-24), on PR #159 adding about 255k lines: "it's still 250k loc added which is a bit crazy. can we do another cleanup?". They chose to prune closed specs: keep open specs whole, and keep each closed spec's short result and verdicts. Closed specs' task files, FRICTION logs and bulky evidence go. Git history keeps everything.

Measured against origin/master on 2026-10-24: `.flow/evidence` adds 41,579 lines, `.flow/specs` 33,283 and `.flow/tasks` 11,891.

## Unknown, check before the plan

- What flowctl needs from a closed spec to keep working: `list`, `show`, `deps`, the review state, and a spec closing that other specs depend on. Can a closed spec's task `.md` and `.json` go, or only the `.md`? Test it on a scratch copy before deleting anything.
- Whether any test, script or doc reads files under `.flow/evidence` or `.flow/tasks`. Grep for it. Anything read stays, or the reader moves.
- What `docs/evidence-retention.md` already says. The prune follows it and amends it.

## Checked (conductor, 2026-10-24, on scratch clones at `0a62c7ec2`)

- **Classes.** 295 specs: 98 open, 40 done specs fn-237 depends on directly, 157 done specs to prune. Taking fn-237's dependencies transitively would protect 24 more; the rule names its direct dependencies, so those 24 are pruned.
- **flowctl.** A task's `.md` and `.json` go together or not at all. Removing only the `.md` gives `validate` 279 "Task spec missing" errors. Removing both leaves `list`, `specs`, `show` and `cat` on every spec working, `spec chain` on every open and fn-237-dependency spec byte-identical, and every non-task field of `show` unchanged; a pruned spec stays `done` with zero tasks. `validate --all` keeps its one error, which predates this spec (fn-82 depends on fn-111, which no longer exists; the same error makes the unscoped `pilot snapshot` and `spec chain fn-82` fail on the base too). Its warnings move from 278 to 235: the 138 "done spec with a todo task" warnings on stale task snapshots go, and 122 pruned specs gain "uncovered R-IDs" because no task is left to cover them. `brief` drops from 90 actionable tasks to 20, because the other 70 were todo tasks of done specs.
- **Readers.** Tests, scripts, CI, catalogue and docs read or cite evidence of pruned specs: `fn9/profiles.json`, `fn34/profiles.json`, `fn19/protocol.json` and `fn19/references.json`, the `fn29` and `fn34` catalogue stills, `fn24/ordinary-hero.png`, `fn32/stills.json`, the `fn71/resolution/` and `fn58/validation/` directories, `fn80/profiles-date-palm.json`, `fn-68`'s `pilot-config.json` and `contact-sheet-protocol.md`, the `fn-193`..`fn-196` `SOURCES.md` files, and the reports README, docs and `scripts/benchmarks/generation.md` link. Every file a tracked file outside `.flow/`, or an open or fn-237-dependency spec's record, names stays. One task file is named that way: `fn-5-…5.md`, cited by open fn-176, so that task pair stays.
- **`docs/evidence-retention.md`** asks a cleanup to check references, keep executable test inputs, and keep a compact manifest with the source revision, paths and checksums. Git already holds each removed file's checksum (its blob id) at the source revision, so the manifest for this prune is the source revision and the recovery command, not a per-file list. The doc also says the policy "does not retroactively delete other specs' evidence"; R1 amends that.
- **Size of the win.** Against origin/master the pruned specs add 12,357 evidence lines and 3,105 task lines; fn-237's dependencies add 19,322 and 4,893, open specs 8,161 and 3,893. The prune removes at most about 15,000 of the 255,546 added lines; the spec files themselves, which stay, add 33,480.
- **Unreviewed friction.** The FRICTION.md of fn-186 (4 entries), fn-209 (30), fn-211 (25) and fn-214 (7) has no host disposition: fn-209, fn-211 and fn-214 were closed as superseded, and fn-186's spec does not take up its entries. Those four files stay until the host decides; every other pruned FRICTION.md was disposed in its spec's Friction section.

## Requirements

- **R1 (the rule, written into `docs/evidence-retention.md`):** a spec that is done, or closed, keeps:
  - its spec `.md` and `.json`;
  - its `RESULT.md`, or the one summary file that stands for it;
  - the compressed sheets the owner's verdicts rest on.

  Everything else goes: the remaining evidence files (logs, tables, scripts, patches, raw notes, FRICTION.md), and whatever task files flowctl does not need. An open spec, and every spec fn-237 depends on until fn-237 closes, keeps everything.

  From the Unknowns (Checked, above), a pruned spec also keeps: every file a tracked file outside `.flow/`, or an open or fn-237-dependency spec's record, names (a task pair stays whole when either half is named, and a kept task keeps the tasks it depends on, transitively, so `validate` finds each dependency: fn-5.5, which open fn-176 cites, keeps fn-5.1 to fn-5.3); the directories code reads whole (`fn71/resolution/`, `fn58/validation/`); the inputs of a kept reader (a kept script keeps its own directory, and a kept script or JSON file keeps every file of its spec it names by basename or relative path, to a fixpoint; so `fn19/onboarding-examples/verify.py`, which `docs/species-onboarding.md` runs, keeps the packets it reads; that example already fails on the base with `ValueError: unsupported protocol definitions/schema`, a separate defect this spec does not fix); its `.gitignore` files; a PNG still its own spec cites; and a FRICTION.md the host has not disposed of. Its task `.md` and `.json` otherwise both go. The summary is the first top-level `RESULT.md`, `RESULTS.md`, `REPORT.md` or `README.md`, else a top-level `*RESULT*.md`; a spec with none keeps its spec as the record, as the 2026-10-16 trim did. Errors: a prune that leaves a named file missing, or a task half, is wrong.
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
- `flowctl validate --all` carries one error that predates this spec (open fn-82 depends on fn-111, which no longer exists). Fixing it edits an open spec's record, so it stays out; R3's "clean" reads as "no error this spec adds".
- `.flow/review-receipts` and `.flow/memory` stay as they are; memory entries that cite a pruned task id keep the id as history.

## Early proof point

The scratch-clone run in Checked already proved flowctl survives the prune. Task fn-312-closed-specs-keep-their-result-and.1 proves the delete list on the real tree: if `flowctl validate --all` gains an error or a test reads a removed file, re-evaluate the keep rules before the docs task.

## Strategy Alignment

_No active strategy track served — review for drift._ The prune serves the release (fn-237) by shrinking its diff; it changes no product behaviour.

## Decision Context

- The manifest for this prune is its source revision and the recovery command, recorded beside the 2026-10-16 entry in `.flow/evidence/ARCHIVE-MANIFEST.md`, not a per-file list: git holds each file's checksum as its blob id, and a per-file list would add back a line for every file removed. The release branch may be squash-merged, so the recovery names the release PR's ref (`refs/pull/159/head`) as well as the commit.
- fn-237's direct dependencies are protected, not its transitive ones, because the owner's rule names the specs fn-237 depends on.
- Two commits: fn-1 to fn-99 (66 specs), then fn-100 and above (91 specs).

## Quick commands

```bash
python3 <scratchpad>/prune_list.py 0 99 | wc -l     # dry run, first batch
flowctl validate --all --json | jq '.total_errors'  # 1, the fn-82 error that predates this spec
flowctl list --json | jq '.specs | length'
```

## Requirement coverage

| Req | Task(s) |
| --- | --- |
| R1 | fn-312-closed-specs-keep-their-result-and.2 |
| R2 | fn-312-closed-specs-keep-their-result-and.1 |
| R3 | fn-312-closed-specs-keep-their-result-and.1 (flowctl, lines); the gate, `npm test` and the codex reviews run at completion |
