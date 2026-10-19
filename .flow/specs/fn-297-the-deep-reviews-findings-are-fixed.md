# The deep review's findings are fixed before 0.2.0 merges

## Goal & Context

After fn-294 fixed the first whole-release review, the owner asked for the in-depth pass over everything it left unchecked (2026-10-19): layouts, buffer bounds, shader maths and races, device memory, and every other changed line, over `428009eb8..d7309355e`. Passes A to C2e ran in the owner's codex session; C1e, D2, D3 and E ran as read-only `codex exec` passes from the host (prompts and outputs: `.flow/evidence/<this spec>/raw/`). The host checked each finding against the code, and a fresh verifier tried to refute each of the later ones. This spec takes the ones that are wrong under a reachable condition, or that let a write run past its buffer before a refusal fires. The rest are listed under Boundaries for fn-296.

Release item (fn-237), after fn-294.

The common shape behind B.1 to B.3: a refusal that fires after the write it should have stopped, or that never sees the real value. The fix for all three is a checked size before the write and a refusal by name (no fallback, `docs/principles.md`).

## Requirements

- **R1 (E7:E.1, P1, real): one Wasm initialisation.** `src/field/index.ts:53` and `src/browser/render.ts:240` each keep their own `loading` promise around the same wasm-bindgen `init`; vite builds that loader as one chunk both entries share, and `__wbg_init` checks `wasm !== undefined` only before its `await`. A page that calls `createRenderer` and `prepareField` together (as the docs suggest) can finalise twice, leaving earlier handles pointing into the replaced instance. Fix: one shared initialisation promise both entries use. Test: a Node or browser-free test of the loader proves two overlapping calls instantiate once.
- **R2 (C.5, P1, code-confirmed): counting and emission agree on a tip that is both.** `resident/stations.wgsl` `measure` (line ~567) tests `tip` before `apex`; `emit` (line ~928) tests `apex` first. A fine stem tip that is both a cluster tip and a rosette apex has one segment counted and several frond segments written, past its allocation, over appended owner words. The CPU (`foliage/prepared.rs`) emits both: the rosette in its run and the cluster appended after every run. Fix: the device does as the CPU does, in both kernels, so counts and writes agree; swapping the order alone drops foliage. Test, red first: the date palm at 40 with `canopy.cauline = 1` and the trunk PA's `cluster_leaves = 1` (codex's case; first confirm it reaches the state, and if validation or the generator rules it out, record which and test the nearest reachable case), CPU = device on counts and records.
- **R3 (C.7, P1, code-confirmed): flower bounds include the drawn stalk.** `generation/place.wgsl:68` folds the organ element's stored vertices into the bounds, but a stalk's stored vertices are surface coordinates; `shaders/stalk.wgsl` draws it from `u.stalk` lengths. `select.wgsl:90` already adds the stalk reach for picking. Fix: the bounds add the same stalk reach plus the ribbon half width. Test: Laurelin's `Prepared::bounds()` contain every drawn stalk vertex (computed on the CPU from the same lengths); report how far the old bounds fell short on Laurelin.
- **R4 (B.1, B.2, B.3, P2): refusals fire before the write.**
  - B.1 `cycle/device.rs` `counts()`: the job-buffer offset `zone_at` is narrowed `as u32`; at ~7.47M sleepers it wraps and sizes 0.9 GB for 18 GB of jobs. Compute in u64 and refuse by name above the device limit.
  - B.2 `cycle/hier_blocks.wgsl` `assign_main`: `opened = atomicLoad(&tally[X_BLOCKS]) + …` is written unguarded into shape columns while X_BLOCKS only grows across compactions; the reader in `hier_members.wgsl:152` clamps. Guard the write against the stride and raise the existing fault flag; the host refuses by name.
  - B.3 `resident/stations.wgsl:636`: `leaves += k` is u32 and can wrap past 2³², passing the host's `max_instances` check (`resident/build.rs:215`). Saturate or flag the overflow so the check sees it.
  - Test each with the smallest configuration that reaches it, or, where no allowed parameter reaches it, a unit test of the sizing function at the boundary.
- **R5 (D2:D.1, D3:D.2, P2, one-line): the ledger is what is held.** Replacing a flowering tree with one that has no flowers releases the old organ buffers (`lib.rs:266-274`; `self.organ.release()` and clear the scene's organ). `CurveGpu::bytes()` counts its two 16-byte feedback buffers (`curve.rs:203-211`). Test: after Laurelin then oak, the allocator ledger equals the tree bytes reported, and the organ holds 0.
- **R6 (C.6, P2): rosette divergence is one turn.** `boots.rs:111` narrows an unreduced divergence (bounds ±1e9°) to f32 radians, about 11° off at the bound. Question, delete: bound `rosetteDivergence` to −360..360 (the same for `divergence` if it shares the path) and reduce in f64 before upload. Shipped values (137.5°, 150°) unchanged. Test: CPU = device leaf-base spiral at 359°.
- **R7:** the gate and `npm test` are green; each task gets a codex review.

## Boundaries

Not here (fn-296, after the release): C.1/C.2 same-value races (hardening), C.8 thinned-leaf culling sphere, C.9 establishment's 64-edge cutoff (unreachable: only the date palm establishes, with no lateral orders), D3:D.1 a spent growth state keeps its buffers until dispose (hidden growth path), E1:E.1 `total_ms` label, E3:E.1 the device-wide curve `OnceLock` after a refused prepare (not reachable from the package), E3:E.2 organ upload checks, E3:E.3 the frames example, E4:E.1 frond padding in mixed crowns, E5:E.1 the angular centroid's order, E6:E.1 an empty first-year snapshot, E6:E.2 the key-span test overflow, and the open items (late fault checks, the wake traversal's silent cap, finite-value bounds). E7:E.2, the GPU smoke script, is already fn-293.8's. The public API changes only by R6's narrower bound.

## Evidence

The codex outputs and the verifier's table go to `.flow/evidence/<this spec>/` (copied from the host's scratchpad `deep/`).
