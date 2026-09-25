# fn-153 friction

## 2026-09-25: the npm trusted publisher names a workflow file

- **Doing:** moving the publish from `release.yml` into `tests.yml`.
- **Hindrance:** npm's trusted publisher for `telperion` is configured on npmjs.com for `release.yml`, and npm checks the calling workflow's filename. The spec and dispatch did not mention this, and the repository cannot change it. Until the owner points the trusted publisher at `tests.yml`, the first release on the new path fails at `npm publish`, after every refusal check and before any tag.
- **Cost:** about 5 minutes to find and confirm against docs.npmjs.com/trusted-publishers.
- **Would have removed it:** a line in the spec's "What exists" naming the trusted-publisher binding as an external constraint.

## 2026-09-25: the dcg hook refuses scripted edits to dynamic paths

- **Doing:** adding the task's `satisfies` frontmatter with a shell `mv`, and simulating the publish job's refusal steps with redirects to a variable scratch path.
- **Hindrance:** dcg blocks `mv` and `>` redirects whose target is a shell variable, so both commands were rewritten with literal paths or the Edit tool.
- **Cost:** about 3 minutes and two retries.
- **Would have removed it:** a `flowctl task set-satisfies` verb (only `task create --satisfies` exists). The dcg rule itself is a local setup matter.
