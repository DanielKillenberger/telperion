---
satisfies: [R7]
---
# fn-112-publish-telperion-to-npm-with-a-release.2 The main entry loads its Wasm as files, not inlined text

## Description
TBD

## Acceptance
R7 of the parent spec is satisfied; judge this task against the spec's criterion directly.

## Done summary
R7: `dist/telperion.js` is JavaScript only (108,459 bytes, from 6,659,763). `src/browser/core.ts` and `render.ts` resolve `new URL(WASM, import.meta.url)` at run time from a constant, as the field entry does; the build scripts copy `telperion.wasm` (1,286,823) and `telperion-render.wasm` (1,813,306) beside the entry in `src/browser/` and `dist/`; wasm-bindgen runs with `--omit-default-module-path` because the glue's string-literal fallback was the last 2.4 MB of inlined base64. Declarations byte-identical; README names the two files; RESULTS.md carries the pack listing beside the old sizes and every check run.

Checks: typecheck and vitest green (123 tests, green baseline too); binding and field browser suites green on the dev server, where `TreeEngine.create()` resolves `telperion.wasm` with no source; a static server over `dist/` in headless Chromium fetched both Wasm files 200 beside `telperion.js` and built a tree; the harness check was `npm run test:render` on the RTX 3080, which built and drew all six presets, dials and views, then stopped on the Oregon white oak orbit's wall-clock p95 (30.00 ms vs 16.7 ms, GPU percentiles valid) while another harness was on port 5173 and the display was in use: recorded inconclusive on that assertion, not green. Gate `cargo test --profile ci --workspace --no-fail-fast` green once at the end, receipt f6c32e81-unittest. baseline: green (npx vitest run, pre-edit). R6 untouched.

Tier: session (host judgment)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: f6c32e819cfd804e504b9fc097252181d9f3e65b
- Tests: npm run typecheck, npx vitest run, npm run rust:test:wasm, node <scratch>/dist-check.mjs (static server over dist/, headless Chromium: both Wasm files fetched 200 beside telperion.js, Ordinary built), RENDER_EVIDENCE=<raw>/render-suite npm run test:render (harness drew all six presets on the RTX 3080; INCONCLUSIVE on the orbit wall-clock p95 assertion, 30.00 ms vs 16.7 ms on a display in use), npm pack --dry-run --json, cargo test --profile ci --workspace --no-fail-fast
- PRs: