## Conversation Evidence

> user (2026-09-15, on the beech round-5 pair): "Second trunk is missing but that's coming later that's fine."
> worker read, fn-38 round 6 (2026-09-15): "the parting is a symmetric V, because both stems are spread about one bearing and lean by the same angle, so neither is the near-vertical stem the photograph's pair has."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 55% [paraphrase], 25% [inferred] -->

fn-38 gave trees a clump of stems, spread about one bearing and leaned by one angle, so a two-stemmed birch parts in a symmetric V. The photograph's pair is one near-vertical stem and one that leans far out before it rises; most clumps are like that, a dominant stem and lesser ones pushed aside. [paraphrase]

This spec lets a clump's lean spread across its stems as a row, with zero reproducing fn-38's even lean. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **A spread row.** `skeleton.habit.stem_lean_spread`, 0 to 1, neutral 0: stems are ordered by the scaffold's own stem order, and stem i leans by `stem_lean` times one minus the spread times the share of the clump before it, so at 1 the first stem stands upright and the last leans by the whole angle; divergence stays about the seed's bearing. [inferred]
- **The rest is fn-38's.** Trunk runs, the pipe share of the base, twig eligibility and the degeneracy refusal are unchanged. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The row has a rail of 0 to 1, neutral 0, refused by name outside it, on the wire, blended, in the regenerated browser metadata and on the harness; every shipped preset byte-identical at neutral. Errors: a value outside the rail is refused naming the field. [inferred]
- **R2:** With a positive spread, stem leans spread from `stem_lean` times one minus the spread to `stem_lean`, in stem order. Errors: two stems left on one heading are refused as in fn-38. [paraphrase]
- **R3:** The silver birch's table states a spread and a lean against S-WHOLE and S-BARE, its pairs are rendered again, the implementer does visual QA before returning, and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** Tests: neutral byte identity, the rail, the leans in order on a synthetic clump, and a blend walk. Errors: a missing case is a review finding. [inferred]

## Boundaries
<!-- scope: business -->

- No coppice, no stem death, no roots. [paraphrase]

## Resolved via Codebase

- The clump: `crates/telperion-core/src/branching/scaffold/stems.rs` (bearing from the seed, `stem_divergence` between neighbours, `stem_lean` outermost by all of it, those between in proportion); rows in `crates/telperion-core/src/branching/traits.rs`.
- The birch's clump: `crates/telperion-core/src/presets/species.rs` (two stems, lean 22).
