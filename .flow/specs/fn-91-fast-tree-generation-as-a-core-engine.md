# Fast tree generation as a core engine capability

## Conversation Evidence

> user: "I'm thinking different species on rotation with different random seeds also"
> user: "is there a path to near instant generation? I thought that a lot of gen time goes to mesh generation and leaf placement. If we're placing leaves 1 by 1 in series surely that must be optimizable. Ideally we could do it all in parallel on a gpu. That'd make it almost instant?"
> user: "we have previously tried to optimize using gpu but couldn't make it can you check the learnings there and make sure we're not running against a wall again."
> user: "The generator won't be locked in for a while, while we develop it further."
> user: "yea i'm not saying never check for byte equivalence where it's useful but as we add more improvements we consciously want to improve the structure of the bytes over time."
> user: "and if we have performance improvements that can't be done byte equivalent but have no perceivable visual regression that's fine too for example"

> user: "it's not just for the website obviously"

> user: "this is crucial for all usecases"

> user: "i mean IF we can get the same byte identical tree for perf improvements that would make verification easier."

## Goal & Context
<!-- Source: [paraphrase] -->

Fast generation is a core requirement for every Telperion use case. Bring tree generation toward near-instant availability while preserving perceived fidelity. The shared capability must serve browser and native consumers; mature-tree builds are the first measured workload, not the limit of its relevance. Species rotation with random seeds on killenberger.com is the immediate use case, not the scope boundary. The owner wants to ship quickly and permits changes to generated structure and representation as the engine develops.

## Architecture & Data Models
<!-- Source: [inferred] -->

Measure the current path before selecting a mechanism. Separate skeleton generation, wood surface construction, attachment preparation, foliage placement, culling, bounds and GPU upload. Investigate compact structural inputs expanded into foliage buffers on the GPU, consumed directly by the renderer. Compare against focused CPU improvements where profiling supports them. Keep the first GPU experiment to foliage; pursue wood expansion only if the remaining measured cost requires it. A whole-generator GPU rewrite is not the starting commitment. Put reusable improvements in the shared generation and rendering boundaries, with browser and native consumers using the same algorithms where applicable. Account for consumers requesting CPU geometry or structural data as well as GPU-resident rendering; do not obtain a website-only win by forcing other consumers through unnecessary representations or readback.

## API Contracts

- Preserve the consumer's ability to request a mature specimen from species parameters and a seed. Its representation and generation interface may evolve; asynchronous generation is allowed. [inferred]
- Shared botanical inputs remain engine-independent and rendering changes apply across supported parameter values without species-specific code. [strategy:The core and integration]

## Edge Cases & Constraints

- Count preparation, allocation, transfer, synchronization and delivery of the requested representation in the candidate's total; include first-frame work for rendering consumers. Validation readback may run separately, but production readback cannot be excluded from its timing. [inferred]
- Distinguish cold consumer initialization from generation with an initialized engine or renderer. Report cold page startup separately for the website use case. Neither a kernel timing nor a warm timing establishes instant cold delivery. [inferred]
- Missing GPU support, device loss, allocation limits and invalid parameters must leave an explicit failure and a usable consumer lifecycle, rather than partial foliage represented as a completed tree. [inferred]

## Acceptance Criteria

- **R1:** Record a fresh baseline and stage breakdown for existing mature browser and native paths on multiple seeds of at least one broadleaf and one needle-bearing species. State generator revision, species, seeds, size, requested representation, viewport where applicable, runtime and hardware; include a desktop and a phone-class device. Record cold startup, initialized-engine time to the requested output, first completed frame for rendering consumers, latency distribution and accounted peak memory. Errors: unavailable devices or unmeasurable memory domains remain named gaps, never passing evidence. [inferred]
- **R2:** Evaluate the first bounded candidate against the baseline and fn-12's failure modes. A GPU foliage candidate must replace CPU expansion and keep its resulting buffers available to rendering, rather than repeat the old CPU-build/upload/query/readback path. Report total latency and cost by stage. Errors: an isolated kernel gain with no end-to-end gain rejects that candidate; record the limiting stage and stop it before expanding into wood or botanical growth. [inferred]
- **R3:** Prefer byte-identical output for performance improvements when practical and use exact comparison to simplify verification. This preference must not block a worthwhile measured gain: a candidate may change output bytes while introducing no perceptible visual regression. Compare the supported whole-tree and close views and relevant motion across the tested species and seeds, and record the owner's visual verdict. Retain exact checks where unchanged output is intended. Errors: an unexplained visual regression rejects the candidate; a byte mismatch alone does not. [paraphrase]
- **R4:** Preserve botanical plausibility, foliage attachment, valid geometry and the consumer's relevant correctness requirements. Keep meaningful repeatability checks within the stated implementation and update changed baselines with evidence. Errors: missed spatial contacts, detached foliage, invalid geometry or silently truncated trees cannot be excused as byte differences. [inferred]
- **R5:** Integrate the qualifying improvement into the shared mature-generation or rendering path, demonstrate its use by the existing browser and native consumers where applicable, and rerun the same specimen matrix against the baseline. State which requested representations benefit and measure any regression in retained CPU-output paths. Meet the latency and memory acceptance bounds settled before candidate tuning, reporting absolute values and before/after differences. Errors: a missed bound or unsupported case is reported explicitly; a failed experiment does not count as delivered fast generation. [inferred]

## Boundaries

- The engine improvement supports multiple species and random seeds; choosing only a curated list of seeds is not a substitute for improving generation. [paraphrase]
- Website layout, rotation behavior, deployment, wind animation and scroll-driven growth are separate work. This spec improves reusable mature-tree generation and delivery. New external-engine adapters are separate work; existing native and browser consumers supply the initial proof. [inferred]
- Byte-identical output is preferred when practical for performance improvements, not required across intentional improvements or CPU/GPU backends. Relevant visual and correctness requirements remain. [paraphrase]
- No commitment to move all generation to the GPU, add a forest renderer or revive the rejected field-query implementation. [inferred]

## Decision Context

- fn-12 tested GPU queries over CPU-built spatial indices, not GPU foliage or wood construction. Cold preparation and transfers erased the gains; one resident giant-grid query improved from 25.4 to 11.8 ms, while contact queries still missed cells. These are historical results, not estimates for this candidate. [inferred]
- The new experiment targets expansion that can stay on the GPU. Its acceptance is end-to-end speed with preserved perceived quality, so it must demonstrate that the old overhead problem has actually been avoided. [inferred]
- The owner prioritizes shipping and substantial engine improvements over freezing today's generated output. GPU execution is a candidate mechanism, not a reason to accept a slower path. [paraphrase]

## Parked unknowns

- The concrete latency and memory bounds and device matrix must be settled before candidate tuning. Under 100 ms from parameters to a drawable tree with an initialized renderer was an agent proposal, not an owner-approved requirement; cold-load bounds are separate.
- Current stage shares and the achievable GPU gain are unmeasured. The baseline resolves these; historical species costs cannot qualify the new path.

## Strategy Alignment

The core and integration permits compact engine-independent descriptions expanded on the GPU. Surface and rendering at scale requires measured runtime cost and fidelity across generated trees. The updated evolution policy permits faster implementations with changed bytes and no perceptible visual regression.
