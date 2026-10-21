# fn-302 friction

## 2026-10-21 (conductor, Unknown 1 measurement)

Doing: R2's paired cost of the hierarchy rounds at their bound on beech 40, palm 12, spruce 40 and the walk frame. Hindered: `build_time` times presets only (no blend frame) and has no way to toggle a limit, so a paired run needs two checkouts with two target/ builds or a hand-written probe. Cost: about 5 minutes writing a test-only toggle and an in-process alternating timer. Would remove it: a `build_time` that takes a walk frame (from, to, t) and a test-only limits override, so a limit change is timed in one process, alternating sides. Also: a refused base cannot be paired, so the frame's row compares a refusal at cycle 12 against a whole build.

## 2026-10-21 (conductor, design survey)

Doing: the host design's survey of gens(c) = deep(c-1), the rounds rise and the units rounds' headroom over 1,587 device builds (1,533 walk frames and the presets at their default age and 12 and 40). Hindered: no committed tool reads every census of a device build through `Generator::prepare`; `grew.censuses` is reachable only from `cycle/` tests that grow below the generator, and the film-walk frame list is private to `film_walk_tests`. Each question about the census is a new scratch tap plus a 14-minute sweep, and a statistic missed the first time (the units rounds' effective headroom) cost a second 14-minute sweep. Would remove it: one ignored survey test that taps every census read by cycle and prints the stepped limits' rises and headroom per limit, over the sweep's frames.

## 2026-10-21 (conductor, base digests)

Doing: R3's base, every film-walk frame's digest line on bb20f3877's code. Hindered: a digest line reads back each frame's instances and wood, about 3 s a frame against 0.55 s for a build alone, so the base sweep takes about 75 minutes, and the change's side costs the same again. Would remove it: a device-side hash of what the renderer receives (one small readback a build), or a sweep digest that reads only counts and a hash computed on the device.

Correction (same day): the whole base run (preset digests, palm digests and 1,533 frames) took 1,081 s, about 0.7 s a frame once the presets were done, not the 75 minutes estimated from its first minutes. The cost was 18 minutes, and the entry's remedy matters less than it said.

## 2026-10-21 (conductor, task 1 impl review)

Doing: finalizing task 1's three-draw codex review, all SHIP with no findings. Hindered: `impl-review-fanout-finalize --merge-plan` with an empty keep list fails ("cannot parse draw correctness") when a draw has no numbered findings, and the first call printed a null verdict with exit 0. The fix was a hand-written `--merged-file`. Cost: about 3 minutes. Would remove it: finalize accepting an all-SHIP, no-findings round with an empty plan.

## 2026-10-21 (conductor, task 1 gate)

Doing: task 1's one gate run (`telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`): 1,472 passed, 1 failed, 116 ignored, about 50 minutes. Hindered: `every_device_preset_builds_the_same_leaves_each_time` was refused at the Telperion with "the device refused the build's settle place of 239 MiB: Out of Memory". Desktop applications held about 6 of the GPU's 10 GiB while the gate ran, and the test passes alone in 21 s with every preset alike. Cost: a gate that cannot be read green without a rerun, about 3 minutes. Would remove it: a gate that admits device tests only below a free-VRAM floor, or names device memory pressure apart from a defect. This is a local setup problem, so it is reported, not specced.
