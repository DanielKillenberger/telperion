> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Browser proof through the shared GPU generator

Host design for invocation 5, after 1e30cf22. The native foliage experiment has an end-to-end gain. This permits a bounded browser integration experiment; it does not qualify the memory target or authorize a new GPU wood algorithm. Retain the current acceptance criteria.

## Shared asynchronous implementation

Make the existing generation module available on wasm32. Keep one shader and preparation algorithm. Move common construction, preparation, compute, mass, validation readback and error-scope completion to asynchronous implementations. Preserve existing native synchronous entry points as pollster wrappers so native callers and benchmarks retain their contract. Browser execution must never block on pollster, channel recv or device polling. Use the existing wasm timing/readback patterns in timing.rs and web/session.rs: a futures-channel oneshot for callbacks, awaited mapping and queue completion. On native, poll the device when required to drive the callback before receiving. Await error-scope pop normally. Preserve all lifetimes and release ordering from GPU-PATH.md.

Replace generation's std::time::Instant with a tiny platform clock helper. Native uses Instant; browser uses the global performance.now clock, which works on window and worker globals. Keep milliseconds and elapsed-time stage definitions. No new timing dependency or wall-clock Date fallback is needed.

Provide a renderer-bound asynchronous constructor whose returned future owns cloned GPU/identity handles and does not borrow Renderer. For example, a normal function returning a 'static future can clone the handles before entering async move. This is required so the web binding never holds a RefCell borrow across await. Keep the standalone CPU-output constructor and its early Resident rejection.

## Explicit browser entry point and lifecycle

Add experimental setTreeGpu returning a Promise with the same submitted count/bounds JSON fields as setTree, plus an explicit backend field. Preserve setTree and the existing default harness behavior. The benchmark opts into setTreeGpu; no website or preset-specific routing is added. Unsupported supported-input combinations retain the documented CPU fallback and report its backend. Invalid input, resource/device failures and stale/disposed requests reject explicitly.

Put the new binding logic in web/generation.rs to preserve the module size convention. Keep a small shared generation state on WebRenderer, containing a revision counter, a pending flag and an optional cached Rc<Generator>. Start one GPU request at a time; concurrent setTreeGpu calls reject as busy. A small request guard owns the shared state and clears pending in Drop, including error paths. A successful synchronous setTree invalidates an outstanding GPU request by incrementing the revision. Disposal invalidates the revision and clears the cached generator before releasing Live. Do not clear pending on invalidation while GPU work is still in flight, which would allow overlapping generation/error scopes.

Parse parameters before marking a request pending. Clone the live renderer's construction context during a short borrow, then release it before shader setup or generation awaits. Reuse only the generator's device/pipelines, never a generated specimen. After awaits, check the revision and live renderer before committing. A stale or disposed result must leave the current tree alone. Submit the complete prepared tree and then apply its material within the same synchronous commit section. Keep all RefCell borrows short. The async future may retain GPU resources until its work resolves after disposal; it must not redraw or replace a disposed canvas.

## Bounded proof

First compile explicitly selected native generator tests and the renderer Wasm target. Reuse the existing native geometry/ownership tests through the synchronous wrappers; add only tests needed for the new request-state lifecycle. No full package test-target build or workspace gate. Record friction immediately.

Extend mature-generation.mjs with an opt-in GPU mode using await setTreeGpu. Preserve its existing completed-queue boundary, camera, viewport, specimen parameters and default behavior. First run a small browser smoke case and verify count/bounds, explicit fallback, invalid input, overlap rejection, superseding synchronous setTree, and disposal during pending work. Verify the resulting tree after stale rejection, not only the error text. Then run oak/spruce seeds 1 and 7 with the existing first-plus-five-warm protocol. Report browser results against the original comparable browser completed-frame baseline, never against native timings. If practical run the current CPU browser matrix to measure the shared CPU cache's effect separately; omit repetition if it would exceed the bounded experiment and name that gap.

Report actual module memory and available buffer accounting with the existing omissions. Do not turn process RSS or WebAssembly pages into a total device-memory claim. The native CPU-output memory miss remains explicit. Capture at most four close-view comparison images for this separate browser capture if the existing harness exposes a suitable close camera, one CPU/GPU pair for each species at seed 1. Keep owner review scoped to what was actually shown; the prior owner verdict covers native hero views only. Do not add camera/product features merely to produce a capture.

Checkpoint the working integration and all measured failures honestly. This is the fifth bounded worker invocation. If R1-R5 remain unmet, leave the task in_progress, record the remaining blocker in its task file and return NEEDS_HUMAN under CLAUDE.md's per-task budget. Do not start a sixth attempt, rescope the spec, mark completion, open a PR or merge.
