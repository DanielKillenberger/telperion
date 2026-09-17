# fn-58 plan review by GPT-5.6 Sol, 2026-09-16

Six rounds, read-only, through cursor-agent print mode with the model gpt-5.6-sol-high. Each prompt carried the CLAUDE.md rules excerpt, fn-57, the current fn-58 and the previous round's findings; round one also carried the strategy approach and the pilot results. Codex was unavailable on quota until 2026-09-19.

The host folded in every finding it agreed with between rounds and declined two: command names and paths in the spec (specs state contracts, the plan fixes names), and, in rounds two and three, the render-measured rule for described levels. The owner's TypeSafe section in CLAUDE.md gained that rule during the session, so the host withdrew the second objection in round four. A determinism probe between rounds two and three found identical answers with drifting probabilities on four identical calls, which shaped R6.


## Round 1

VERDICT: NEEDS_WORK

[P1] R6: The model-swap test does not measure cheap-driver success. Idempotence can make the second run reuse the first run’s outputs, while clean runs can differ because Firecrawl search, `jev-latest`, and ledger metadata are mutable. Use isolated uncached runs with pinned source bytes, tools, question sets, and manifests; compare canonical semantic outputs; record command compliance and unauthorized decisions; and require repeated successful trials with a named cheap model. Keep this live test outside workspace-gating tests.

[P1] Goal & Context: Load-bearing choices remain hidden in the input manifest. Source inclusion, search queries, taxon context, proxy species, site class, composition method, and value tables all determine the result before the command-only driver starts. Define a human-authored seed manifest as the pipeline boundary, or add typed selection policies and decision gates for each choice.

[P1] R3: The three parameter routes are neither complete nor compliant. Mapping Jev’s qualitative level directly through a numeric table can ship a number without the project-required code-proposed, render-measured candidate process. The routes also omit engineering parameters such as recruitment, shoot step, shedding, and foliage placement. Separate evidence fields from preset calibration; make qualitative judgments select candidate values that code measures; require owner acceptance where evidence cannot determine a value; and represent unsupported fields explicitly.

[P1] API Contracts: The decision list cannot reliably stop and resume the pipeline. It lacks stable IDs, typed per-kind payloads, enumerated options, status, resolution provenance, and input hashes; free-text `asks` leaves interpretation to the driver. Appending before a non-zero exit also conflicts with “no partial output” and can duplicate decisions on rerun. Define a discriminated union, atomic aggregate writes, stable deduplication keys, a separate human-resolution artifact, and explicit downstream blocking. Allow the report to describe `visual-unassessed` before the final stop.

[P1] R2: The Firecrawl contract assumes unproven capabilities. Scrape produces transformed markdown rather than guaranteed original response bytes, parse requires an already-local PDF, forestry coverage in the research index is unresolved, and a flattened table cannot report its lost row count without an independent expectation. Resolve the parked experiments first, checksum raw and transformed artifacts separately, specify PDF acquisition, final URL and content type, use fixture-known row counts, and define a typed unavailable-source decision instead of hard-coding Firecrawl as the only transport.

[P1] R4 / fn-57 dependency: fn-58 assumes tools fn-57 does not contractually provide, including qualitative-level scoring, source-to-field binding, source-quality selection, and packet-obligation verification. R4 also delegates deterministic invariants such as provenance presence and image-evidence typing to Jev. Add versioned question sets, labelled cases, and output schemas here or in an explicit dependency; enforce structural obligations in code and reserve Jev for semantic claims.

[P1] R1 / R3: “Typed JSON” is not testable without schemas, command names, paths, canonical serialization, or a complete cache key. Existing `species.json` is closed and cannot carry per-field spans and ledger references. Enumerate every artifact and schema version, place provenance in a sidecar keyed by JSON Pointer, and include source, manifest, question-set, model, and tool versions in idempotence checks.

[P1] R5: Curve fitting remains underspecified despite fn30’s known DBH failure. The spec does not fix the objective, parameter ranges, interpolation and extrapolation rules, unit conversions, age selection, or how height and DBH compete when one trait cannot fit both. Specify the algorithm and fixtures, gate every proxy and composition choice through a human decision, and test generated height and operational DBH independently at all three ages.

[P1] R7: The criterion is vacuous because either reproducing fn30 or merely listing every difference passes, while labelled-set accuracy has no required threshold. Define expected normalized outputs, minimum per-question accuracy and confidence behavior, mandatory dispositions for differences, and negative or holdout cases that can fail.

[P2] Boundaries: Filling `species.json` and `specimens.json` collapses onboarding’s capability assessment, implementation ownership, seed audit, integration, and resource-window gates into a literature pipeline. Preserve those handoffs explicitly and stop before generation when anatomy, registration, audited seeds, or measurement ownership is unresolved.

The central claim is plausible only after these fixes. Code can make a cheap model an effective command runner, but the current spec neither converts every load-bearing reading into a contracted Jev judgment nor proves success with R6; several human choices are merely hidden in manifests, tables, and source selection.

## Round 2

VERDICT: NEEDS_WORK
[P1] R6: PARTIAL - Isolated repeated trials are specified, but an unpinned Jev alias and exact comparison of stochastic probabilities prevent a reliable semantic equivalence test.
[P1] Goal & Context: RESOLVED - The human-authored manifest now explicitly owns every pre-run source, proxy, site, composition, and value-table choice.
[P1] R3: PARTIAL - Engineering parameters still lack a defined value-producing route, and described-level mapping conflicts with the project’s code-proposed, render-measured tuning rule.
[P1] API Contracts: PARTIAL - Decisions are typed and atomic, but resolutions are not bound to decision input hashes and can silently survive changed evidence.
[P1] R2: PARTIAL - PDF handling, row coverage, checksums, and unavailable sources are covered, but the spec still assumes without testing that Firecrawl supplies original response bytes.
[P1] R4 / fn-57 dependency: PARTIAL - Structural checks and new question sets are contracted, but fn-57 still has no acceptance criterion guaranteeing the selection tool fn-58 consumes.
[P1] R1 / R3: PARTIAL - Canonicalization and provenance sidecars are required, but concrete command names, artifact paths, schema names, and schema versions remain unspecified.
[P1] R5: PARTIAL - The objective and interpolation rules improved, but rate and shape ranges, grid resolution, tie-breaking, and the tolerance source remain undefined.
[P1] R7: PARTIAL - Expected artifacts, dispositions, and holdouts are present, but only statement-kind and citation-relation sets receive accuracy thresholds.
[P2] Boundaries: RESOLVED - Capability, registration, audited-seed, visual-verdict, and measurement-window handoffs remain explicit gates.

The central claim still does not hold: a cheap driver has fewer hidden choices, but engineering-value authorship remains undefined and R6 cannot yet distinguish semantic driver equivalence from mutable Jev output.

## Round 3

VERDICT: NEEDS_WORK
[P1] R6: PARTIAL - Probability drift is excluded from comparison, but fn-57 still exposes only an unpinned alias, so the repeated trials do not isolate driver equivalence from Jev version drift.
[P1] Goal & Context: RESOLVED - The manifest remains the explicit human-owned boundary for sources, proxies, sites, composition methods and value tables.
[P1] R3: PARTIAL - Engineering authorship is defined, but described-level mapping still conflicts with the project rule that “the value that ships is one code proposed and a render measured.”
[P1] API Contracts: PARTIAL - The architecture requires resolutions to carry decision input checksums, but the normative resolution shape omits them.
[P1] R2: RESOLVED - Adapter fidelity is now an explicit prerequisite, with raw bytes independently checksummed before PDF parsing.
[P1] R4 / fn-57 dependency: RESOLVED - fn-57 R3 now guarantees the typed selection tool consumed by this pipeline.
[P1] R1 / R3: PARTIAL - Artifact versions and canonicalization are specified, but concrete command names, paths and schema names are still deferred to the implementation plan.
[P1] R5: RESOLVED - Ranges, grid steps, tie-breaking, interpolation, extrapolation policy and the onboarding tolerance source are now explicit.
[P1] R7: RESOLVED - Every judgment set now has a stated held-out threshold, including source ranking, described levels and semantic obligations.
[P2] Boundaries: RESOLVED - Capability, registration, audited-seed, visual-verdict and measurement-window handoffs remain explicit gates.
[P1] R6: The byte comparison includes ledger references while excluding ledgers whose probabilities and timestamps may vary, so references need deterministic request-derived identities or explicit normalization to prevent semantically identical trials from differing.

The central claim still does not hold: the runtime driver has few hidden choices, but the described-level route violates the explicit render-measured shipping rule, and R6 still cannot cleanly prove driver equivalence independently of mutable Jev identity and ledger metadata.

## Round 4

VERDICT: NEEDS_WORK
[P1] R6: RESOLVED - Version pinning when available and otherwise interleaved trials adequately control Jev alias drift.
[P1] Goal & Context: RESOLVED - The manifest remains the explicit human-owned boundary for all substantive choices.
[P1] R3: PARTIAL - Candidates are now code-proposed and render-measured, but the deterministic rule selecting the shipped candidate is unstated.
[P1] API Contracts: RESOLVED - Resolutions normatively carry and validate the decision input checksums.
[P1] R2: RESOLVED - Adapter fidelity and pre-parse raw-byte checksumming remain explicit prerequisites.
[P1] R4 / fn-57 dependency: RESOLVED - The pipeline consumes fn-57’s typed selection contract.
[P1] R1 / R3: RESOLVED - The spec fixes behavioral and schema contracts while leaving implementation identifiers to the plan under project convention.
[P1] R5: RESOLVED - Fit ranges, steps, tie-breaking, interpolation, extrapolation and tolerance are fully specified.
[P1] R7: RESOLVED - Every judgment set has explicit held-out success criteria.
[P2] Boundaries: RESOLVED - Capability, registration, audited-seed, visual-verdict and measurement-window gates remain explicit.
[P1] R6 / ledger references: PARTIAL - References exclude volatile ledger fields, but their identity omits the model name or version and therefore can collide across materially different requests.
[P2] Architecture & Data Models: OPEN - The artifact contract says ten artifacts but enumerates thirteen, leaving the required artifact set contradictory.

The central claim nearly holds, but not yet: the driver still faces an unstated choice among render-measured described-level candidates, and ledger identities can conflate judgments from different model identities.

## Round 5

VERDICT: NEEDS_WORK
[P1] R6: RESOLVED - Version pinning or interleaved trials still adequately controls Jev alias drift.
[P1] Goal & Context: RESOLVED - The manifest remains the explicit human-owned boundary for substantive choices.
[P1] R3: RESOLVED - The in-range nearest-midpoint rule with table-order tie-breaking deterministically selects the shipped candidate.
[P1] API Contracts: RESOLVED - Resolutions carry and validate the decision input checksums.
[P1] R2: RESOLVED - Adapter fidelity and raw-byte checksumming remain explicit prerequisites.
[P1] R4 / fn-57 dependency: RESOLVED - The pipeline consumes fn-57’s typed selection contract.
[P1] R1 / R3: RESOLVED - Behavioral and schema contracts are fixed while implementation identifiers remain plan-owned.
[P1] R5: RESOLVED - Fit ranges, steps, tie-breaking, interpolation, extrapolation and tolerance remain fully specified.
[P1] R7: RESOLVED - Every judgment set retains explicit held-out success criteria.
[P2] Boundaries: RESOLVED - Capability, registration, audited-seed, visual-verdict and measurement-window gates remain explicit.
[P1] R6 / ledger references: RESOLVED - Ledger identity now incorporates the model name or version.
[P2] Architecture & Data Models: RESOLVED - The contract correctly states and enumerates thirteen artifacts.
[P1] Architecture & Data Models / R3 / R6: OPEN - The sidecar must contain Jev probabilities under R3 but must exclude them under R6’s byte-identical comparison contract.
[P1] API Contracts / R3: OPEN - Described-level range failures use tolerance-miss, whose required age/reference/error payload cannot represent the candidate set and target range unambiguously.

The previous selection and ledger-identity blockers are resolved, but the central claim still does not fully hold because the worker faces contradictory sidecar requirements and an incompatible decision schema for described-level misses.

## Round 6

VERDICT: SHIP
[P1] Architecture & Data Models / R3 / R6: RESOLVED - Probabilities now live exclusively in the ledger and are excluded from byte-identical artifact comparisons.
[P1] API Contracts / R3: RESOLVED - The level-miss kind explicitly carries the chosen level, target range, candidate values and measurements.

The central claim now holds: the pipeline leaves substantive choices to the manifest or typed human decisions while keeping model judgment attributable, deterministic outputs comparable and shipped values code-proposed and render-measured.
