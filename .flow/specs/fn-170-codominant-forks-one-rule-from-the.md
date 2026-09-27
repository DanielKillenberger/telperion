# Codominant forks: one rule from the ground to the crown

> HTML render lens: `.flow/artifacts/fn-170-codominant-forks-one-rule-from-the/spec.html` (gitignored — open locally; regenerable, markdown is the record). <!-- flow-next:artifact-link -->

## Conversation Evidence

> user (2026-09-27, turn 1, part 1): "[Image #1] do you know what species of tree this is? I took a photo in Basel Schützenmattpark."
> user (turn 1, part 2): "I feel our generator fundamentally lacks the ability to have strong limbs that quickly taper at the edge of the crown to leaf level. Do we have the ability to create trees like this? if not do we have a spec to fix that?"
> user (turn 1, part 3): "I also think currently the multple stem slider is probably not good design. We should have branching rules that make it more likely to grow equivelantly large stems to the first one."
> user (turn 2): "but isn't 3 just another instance of a codominant fork happening lower in the tree? this seems like it should be one rule with different params. Then you can also randomly have different looking trees from the same species where some have codominant forks a the bottom some don't. that seems much more like trees work."
> user (turn 3): "what's the separate taper profile spec for and don't we have a spec for that already?"
> user (turn 4): "that makes sense"
> owner (2026-09-27, beech session): the beech needs fn-170 as well as fn-61 to achieve its look; "take over with your plan"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 70% [paraphrase], 30% [inferred] -->

The owner's photograph from Basel's Schützenmattpark is a London plane (*Platanus × hispanica*): one short trunk that forks at about three metres into three or four limbs of near equal girth, each of which forks the same way again higher up, and every one of them stays thick almost to the crown's edge. The generator cannot draw it. A tree gets more than one trunk only through the stems rows, which place a count of order-zero axes at the root or, with the fork-height row, up to half the bole. Nothing up in the crown can produce a limb that grows as large as the one it left. [paraphrase]

The owner's reading is that the stems rows are one case of a general rule. A clump that parts at the ground is a codominant fork at height zero; the birch's own table already states its pair as a fork a metre up the trunk, and the plane's forks are the same event higher up and repeated. So there should be one branching rule whose rows say how often a structural fork is codominant, where on the tree it happens and how the wood divides, and the seed decides per specimen whether a given fork happens. Two planes from the same table then differ the way two planes in a park do: one splits low, one does not. [paraphrase]

The spec also absorbs fn-103 ("Leaders keep their girth"). That spec gives the continuing axis at a fork most of the parent's wood and thins the laterals, and it assumes exactly one continuing child per fork. A codominant fork has two or more, so the two rules share one definition of continuation and are built together. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 20% [paraphrase], 80% [inferred] -->

- **Today, checked 2026-09-27 (rechecked on master `1f80e778`, after fn-61).** The stems rows (`stems`, `stemDivergence`, `stemLean`, `stemLeanSpread`, `stemForkHeight`) drive a dedicated stem-placement path in the scaffold: a clump's later stems are held on the first until it reaches the fork height, then enqueued as order-zero axes. The shipped silver birch is the only preset that sets them (two stems, lean 28°, fork height at the row's top of half the bole). The radius solve divides wood at every fork by tip count through the fork exponent and does not distinguish a continuation from a lateral. The surface already follows the widest child through a fork of stems above the root. [checked]
- **One fork rule in the scaffold.** When the scaffold places a lateral on a structural axis, a draw from the seed decides whether that lateral is codominant: it takes the parent's vigour, length rule and order instead of a lateral's, and is enqueued as another continuation of the parent's axis. The chance comes from a rate row times a height profile over the tree, so a species states where its forks fall. A codominant child is an axis like any other and can fork codominantly again, which gives the plane's repeated forks. A fork at the root is the same event at height zero; the separate stem-placement path is removed. [inferred]
- **Continuation is recorded, not guessed.** The scaffold knows at generation time which children continue the parent's axis (the leader and any codominant siblings) and records it on the node, so the radius solve and the surface read the same fact. fn-103 left this as unknown; making the scaffold the source settles it. [inferred]
- **The radius law divides by role.** fn-103's `lateralShare` row thins laterals where they leave the parent; continuations divide the parent's wood between them by a balance row, so a plane's three limbs leave the fork close in girth and a birch's leaning stem can leave thinner than the upright one. At neutral values the law is today's pipe model. [paraphrase]
- **Rows (names to be settled in planning).** A codominance rate (0 to 1, dormant at 0), a height profile (centre and spread as shares of height, dormant at rate 0), the number of ways a codominant fork splits (a count, 2 to 4, stepping one child at a time), divergence and lean between the children (taking over what the stem divergence and lean rows do), the continuation balance, and `lateralShare`. Every row is a numeric trait of every tree, blended and on the harness dials. [inferred]
- **The beech's fork gap points here.** The beech's recorded capability assessment (`crates/telperion-jev/tests/fixtures/replay/european-beech`, packet/capability.json) links its codominant V fork to fn-61, which cannot make a fork; this spec moves that `covers` entry to an fn-170 need and updates the replay test's expected gap classes. [checked]
- **The growth path** stays hidden and buildable and reads the same rows. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Family rows** as above: validated by name, on the wire, blended (counts by unit, shares and degrees continuously), in the regenerated browser metadata, in the dial table with authored steps so the tuning loop can move them. [inferred]
- **Retired rows.** The five stems rows leave the family. A value file or wire payload that still names one is refused with an error naming the row and its replacement; nothing reads it silently. [inferred]
- **Species metrics.** `dbh_m` reports the largest axis crossing breast height and the count of axes there, whatever height the fork was at. [inferred]
- **Views and commands unchanged.** [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Rate zero is a tree with no codominant forks** on every seed, whatever the other new rows say. Shipped presets other than the birch are unchanged in structure. Byte identity is not required (generator evolution policy), but a preset that did not set the stems rows has no reason to change and a change is investigated. [inferred]
- **Interpolation is smooth.** Walking the rate from zero upward adds codominant forks one at a time across seeds; walking the ways count adds one child at a time, the new child growing in from the fork rather than appearing whole. [paraphrase]
- **Forks stay inside the envelope** and do not pass through one another at the fork, as the stems path guarantees today. [inferred]
- **Node budget.** A codominant fork adds a crown's worth of wood. The planner's node estimate counts the expected forks, and the node ceiling does not move. [inferred]
- **Foliage eligibility** keeps measuring against the largest stem when the fork is at the root, as it does today. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The codominance rows exist with their rails, are validated by name, on the wire, blended, in the browser metadata and the dial table; at rate zero no seed of any preset has a codominant fork. Errors: a value outside its rail is refused naming the field; a profile or ways value with rate zero is inert, never an error. [inferred]
- **R2:** One rule builds every multi-trunk tree: the stems rows and their placement path are gone, and a synthetic family whose height profile sits at the ground produces its trunks from the root through the fork rule. Errors: a value file naming a retired stems row is refused with a message naming the row and its replacement. [paraphrase]
- **R3:** The seed decides each fork: on a synthetic family at a middle rate, a fixed 24-seed set contains specimens with a low codominant fork and specimens without one, the share with a fork rises as the rate rises and is zero at zero, and the same seed and rows give the same tree. Errors: no error surface beyond R1. [paraphrase] [strategy:Growth and botanical fidelity]
- **R4:** Codominant forks repeat: a codominant child can fork codominantly again, and on a synthetic family whose profile covers the crown, forks of order one and higher occur on the fixed seeds. Errors: no error surface beyond R1. [paraphrase]
- **R5:** Wood divides by role (fn-103's R1 and R2, carried here): with `lateralShare` at 1 and the balance neutral, radii are today's pipe model; below 1, on a synthetic fork, a lateral leaves thinner by the stated factor and is never thicker than a continuation at the same fork, and codominant siblings leave the fork within the balance row's ratio. Errors: `lateralShare` outside 0 to 1 is refused by name. [paraphrase]
- **R6:** A codominant fork at any height is drawn as a fork of the surface, never as two separately rooted trunks unless it is at the root, and `dbh_m` reports the largest axis at breast height with the count of axes there. Errors: no error surface beyond R1. [inferred]
- **R7:** The birch is re-expressed through the new rows and its seed-1 still sits beside the pre-change still for the owner; an 8-tree London-plane candidate table with a low `lengthTaper`, not a shipped preset, is rendered, and its stills go to the owner with one question: do the limbs hold their girth to the crown edge, or is a taper-profile spec needed. Errors: an owner rejection of the birch stops the spec with the owner's words. [paraphrase]
- **R8:** The beech's capability assessment names fn-170 for its codominant V fork, and the recorded beech's Gaps classes the fork as identity against fn-170 (the limbs and crown stay against fn-61). [inferred]

## Boundaries
<!-- scope: business -->

- No taper-profile row along a limb; whether one is needed is decided from R7's plane render. [paraphrase]
- No London plane preset ships here; the plane is onboarded by its own species spec, which depends on this one. [paraphrase]
- The node ceiling does not move. [inferred]
- The growth path stays hidden behind `?growth=1` and is not made a default. [paraphrase]

## Decision Context
<!-- scope: both -->

One rule over two mechanisms: the stems rows are a switch between ways of building a tree (placed trunks at the base, branching everywhere else), and they cannot produce the plane's repeated forks up the crown. A fork rule with a height profile covers the birch, the clump and the plane with values alone, and lets the seed vary the specimen within a species, which the owner named as how trees work. [paraphrase]

fn-103 is folded in rather than built first because its single-continuation assumption would be reworked the moment codominant forks exist. [paraphrase]

A taper-profile row was considered and deferred: the pipe model may already hold a limb's girth to the crown edge once laterals are thin and the length taper is low, and speccing a row for a gap nobody has rendered breaks the rule that specs rest on checked claims. [paraphrase]

## Strategy Alignment

- **Growth and botanical fidelity:** codominant forks are a botanical architecture the generator lacks, and per-seed variation within a species is the specimen-level variety the approach asks of seeds. [strategy:Growth and botanical fidelity]
- **Approach:** replacing the stems path with one rule removes a switch between ways of building; every new row is dormant at zero and changes the tree by degree. [strategy:Growth and botanical fidelity]

## Strategy Conflicts

None found.

## Parked unknowns

- Whether the scaffold's current lateral placement can host a codominant child without a second frontier pass; resolved by the planning scout reading the scaffold.
- Whether the birch's two-stem look survives exactly as the owner judged it once the pair comes from the fork rule; resolved by R7's still and the owner.

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | fn-N.M (TBD - populate via /flow-next:plan) |
| R2 | fn-N.M (TBD - populate via /flow-next:plan) |
| R3 | fn-N.M (TBD - populate via /flow-next:plan) |
| R4 | fn-N.M (TBD - populate via /flow-next:plan) |
| R5 | fn-N.M (TBD - populate via /flow-next:plan) |
| R6 | fn-N.M (TBD - populate via /flow-next:plan) |
| R7 | fn-N.M (TBD - populate via /flow-next:plan) |
| R8 | fn-N.M (TBD - populate via /flow-next:plan) |
