# fn-313 friction

## 2026-10-24 13:05, conductor, plan preflight

- **Doing:** writing `flowctl preflight --json` to a scratch file, as the plan skill's Step 0 says (`> "${TMPDIR}/flow-plan-config-<suffix>.json"`).
- **Slowed by:** the dcg hook blocks a truncating redirect into any home path outside a VCS worktree, and the session scratchpad is under `~/.cache`. Two blocked calls before switching to `| dcg create-new <path>`.
- **Cost:** about 2 minutes and two tool round-trips.
- **Would remove it:** the flow-next skills writing their scratch through `dcg create-new` or into `.flow/tmp/`, or the scratchpad being on dcg's allowed list.

## 2026-10-24 13:08, conductor, actionlint

- **Doing:** running `actionlint` for R4.
- **Slowed by:** `actionlint` and `shellcheck` are mise shims with no version set, so both fail; downloaded the official actionlint 1.7.12 (checksum-verified) and shellcheck 0.11.0 binaries into the scratchpad and passed `-shellcheck=<path>`.
- **Cost:** about 3 minutes.
- **Would remove it:** a local setup problem on the owner's machine (`mise use -g actionlint shellcheck`); reported, not specced.
