---
satisfies: [R4]
---
# fn-319-the-release-audits-slice-lands-before.5 compileField compiles once, and the CI key reads the parameter reference

## Description
Restore 0.1.4's memo on the default `compileField()` and add the parameter reference's two files to the CI key and the workflow filter.

**Size:** S
**Files:** `src/field/index.ts`, a field test, `scripts/ci-key.mjs`, `scripts/ci-key.test.mjs`, `.github/workflows/tests.yml`, `docs/field-package.md`
**Touches:** [src/field/index.ts, src/field/*.test.ts, scripts/ci-key.mjs, scripts/ci-key.test.mjs, .github/workflows/tests.yml, docs/field-package.md]

### Approach
- Follow `git show v0.1.4:src/field/index.ts` lines 52-62: a module-level promise for the default source, cleared on rejection; a `Module` source returned as is; an explicit source compiled uncached. Keep the `new URL("./telperion-render.wasm", import.meta.url)` literal for bundlers. Test by mocking `WebAssembly.compile` or `wasmSource` as `src/field/device.test.ts` mocks `../browser/module`.
- `crates/telperion-core/tests/parameter_reference.rs:25,30` reads both files: add both to `CORE_TESTS` (`scripts/ci-key.mjs:29-35`). `crates/telperion-jev/src/runner/tune.rs:58,145` reads `docs/parameters.md`: add it to `rust-jev`. The `node` suite already covers `src`.
- `ci-key.test.mjs`: one test per reading suite that a changed byte in each file changes the key, in the existing style.
- `.github/workflows/tests.yml:15-37` excludes `docs/**` and `**.md`: allow `docs/parameters.md` under both `push` and `pull_request`. Read `scripts/release-workflow.test.mjs` first in case it pins the workflow.
- Fix the `compileField` wording in `docs/field-package.md:95-98`.

### Investigation targets
**Required:**
- `src/field/index.ts:95-125`
- `scripts/ci-key.mjs`, `scripts/ci-key.test.mjs`
- `.github/workflows/tests.yml:1-40`

### Acceptance
- [ ] Two concurrent default calls resolve one module; a rejected compile is retried
- [ ] A change to either file changes every reading suite's key (tests)
- [ ] The workflow runs on a change to `docs/parameters.md`

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
