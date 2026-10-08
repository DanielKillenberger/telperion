# fn-235 friction

## 2026-10-09, task 1: bringing the birch onto a scratch branch

- Doing: cherry-picking fn-213's preset commits onto a scratch branch for the native repro, as the dispatch asked.
- Hindered: every preset commit also edits fn-213's evidence files, which F4 has deleted, so each pick stopped on a modify/delete conflict; the picked `birch.rs` did not compile against F4's `PaState` and `Form`, and the picked `presets.generated.ts` kept a retired field (`shortShootLeaves`), so the page's JSON was refused until fn-213's generated TS and harness were applied whole.
- Cost: about 15 minutes and three rebuilds.
- Would remove it: a species branch whose preset commits touch only the preset files, or a repro that takes the species branch's own generated family JSON as input.

## 2026-10-09, task 1: dumping the page's family JSON

- Doing: getting the exact family JSON the harness sends (`familyJson(presetToParams(presetById(id)))`) for the native repro.
- Hindered: no CLI prints it; a one-off vitest file did, and each vitest run took 148 s of startup for a 4 ms test.
- Cost: about 5 minutes of waiting, twice.
- Would remove it: a script that prints a preset's page JSON at an age (`npm run family -- silver-birch 20`).

## 2026-10-09, task 1: the hang is a cold pipeline compile

- Doing: running the browser route natively, one tree a process, for F4's beech, spruce and oak at 5, 10 and 20 and the birch at 5, 20, 40 and 80.
- Hindered: none of them hung, because the NVIDIA driver's disk cache already held every kernel from earlier test runs; the hang only showed with `__GL_SHADER_DISK_CACHE=0`, after about 20 minutes of warm runs that could not have reproduced it. A first try with naga's bounds checks off looked like a new hang and was the same cold compile (different SPIR-V, so a cache miss).
- Cost: about 30 minutes, and each cold run 100 to 150 s.
- Would remove it: a repro that turns the driver's shader cache off by default, so a native run sees what a fresh browser profile sees.
