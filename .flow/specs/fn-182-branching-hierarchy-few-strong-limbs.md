## Conversation Evidence

> owner (2026-09-27, on fn-177's plane candidate beside the Schützenmattpark photograph): "The structure of those trees is much different. Now we have 1 super thick center and then it tapers off to be thin very quickly anyway. The plane i sent has very thick multiple leaders and thick branches from that. Then there's actually very few branches off that secondary level that lead to many twigs with leaves."
> owner: "Is thie branching hierarchy part of 177? i think it would also fix the beech"
> owner (2026-09-28, on a beech Tune candidate): "why arey ou showing me this abomination?" (a crown that is a ball of long straight twigs with no limb structure)
> owner (2026-09-28): "ok merge it and spec"; fn-173 is being built, so this builds after it.

## Goal & Context
<!-- scope: business -->

Open-grown broadleaves read by their branching hierarchy: a few strong scaffold limbs that carry their girth far out, sparse branching in the middle orders, and dense, short twigs concentrated at the crown's edge. The owner's London plane photograph (`.flow/references/london-plane/schutzenmattpark-basel-2026-09-27.jpg`) and the beech's references show it. The generator does not: fn-177 measured that on fn-170's plane candidate each lateral an axis bears takes its share of the pipe (96 percent of the thinning), so every limb is born thin and branches everywhere; the beech's two runner Tunes (fn-62, 2026-09-27) kept nothing and their best candidates were crowns of similar-sized sticks with no limbs. After fn-61 (limb angle by height, ragged reach), fn-170 (codominant forks), fn-177 (held girth) and fn-180 (even finish at the budget), this is the capability the beech still lacks. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-28 on master.** The scaffold's lateral rows are `lateralsPerStation`, `lateralSpacing`, `lateralLengthRatio`, `lateralOrders` and `apicalDominance` (`crates/telperion-core/src/pipeline/branching/traits.rs:25-162`); the twig layer's are `laterals` per station, `stationsPerInternode`, `lengthRatio`, `ratioPower` and `generations` (`crates/telperion-core/src/pipeline/twigs.rs:50-147`). Each is one value per family. [checked]

**Unknown until measured.** Whether any of these already varies by order in effect (through `lengthRatio`/`ratioPower`, vigour or the scaffold's room), and what the beech and the plane need per order. [unknown]

**Measure first (R1).** On the shipped beech, fn-170's plane candidate and the oak at seeds 1 to 4, per branch order: laterals per metre of parent, lateral length against parent length, and girth at birth against the parent's; and, from the owner's two photographs and the beech's curated B-BARE, the host's read of the same three quantities per visible order (few / many, long / short, thick / thin). The report names which quantity differs and at which orders. [inferred]

**Shape, decided by the host from the measurement.** Most likely a per-order profile on the rows that differ (density, length and birth girth falling or rising with order), each dormant at today's value and changing the tree by degree, declared once in the catalogue; or a vigour-weighted lateral allocation if density alone does not produce few strong limbs. The spec is updated with the decision before any code. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The per-order measurement of the three trees and the photographs is reported, naming the quantities that differ and at which orders. [inferred]
- **R2:** The host's design decision is recorded in this spec before code: the rows, their dormant values, and why they reach the photographs' hierarchy. [inferred]
- **R3:** The new rows are dormant at their neutral values: every shipped preset is byte-identical to master at seeds 1 and 7; they are declared once in the catalogue, blended, on the dials, refused by name off their rails. [inferred]
- **R4:** A synthetic family shows the profile: few, long, thick first-order limbs and many short twigs at the tips, continuous as the rows move. [inferred]
- **R5:** Candidate values for the plane and the beech (host-chosen, not shipped presets) are rendered and opened in the harness (`species --look` / `harness/looks/`) beside the owner's photographs, with one question: does the structure now read like the photographs? [inferred]
- **R6:** The workspace gate and `npm test` are green; build time, peak memory and every shipped artifact's size are reported (the size check's 5 percent per PR applies). [inferred]

## Boundaries
<!-- scope: business -->

- Not the beech's values (fn-62) or a London plane preset. Not girth (fn-177) or forks (fn-170). Builds after fn-173 (growth reads a prepared crown shell), which reworks the same growth code.

## Strategy Alignment

- Serves "Mature trees are the product" and "Growth and botanical fidelity": the owner judges trees by their structure first. [strategy:Our approach]
