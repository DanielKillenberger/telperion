# Friction, fn-319

## 28 Oct 2026, task .4 (parallel worker, worktree fn-319-t4)

- Doing: the baseline before the first edit (typecheck and the focused vitest files).
- Slowed by: a fresh worktree has neither the renderer glue (`src/browser/render/`, needed by `tsc`) nor `src/browser/telperion.wasm` (needed by `presets.test.ts` and `voxelize.test.ts`). The first vitest run took 113 s and failed three tests on ENOENT, which reads like a red baseline until the log is opened. Both had to be built through the heavy queue (`render:build` 33 s, `wasm:build` 17 s) before the baseline meant anything.
- Cost: about 4 minutes and one wasted 113 s suite run.
- Would remove it: the conductor building both generated modules once when it creates a task worktree, or the worker dispatch naming `npm run wasm:build && npm run render:build` (through `telperion-heavy`) as the first step for TypeScript tasks.
