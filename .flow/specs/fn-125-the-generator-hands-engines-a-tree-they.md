# The generator hands engines a tree they only draw

## Conversation Evidence

> user: "do we need the cpu chain for leaves? beacuse if you're going to render millions of leaves you'll need a gpu anyway to not stutter like crazy..?"
> user: "ok but let's think about this. If we're generating leaves and wood to render on the gpu. Why would we generate it on the cpu slowly?"
> user: "gpu path as the renderer's default still means that it's the generator pipeline that runs on gpu though right?"
> user: "i think it makes sense no? most consumers will be some sort of gpu driven application and having to reimplement the rendering of leaves is besides the point? we should provide the full fidelity tree for them to just render. And the generation should be FAST. so using GPU seems like it's a must."

## Goal & Context

<!-- Goal & Context: 50% [user], 50% [inferred] from code and fn-91 evidence -->

fn-91 built a GPU path in the renderer: `Generator::prepare_async` with `Delivery::Resident` expands leaf stations into leaves and culls them in shaders, and can expand wood on the GPU. The harness draws through it (`setTreeGpu`, `harness/GrowerDev.tsx:79`). It is the renderer's code, written for wgpu, and falls back to the CPU recipe for any family it cannot express. [inferred]

The owner decided on 2026-09-23 that consumers are GPU applications. They receive the full-fidelity tree to draw and never reimplement leaf expansion, and generation must be fast, so the GPU is the drawn path. The generator owns the expansion of the plan into leaves and wood as a contract. [user]

## Architecture & Data Models

The contract has three parts. [paraphrase]

1. **The plan layout.** fn-102's stage 3 artifact, in a documented binary layout an engine can upload as is: station segments, wood rings or their compact form, the element, the envelope and the cull configuration.
2. **A CPU reference expander** in the core that turns the plan into leaves and wood, defining correct. The current CPU placement and cull stay as the family fallback until every family has a plan form.
3. **Portable shaders.** The GPU expansion ships as WGSL that the generator owns. naga, wgpu's shader translator, can emit HLSL, GLSL, MSL and SPIR-V from it. The renderer becomes one consumer of these shaders instead of their owner.

GPU output matches the reference within a stated tolerance, not byte for byte (2026-09-20 policy). Stages 1 to 3 stay on the CPU. [user]

The renderer's GPU preparation carries work this spec removes or overlaps (fn-102 review, item G): up to four surface sweeps on the contact path (`preparation.rs:43, :159, :209, :331`), stations prepared twice when position admission fails (`:89, :209`), masses computed and dropped on CPU delivery (`compute.rs:217`, `preparation.rs:289`), wood expansion waiting for foliage compute (`:300`), and rings and segments copied twice (`compute.rs:57`, `data.rs:86`). [inferred]

## Edge Cases & Constraints

- Families without a station form fall back to the CPU recipe today: short shoots, limb clumping, no twig layer, lobed crown surfaces (`preparation.rs:33`) and the numeric fallback. Each needs a plan form before the GPU is the only drawn path; each is its own gap spec, per the project rules, which the owner decides. [inferred]
- A consumer without a GPU uses the reference expander, and gets the same tree within the tolerance. [inferred]
- Device loss, allocation limits and unsupported adapters leave an explicit failure and a usable lifecycle, as fn-91 requires. [inferred]

## Acceptance Criteria

- **R1:** The plan layout is documented under `docs/` and versioned; a test round-trips it for every catalogue preset. [inferred]
- **R2:** The core's reference expander produces leaves and wood from the plan for every family that has a plan form, and a test compares it with today's CPU placement and cull within the stated tolerance for every catalogue preset at seeds 1 and 7. [inferred]
- **R3:** The GPU expansion shaders live with the generator, the renderer consumes them without its own copy, and a test translates them through naga to HLSL and SPIR-V without error. [inferred]
- **R4:** `setTreeGpu` completed-frame medians for the oak and spruce at seeds 1 and 7 are no slower than fn-91's final 158.1/180.8 and 178.9/164.2 ms, and the renderer overlaps from item G are each measured. [inferred]
- **R5:** GPU output matches the reference within the tolerance on every supported family, and the owner's visual verdict is recorded for the oak, spruce and birch. [user]

## Boundaries

- Growth and plan preparation stay on the CPU. Growth speed is its own spec. [inferred]
- No external-engine adapter; fn-17 owns the first integration and proves whether naga's output is enough. [inferred]
- The core itself gains no wgpu dependency; the slim package (fn-101) stays GPU-free. [inferred]

## Decision Context

- Owner decision 2026-09-23 (fn-102 review, item I). fn-102's deferred ideas "a CPU reference expander over station segments", "the GPU station path as the renderer's default" and "a station form for short shoots" land here and in the gap specs. [user]
- Depends on fn-102, which defines stage 3 in one place. [inferred]

- Retire the separate ring sweep. After fn-102 the wood's float32 vertex array is the one ring store the mature build reads; `AttachmentSurface::new`'s own sweep survives only for `foliage::prepared::prepare_stations`, `place_on_surface`, examples and tests. The GPU contract rebuilds station preparation, so it moves that caller onto the wood's rings and deletes the second sweep. [inferred, fn-102 review 2026-09-24]

## Open before ready

- Where the shaders and the layout live: a new crate beside the core, or a data module in the core with no wgpu dependency. A host design call before ready.
- The tolerance: what distance in metres and what normal angle counts as the same tree. Set from fn-91's measured CPU/GPU differences (positions below 2.336 µm) before ready.
- Whether R4's "no slower" should instead be a target speed-up; the owner sets it.
- The gap specs for the five fallback families are not written.
