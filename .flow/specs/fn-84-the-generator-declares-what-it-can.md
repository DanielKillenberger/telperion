## Goal & Context
<!-- scope: business -->

The species pipeline's capability gate is meant to answer one question: can the generator express what this species needs? It does not answer it. It asks an already-registered preset what its own value table produces, derived from thresholds on that preset's parameters, and uses the answer as if it described the generator. For a species that has no preset yet, which is every new species at the moment the question matters, the probe returns an empty list and exits zero, so nothing is ever missing and the gate passes silently. [paraphrase]

The date palm run of 2026-09-18 is the worked example. Its capability assessment named six needs; the gate reported all six missing, including `woody-axes`, which the generator has and every preset declares. The one true finding, five unmet organs and placements, arrived mixed with a false positive that would have sent the gap loop to write candidate fixes for a capability that already exists. [paraphrase]

This spec splits the question in two. The generator declares what it can express, once, as a versioned list that no preset owns. The species' required list is compared against that. The preset-derived list survives as a different and still useful check: whether a registered preset's table actually produces what it claims. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The vocabulary is a declared list in the core, not a derivation.** One entry per capability the field can express, each with its name and a one-line statement of what the name means, in the same register as the existing names. It is the answer to "what can the generator do", so a capability the generator cannot express is simply absent; there is no false entry to read. [paraphrase]
- **The gate compares required against the vocabulary.** `missing` is the required names the vocabulary does not carry. The preset's registration state no longer enters that computation, so a species with no preset yet is assessed on its needs rather than on its absence. [paraphrase]
- **The preset-derived list keeps its own question.** For a registered preset it still answers whether that table produces the capabilities it claims, which is a real check for a species that has shipped. It never gates a species that has no preset. [inferred]
- **The vocabulary carries a version, and the gate records it.** The runbook's capability round records the vocabulary version it assessed against; the gate's artifact carries the same version, so a round and the gate that follows it can be compared. [paraphrase]
- **A landing gap spec adds its name.** When a spec gives the generator a capability, adding its name to the list is part of that spec, one line, with the test that failed before it. That is the mechanism by which a later assessment round sees the world change. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The generator declares its capability vocabulary as one versioned list in the core, independent of any preset, each name carrying a one-line meaning; the six names the existing derivation can produce all appear. [paraphrase] Errors: a duplicate name is refused at build or by test.
- **R2:** The gate computes `missing` from the species' required list against that vocabulary. On the date palm's recorded required list, `woody-axes` is not missing, and `apical-rosette`, `pinnate-frond`, `acanthophyll`, `persistent-leaf-base` and `infructescence` are. [paraphrase] Errors: a required name that is not a vocabulary name and not a known organ is reported as unrecognised rather than silently missing.
- **R3:** The preset-derived capability list survives as its own check over a registered preset and never contributes to the gate's `missing` for a species without a preset; the oak, the spruce and the birch report what they report today. [paraphrase]
- **R4:** The gate's artifact records the vocabulary version it compared against, and the version changes when the list changes. [inferred]
- **R5:** The gate fails closed: a capability check that cannot be performed files a decision naming why, and never passes. [paraphrase] Errors: a probe that errors is that case, not an empty list.
- **R6:** A capability absent from the vocabulary is added by the spec that implements it, in one line, with a test that fails before the implementation and passes after. [inferred]

## Boundaries
<!-- scope: business -->

- No generator or renderer behaviour changes, no new parameters, no preset value moves and no identity pin moves. [paraphrase]
- The architectural-model coverage file, fn-35's list of twenty-three models, is not this spec. This spec names capabilities the generator expresses, not the models the catalogue classifies. [paraphrase]
- The organs the date palm needs are not implemented here. They are absent from the vocabulary, which is what makes them missing, and each becomes its own spec. [paraphrase]

## Decision Context

- The owner chose on 2026-09-19 to fix the false positive systemically rather than patch the unregistered-preset case, because the patch would leave the same confusion in place for registered presets. [user]
- The date palm's recorded artifacts are the fixture for R2, so the criterion is checked against a real run rather than a constructed case. [paraphrase]
