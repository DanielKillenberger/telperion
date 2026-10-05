# fn-195: the pedunculate oak as a point (worker)

## Round 1 (2026-10-05)

**Values only, no engine change.** `crates/telperion-space/src/oak.rs` places Rauh's model on the engine this branch carries (sag with large deflection and ground support, dormant buds, the renderer dispatch fix). Seven physiological ages: trunk, fork, limb, bough, branch system, long shoot, short shoot. Each value and its source is in SOURCES.md.

| Rule (MODEL-OAK.md) | How the engine draws it |
|---|---|
| 1, 5: rhythmic growth, acrotony | One unit a year: bare nodes at the base, laterals below the top, a cluster of 3 to 4 buds at the top |
| 2, 3: monopodial, orthotropic axes repeating the trunk | Limbs and boughs bend towards 0.85 and 0.8 rad, every axis on a 2/5 spiral |
| 6: the mature oak forks | After 13 years the top cluster grows 4 limbs; the stem carries on as a weak fifth. Limbs fork again now and then |
| 6: frequent apex death | Limbs and boughs abort at 0.25 a unit and a top bud relays them: kinks |
| 7: reserve buds | Sleeping branch buds on every limb zone (p 0.1, waking after 6 years at 0.04 a year) |
| 8: early branch death | The trunk's laterals are temporary branch systems, shed within about 16 years, so the bole clears |
| Design question 3: low limbs spread | Sag on limbs (2e-4) and boughs (5e-5) |

**Stills.** `crates/telperion-render/examples/space_oak.rs` draws them on the shared `examples/space/still.rs`, dressed by the oregon-white-oak preset's rows overlaid with no clusters, no clumping, leaves 8 mm apart and `shootRadius` 0.06 (FRICTION.md, second entry).

**Colours, in the preset** (`oregon-white-oak.values`); `catalogue_identity`'s three digests are re-pinned, so today's oak changes colour too.

| Target | Photograph | Render |
|---|---|---|
| Bark: a 500 px crop of S3-robur's trunk | 0.116 / 0.086 / 0.053 | 0.118 / 0.084 / 0.054 |
| Leaves: the green pixels of S2-garryana's crown | 0.083 / 0.101 / 0.064 | 0.115 / 0.148 / 0.071 at the first darkening |

The leaf render barely moves with the albedo: the sheen and sky terms dominate, as fn-193 found. The blade was darkened twice and its underside set grey-green; the crown now reads a dark green close to S2-garryana's.

**Probes** (`raw/p1` to `raw/p8`, `raw/c1` to `raw/c3`): the first values grew a vase with wood sprawling on the ground (the trunk's boughs). Those boughs became temporary branches; relays came in for tortuosity; the stem was carried on to fill the centre; limb sag was walked at 0, 1e-4, 1.5e-4, 3e-4, 4e-4 and 8e-4. 8e-4 puts a limb of seed 2 below the ground (`BelowGround`, an error as it should be). A first sheet at 4e-4 (`raw/round1-sag4e-4/`) read as flat umbrellas 1.6 to 1.9 times wider than tall; the sheet below is at 2e-4.

## Sheets (`raw/round1/`, ignored, on disk; I viewed every still)

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | S1 (two), S2 (garryana, robur), today's oak at seeds 1 and 7; then the 80-year trees at seeds 1, 7, 2, 3, 4 in leaf, bare, as limb close-ups (bare) and as sprays (in leaf) |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7: in leaf, bare, spray |
| `close-ups.png` | Trunk bases at 80 years (five seeds) and 40 years (two), beside S3-robur, S3-garryana and S4 |

**Measures at 80 years** (`run.log`, `err.log`). Width is the larger of the two horizontal spans.

| Seed | Height | Width | w/h | Leaves | Wood triangles | Grown in |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 17.8 m | 22.6 m | 1.27 | 3.05M | 121M | 6.0 s |
| 7 | 16.1 m | 25.2 m | 1.57 | 2.50M | 105M | 6.0 s |
| 2 | 15.6 m | 24.3 m | 1.56 | 1.45M | 94M | 4.7 s |
| 3 | 18.7 m | 23.0 m | 1.23 | 4.39M | 140M | 6.6 s |
| 4 | 17.5 m | 24.2 m | 1.38 | 3.73M | 158M | 7.4 s |

The reference w/h is 1.07 to 1.22 (MODEL-OAK, R1-BANDS). The trees stay under the 210M wood-triangle limit; none needed a GPU retry. Younger: 3.6 m at 10 years (HEIN's 1.3 to 4.2 m at about 8), 6.2 to 6.4 m at 20, 10.4 to 10.7 m at 40 (8 to 9 m wide).

## Reading, per seed (80 years)

| Seed | In leaf | Bare | Limb close-up and spray |
|---|---|---|---|
| 1 | The best: a short bole to about a third of the height, a broad, dense, dark crown slightly wider than tall. The top is flatter and the crown base a harder horizontal line than S1 or S2 | A low division into several ascending limbs and a weak centre, a broad dome of fine twigs: the nearest to S1-november. The limbs are slender for an oak and the twig mass is a tan haze | Many crossing, kinked branches; sprays dense and dark |
| 7 | Lopsided: a heavy left side and a lower, thinner right side with a gap between | One side's limbs run out long and low; reads as an oak that lost a limb | Dense, kinked; spray fine |
| 2 | **Fails:** a V-shaped "butterfly", two long limbs splaying wide with a hollow centre; the thinnest foliage of the five (1.45M leaves) | The same V; limbs long and near straight, little forking | Busy crossing wood, some heavy |
| 3 | A broad, high dome leaning left; dense | Good: several ascending limbs from a short bole, a rounded twig crown | Heavy kinked limbs; one of the better close-ups |
| 4 | A wide, flat-topped crown on a short bole; reads as a broad open-grown oak, the top too level | Broad dome, many limbs; close to S1-old-open-winter's spread, flatter on top | Long low limbs hung with dense spray |

**Overall:** seeds 1, 3 and 4 read as broad open-grown broadleaves with an oak's short bole and low division, and are a large step from today's oak (a shapeless bush). They are not yet clearly *oaks*:
- **Heavy, kinked limbs (trait 3):** the relays kink the wood, but the limbs are too slender and too many. The crown reads as fine crossing wood, not S3's few heavy, angular limbs.
- **Dome (trait 2):** too flat on top, and the crown base is a hard horizontal line. Seeds 2 and 7 are one-sided or V-shaped.
- **Twig mesh (trait 4):** full to the edge; it reads as a tan haze because the fine wood takes the browner bark colour.
- **Bark:** the trunk is a smooth column, without S3's deep plates and furrows. Seeds 1 and 4 show a horizontal collar ring near the base.

**Young trees.**
- **10 and 20 years: fail.** A straight pole with short temporary branches, which reads as a conifer sapling. Rauh's monopodial sapling is right in kind, but the laterals are too short and too few.
- **40 years:** a lollipop. A clean 5 m bole carries a compact crown, and the bole is too tall and thin for an open-grown 40-year oak.

**Spray close-ups:** the preset's five-lobed blade reads at close range (seed 7's spray shows lobed leaves). Edge-on, the blades read as rows of small leaflets.

## For the host

No engine capability was missing for round 1. Each item below is a values next step or a design question, and none is decided here.

1. **Leaf clothing moves with the stem's shape (design question).** The renderer clothes shoots thinner than `shootRadius` times the stem's radius. `Tree::stem_radius` takes that radius where the stem first parts, which moved up the weak central limb once the stem carried on, and leaves fell five-fold at seeds 1 and 2 (FRICTION.md). Options:
   - (a) keep a per-species stills row, as now;
   - (b) an absolute leaf-bearing radius for tree-space stills;
   - (c) the conversion marks the short-shoot PA's axes as leaf-bearing, so leaves follow the botany.
2. **Seed variety versus symmetry (values).** Seeds 2 and 7 splay one-sided because 4 fork limbs at one insertion wander freely in azimuth. Candidates:
   - lower limb wander;
   - `dominance` on limbs;
   - a fork over two units, so limbs leave at several heights (K19's main branches "of unequal insertion height").
3. **Heavy few limbs (values).** Fewer limb forks and boughs, a larger limb pipe and exponent, and more bough shedding, so that a few heavy limbs carry the crown.
4. **Saplings (values).** Longer, more vigorous trunk laterals in the first 10 to 20 years, and a fork earlier than 13 years at some seeds.
5. **Bark relief and the collar ring.** These belong to the renderer and preset, not to the tree space. Whether the oak's bark plates are in scope for this spec is the host's call.

Not run, as directed: Astra, the review, the gate, the R3 walk, the R4 regression. The task is not marked done.
