# The release branch's CI is green, and the local gate sees what CI sees

## Goal & Context

PR #159's CI fails two jobs the local gate never runs. This is run 40087469094, on the release branch pushed 2026-10-24.

- **`core without geometry`:** `telperion-core`'s lib tests do not compile with the geometry feature off: `error[E0425]: cannot find function bloom_element in this scope`.
- **`package`:** `node scripts/artifact-budgets.mjs` exits 1. `scripts/artifact-budgets.json` still names `dist/wasm-source.js`, but vite now emits that shared chunk as `dist/module.js` (found by fn-313, 2026-10-24). Every ceiling that is checked holds; only the missing entry fails.

The owner wants the release merged, and a red PR cannot be merged with confidence.

## Requirements

- **R1:** the no-geometry build's tests compile and pass.
  - Find which merge broke them (`git log -S bloom_element`).
  - Fix the code so that `bloom_element`, or its test, is gated with the feature.
  - Run CI's command for that job as written in `.github/workflows/tests.yml`.
- **R2:** the budget entry names the chunk the build emits.
  - Either rename the entry to `dist/module.js`, carrying its ceiling and reason, or make vite name the chunk `wasm-source` again.
  - Choose whichever keeps the published file names stable for consumers. Check which name the package's exports and `scripts/test-dist.mjs` use, and say why.
  - The check passes on a fresh `npm run build`.
- **R3 (the cause):** neither failure could reach CI if the local gate ran these two checks.
  - Add both to `scripts/gate.py`'s pre-run guards, beside fn-306's two guards, at their measured cost.
  - If either costs more than about a minute, run it beside the gate instead and say so in AGENTS.md's gate line.
- **R4:** run every job in CI's `tests.yml` locally on the fixed branch through its own command, or state why one can't run locally. The gate and `npm test` are green, with a codex review dispatched inside telperion-heavy.

## Boundaries

- No ceiling is raised.
- Joins the release (fn-237).
