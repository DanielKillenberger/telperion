# Wood ring preparation on the GPU, probe first

> HTML render lens: `.flow/artifacts/fn-171-wood-ring-preparation-on-the-gpu-probe/spec.html` (gitignored — open locally; regenerable, markdown is the record). <!-- flow-next:artifact-link -->

## Conversation Evidence

> user (2026-09-27): "ok that seems good. We're aiming to halve the speed? how can that be achieved?"
> user (2026-09-27, on moving ring preparation out of fn-126, selected): "Two specs"
> user (2026-09-27): "first investigate if we haven't tried gpu ring prep spec before and failed. If no and it's a good approach we should spec it"
> user (2026-09-27): "126 is good to mark ready. Explain 1 and 2 yes it should start with experiment and if it reaches the expected improvements integrate it"
> user (2026-09-27, on the experiment's pass margin, selected): "30%"
> user (2026-09-27, on resuming this capture): "yes"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 80% [paraphrase], 20% [inferred] -->

Before the GPU draws a tree's wood, the CPU prepares its rings: it orders the runs, samples each run's rings, computes each ring's frame, packs the ring words and works out where branches touch. On master (c71ef3e1, native, the harness's GPU path) that preparation takes 40 to 42 ms on the oak and 32 to 33 ms on the spruce at seeds 1 and 7, and station preparation another 20 to 25 ms; stage-3 CPU preparation totals about 62 ms on the oak and 57 ms on the spruce (`.flow/evidence/fn-125-the-generator-hands-engines-a-tree-they/BASELINE.md`). [paraphrase]

fn-125 set out to halve that and could not: the baseline showed the time is the preparation work itself, not duplication, so the owner rescoped fn-125's target and asked for this work as its own spec. fn-91 already moved the last step onto the GPU (emitting vertex positions from prepared rings), which made the browser oak 44 to 45% faster, and kept the ring preparation on the CPU by a scope rule, not because it failed. Nothing has tried moving it since. [paraphrase]

This spec moves ring preparation onto the GPU, experiment first. A probe measures it on the oak and spruce; only if the probe clears the owner's margin is it built into the pipeline. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 25% [paraphrase], 75% [inferred] -->

- **Today, checked 2026-09-27.** The CPU compaction walks every run, samples its rings twice (once to rank runs by radius, once to emit), computes parallel-transport frames, and packs one ring word per ring: centre, radius, frame and vertex offsets. The GPU position pass reads those words and emits vertices, admission and bounds. fn-91 measured the second sampling, frames, packing and contact ranges at 26.6 to 33.0 ms on the native oak and the ranking pass at 5.6 to 6.9 ms. [paraphrase]
- **The probe.** The GPU computes the ring words from the plan's run and node data: one invocation per run samples it, carries its frame from ring to ring, and writes its words where the CPU would. Runs are independent, so the pass is parallel across runs and sequential only along one run; the longest run sets its time. What the probe moves is decided by measurement, stage by stage: sampling and frames first, then packing; run ranking and contact ranges move only if the probe shows they pay. [inferred]
- **One algorithm, two executors.** fn-125's CPU reference executor keeps the same ring preparation in Rust and defines correct. The GPU version is held to it within fn-125's tolerance on everything the ring words feed: the emitted wood (positions and vertex normals) and the foliage seated on it. The wood positions are the shared buffer the GPU foliage pass seats leaves on and then culls (fn-91 task 6), and a tree with surface contact, the spruce among them, carries contact ranges through the same path, so a ring or contact error shows up as displaced or lost leaves. [paraphrase]
- **The precision domain does not widen.** Ring centres within 64 m, radius at most 32 m and the relative floor stay fn-125's named domain; an input outside it is fn-125's named error, never a CPU fallback. [inferred]
- **Integration rides fn-125's contract.** The ring pass joins the `gpu` module's shaders and reads the plan layout fn-125 defines; the layout gains what the pass needs to read, versioned as fn-125 requires. [inferred]

## API Contracts
<!-- scope: technical -->

- **Plan layout.** Any field the ring pass reads is added to fn-125's layout under its versioning rule; no second upload format. [inferred]
- **Metrics.** The renderer's stage metrics report the GPU ring pass on its own line beside position preparation, so R2 and R4 read one field each. [inferred]
- **Views and commands unchanged.** [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The longest run.** A run with many rings runs its frames serially on one invocation; the probe reports the longest run on each fixture and its share of the pass. [inferred]
- **Shaped and lobed wood.** The palm's shaped runs and lobed wood use the per-run cell record and lobe term fn-125 adds; the ring pass reads them rather than branching on them. [inferred]
- **Device limits and loss** fail explicitly as fn-91 and fn-125 require. [inferred]
- **Budget.** The probe runs on the 8-tree scale at most; no full-forest capture. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A probe computes the ring words for the oak and spruce at seeds 1 and 7 on the GPU, and what they feed agrees with fn-125's CPU reference within fn-125's tolerance: the emitted wood's positions and vertex normals, and the foliage seated on it (equal surviving leaf counts, and leaf position, orientation and scale within fn-125's leaf bounds). The spruce fixture exercises shared contacts. Errors: a fixture outside the tolerance is recorded with its largest error and ends the experiment without integration. [paraphrase]
- **R2:** The experiment passes only when, measured by `BASELINE.md`'s method on the base (fn-125 merged) and the probe, stage-3 CPU preparation (position prepare plus descriptors) is at least 30% lower for the oak and the spruce at both seeds. Errors: a miss on any of the four fixtures reverts the probe's production changes, records the profile, and stops with `NEEDS_HUMAN`; nothing is integrated. [paraphrase]
- **R3:** Only after R2 passes, ring preparation runs on the GPU inside the one pipeline for every catalogue and in-work preset, with no new capability gate or fallback; the GPU and reference executors agree within fn-125's tolerance for each at seeds 1 and 7, on the wood and on the foliage seated on it as R1 states, with every preset that uses surface contact exercising its contacts, including when run ranking or contact ranges have moved to the GPU. Errors: an input outside the precision domain fails with fn-125's named error. [paraphrase]
- **R4:** After integration, the four fixtures keep at least R2's 30% cut, and the harness's browser completed-frame medians for the oak and spruce are recorded on base and candidate and are no slower within fn-125's 5% measurement tolerance. Errors: a browser regression above 5% stops with `NEEDS_HUMAN` and the profile. [paraphrase]
- **R5:** A report in the spec's evidence directory records the probe's stage split, the longest run per fixture and its share, the four fixtures' base and probe numbers, and the verdict. Errors: no error surface beyond R2. [inferred]

## Boundaries
<!-- scope: business -->

- The CPU reference executor's own speed is fn-126's, not this spec's. [paraphrase]
- No change to what any tree looks like; the wood is the reference's within tolerance. [inferred]
- The float32 precision domain is not widened here. [inferred]

## Decision Context
<!-- scope: both -->

Experiment first because the one risk is structural: frames are sequential along a run, so the GPU's parallelism is across runs only, and nobody has measured how that nets out. fn-91 took the same route for position emission (a probe, then integration) and it paid 44 to 45%. [paraphrase]

The 30% margin is the owner's. The movable part is 32 to 42 ms of wood preparation out of about 57 to 62 ms, so 30% means moving most of it while leaving room for the sequential frame step. [paraphrase]

Rejected here: removing only the ranking pass on the CPU, which fn-91 task 10 measured under 5% and reverted. [paraphrase]

Depends on fn-125, whose plan layout, `gpu` module and reference executor this spec extends. [paraphrase]

## Strategy Alignment

- **The core and integration:** the expansion stays one algorithm with a GPU executor and a CPU reference held equal by tests; this spec moves more of it onto the GPU executor without a second path. [strategy:The core and integration]
- **Approach, measured cost:** the work is gated on a measured gain, and a probe that misses is reverted, not kept. [strategy:The core and integration]

## Strategy Conflicts

None found.

## Parked unknowns

- Whether run ranking can move to the GPU or must stay a CPU sort ahead of the pass; resolved by the probe's stage split.
- Whether contact ranges move with the rings or stay on the CPU; resolved by the probe's stage split on the fixtures that need contacts.
- How long the longest run is on the oak and spruce, which bounds the sequential frame step; resolved by R5's report.

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | fn-N.M (TBD - populate via /flow-next:plan) |
| R2 | fn-N.M (TBD - populate via /flow-next:plan) |
| R3 | fn-N.M (TBD - populate via /flow-next:plan) |
| R4 | fn-N.M (TBD - populate via /flow-next:plan) |
| R5 | fn-N.M (TBD - populate via /flow-next:plan) |
