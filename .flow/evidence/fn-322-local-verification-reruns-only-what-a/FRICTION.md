# fn-322 friction

- 2026-11-01, narrow vitest: running one `scripts/*.test.mjs` file goes through `vitest.config.ts`'s global setup, which builds two release binaries under fat LTO (`harness/release-binaries.setup.ts`) though a script test reads neither. Cost here: avoided with a scratch config outside the repo, about 2 minutes to find. What would remove it: a `vitest` project or environment switch that keeps the setup for harness tests only.
- 2026-11-01, shell hook: `dcg` refuses a redirect to a path held in a variable, so scratch files go through a literal path or the Write tool. Cost: one retried call, under a minute. Local setup, reported only.
