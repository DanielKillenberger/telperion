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
