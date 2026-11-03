# fn-329 protocol

Frozen 3 Nov 2026, before any number below is read; the probe's trial on the first four trees only checked that it runs.

- **Build:** the package's `dist/` from `npm run build` at `11bd70e49` (fn-328's rows), `telperion.js`, `presets.js` and `telperion.wasm`.
- **Runtime:** Node v26.12.1 (V8, the engine of Chromium), AMD Ryzen 9 5950X, one thread pinned with `taskset -c 10`, inside `telperion-heavy`, nothing else heavy running.
- **Probe:** `probes/wasm.mjs`. Every preset at its preset age, at fidelity 1 and then 0, seeds 1, 1234 and 5678. Each tree is built once cold, then five times warm; the time is the median wall clock around `build(family, { field: { n: 36 } })`, with the package's own stage timings as medians beside it.
- **Native beside it:** the same trees' cycle time, one thread pinned, `--profile ci`, through `fidelity_rows` on seeds 1, 1234 and 5678 with each preset's own coarse row, five repeats less the first. Presets without coarse rows grow the same at both fidelities, so only the Wasm read is given for them.
- **0.1.4's times:** AUDIT-2026-10-26.md in fn-237's evidence, a browser on this machine, grid 36: oak 233 to 260 ms, birch 239 to 304 ms, spruce 203 to 212 ms. A preset is within 0.1.4's time when its median is at or under the range's low end, and the high end is reported beside it.
- **Not this:** R2's full benchmark (a browser, both tarballs, the grove plan, WebGPU on and off) stays for a later child.
