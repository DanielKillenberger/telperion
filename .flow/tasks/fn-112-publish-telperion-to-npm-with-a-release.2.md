---
satisfies: [R7]
---
# fn-112-publish-telperion-to-npm-with-a-release.2 The main entry loads its Wasm as files, not inlined text

## Description
Ship the main entry's generator and renderer Wasm as files beside the JavaScript instead of inlined base64.

## Acceptance
Spec R7.

## Done summary
`dist/telperion.js` 6,659,763 -> 108,459 bytes; `core.ts` and `render.ts` resolve `new URL(WASM, import.meta.url)` at run time; build scripts copy `telperion.wasm` and `telperion-render.wasm` beside the entry; wasm-bindgen `--omit-default-module-path`. Declarations byte-identical. Typecheck, vitest (123), binding and field suites green; a static `dist/` server fetched both Wasm files and built a tree. Harness render suite drew all six presets but was inconclusive on one orbit wall-clock assertion on a contended display. Gate green.

## Evidence
- Commits: f6c32e819cfd804e504b9fc097252181d9f3e65b
- Tests: npm run typecheck; npx vitest run; npm run rust:test:wasm; npm pack --dry-run --json; npm run test:render (inconclusive on orbit p95); cargo test --profile ci --workspace --no-fail-fast
- PRs:
