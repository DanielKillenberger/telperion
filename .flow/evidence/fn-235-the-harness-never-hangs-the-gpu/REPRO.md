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

## Bisection of `grow`'s cold compile (task 1, 2026-10-09)

`hang_tests::make_grow` makes `grow` alone from a WGSL file, one a
process, `__GL_SHADER_DISK_CACHE=0`, RTX 3080. Load average 6 to 38
from other agents, so a single run is good to about ±30%; a stub that
removes code can make the compile slower (the driver's optimiser is
not monotone in the code it is given), so only large, repeated drops
are read as a part's share.

| Variant of `grow` | Cold make, s |
|---|--:|
| F4 as it was | 91.6 (earlier 98.7, 144.7) |
| no apex run at all (`run` called nowhere) | 0.1 |
| `run` called once instead of twice (grow's cycle loop and `woke`) | 28.8 |
| and fail's tail not reached | 11.2 |
| and move_on not reached | 17.9 |
| and grow_unit not called | 15.2 |
| and log2's fraction stubbed / exp2's stubbed | 18.9 / 23.8 |
| and the long-division loops kept rolled (opaque bound) | 73.4 (worse) |
| and `shr128` without its local array | 45.6 (worse) |
| and fail and move_on dispatched from `run`'s loop, not nested | 27.8 |

Every part's cost is the size of the one body the driver inlines `run`
into: everything outside `run` makes in 0.1 s, and the cost grows
faster than the body. The body holds 528 inlined copies of `mul32` and
380 of `shr64` (the 64-bit emulation under contract A's fixed point).

Committed, bit-identical (the chain parity test and every
`generation::fine` test green): `run` has one call site (a woken bud's
apex runs on the cycle loop's first pass), and `grow_unit` and
`abortion_after` one each (`advance` grows a failing unit and draws its
abortion threshold; `fail` keeps its tail). `grow` now makes cold in
13.2 to 16.7 s, from 92 to 145 s.

What is left, on that source:

| Variant | Cold make, s |
|---|--:|
| as committed | 13.2 to 16.7 |
| move_on not reached | 7.3 |
| grow_unit not called | 9.8 |
| neither | 2.3 |
| abortion_after not called | 18.3 (worse) |

So 2 s needs move_on and grow_unit both out of `grow`'s body, and even
then sits at the bound: only a split meets it. Taking one of moved_q's
three `expq` copies out gave 13.2 against 13.8 s, inside the noise, and
was reverted.

## Host decisions (2026-10-09, second)

1. **Split `move_on` and `grow_unit` out of `grow`, as their own pipelines, if the run cost holds.** The single-pipeline slimming is kept: it took `grow` from 92-145 s to 13-17 s with identical output. The measurement shows only the split brings `grow` near 2 s.
   - **Why the extra passes are allowed:** the owner's "no extra passes" was set for the light algorithm, and nothing short of this split reaches the bound.
   - **Condition:** the split is accepted only if the build time it adds, at 40 and 80 for beech, oak and spruce, is no more than 5 % against the slimmed single pipeline (best of 3, the same machine load stated). If it adds more, stop and report the numbers.
   - **Output:** stays identical, and contract A and parity stay green.
2. **No async creation for now.** With every pipeline at or under about 2 s, synchronous creation stays well under Chrome's GPU watchdog. The wgpu API gap is recorded in FRICTION.md and fn-235's report, and is not built around.
   - **The bound:** at most 2.5 s per pipeline, which gives the measured 2.3 s its noise margin on this loaded machine. The first tree's cold build stays at 5 s or under. Rename the test to match.
3. **Decisions 3 and 4 go ahead now,** whatever happens with point 1: the 6 GiB budget and the device-loss handling. After them come R4, R5's native checks, the gate, npm test, impl-review and `flowctl done`.

## The split's cost (task 1, 2026-10-09, after the second decisions)

`grow_unit` and `move_on` sit inside each chain's sequential loop: the
sleepers `grow_unit` lays are read by `wakes` and `asleep_from` in later
cycles of the same chain, its phytomers by `relay_node` and the carry,
and `move_on`'s result decides the apex loop's next step. As their own
pipelines, every chain would have to stop before each call, keep its
state (counters, the relay stack, the loop positions, `advance`'s
locals) in a buffer, and resume after the other pipeline ran: one round
of dispatches a growth unit at least.

The rounds' least count, from the CPU reference's chains (seed 1):
the most growth units in one chain is 40 (beech 40, spruce 40), 67
(beech 80) and 80 (spruce 80); the beech at 80 dispatches `grow` 33
times a build, the oak not at all (it builds whole on the CPU).

The floor alone, measured: each `grow` region followed by `2 x age`
dispatches of a kernel that touches one word a chain (no state kept,
no `move_on` rounds, so less than the split would cost), warm cache,
best of 3, twice, load average 6 to 7:

| Tree | Slimmed single pipeline, ms | With the floor, ms | Added |
|---|--:|--:|--:|
| beech 40 | 166 / 170 | 188 / 190 | +12 to +13 % |
| beech 80 | 657 / 653 | 734 / 728 | +11 to +12 % |
| spruce 40 | 330 / 350 | 334 / 340 | noise |
| spruce 80 | 1,410 / 1,407 | 1,359 / 1,486 | noise (±5 %) |
| oak 40 | 827 / 880 | 860 / 798 | none (no `grow`) |
| oak 80 | 10,224 / 9,968 | 10,384 / 10,226 | none (no `grow`) |

The beech pays more than 5 % for the dispatches alone, before any state
is kept, so the split fails decision 1's condition and was not built.
`grow` stays one slimmed pipeline at 13 to 17 s cold, over the 2.5 s
bound; `every_pipeline_within_2_5_s_and_the_first_tree_within_5_s_cold`
stays red on it.

## Host decisions (2026-10-09, third)

1. **`grow` is a named exception to the cold-compile bound.**
   - **The bound:** at most 20 s natively cold. Its cause and measurements are recorded beside the test: the apex loop's inlined copies of the 64-bit arithmetic behind contract A, and the split's measured 11-13 % run cost on the beech.
   - **Every other pipeline** stays at 2.5 s or less, and the test turns green.
   - **What decides it:** the owner's browser check, with a fresh profile and a cold cache. Chrome compiles through its own shader translator, so the native time is not Chrome's time.
   - **If Chrome loses the device** on the first build, the next step is either async creation or a contract-A arithmetic form that inlines less. The host decides which; the worker does not start either.
2. **The budget counts the drawn tree, so make room before refusing.** A build that fits the budget on its own, but not beside the drawn tree, first releases the drawn tree, then builds. The screen is empty while it builds, and the status says so. A build that does not fit even alone is refused by name, and the drawn tree is kept. This is one stated rule, not a fallback: the tree is either built or refused. A test covers each case.
3. **R2's per-submission caps are still required.** Every shader loop gets a cap derived from its inputs, and no single submission of a shipped species at 80 may run longer than 1 s on the 3080. A test asserts the caps and the measured worst submission.

Then run impl-review and `flowctl done`. After that, the host asks the owner for the browser check.
