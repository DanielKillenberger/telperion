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

## 2026-10-24 13:30, task 2 worker, local dry run

- **Doing:** R4's local dry run: `npm run build`, then `node scripts/artifact-budgets.mjs`.
- **Slowed by:** the size check fails on the base, before any edit of this task: `FAIL dist/wasm-source.js: not built by the recipe`. Vite now emits the shared chunk as `dist/module.js` (64,631 bytes), and `scripts/artifact-budgets.json` still names `dist/wasm-source.js`. The release branch's own Tests run 40087469094 (2026-10-24) fails its package job on the same line. Until the budgets entry follows the chunk, the package job, and so the prerelease workflow's build, fails before any version is set; the CI dry run of R4 cannot pass.
- **Cost:** about 6 minutes to confirm it is inherited (budget script, vite config, the branch's CI log).
- **Would remove it:** renaming the budget entry to `dist/module.js` (or naming the chunk `wasm-source` again) in its own change; outside this task's Touches, so reported to the host.

## 2026-10-24 13:32, task 2 worker, raw output

- **Doing:** writing `npm pack --dry-run --json` into the evidence `raw/` directory through a shell variable for the path.
- **Slowed by:** dcg blocks a truncating redirect to a variable path (`> $R/x.json`), even inside the worktree; the literal path passes.
- **Cost:** one blocked call, about a minute.
- **Would remove it:** nothing in the repository; write literal paths.

## 2026-10-24 13:50, conductor, completion review

- **Doing:** dispatching the codex completion review, after the `ps ... target/ci/deps` check the machine rules require.
- **Slowed by:** another worktree's gate (f4-311, `cargo test --profile ci`) runs its test binaries under `target/ci/deps`, so the review waits for it; the check also matches that worktree's `rustc` compiles (`--out-dir .../target/ci/deps`), which are not test binaries and need a filter to tell apart.
- **Cost:** over 10 minutes of waiting, so far.
- **Would remove it:** a `telperion-heavy` mode that waits for running test binaries itself, so a reviewer dispatch (which builds nothing) is not serialised behind a sibling gate by hand.

## 2026-10-24 14:35, conductor, task 3 review

- **Doing:** dispatching task 3's codex impl-review after the host's trusted-publishing ruling.
- **Slowed by:** a second sibling gate (f4-312) started its test binaries just before the dispatch, so the review waits again; f4-311's gate had already cost about 25 minutes of waiting before the completion review earlier.
- **Cost:** about 20 minutes per sibling gate, serialised by hand.
- **Would remove it:** the same as the 13:50 entry: a queue in `telperion-heavy` that holds a dispatch while a gate's test binaries run, instead of each conductor polling `ps`.
