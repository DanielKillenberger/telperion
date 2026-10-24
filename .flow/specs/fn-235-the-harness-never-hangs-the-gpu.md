# The harness never hangs the GPU

## Goal

On 2026-10-08 the owner could not open the harness (`npm run dev` on the birch branch, fn-213 round 14): every load took a long time, then reported "the GPU device was lost: A valid external Instance reference no longer exists", and Chrome's GPU process crashed until Chrome blocked WebGPU. The host reproduced it in headless Chrome on the RTX 3080: `?species=silver-birch&age=20` sat at "building..." for 60-90 s while the same tree builds natively in under a second; after one hang no new Chrome got a WebGPU adapter until a reboot. The kernel also logged NVIDIA "can't alloc VA space" errors when workers' GPU tests ran beside the browser. Host-created release blocker (2026-10-08): the harness is the owner's judging tool and the web package is the product. Harness and browser GPU path only.

## Decisions

Full record in `REPRO.md`. Host, 2026-10-09:
- Find the hang natively (a browser hang costs a reboot); a browser repro only after a native fix and with the owner's go.
- **First:** measure, then slim `grow` in the same single pipeline (a split is the host's call); 6 GiB browser budget (10 GiB card less 3 GiB for other applications less 1 GiB for textures, pipelines, driver; the spruce at 80 peaks at 5.48 GiB); a lost device is named and rebuilt once on the owner's retry, never looping.
- **Second:** a split of `move_on`/`grow_unit` allowed only if it adds at most 5 % build time (it measured 11-13 % on the beech, so not built). No async pipeline creation for now (wgpu 30 has no public API for it). Per-pipeline cold bound 2.5 s.
- **Third:** `grow` is a named exception at 20 s cold (cause: the apex loop's inlined 64-bit contract-A arithmetic). The budget counts the drawn tree: a build that fits only alone releases the drawn tree first; one that does not fit alone is refused by name with the drawn tree kept. R2's per-submission caps still required.

## Acceptance Criteria

- **R1:** The hang found natively on the browser's build path, each step logged, a standalone test red on it — met: the chain grower's `grow` pipeline took 92-145 s to compile on a cold NVIDIA cache (`hang_tests::browser_route`); no tree hung on a warm cache.
- **R2:** Every shader loop capped from its inputs; no submission of a shipped species at 80 longer than 1 s, asserted by a test — met (worst 177 ms, 256 dispatches, 2^23 workgroups).
- **R3:** A stated admission budget refusing by name with the previous tree kept; a forced device loss named and rebuilt on retry — met.
- **R4:** Device-limit GPU tests excluded from the default render suite unless asked — met.
- **R5:** The owner opens beech, birch and oak at 40 and 80 in the harness without a device loss; gates green — gates green; the browser check left to the owner after the host's go.

## Result

`grow` slimmed in one pipeline from 92-145 s to 13-17 s cold, output unchanged, held as the named 20 s exception; every other pipeline under 2.5 s; the first tree builds in 5 s besides `grow`, cold. Cold-compile timings read the machine's load (`grow` 12-45 s at load 7-40).

## Open

- R5's browser check (cold profile, beech/birch/oak at 40 and 80) is the owner's.
- If Chrome still loses the device on a first build: async creation or a less-inlining contract-A arithmetic, the host's choice.
- Branch `scratch-fn-235-birch` holds fn-213's birch for the repro only; never merge it.
- Twice the spruce at 80 exited 101 after the beech's; parallel GPU tests failed on VA-space exhaustion when other worktrees ran GPU tests outside the lock.

## Friction

- Cherry-picking fn-213's preset commits conflicted on evidence files F4 had deleted -> species branches whose preset commits touch only presets.
- No CLI prints a preset's page JSON (148 s vitest startup per dump) -> proposed `npm run family -- <species> <age>`.
- The hang only reproduced with `__GL_SHADER_DISK_CACHE=0`, after ~20 min of warm runs -> repros should turn the driver's shader cache off by default.
- Bisecting cold compiles on a loaded machine (±30 %) and cold bounds failing with load -> an idle machine or reserved cores for the GPU lock holder; format the three drifting files once on master.
- No async pipeline creation in wgpu 30 -> needs a host decision (raw `web_sys` call or a wgpu change).
- Other worktrees' GPU tests ran outside this session's lock -> one GPU lock for every worktree.

## Evidence

- `.flow/evidence/fn-235-the-harness-never-hangs-the-gpu/REPRO.md` (the repro, bisection, three host decision rounds; cited by `compile_size.rs`)
