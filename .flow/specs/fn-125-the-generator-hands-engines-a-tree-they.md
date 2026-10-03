# The generator hands engines a tree they only draw

## Conversation Evidence

> user: "do we need the cpu chain for leaves? beacuse if you're going to render millions of leaves you'll need a gpu anyway to not stutter like crazy..?"
> user: "ok but let's think about this. If we're generating leaves and wood to render on the gpu. Why would we generate it on the cpu slowly?"
> user: "gpu path as the renderer's default still means that it's the generator pipeline that runs on gpu though right?"
> user: "i think it makes sense no? most consumers will be some sort of gpu driven application and having to reimplement the rendering of leaves is besides the point? we should provide the full fidelity tree for them to just render. And the generation should be FAST. so using GPU seems like it's a must."
> user (2026-09-24): "yea we want to aim for aggressive performance gains wherever we can."
> user (2026-09-24): "i mean i'm surprised that we have different paths for different trees. We want as little duplication/separation as possible. One pipeline cleanly defined and executed."
> user (2026-09-24): "that makes more sense. [...] Can we do a feasibility check on 125 first maybe?"
> user (2026-09-24): "we need to ratify this in strategy [...] One streamlined pipeline that's efficient and clean." (STRATEGY.md amended the same day)
> user (2026-09-27, on the palm leaf-base cell carried per shaped run and the reference executor grown out of `pipeline::executor`): "ok that seems good."
> user (2026-09-27, on the master baseline, choosing option 1 for the speed target): "1 is good"
> user (2026-09-27, on dropping R7's order-key pass clause after fn-91 task 10's reverted attempt): "Drop it"

## Goal & Context

<!-- Goal & Context: 50% [user], 50% [inferred] from code read and feasibility check 2026-09-24 -->

Every tree passes through one pipeline: grow, plan, expand, cull, draw. The generator owns the expansion from plan to drawn leaves and wood as a contract that engines consume, and the harness renderer is its first consumer. The expansion is one algorithm with two executors: the GPU (WGSL), and a CPU reference that defines correct and serves consumers without a GPU. There are no fallback paths; an input the pipeline cannot represent is an explicit error. This is STRATEGY.md's "Our approach" as amended on 2026-09-24. [user]

Today the trees split across three routes (checked 2026-09-24 against origin/master 123c4261). [inferred]

| Tree | Status | Wood | Leaves | Why |
|---|---|---|---|---|
| ordinary, oak, spruce, birch | catalogue | GPU | GPU | fits the station path |
| Telperion, Laurelin | catalogue | CPU | GPU | lobed surface fails the gate at `preparation.rs:35` |
| European beech | in work | CPU | CPU | short shoots and limb clumping fail `plan::supports` (`plan.rs:20`); built by `mesh::assemble` (`preparation.rs:217-234`) |
| date palm | in work | CPU | CPU | a rosette fails `plan::supports` |

The expansion is also written more than once. There is the CPU placed path (`placement.rs:133` `place_on`, `station.rs:70` `place_run`, `short_shoots.rs`, `rosette.rs`, `clumping.rs`), and there are the GPU station shaders (`place.wgsl`, `pack.wgsl`, `scatter.wgsl`). The leaf matrix exists in both (`station.rs:252-316`, `place.wgsl:159-176`), and so do the cull (`foliage.rs:233`, `place.wgsl:32-43`) and the wood ring formula (`surface/build.rs:236-240`, `positions.wgsl:26-28`). [inferred]

## Architecture & Data Models

**Station sources.** A leaf is a pure function of (source, index, seed). The feasibility check of 2026-09-24 found every current feature expressible this way. [inferred]
- **Twig runs:** today's stations, the segments of `plan::Run` (`pipeline/foliage/prepared.rs`, `StationSegment`).
- **Short-shoot wood:** each cluster is closed-form in (wood node, cluster index, seed). Its phase, bearing and fan come from `key(seed, birth_order, k)` (`pipeline/foliage/short_shoots.rs`, the key at `:245`). The cluster's leaves draw from a counter-based `Rng` (`rng.rs:11-17`), so leaf j's draws are indexable. The wood it clothes is selected by `short_shoot_radius` and is not the run plan's wood (`short_shoots.rs:176`), so it is its own source. A cluster below the crown floor becomes a culled station, not a skipped one (`:223`).
- **Rosette apices:** frond k and leaflet j are closed-form in (apex, k, j, seed) (`pipeline/foliage/rosette.rs`, `fronds` `:163-201`, `fan` `:261-291` through `leaflet.rs`), and their draws are indexable from the frond's key (`stream`/`key` `:305-316`). Spacing is angular and by age, so this is its own source kind, not a synthetic run.
- **The dead-frond skirt is the same source.** Fronds `k` from the living count up to living plus `skirt` continue the spiral down the stem at the skirt's pitch, and the source sets the withered bit (the top bit of packed word 2, `packed.rs`) on them. No new source kind and no new leaf word; the shaders already read the bit. (checked 2026-09-27)

**Leaflets.** A station expands to its leaflets on the GPU. Today nothing in `telperion-render` handles leaflets, and `prepared.rs:226` counts stations, not `count*leaflets`, although the plan already counts leaflets (`pipeline/foliage/plan.rs:88, 107, 237, 242`). This serves the rosette and every compound leaf (fn-33). [inferred]

**Limb clumping is one reduction pass.** Each limb system's centre is today the mean of that system's placed leaves before the cull (`clumping.rs:61-91`), and its walls are the planes to its nearest centres. After the reduction, a leaf's keep test is per leaf: its position, its system, the cell table and a keyed draw (`clumping.rs:98-125, 175-193`). The pipeline therefore becomes expand, sum per system, thin, cull and compact. The system map is `clumping::systems`, which the plan's `Descriptor.system` already uses (`plan.rs:221, 238`). A run leaf's owner is its run's first node, and a plan descriptor's owner is its segment's distal node. The worker checks whether these ever differ and, if they do, keys by the segment. [inferred]

**Wood is one ring formula.** The lobe term `1 + lobe_depth*cos(lobes*(angle + phase))` (`angular.rs:16-21`) is per vertex. The GPU ring descriptors already carry the phase (`pipeline/surface/compact.rs:197`), and `position_probe.wgsl:19-21` already implements it. `positions.wgsl` gains the term, and both lobe gates go: `Expansion::round_section` (`pipeline/executor.rs:118-121`, read at `preparation.rs:23`) and the lobe clause of `CompactSurface::qualified` (`compact.rs:37-38`). fn-91 left lobes out only as a first-scope choice (`.flow/tasks/fn-91-...8.md:11`). [inferred]

**Draw keys.** GPU station draws are the n-th draw of the sequential stream (`pack.wgsl:31-35`, `rng.rs:11-16`), so ordinary, oak, spruce and birch keep their leaves. Short-shoot and rosette draws are already keyed. Clumping's draw is keyed by placement index across runs and then short shoots (`clumping.rs:119, 186`), so the global station ordinal must reproduce that order, or the beech's thinning changes bytes. Either is allowed under the 2026-09-20 policy with the beech's visual check. [inferred]

**What goes.**
- The CPU placed path as a separate implementation: `place_on` (`pipeline/foliage/placement.rs:138`), `place_run`'s back-scan (`station.rs:45`), and the renderer's `Backend::CpuFallback` (`preparation.rs:178, 188, 288`) through `Expansion::mesh` (`executor.rs:179`).
- The no-twig-layer placement: reachable only through `place`/`place_on_surface` with `None` (`placement.rs:17, 31`, the branch in `runs()` `:78-91`), used by tests and no family.
- Every silent fallback in `preparation.rs` (the capability gate at `:23`, `Ok(None)` returns at `:76` and `:94-98`, and the "station capability" and "CPU triangle admission" paths at `:98, :112, :122` and `:165, :280`).

The growth path that also called these station functions was removed by fn-181 (2026-10-02). [checked]

**Numeric domain, stated.**
- The phyllotaxis check (`prepared.rs:170-180`) becomes a parameter rule on `internodes * |divergence|`, refused by name. It trips above roughly 8e12 degrees; catalogue divergences are 99.5 to 180.
- The float32 precision domain (`compact.rs:28-33`: ring centres within 64 m, radius at most 32 m and at least max(1, |centre|)/131072) becomes an explicit, named error. The reference executor does not silently widen it. [inferred]

**The contract.**
1. **The plan layout.** Fn-102's stage 3 artifact, documented and versioned, in a binary layout an engine can upload as is: the station sources, the wood rings or their compact form, the element, the envelope and the cull configuration. Today `Prepared` holds the element and `Plan` the descriptors (`stage.rs:11-18`), rings are separate (`drawn.rs:147`), and the cull configuration sits in neither.
2. **The CPU reference executor.** It runs the same steps serially (native may parallelise chunks joined in order), defines correct, and is what tests and GPU-less consumers use.
3. **The shaders.** The WGSL the generator owns.

GPU output matches the reference within the tolerance below. Stages 1 and 2 (growth, plan preparation) stay on the CPU. [user]

**Renderer duplicate work** (fn-102 review, item G) goes as part of the same restructure:
- up to four surface sweeps on the contact path (`preparation.rs:30, :130, :171, :293`);
- stations prepared twice when position admission fails (`:67, :171`);
- masses computed and dropped on CPU delivery (`compute.rs:217`, `preparation.rs:255-258`);
- wood expansion waiting for foliage compute (`:264`);
- rings and segments copied twice (`compute.rs:57`, `data.rs:86`).

The separate ring sweep in `AttachmentSurface::new` goes too: station preparation reads the wood's rings. [inferred]

On master these duplicates run only on the fallback branches. The oak's and spruce's successful GPU path compacts the wood once and prepares stations once (`.flow/evidence/fn-125-the-generator-hands-engines-a-tree-they/BASELINE.md`), so removing them speeds the trees that fall back today, not the oak or spruce. (checked 2026-09-27)

**Shaped wood (the palm's leaf-base boots, #114) draws through the same ring pass.** A shaped run's rings are not circles: each vertex stands on the run's cell, blended by `flatness` from the ellipse through the cell's corners to the flat-faced cell (`tree::Section`), and its frame is fixed to the cell (`pipeline/surface/section.rs`). The ring words carry a round section only, which is why `CompactSurface::qualified` refuses any shaped surface today (`compact.rs:37`). The plan layout gains a per-shaped-run cell record (frame, corners, flatness and the three section rings); round runs carry none, and the ring pass evaluates the cell for a run that has one. Every round tree's rings, and its bytes, are unchanged. The `shaped` gate goes. (owner, 2026-09-27)

**The reference executor grows out of `pipeline::executor`.** `Expansion` already owns the CPU build (`Expansion::mesh`); it becomes the reference executor, running the same stage steps as the shaders. The `gpu` module holds the plan layout and the WGSL only. `docs/pipeline.md` is rewritten to match: it now calls the GPU executor a "sanctioned exception" and `Expansion::mesh` "the reference the GPU falls back to" (lines 6 and 44). (owner, 2026-09-27)

## Host decisions (2026-09-24)

- **Where the layout and the shaders live: a `gpu` module in `telperion-core`, not a new crate.** The WGSL files sit beside the Rust that encodes what they read, exported as `&'static str` constants behind the `geometry` feature. The renderer composes its passes from those constants and keeps no copy. A string constant adds no dependency, so the core stays wgpu-free and `telperion-field` (`default-features = false`) is unchanged. The leaf encoding moves first: its encoder `foliage/packed.rs` is in core and its decoder `shaders/leaf.wgsl` in render. The naga translation test lives in `telperion-render`'s tests with naga as a dev-dependency pinned to the version wgpu 30 locks (naga 30.0.1). [inferred, checked 2026-09-24]
- **The tolerance.**
  - Counts are equal.
  - Wood positions: max 50 µm and RMS 10 µm, fn-91's screen, set before measuring. The measured differences were 2.336 µm max and 0.61 µm RMS (`gpu-positions/REPORT.md`, `position-integration/numeric.json`).
  - Wood vertex normals: at most 0.01 rad, against a measured 0.00405. Face normals are not compared, because a skinny oak face measured 0.322 rad with its vertices inside the position bound.
  - Leaves: the bounds `generation/tests.rs` asserts. Position is within two quantisation steps of the reference box plus float slack, orientation within 0.005 in normalised column distance, and scale within `len/1024`.
  - The domain is the tree in its own frame, root at the origin, within the precision domain above. [inferred, checked 2026-09-24]
- **Two source languages is the accepted duplication.** The algorithm exists in Rust (the reference) and WGSL (the GPU), held equal by tests. A single source would need the CPU consumer to run the WGSL on a software adapter, which puts wgpu into the core. [inferred]
- **Motion rides on this contract.** fn-15 adds its bone table and motion stream to the layout and its motion function to the same module. This spec only keeps the layout versioned and the module open to a second shader family. [inferred]

## Edge Cases & Constraints

- **A consumer without a GPU** uses the reference executor and gets the same tree within the tolerance. [inferred]
- **Device loss, allocation limits and unsupported adapters** fail explicitly and leave a usable lifecycle, as fn-91 requires. [inferred]
- **Unchanged output where intended.** Ordinary, oak, spruce and birch are expected byte-identical in leaves and wood. A difference is investigated, not re-pinned. [inferred]
- **Changed output needs visual evidence.** Beech, date palm, Telperion and Laurelin may change bytes. Each change carries the 2026-09-20 policy's visual and correctness evidence. [AGENTS.md]
- **No full-forest capture.** Captures follow the budget rules. [AGENTS.md]

## Acceptance Criteria

- **R1:** The plan layout, including every station source, is documented under `docs/` and versioned. A test round-trips it for every catalogue and in-work preset. [inferred]
- **R2:** Every catalogue and in-work preset (ordinary, oak, spruce, birch, Telperion, Laurelin, beech, date palm) builds through the one pipeline at seeds 1 and 7, on both executors. No capability gate, `Ok(None)` fallback or `Backend::CpuFallback` remains. The no-twig-layer placement is deleted. An input outside the numeric domain fails with a named error, each with a red/green test. [user]
- **R3:** For every preset in R2, the GPU and reference executors agree within the tolerance. For ordinary, oak, spruce and birch, the reference executor's output is byte-identical to today's direct build. Each preset whose bytes change states the change and carries its evidence. [inferred]
- **R4:** *(Superseded by R7, owner 2026-09-27: the baseline showed the halving had no source on the oak's and spruce's GPU path.)* An aggressive speed target, not "no slower" (owner, 2026-09-24). This is a feasibility gate: if the target is missed, the worker records the profile and stops with `NEEDS_HUMAN`. [user]
  - Browser stage-3 preparation, the part this spec reshapes (36 to 44 ms on the oak and 20 to 25 ms on the spruce, fn-91 `cpu-profile/REPORT.md` as quoted in fn-124), is halved at seeds 1 and 7.
  - `setTreeGpu` completed-frame medians against fn-91's final 158.1/180.8 ms (oak) and 178.9/164.2 ms (spruce) fall by at least the time saved.
  - Beech and Telperion are measured on base and candidate, where they leave the CPU fallback.
  - Each item-G overlap is measured on its own.
- **R5:** The shaders live in the core's `gpu` module. The renderer consumes them with no copy of its own. A test translates them through naga to HLSL and SPIR-V without error. `telperion-core` and `telperion-field` gain no wgpu dependency. [inferred]
- **R6:** The owner's visual verdict is recorded for the oak, spruce, birch and Telperion, and for the beech and date palm where their output changed. [user]
- **R7:** Speed, measured by `BASELINE.md`'s method (native `generation_gpu`, `gpu-render`, warm medians at seeds 1 and 7) on the base and the candidate. The oak's and spruce's stage-3 CPU preparation (position prepare plus descriptors) is no slower than the base. (The clause requiring the order-key sampling pass removed was dropped 2026-09-27: fn-91 task 10 measured that removal under 5% and reverted it, `radius-order/REPORT.md`.) The beech, Telperion, Laurelin and date palm record base and candidate preparation totals with their stage split, since they leave the CPU fallback here. The harness's browser `setTreeGpu` completed-frame medians for the oak and spruce are recorded and no slower. "No slower" means a median within 5% of the base, the measurement tolerance of five warm runs on one machine; a median 0 to 5% slower passes and is reported with its numbers. Errors: a median regression above 5% on any fixture stops the build with `NEEDS_HUMAN` and the profile. [paraphrase]
- **R8:** The palm's shaped leaf-base runs build through the GPU ring pass from their per-run cell record, and the GPU and reference agree within the tolerance on the date palm at seeds 1 and 7; no `shaped` or lobe gate remains, and every round preset's wood bytes are unchanged. Errors: a cell record on a run with no section, or a malformed cell, is a named error with a red/green test. [paraphrase]
- **R9:** `docs/pipeline.md` describes `pipeline::executor` as the reference executor and the GPU as the second executor with no fallback; the words "sanctioned exception" and "falls back" no longer describe the GPU executor. Errors: no error surface beyond R2. [inferred]

## Boundaries

- Growth and plan preparation stay on the CPU; growth speed is fn-124. [inferred]
- No external-engine adapter; fn-17 owns the first integration. [inferred]
- No new species anatomy. Leaflets on the GPU carry what the CPU already draws; new compound-leaf form stays fn-33's. [inferred]
- No change to what any catalogue tree looks like beyond R3's stated changes. [user]

## Decision Context

- Owner decision 2026-09-23 (fn-102 review, item I): consumers are GPU applications and receive the full-fidelity tree. [user]
- Rescoped 2026-09-24 from "contract for the families the station path supports, the rest in gap specs" to one pipeline for every tree, after the owner rejected per-family paths. The feasibility check read short shoots, limb clumping, rosettes, lobed wood, the numeric fallbacks and every current expansion implementation; no feature needs a second path. [user]
- fn-126 speeds up this spec's CPU reference executor once it lands, and depends on this spec. [user]
- Depends on fn-102 and fn-134, both merged. [inferred]
- ~~Depends on fn-173 (owner, 2026-09-27)~~ Dependency dropped (owner, 2026-10-02): fn-183 removed most of growth's crown queries, so fn-173 is re-scoped, not ready, and its table may never be built. The GPU cull keeps its envelope formula; if fn-173 later lands a table, the plan layout gains it then. R3's byte-identity base and R7's speed base are taken on master after fn-183 (#137); `BASELINE.md` (master c71ef3e1) is retaken on that base before R7 is judged. [user]
- Speed target rescoped 2026-09-27 (R7 over R4). The master baseline showed the oak's and spruce's stage-3 time is the compaction and station work itself, not duplication, so halving it would need ring sampling and frames on the GPU. That move is a separate performance spec, not this one; the large gains here are the trees that leave the CPU fallback. [user]
- Maintainability (plan review): duplication - none identified beyond the explicitly accepted Rust/WGSL implementations; structure - none identified
- The palm's leaf-base cell rides in the plan layout rather than as CPU-built vertices uploaded to the GPU; uploading ready vertices would be a second way to draw wood. [user]

## Resolved via Codebase

- Routes on master (checked 2026-09-27): ordinary, oak, spruce and birch run wood and leaves on the GPU (`BASELINE.md` for oak and spruce); the date palm is now a catalogue preset (`presets.rs:34`) and the beech is still in work (`presets.rs:43`). Goal & Context's route table is as of 2026-09-24.
- Every file this spec cites moved under `crates/telperion-core/src/pipeline/` with fn-158 (#126); the references above are re-anchored to master c71ef3e1.
- The dead-frond skirt (rosette `skirt`, packed withered bit) needs no new source kind (`rosette.rs:65-80, 220`; `packed.rs:24-28`).
- The ring words already carry a per-ring frame (`compact.rs`, ring packing); what a shaped run lacks is the non-circular cell (`tree/section.rs:21-42`).

## Leaf-station contract with fn-190-one-growth-law-species-are-points-in-a (owner, 2026-10-03)

fn-190-one-growth-law-species-are-points-in-a replaces the growth stage. Whether the beech's short shoots are grown wood carrying leaves or foliage stations on existing wood is decided in fn-190-one-growth-law-species-are-points-in-a's R2 and written into both specs; this spec optimizes the station types growth produces and otherwise proceeds in parallel. [user]

The contract also removes the outline-based leaf cull (`foliage.rs:255`, `place.wgsl`) and limb clumping as thinning: growth decides where leaves are, so expansion places them without culling (owner, 2026-10-03). Until fn-190's R2 lands, this spec does not invest in optimizing the cull. [user]
