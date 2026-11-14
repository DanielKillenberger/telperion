# The release branch's CI is green, and the local gate sees what CI sees

## Goal & Context

PR #159's CI fails two jobs the local gate never runs. This is run 40087469094, on the release branch pushed 2026-10-24.

- **`core without geometry`:** `telperion-core`'s lib tests do not compile with the geometry feature off: `error[E0425]: cannot find function bloom_element in this scope`.
- **`package`:** `node scripts/artifact-budgets.mjs` exits 1. `scripts/artifact-budgets.json` still names `dist/wasm-source.js`, but vite now emits that shared chunk as `dist/module.js` (found by fn-313, 2026-10-24). Every ceiling that is checked holds; only the missing entry fails.

The owner wants the release merged, and a red PR cannot be merged with confidence.

Both breaks came in with fn-297 (merge 9e7818f41, 2026-10-20): 6a6f74ffb added a lib test that calls the geometry-only `bloom_element`, and 171e522cc added the renderer's shared module, which both entries now import, so vite's `[name].js` chunk naming names the shared chunk after it. Reproduced on the base (plan, 2026-10-24): CI's `core without geometry` commands fail on exactly one error, `bounds_tests.rs`'s use of `bloom_element`, in 49 s from a cold target at two threads.

## Acceptance Criteria

- **R1:** the no-geometry build's tests compile and pass.
  - Find which merge broke them (`git log -S bloom_element`).
  - Fix the code so that `bloom_element`, or its test, is gated with the feature.
  - Run CI's command for that job as written in `.github/workflows/tests.yml`.
  - Errors: a second geometry-only use that the first error hid fails the same command and is fixed the same way; the gated test still runs, and passes, in the default (geometry) build, so no coverage is lost.
- **R2:** the budget entry names the chunk the build emits.
  - Either rename the entry to `dist/module.js`, carrying its ceiling and reason, or make vite name the chunk `wasm-source` again.
  - Choose whichever keeps the published file names stable for consumers. Check which name the package's exports and `scripts/test-dist.mjs` use, and say why.
  - The check passes on a fresh `npm run build`.
  - Errors: no ceiling changes; the entry's `baselineBytes` and `baselineOn` are re-measured on this branch's build, since the old figure measured another chunk.
- **R3 (the cause):** neither failure could reach CI if the local gate ran these two checks.
  - Add both to `scripts/gate.py`'s pre-run guards, beside fn-306's two guards, at their measured cost.
  - If either costs more than about a minute, run it beside the gate instead and say so in AGENTS.md's gate line.
  - Errors: a check that fails, or cannot start (no `node_modules`, no npm, no wasm target), fails the gate with its own output and a nonzero exit; it is never skipped. Each check's command is CI's, copied from `tests.yml`. A check placed after the test run still runs, and still fails the gate, when the test run failed.
- **R4:** run every job in CI's `tests.yml` locally on the fixed branch through its own command, or state why one can't run locally. The gate and `npm test` are green, with a codex review dispatched inside telperion-heavy.
  - Errors: a job that cannot run locally (the receipt cache jobs) is named with the reason; the package job's `test:dist` runs on the dist unpacked from the packed tarball, as CI does.

## Early proof point

Task fn-314-the-release-branchs-ci-is-green-and-the.1 validates the core approach (CI's two failing commands go green with the smallest code change). If it fails, re-evaluate whether more of the core's tests depend on geometry before continuing with .2.

## Boundaries

- No ceiling is raised.
- Joins the release (fn-237).
- The budget check does not gain a reverse check (an emitted `dist/*.js` no entry lists); that is a follow-up the owner may take, not part of this fix.
- The three dead-code warnings the no-geometry build prints are not touched; CI does not deny warnings.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the release branch's integration checks go green and the local gate runs the two CI checks it missed.

## Decision Context

- **R2, rename the entry rather than pin the chunk name.** `package.json`'s `exports` name only `telperion.js`, `field.js`, `voxelize.js` and the two wasm files, and `scripts/test-dist.mjs` loads the entries only, so no consumer can import the chunk; its name is internal. The 0.1.4 tarball ships `dist/wasm-source.js`, but `exports` blocks a deep import of it. `vite.config.ts` names every chunk after its module by design (`[name].js`); pinning one chunk's name would add config to keep an internal file name, so the entry follows the build.
- **R3, where the checks run.** fn-306's two guards are nextest priority overrides in `.config/nextest.toml`, not steps in `gate.py`; the new checks are cargo and npm commands, so they run as `gate.py` subprocesses. A check measured under about a minute on the desk (after a change to the core crate, at the gate's four threads) runs before the test run; one over it runs after the test run, reported beside it and not counted in the run's 1,560 s budget, as the build and listing are.

## Quick commands

```bash
~/.local/bin/telperion-heavy 5 --threads 2 -- bash -c 'cargo build --profile ci -p telperion-core --no-default-features && cargo test --profile ci -p telperion-core --no-default-features --lib'
~/.local/bin/telperion-heavy 5 --threads 2 -- bash -c 'npm run build && node scripts/artifact-budgets.mjs'
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | the no-geometry build's tests compile and pass. | fn-314-the-release-branchs-ci-is-green-and-the.1 | — |
| R2 | the budget entry names the chunk the build emits. | fn-314-the-release-branchs-ci-is-green-and-the.1 | — |
| R3 | (the cause): neither failure could reach CI if the local gate ran these two checks. | fn-314-the-release-branchs-ci-is-green-and-the.2 | — |
| R4 | run every job in CI's `tests.yml` locally on the fixed branch through its own command, or state why one can't run locally. The gate and `npm test` are green, with a codex review dispatched inside telperion-heavy. | fn-314-the-release-branchs-ci-is-green-and-the.3 | — |

