# Pull request format

The body tells the owner what changed, what proves it and where to look, in one screen. GitHub shows the diff; the spec holds the plan; the body repeats neither. This file replaces make-pr's body phases in this repository (owner, 2026-09-20). The reason: bodies had grown to 12 to 57 KB, and reading make-pr's `workflow.md` cost about 30k tokens before a line was written (fn-72 friction, 2026-09-19).

## Procedure

Do not read make-pr's `workflow.md`, `pr-cognitive-aid.md`, `create-and-finalize.md`, `mermaid-rules.md` or `html-lens.md`. No cognitive-aid artifact, no mermaid, no `pr.html`.

Before staging evidence, follow [evidence-retention.md](evidence-retention.md): keep summaries and reusable verification sources; leave raw output in ignored storage.

1. Pre-flight: `gh auth status` passes, HEAD is ahead of the base (default `origin/master`), and `gh pr view --json state -q .state` is not `OPEN` for this branch.
2. One call: `flowctl spec export-cognitive-aid <spec-id> --base <ref> --json`. It is the only source for the body, together with the spec's FRICTION.md and QA verdict when they exist.
3. Write the body below to the scratchpad. Title is the spec title verbatim.
4. `git push -u origin HEAD`, then `gh pr create --title … --body-file …`. Draft when `tasks_summary.open`, `uncovered_r_ids`, `undeclared_r_ids` or `deferred_findings` is non-empty, or the run is autonomous; `--draft` and `--ready` win over both. `--dry-run` prints the body and stops before the push.
5. Print the PR URL. Never merge.

## Stacked mode

A species run is one GitHub stack (`docs/species-onboarding.md`, "The stack"). When `gh stack view` run on the spec's branch lists a stack, the branch below it in that list is the base: pre-flight compares against `origin/<that branch>`, the export takes `--base origin/<that branch>`, and the PR is opened with `gh pr create --base <that branch>`, never against master. The body's first line under Change names the stack's place: `Stack: <species> <n> of <m>, above <branch below>`. Nothing in this procedure merges; the stack merges once, by the owner.

## Body

```
## Change
## Proof
## Look here
## Decisions
## Open
<!-- flow-next:make-pr spec=<spec-id> base=<base-ref> -->
```

- **Change.** Two to four sentences: the behaviour before, the behaviour after, and what did not change.
- **Proof.** One line per R-ID: `R1 ✅ <test or command>`, `⏳ claimed, not yet evidenced`, or `⚠️ uncovered`. Then one line for the commands that passed, with counts. Anything untested is said plainly, with the reason.
- **Look here.** The paths that deserve the owner's eyes, one reason each, drawn from `diff_summary` (churn, public exports, cross-module changes, security paths). The rest is safe to skim, and the section says so.
- **Decisions.** One line per entry in `memory_during_epic.decisions`, with its ID. Omitted when there are none.
- **Open.** Open tasks, deferred findings, the QA verdict, and the count and path of FRICTION.md entries. Omitted when empty.
- **Marker.** The last line, always; land's authorship probe keys on it.

A section with nothing to say is left out, never left as an empty heading.

## Two tiers

| | Small | Large |
|---|---|---|
| Applies when | under 200 changed lines and under 6 files | otherwise |
| Target, cap | 1,500 characters, 3,000 | 4,000 characters, 4,000 |
| Look here | at most 3 paths | at most 5 paths |
| File table | none | one flat table: path, +/−, one-phrase role |

Generated and mechanical files take one row together in the large tier's table. A body over its cap is cut, not spilled to a file.

## Guardrails

Every claim traces to a field of the export payload. No invented paths, SHAs, R-ID attributions or reasons; unknown is written as unknown. No raw diff content and no code snippets. A later review round edits the sections in place and never appends a second body.
