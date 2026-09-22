# The apical rosette: fronds borne only at the apex of an unbranched stem

## Conversation Evidence

> gap loop, round two on `date-palm/gate/onboarding-gate/capability`, 2026-09-22: the stronger set's winner `apical-rosette-of-pinnate-fronds`, route `owner` (row 0, table version 1, the winning fix moves a pin), ledger `689d507b53a140dcfbdded02`. Jev: best match 0.42 against none 0.34; change kind generator, generalizes.
> user (2026-09-22): "ok it's really hard for me to judge if the palm needs a apical rosette but if confidence is high let's go for it?" Resolved as the owner's confirmation on the gap-fix decision.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The date palm's gate halts on five capabilities the generator does not express. Two of them are the palm's crown: `apical-rosette`, fronds borne only as a rosette at the apex of a thick unbranched stem with no laterals and no twig clothing of the trunk, and `pinnate-frond`, one frond as leaflets along a rachis in one placement. The host's capability assessment of 2026-09-19 reads both as unsupported anatomy at high confidence: the canopy layer bears foliage on wood below a radius threshold along branch systems, and the element layer draws one blade, so nothing places a whorl of large compound leaves at one point. [paraphrase]

This spec gives the generator that crown, off by default so every shipped preset stays byte-identical, and it is minted by the gap loop as fn-82's dependency: the second spec the loop has minted, and the first that changes the generator. The trunk organs (persistent leaf bases, acanthophylls) and the infructescence are the gate's other three names and their own specs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The design is the strong tier's, not this spec's.** The conductor judged the gap's design complexity and routes the design to a high-reasoning model at medium effort; that design handoff resolves the interfaces, invariants, difficult cases and verification, and names what stays unknown. This section records only what the assessment checked. [host design]
- **What the code has, checked 2026-09-19 in the capability packet:** a single unbranched stem is reachable (`stems` 1, `lateral_orders` 0, `apical_dominance` 1, `length_taper` 0, landed as fn-108). Foliage placement keys on wood radius (`canopy.shoot_radius`, `foliage/placement.rs`); the element layer (`element`: length, width, lobe count, section roundness) draws one simple blade per placement; the twig layer clothes wood below a diameter. None of these places a rosette. [checked]
- **The shape the design must land on:** a rosette organ at the stem's apex bearing N fronds on a phyllotactic spiral, each frond a pinnate element (a rachis with leaflets, the frond's own length, arch and droop), with existing capability names `apical-rosette` and `pinnate-frond` moved into the vocabulary's expressed list in the one line the runbook names, with the test that failed before the implementation. Every parameter is a row on the family wire with a validated range and a doc comment, and appears in the dial table. [inferred]
- **Off by default.** With the rosette absent every shipped preset builds byte-identically; the beech, oak, spruce, birch pins and the catalogue tests are the check. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The palm run's gate, rerun at the landed commit, names `apical-rosette` and `pinnate-frond` as expressed and halts on `acanthophyll`, `persistent-leaf-base` and `infructescence` alone. Errors: a gate still naming either of the two means the vocabulary line was not moved or the capability is not produced by the value table.
- **R2:** Every shipped preset is byte-identical in mesh and metrics with the rosette absent. Errors: a changed pin on any preset is a defect.
- **R3:** The date palm at seed 1 builds with a rosette at the apex and no foliage elsewhere on the stem, and `species_measure` reports it within the species gates once fn-82's values are set; the palm's still, rendered by the headless renderer, is judged by the owner's eye and the reviewer, never by this spec. Errors: foliage on the stem below the apex, or a frond drawn as a simple blade, fails.
- **R4:** The new rows are in the dial table with meaning, range and steps, and the table's coverage test passes. The workspace gate is green.

## Boundaries
<!-- scope: business -->

- No trunk organs, no infructescence, no bark or material work; those are the gate's other three names.
- No species branch: the palm is a value table over the new capability like any other.
- Not the palm's acceptance: fn-82 owns the species and the owner's verdict on stills.

## Strategy Alignment

- Serves "The catalogue": a form the generator could not draw becomes a shared capability. [strategy:The catalogue]
