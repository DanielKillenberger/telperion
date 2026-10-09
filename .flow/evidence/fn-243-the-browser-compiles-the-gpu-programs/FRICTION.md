# fn-243 friction

## 2026-10-09, task 1 (worker)

- **What:** running one new vitest file, `src/browser/render.test.ts`, three mocked tests that take 6 ms.
- **What slowed it:** vitest's `globalSetup` (`harness/release-binaries.setup.ts`) builds two native release binaries before any test file runs, whichever files are selected. On this worktree's cold `target/` it took 1 min 30 s, for a run that needed neither binary.
- **Cost:** about 2 minutes of wall time for a 6 ms test.
- **What would remove it:** build the binaries only when a selected test file needs them, for example in those files' own `beforeAll` or in a vitest project limited to them.
