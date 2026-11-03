# The coarsest tree's time in Wasm

## Conversation Evidence

> goal R2: "Every preset at grid 36 and seeds 1, 1234 and 5678, at least five warm samples each, has a median tree time at or under 50 ms and, for a species 0.1.4 has, at or under 0.1.4's."
> goal, the route now: "It reports time against both R2's 50 ms and 0.1.4's. If it reaches 0.1.4's time and cannot reach 50 ms, that is a question for the owner, since R2 needs both."

## Goal & Context

Child of goal fn-321, serving R2. Every time the run has so far is native, one thread: after fn-328 the oak's coarsest is 151 ms and the spruce's 558 ms on fifty tuning seeds, and the birch keeps fn-326's rows (222 ms on held-out seeds). The website runs the package's Wasm, whose cost is unmeasured, and the goal's question to the owner turns on it. This child measures the tree time the website would see, so the next attempt is chosen on the number R2 judges. [paraphrase]

The package's engine grows a family on the CPU in Node, with a field read to a grid of n cells and per-stage timings, as the 26 Oct audit's CPU validation does; the presets carry the fidelity and the coarse rows. [inferred]

## Acceptance Criteria

- **R1 (a frozen protocol):** the measuring script, the build it measures (commit and package files), the runtime and its version, the CPU and pinning, the warm-up and the sample count are committed in evidence before any number is read. [inferred]
- **R2 (the numbers):** for every preset at its preset age, at fidelity 1 and 0, grid 36, seeds 1, 1234 and 5678, at least five warm samples each, a results file gives the median tree time and the package's own stage timings, against 50 ms and, for oak, birch and spruce, against 0.1.4's time as the goal's evidence records it. Beside each, the native one-thread time of the same tree. A tree the package refuses is reported with its refusal. [inferred]
- **R3 (the reading):** the results say which presets are within 50 ms, which are within 0.1.4's time, the ratio of Wasm to native, and which stage dominates. If the oak, birch or spruce reaches 0.1.4's time and not 50 ms, it is written as the goal's question to the owner, with the measured numbers. [inferred]
- **R4 (verification):** where code outside evidence changed, the gate is green and the codex review of the child's diff ends SHIP; a measurement-only child records its review skip with the reason. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. [paraphrase]
- This is not R2's full benchmark: the browser, both tarballs, the grove plan and WebGPU on and off stay for a later child. [inferred]

## Decision Context

Settled by the goal: one grower; the coarse path on the CPU for every visitor; a fast tree held to the full cycle's statistics. A result is information for the next attempt and does not stop the run. [paraphrase]

## Open Questions

None.
