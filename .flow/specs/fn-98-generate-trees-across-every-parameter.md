# fn-98-generate-trees-across-every-parameter Generate trees across every parameter combination

## Conversation Evidence

> user: "the generator shouldn't fail for any param combo"

The accompanying renderer screenshot reports `invalid input: stems pass through each other = stem 1`. The exact seed and parameter set are unknown. Prior conversation compaction occurred; the quoted requirement and screenshot remain available.

## Goal & Context
<!-- scope: business -->

[paraphrase] Users can explore combinations of tree parameters without the generator rejecting the resulting tree. This is an engine requirement across consumers.

[inferred] The first regression case is multiple stems with coincident headings, including zero lean. Individually exposed parameter values currently combine into a generation error. The broader goal covers interactions throughout generation, beyond this one stem check.

## Architecture & Data Models
<!-- scope: technical -->

[inferred] Resolve geometric degeneracies in the shared generation core so browser, native and alternative output representations inherit the behavior. Inventory combination-dependent rejection paths before selecting resolution rules. Keep the requested parameters attributable alongside any effective interpretation needed to construct valid geometry.

[inferred] Prefer construction rules that remain well-defined at parameter boundaries. Choose how coincident stems resolve during design review; this capture does not select angle perturbation, stem merging or parameter clamping.

## API Contracts
<!-- scope: technical -->

[inferred] Provisional domain is every combination of supported finite parameter values and supported seeds. A geometric conflict within that domain produces a usable tree through a defined resolution rule instead of an invalid-input error.

[inferred] Separate malformed input, unsupported ranges and resource or device failures from combination-dependent geometric failures. The exact domain and error policy remain open decisions; they must not become a blanket exemption for inconvenient combinations.

## Edge Cases & Constraints
<!-- scope: technical -->

[inferred] Cover coincident stem headings, zero lean or divergence, maximum stem count, collapsed dimensions, fork and crown boundaries, and seed-dependent degeneracies. Audit the remaining generation stages for equivalent interactions.

[inferred] A successful result must satisfy the relevant structural and geometric invariants. Empty output, non-finite coordinates, silently discarding requested structure or substituting an unrelated preset do not establish robustness. Meaningful degenerate forms need explicit semantics.

[strategy:The core and integration] The same core serves all consumers. Resolution must respect runtime and memory budgets; no unbounded retry-until-success loop.

[inferred] Repeated input within a stated generator revision and backend should resolve reproducibly. Historical byte identity must not prevent better construction rules; use exact checks where outputs are intended to remain unchanged.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** [paraphrase] The generator produces a tree for every combination in its supported parameter domain without rejecting the combination because its generated geometry is degenerate or conflicting. Errors: explicitly distinguish inputs outside that domain and environmental failures; settle the domain before implementation rather than excluding previously supported combinations to pass this criterion.
- **R2:** [inferred] Multi-stem inputs with coincident headings, including zero lean and zero divergence, generate usable trees under documented resolution semantics. Errors: cover boundary counts and fork or crown interactions; preserve relevant geometry invariants rather than merely suppressing the rejection.
- **R3:** [inferred] Shared-core consumers and supported generation backends apply the documented combination semantics, and repeated requests are reproducible within a revision and backend. Errors: resolution terminates within defined generation limits; resource and device failures remain distinguishable from parameter-combination failures. Cross-backend byte equality is not required.
- **R4:** [inferred] Regression cases, parameter-boundary coverage and reproducible sampled combinations exercise every combination-dependent failure category found in the audit. Checks verify usable structure and finite, valid output, with seeds and parameters recorded for failures. Errors: a failed case remains a test failure, never a skipped sample or success through an unrelated preset; sampling alone is not a proof of the universal contract.
- **R5:** [inferred] Visual and performance evidence shows that resolution produces meaningful trees and preserves applicable fidelity and runtime requirements across materially different tree forms. Errors: blank trees, erased requested features, visual regressions and unbounded retries fail validation; intentional structural changes are reviewed using the current strategy's evidence policy.

## Boundaries
<!-- scope: business -->

[inferred] The scope is generation behavior across parameter combinations. New species, a new renderer and website-specific integration are outside it. UI-only restrictions cannot satisfy the engine requirement.

[inferred] Arbitrarily large or non-finite numeric inputs and unavailable hardware are not assumed to guarantee successful generation. Their exact handling is an open domain decision, not an approved restriction on the user's requirement.

## Decision Context
<!-- scope: both -->

[paraphrase] The user expects parameter exploration to succeed without learning combinations that crash generation.

[inferred] The stem rejection is a concrete starting point for an engine-wide audit. Removing that guard alone would leave the validity of the produced tree unresolved. Construction semantics and invariants must be reviewed together.

Related completed work includes fn-38 (multi-stem trees) and fn-48 (unequal stem lean). This spec extends robustness across the parameter space.

## Strategy Alignment

[strategy:The core and integration] Shared-core handling serves browser and native integrations. The strategy's continuous tree space requires combinations and interpolation to remain meaningful.

## Strategy Conflicts

None identified. The current strategy already allows intentional changes to generated structure and bytes, with correctness, visual and performance evidence.

## Parked unknowns

- Exact supported domain and treatment of malformed, out-of-range and resource-limited requests.
- Resolution semantics for coincident stems and other discovered degeneracies, including continuity during interpolation.
- Audit coverage, validation invariants and bounded cost of resolution.
