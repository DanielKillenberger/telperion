# Multi-stem trees

## Conversation Evidence

> user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
> gap analysis (`.flow/evidence/fn34/GAPS.md`, round 3): the birch's two stems from the base, one leaning, are a gap no row can close.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 45% [paraphrase], 25% [inferred] -->

The birch in the S-WHOLE photograph stands on two stems that part at the ground, one leaning out; the generated birch stands on one. Birches, hazels, alders, coppiced oaks and many garden trees grow this way, and the generator cannot: one stem is born at the origin and every stage downstream assumes it. [paraphrase]

This spec adds stems as rows: how many stems leave the base, how far apart they lean, and how the base's radius is shared between them, with one stem as the neutral value so every shipped tree is untouched. The birch is the first specimen judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Stems are order-zero axes born at the root.** The scaffold frontier enqueues one order-zero axis at node zero today; with `stems` above one it enqueues that many, headings spread by `stem_divergence` around a per-seed bearing and tilted outward by `stem_lean`, each with the leader's length rule. The bole gates in the scaffold, which refuse a crown axis in the bole and a station below the bole, treat every order-zero axis as a stem, so a second stem is never a lateral. [inferred]
- **The base radius is shared by the pipe model.** The radius solve keeps normalising on node zero, and a basal fork divides the trunk allocation through the existing fork exponent; the profile's DBH is measured per stem and the largest reported as the proxy, with the count recorded, in place of today's ambiguous status. [inferred]
- **Every stem is a trunk run to the surface.** The surface seeds its runs from the leaders and marks only the first as the trunk; with stems, every order-zero run is a trunk run: buried root sample, flare by height, no fork socket at the base. The flare is already a function of height and applies to each. [inferred]
- **Birth years and shedding.** Node zero is year zero; a stem's first node born in the first slice is year one today, so stems record birth year zero with the root and the chronicle never sheds a stem's root node. [inferred]
- **Rows.** `skeleton.habit.stems` (count, 1 to 6, neutral 1, blended as a count), `skeleton.habit.stem_divergence` (degrees between stems, 0 to 120), `skeleton.habit.stem_lean` (degrees from vertical, 0 to 45); harness sliders since habit rows render generically. [inferred]

## API Contracts
<!-- scope: technical -->

- **Family rows** as above, validated by name, on the wire, blended (count and degrees), in the browser metadata, in the sweep's moved or held lists. [inferred]
- **Species metrics** `dbh_m` gains `stems` (count) and reports the largest stem's proxy; `multiple structural stems` stops being an ambiguity when the family declares stems. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Stems at one is byte-identical to today on every shipped preset, asserted by the pins. [paraphrase]
- **Stems stay inside the envelope** and never pass through one another at the base: the frontier rejects a second stem whose first edge leaves the shell or enters another stem's radius, and reports it. [inferred]
- **Foliage eligibility** keys off the root radius today; with stems the root's radius is the combined pipe, so twig eligibility is measured against the largest stem's radius instead, and the spruce, oak, beech and birch at one stem keep their placements byte-identical. [inferred]
- **The blend walks the count** the way leaf lobe counts are walked: a walk from one to two stems has the second stem grow in from the base rather than appearing whole. [inferred]
- **File sizes.** The scaffold is at 370 lines and the surface at 393; the trunk-run change lands with a split of the surface's run seeding into its own module. [paraphrase]
- **Cost.** Two stems are two crowns' worth of laterals inside one envelope; the birch's node count and build time are recorded beside round 3's and growth must complete. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `stems`, `stem_divergence` and `stem_lean` are habit rows with rails, neutral at one stem, on the wire, blended and in the browser metadata; every shipped preset is byte-identical at neutral. Errors: a value outside its rail is refused naming the field. [inferred]
- **R2:** A family with two or more stems grows that many order-zero axes from the root, spread and leaned as stated, inside the envelope, never through each other, each swept as a trunk run with a buried root and a flare and no fork socket at the base. Errors: a stem that cannot be placed inside the shell fails the build naming the stem. [paraphrase]
- **R3:** The radius solve shares the base through the pipe model, the profile's DBH reports the largest stem with the count, and twig eligibility keys off the largest stem so single-stem trees are untouched. Errors: no error surface beyond R1. [inferred]
- **R4:** The birch's table declares two stems against S-WHOLE and S-BARE and its matched pairs are rendered again, with the numbers recorded beside round 3's; the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R5:** The tests cover: neutral byte identity on every preset, the rails, stems inside the shell and apart, the trunk-run surface on each stem, the DBH proxy with a count, a walk from one to two stems with no kind switch, and the birth years of stem roots. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No root system, no coppice regrowth from a cut, no stem death; fn-16 and fn-20 own those. [inferred]
- No pendulous shoots and no crown irregularity; their own specs. [paraphrase]
- One species proof, the birch; Yggdrasil's roots are fn-20's, not stems. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's loop: species expose gaps, gaps become generator specs, the comparison runs again. The birch's second stem is the clearest form the generator lacks. [user]

### Implementation Tradeoffs

- Stems as order-zero siblings over a basal fork born as a lateral: the scaffold's bole gates forbid a crown axis in the bole by design, and a stem is a trunk to the surface and the radius solve; a sibling axis keeps every downstream stage honest. [inferred]
- A count row walked like lobes over a boolean: no family field is a switch; the blend already knows how to walk a count. [strategy:The catalogue]

## Strategy Alignment

- Follows "The catalogue": a species exposed an unsupported form and the generator gains rows, never a branch.
- Follows "Growth and botanical fidelity": the form is judged against the photograph on the matched pairs.

## Resolved via Codebase

- One axis is born: `crates/telperion-core/src/branching/scaffold/frontier.rs:29-33`; the root node at `:159-161`; `Tree` allows several children of node zero (`tree.rs:96-100`).
- Bole gates: `crates/telperion-core/src/branching/scaffold.rs:216-219` (no station below the bole), `:103-110` (no crown axis in the bole).
- Radius solve normalises on node zero and forks through `fork_exponent`: `crates/telperion-core/src/radius.rs:60-77`.
- Surface marks only run zero as the trunk: `crates/telperion-core/src/surface/paths.rs:48-54`, `:75-79`; trunk burial `surface.rs:135-142`; branch sockets `:150-172`; flare by height `:129-132`.
- DBH ambiguity for several stems: `crates/telperion-core/examples/species_metrics/mod.rs:99-120`.
- Foliage seeds from every child of node zero: `crates/telperion-core/src/foliage/placement.rs:200-209`; root-radius scaling at `:160`, `:246`, `branching.rs:172`, `local/seed.rs:139,149`.
- Birth year of node zero hard-coded: `crates/telperion-core/src/branching/specimen.rs:158-162`; death refused for index zero only: `specimen/chronicle.rs:52`.
- Lean exists only as a whole-tree bias row: `crates/telperion-core/src/bias.rs:37`, `:83-88`, `:117-120`; all species presets set the bias to none.
- Legacy colonization (`crates/telperion-core/src/colonization.rs:159-213`) is test-only; species grow through the scaffold.
- File sizes: `scaffold/frontier.rs` 284, `scaffold.rs` 370, `surface.rs` 393, `radius.rs` 90, `traits.rs` 104.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
