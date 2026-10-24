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

## 2026-10-24 - task 1 baseline: the AC's ci-key command cannot run

- Doing: the pre-edit baseline for fn-313.1's acceptance `node --test scripts/ci-key.test.mjs`.
- Hindered: the file is a vitest test (`import ... from 'vitest'`), so `node --test` fails on the base before any edit (`TypeError: Cannot read properties of undefined (reading 'config')` in @vitest/runner). `npx vitest run scripts/ci-key.test.mjs` is the working form, but the vitest config's global setup compiles a release cargo build (telperion-jev, 1m 26s) outside telperion-heavy, which the dispatch did not anticipate.
- Cost: about 4 minutes and one unplanned cargo build.
- Remedy: write the AC as `npx vitest run scripts/ci-key.test.mjs`, and note that it builds Rust.
