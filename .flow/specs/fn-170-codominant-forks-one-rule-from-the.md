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

- **Today, checked 2026-09-27 (rechecked on master `1f80e778`, after fn-61).** The stems rows (`stems`, `stemDivergence`, `stemLean`, `stemLeanSpread`, `stemForkHeight`) drive a dedicated stem-placement path in the scaffold: a clump's later stems are held on the first until it reaches the fork height, then enqueued as order-zero axes. The shipped silver birch is the only preset that sets them (two stems, lean 28°, fork height at the row's top of half the bole). The radius solve divides wood at every fork by tip count through the fork exponent and does not distinguish a continuation from a lateral. At a fork of stems above the root the surface followed the straightest stem, overriding its widest-child rule (Astra, 2026-09-27). [checked]
- **One fork rule in the scaffold (host, 2026-09-27, Astra review).** Every structural axis makes one fork decision when it is born, from its own keyed stream. The profile is over the tree's height share (the envelope height), not restarted per axis: the axis draws a target height from a smooth bell, centre `forkHeight`, width `forkHeightSpread` (a width of zero is exactly the centre), and forks where it climbs through that height. A target at or below its birth height, or less than one station above it, draws no fork, so every fork follows a station of growth and none re-forks at once. The root's axis is the one base exception: a target of zero forks at the root, which is a clump. There is no "fork wherever the axis stopped" fallback. At the fork the axis carries on as the primary, keeping its station spacing and curvature progress. The siblings leave beside it with its order, its remaining length and its limb bound. Directions are relative to the incoming axis. `pitchByHeight` applies only at lateral insertion, and `raggedReach` is drawn only where a first-order limb system begins. Each part, the primary included, draws its own target above its birth, which is how forks repeat up the crown. The separate stem-placement path is removed. [host decision]
- **Forks grow in, never pop (host, 2026-09-27).** Each axis draws its own point on the rate. Past that point the fork's siblings take a weight that ramps smoothly from 0 to 1 over a fixed width of the rate (0.05), and a rate of one forks every tree whole. `forkWays` is real-valued, 2 to 4. Its whole part is the full siblings, and its fraction is the weight of one more. The parts fill four fixed slots in order, so existing children never move when a child is added. Slot `k` stands `k` times `forkDivergence` of bearing round from the primary's, and every sibling leans `forkLean` from the axis. `forkLeanSpread` stands the primary back toward the axis. The primary's own lean also grows in with the weight. A sibling's length is its weight of the remaining length. [host decision]
- **Continuation is recorded, not guessed.** The scaffold already marks the first node of every lateral `BudFate::Lateral` (checked 2026-09-27). A codominant sibling's first node carries its weight as the node's `codominant` field, and the primary stays unmarked. The mark and the weight are part of the node's structural role: they survive every read, serialization and compaction, geometry-only growth reads included. The radius solve, the surface, the contacts query, foliage clumping and the species metrics read the recorded fact, never the fan order. Codominant siblings shed like continuations. [host decision]
- **The radius law divides by role (host, 2026-09-27).** One allocation function, shared by the full and incremental solves, gives each child the share of its own pipe it takes at its fork: a lateral `lateralShare`, a codominant sibling `forkBalance` times its weight, and a primary all of it. The child's whole subtree thins by the root of that share. The parent carries the sum of what its children leave with, so wood is conserved at every fork and no child leaves thicker than the wood it leaves. With `lateralShare` 1, `forkBalance` 1 and no codominant weight, the law is today's pipe model exactly. At a `forkBalance` of 1 a sibling takes its own pipe as the primary does, and lower leaves it thinner. [host decision]
- **Rows (host, 2026-09-27).** `codominance` (0 to 1, dormant at 0), `forkHeight` and `forkHeightSpread` (shares of the tree's height), `forkWays` (real, 2 to 4), `forkDivergence` (0 to 120 degrees), `forkLean` (0 to 45 degrees) and `forkLeanSpread` (0 to 1) on `/skeleton/habit`. `lateralShare` and `forkBalance` (0.01 to 1, neutral 1) sit on `/radii`. Every row is a numeric trait of every tree, blended continuously and on the harness dials. [host decision]
- **The beech's fork gap points here.** The beech's recorded capability assessment (`crates/telperion-jev/tests/fixtures/replay/european-beech`, packet/capability.json) links its codominant V fork to fn-61, which cannot make a fork; this spec moves that `covers` entry to an fn-170 need and updates the replay test's expected gap classes. [checked]
- **The growth path** stays hidden and buildable and reads the same rows. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Family rows** as above: validated by name, on the wire, blended continuously (shares and degrees; `forkWays` is real-valued), in the regenerated browser metadata, and in the dial table with authored steps so the tuning loop can move them. [host decision]
- **Retired rows.** The five stems rows leave the family. A wire payload, an overlay or a value file that still names one is refused with an error naming the row and its replacement; nothing reads it silently. Executable replay inputs are migrated mechanically, and historical measurements keep their provenance. [host decision]
- **Species metrics.** `dbh_m` reports the largest continuation system crossing 1.3 m, up or down, and the count of them there. A system is a root stem or a codominant sibling, read from recorded roles; laterals are excluded. Branch-order metrics and foliage clumping read the codominant mark. Among the other children the widest still carries the axis on, so a table that does not fork keeps its systems. [host decision]
- **Views and commands unchanged.** [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Rate zero is a tree with no codominant forks** on every seed, whatever the other new rows say. Shipped presets other than the birch are unchanged in structure. Byte identity is not required (generator evolution policy), but a preset that did not set the stems rows has no reason to change and a change is investigated. [inferred]
- **Interpolation is smooth.** Walking the rate from zero upward grows codominant forks in one at a time across seeds, and for a fixed seed the fork grows in over a short width of the rate. Walking `forkWays` grows one child in from the fork rather than adding it whole, and the children already there never move. [host decision]
- **Fork headings never coincide.** Placement refuses a fork whose parts would leave on one heading. Whether wood grown from separate headings meets is measured in R7, not guaranteed. [host decision]
- **Node budget and termination.** A codominant fork adds a crown's worth of wood. No planner estimate exists (checked 2026-09-27): the node ceiling stays a hard stop and does not move. Every fork follows at least one station of growth, so pending work stays bounded. Forks draw from their own keyed streams with stable child keys, so repeat runs and sliced or resumed growth are byte-identical. On the 8-tree plane candidate R7 reports structural and twig counts, build time and peak memory, against the unchanged birch and a no-fork baseline, and whether the forked tree starves its twig layer against the ceiling. [host decision]
- **Foliage eligibility** keeps measuring against the largest stem when the fork is at the root, as it does today. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The codominance rows exist with their rails, are validated by name, on the wire, blended, in the browser metadata and the dial table; at rate zero no seed of any preset has a codominant fork. Errors: a value outside its rail is refused naming the field; a profile or ways value with rate zero is inert, never an error. [inferred]
- **R2:** One rule builds every multi-trunk tree: the stems rows and their placement path are gone, and a synthetic family whose height profile sits at the ground produces its trunks from the root through the fork rule. Errors: a value file naming a retired stems row is refused with a message naming the row and its replacement. [paraphrase]
- **R3:** The seed decides each fork: on a synthetic family at a middle rate, a fixed 24-seed set contains specimens with a low codominant fork and specimens without one. The share with a fork rises as the rate rises and is zero at zero, and the same seed and rows give the same tree. Across one seed's point on the rate, the fork grows in from nothing. Walking `forkWays` just below and above 2 and 3 grows one part in, and the parts already there do not move. Errors: no error surface beyond R1. [paraphrase] [strategy:Growth and botanical fidelity]
- **R4:** Codominant forks repeat: a codominant child can fork codominantly again, and on a synthetic family whose profile covers the crown, forks of order one and higher occur on the fixed seeds. Errors: no error surface beyond R1. [paraphrase]
- **R5:** Wood divides by role (fn-103's R1 and R2, carried here). With `lateralShare` at 1, the balance neutral and no codominant weight, radii are today's pipe model. Below 1, on a synthetic fork, a lateral leaves thinner by the stated factor, and thinner than a continuation carrying the same number of tips. Codominant siblings leave the fork within the balance row's ratio, times their weight. Wood is conserved at every fork, and no child leaves thicker than its parent there. Errors: `lateralShare` or `forkBalance` outside 0.01 to 1 is refused by name. [host decision]
- **R6:** A codominant fork at any height is drawn as a fork of the surface, never as two separately rooted trunks unless it is at the root. The run through the fork carries on into the primary, and the girth eases and contacts follow the recorded roles. This is tested on crown forks, three- and four-way forks and fractional children. `dbh_m` reports the largest continuation system crossing 1.3 m and their count, tested with forks below, at and above 1.3 m and a descending crossing. Errors: no error surface beyond R1. [host decision]
- **R7:** The birch is re-expressed through the new rows, and its seed-1 still sits beside the pre-change still for the owner. An 8-tree London-plane candidate table with a low `lengthTaper`, not a shipped preset, is rendered, with its structural and twig counts, build time and peak memory against the unchanged birch and a no-fork baseline. Its stills go to the owner with one question: do the limbs hold their girth to the crown edge, or is a taper-profile spec needed. Errors: an owner rejection of the birch stops the spec with the owner's words. [host decision]
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
