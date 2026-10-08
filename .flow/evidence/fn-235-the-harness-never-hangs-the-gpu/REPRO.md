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
