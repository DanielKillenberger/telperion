# The old oak: which cause carries the bare pole

## Conversation Evidence

> goal, owner, 9 Nov 2026: "0.2.0 looks bad. But .2 is better conceptually with the botanically accurate way of growing it just needs to be even better and grow larger trees more efficiently"
> owner, 10 Nov 2026, on the oak at 300: "i mean wtf is that ok at 300..?"
> owner, 10 Nov 2026: "Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction."
> fn-342 RESULT.md: "every trunk-line limb and bough on both seeds died the year the carbon balance shed it"

## Goal & Context

Child of goal fn-321. The owner asked for larger trees, and the largest tree the generator grows, the oak at 300, is a bare, snaking pole under a tuft. fn-342 traced it to three things, each a prescribed value: limbs are shed under the upkeep of wood that is already dead; no bud on the trunk can make a limb, so replacements are born only at the rising top; and above 7 m the trunk is a chain of limb axes pulled toward 60 degrees, with nothing ending its rise. [paraphrase]

fn-343 showed the method that works in this run: switch one cause off alone, draw it beside today's and let the eye say which cause carries the defect, before anything is designed. This child does that for the old oak. It designs and proposes nothing. [inferred]

In hand: fn-342's `RESULT.md` and `probes/turns.patch` (the yearly record keyed by lineage); fn-343's `probes/ablate.patch` (preset values overridden from the environment) and its sheet script; fn-341's `R3-RESULT.md` (charging living sapwood only doubled the held structure at 120 and outgrew 24 GB at 300). [paraphrase]

## Acceptance Criteria

- **R1 (the variants):** on today's oak, seed 1001 at 120, 200 and 300 and seed 1007 at 300, each of these alone and every other setting as today, through one probe patch: (A) today; (B) wood that has died is no longer charged to the limb that bore it, while all living wood is charged as today; (C) the trunk line above the leader is pulled toward vertical as the leader is, in place of the limb's angle; (D) the leader row's life long enough that the trunk stays a leader for the whole run; (E) sleeping buds on the trunk line wake as limbs where today they wake as branches or not at all; (F) B, C and E together. Where a variant cannot be made from an existing setting, the note says what the probe had to add. [inferred]
- **R2 (kept in bounds):** every grow prints a yearly census of held phytomers and is stopped when it passes four times today's count at that age; a variant that outgrows the desk is reported as that, with the year it left today's curve. No run is left to be killed by the memory cap. [inferred]
- **R3 (the sheets and the measures):** leafed and leafless stills at the whole view, and a mid view of the lower trunk, A beside each variant, labelled. Beside each: height, spread, trunk, lowest living limb, living limbs on the trunk line, current-year leaf, peak memory and seconds against A's. [inferred]
- **R4 (the read, back to the host):** the worker views every still and says plainly, per variant, whether it reads as nearer an old open-grown oak (a short thick bole, heavy low limbs, a crown about as wide as tall) than A, and what it lost or broke. It proposes no fix. [inferred]
- **R5 (verification):** no product behaviour changes; the probe leaves the crates as a patch in evidence. [paraphrase]

## Boundaries

- These variants are not candidates for the product; they are instruments. No value is tuned to a sheet. [paraphrase]
- No sheet goes to the owner from this child. [inferred]
- Heavy work runs through `telperion-heavy`. Nothing merges to master, nothing is tagged or published.

## Decision Context

Settled by the host after fn-343: five design directions drawn from counts were refuted by measurement, and the one step that told the host something about what the eye sees was an ablation sheet. So the old oak starts with one. [paraphrase]

## Open Questions

None.

## Result

Closed 10 Nov 2026. `RESULT.md` in this spec's evidence holds every table; the host viewed `sheet-B.jpg`, `sheet-E.jpg` and `sheet-H.jpg`.

- **The collapse is one event with one cause.** Today's oak is charged upkeep for wood already dead: 78% of its whole upkeep at 49, 88% at 89, 91% at 109, 97% at 129. The charge reaches its light income at 109; the fork limbs are shed at 132 and living tips fall from 189k to 62k. After that it is a pole.
- **B (dead wood not charged)** keeps every low limb (lowest at 4.7 m to 187) and is a runaway: its crown's ground area goes from 81 to 772 square metres between 49 and 149 and its tips follow. Light holds density steady in both A and B (about 125 income and 650 tips per square metre) and bounds nothing else; fewer than 0.01% of tips have a negative balance.
- **C (trunk line pulled to vertical)** removes the large bends and leaves the bare pole; **D (a long-lived leader)** makes a 98 m pole; **E (trunk buds wake as limbs)** makes a low dense bush with 54 limbs and a 4.3 m trunk.
- **H (B, and each new internode shortened by 1 - L / 34.8 m)** levels the oak at 25 to 27 m tall and 22 to 25 m wide with limbs alive from 4.4 m. The host's read: at 200 and 260 its silhouette is an old open-grown oak's where today's is a pole; at 120 it is worse than today's (the open V is lost, 18.8 m against 25.1); its wood is broken (nodes packed into knots, tips per square metre up to 3,040, a girth step on the bole, a hollow tube at 4 m) and the device refuses it at 300. An instrument, not a candidate.
- **The host's stop rule was wrong** (R2 compared with today's count that year, which halves at 132); corrected to the most today's tree had held.
- **Unexplained:** B's resident memory differs between two runs of the same seed (20 GiB at 188 in one, 14.2 GiB at 219 in another), and tip counts and memory depend on the requested age. fn-345 measures memory and must find this.
- No code stays; the probe is `probes/oldoak.patch`. No gate and no review ran.

## Friction

- **The local shell hook refused two helper commands** (3 minutes). Local, reported and not specced.
- **A tree's drawing can need more memory than its grow** (one rebuild). A still runner that refuses, before dressing, a tree its estimate says will not fit is a line for the next child that changes the stills example.

