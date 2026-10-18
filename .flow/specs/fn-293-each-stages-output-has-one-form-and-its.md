# Each stage's output has one form, and its consumers read it

## Goal & Context

The owner (2026-10-16), after the field turned out to rebuild its own copy of the drawn tree: "we defined we wanted a clean pipeline with consumers that take the output from previous stages" and "how much clearer do i need to be?". The host's audit (`PIPELINE-AUDIT.md` beside this spec; checked by grep and an adversarial reviewer, no measurements) found eighteen places in the release where a consumer rebuilds a stage's output, a second chain re-derives it, or code no production path reaches is compiled in. The field itself (finding 1, both executors) is fn-257's R8.

## Requirements

Every change keeps every shipped tree byte-identical (presets at their ages, seeds 1, 7, 42) unless a finding says otherwise; each is measured where it costs time or memory.

- **R1 (copies in the drawing path):** the device sweep reads the drawn records instead of copying them into its 39-word node table (finding 2); the sweep writes the renderer's curve layout directly, without a handoff pass (finding 3, re-questioning fn-212 decision 2 with its reason); a mesh request sweeps the wood once (finding 7).
- **R2 (retired and parallel routes out of production):** the scaffold grower and `validate_skeleton`'s scaffold preconditions leave production, and `Tree.pipe`'s fallback goes (finding 4; fn-288 keeps the test migration); the experimental growth passes and their hooks in the production grower go (finding 5); the CPU delivery route goes if nothing in production reaches it (finding 6); test-only WGSL leaves the production modules (finding 10); the zero-reporting branch diagnostics are deleted (finding 8).
- **R3 (one derivation of each quantity):** the station count is called, not restated (finding 12); one preparation of an `Expansion` (finding 13); the kept recursion and its reduced predictor, the node-count prediction in core, the unquantised light field and production-compiled test-only items are each kept only where production reads them (findings 11, 15, 16, 17).
- **R4 (carried-over contracts):** the spec, README and package requirements the audit lists as preserving a pre-engine format without a named owner (fn-257 R1/R1a beside R8, the CPU field snapshot as API, the `{structure:true}` layout's scaffold column, "twelve bytes a leaf", the three compatibility rows, the inferred byte-identity requirements in fn-15, 28, 123, 125, 126, 171, 176) are reconciled: kept with a named reason or removed.
- **R5:** gate and npm test green; codex review per task; lines, bytes and build time before and after.

## Boundaries

Release item (fn-237), after fn-257's R8. Findings 9, 14 and 18 (test-only species constructors, capability thresholds, the harness's own checks) are fn-288's and after the release.

`scripts/benchmarks/field.mjs`, `scripts/benchmarks/generation.mjs`, `generation-inputs.mjs` and `experiments/voxel-field/voxels.mjs` still call the field query fn-257 removed, and `tests/migration/README.md` still mentions `{ field: true }` (host, 2026-10-18, fn-257 landing); fn-293 removes or migrates them.
