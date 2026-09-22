# Leaders keep their girth: a lateral share at forks in the radius law

## Conversation Evidence

> user (2026-09-22, on the beech stills): "If you look at the reference you have two massive strong stems that grow super high that only taper towards the top. And branches at the top taper more? (They're thinner than bottom ones). Currently it seems like the dials can only achieve thin branches if they taper evenly all over. This needs a new parameter i'd say."
> user: "Because the left most packet clearly attempted to have the strong center stem"
> user: "yes draft it"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 30% [paraphrase], 20% [inferred] -->

The beech reference (`.refs/fn34/european-beech/fasy896.jpg`, bare view) shows a trunk that continues as two strong ascending leaders almost to the top, losing girth slowly, while the laterals off them are thin from their first metre and thin further with each order. The fn-68 loop's best tree (bundle 1 and its two adoptions, seed 1) has the strong trunk and the low fork, and then every division splits the wood evenly: three metres above the fork the leaders are no thicker than the laterals leaving them, and the middle crown is a lattice of same-gauge branches. The reviewer wrote it as "exposed thick scaffold limbs" and "crossing branches" for 44 rounds; the owner read it as the gap: the dials can only give thin upper branches by tapering everything. [user, host read of the stills]

The radius law has four knobs, trunk radius at the ground, a fork exponent, a length taper and a taper cap, and none of them says who gets the wood at a fork. This spec adds that one asymmetry so the continuing axis keeps most of the parent's girth and a lateral takes little. The beech spec (fn-62) depends on it; the loop consumes it as one more dial. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Where the split lives, checked 2026-09-22.** `crates/telperion-core/src/radius/incremental.rs`, `Pipes::record`: radii are solved bottom-up, a parent's distal radius is `(Σ child.proximal^e)^(1/e)` over its structural children with `e = fork_exponent`, and a child's proximal radius is its distal radius grown by the length taper accumulated since the parent (`shed`). The trunk's ground radius then scales the whole tree (`scale = trunk_radius * height / distal[0]`). Nothing in the walk distinguishes the child that continues the axis from a lateral. [checked]
- **What the tree records.** `tree.rs` has no leader or lateral mark on a node: `NodeKind` is Structural, Branch or Twig, and stems are documented as order-zero axes. Whether the branching stage knows which child continues the parent's axis at generation time is **unknown**; if it does not, the continuation child is the structural child whose direction departs least from the parent segment's direction, decided once per fork and recorded on the node. [checked; unknown stated]
- **The parameter.** One new row in `/radii`, `lateralShare`, range 0 to 1, default 1.0. At a fork every child that is not the continuation contributes `proximal^e * lateralShare` to the parent's sum, and its own proximal radius at the junction is scaled by `lateralShare^(1/e)`, so a lateral leaves its parent thinner than its own subtree alone would make it and the parent's girth is carried by the axis. At 1.0 the sum and every radius are unchanged, so the default is byte-identical with today. Presets and the dial table pick the value; no species branch anywhere. [host design]
- **Not a second taper.** `length_taper` and `max_taper_exponent` still act along every piece of wood; this row only changes the division at forks. The trunk's ground radius still anchors the scale. [inferred]

## API Contracts
<!-- scope: technical -->

- `RadiusParams.lateral_share: f64`, wire `/radii/lateralShare`, validated in `ranges.rs` like the other three, serde default 1.0 so every existing preset and family wire parses unchanged. [inferred]
- The dial table (`crates/telperion-jev/data/dials.json`) gains the row with its meaning and authored small and substantial steps, so the loop can move it. [inferred]
- The capability declaration (fn-84's, if landed) names the trait it expresses: leaders hold girth, laterals thin at the base. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Default output is byte-identical.** `lateralShare = 1.0` reproduces every shipped preset's mesh and metrics byte for byte; the golden checks in the repository cover it. [paraphrase]
- **A fork with one child has no lateral.** The single child is the continuation whatever its angle. [inferred]
- **Multi-stem trees.** Each stem is its own axis from the root; the share applies within a stem's forks, never between stems. [inferred]
- **Twigs and canopy wood** below the structural crossover keep their own radius rules; this row acts on structural forks only, where the reference shows the effect. [inferred]
- **Growth path** stays hidden and buildable; the row is read there too, unchanged default. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `lateralShare` exists on `/radii`, defaults to 1.0, is range-checked, and every shipped preset builds byte-identically at the default (mesh and `species_measure` metrics). Errors: a value outside 0 to 1 is refused by name.
- **R2:** A red/green test on a synthetic three-node fork: at share 1.0 both children leave the parent with the radii the current law gives; at share 0.3 the continuation child's proximal radius is unchanged relative to the parent, the lateral's is smaller by the stated factor, and the parent's distal radius follows the stated sum. Errors: a lateral never ends up thicker than the continuation at the same fork for any share below 1.
- **R3:** The continuation child at every structural fork is decided by one recorded rule (generation-time mark if it exists, otherwise least direction change) and is deterministic for a seed. Errors: a fork with no structural children is untouched.
- **R4:** On the beech at seed 1 from fn-68's round-42 overlay, one matched still pair (bare view) at share 1.0 and at one authored lower value shows the leaders holding girth into the upper crown while laterals thin, and the numeric species gates still pass. Reported as a comparison, judged by the owner; not a readiness claim.
- **R5:** The dial table row exists with meaning, range and steps, and its coverage test passes; the workspace gate is green.

## Boundaries
<!-- scope: business -->

- No change to `fork_exponent`, `length_taper` or their defaults. No species branch. No renderer change.
- The beech's shipped value is fn-62's decision; this spec ships the row and its evidence, not a tuned preset.
- Foliage organisation, hanging droop and bark are separate gaps.

### The overarching issue, checked 2026-09-22

The owner asked, before minting the other gaps, whether one thing fixes them more elegantly. It does, and this spec is that thing. Every decision that gives a tree its hierarchy below the crossover is a radius threshold, checked in code: laterals start where wood is thinner than `limbRadius` times the root radius (`branching/local/seed.rs:150`, `advance.rs:86`); twigs bear where wood is thinner than half `bearingDiameter` (`advance.rs:77`); a station's shoots hang where wood is thinner than `pendulousRadius` times the root radius (`branching/local/pendant.rs:77`); wood bears foliage of its own where it is thinner than `shootRadius` times the stem radius (`foliage/placement.rs:237`). Radius is the generator's only hierarchy signal. With the even fork split, every branch crosses those thresholds at the same distance from the trunk, so laterals, twigs, foliage and droop all switch on as one shell around the crown: the reviewer's "continuous veil across broad shoulders", "outer sprays remain ascending", and "same-gauge lattice" are one symptom seen three times. With leaders holding girth and laterals thin from their base, the same thresholds fire early on lateral systems and late on the axes, so foliage and droop gather on the laterals and the axes stay open between them. fn-104 and fn-105 therefore start by measuring this spec's tree before they add any parameter, and fn-106 (bark) is independent of it. [host design, code checked]

## Strategy Alignment

- Serves "Growth and botanical fidelity": the reference's strong leaders and thin laterals become expressible by one parameter. [strategy:Growth and botanical fidelity]
