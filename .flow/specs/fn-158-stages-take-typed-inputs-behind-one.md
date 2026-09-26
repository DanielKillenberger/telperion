## Conversation Evidence

> Astra review of fn-152 (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md`): `pub(crate)` alone does not stop a second chain; GPU preparation orchestrates stages itself (`telperion-render/src/generation/preparation.rs:14`). "Design the executor boundary first", then migrate stage inputs and visibility together.
> owner (2026-09-25, fn-151): "structure over policing": stages private to the pipeline, so a second chain does not compile.
> owner (2026-09-26): "ok that seems reasonable. /flow-next:flow this to its completion in one stack with pr's for each spec"

## Goal & Context
<!-- scope: business -->

Generation has one path, and the compiler keeps it that way. Stages take only the typed input they read, never the whole family, and they sit behind the pipeline. The renderer's GPU executor and the hidden growth path reach generation through a narrow interface, not by calling stages directly. Second of four stacked specs, after fn-152. Output does not change. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-25.** Stages read `&Family` fields directly. The GPU preparation calls core stage functions itself, and overlaps GPU position work with CPU station preparation (`preparation.rs:14`, `:85`). The growth path builds its own chain. [checked]
- **Executor boundary first.** Define the smallest prepared-artifact and execution interface that the GPU executor and the growth path need. STRATEGY.md's "one algorithm with two executors" is the sanctioned shape. [paraphrase]
- **Typed stage inputs.** The pipeline builds narrow typed inputs from `Family` (for example `GrowInput`, `PlanInput`, `SurfaceInput`) with no back-reference to it. Family-level values are derived once there; branch- and sample-level values stay derived inside their stage, where their inputs exist. [paraphrase]
- **Visibility.**
  - Stage implementations live in private pipeline modules.
  - Only `pipeline::build` and the executor interface are public.
  - Implementation tests move into private test modules; there is no public test feature.
- **Preserve:** lazy requests, scheduling and overlap, arithmetic order, `f32` conversion order and random-number consumption. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** No stage takes `&Family`. A compile-fail test with a positive control shows that a stage call from `telperion-wasm`, and one from `telperion-render` outside the executor interface, does not compile. Errors: the positive control compiling proves the test harness itself works. [paraphrase]
- **R2:** The GPU executor and the growth path build through the executor interface. [paraphrase]
- **R3:** Every shipped preset is byte-identical in mesh, field and metrics, and CI's artifact budgets hold. Small parameterised cases cover executor coverage, contacts, optional overrides and request subsets. [paraphrase]
- **R4:** Timings for each shipped preset are taken before and after, release profile, on the owner's RTX 3080 workstation, at least five runs each, with medians and spread reported; resident GPU and CPU delivery are measured separately. Errors: a regression beyond the spread is reported with its cause. [paraphrase]
- **R5:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Not consumer migration (fn-159) or presets (fn-160). No behaviour changes; the GPU leaflet defect is its own spec.

## Strategy Alignment

- Serves "Our approach": one pipeline every tree passes through. [strategy:Our approach]
