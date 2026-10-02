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

**Decision (host, 2026-09-28, from R1's measurement in `.flow/evidence/fn-182-branching-hierarchy-few-strong-limbs/R1-MEASUREMENT.md`).** [checked]
- The scaffold is already sparse (4 to 8 laterals per parent; 0.5 to 0.7 per metre at orders 2 and 3, near the photographs). The difference is the twig layer: it bears shoots at 3.7 to 10.8 per metre along every scaffold limb, inner wood included, where the photographs show bare inner limbs and a twig shell 1 to 3 m deep at the crown's edge.
- **The capability is a twig shell.** Two rows in the twig layer: `twigShell`, the depth from the crown's outer surface, as a share of the crown's radius, within which the twig law bears shoots on scaffold wood; and `twigShellSoftness`, the width of the fade at that depth. Unset `twigShell` is today's law (no shell), so every shipped preset is byte-identical; walking it moves the tree by degree. It reads the crown's outline geometry for depth (not fn-173's table, Astra review 2026-10-02), and builds on fn-183's twig layer, which changes the same code.
- **Not new rows:** the plane's thin first-order limbs (born at 0.18 of the parent against 0.5 to 0.7 in the photograph, because each stem carries about 22 first-order laterals where the photograph shows 3 to 4 leaders) are values: `lateralSpacing` and `lateralsPerStation` on the stems, (the claim that removing inner twigs leaves the remaining limbs thicker was wrong: the pipe solve sums only nodes before the crossover, `radius.rs:169`; Astra visual review, 2026-10-02). The beech's 32 m single stem, where its reference dissolves into 4 to 6 limbs at 30 to 40 percent of the height, is `codominance` with a fork height near 0.3 (fn-170). Both are values the beech's Tune and the plane's candidate set.
- Length ratios differ least and are left alone.

**Build choices (host, 2026-10-02, from `R5-RESULT.md`).** [checked]
- **Depth reads the smooth outline,** as shedding, the cull and the fill do: the smaller of the horizontal room and the distance to the smooth profile, read once per seeding pass. A lobed depth would move 0.7 to 2.4% of the beech's stations across shells 0.15 to 0.4.
- **The fade runs past `twigShell`,** so a shell of 1 is a pure neutral at any softness; R4's "bears no shoots deeper than `twigShell`" holds past `twigShell + twigShellSoftness`, and exactly at softness 0.
- **The neutral is a value, `twigShell` = 1** (the top of its rail), not an absent option.
- **Laterals only:** a scaffold tip deep in the crown still grows its terminal twig. Wood below the crown base counts as outside the outline and bears. A deep station bears no hanging shoots either; no shipped hanging preset sets a shell. On the hidden growth path depth reads that year's outline.

**Held (host, 2026-10-02, after the owner's verdict and Astra's visual review in `ASTRA-VISUAL-REVIEW.md`).** The R5 stills show the shell does not give the beech its hierarchy: the beech still reads as long straight rods, and the plane is "slightly closer but not there at all" (owner). The rods are rule-grown: scaffold laterals take the whole parent axis's length times `lateralLengthRatio` (0.65 on the beech) wherever they depart, with 3 degrees of crookedness (`scaffold.rs:317`). The PR is held. Next is a bounded comparison on the beech with the shell neutral, using existing rows (`lateralLengthRatio`, `raggedReach`, `crookedness`, `pitchVariation`, `codominance` with `forkWays` near 2, the short-shoot radius and spacing, `limbRadius`, `sheddingThreshold`), measured on a few traced branch systems (daughter thickness and length against the parent's remaining extent, length between substantial divisions, curvature, departure-angle variation, codominant continuations included); then a shell ablation on the best candidate. The shell is kept only if it adds a distinct improvement after that. Two limits Astra verified in the code may become gaps if values cannot reach the look: forks are placed by absolute tree height, so a limb cannot fork along its own course; and deeper laterals take a fixed share of the parent's whole length, with no term for where they depart. [checked]

**Shape (superseded by the decision above).** Most likely a per-order profile on the rows that differ (density, length and birth girth falling or rising with order), each dormant at today's value and changing the tree by degree, declared once in the catalogue; or a vigour-weighted lateral allocation if density alone does not produce few strong limbs. The spec is updated with the decision before any code. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The per-order measurement of the three trees and the photographs is reported, naming the quantities that differ and at which orders. [inferred]
- **R2:** The host's design decision is recorded in this spec before code: the rows, their dormant values, and why they reach the photographs' hierarchy. [inferred]
- **R3:** The new rows are dormant at their neutral values: every shipped preset is byte-identical to master at seeds 1 and 7; they are declared once in the catalogue, blended, on the dials, refused by name off their rails. [inferred]
- **R4:** On a synthetic family, scaffold wood deeper than `twigShell` from the crown's surface bears no twig-layer shoots, wood within it bears them as today, the fade is continuous over `twigShellSoftness`, and the tree moves continuously as the rows move. [inferred]
- **R5:** Candidate values for the plane and the beech (host-chosen, not shipped presets) are rendered and opened in the harness (`species --look` / `harness/looks/`) beside the owner's photographs, with one question: does the structure now read like the photographs? [inferred]
- **R6:** The workspace gate and `npm test` are green; build time, peak memory and every shipped artifact's size are reported (the size check's 5 percent per PR applies). [inferred]

## Boundaries
<!-- scope: business -->

- Not the beech's values (fn-62) or a London plane preset. Not girth (fn-177) or forks (fn-170). Builds after fn-183 (the twig layer grows without the crown as a wall), which reworks the same twig-layer code (host, 2026-10-02; was fn-173).

## Strategy Alignment

- Serves "Mature trees are the product" and "Growth and botanical fidelity": the owner judges trees by their structure first. [strategy:Our approach]
