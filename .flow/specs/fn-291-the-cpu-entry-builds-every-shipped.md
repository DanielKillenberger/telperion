# The CPU entry builds every shipped preset or refuses it by name

## Goal & Context

`scripts/test-dist.mjs` (the package as Node loads it; not in CI) fails on F4 before fn-257 (checked at 9241e990c, 2026-10-16): `TreeEngine.build(TELPERION, seed 7, { surface: true, foliage: true })` in `dist/telperion.js` throws "Tree core: surface allocation". The package's main entry is the README's first example, and docs/principles.md asks that every shipped preset's every artifact builds through each package entry. Unknown: which presets, ages and outputs fail in the Wasm entry, and why (Wasm's 4 GiB memory, one allocation's cap, or a sizing estimate).

## Requirements

- **R1 (probe first):** every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark; the cause of each failure traced to code and written down before any fix.
- **R2 (host decides from R1):** either the entry builds them (a sizing or memory fix, measured), or the limit is a named, documented refusal that the package's README and types state, with the owner's word if a shipped preset is refused.
- **R3:** `test:dist` runs in CI's package job and passes; gate and npm test green.

## Boundaries

Release item (fn-237). Found by the host on 2026-10-16 while checking fn-257's report.

## The owner's decision on R2 (2026-10-16)

"we shouldn't refuse by name. If we have params we can't build we should refuse those cleanly, but ideally we don't have that." Every shipped preset at every seed builds through the CPU entry; a refusal is reserved for parameters that truly cannot be built, predicted before allocation and named. So R2 is a memory fix, chosen by the host from PROBE.md:

- **R2a (the build's peak):** what the surface stage does not read is freed before it allocates (the grown tree's working state, the foliage plan's scratch), and the wood's intermediate buffers are written once at their final size, not grown or copied.
- **R2b (compact wood):** the wood's per-vertex data in the smallest layout the drawing allows (for example oct-encoded normals, bark coordinates in 16-bit), each field's precision bounded by a test at the drawn tolerance; if the package's output format changes, the TypeScript types, README and a migration note change with it.
- **R2c (proof):** all 24 cases of PROBE.md build with the Wasm high-water mark at or under 3.5 GiB (headroom), stills of Telperion and Laurelin unchanged to the eye, every other preset's surface byte-identical or within the bounded precision; the probe joins `test:dist`.
- Fewer sides on thin wood (PROBE.md option 3) changes the tree's look and is not taken here; if R2a-R2b cannot reach the bar, the run stops and reports the measured gap.
