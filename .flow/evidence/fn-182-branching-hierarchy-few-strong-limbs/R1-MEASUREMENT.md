# fn-182 R1: branching hierarchy per order (measurement only)

Measured 2026-09-28 on master `0ae725d0`, in a detached checkout with its own `target/`. The tool is an uncommitted example, `crates/telperion-core/examples/branch_orders.rs`. It runs the direct build (`pipeline::build`, skeleton only), then assigns every node to an axis and an order:

- An axis runs through each node's `BudFate::Terminal`, non-codominant child.
- A `BudFate::Lateral` node starts an axis one order up.
- A codominant sibling starts an axis at its parent's order. These are counted as "forks" and excluded from the lateral statistics.
- The twig law's laterals are counted by the same rule, so the orders run on through the twig generations.

`maxTurnPerStep` is set the way `presets::by_identity` sets it. The trees are:

- **european-beech:** in-work preset, `Preset::from_id`.
- **oregon-white-oak:** shipped preset.
- **plane:** `oregon-white-oak` with fn-170's `plane-candidate.json` overlaid, plus `radii.girthHold 0.75` and `radii.girthFall 4`.

Seeds 1 to 4. The tables average the per-seed values: medians, the IQR ends and the densities.

**The quantities:**

- **laterals / m parent:** order-k laterals ÷ total length of the order-(k−1) axes.
- **length / parent:** the lateral axis's length ÷ its parent axis's whole length.
- **birth girth / parent:** the lateral's first `start_radius` ÷ the parent node's `radius` at the junction. This is the drawn radius, so `girthHold` is included.
- **along:** where on the parent the lateral leaves; 0 is the parent's base, 1 its tip.
- The "structural" columns restrict the lateral statistics to scaffold laterals (first node `NodeKind::Structural`). The other columns also include the twig law's `Branch` and `Twig` laterals.

## Node budget

The shipped `maxNodes` is unset, which means 250,000. There is no `twig_detail` reduction on master, because fn-180 is still open. The only signal is `diagnostics.node_capped`.

| tree | maxNodes used | nodes s1–s4 | structural nodes | node_capped | build (skeleton) |
|---|---|---|---|---|---|
| beech | default 250k | 187,968–191,358 | 3,850–3,931 | no | ~100–200 ms |
| oak | default 250k | 125,087–149,461 | 2,750–3,392 | no | ~60–150 ms |
| plane | **raised to 1,000,000** | 556,275–594,144 | 36,861–45,531 | no | ~300–900 ms |
| plane (for reference) | default 250k | 250,000 on all four | same | **yes, all four** | ~200–280 ms |

At the default budget the plane is cut in its twig layer. Its order-2 mean axis falls from 1.82 m to 0.96 m, order 3 from 0.75 m to 0.49 m, and order 5 vanishes. The plane table below is the unreduced tree at 1M.

## European beech (in-work preset), seeds 1–4, unreduced

Root radius 0.512 m. One excurrent stem, 31.9 m, no forks.

| order | axes | total length m | mean axis m | laterals / m parent | laterals per parent axis med [IQR] | length / parent med [IQR] | birth girth / parent med [IQR] | birth r med cm | structural axes | structural: length / parent | structural: girth / parent | structural birth r cm | along med |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 1 | 32 | 31.9 | – | – | – | – | – | 1 | – | – | – | – |
| 1 | 54 | 247 | 4.58 | 1.69 | 54 | 0.07 [0.05–0.21] | 0.17 [0.14–0.50] | 1.5 | 18 | 0.34 [0.21–0.43] | 0.57 [0.50–0.66] | 17.8 | 0.71 |
| 2 | 825–839 | 2,056 | 2.47 | 3.37 | 5 [4–33] | 0.21 [0.16–0.37] | 0.16 [0.14–0.27] | 1.6 | 100 | 0.65 [0.65–0.65] | 0.68 [0.58–0.79] | 9.4 | 0.71 |
| 3 | 7,391–7,461 | 11,069 | 1.49 | 3.61 | 4 [4–6] | 0.19 [0.12–0.26] | 0.16 [0.14–0.21] | 0.97 | 371 | 0.76 [0.46–1.14] | 0.69 [0.62–0.79] | 5.7 | 0.53 |
| 4 | 32,749–33,295 | 20,593 | 0.62 | 2.98 | 4 [0–6] | 0.13 [0.12–0.16] | 0.23 [0.16–0.31] | 0.25 | 0 | – | – | – | 0.43 |
| 5 | 40,270–41,360 | 10,174 | 0.25 | 1.98 | 0 [0–4] | 0.16 [0.15–0.17] | 0.39 [0.34–0.46] | 0.25 | 0 | – | – | – | 0.43 |

Laterals on structural parents only:

| lateral order | structural parents | their length m | all laterals / m | structural laterals / m | all per parent med [IQR] | structural per parent med [IQR] |
|---|---|---|---|---|---|---|
| 1 | 1 | 32 | 1.69 | 0.56 | 54 | 18 |
| 2 | 18 | 181 | 3.69 | 0.55 | 37 [33–43] | 6 [4–7] |
| 3 | 100 | 708 | 6.74 | 0.52 | 48 [35–62] | 4 [3–5] |
| 4 | 371 | 2,281 | 4.74 | 0 | 29 [20–40] | 0 |

## Oregon white oak (shipped), seeds 1–4, unreduced

Root radius 0.432 m. The trunk axis is 5.9 m and ends at the crown base, where 10 limbs leave together (along = 1.0). No forks.

| order | axes | total length m | mean axis m | laterals / m parent | laterals per parent axis med [IQR] | length / parent med [IQR] | birth girth / parent med [IQR] | birth r med cm | structural axes | structural: length / parent | structural: girth / parent | structural birth r cm | along med |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 1 | 6 | 5.86 | – | – | – | – | – | 1 | – | – | – | – |
| 1 | 10 | 115 | 11.5 | 1.71 | 10 | 2.05 [1.65–2.27] | 0.38 [0.25–0.44] | 12.4 | 10 | 2.05 [1.65–2.27] | 0.38 [0.25–0.44] | 12.4 | 1.00 |
| 2 | 178–223 | 653 | 3.22 | 1.76 | 21 [15–25] | 0.24 [0.19–0.45] | 0.37 [0.33–0.42] | 1.5 | 76 | 0.45 [0.45–0.45] | 0.47 [0.36–0.64] | 3.8 | 0.93 |
| 3 | 4,426–5,472 | 8,530 | 1.75 | 7.48 | 7 [6–50] | 0.33 [0.26–0.40] | 0.35 [0.31–0.39] | 0.95 | 269 | 0.89 [0.82–0.98] | 0.66 [0.52–0.72] | 1.7 | 0.55 |
| 4 | 25,323–31,327 | 16,065 | 0.58 | 3.27 | 5 [4–6] | 0.14 [0.12–0.24] | 0.34 [0.29–0.40] | 0.25 | 0 | – | – | – | 0.42 |
| 5 | 25,751–30,582 | 7,055 | 0.25 | 1.76 | 0 [0–3] | 0.18 [0.17–0.19] | 0.54 [0.47–0.61] | 0.25 | 0 | – | – | – | 0.40 |

Laterals on structural parents only:

| lateral order | structural parents | their length m | all laterals / m | structural laterals / m | all per parent med [IQR] | structural per parent med [IQR] |
|---|---|---|---|---|---|---|
| 1 | 1 | 6 | 1.71 | 1.71 | 10 | 10 |
| 2 | 10 | 115 | 1.76 | 0.66 | 21 [15–25] | 8 [6–9] |
| 3 | 76 | 384 | 10.80 | 0.70 | 57 [47–64] | 4 [3–4] |
| 4 | 269 | 1,226 | 6.35 | 0 | 32 [26–34] | 0 |

## Plane candidate (oak + fn-170 plane-candidate + girthHold 0.75 / girthFall 4), seeds 1–4, maxNodes 1,000,000, unreduced

Root radius 0.468 m. There are 4–7 order-0 axes: the trunk plus 3–6 codominant stems, 60 m of stem in total. A further 42 forks (mean) part limbs at order 1 and 36 at order 2; these are excluded from the lateral statistics.

| order | axes (incl. forks) | total length m | mean axis m | laterals / m parent | laterals per parent axis med [IQR] | length / parent med [IQR] | birth girth / parent med [IQR] | birth r med cm | structural axes | structural: length / parent | structural: girth / parent | structural birth r cm | along med |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 4–7 | 60 | 9.81 | – | – | – | – | – | 6 | – | – | – | – |
| 1 | 164–230 | 1,933 | 9.92 | 2.58 | 22.5 [22–37] | 0.79 [0.57–1.05] | 0.18 [0.12–0.31] | 3.0 | 195 | 0.78 [0.57–1.05] | 0.18 [0.12–0.31] | 3.0 | 0.71 |
| 2 | 8,579–14,793 | 21,651 | 1.82 | 6.08 | 61 [50–70] | 0.14 [0.11–0.19] | 0.30 [0.25–0.36] | 0.67 | 1,340 | 0.45 [0.45–0.45] | 0.38 [0.32–0.64] | 1.2 | 0.68 |
| 3 | 77,903–112,868 | 71,677 | 0.75 | 4.51 | 4 [3–5] | 0.17 [0.14–0.21] | 0.38 [0.31–0.47] | 0.25 | 4,788 | 0.70 [0.65–0.77] | 0.56 [0.48–1.00] | 0.6 | 0.50 |
| 4 | 188,684–211,585 | 55,420 | 0.28 | 2.81 | 0.5 [0–2] | 0.13 [0.06–0.22] | 0.56 [0.42–0.77] | 0.25 | 0 | – | – | – | 0.44 |
| 5 | 328–30,651 | 3,892 | 0.25 | 0.27 | 0 | 0.27 | 0.98 | 0.25 | 0 | – | – | – | 0.40 |

Laterals on structural parents only:

| lateral order | structural parents | their length m | all laterals / m | structural laterals / m | all per parent med [IQR] | structural per parent med [IQR] |
|---|---|---|---|---|---|---|
| 1 | 6 | 60 | 2.58 | 2.56 | 22.5 [22–37] | 22.5 [21–37] |
| 2 | 195 | 1,931 | 6.09 | 0.67 | 61 [50–70] | 6.8 [5–9] |
| 3 | 1,340 | 6,764 | 8.53 | 0.71 | 44 [33–54] | 3.8 [3–4.5] |
| 4 | 4,788 | 18,077 | 6.29 | 0 | 24 [20–28] | 0 |

## The photographs (host's read, qualitative; one view each)

These are reads by eye from single photographs, not measurements. Counts are what is visible and are approximate.

**London plane, Schützenmattpark Basel (in leaf).**

- **Order 0:** a clear trunk to about 1/5 of the height, then it parts into about 3 to 4 thick leaders, roughly one central pair and one to either side. A further low limb leaves on the left.
- **Order 1, the leaders:** each is roughly 0.5 to 0.7 of the trunk's diameter at the fork. They stay thick for most of their length and run almost to the crown top, so their length is about 0.7 to 0.9 of the tree above the fork.
- **Order 2:** few per leader. About 3 to 6 are visible per leader over roughly 10 to 15 m, so about 0.3 to 0.5 per metre, mostly in the upper half. Each is about 0.3 to 0.5 of its leader's diameter and several metres long.
- **Order 3 and up:** not individually visible. Fine twigs and leaves concentrate in an outer shell of roughly the outer 1 to 3 m. The interior shows bare, smooth leaders and second-order wood with little fine branching on it.

**Beech B-BARE (fasy896, leafless, small image).**

- **Order 0:** a short clear trunk, about 1/5 of the height. The trunk then dissolves, around 30 to 40 percent of the height, into about 4 to 6 steeply ascending first-order limbs or leaders. Among them the leader is not clearly dominant.
- **Order 1:** each is roughly 0.4 to 0.6 of the trunk's diameter, long, and runs most of the way to the top in a narrow upright ovoid.
- **Order 2:** more numerous than on the plane but still spaced. Long, slender, ascending branches, perhaps 5 to 10 visible per limb; the resolution is too low to count reliably.
- **Order 3 and up:** a dense, fine, even twig haze over the whole crown periphery, thickest in the outer shell and top. The inner crown reads as a few dark limb lines through a fine mesh.

**Beech B-WHOLE (fasy951, two trees in leaf).**

- **Structure:** mostly hidden. On the left tree the trunk forks low into two stems. On the right a single trunk is clear to about 1/6 of the height before the crown.
- **Foliage:** in layered sprays at the periphery with a dark interior. This supports the reading that leaf-bearing twigs sit at the crown's surface, not on the inner wood.
- **Per-order counts:** not readable from this image.

## Which quantities differ most, and at which orders

1. **Birth girth at order 1 differs most, on the plane.** The plane candidate's limbs and leaders are born at a median 0.18 (IQR 0.12 to 0.31) of the stem at the junction, 3.0 cm in radius on a 47 cm root. The photograph's leaders are about 0.5 to 0.7 of the trunk, and its second-order wood about 0.3 to 0.5 of the leader. The candidate's own order-2 ratio (0.30 to 0.38) is already in the photograph's range; the deficit is at order 1.

   The beech's 18 structural limbs are born at 0.57 (17.8 cm), close to B-BARE's roughly 0.4 to 0.6. The oak's 10 are born at 0.38 (12.4 cm).

2. **The count and density of laterals on scaffold wood differ at orders 1 to 3, on all three trees.**
   - **Order 1:** the plane candidate has 164 to 230 order-1 axes, about 22 per stem at 2.6 per metre, against the photograph's 3 to 4 leaders plus one or two low limbs. The beech has 18 structural limbs plus 36 twig-law shoots on its trunk (54 per trunk, 1.7 per metre), against B-BARE's roughly 4 to 6.
   - **Orders 2 and 3:** the scaffold's own laterals are sparse on all three trees, 0.5 to 0.7 per metre and 4 to 8 per parent. That is near the photographs' few per limb. The twig law, however, seeds laterals on every structural axis whose radius is under `limbRadius`. Each structural limb then carries a median 37 laterals on the beech, 21 on the oak and 61 on the plane, at 3.7 to 10.8 per metre along the scaffold. The photographs show the inner scaffold nearly bare.
   - This mid-order density is where the three trees differ most from the photographs in count, and it is mostly twig-law wood (`NodeKind::Branch`) born directly on limbs, not scaffold laterals.

3. **Where the fine twigs sit differs at orders 3 to 5.** The fine orders are laid evenly along their parents: the median birth position is 0.40 to 0.55 of the parent's length, with an IQR of about 0.25 to 0.75 on every tree. They are not concentrated toward the tips or the crown's edge.
   - Total length peaks at order 3 or 4: 11 to 21 km on the beech, 8.5 to 16 km on the oak, 72 km at order 3 on the plane.
   - Order-4 and order-5 twigs are single 0.25 m segments at 2 to 3 per metre of their parent.
   - The photographs put the fine twigs in an outer shell and leave the interior limbs bare. This finding rests on the along-parent position only; no radial or crown-shell distribution was measured.

4. **Length ratio differs least.**
   - **Plane:** order-1 length ÷ parent is 0.79. The leaders reach far, which matches the photograph; the photograph's structure differs in count and girth, not reach.
   - **Beech:** its structural limbs are short against its 32 m excurrent trunk (0.34, about 11 m), where B-BARE's limbs run most of the way to the top of a crown whose trunk dissolves. So the beech's order-1 difference is partly a trunk-form difference (excurrent leader against dissolving leaders), not only a length ratio.
   - **Orders 2 to 5:** the ratios of 0.13 to 0.24 give the short twigs the photographs show.

In short, the plane's first-order girth is too thin: about 0.2 of the parent against about 0.5 to 0.7 in the photograph. First-order and mid-order lateral counts are far too high on scaffold wood, mostly because the twig law seeds 20 to 60 laterals along every limb (3.7 to 10.8 per metre). The fine orders are spread evenly along their parents instead of gathered at the edge. The length ratio is roughly right at every order, except the beech's short limbs on its excurrent trunk.
