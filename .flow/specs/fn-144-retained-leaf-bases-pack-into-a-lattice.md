# Retained leaf bases pack into a lattice of flat-faced boots

## Conversation Evidence

> host, fn-80 capability assessment (2026-09-24): the palm's tuning revision stopped on the no-progress guard after 30 rounds; every round's reviewer names the trunk: "sparse, rounded cylindrical pegs set in widely spaced rows on an otherwise smooth trunk" where the references show "densely tessellated, blunt, flat-faced wedges in a spiral diamond lattice". The tuner moved `leaf_bases` 66 times (to 96), `leaf_base_length` 73 and `leaf_base_radius` 25 without closing it.
> owner (2026-09-24): specs are minted where the generator is missing a capability the species needs to be identifiable; the host classifies identity against improvement at capability assessment.

## Goal & Context
<!-- scope: business -->

A date palm's trunk is clothed in the boots of shed fronds: broad, flattened wedges cut off square, packed edge to edge on the crown's spiral so the trunk reads as a diamond lattice from any distance. Fn-110 gave the generator retained bases on the right spiral, but each is drawn as a round peg a share of the trunk's radius wide, so no dial setting packs them into a lattice. This spec gives a base the shape and the packing; the palm (fn-82) depends on it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `branching/leaf_bases.rs` appends each base as a two-node wood run after the radius solve, at an exact height on the stem polyline, on the rosette's spiral (`(rosette_fronds + k) * rosette_divergence`). Its thickness is `girth * leaf_base_radius` (row range 0 to 1), its lean `leaf_base_pitch`, its length `leaf_base_length`, its wear `leaf_base_weathering`; count `leaf_bases` up to `MAX_LEAF_BASES` 256, spread evenly over the clothed run. The surface builder draws the run like any wood: a round section with socket and swell, bark material inherited. [checked]
- **Shape.** [inferred]
  - A base's section can be broad across the trunk and thin along the radial: a width and a flatness row (width as a share of the lattice cell the spiral gives that base, so 1 means neighbours touch whatever the count and trunk girth), and an outer end cut square rather than rounded.
  - Neutral values (today's round peg) leave every family byte-identical; rows validated, doc-commented and in the dial table with meaning, range and steps.
  - Generic to any rosette stem, never a palm branch.
- **Unknown.** Whether a non-round section rides the surface builder's run (a section profile on the ring) or needs the base drawn as its own mesh piece that still inherits bark material, bounds, shadow and wood level of detail. The implementer measures both on cost and picks. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** With width 1 and a flat section, neighbouring bases on the spiral touch without gaps or deep interpenetration, at 32, 96 and 256 bases, measured by code on the built mesh. Errors: a gap wider than a stated share of the cell, or overlap past it, fails.
- **R2:** Neutral rows are byte-identical to today for every shipped preset (species digests and catalogue pins).
- **R3:** The date palm at seed 1, with values code proposes, builds a trunk that the headless P-TRUNK and P-BASE stills show as a packed diamond lattice of flat-faced boots; the Opus reviewer's `trunk-leaf-base-diamond-pattern` cell on that still is recorded. The palm's preset values themselves are left to tuning.
- **R4:** The rows are in the dial table; `leaf-base-lattice` joins the capability vocabulary's expressed list; the workspace gate is green.

## Boundaries
<!-- scope: business -->

- Not the fibre matting between bases (improvement, backlog). Not the dead-frond skirt (fn-120). Not the palm's acceptance, which fn-82 owns.

## Strategy Alignment

- Serves "The catalogue": the lattice is the date palm's trunk, and Phoenix, Butia and Sabal trunks share it. [strategy:The catalogue]
