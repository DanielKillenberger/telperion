---
satisfies: [R5]
---
# fn-259-the-release-meets-the-bar-on-every.2 The browser bindings suite green, headless, and in CI

## Description
Make `tests/browser/bindings.mjs` pass in headless Chromium with no WebGPU, give it its own entry, and run it in CI. Today it stops at its first red check (`ordinary has local branches and a leaf plan`, measured at plan time with a vite server and the rebuilt Wasm); fn-257 task 14 listed 11 red checks plus `maxInstances = 40000` refused and the empty-tree checks red because `maxNodes = 0` no longer empties a tree.

**Size:** M
**Files:** `tests/browser/bindings.mjs`, `scripts/test-wasm.mjs` (or a new `scripts/test-bindings.mjs`), `package.json`, `.github/workflows/tests.yml`, `README.md`, `docs/gpu-tests.md`
**Touches:** [tests/browser/bindings.mjs, scripts/test-wasm.mjs, scripts/test-bindings.mjs, package.json, .github/workflows/tests.yml, README.md, docs/gpu-tests.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/BINDINGS.md]

### Approach
- First, list every red check: temporarily turn `check` into a collector in a scratch copy (never committed) or iterate fix-and-rerun; record the list with each cause in the evidence (`BINDINGS.md`) before changing a check.
- For each: if the subject still exists, fix the fixture or the expectation to today's contract (read the core's behaviour, e.g. how a tree is emptied now, the instance budget's limit, the catalogue's listing of the beech, crookedness range checks); if the subject is gone, delete the check and name why. A check whose subject exists and is red after an honest fixture fix is a defect: stop and report it, never delete it.
- Entry: split `scripts/test-wasm.mjs` so the bindings suite runs alone (build Wasm, vite on port 0, set `BROWSER_URL`, import `bindings.mjs`). `field.mjs` needs WebGPU: keep it out of this entry and out of CI; say so in the script header and `docs/gpu-tests.md`.
- CI: a job that installs the toolchain and wasm-bindgen as the `node` job does (`tests.yml:237-270`), `npm ci`, `npx playwright install --with-deps chromium`, then the entry. It runs on every run the workflow triggers, with no receipt skip (the `node` receipt certifies `npm test`, not this suite, and its inputs omit the entry).
- Local run: the plan-time probe `/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/run-bindings.mjs` shows the shape (needs `PLAYWRIGHT_MODULE=node_modules/playwright/index.mjs` in this worktree). Run under `telperion-heavy 6 --threads 4`.

### Investigation targets
**Required:**
- `tests/browser/bindings.mjs` (whole file, 244 lines)
- `scripts/test-wasm.mjs`
- `.flow/tasks/fn-257-the-site-grows-its-trees-on-the-device.14.md:40-46` - the red list and causes seen then
- `src/browser/core.ts` - the binding's contract
**Optional:**
- `crates/telperion-wasm/src/` - the native C ABI the last block probes

### Key context
Never run WebGPU in a browser. This suite needs none (`bindings.mjs:7-12`); if a check turns out to need an adapter, it leaves this suite and is reported. The gate (`telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, background, no sleep-polling) and `npm test` run once at the end of this task.
## Acceptance
- [ ] Evidence lists every check that was red at the start, its cause, and fixed/deleted with the reason.
- [ ] The entry runs the suite to its final `console.log` with exit 0 in headless Chromium, no WebGPU flag.
- [ ] A CI job runs that entry on every run the tests workflow triggers, never skipped by a receipt; the YAML is valid (`npx --yes yaml-lint` or an equivalent parse) and the job's steps match what ran locally.
- [ ] `field.mjs`'s WebGPU need is stated where the suite is documented; it is not in the CI job.
- [ ] Gate and `npm test` green, counts recorded.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
