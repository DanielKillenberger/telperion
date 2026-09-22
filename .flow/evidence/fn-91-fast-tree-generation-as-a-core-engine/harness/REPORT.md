> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Interactive renderer integration

The mature harness now calls the resident GPU API. A per-stage queue allows one active build and one replaceable pending request; effect cleanup and disposal suppress stale UI callbacks, and only the latest completed request updates statistics and camera framing. The unused synchronous Stage forwarding method is removed. CPU fallback/output and opt-in growth remain engine capabilities.

Live QA exercised the actual React page: oak seed 1, rapid edits 17/18/19, spruce, and oak again. All four executed builds reported Gpu and gpuPositions=true. Intermediate edits coalesced to seed 19; maximum active builds was one, synchronous CPU calls zero, live devices one, and alerts/page errors zero. Screenshot confirms a rendered exterior tree. Observed API durations were 501/274/247/424 ms while the live render loop ran; these are diagnostic UI observations, not replacements for the controlled completed-frame benchmark. Initial module/device loading and shader compilation remain separate startup costs.

Focused scheduling and parameter tests passed 34/34; TypeScript checking passed. Source review confirms growth handling is unchanged. The original full Rust/JS gate remains recorded for the engine change; this harness-only fix adds focused async scheduling validation.

The initial browser probe encountered a blank navigation and then missed Vite timestamp query strings in its instrumentation route. Navigation and instrumentation guards now distinguish those setup failures from generator failures. The next probe also incorrectly read top-level seed instead of skeleton.seed; the corrected probe records the actual requested seed. These were QA-script issues, not production changes. Failed logs remain beside live.json; the reproduction script is live.mjs.
