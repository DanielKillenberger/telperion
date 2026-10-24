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
`tests/browser/bindings.mjs` now passes in headless Chromium with no WebGPU. It runs alone through `npm run test:bindings` (`scripts/test-bindings.mjs`), and a new `bindings` job in `tests.yml` runs it on every run with no receipt skip. `scripts/test-wasm.mjs` now runs only `tests/browser/field.mjs`, which needs WebGPU and is in no CI job. README and `docs/gpu-tests.md` say which suite needs WebGPU.

Twelve checks were red at the start. Every cause is in `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/BINDINGS.md`.

- **Fixed (7):**
  - The leaf-plan check no longer expects nodes past a crossover; the engine sets crossover equal to nodes.
  - `bad height` keeps its NaN and Infinity cases. The -1 case is dropped because the envelope is a scaffold row that no build reads.
  - `valid empty outputs` and `native empty recovery` become the age-0 refusal ("a tree grows at least one cycle"), checked in TS and natively with the whole family.
  - The catalogue check now counts 8 presets.
  - The spruce and birch threw on the 40,000-leaf ceiling. The fixture now shrinks the trees by age 8, because its envelope, attractor and maxNodes rows are read by no build.
- **Deleted (5), because no build reaches the subject:**
  - `caps explicitly partial`: only the scaffold sets a cap.
  - `the beech in work is not listed` and `the beech in work is not built by name`: the beech ships, and `IN_WORK` is empty.
  - `refuses crookedness`: a scaffold habit row the build never judges. fn-198 `#[ignore]`d the Rust build/validate agreement test for the same reason.
  - `an empty tree has no field`: no family grows an empty tree.

Decision for the host to see: the scaffold skeleton rows (`envelope`, `attractors`, `step`, `growth.maxNodes`, `habit`) are still accepted on the wire and ignored. I read that as the recorded fn-198/fn-288 state, not a new defect, and deleted the checks whose subject only those rows held.

The CI job has no wasm-bindgen step, because the entry builds only the core Wasm (`build-wasm.mjs`), never the render glue. The integration reviewer agreed.

Tests (measured):
- `npm run test:bindings`: exit 0, reaching its final `console.log`.
- Gate `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`: exit 0, with 1,401 passed, 0 failed and 111 ignored across 146 binaries.
- `npm test`: 19 files, 170 tests passed.
- The workflow YAML passes `npx --yes yaml-lint`, and a Python parse lists the `bindings` job with no `needs` and no `if`.

stage: impl-review - ran (codex, three lenses SHIP; two P2 findings fixed after the verdict in ed1cfa7bd: the native age-0 check now sends a whole family, and the evidence no longer calls `skeleton.twigs` dormant)
Tier: (none given) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7dccbda22751f22578d98626ea1b9b9c9ab63717, 21d050d9aefbeffa8cf1c93bcc725301ad25f2db, ed1cfa7bd4b55f563d0db9c5167f96d12c741aeb
- Tests: npm run test:bindings, telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1401 passed, 0 failed, 111 ignored), npm test (170 passed)
- PRs: