# The ring at the birch's fork

## Conversation Evidence

> user (2026-09-16): "it looks like a potato at the bottom.."
> user (2026-09-18): "we have things to improve but this looks good"

## Goal & Context
<!-- scope: business -->

On the silver birch at seed 1 the single trunk flares from the ground, then ends at the fork about a metre up in a hard ring, and the two stems rise out of it like plants from a pot. The owner called it a potato in the harness on 2026-09-16, and it reproduces in the headless renderer with the leaves hidden (`.flow/evidence/fn34/measure/protocol-round26/silver-birch-1-S-FORK.png`), so it is in the mesh. The trunk below the fork is wider than the two stems it splits into, and the surface steps down where they meet. fn-48.3 was meant to make this fork clean and looked clean from the matched cameras, because none of them frames the base at this height and angle. [paraphrase]

The birch shipped at fn-34 round 26 with this defect named and set aside as generator work. It is a bug in the fork's surface and radius law, not a birch setting, and it stays visible on the mature path the harness now draws. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The defect is measured before it is fixed.** A base close-up at 2 m with leaves hidden, the S-FORK shot round 26 added, is the reference frame; the measurement is the radius profile up the bole through the fork, read from the node buffer: the trunk's radius just below the fork against the sum of the two stems' cross-sections just above it, and the surface's step at the junction. A real fork conserves cross-section roughly, so the parent's area near the sum of the children's is the target, and a step in the surface is the defect. [inferred]
- **The fix is in the fork's law, not in a preset.** Wherever fn-48.3 solves the fork collar and the radius hand-over from the clump to its stems, the parent radius at the fork follows the children and the surface is continuous through the junction. Presets are value tables and gain no birch branch. [inferred]
- **Every clumped species is re-pinned once.** The birch and any other preset with two or more stems moves; single-stem trees are byte-identical. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A test on the birch at seed 1 asserts the trunk's cross-section just below the fork is within a stated tolerance of the sum of the stems' cross-sections just above it, and fails on the code as shipped at fn-34 round 26. [inferred] Errors: the failure prints both areas and the ratio.
- **R2:** The base close-up (S-FORK) of the birch at seed 1 shows one trunk becoming two stems with no ring at the junction, judged by the owner in the harness on the mature path. [user]
- **R3:** Single-stem presets are byte-identical before and after; the birch's identity pins are re-recorded once with the reason. [inferred]

## Boundaries
<!-- scope: business -->

- The flat-ended limbs are fn-4. [paraphrase]
- No birch setting moves; the fix is the fork's law. [inferred]
- The growth path is not touched. [paraphrase]
