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
`compileField()` with no source now compiles the module beside the entry once, as 0.1.4 did. Concurrent and repeated calls share one promise, and a rejected compile clears it so the next call retries. A `Module` is returned as given, and an explicit source is compiled on every call. The `new URL("./telperion-render.wasm", import.meta.url)` literal is kept.

The CI key now reads the parameter reference:
- `rust-core` lists `docs/parameters.md` and `src/browser/parameters.generated.ts`, which `parameter_reference.rs` reads.
- `rust-jev` lists `docs/parameters.md`, which `tune.rs` reads.
- `node` already covered `src`.

The tests workflow's `push` and `pull_request` path filters now include `docs/parameters.md`. `docs/field-package.md` now says what `compileField` caches.

- Tests:
  - `src/field/compile.test.ts` (new) stands in for `wasmSource` and spies on `WebAssembly.compile`. Its cases: two concurrent default calls and a later one resolve one module from one compile; a rejected compile is retried; a Module is returned as given; an explicit source is compiled on each call.
  - Measured red on the base: 3 compiles where 1 was expected. That test is committed alone, ahead of the fix.
  - `scripts/ci-key.test.mjs` gains one `it.each` row per reading suite (`rust-core`, `rust-jev`, `node`). Each row checks that a changed byte in each file it reads changes the key.
- The workflow filter is not covered by a test. `scripts/release-workflow.test.mjs` pins `release.yml` only. The new entry follows the existing re-include entries.

baseline: green (`npx vitest run scripts/ci-key.test.mjs src/field`, 24 passed)

Gates, all measured:
- `npx vitest run scripts/ci-key.test.mjs src/field`: 30 passed, 5 files.
- `tsc --noEmit`: clean.
- `node scripts/ci-key.mjs rust-core`, `rust-jev` and `node`: each prints a key, so the new inputs exist in HEAD.

`gate classify`: FULL, because `.github/workflows/tests.yml` changed. Focused Quick commands only; no cargo build was needed.

stage: impl-review - skipped(policy: parallel-wave - conductor owns the gate)

Tier: session model (model: claude-opus-5-5)

Integrated on the spec branch. Review: codex three lenses SHIP, no findings (rid 7e2f4686).
stage: plan-sync - skipped(config: planSync.enabled=false)
## Evidence
- Commits: 93d4ad9263e274d0950e172c126ad46cf6a18c6a, 6ea5bf24b11f63bd984f4dfb0cbfc9687029739d
- Tests: npx vitest run scripts/ci-key.test.mjs src/field, npx tsc --noEmit -p tsconfig.json, node scripts/ci-key.mjs rust-core|rust-jev|node
- PRs: