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

## 2026-10-09, task 1: bisecting a cold compile on a loaded machine

- Doing: timing `grow`'s cold compile per stubbed variant (host decision 1).
- Hindered: each run takes 15 to 90 s, the load average swung from 6 to 38 under other agents' builds, so one run is only good to about ±30%; and stubbing code out sometimes made the driver slower, so small effects could not be read at all.
- Cost: about 40 minutes, 20 cold compiles.
- Would remove it: an idle GPU and CPU while measuring, or a reserved core set for the GPU lock's holder.

## 2026-10-09, task 1: no async pipeline creation in wgpu

- Doing: host decision 2, creating the browser's pipelines with `createComputePipelineAsync`.
- Hindered: wgpu 30's public API has only the synchronous `Device::create_compute_pipeline`; the async binding exists in its private `webgpu_sys` module and is not reachable from this crate without going around wgpu.
- Cost: 5 minutes to find; the item is not done.
- Would remove it: a host decision on how to reach it (a raw `web_sys` call beside wgpu, or a wgpu change).

## 2026-10-09, task 1: cold-compile bounds on a shared, loaded machine

- Doing: holding `grow` to its 20 s cold exception and the first tree to 5 s besides it.
- Hindered: the load average ran 7 to 40 from other agents' builds; the same `grow` source made in 12 to 45 s across runs, so the cold test passes or fails with the machine's load, not the change. `cargo fmt -p telperion-render` also rewrites three files nobody touched (an example and two integration tests) on every run.
- Cost: about 30 minutes of repeated cold compiles; a stray-diff revert after each format.
- Would remove it: cold-compile bounds measured on an idle machine (or the test reading the load and saying so), and the three files formatted once on master.
