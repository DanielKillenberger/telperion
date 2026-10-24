---
satisfies: [R1, R2]
---
# fn-314-the-release-branchs-ci-is-green-and-the.1 CI's two failing checks pass: gate the bloom test, rename the budget entry

## Description
Fix both red jobs with the smallest change: gate the one lib test that calls the geometry-only `bloom_element`, and rename the budget entry to the chunk vite emits. One task because both are one-line data or attribute changes proven by CI's own commands.

**Size:** S
**Files:** `crates/telperion-core/src/pipeline/foliage/bounds_tests.rs`, `scripts/artifact-budgets.json`
**Touches:** [crates/telperion-core/src/pipeline/foliage/bounds_tests.rs, crates/telperion-core/src/pipeline/foliage.rs, scripts/artifact-budgets.json]

### Approach
- R1: put `#[cfg(feature = "geometry")]` on `stalks_reach_from_each_origin_and_none_without_length` (`bounds_tests.rs:157-190`), the pattern `foliage.rs:17-21` uses for the re-export. Gate the test, not the module: the file's other tests need no geometry. If CI's command then shows a further geometry-only use, gate it the same way.
- Red first: CI's `core without geometry` commands (`tests.yml:173-176`) fail on the base (reproduced at plan time, one error). Green after. Then run the gated test in the default build (`cargo test --profile ci -p telperion-core --lib stalks_reach`) to show it still runs.
- R2: rename the `dist/wasm-source.js` entry (`artifact-budgets.json:36`) to `dist/module.js`; keep `ceilingBytes` 70000 and the reason text, and add to the reason that fn-297 (171e522cc) renamed the shared chunk after `src/browser/module.ts`. Re-measure `baselineBytes`/`baselineOn` on this branch's `npm run build`. Record the old and new byte sizes in the commit body.
- Run `npm ci` once if `node_modules` is absent, then `npm run build && node scripts/artifact-budgets.mjs` through telperion-heavy.
- Every cargo and npm build goes through `~/.local/bin/telperion-heavy 5 --threads 2 -- ...`; check `ps -eo pid,args | grep target/ci/deps` first.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/foliage/bounds_tests.rs:150-190`
- `crates/telperion-core/src/pipeline/foliage.rs:1-40`
- `scripts/artifact-budgets.json`
- `scripts/artifact-budgets.mjs:40-60` (check)

### Key context
- `vite.config.ts:52` names chunks `[name].js` on purpose; do not change it (Decision Context).
- The no-geometry build prints three dead-code warnings; leave them (Boundaries).

## Acceptance
- [ ] CI's `core without geometry` commands, as written in `tests.yml`, fail on the base and pass on the fix
- [ ] The gated test passes in the default build
- [ ] `npm run build && node scripts/artifact-budgets.mjs` exits 0; `dist/module.js` is listed with ceiling 70000 and a re-measured baseline
- [ ] No other budget entry changes

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
