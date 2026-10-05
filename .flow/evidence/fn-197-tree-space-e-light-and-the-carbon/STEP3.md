# fn-197 step 3: light stops and slows the oak, 2026-10-05

Host decisions after steps 1 and 2 are applied:

- **Leaf area is per PA** (`PaState::leaf_area`), and `NODE_LEAF` is gone.
- **The oak's leaf area is 30 cm² a node (estimated).**
  - One leaf a node (Go Botany), about 10 cm long (Woodland Trust), about half as wide and lobed to 0.6 of its rectangle.
  - No measured leaf area was found (FRICTION.md).
- **The beech and the spruce stay at 0:** light is off for them.
- **k = 0.5** is the textbook extinction for leaves at every angle (spherical leaf-angle distribution, G = 0.5; Ross 1981, Campbell & Norman 1998; cited from memory).
- **The oak's non-light values are untouched.** Shade is set for the renders only (`space_oak --shade φ,ψ` on every PA).

## Leaves and sag: one deviation for the host

Light reads `leaf_area` × scale on the cycle's nodes. Sag's foliage term (own pipe² × length) is unchanged.

- **Why not one term:** making sag read `leaf_area` would change every sagging species at neutral light. The oak's limbs sag, and so does the spruce. That breaks the neutral rule, and it would also change the oak's look before any light acts.
- **The host's ruling is needed:**
  - keep them apart;
  - or move sag onto `leaf_area` as a separate, judged change.

## The walks (`tests/walks.rs`, `light_changes_the_tree_by_degree`)

**The walk tree in leaf:**

- limbs at 0.995 viability and twigs at 0.99, near-certain as an oak's boughs are;
- φ 1 and ψ 0.5;
- 0.3 m² of leaf a node, k 0.5, sky 0.5.

**Red first** (commit `51ad37f3`, RED-MULTIPLICATIVE.txt): with survival = viability · I^φ, the leaf-area walk fails the bound at 39.2 (bound 30). Leaf area and extinction both jump near no shade, where certain-to-near-certain survival turns a small light change into a large change in log-odds. ψ's walks are also flagged as jumps there.

**Hazard form** (`ln s = ln v · I^−φ`):

| Walk | Steepest slope |
|---|--:|
| φ limbs | 0.76 |
| φ twigs | 0.95 |
| ψ limbs | 1.30 |
| ψ twigs | 1.67 |
| leaf area | 6.48 |
| extinction | 3.89 |
| sky | 11.65 |

**One flag remains: sky at seed 3, step 0.010 to 0.015.**

- The test refines a step three times and calls it a jump unless it shrinks 20-fold. This one shrinks 13.5-fold over three levels: 0.058, 0.027, 0.0050, 0.0043.
- Refined to six levels it goes on shrinking: 0.0014, 0.00018, 0.000023. It is continuous, but its local slope reaches about 1,200 per unit of sky.
- The cause is a narrow (twig) presence window crossed while sky moves many buds' light at once.
- **The test is red on this one item.** Loosening the test, or the window, is the host's call.

## Renders (oak, 80 years, under the GPU lock)

Sheets are in `raw/step3/`, and I viewed every still:

- `explore-bare.png`, `explore-whole.png`: seed 1, under sky 0 and 0.5 × (φ, ψ) of (1, 0), (3, 0), (3, 1) and (0, 1), beside round 4 and the references;
- `sag-gap.png`: seed 1 with sag and without, no light and φ 1 and 3;
- `bare-vs-refs.png`, `in-leaf.png`: seeds 1, 7, 2, 3 and 4 at sky 0.5, φ 1, ψ 0, under round 4's same seeds.

| Seed 1 | Leaves | Wood (km) | Grown in |
|---|--:|--:|--:|
| Round 4, no light | 2.16M | 17.3 | 4.0 s |
| φ 1, sky 0.5 | 1.19M | 9.9 | 3.4 s |
| φ 3, sky 0.5 | 0.60M | 5.3 | 1.9 s |
| φ 3, ψ 1 | 0.32M | 2.7 | 2.9 s |
| ψ 1 alone | 0.40M | 3.1 | 5.8 s |

### Reading

- **No setting closes the outline toward a dome.** Light only takes away: shaded tips die (φ) or stay short (ψ).
  - At φ 1 the interior web thins and the crown opens into clumps along the boughs. The outline is where round 4 had it, as ragged or slightly more so.
  - At φ 3, or with ψ 1, the oak strips to its scaffold: long bare limbs with tufts at their ends. In leaf it reads as clumps on bare limbs, the opposite of S2's closed dome.
  - Nothing grows into the gaps between bough tips. A bough that lags is not pushed out; it is only shaded, if anything shades it.
- **Sky 0 against sky 0.5 makes little visible difference** at these settings.
- **The bole:** light alone clears no bole on the oak. Its trunk and limbs have viability 1, which the hazard form leaves immune by design, and lower boughs survive where sag carries them out.
- **Cost:** φ 3 halves the grown tree, and growth takes 1.9 s against 4.0 s. That is phase F's lever, visible already.

### Does the sag gap misplace the dome? Yes, visibly

- **Light sees a column, and the drawn tree is spread.** The rough layout has no sag, so the oak light shades is a narrow upright column 16 m across (`sag-gap.png`, bottom row). The drawn tree is 23 to 25 m across.
- **Its boughs are shaded inside the column, then drawn in the open.** Light kills the twigs on the lower and outer boughs inside the column. Sag then carries those boughs out into the open, where in the drawn tree they stand bare and unshaded: the long bare sweeping limbs at φ 3, sagged.
- **The growth light judged is the same tree in both rows.** The sagged and unsagged runs grow the same nodes (909,598 at φ 1) because growth never sees sag. Only the final lay differs.

## For the host before step 4

1. **The dome needs something light cannot give by taking away.** What fills the outline: boughs that grow toward light and into gaps, rather than only dying in shade? That is a design question. fn-190 rounds 4 and 5 found that phototropism alone made columns.
2. **The rough layout's sag gap matters on the oak.** Decision 1's re-lay with sag every K cycles looks needed before φ can be judged. It would also close the beech's erection gap.
3. **Rule on the remaining sky jump flag** (above).
4. **Rule on sag's foliage term against `leaf_area`** (above).
