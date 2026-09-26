# fn-160 friction

## 2026-09-26: the field wasm budget surfaced only after the gate

Doing: checking `node scripts/artifact-budgets.mjs` after `npm run build`, as the
dispatch asked, once the workspace gate and `npm test` were green.
Hindered: `dist/telperion-field.wasm` came out at 380,932 bytes against a
362,000 budget (base 356,393, measured in a second worktree with its own
`target/`). Nothing on the gate or in `npm test` measures the artifact, so the
size cost of routing presets through the catalogue's setters showed up after
the gate had already run, and measuring the base took a second release build.
Cost: about 5 minutes and a second wasm build.
Would remove it: a budget check that runs on the field module alone, fast
enough to sit beside the core tests, or the budgets recorded per commit so the
base size needs no rebuild.
