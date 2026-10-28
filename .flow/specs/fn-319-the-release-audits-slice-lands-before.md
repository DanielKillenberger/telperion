# The release audit's slice lands before the tag

## Goal & Context

Codex's release audit of 26 Oct 2026 (`.flow/evidence/fn-237-release-the-tree-space-engine-replaces/AUDIT-2026-10-26.md`, NEEDS_WORK) found defects on the release branch after killenberger.com reported slower trees than 0.1.4. The owner chose what 0.2.0 takes (26 Oct: audit packages 1, 3, 4 and 5; 27 Oct: the floret reduction, "i think we should fix it. We want high quality 0.2.0"). Matching 0.1.4's per-tree speed is not required (fn-285); the site's warm-up (package 2) is killenberger.com's. The five parts are independent and ship in one tag, so they are one spec with one task each (owner, 27 Oct: "ok sure" to one spec with five tasks).

## Acceptance Criteria

- **R1 (field startup; audit package 1):** the device field prepares and holds only the programs it dispatches: the cycle's kernels, the hanger, dresser, stationer and fielder, and the generator passes its flowering trees use. The curve's surfacing passes, the sweeper and the ringer are not made for a field, and the type system keeps a field's narrower tool set from reaching a renderer path that needs the rest (a compile-time guarantee, not a runtime check). The renderer's preparation keeps every program it dispatches. Deleting the field's use of the renderer's tool set was tried before making it cheaper; no second chain of stages is added (`docs/pipeline.md`). Proof: a paired cold measurement of preparation and first tree before and after, natively, recorded in evidence; field answers equal before and after on every shipped preset; a test that a field generator holds no curve pass. Errors: a device lost while the field's programs are made rejects the call by name and the next call starts afresh.
- **R2 (validation parity; audit package 3):** a family the CPU entry refuses is refused by the device's field and by the device's tree build (`setTreeGpu`'s path), with the CPU's message, through `Family::validate` and the counted instance budget, never a copy of either. Reproduced on Ordinary, seed 1, n 12: `canopy.maxInstances=1` (CPU: "foliage instance budget") and `canopy.scatter=-1` (CPU: "scatter") return a tree on the device today. Errors: a family's own error comes before a request for the device, so a page without WebGPU learns its family is invalid rather than "webgpu-unavailable"; a valid family on a page without WebGPU still gets "webgpu-unavailable". Proof: both inputs rejected natively through `Generator::grow_field` and the device build, compared with `pipeline::build`'s message, as ordinary tests (skipping without an adapter only where the device is needed); every shipped preset still grows.
- **R3 (typed renderer API; audit package 4):** `setTree`, `setTreeGpu`, `admission` and `beginGrowth` take a `Family` and serialize it with the one wire serializer `TreeEngine.build` and `growField` use; the harness's second serializer goes. The CPU field's frame and answer types and the device field's `Grid`, `FieldAnswer`, `FieldGrid` and `Bounds` have one definition each, importable by both entries without loading the wasm module. Errors: a preset id string passed where a `Family` is expected is refused by the wire serializer's existing message. Proof: typecheck, every caller in the repository updated, the package's export and dist tests, the migration notes.
- **R4 (compile once, and the CI key; audit package 5):** `compileField()` with no source returns one module for concurrent and repeated calls, as 0.1.4 did; a failed compile is not kept, so the next call retries; an explicit source is compiled as given. `scripts/ci-key.mjs` includes `docs/parameters.md` and `src/browser/parameters.generated.ts` in every suite whose tests read them, and the tests workflow's path filters let a change to `docs/parameters.md` through. Proof: a test that two concurrent default calls resolve the same module and a rejected one is retried; `ci-key.test.mjs` shows a change to either file changes each reading suite's key.
- **R5 (florets reduce on the device; audit roadmap 4, fn-257 R8):** a flowering tree's field reads back only the grid and its answers. The seats' box (non-withered florets, as `seats` defines them) and each cell's floret count are computed on the device from the placed florets; the floret list is not copied to the CPU. Errors: a floret outside the field stays an error by name, as the CPU's "a floret outside the field"; a flowering tree with no seats leaves the box and every count as a tree without flowers does; the grid and answers limits (`within`) count the new words, so a side of 256 refuses or grows exactly as the device's limits say, tested on both sides. Counts are integers added atomically, so the channel is order-independent; the device's f32 frame can place a floret on the cell edge's other side than the CPU's f64 does, so the flower channel is held to the CPU by `flowers_agree`'s existing bounds (`FLOWERS_TOTAL`, `FLOWERS_MOVED`), not byte equality, and the measured moved share is recorded. Proof: Telperion's and Laurelin's counts as today's tests state them, every non-flowering preset's answers byte-equal before and after, the time before and after recorded, and no floret readback left in the field path.

## Early proof point

Task fn-319-the-release-audits-slice-lands-before.1 validates the core approach (the floret count reduced on the device within the CPU's agreement bounds and inside the device's limits). If it fails, re-evaluate whether the flower channel moves to the device in 0.2.0, and put it to the owner, before continuing with the other field tasks.

## Boundaries

Out: 0.1.4's per-tree speed (fn-285), the lean field artifact (fn-242), GPU lifetime and budget (audit roadmap 5), grow once and sample repeatedly (roadmap 6), GPU pregeneration (fn-318), the site's warm-up, and fn-307's and fn-123's tree-free validation (they reuse what R2 calls). No preset or botanical change. The wasm boundary keeps JSON (`web.rs` takes a string); only the TypeScript surface changes. Builds on a branch off fn-198 (F4) and merges into F4; nothing is pushed or published without the owner.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the package's typed entry, its validation and its CI key are the integration surface the site consumes.
- **Surface and rendering at scale** — the field's startup and transfers are the site's cost.

## Decision Context

Host, 28 Oct 2026, on the gap analysis: validation runs before the device is requested (the family's error is the user's first fact); the device's tree build is the field's sibling and shares the defect, so it is fixed with it (working rules: the same fallback); the field's narrower tool set is a type, so the narrowing cannot fail at run time; the flower channel's equality is the existing CPU/device agreement bound because f32 cell edges cannot be byte-equal to f64 ones, and counts themselves are exact integers. A GPU pass that counts florets is the device executor's own stage beside the CPU's, as the field's other channels already are (fn-257), not a second pipeline.

## Quick commands

```bash
cargo test --profile ci -p telperion-render field -- --nocapture
npx vitest run src/field src/browser/render.test.ts harness/grow-in.test.ts
npx vitest run scripts/ci-key.test.mjs
```

## Requirement coverage

| Req | Task |
| --- | --- |
| R1 field startup | fn-319-the-release-audits-slice-lands-before.2 |
| R2 validation parity | fn-319-the-release-audits-slice-lands-before.3 |
| R3 typed renderer API | fn-319-the-release-audits-slice-lands-before.4 |
| R4 compile once, CI key | fn-319-the-release-audits-slice-lands-before.5 |
| R5 florets on the device | fn-319-the-release-audits-slice-lands-before.1 |
