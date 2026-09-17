# Pendulous shoots as rows

## Conversation Evidence

> user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
> gap analysis (`.flow/evidence/fn34/GAPS.md`, round 3): the birch's hanging curtain of long shoots, its crown base at 0.20 against the photograph's 0.08, and its leaves in tufts rather than strings are one gap.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 45% [paraphrase], 25% [inferred] -->

The birch in the round-3 pair is a rounded mop where the photograph is a weeping curtain of long thin shoots reaching almost to the ground with the leaves strung along them. The value tables cannot get there, and the reason is not that the generator lacks hanging: it has a curtain mode that the spruce and the birch already use. The reason is that the mode is hidden and fixed. A negative secondary rise flips it on as a switch, its droop is a constant nobody can dial, its shoot length is borrowed from the twig anatomy, and no leaf may point downward because every canopy orientation row is bounded at zero. [paraphrase]

This spec turns the curtain into rows a value table can set and the blend can walk: how strongly a shoot hangs, how long a pendulous shoot grows unbranched, below what wood radius shoots hang, and how far a leaf may lean down its shoot. The birch is the first specimen judged on the matched pairs; the spruce keeps its curtain byte-identically at the values that reproduce today's constants; willow, weeping beech and the White Tree's sources follow as species. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The curtain becomes continuous.** The pendant predicate, today `rise_secondary < 0.0` in the twig seed, becomes a hang strength row on the twig layer, `hang`, in 0 to 1, neutral 0. At zero nothing hangs and the oak is unchanged; between zero and one the droop cap and slope that are constants today scale with it; at one they are today's values. The secondary rise keeps bending deeper orders as it does and stops doubling as a switch. [inferred]
- **Three more twig rows.** `pendulous_length`, the metres a pendulous shoot grows before it stops (today borrowed from the terminal twig length); `pendulous_radius`, the fraction of the root radius below which a shoot is pendulous (today every shoot under a pendant ancestor); and `curtain_separation`, the degrees between neighbouring shoots in a curtain (today 4). Each is a number with a rail, validated by name, blended linearly, on the wire and in the browser metadata. [inferred]
- **Leaves may hang.** The canopy's `forward_lean`, `lean_rise`, `outward` and `upward` rails widen to signed ranges so a leaf can lean down its shoot and toward the ground; zero stays zero and every shipped preset is byte-identical. [inferred]
- **The constants move, the behaviour stays.** The 0.35 cap and 0.5 slope, the 0.8 floor-clearance factors and the 4 degree separation in the twig advance leave the source as literals and arrive as rows; the spruce's table states the values that reproduce them, and its pins hold. [paraphrase]
- **The birch is the proof.** Its table sets hang, pendulous length and radius, curtain separation and a downward leaf lean against the S-WHOLE and S-BARE references; the matched pairs are rendered again and the crown base and occupied numbers are recorded beside round 3. [user]

## API Contracts
<!-- scope: technical -->

- **Twig rows** `skeleton.twigs.hang` (0 to 1, neutral 0), `skeleton.twigs.pendulous_length` (0.05 to 5 m), `skeleton.twigs.pendulous_radius` (0 to 1 of the root radius), `skeleton.twigs.curtain_separation` (1 to 45 degrees); harness sliders for each since twig rows are not rendered generically. [inferred]
- **Canopy rails** `canopy.forward_lean` and `canopy.upward` in -1 to 1, `canopy.lean_rise` in -2 to 2, `canopy.outward` in -1 to 1; validation names the field. [inferred]
- **Blend**: the four rows walk linearly; a walk from the oak to the birch ramps the curtain up from nothing with no frame where the tree changes kind. [strategy:The catalogue]

## Edge Cases & Constraints
<!-- scope: technical -->

- **No switch frame.** The sweep's walk from every preset to the birch has no step where the pendant predicate flips; the predicate is a function of the walked row, so the tree changes continuously. [paraphrase]
- **The floor still holds.** A pendulous shoot never grows below the curtain floor its ancestor set; the floor clearance is a row-scaled factor, never zero, so shoots do not stack on the ground. [inferred]
- **Byte identity.** Oak, beech, Telperion and Laurelin (hang 0) and the spruce (hang at the value reproducing today's constants) keep their identity pins; the birch's pins move once, with the reason stated. [paraphrase]
- **File sizes.** The twig advance is at 387 lines and the scaffold at 370; the rows land with a split of the pendant path into its own module rather than growth past the rule. [paraphrase]
- **Determinism.** Same seed, same rows, same tree. [paraphrase]
- **Cost.** A curtain is more shoots; the birch's build time and node count are recorded beside round 3's, and a growth that hits the node cap fails the template rather than truncating silently. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The pendant mode is a continuous twig row, `hang`, neutral zero, with the droop cap and slope scaling on it; a family at zero is byte-identical to today, and a walk across any value has no frame where the tree changes kind. Errors: a value outside 0 to 1 is refused naming the field. [paraphrase]
- **R2:** Pendulous shoot length, the radius below which shoots hang, and the curtain's shoot separation are twig rows with rails, on the wire, blended and in the browser metadata; the spruce's table states the values that reproduce today's constants and its pins hold. Errors: no error surface beyond row validation. [inferred]
- **R3:** The canopy's orientation rows accept signed values so a leaf may lean down its shoot and toward the ground; every shipped preset is byte-identical at its current values. Errors: a value outside the widened rail is refused naming the field. [inferred]
- **R4:** The birch's table uses the rows and its matched pairs are rendered again; the crown base and occupied numbers on S-WHOLE and S-BARE are recorded beside round 3's, and the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R5:** The tests cover: the predicate's continuity across zero, the four rails, the floor under any hang value, the spruce's byte identity, a sweep walk from the oak to the birch, and the signed canopy rails on placements. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No new shoot class, no physics; the curtain that exists becomes rows. [paraphrase]
- No multi-stem and no crown irregularity; those are their own specs. [paraphrase]
- No species beyond the birch and the spruce's byte-identity check; willow and weeping beech are species specs afterwards. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner wants the species to drive the generator: get as close as values allow, name the gap, spec it, compare again. The birch's curtain is the first gap, and reading the code showed it is a hidden switch with hard-coded magnitudes rather than a missing form. [user]

### Implementation Tradeoffs

- Rows on the existing curtain over a gravity term in the bias field: the field is positional and height-keyed and cannot know which shoot is pendulous; the twig layer already knows and already hangs, so the rows go where the behaviour is. [inferred]
- Widening canopy rails over a separate "hang" leaf row: the four orientation rows already describe the lean, only their sign is forbidden. [inferred]

## Strategy Alignment

- Follows "The catalogue": a species exposed an unsupported form and the generator gains a row, never a branch.
- Follows the approach's continuous-tree-space rule: no family field is a switch between ways of building.

## Resolved via Codebase

- The pendant predicate: `crates/telperion-core/src/branching/local/seed.rs:171-176` (`rise_secondary < 0.0` sets `pendant`), `Stations::floor` at `:39-57`, `curtain_across` at `:194`.
- Hard-coded magnitudes: `crates/telperion-core/src/branching/local/advance.rs:147-151` (length capped to `twig.length`), `:169-179` (droop `clamp(.. * 0.5, 0, 0.35)`), `:199-208` and `:235-240` (0.8 floor clearance), `:285-289` (4 degree separation), `:366-368` (propagation).
- Secondary rise applies to orders at or above two: `crates/telperion-core/src/branching/scaffold.rs:268-272`, `:315-320`; rail -1 to 1 at `branching/traits.rs:86`.
- Gravitropism is always `+Y` and non-negative: `crates/telperion-core/src/bias.rs:154`, `:55-68`.
- Canopy orientation rows and rails: `crates/telperion-core/src/foliage/station.rs:163-177`, `foliage/placement.rs:114-129` (all non-negative).
- Users today: spruce `rise_secondary -0.8` (`presets.rs:160`), birch `-0.85` (`presets/species.rs:83`); the spruce curtain pin at `tests/species.rs:384-397`; `examples/curtain_audit.rs`.
- Plumbing: `params.rs:11-157`, `blend.rs:34-117`, `tests/sweep.rs:63-93` HELD, `harness/params.ts:224-336` and `harness/family.ts:51-57` for twig sliders.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
