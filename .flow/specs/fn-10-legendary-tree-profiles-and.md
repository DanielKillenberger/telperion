# Legendary tree profiles and supernatural traits

## Conversation Evidence

> E1: "can we do this for certain \"magical\" trees also? famous ones? that have plenty of references."
> E2: "I think we should have all \"supernatural\" attributes be separated from natural ones. and by default have them off."
> E3: "separate spec is also fine"
> E4: "so each species has a profile that we work towards with the template"

> E5: "ofc the supernatural templates will have the supernatural traits on.."

## Goal & Context

Extend reference-driven profiles and templates to famous magical or legendary trees with ample reference material, while keeping natural anatomy distinct from optional supernatural traits. [paraphrase] (E1, E2, E4)

## Architecture & Data Models

Each profile identifies the natural base and the supernatural interpretation. Templates expose these as separate controls. General defaults and ordinary templates leave supernatural attributes off; supernatural templates explicitly enable the traits that define them. [paraphrase] (E2, E4, E5)

## Edge Cases & Constraints

Distinguish documented descriptions, visual depictions and estimated dimensions. Conflicting depictions require a declared target interpretation. A natural base need not reproduce features that exist only when supernatural traits are enabled. [inferred]

## Acceptance Criteria

- **R1:** Select a bounded set of famous fictional or legendary trees with ample attributed references and define profiles for their form, dimensions, branching and foliage. Identify insufficient evidence and label estimates rather than treating them as measured facts. [inferred]
- **R2:** Build a template for each selected profile and validate its identifying features across multiple seeds using visual and quantitative comparisons. Record both the natural-base target and the supernatural reference interpretation; report mismatches in the mode being judged. [inferred]
- **R3:** Keep all supernatural attributes separately identifiable and controllable from natural attributes. General defaults and ordinary templates leave them off. Selecting a supernatural template enables that template's intended supernatural traits; the user can independently disable them. Disabling them restores the natural result for the same seed and natural parameters, without residual supernatural deformation. [paraphrase] (E2, E5)
- **R4:** Reuse the shared botanical and foliage foundation, adding only the profile-required geometry capabilities. Unsupported traits remain explicit unmet requirements; natural-template comparisons detect unintended effects on ordinary species. [inferred]

## Boundaries

This pass covers geometry and procedural foliage. Supernatural materials, light emission, other appearance effects and lifecycle simulation remain later work; their absence must not be presented as complete visual reproduction. [inferred]

## Decision Context

Magical subjects can have a separate spec. [paraphrase] (E3)

Depends on “Real-species profiles and procedural templates” for the shared profile, template and validation foundation. [inferred]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | TBD during planning |
