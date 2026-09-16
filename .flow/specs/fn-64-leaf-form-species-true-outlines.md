# Leaf form: species-true outlines

## Conversation Evidence

> user (2026-09-10, fn-24, on the Oregon white oak single-leaf stills; vault note `feedback_telperion_oak_leaf_shape_preference`): the pointed five-lobe blade felt closer to an oak than the broad-lobe pass that shipped; further lobe and margin work is deferred to "a later focused foliage spec".
> user (2026-09-16, fn-31 round 13, on the first-pass pedunculate oak leaf): "close enough but we need this drastically improved in a later spec if we don't have that already"
> user (2026-09-16): "I want to represent trees that grow in the wild as closely as possible"
> user (2026-09-16): "why wouldn't we just follow the data make sure it's accurate and follow that. I'm no botanist"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 45% [paraphrase], 25% [inferred] -->

Every leaf Telperion draws comes from one outline built from a handful of
numbers: length, width, where the blade is widest, how full its base is, how
sharp its tip is, how many lobes it has and how deep they cut
(`foliage/element.rs`, `foliage/outline.rs`, 71 lines). That is enough for a
generic leaf and not enough for a real one. [inferred]

Round 13 of fn-31 moved the oak to a pedunculate oak (Quercus robur) and set
those rows from the sources. The render shows rounded lobes. It cannot show the
small ear-shaped lobes (auricles) at the base that identify the species,
because the outline has no term for them, and its sinuses are narrow notches
rather than the open rounded bays of a real leaf. The owner accepted it for
fn-31 and asked for the leaf to be "drastically improved" in its own spec.
[user] [paraphrase]

The same limit applies to the leaves fn-34 adds. A beech leaf has a wavy,
fringed margin; a birch leaf is doubly toothed. The current outline has no
margin teeth at all. [inferred]

The owner's rule for the whole project applies here: follow the sources, verify
them, and do not ask the owner to make botanical calls. The owner judges the
result by eye beside a reference. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The outline stays one rule.** New shape features are numeric element rows
  with validated ranges, carried by every family and walked by the blend. No
  species or preset branch enters the outline, the element builder or the
  renderer. A zero or neutral value leaves every current leaf byte-identical.
  [strategy:Growth and botanical fidelity]
- **Features the outline lacks.** The mechanism is the implementer's to choose;
  the references judge it. [inferred]
  - basal auricles, their size and turn;
  - sinus width and shape, from a notch to an open rounded bay;
  - lobe shape, from rounded to pointed;
  - how lobe size changes from base to tip;
  - margin teeth: size, spacing and a second order, for doubly toothed
    margins;
  - an uneven base.
- **Enough outline resolution to show them.** Stations per lobe and per tooth
  follow the features, and the vertex cost per leaf is measured and reported,
  not capped. CLAUDE.md applies: improve performance before reducing form.
  [inferred]
- **References with provenance.** Each species has at least one reference
  leaf: a herbarium scan, a botanical plate or a measured outline, with its
  URL, licence and sha256, beside a description from a botanical source quoted
  verbatim. Numeric rows come from measured descriptors where sources give
  them: lobe pairs, lobe depth as a share of the half-width, and length and
  width. TypeSafe's Jev may screen sources (CLAUDE.md, "TypeSafe"); code copies
  every number. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Current leaves.** The pedunculate oak (fn-31) and the Norway spruce needle
  are the first to move. The beech and birch follow when fn-34 lands. The
  synthetic presets (Ordinary, Telperion, Laurelin) keep their leaves unless
  the owner asks otherwise. [inferred]
- **Byte-identical by default.** A preset that authors no new row renders
  exactly as before, and its element hash holds. [inferred]
- **Scale.** At viewing distance the outline feeds the canopy's instanced draw,
  so the per-leaf vertex count multiplies by millions of instances. The report
  states the vertex count per leaf and the mature draw cost before and after,
  and any level-of-detail change it needs. [inferred]
- **Leaf size by age is fn-43's; colour and translucency are fn-29's and
  fn-46's.** This spec changes shape only. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** For each moved preset, the owner judges the rendered single leaf
  beside its reference and records an accepting verdict in the owner's own
  words. [user] Errors: a rejecting verdict stops the spec with the owner's
  words and the rows it names.
- **R2:** The pedunculate oak leaf shows basal auricles, open rounded sinuses
  and 3 to 6 lobe pairs cut no more than halfway to the midrib, on a 2 to 3 mm
  petiole, matching its cited sources. A test asserts these as measured outline
  properties. [inferred] Errors: a property outside its sourced range fails
  the test naming the property and the source.
- **R3:** Every new row is a validated numeric trait the blend walks, with no
  species branch. Every preset that authors none keeps its element hash.
  [strategy:Growth and botanical fidelity] Errors: a moved hash on an untouched
  preset fails the pin test naming the preset.
- **R4:** The report states the vertex count per leaf and the mature canopy's
  draw cost for each moved preset against the current numbers. No form is
  reduced to meet a cost. [inferred] Errors: none beyond reporting.
- **R5:** Every reference image and quoted description carries its URL,
  licence and sha256 in the spec's evidence folder. [inferred] Errors: a
  reference without provenance is not used.

## Boundaries
<!-- scope: business -->

- **Excluded:** leaf size by age (fn-43), leaf colour and young-shoot colour
  (fn-29, fn-46), canopy lighting (fn-52), and veins and translucency beyond
  what the new outline needs (fn-26 shipped them). [paraphrase]
- **Excluded:** leaf arrangement, phyllotaxis and how many leaves a shoot
  carries. [inferred]
- **Excluded:** new species; this spec improves the leaves of existing presets
  and the ones fn-34 brings. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- **Owner's call.** The owner asked for the leaf to be drastically improved and
  judged the fn-31 leaf close enough only for that spec. [user]
- **The missing term.** The auricle that identifies a pedunculate oak cannot be
  drawn today, and no row reaches it. [paraphrase]

### Implementation Tradeoffs

- **Rows over templates.** New outline rows keep "presets are value tables"
  and let fn-34's species use the same terms. A per-species outline table would
  be faster to author but breaks the no-branch rule. [inferred]

## Strategy Alignment

- **Growth and botanical fidelity.** This follows that track: reference-based
  visual QA against real specimens. [strategy:Growth and botanical fidelity]

## Parked unknowns

- **Leaf references.** Which licensed herbarium or plate source gives a clean
  pedunculate oak, beech and birch leaf image at a known scale.
- **Distance.** Whether the improved outline needs a simpler form at distance
  to hold the canopy's cost.
