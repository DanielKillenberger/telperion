# The generator declares what it can express

## Goal
The species pipeline's capability gate asked a registered preset what its own value table produces and treated that as what the generator can do; for a species with no preset yet it returned an empty list and passed silently. The date palm run (2026-09-18) reported all six needs missing, including `woody-axes`, which the generator has. This spec splits the question: the generator declares one versioned capability vocabulary no preset owns, and the species' required list is compared against it; the preset-derived list survives as a separate check of a registered preset.

## Decisions
- Owner, 2026-09-19: fix the false positive systemically rather than patch the unregistered-preset case, because the patch would leave the same confusion for registered presets.
- The date palm's recorded artifacts are the R2 fixture. A spec that gives the generator a capability adds its name to the vocabulary, one line with a failing-then-passing test.
- Boundaries: no generator, renderer, preset or pin change; the architectural-model coverage file is not this spec; the palm's organs are each their own spec.

## Acceptance Criteria
- **R1:** One versioned vocabulary in the core, independent of presets, each name with a one-line meaning; duplicates refused — met (`capability.rs`: `EXPRESSED` 8 names, `UNEXPRESSED` 7).
- **R2:** `missing` is required names against the vocabulary; on the palm `woody-axes` is met and the five organs missing; unknown names reported as unrecognised — met.
- **R3:** The preset-derived list survives as its own check and never feeds `missing` — met (filed as `preset-capability`).
- **R4:** The gate artifact records the vocabulary version, which changes with the list — met (FNV-1a over names and list membership).
- **R5:** The gate fails closed; a probe error files a decision — met.
- **R6:** A capability is added by the spec implementing it with a before/after test — met (documented contract).

## Result
`geometry_benchmark --vocabulary` prints the vocabulary with its version. A defect the fail-closed rule exposed was fixed: the generate stage wrote `packet/species.json` with an empty `required_capabilities`, which the gate preferred over the manifest's row; the gate now reads the manifest when the packet names nothing. PR #43, commits d4244947, 6483e80b, c9978710.

## Open
None.

## Friction
- Machine had only system Rust 1.85, no rustup; errors read as generator breakage -> local setup, not specced.
- `species` suite OOM-killed at 5 GB in one binary; SIGKILL reads as a test failure -> proposed a thread cap or bounded suite memory on the gate.
- Two-crate change paid fat-LTO release links twice (~10 min) -> proposed a local pre-PR gate on the `ci` profile (later adopted as the gate).
