## Conversation Evidence

> owner (2026-09-27, fn-170 session, Basel Schützenmattpark photograph): "I feel our generator fundamentally lacks the ability to have strong limbs that quickly taper at the edge of the crown to leaf level."
> owner (2026-09-27, on fn-170's R7 London plane candidate): "birch looks good and the plane looks decent but yes it thins out too quickly for sure"

## Goal & Context
<!-- scope: business -->

A London plane's limbs, and a beech's, stay thick almost to the edge of the crown and then break into fine wood over a short distance. The generator's limbs thin steadily from the fork, so by mid-crown they have already become fine wood. fn-170 made the plane's codominant forks possible; on its 8-seed candidate (oak base, 26 m, `lengthTaper` 0.2, `lateralShare` 0.45) the owner confirmed the forks read and the limbs thin too early. This spec lets a table hold a limb's girth over most of its reach and taper it to leaf level near the crown's edge. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-27 on the fn-170 branch (PR #131), merged to master as `ed23f773`.** The radius solve (`crates/telperion-core/src/pipeline/radius.rs:121`) thins wood two ways: bottom up, each node's pipe is its children's pipes combined through `forkExponent`, so every lateral that leaves takes its share (scaled by fn-170's `lateralShare` and `forkBalance`); and `lengthTaper` adds an exponential thinning over the path length from the root. There is no row that makes a limb's girth depend on how far along its own reach a point is. [checked]

**Measure first.** Which of the two thins the plane's limbs by mid-crown is not known. The implementer measures radius against distance along the thickest limbs of fn-170's plane candidate (`.flow/evidence/fn-170-codominant-forks-one-rule-from-the/plane-candidate.json`), split into the tip-count part and the length-taper part, and reports it before building. [unknown]

**Shape, to be settled from the measurement.** A limb-profile row pair: the share of a limb system's reach over which its girth is held, and how fast it falls to twig size after that. It acts per limb system (fn-61's limb bound gives each system its reach), is dormant at zero (today's law), and changes the tree by degree. Where the pipe model would make a held limb thinner than its held girth, the row decides which wins, and the conservation that is given up is stated. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The measurement is reported: radius along the thickest limbs of the plane candidate at 8 seeds, split into the tip-count and length-taper parts. [inferred]
- **R2:** The rows exist with rails, validated by name, on the wire, blended, in the browser metadata and the dial table; at zero every shipped preset's skeleton, mesh, leaves and field match master. [inferred]
- **R3:** On a synthetic limb, girth stays within a stated tolerance of its base value over the held share and falls to twig size over the rest; walking the rows moves the profile continuously. [inferred]
- **R4:** The plane candidate is rendered at the 8 seeds with the rows set, bare and in leaf, and the stills go to the owner with one question: do the limbs hold their girth to the crown's edge now? [inferred]
- **R5:** The workspace gate and `npm test` are green; build time and peak memory on the candidate are reported against fn-170's numbers. [inferred]

## Boundaries
<!-- scope: business -->

- Not a London plane preset: the plane is onboarded by its own species spec. Not the fork rule (fn-170). No change to how many twigs or leaves a tree bears.

## Strategy Alignment

- Serves "Growth and botanical fidelity": a limb that holds its girth to the crown's edge is the look of the open-grown broadleaves the catalogue needs. [strategy:Growth and botanical fidelity]
