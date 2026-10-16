# The CPU entry builds every shipped preset or refuses it by name

## Goal & Context

`scripts/test-dist.mjs` (the package as Node loads it; not in CI) fails on F4 before fn-257 (checked at 9241e990c, 2026-10-16): `TreeEngine.build(TELPERION, seed 7, { surface: true, foliage: true })` in `dist/telperion.js` throws "Tree core: surface allocation". The package's main entry is the README's first example, and docs/principles.md asks that every shipped preset's every artifact builds through each package entry. Unknown: which presets, ages and outputs fail in the Wasm entry, and why (Wasm's 4 GiB memory, one allocation's cap, or a sizing estimate).

## Requirements

- **R1 (probe first):** every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark; the cause of each failure traced to code and written down before any fix.
- **R2 (host decides from R1):** either the entry builds them (a sizing or memory fix, measured), or the limit is a named, documented refusal that the package's README and types state, with the owner's word if a shipped preset is refused.
- **R3:** `test:dist` runs in CI's package job and passes; gate and npm test green.

## Boundaries

Release item (fn-237). Found by the host on 2026-10-16 while checking fn-257's report.
