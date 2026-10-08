# fn-235 R1: the step that never completes

Native, the browser's build route (`Generator::new_async`, `prepare_async`
resident) on the page's limits (`maxBufferSize` 4,294,967,292,
`maxStorageBufferBindingSize` 2,147,483,644, WebGPU defaults beside), one
tree a process under `timeout -s KILL`, `TELPERION_TRACE=1`, RTX 3080,
driver 610.57.04, 2026-10-09 (`hang_tests::browser_route`).

| Run | Driver cache | Result |
|---|---|---|
| beech, spruce, oak at 5, 10, 20; beech and oak at 40, 80 (F4 presets) | warm | built in 0.01 to 0.90 s (oak at 80: 11.35 s, CPU) |
| birch at 5, 20, 40, 80, the page's own JSON (fn-213 round 14, scratch branch only) | warm | built in 0.13 to 2.65 s |
| birch at 20, page JSON | cold (`__GL_SHADER_DISK_CACHE=0`) | built in 108.2 s, 0.9 s of it after the first submit |
| beech at 20, F4 | cold | built in 149.8 s; pipeline `grow` made in 144.7 s, `girths` 1.5 s, the other 55 under 0.6 s |

The step is not a submission: it is making the chain grower's `grow`
compute pipeline (`chains::Grower::new`, the `zones` contract with
`chain`, `presence`, `assign`, `carry`, `apex`, `sleep` and `shed` WGSL),
which the NVIDIA compiler takes 100 to 150 s over on a cold cache. Every
species on the fine path makes it on its first tree. Inferred, not run:
Chrome compiles it in its GPU process (its own cache, Tint's SPIR-V), so
the page sits at "building..." while the GPU process is busy until the
watchdog kills it.

Red test: `hang_tests::every_pipeline_is_made_cold_within_a_second`
(child process, cache off): `made cold over 1000 ms: [("grow", 98666.0),
("girths", 1290.0)]`.

## Host decisions (2026-10-09)

1. **Measure first, then slim `grow`.** Splitting it into extra passes is a last resort.
   - **Measure:** before choosing a fix, find what makes the NVIDIA compiler slow on `grow`. Bisect natively with the cache off, stubbing function bodies or call sites one at a time, and record each part's share of the cold compile in REPRO.md. Likely suspects are inlined copies of the contract-A fixed-point transcendentals, loops the compiler unrolls, and very large switch or select chains.
   - **Fix:** remove the cause in the same WGSL and the same single pipeline. Examples: keep a loop rolled, call a shared function instead of repeating its expansion, take a table instead of a long select chain.
   - **Unchanged:** contract A parity and the CPU-versus-device tests stay green, and outputs stay bit-identical.
   - **If only a split meets the bound,** stop and report with the measurements. More passes is the host's call; the owner asked that GPU work avoid added passes.
2. **Cold-build bound.** On the RTX 3080 with the driver cache off, no pipeline takes more than 2 s to build, and the first tree's whole cold build takes no more than 5 s. `girths` at 1.3 to 1.5 s passes. `every_pipeline_is_made_cold_within_a_second` is renamed to match and turns green on the fix. Where they are not already, the browser route creates its pipelines asynchronously (`createComputePipelineAsync` through wgpu's web backend), so that compilation does not hold up the page's frame loop.
3. **R3 admission budget.** `BROWSER` drops from 8.5 GiB to 6 GiB. That is the reference 10 GiB card, less 3 GiB held by other applications (the supported condition), less 1 GiB for textures, pipelines and the driver. One shipped tree at 80 fits within it: the spruce's measured peak is 5.48 GiB. Record the derivation beside the constant. A build over the budget is refused by name, and the previous tree is kept.
4. **R3 device loss.** On a loss the harness shows a named error and keeps the previous tree's parameters. On the owner's "retry build" it requests a fresh adapter and device and rebuilds once. It never retries automatically, so it never loops. A test forces a device loss (`device.destroy()`) and checks this.
5. **R4 is unchanged.** Then run R5's native checks, the gate, npm test and impl-review. The browser check waits for the owner, who may need to reboot first.
