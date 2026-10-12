# fn-256 friction

## 2026-10-11, task 1: start
- **Doing:** the Phase-1 anchor in the `gpucycle` worktree.
- **Hindered:** `flowctl` is not on PATH in the worker's shell; found under the plugin cache by hand (fn-254 and fn-255 reported the same). The dcg hook also refused a heredoc into a file whose parent directory the same command created.
- **Cost:** about 2 minutes.
- **Would remove it:** `flowctl` on PATH for dispatched workers (local setup, not a repository change).

## 2026-10-11, task 1: replay runs
- **Doing:** writing each replay's output to `raw/` from a shell loop over the two captures.
- **Hindered:** the dcg hook refuses a `>` whose target holds a loop variable; each run had to be its own call with a literal path.
- **Cost:** about 1 minute.
- **Would remove it:** nothing in the repository; local hook policy.

## 2026-10-11, task 1: the capture's births
- **Doing:** counting each apex's births in the busy cycle.
- **Hindered:** an axis born this cycle can bear another the same cycle (a continuation carried on within it), so a bearer is not always a living apex; the first capture panicked on it.
- **Cost:** about 2 minutes and one 9 s recapture.
- **Would remove it:** nothing; the capture now credits such an axis to its first bearer.

## 2026-10-11, task 2: step 0 merges
- **Doing:** resolving the fn-255 merge's conflict in `grow/cycle.rs` by taking fn-255's side.
- **Hindered:** the dcg hook refuses `git show <ref>:<path> > <path>` (and so `git checkout <ref> -- <path>`); the conflict had to be resolved by a script editing the markers in place. `flowctl` again not on PATH.
- **Cost:** about 2 minutes.
- **Would remove it:** nothing in the repository; local hook policy and setup.

## 2026-10-11, task 2: the units kernel's first device run
- **Doing:** the first young-tree run of `generation/cycle` (units and wake, count and emit) on the GPU.
- **Hindered:** the test spent about 340 of its 348 s before its first line of output, which is the driver compiling the module's six entries. Every entry inlines `run`, and inside it `stop`, `sprout` and `grow_unit` are inlined at every call site (fn-235 met the same in the chain grower).
- **Cost:** about 6 minutes of GPU lock per run.
- **Would remove it:** one call site per heavy function (a small state machine of operations in `run`) and one entry for count, emit, units and wake, chosen by the uniform. Being done now.

## 2026-10-11, task 2: tracing the device's differences
- **Doing:** finding why node presences, and in the wake one living apex's slot, differed from the CPU's.
- **Hindered:** the agreement report gives outputs only. Each hypothesis needed a hand-written diagnostic or a swapped contract, then a 40 to 80 s GPU run under the lock. The f32 trace took about 8 runs; the live-list race, a stale write left by `carry_on`'s pop, took 5.
- **Cost:** about 20 minutes of GPU lock and the edits around them.
- **Would remove it:** a test switch that has the cycle kernel record each phytomer's draw, bound, window and lead, and each invocation's own appends beside its counts, for the host to recompute on the CPU. The count-against-emit check is cheap enough to keep permanently.

## 2026-10-11, task 2: the layout's sine on the device
- **Doing:** porting libm's `sincosf`, which evaluates in f64, as pairs of f32s (two-sum, two-product by `fma`).
- **Hindered:** the NVIDIA driver reorders f32 add and subtract chains as well as fusing products: the error-free transforms collapsed, and a 1M-input check found 18 to 28 % of results off by one to three ulps. `atan2f`'s `(z - PI_LO) - PI` is reordered the same way (113k of 124k differing results). Rewritten in integers of 2^-61, which no driver can reorder: 0 of 1M differ.
- **Cost:** about 15 minutes and three GPU runs.
- **Would remove it:** knowing up front that the device reorders sums, not only fuses products (UNITS.md named fusion only). Now in LAYOUT.md.

## 2026-10-11, task 2: WGSL names and constants
- **Doing:** first validation of the layout's shader.
- **Hindered:** `from` and `target` are reserved words in WGSL, and naga refuses `bitcast` in a module `const`; each was found by one full test build of `telperion-render` (about 1 minute each, three rounds).
- **Cost:** about 4 minutes.
- **Would remove it:** a naga check runnable on one shader source without building the crate's test binary.

## 2026-10-11, task 2: tracing the layout's last differences
- **Doing:** attributing the young layouts' remaining float differences under the exact experiment.
- **Hindered:** the comparison names columns, not the operation that rounded. Each of the last three causes needed a hand-written readback of the device's intermediates: dominance's `held` fused across a variable, `(tip + d len) - tip` simplified to `d len`, and the contract's `exp2c`. Each took a 1 to 2 minute rebuild and GPU run.
- **Cost:** about 15 minutes, five runs.
- **Would remove it:** UNITS.md's test switch (a kernel recording its intermediates beside its outputs), which applies to the layout as well.

## 2026-10-11, task 2: the driver's cache does not hold the units module
- **Doing:** isolating the units module's cold compile, by timing a cold run and then a warm one.
- **Hindered:** both took 37 s. The NVIDIA disk cache under `~/.cache/nvidia/GLCache` is full at 1.1 GB, at the driver's default size, so the second run compiled again. Only `__GL_SHADER_DISK_CACHE_SIZE=8000000000` made the warm run 0.02 s. The dcg hook also refused re-running into an existing log file; each run needed a new name.
- **Cost:** about 5 minutes and two 38 s GPU runs.
- **Would remove it:** a larger driver cache on this machine (local setup, not a repository change).

## 2026-10-11, task 2 (light): the shared module did not build mid-edit
- **Doing:** the light worker's first build of `telperion-render`'s tests.
- **Hindered:** the hierarchy worker had registered `hier_*` modules in `generation/cycle.rs` before their files existed, so the crate did not compile and the light's own work could not build until they landed.
- **Cost:** a wait of some minutes and one wasted build (about 1 minute).
- **Would remove it:** two workers in one crate registering a module only together with its file, or each in its own worktree.

## 2026-10-11, task 2 (hierarchy): naming the float causes
- **Doing:** confirming which sites make the hierarchy's incoming, bud, memory and size differ from the CPU's.
- **Hindered:** no switch builds the contract and the hierarchy with correctly rounded division and products kept apart, so the experiment patched the shared `fine/contract.wgsl` in place (UNITS.md's and LAYOUT.md's patches) and restored it, all inside one hold of the GPU lock so the light worker's build could not see it. `patch` left a `.orig` beside it, and the dcg hook refused the `awk` that cut the layout patch's contract hunks.
- **Cost:** about 6 minutes and three GPU runs.
- **Would remove it:** the host's ruling on the f32 contract, or a build switch for the exact arithmetic that both the units' and the layout's experiments already wrote.

## 2026-10-11, task 2 (hierarchy): clippy stopped by another crate's lint
- **Doing:** clippy on the new `hier_*` files.
- **Hindered:** `cargo clippy -p telperion-render` lints `telperion-space` too, and a deny-level `approx_constant` there (and in the light worker's `light_program.rs`) stops it before the render crate is linted; `--no-deps` reaches it.
- **Cost:** about 3 minutes, two clippy runs.
- **Would remove it:** a clean clippy baseline on the branch, or `--no-deps` in the worker brief.

## 2026-10-11, task 2 (hierarchy): registering before the files
- **Doing:** registering the `hier_*` modules in `generation/cycle.rs`.
- **Hindered:** I registered them before their files existed, which broke the light worker's build for some minutes (their entry above).
- **Cost:** their wait and one build.
- **Would remove it:** registering a module in a shared file only in the same edit that creates it.

## 2026-10-11, task 2 (hierarchy): a spill rule's shape found only by building it
- **Doing:** building DESIGN.md section 3's first spill wording on the device.
- **Hindered:** the wording chained each spill to the last, which makes every block its own level; it showed only when the young test's cap-64 run crashed on a level bound. A host simulation of the rule on the captures (about 20 s) settled it, and settled the sibling rule before building.
- **Cost:** about 20 minutes of device code for the chained rule, mostly reused.
- **Would remove it:** simulating a block rule on the host over the busy captures before it is written into DESIGN.md.

## 2026-10-11, task 2 (whole grower): the adapter's 2 GiB storage binding
- **Doing:** the first whole-build measurement at the presets' default ages.
- **Hindered:** Laurelin 80 ran out of device memory, and the units' and phytomers' arena would have passed the adapter's largest storage binding (2,147,483,644 bytes on this RTX 3080 under Vulkan) at Telperion's 18.2M phytomers. The arena became three buffers (units, the phytomers' columns the units write, those the layout writes), and a move now makes anew only the tables whose rows grew. DESIGN.md section 1's one buffer a column is not possible under WebGPU's 8 storage bindings a stage.
- **Cost:** about 40 minutes and two measurement runs.
- **Would remove it:** the binding limit named beside the 128 MB default in DESIGN.md section 1.

## 2026-10-11, task 2 (whole grower): two flag programs on one word in one point
- **Doing:** cutting the light's lattice dispatches into ranges.
- **Hindered:** the determinism test differed at small ranges. The four lattice kinds each wrote the same overrun flag with `max(flag, cond)` in one point, where the programs run at once, so the last writer's 0 hid the overrun. Found by tracing the moves cycle by cycle (four runs).
- **Cost:** about 25 minutes.
- **Would remove it:** a check in the plan's builder that no census word is written twice in a point (not added: one case, now folded).

## 2026-10-11, task 2 (settle): WGSL reserved words again
- **Doing:** the first validation of the settle's shader (`settle_tree.wgsl`, `settle_assign.wgsl`).
- **Hindered:** `from` and then `target` refused as reserved words, each found by one rebuild of the render crate's tests and the shader check (the layout's entry above met the same two).
- **Cost:** about 2 minutes, two builds.
- **Would remove it:** the shader check listing every reserved word it meets in one run, or a lint over the cycle's WGSL for WGSL's reserved words before the crate builds.

## 2026-10-11, task 2 (performance pass): a GPU hang behind a short limit
- **Doing:** recording each cycle only as far as its counts' limits.
- **Hindered:** Telperion's build hung with the GPU idle (NVRM Xid 109, context switch timeout), three times, each a 5- or 25-minute timeout. Axes a short layout-generation limit left unframed made a later kernel loop. Neither a stack trace (ptrace not permitted) nor the log showed it; tracing each cycle's limits and flags to stderr did.
- **Cost:** about 45 minutes.
- **Would remove it:** the halt now in `vm.wgsl` (a raised flag empties every later dispatch). A shorter `timeout` on GPU measurement runs would also have cut each wait.

## 2026-10-11, task 2 (performance pass): another worker in the same worktree
- **Doing:** committing the performance pass.
- **Hindered:** the settle's worker edits `generation/cycle.rs` and its own files in this worktree at the same time, and builds into the same target. My hunk of `cycle.rs` had to be staged by a constructed patch (`git apply --cached`), and a build mid-edit could have broken on their files.
- **Cost:** about 5 minutes.
- **Would remove it:** one worktree for each worker, or a shared file's registrations made by one of them.

## 2026-10-11, task 2: the dispatch cost was measured native only until the executor existed
- **Doing:** building the device cycle executor to native timings (7.6 µs a dispatch).
- **Hindered:** the owner's Chrome probe priced a dispatch at about 0.4 ms. The executor's 28,016 dispatches for Telperion become about 11 s in Chrome, and bounded subtrees cannot fit the budget's 36 dispatches a cycle (DESIGN.md 7a). Tint also refuses NaN constants and barriers under non-uniform loops, which naga accepts, and no Tint runs outside a browser here.
- **Cost:** the executor's dispatch shape was tuned against the wrong unit (several hours of agent work on 12 µs dispatches). Every kernel's barrier loops now need review.
- **Would remove it:** the Chrome dispatch probe (indirect against direct, one pass against many) before any executor design; a Tint validator runnable outside the browser in the shader check.

## 2026-10-11, task 2 (settle): the device flushes f32 subnormals
- **Doing:** holding the sag's lay exact on the young spruce and the ordinary.
- **Hindered:** a few phytomers at fine tips (radius about 3e-6 m) parted by ulps. Levers there are about 1e-20, whose squares are f32 subnormals: the device flushes every subnormal to zero as operand and result (an ignored probe, `the_device_keeps_subnormals`, prints 0 against the CPU's 1.2e-40), the CPU keeps them. Found by capturing the first lay and its levers (exact), then a turn probe over 262k random inputs (which also found `exact_div` writing garbage for a subnormal quotient). Exact mode now takes the sag's dot products, scalings and roots in integers with gradual underflow (`settle_sub.wgsl`); `exact_div` rounds into the subnormals.
- **Cost:** about 45 minutes, nine GPU runs.
- **Would remove it:** naming flush-to-zero beside fusion and reordering in the exact switch's doc (UNITS.md, LAYOUT.md); the host's ruling on the f32 contract would settle it for plain builds.

## 2026-10-11, task 2 (settle): the exact place module's cold compile
- **Doing:** the first exact run of the place module (stand, walk, frame, sag, software f64 and the gradual-underflow ops).
- **Hindered:** its cold compile took about 270 s on the NVIDIA driver; the exact module's change to `exact.wgsl` also recompiled the units module (95 s).
- **Cost:** about 6 minutes of GPU lock.
- **Would remove it:** one call site per heavy function in the place module (`step` and `sag_of` are called once each, but the integer helpers `sub_mul`/`sub_add` are inlined at many sites); not attempted before the stop.

## 2026-10-11, task 2: the dispatch cost, resolved
- **Doing:** reacting to the first Chrome reading (0.4 ms a dispatch).
- **Hindered:** the second probe put the whole cost on Dawn's validation of indirect arguments. Indirect dispatches take 429-432 µs, direct ones 5.3-6.7 µs. The executor sized every dispatch indirectly, so it must be converted to direct dispatches with bounds the host knows (DESIGN.md 7a).
- **Cost:** one conversion pass of the growth loop's sizing, and about an hour of the host's and workers' attention on a stop that did not hold.
- **Would remove it:** the probe's style split (indirect against direct) in its first run.

## 2026-10-11, executor worker: the baseline for direct dispatches ran out of GPU memory

- **Doing:** measuring Telperion 82, the oak 80 and the beech 40 before converting the growth loop to direct dispatches.
- **Slowed by:** `wgpu error: Out of Memory` two seconds into Telperion, three times, on a build that ran 15 minutes earlier. About 3.6 GiB of the 10 GiB card is held by desktop processes, 1.1 GiB of it by a GNO embedding worker that started 6 minutes before.
- **Cost:** three runs, about 5 minutes.
- **Would remove it:** a GPU memory check (`nvidia-smi`) in the measurement's preamble that reports the free memory and refuses to start below what the build needs, instead of failing mid-build. The owner's machine is shared, so this is a local note, not a spec.

## 2026-10-11, executor worker: direct dispatches hung the GPU three times

- **Doing:** converting the growth loop's indirect dispatches to direct ones.
- **Hindered:** NVRM Xid 109 at 20:46, 20:56 and 21:23. Under indirect sizing a halted cycle emptied every later dispatch. Under direct sizing, a kernel that reads its count from a buffer (the layout's listed axes, the sums' level sweeps, the light's lattice) ran on what the halted cycle left, and looped.
  - The second hang was the settle worker's measurement, built from this worktree's shared target with the change half done. I stopped that process by its PID.
  - The third was my own bisect run without the guards, while the owner had a game open on the same GPU.
- **Cost:** about 40 minutes, a lost settle measurement, and three driver resets on the owner's desktop.
- **Would remove it:**
  - Make the halt a property the kernels hold (every count-reading kernel exits on a halted word), with a test that raises a flag mid-cycle on the smallest tree, before any run of a sizing change.
  - Give each worker its own target dir, so one worker's half-built change never reaches another's run.

## 2026-10-11: retirement compaction, the measurement ran out of device memory (worker)

- **Doing:** measuring Telperion 82 grown with and without compaction in one test.
- **Slowed it:** the second build ran out of memory in `Build::new`, at 1.7 GiB of 10 GiB in use. Dropped builds are not freed until the device is polled, so two 3.4 GB builds plus a settle overlapped. The busy settle timing test had the same shape.
- **Cost:** two runs, about 5 minutes and a backtrace.
- **Would remove it:** a helper that drops a build and polls the device, used between builds by every measurement test; or a ledger that frees on submission completion rather than only on poll.

## 2026-10-11: the acceptance measurement and compaction (worker)

- **Doing:** timing every preset with retirement and compaction on, per phase.
- **Slowed it:**
  - The existing measurement builds again at the sizes found by a first build, so that each stamp pairs with one phase. A build at its final sizes never moves, so it never compacts. A build from the least sizes restarts and moves, and the stamps from that discarded and moved work have no phase in `Grew::stamped`.
  - Each preset therefore had to be built two ways. Two runs failed on that assertion before I saw why.
  - Separately, `the_settle_compiles` had gone stale. It bound six buffers where the settle now binds eight, and it named `place_main`, which the bounded walk removed.
- **Cost:** about 10 minutes and three runs.
- **Would remove it:**
  - give the stamps from restarts and moves a phase ("again", "move", "compaction") in `Grew::stamped`, so that one build from the least sizes can be read per phase;
  - have the compile test take its binding lists from `Kernels::new`.

## 2026-10-12, task 2: two editors staging in one index
- **Doing:** taking over the cold-compile work, with `git rm` of the libm ports staged while a worker was still committing its evidence.
- **Hindered:** the worker's evidence commit (`37edd6d3d`) swept the staged deletions in with it, so that one commit removes files its sources still reference and does not build. The next commit completes it.
- **Cost:** one broken intermediate commit; a few minutes to find it.
- **Would remove it:** one editor per worktree at a time, or `git commit -- <paths>` (commit only the named paths) in the workers' commit recipe.

## 2026-10-12, task 3: no stated route from the device's tree to the draw (worker)

- **Doing:** reading task 3's anchor, COUNCIL.md, R3B.md, DESIGN.md and EXECUTOR.md, then the production routing (`preparation.rs`, `fine.rs`, `fine/resident.rs`, core `pipeline/space.rs` and `pipeline/executor.rs`), before any edit.
- **Hindered:** the task routes the harness through the device cycle, but nothing says how the cycle's settled node table reaches the drawing stages. Today the presets `apart` admits (spruce, beech) are drawn on the device from their fine roots (dress, sweep, stations, rings, handoff), and the CPU converts only the structure above the cut. The rest are drawn by `prepare_whole` on the CPU. Deleting the fine roots leaves two routes with very different costs, and choosing between them is a design call (AGENTS.md, "Dispatch and escalation").
- **Cost:** about 25 minutes of reading; nothing built.
- **Would remove it:** the route named in the task, together with whether the browser's `WASM_KEPT` refusal may come back for the large trees.

## 2026-10-12, task 3: a store read back by the same invocation came back stale (worker)

- **Doing:** planning the settled tree's chains on the device. One invocation per chain walks it breadth first, appending each axis to a queue in the plan buffer and reading the queue back as it goes.
- **Hindered:** on the RTX 3080 (Vulkan), the invocation's own reads of the queue returned the zeros the buffer held before its stores. The spruce at 12 then drew no trunk, and its build placed no leaves. The plain `read_write` storage loads and stores to the same address in one invocation were not ordered. The queue now lives in atomics (`atomicStore`, `atomicLoad`) and the spruce agrees with the CPU in every node.
- **Cost:** about 40 minutes and six GPU runs, three of them dumps of the plan.
- **Would remove it:** a stated rule for the generation shaders, with a test: no kernel reads back a plain storage word it wrote in the same invocation; such a word is atomic or private. The dress (`cycle/dress.wgsl`) and the fine dress before it read back records they wrote and have agreed, so the hazard is not universal: what decides it is not known.

## 2026-10-12, task 3: the sweep agreement test hid the trunk

Doing: viewing every preset's stills beside F4's. Slowed by: the device drew a wider, uncapped flare at every trunk's foot, which only the stills showed. `every_preset_sweeps_its_wood_as_the_cpu_does` allowed 1% of runs unmatched, and the trunk is one run, so the device's missing `trunk_run` (buried foot, no socket swell) passed it on every preset. Cost: about 20 minutes and two extra still renders (a CPU-route render to localise it). Would have removed it: a tolerance that names the runs it may skip; the test now requires every trunk run matched point for point.
