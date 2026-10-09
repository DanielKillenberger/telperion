# Where the trunks' dents come from (fn-244 R1)

Base `1f9afd53` (F4 with fn-243). Stills from `space_<species>` (`--shots base,limb,bare`, ages 40 and 80, seeds 1, 7, 42) and `space_two_trees` (40 and 80, seed 1), all viewed as contact sheets in `raw/sheets/`.

## What the stills show

- **European beech: collars.** Every 80-year beech and most 40-year ones have rings along the bole where the radius steps in by 15 to 25 % and back out above. Worst cases: `beech-80-7-base`, `beech-80-42-base`.
- **Ordinary: waists.** In `ordinary-40-42-base` and `ordinary-80-42-base` the stem pinches to a quarter of its radius at one node. `ordinary-80-1-base` has a terrace near the ground.
- **Oak: hairline rings.** In `oak-40-1-base` and `oak-40-7-base` there are faint seams above each lateral. Each is a 1 to 4 % step.
- **Birch, palm and spruce: no collars on the bole.** The ledge in `spruce-80-7-base` is the flare's top edge among the whorl branches. The trunk radii there fall by no more than 1 % per node.

## The stage and the rows

The dents are already in the converted tree's node radii, before any surface stage. A probe printed each trunk node's height and radius from the root up. For beech 80-42 the radius runs 0.628 m, then 0.554 m from 0.14 to 0.42 m, then 0.628 m again, then 0.483 m at 2.9 to 3.0 m, then 0.628 m. For ordinary 40-42 it runs 0.136 m, then 0.043 m at 1.35 m, then 0.134 m.

The cause is in `crates/telperion-core/src/pipeline/space.rs`, `converted`, in the `Origin::Relay` branch. The girth model and the drawing put a relay in two different places:

- **Girth model.** `telperion-space` `girth::thicken`, `relay_parts`, enters the relay's pipe at its bud's node by `t` and at the node below by `1 - t`, where `t = relay_span(axis, relay_at, node)`. When the relay stands early in that span (small `t`), the bud's node carries almost none of it. Beech at `relay_at: 0.15` (`beech.rs:106,199`) always does. Ordinary often does.
- **Drawing.** The drawing hangs the relay from `joint`, at or above the bud. It patches the stem from `bud + 1` to `joint` up to `girth = radius at the bud's node`, and that radius lacks the relay's pipe. The stem between the bud and the joint is therefore drawn without the relay's wood, and the relay leaves it at full girth: a constriction, then a collar.
- **The tip-relay case.** A relay whose joint is its parent's last node goes through the `Continuation | Relay` arm instead and hangs from the parent's end node. That node has the same deficit; it makes the ordinary-80-1 terrace at 0.25 m.

Probe output (beech 80-42, first trunk relay), with `bud 1 joint 5`:

| Source | Radii from the base up (m) |
|---|---|
| Parent phytomers | 0.6284, 0.5535, 0.5320, ... |
| Relay's own | 0.6284 |

The pipe went in at node 0. Node 1, the `girth`, did not get it.

## Each candidate switched off in turn (beech 80-42 and ordinary 80-42, base shot)

The sheets are `raw/sheets/switch-european-beech-80-42.png` and `raw/sheets/switch-ordinary-80-42.png`. They hold five stills each:

1. As shipped.
2. Lobes off (`/surface/lobes 0`, `lobeDepth 0`).
3. Flare off (`flareRadius 1`).
4. Bark relief off (`ridgeScale 0`, `plateCellScale 0`, `barkGrainStrength 0`).
5. Wood level forced to 0 (`Level::Forced(0)`) in place of the chosen screen-error level.

The collars and the waist stand unchanged in all five. None of these surface stages makes them.

The positive check was an env-gated diagnostic, kept out of the tree as `raw/diagnostic-relay-girth.patch`. It does two things:

- It takes the girth from the node below the bud, where the pipe has fully entered.
- It patches from the bud itself.

The bole's rises vanish in the probe. The collars and waists are gone in `raw/sheets/probe-relay-girth-{beech,ordinary}-base.png`. What remains is the tip-relay case (the ordinary-80-1 terrace), which the diagnostic does not cover, and the oak-style hairlines.

The oak hairlines are the pipe model's own step at each lateral. They are sharp because each step falls between two drawn nodes about 4 cm apart. They are not the relay defect, and they are minor at the harness views.

## Reference photographs

The only real trunks in `.refs/` are `fn9/quga788B.jpg` (Oregon white oak) and `fn9/piab977.jpg` (Norway spruce). Both are whole-tree photographs, not trunk close-ups. `raw/sheets/refs-oak-spruce.png` sets them beside the 80-year stills. Neither shows a ring or collar on the bole. A beech has a smooth, near-cylindrical bole, so the collars are not its bark form. No photograph was downloaded.

## Verdict for R1

This is not a values fix. The defect is how `converted` draws a relay's stem against where the girth model enters its pipe. The design goes to the host, and this task adds no code before then.

`relay_at` is a species' botany (Troll's relay in the curvature zone), and moving it to 1.0 would only hide the defect in beech. Ordinary dents at its own `relay_at`.

## Host decision (2026-10-09): one source for where a relay joins

**Option 2.** The relay is drawn from the node where the girth model already adds its wood. The girth model's attachment, `relay_at` and the node it resolves to, becomes the one place that says where a relay joins its parent. The drawing, `converted`'s `Origin::Relay` branch and its last-node case, reads that attachment. It no longer picks its own "node nearest its base". The trunk below the join then already carries the relay's wood, because girth computed it. No second thickening rule is added (option 1), and no value is moved to hide the dent (`relay_at` stays).

- **Both cases:** the mid-axis relay and the relay at the parent's last node.
- **The device path:** check whether it has its own copy of this conversion. If it does, it reads the same attachment. CPU and device agree within fn-125's screen.
- **The oak's hairline rings** (a 1-4 % step above each side branch) are the pipe model working as intended. They are not this defect, and no change is made.
- **Proof:**
  - A red/green test: no radius along a stem rises again just below a relay join beyond the pipe model's own step, on beech 80/42 and ordinary 80/42 and 40/42.
  - The before and after still sheets for every species, viewed.
  - `space_hash` changes only where relays are drawn, and each change is listed.
  - The gate, npm test and impl-review.

Keep the task open for the owner's verdict.

## Option 2 built and measured: it moves the dent, it does not remove it (2026-10-09)

I built option 2 on the CPU path. It is saved as `raw/option2-lowest-entry.patch` and reverted, so the code tree is unchanged:

- `telperion_space::relay_join` derives the join from `girth::relay_shares`.
- `converted` hangs the relay there.
- `joint` and the bud-to-joint thickening loop are deleted.

**What the girth model actually says.** It splits a relay's pipe three ways:
- `b * t` enters at the bud's node;
- `b * (1 - t)` enters at the node below it;
- `1 - b` enters at the parent's tip, through the stopped module's head.

Trunk relays keep a blend `b` well short of one for life. Beech at seeds 42 and 7, both ages, shows `b` = 0.35, 0.47, 0.62 and 1.0, with `t` = 0.2 or 0.9. Ordinary 40-7 shows `b` = 0.09. So most of a relay's pipe often still runs up the old leader's head.

**What a single drawn join therefore does.** The patch joins at the lowest entry taking a share, which is the node below the bud whenever `b > 0` and `t < 1`.

- **The trunk.** Every node below the join carries the whole relay. The probe finds no rise along any trunk sampled, and the 80-year beech collars are gone.
- **The head.** The head is turned aside at the join and carries `(1 - b) + b * t` of the relay. It is drawn as a lateral nearly as thick as the stem (beech 40-42: heads of 0.18 to 0.20 m on a 0.21 m trunk).
- **The new defect.** The head's thick base appears as a new flange ring at the join. See `beech-40-42`, `beech-40-1`, `ordinary-40-1` and `ordinary-40-7` in `raw/sheets/ba-*-base.png`, and `raw/sheets/option2-stubs.png`.

**The other rules would not help.**
- **The largest share as the join** leaves the trunk short of the minority share below the join, which is the original dip at a smaller size.
- **Any single node** leaves the minority share drawn in the wrong place, because the girth model spreads the pipe over the bud span and the tip while the drawing must hang the relay from one node.

**Byte changes.** Oak, palm and spruce stills are byte-identical. Birch changes in 7 of 18 stills. Beech, ordinary, Laurelin and Telperion change in all of theirs.

**Still open, a host design call.** Option 2 as written cannot be dent-free while `b < 1`. Two ways forward:
- **Change the girth side.** The drawing reads the join, and the girth model enters all of a drawn relay's pipe at that join. This changes girth, sag and the device's girth.
- **Change the drawing side.** The drawing hangs the relay at the join and draws the head as carrying only its own wood. The girth model and the drawing would then disagree on the head.

**The device path.** `crates/telperion-render/src/generation/fine/dress.wgsl` has its own copy (`joint` at line 241, the thickening loop at lines 484-499). Whichever rule is chosen goes there as well. `girth.wgsl` already has `relay_span_q`.

## Host decision, second (2026-10-09): change girth, so a relay's wood enters at its join

**Option 1.** Botanically, a relay's conducting pipes reach the parent axis through the node it grows from, and then run down the stem below. They do not pass through the old leader's tip, which becomes a stub carrying only its own wood. The girth model's split, which sends `1 - b` of the relay's wood to the old leader's tip, is the one rule out of step with that, and the dents follow from it. So the fix goes where the cause is, and girth and drawing stay one source (option 2 would make them disagree about the stub).

1. **Girth:** a drawn relay's wood enters the parent at its join. The node below carries it, and the stub carries its own wood only. Apply the same rule in the device's girth shader, and in `dress.wgsl`'s `joint` and thickening loop, so CPU and device stay one rule.
2. **The drawing:** it joins at that node, as option 2 built. Reuse the saved patch.
3. **`relay_at` and the blend `b`:** state what each still means once the wood enters at the join.
   - **If either becomes inert,** say so with a test. It is retired only on the host's word, by name in the report. Do not keep a row that does nothing.
   - **If either still shapes something real,** such as where along the parent's last unit the join sits, keep it and document it.
4. **Sag and everything else that reads girth:** these change by design. Measure the change on the beech and the ordinary tree (sag of the trunk and limbs at 40 and 80).
5. **Proof:**
   - The red/green test from before, with no rise along a stem below or above a join, plus a stub-radius check.
   - Before/after sheets for every species, viewed.
   - `space_hash` changes listed for every tree with relays. Oak, palm and spruce stay identical.
   - CPU and device agree within fn-125's screen.
   - The gate, npm test and impl-review.

Keep the task open for the owner's verdict.

On your friction note: a probe line for `b` and `t` before a decision is a good rule. It belongs in the onboarding method's "measure before building" step. The host carries it to fn-237's friction list.
