# fn-311 friction

## 2026-10-24, task .1: dcg blocks every log redirect into the scratchpad

Doing: capturing `cargo check` output to a log under the session scratchpad, per the suite-output capture rule.
Hindered: dcg refused `> "$S/logs/x.log"` (dynamic path) and then the same redirect with a literal scratchpad path (home path, truncating). Only `| dcg create-new <path>` passed, which also changes how the exit code is read (`${PIPESTATUS[0]}`).
Cost: two blocked calls, about 2 minutes.
Would remove it: dcg treating the session scratchpad like `/tmp` for truncating redirects, or the worker brief naming `| dcg create-new` as the capture form for cargo logs.

## 2026-10-24, task .1: impl-review scopes to telperion's range while the code is in the fork

Doing: the codex impl-review of a task whose code lives in a scratch clone of the wgpu fork.
Hindered: flowctl's review reads `--base..HEAD` in telperion, which held only an evidence note. Its `triage-skip` step would have passed that docs-only diff as SHIP without reviewing the fork, so the worker skipped triage and steered the reviewer to the fork through `--focus`. The reviewer could read the clone but not run cargo or reach GitHub from its read-only sandbox.
Cost: about 3 minutes of working out the route; no failed dispatch.
Would remove it: a review flag that names another repository and range as the artifact under review.

## 2026-10-24, task .2: a one-file vitest run builds the release binaries first

Doing: running `npx vitest run scripts/wgpu-family.test.mjs`, the spec's Quick command for a check that only reads `Cargo.lock` and `Cargo.toml`.
Hindered: `vitest.config.ts`'s `globalSetup` (`harness/release-binaries.setup.ts`) builds the workspace's release binaries before any test file runs, whichever files are selected. In a fresh worktree that was a cold `cargo build --release`.
Cost: 143 s for a run whose tests took 9 ms.
Would remove it: a global setup that builds only when a selected test file needs the binaries, or a vitest project for `scripts/**/*.test.mjs` without the setup.

## 2026-10-24, task .2: test:render fails on "Ordinary generated no wood"; the base has to be built apart to tell whose it is

Doing: proof step 4, `npm run test:render` once on the fork.
Hindered: it failed at the first preset ("Ordinary generated no wood") with no alert and the page started. Whether the base fails the same way needs a second cold build of both wasm modules from an archive of the base commit (two checkouts never share a target dir).
Cost: one failed browser run plus a cold base build (minutes, recorded below when measured).
Would remove it: a recorded last-green test:render per master commit, so a worker reads whether the failure predates it.
Measured: the base (an archive of `8bb9f9e9b`, its own target dir) fails `npm run test:render` with the same "Ordinary generated no wood" at the same line; the base build took about 75 s of cargo plus the browser run. The failure predates fn-311.
