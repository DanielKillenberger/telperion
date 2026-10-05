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

## Host decisions after round 1 (2026-10-05)

- Leaf clothing: (a), a per-species stills row for now.
- Round 2 values as proposed, plus: a rounded dome at about 1.0 to 1.2 wide for its height; height about 20 to 25 m at 80 years where sources support it; fine twigs dark, not a tan haze; at 40 years a lower crown base, not a lollipop.
- Bark plates and the collar ring are out of scope (known gaps). The collar ring is to be fixed only if it is the beech's conversion-joint mechanism.

**Phase F item (fn-198's station contract):** the conversion should mark the short-shoot PA's axes as leaf-bearing, so leaves follow the botany. Today the renderer clothes shoots by `shootRadius` times the stem's radius, which moves with where the stem first parts.

## Round 2 (worker)

**Staging, checked:** `space_oak.rs` already uses the shared `examples/space/still.rs`, which sets the sun at azimuth 115°, elevation 60°. The camera's `hero_pose` looks from the same azimuth (`Shot::default`, 115°), and the shadow fit uses the same azimuth convention. So the sun is behind the camera. The shadow reaches in front of the trunk because the crown is broad: a crown 20 m across, lit from 60°, throws its shadow back by only 3 to 10 m, so its front edge lies ahead of the trunk. The narrow spruce hides its shadow behind itself. `raw/today/` is drawn by fn-190's runner, which has no sun row.

**Values** (`oak.rs`; SOURCES.md is updated):

| Change | Why |
|---|---|
| Fork over 2 years, 3 buds a year at p 0.55: limbs at two heights | Seeds 2 and 7's one-sided splay; K19's unequal insertion heights |
| A leader for 20 years above the fork, shoots of 0.4 m, bearing limbs and boughs, then a limb | A stronger central stem; the top fills and rounds; the height |
| One limb PA, elevation 0.95 (was 0.85), wander 1.0 (was 1.5), pipe 0.009 at exponent 2.8 (was 0.006 at 2.6), forks 0.007, boughs 0.08 (was 0.15), sag 2.5e-4 | More ascending and fewer, heavier limbs. The leader's limbs are younger and shorter than the fork's, so the crown rounds |
| Trunk laterals are long shoots (`SPRIG`, 0.2 m a year for 8 years); trunk 11 years (was 13) | Longer laterals on young trunks; a lower crown base |
| Young wood dark grey-brown under 12 mm (preset; identity re-pinned) | Fine twigs read dark |

**Tried and dropped** (`raw/q1` to `raw/q5`):
- A 30-year leader made a conifer-like spire at 40 years.
- Separate steep upper limbs on the leader (elevation 1.2), with the fork's limbs at 0.7, made a tall column, and the fork's long limbs lay on the ground.
- Limb sag of 6e-4 and above drops limbs to the ground. At 1e-3, seed 7 errors `BelowGround`.
- The sag walk at 1e-4, 2e-4 and 3e-4 put the round dome between 2e-4 and 3e-4.

**Sheets** (`raw/round2/`, ignored, on disk; I viewed every still). The layout is round 1's; the reference row shows round 1's seed 1 in place of today's seed 7.

**Measures** (`run.log`; heights and widths from the stills runner, x by z):

| Seed (80 years) | Height | Width | w/h | Leaves | Wood triangles | Grown in |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 21.7 m | 23.8 × 18.5 m | 0.85 to 1.10 | 2.08M | 60M | 3.1 s |
| 7 | 19.8 m | 19.5 × 21.6 m | 0.98 to 1.09 | 2.17M | 67M | 4.1 s |
| 2 | 20.4 m | 19.4 × 25.5 m | 0.95 to 1.25 | 3.37M | 95M | 4.9 s |
| 3 | 20.6 m | 22.7 × 22.4 m | 1.09 to 1.10 | 3.03M | 103M | 5.9 s |
| 4 | 19.2 m | 24.9 × 21.9 m | 1.14 to 1.30 | 0.77M | 45M | 2.5 s |

At 40 years the trees are 13.0 to 13.3 m tall and 7 to 8 m wide, against Jüttner's 12.0 to 15.6 m. At 20 years they are 7.0 m; at 10 years, 3.6 m.

### Reading, per seed (80 years)

| Seed | In leaf | Bare | Limb close-up and spray |
|---|---|---|---|
| 1 | **Not a dome**: a narrow, ragged ovoid, taller than wide, with a side lobe | A central stem with short limbs; reads as a generic broadleaf | Fine, dark-tipped branches |
| 7 | **The best**: a broad, rounded crown on a short bole, limbs spreading from a low division, one low limb reaching out. Reads as an open-grown oak at whole-tree distance | Spreading, kinked limbs from a low fork; a central stem | The camera sits inside the crown, so the spray is a wall of leaves |
| 2 | Broad and irregular; a long low limb on the left; the top rounded but lumpy | Spreading limbs and a strong centre. A heavy low limb crosses in front of the trunk at about 2 m (base close-up) | Inside the crown, as seed 7 |
| 3 | **Good**: a rounded dome, close to S1-november's outline, a little denser | A broad dome of ascending limbs on a short bole | Spreading branches, dark tips |
| 4 | Sparse and open: thin foliage (0.77M leaves, 73 a metre of fine wood against about 130 at the other seeds), the limbs visible through it | An open crown of a few long limbs | A long limb with a light spray |

**Overall:**
- **Dome (trait 2):** seeds 7 and 3 now read as rounded, broad oak crowns. Seed 2 is broad but lumpy. Seed 1 is too narrow, and seed 4 too sparse. Round 1's flat fans and seed 2's butterfly are gone.
- **Limbs (trait 3):** fewer and heavier than round 1, and kinked, but still not S3's few massive, angular limbs.
- **Twigs:** the fine ends now read dark. The middle branches (12 to 25 mm) still read pinkish tan in sun, so the bare crowns are less hazy but not yet S1's dark mesh.
- **Height:** 19.2 to 21.7 m, at the lower end of the 20 to 25 m asked for.
- **Young trees:**
  - 40 years: no longer a lollipop. An ovoid crown from about 4 m, though still a young-tree spire at the top.
  - 10 and 20 years: still conifer-like, a straight pole with whorl-like long shoots. Better clothed than round 1, but not an oak sapling.

### Known gaps and checks

- **Collar ring:** a faint horizontal band about 0.3 m up the trunk at seeds 1, 7, 2 and 3. It is **not** the beech's mechanism. That came from relays at module joints (`aside` in `space/tree.rs`), and the oak's trunk never aborts, so it has no relays and `aside` never runs on it. Not fixed, as directed; a known gap with the bark plates.
- **Leaf density varies by seed** with the fixed `shootRadius` row (seed 4: 73 leaves a metre against about 130), the phase F item above.
- **Spray close-ups:** for a broad crown the close-up camera stands inside the foliage (seeds 7 and 2). The staging is shared with the beech and spruce, so it is left unchanged and noted.

## Host decisions after round 2 (2026-10-05)

The staging explanation is accepted. The main gap is the bare tree: S1 and S2 are domes made of a dense web of fine twigs of many orders whose ends form one smooth outline, where round 2 has a few spiky upright boughs with sparse twigs. Round 3, values:
- a fine twig web (another order, more laterals, shorter twig internodes, within the 210M-triangle limit);
- a smooth envelope (less upturn at bough tips, more equal bough lengths);
- seed 1 wider by values that apply to every seed; seed 4's leaves checked against its fine wood;
- bushy saplings at 10 and 20 years.

## Round 3 (worker)

**Values** (`oak.rs`, `e8bc056e`; SOURCES.md is updated):

| Change | Aimed at |
|---|---|
| A twig order (`TWIG`): 3 years of short growth at 15 mm internodes, bearing short shoots, borne along and at the top of every branch system | The fine web |
| Branch systems live 14 years at viability 0.97 (were 12 at 0.95), at 20 mm internodes (was 25); boughs bear fewer of them (p 0.3, was 0.45 and 0.35) | Fine wood kept inside the crown instead of shed, within the grown budget |
| Boughs: tropism 0.4 towards 0.6 rad (was 0.8 towards 0.9) | Less upturn at the outline |
| Limbs leave at 0.6 rad from their bearer (was 0.45) | A wider spread at every seed |
| Leader 15 years (was 20) | Seed 1's columnar top |
| Sapling laterals (`SPRIG`): 0.35 m a year for 10 years, ascending at 0.7 rad, bearing twigs and branch systems; borne at p 0.4 and 0.6 (were 0.35 and 0.5) | Bushy saplings |
| Young wood dark under 20 mm (was 12; preset, identity re-pinned) | The tan haze |

**Tried and dropped** (`raw/r1` to `raw/r3`):
- A twig order bearing long shoots, with long shoots bearing long shoots, was supercritical: 1.6M phytomers at 20 years and the 20M budget refused at 80. The fine-wood chain is now subcritical.
- 18-year branch systems with 4-year twigs refused the budget at three seeds.

**Sheets** (`raw/round3/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 2's seed 7 in leaf and bare, then the 80-year trees in leaf, bare, limb and spray |
| `bare-vs-refs.png` | S1-november, S1-old-open-winter and S4 beside the five bare 80-year trees |
| `young.png` | 10, 20 and 40 years |
| `close-ups.png` | Trunk bases and close references |

**Measures at 80 years** (`run.log`, `err.log`):

| Seed | Height | Width (x × z) | Leaves | Leaves a metre of fine wood | Wood triangles | Grown in |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 21.5 m | 21.8 × 21.2 m | 2.04M | 136 | 75M | 3.6 s |
| 7 | 20.3 m | 18.4 × 19.8 m | 2.34M | 137 | 92M | 5.4 s |
| 2 | 19.6 m | 19.8 × 25.1 m | 2.48M | 106 | 117M | 6.1 s |
| 3 | 20.6 m | 17.3 × 20.1 m | 2.18M | 112 | 126M | 7.2 s |
| 4 | 19.4 m | 23.7 × 21.5 m | 1.40M | 139 | 56M | 3.2 s |

Expected grown phytomers: 14.0M at 80 years, under the 20M budget.

| Age | Height | Width |
|--:|--:|--:|
| 40 | 12.0 to 12.7 m | 8.4 to 10.7 m |
| 20 | 6.8 to 7.1 m | 5.7 to 6.6 m |
| 10 | 3.5 to 3.6 m | 3.3 to 4.5 m |

**Seed 4's leaves now follow its fine wood:** 139 a metre, against 73 in round 2. Seeds 2 and 3 sit a little lower (106 to 112).

### Reading against the bare references (`bare-vs-refs.png`)

S1-november and S1-old-open-winter are domes. The limbs divide again and again into ever finer, near-equal branches, and their tips end together on one smooth outline. The web is dark against the sky.

| Seed | Bare, beside S1 | In leaf |
|---|---|---|
| 1 | Wider than round 2, but still taller than wide and leaning, a ragged column. The twig web is denser, as tufts along the boughs | A dense ovoid, lumpy at the top |
| 7 | Broad-shouldered, with a central stem and boughs rising to spiky tips. The web fills between the boughs better than round 2 | Broad, but with a low limb hanging out on the left; less round than round 2's seed 7 |
| 2 | The broadest, with low boughs spread nearly level. The web is dense, but the outline is a ragged fan, not a dome | Broad and bushy, with clumps along the outline |
| 3 | A V of two upright boughs with a gap between | Two lobes; less round than round 2's seed 3 |
| 4 | A tall left bough and a spreading right side; lopsided | Open and lopsided, but no longer sparse |

**Overall, against the bare references:**
- **Better:**
  - more fine wood stays inside the crown, so the bare trees read as a web rather than bare rods with tufts at the ends;
  - the fine wood is darker;
  - seed 4's foliage follows its wood.
- **Not yet S1:**
  - **Outline:** still ragged and spiky. The bough tips end at different distances, and the web is a set of tufts along a few boughs, not one shell.
  - **Colour:** wood of 20 to 40 mm still reads pale tan in full sun.
  - **Shape at seeds 3 and 7:** less round than round 2. The sheet traded round 2's in-leaf dome at those two seeds for a denser web at all five.
- **Young trees:**
  - 10 and 20 years: bushy, ascending saplings, no longer conifer poles. A thin leader still spires above.
  - 40 years: a bushy ovoid with sapling twigs still low on the bole.
- **Close-ups:**
  - The limb close-ups show many orders of crossing fine wood.
  - In leaf, the close-up camera still stands inside the crown.
  - At 10 years the oak blade, edge-on, reads as rows of leaflets.

### For the host

- **A smooth outline (design question).** In the references the outline is even because the tips stop where their neighbours stop. That is crown packing under light and space, which this engine does not model; phase E's light and carbon balance is where it would come from. The values tried here give each bough an independent random length, so the outline stays ragged. Options:
  - (a) accept a ragged outline until phase E;
  - (b) raise viability and lower wander on boughs and branch systems, so the lengths spread less (values only, likely stiffer and more regular);
  - (c) bring forward a crown-envelope or space-competition rule, which is engine work and its own spec.
- **Round 2 against round 3:** round 2's seeds 7 and 3 had the rounder in-leaf domes, and round 3 the better bare web. Whether to keep round 3's web and go back towards round 2's limb set is the host's call.
- **Known gaps, unchanged:** the collar ring (not the beech's relay mechanism), bark plates, close-up staging inside broad crowns, and the phase F leaf-bearing marking.

## Host decisions after round 3 (2026-10-05)

1. **Smooth outline:** (b), values-only evening, now. If the outline is still ragged after round 4, "smooth crown envelope" is recorded as an E-dependent trait, and the oak's final judgement comes on phase E's engine (fn-197: light, shadow propagation, carbon-bounded growth). Host decision 5; not a stop.
2. **Base:** round 2's limb set (fork heights, leader length, limb spread) with round 3's twig web, sapling laterals and dark young wood.
3. **Seed 3's V:** settle it by the round 2 limb set, else by `dominance` among the fork limbs.
4. **Pale mid wood:** darken it a little toward the bark colour (preset; re-pin).

## Round 4 (worker)

**Values** (`oak.rs`, `e1b6e1ea`; SOURCES.md is updated):

| Change | Aimed at |
|---|---|
| Limb insertion back to 0.45 and the leader back to 20 years (round 2) | The base the host chose |
| Boughs: viability 0.999 (was 0.995), wander 0.8 (was 1.5), rising at 0.6 towards 0.75 rad (between round 2's 0.8 and 0.9 and round 3's 0.4 and 0.6) | Evener bough lengths; round 3's flat boughs made tiers |
| Branch systems: viability 0.98 (was 0.97), wander 0.8 (was 1.5), 12 years (was 14, to stay inside the growth budget); twigs: wander 1.2 (was 2.0) | Evener fine wood |
| Limb sag 3e-4 (was 2.5e-4) | Seed 1's narrow column |
| Young wood under 30 mm, blending to the bark by 60 mm, 0.08 / 0.066 / 0.05 (preset; identity re-pinned) | The pale 20 to 40 mm wood |

`dominance` was not needed: on round 2's limb set, seed 3 has a central stem and no V.

**Probes** (`raw/s1`, `raw/s2`):
- With round 3's flat boughs on round 2's limb set, seed 3 reads as tiers along a central stem.
- Branch viability 0.985 at 14 years exceeded the growth budget at seed 3.

**Sheets** (`raw/round4/`, on disk; I viewed every still): `bare-vs-refs.png`, `five-seeds.png` (the reference row ends with round 3's seed 7), `young.png` and `close-ups.png`.

**Measures at 80 years** (`run.log`, `err.log`):

| Seed | Height | Width (x × z) | Leaves | Leaves a metre of fine wood | Wood triangles | Grown in |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 21.9 m | 23.5 × 18.6 m | 2.16M | 135 | 80M | 4.0 s |
| 7 | 20.1 m | 19.9 × 21.5 m | 2.22M | 136 | 87M | 6.1 s |
| 2 | 20.7 m | 19.7 × 25.2 m | 3.51M | 134 | 127M | 6.9 s |
| 3 | 20.8 m | 22.0 × 22.1 m | 3.09M | 137 | 134M | 7.6 s |
| 4 | 19.0 m | 24.8 × 21.8 m | 0.87M | 83 | 58M | 3.3 s |

Expected grown phytomers are 14.3M against the 20M budget.

| Age | Height | Width |
|--:|--:|--:|
| 40 | 13.0 to 13.3 m | 8 to 9 m |
| 20 | 7.0 m | 6.0 to 6.6 m |
| 10 | 3.6 m | 2.4 to 4.1 m |

### Reading against the bare references (`bare-vs-refs.png`)

| Seed | Bare, beside S1 | In leaf |
|---|---|---|
| 1 | Still a narrow, leaning column; the extra sag did not widen it | A dense ovoid, taller than wide |
| 7 | Broad-shouldered, with a central stem to a peaked top; a dense web along every bough | Broad below, peaked at the top |
| 2 | The broadest: low boughs spread wide, a dense web, but a ragged fan of bough tips at the outline | Broad and irregular, with lumps along the outline |
| 3 | **No V:** a central stem, boughs rising both sides, and a web filling between them; a vase-shaped outline with a peaked top | Broad, peaked |
| 4 | Open and lopsided, a long low limb on the left | Sparse again: 83 leaves a metre of fine wood against about 135 at the others (the phase F item) |

**Overall:**
- **Fine web:** the twig web of round 3 is kept, and the fine wood is darker. The limb close-ups now read grey-brown rather than pink-tan, and the bare crowns read as a dark web against the sky more than round 3's.
- **Outline: still ragged.** Evener bough lives and less wander made the boughs straighter and steadier, but their tips still end at different distances. The crowns are vase-shaped with peaked tops, not S1's even dome. A further step would stiffen the tree (straight, regular boughs), so I stopped here as directed.
- **Shape:**
  - Seed 3's V is gone.
  - Seeds 7 and 3 read broad but peaked. Round 2's in-leaf domes at those seeds were rounder, while round 4 has the better web.
  - Seed 1 stays narrow.
- **Young trees:** as round 3. Bushy ascending saplings at 10 and 20 years, and a bushy ovoid at 40 with a slight spire.
- **Close-ups:** the trunk bases are unchanged (the collar band remains). In leaf, the close-up camera still stands inside the crown.

### E-dependent trait (host decision 5)

**Smooth crown envelope.** In S1 and S2 the outline is even because every tip stops where its neighbours shade it. After round 4 the values-only evening does not produce that. It is recorded as an E-dependent trait: phase E's light pass and carbon-bounded growth (fn-197) is where the oak's outline is expected to come from, and the oak's final judgement comes on E's engine.

**Known gaps, unchanged:**
- the collar ring (not the beech's relay mechanism);
- bark plates;
- close-up staging inside broad crowns;
- the phase F leaf-bearing marking, which seed 4's sparse foliage shows again.

## Round 5: the oak with light as its own values (fn-197's engine; host decision 29), 2026-10-05

**Changes** (`oak.rs`, the shared `state()` every PA starts from; nothing else in the oak changes):

| Value | Round 5 | Source |
|---|---|---|
| Leaf area a node | 18.1 cm² (was an estimated 30) | Visakorpi et al. 2020, Table 1; SOURCES.md |
| φ (shade hazard), ψ (shade size), λ (apical control) | 1, 1, 0.45 | fn-197 host values |
| Upkeep, balance hazard, tolerance | 0.35, 2, 0 | fn-197 host values |
| Retained, leaf girth | 0.5, 1 | fn-197; retained tried at 0.4, 0.6 and 0.8 (`raw/round5/explore.png`) |
| Limbs' dead-branch persistence | 5 years | fn-197 (E-side), estimated |

- **The site's light is the request's,** not the species': the renders take `space_oak --light 0.5,0.5` (k 0.5, standard overcast sky).
- **At neutral light the oak's girth now carries the retained pipes,** so its 80-year hash differs from round 4's.
- **`material_detail` fixed:** the expected young-wood list now includes `oregon-white-oak`. Round 4 gave the oak a young-wood row and left the list behind.
- **`tests/light.rs`'s neutral test** sets each species' light-reading values to 0 before comparing, since the oak now reads light by its own values.
- **Tests:** `cargo test --profile ci -p telperion-space` is green, and `material_detail` passes.

**Retained, seed 1** (`raw/round5/explore.png`): every step thickens and flares the trunk's base, from 0.4 to 0.8. The crown and the limbs barely change. Retained pipe accumulates down the stem to the base, and the limbs gain little. **0.5 is kept,** the top of the host's range. S3's massive limbs need a heavier limb exponent or a different term; not tried this round.

**Sheets** (`raw/round5/`, ignored, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `sheet-bare.png` | S1 ×2, S4, S3; round 4 and round 5 at seeds 1, 7, 2, 3 and 4 |
| `sheet-whole.png` | S2 robur and garryana, S5; round 4 and round 5 in leaf |
| `young.png` | 10, 20 and 40 years, seeds 1 and 7, bare and in leaf |
| `close-ups.png` | Trunk bases, limbs and sprays at five seeds, beside S3, S4 and S5 |

**Measures at 80 years** (`run80.log`, `run-young.log`):

| Seed | Height | Width (x × z) | Leaves | Fine wood | Grown in |
|--:|--:|--:|--:|--:|--:|
| 1 | 22.3 m | 26.4 × 22.4 m | 1.51M | 11.2 km | 9.4 s |
| 7 | 22.5 m | 20.8 × 21.4 m | 1.56M | 11.5 km | 11.5 s |
| 2 | 21.4 m | 20.5 × 23.8 m | 2.34M | 17.4 km | 25.0 s |
| 3 | 23.8 m | 21.2 × 19.8 m | 1.95M | 14.4 km | 14.1 s |
| 4 | 22.0 m | 21.3 × 22.8 m | 0.44M | 8.3 km | 7.9 s |

| Age | Height | Width |
|--:|--:|--:|
| 40 | 13.0 m | 9.4 to 10.6 m |
| 20 | 7.0 m | 6.4 to 7.0 m |
| 10 | 3.6 m | 2.5 to 4.4 m |

**About the grow times:** the machine was loaded by other sessions, so they are high and noisy. Seed 2 took 25 s.

### Reading against the references

- **Bare, beside S1:**
  - Seeds 1, 7 and 2 have rounded, closed outlines over a dense web, on a short trunk that divides low into several heavier limbs: nearer S1 than any round before.
  - Seed 3 is vase-shaped, two main arms in a V under a broad top.
  - Seed 4 has two leaning stems.
  - Every trunk is heavier, with a flared base.
- **In leaf, beside S2:**
  - Seeds 1, 7 and 2 are rounded, closed domes.
  - Seed 3 is broad and flat-topped on a long bare trunk, nearer S2 garryana's habit.
  - Seed 4 is still sparse and lopsided: the phase F leaf-marking item.
- **The crowns are narrower than fn-197's step 4c** (20 to 26 m against 21 to 31 m) and 1 to 4 m taller. Heavier wood sags less.
- **Young trees:**
  - At 10 years, a thin sapling with a spire leader and rods (seed 1), or a spreading bush (seed 7).
  - At 20, bushy and ascending.
  - At 40, a dense ovoid.
  - Little changed from round 4: light only acts where a crown shades itself.
- **Close-ups:**
  - The trunk bases are flared cones with the collar band, round 4's known gap. They are not S3's buttressed, furrowed trunk.
  - The limbs are smooth tubes, heavier than round 4's.
  - The sprays carry the preset's feathery leaves along the shoots, not S5's lobed rosettes: a leaf-model item, outside this spec.

**Known gaps:**

- S3's massive limbs: retained thickens the base, not the limbs.
- The collar ring.
- Bark plates.
- The 10-year sapling's spire.
- Seed 4's leaf marking (phase F).
- The leaf form in close-ups.

## Gate, round 5 (host and Astra, 2026-10-05)

**Host:** seeds 1, 7 and 2 read as rounded oaks; seed 3 is a vase on a straight trunk; seed 4 is sparse.

**Astra: FAIL in all three samples** (`ASTRA-VERDICT-R5-1.md` to `-3.md`):

| Seed | Verdicts |
|--:|---|
| 2 | YES in all three |
| 7 | YES in one |
| 1 | BORDERLINE in all three |
| 3 | NO in two |
| 4 | Mostly BORDERLINE |

The fault all three name: "predominantly ascending main limbs: they need more outward reach and irregular changes of direction to produce heavy, tortuous oak architecture."

**Host decisions for round 6:**

1. Limbs leave nearer horizontal.
2. More wander and relay kinks on limbs and boughs.
3. Divide lower and earlier, with a shorter leader.
4. Seed 4's sparseness is phase F's.

## Round 6: outward, tortuous limbs, a lower division (values only), 2026-10-05

**Probes on seeds 3 and 1** (`raw/round6/probeA.png`, `probeB.png`):

- **Probe A** (limb insertion 0.8, elevation 0.55, wander 1.4; bough elevation 0.5; leader 10 years; trunk 9) spread too far. The trees stood 15 to 17.5 m tall and up to 33 m wide, and seed 1's crown went sparse and shrubby.
- **Probe B** sits between round 5 and A, and is kept.

**Kept values** (`oak.rs`):

| Value | Round 5 | Round 6 |
|---|---|---|
| Limb insertion | 0.45 | 0.65 |
| Limb elevation | 0.95 | 0.75 |
| Limb straightening | 0.2 | 0.15 |
| Limb abortion (each relayed) | 0.25 | 0.3 |
| Limb wander | 1.0 | 1.25 |
| Limb and bough epitony | 0.3 | 0.2 |
| Bough elevation | 0.75 | 0.6 |
| Bough wander | 0.8 | 1.0 |
| Bough abortion | 0.25 | 0.3 |
| Leader lifespan | 20 | 14 years |
| Trunk lifespan | 11 | 10 years |

**Sheet:** `raw/round6/sheet.png`, the host's layout: references (S1 ×2, S2 ×2, S3), round 5 in leaf, round 6 in leaf, round 6 bare. I viewed every still.

| Seed | Height | Width | Leaves | Fine wood | Leaves a metre (round 5) |
|--:|--:|--:|--:|--:|--:|
| 1 | 20.5 m | 23.1 × 29.4 m | 0.34M | 11.2 km | 30 (135) |
| 7 | 20.8 m | 25.7 × 21.4 m | 0.81M | 11.8 km | 69 (135) |
| 2 | 17.2 m | 23.8 × 28.3 m | 0.91M | 15.3 km | 60 (135) |
| 3 | 19.4 m | 24.1 × 18.5 m | 1.80M | 13.2 km | 136 (136) |
| 4 | 18.8 m | 22.2 × 23.1 m | 0.38M | 7.7 km | 49 (53) |

### Reading

- **Bare: the limbs now reach out and kink.** At all five seeds a short, heavy trunk divides low into thick limbs that leave near horizontal, then rise and change direction several times under a broad, rounded outline. This is the nearest the round has come to S1's and S3's heavy, tortuous architecture.
  - Seed 3's straight stem is gone; it divides low into spreading limbs.
  - Seed 2 is broad and low.
- **In leaf:** seed 3 is a full, closed dome and seed 7 a rounded one. Seeds 2, 1 and 4 show their limbs through a thinner crown.
- **A new regression: the leaves.**
  - Seeds 1, 7 and 2 carry 30 to 69 leaves a metre of fine wood, against 135 in round 5. Their fine wood is unchanged (11.2 to 15.3 km).
  - The crowns are sparse in leaf for that reason: seed 1 most of all, which now looks like seed 4.
  - Only seed 3 keeps 136.
  - **Not checked:** this is the pipeline's leaf-bearing marking (phase F), which reads twig radius against the trunk's, so the heavier, outward limbs change which twigs it marks. Fine wood is not the cause.
- **Seed 4** is sparse as before (phase F).

**Known gaps:**

- the leaf marking now reaches seeds 1, 7 and 2 (phase F's contract);
- the collar ring;
- bark plates;
- the leaf form in close-ups.

Young trees and close-ups were not re-rendered this round; the host's sheet holds the 80-year trees only.

### Round 6: what causes the leaf collapse (checked 2026-10-05; the host's hypothesis not confirmed)

**The hypothesis tested:** `canopy.shootRadius` is read relative to a stem radius, and retained girth thickened the trunk. A scratch probe (kept at `scratchpad/zz_leafprobe.rs`, not committed) converted seed 1 at 80 years and measured what `Tree::stem_radius` (`telperion-core/src/tree.rs:151`) returns. It also measured how much wood falls under the slender threshold of 0.06 × that radius (`foliage/plan.rs:275`, `placement.rs:85`).

| Seed 1, 80 years | Root radius | Fork found | Stem measure | Slender (leaf-bearing) length | Fine wood |
|---|--:|---|--:|--:|--:|
| Round 5 values | 0.778 m | none | 0.778 m | 11.37 km | 11.16 km |
| Round 6 values | 0.790 m | node 37,916 | **0.027 m** | **2.37 km** | 11.20 km |
| Round 6, χ 0 | 0.706 m | node 37,453 | 0.030 m | 2.23 km | 10.94 km |
| Round 6, retained 0 | 0.513 m | node 38,086 | 0.027 m | 2.34 km | 10.87 km |
| Round 6, both 0 | 0.432 m | node 37,742 | 0.028 m | 2.12 km | 10.46 km |

- **Retained girth is not the cause.** With retained and χ at 0, the root thins to 0.43 m and the leaves stay collapsed.
- **The cause: `stem_radius` finds a fork deep in the crown.**
  - It finds a node where two children both carry the `stem` flag, high in the crown at about 2.7 cm. It measures the canopy against that, not against the trunk.
  - The slender threshold falls about 29-fold, and leaf-bearing length from 11.4 km to 2.4 km.
- **Why round 6 has such a node:**
  - The leader now turns limb at 14 years, not 20, so the trunk's own lineage aborts and relays as limbs do (abortion 0.3, relay 1.0).
  - The conversion (`examples/space/tree.rs`) hands the `stem` flag on to a continuation or relay from its parent's end (`tree.rs:124-126`).
  - **Not checked:** two of them can therefore start at one node, likely a relay whose parent drew no phytomers and so ends where its own parent ended.
- **Status:** the stills have not been changed. The host said to stop if the hypothesis was not confirmed. An absolute radius (decision 30) would also remove this coupling, but it was decided on the other cause.

## Round 6b: round 6's values on the fixed conversion (host decision 31), 2026-10-05

**The fix** (`670dacda`, `examples/space/tree.rs`): the conversion marks an axis as stem only where it carries the trunk on at one of its species' trunk-level ages.

- The trunk-level ages: the oak's young stem, fork and leader; the beech's seedling stem, leader and fork; the spruce's seedling, sapling, trunk and crown leader.
- A leader turned limb is no longer stem, and a limb's relays never are.
- Two trunk-level axes leaving one node, a codominant fork of the trunk, are both stem.

**Where the pipeline reads `stem`:** `Tree::stem_radius` (the canopy's slender-wood measure, short shoots, the local branching seed) and `Tree::stem_apices` (the palm's rosette, branching, the twig-extent suite). The other uses are in the scaffold and leaf-base builders, which tree-space trees do not pass through, and in tests.

**Checks:**

- **Red first:** `tests/space_stems.rs` (render crate) failed on the old conversion, with stems parting at nodes 37,916 and 105,644 of the oak at seed 1 (round 6, light 0.5/0.5). It passes on the new one.
- **The beech and the spruce at 80 years, seed 1, bare and in leaf, are pixel-identical before and after** (ImageMagick absolute-error count 0).
- **The palm is not checked:** its conversion lives on its own branch, which needs its own trunk-level list when the two meet.

**Sheet:** `raw/round6b/sheet.png`, the host's layout: references, round 5 in leaf, round 6b in leaf, round 6b bare. I viewed every still.

| Seed | Height | Width | Leaves | Leaves a metre of fine wood (round 6) |
|--:|--:|--:|--:|--:|
| 1 | 20.5 m | 23.1 × 29.4 m | 1.51M | 135 (30) |
| 7 | 20.8 m | 25.7 × 21.4 m | 1.60M | 136 (69) |
| 2 | 17.2 m | 23.8 × 28.3 m | 2.07M | 135 (60) |
| 3 | 19.4 m | 24.1 × 18.5 m | 1.80M | 136 (136) |
| 4 | 18.8 m | 22.2 × 23.1 m | 1.04M | 136 (49) |

### Reading

- **Every crown is full again.** That includes seed 4, whose sparseness since round 3 had the same cause, not phase F's marking. It now carries 136 leaves a metre, like every other seed.
- **In leaf:**
  - Seed 3 is a full, closed dome, and seed 7 a rounded one.
  - Seeds 1, 2 and 4 are broad, spreading domes, wider than tall, with their heavy limbs showing below the foliage.
  - All five are nearer S2 garryana's spread than any earlier round.
- **Bare:** as round 6. A short, heavy trunk divides low into thick limbs that leave near horizontal, rise and change direction several times, under a broad, rounded outline.
- **Grow times (8.6 to 42 s):** the machine was heavily loaded by other sessions.

**Known gaps:**

- the collar ring;
- bark plates;
- the 10-year sapling's spire (young trees not re-rendered this round);
- the leaf form in close-ups.

**Phase F's note on seed 4's leaf marking (fn-197 decision 26) was this stem-fork measure.** It is fixed here in the conversion, not in the pipeline's marking.

## Gate, round 6b (host and Astra, 2026-10-05)

**Host:** the most oak-like yet.

**Astra: FAIL in all three samples, with no NO** (`ASTRA-VERDICT-R6b-1.md` to `-3.md`):

| Seed | Verdicts |
|--:|---|
| 2 | YES in all three |
| 7 | YES in one |
| The rest | BORDERLINE |

All three name the outer crown: "too many straight, pointed branch sprays replace the sustained, heavy, tortuous boughs and finely ramified rounded crown." The foliage reads as separate pointed lobes with gaps between them.

**Host decisions for round 7** (values only):

1. Boughs and branch systems keep winding to the edge.
2. Finer ramification at the periphery.
3. ψ 1.5 to 2, to fill the gaps.
4. No per-seed fix for seed 3.

## Round 7: the outer crown (values only), 2026-10-05

**Kept values** (`oak.rs`; probe A on seeds 1 and 7, `raw/round7/probeA.png`):

| Value | Round 6b | Round 7 |
|---|---|---|
| Bough tropism | 0.6 | 0.35 |
| Bough elevation | 0.6 | 0.45 |
| Bough abortion (each relayed) | 0.3 | 0.4 |
| Bough wander | 1.0 | 1.3 |
| Branch-system abortion | 0 | 0.2, each relayed (epitony 0.2) |
| Branch-system tropism | 0.4 | 0.25 |
| Branch-system wander | 0.8 | 1.1 |
| Branch-system internode | 0.02 m | 0.017 m |
| Branch-system twig laterals below the top | 0.15 | 0.18 |
| Twig internode | 0.015 m | 0.012 m |
| ψ | 1 | 1.5 |

**The extra twig order was tried and dropped:** twigs bearing twigs at 0.15 in their top cluster refused the 20M-phytomer budget at seed 1. It runs only without it, so finer ramification comes from shorter internodes and more twig laterals on the branch systems.

**Sheet:** `raw/round7/sheet.png`, the host's layout: references, round 6b in leaf, round 7 in leaf, round 7 bare. I viewed every still.

| Seed | Height | Width | Leaves | Fine wood | Leaves a metre |
|--:|--:|--:|--:|--:|--:|
| 1 | 21.3 m | 24.8 × 34.6 m | 1.87M | 13.8 km | 135 |
| 7 | 20.9 m | 27.8 × 27.8 m | 1.95M | 14.3 km | 136 |
| 2 | 19.5 m | 27.7 × 29.1 m | 2.58M | 19.1 km | 135 |
| 3 | 23.8 m | 29.1 × 22.5 m | 2.06M | 15.2 km | 136 |
| 4 | 21.0 m | 23.7 × 23.5 m | 1.26M | 9.3 km | 136 |

### Reading

- **Bare: the boughs wind all the way to the crown's edge.**
  - The straight, pointed sprays of 6b are gone. The outline is a tangle of kinked boughs ending in short twigs, over the same low-dividing heavy limbs.
  - The crowns are wider: seed 3 is 29 m across, against 24 m in 6b, and no longer reads upright.
  - **Possibly overdone:** the boughs now writhe more than S1's and S3's, a busy, contorted web rather than S1's even, fine one.
- **In leaf:**
  - The crowns are rounded and fuller at the edge than 6b. Seed 7 is a broad dome, and seed 2 a broad, low one.
  - The winding boughs show through the foliage at every seed, so the crown still reads partly as foliage clumps on visible wood, not one closed mass.
  - The gaps between lobes are smaller than in 6b, but not gone.
- **Size stays bounded:** 19.5 to 23.8 m tall. The wider crowns carry 20 to 25% more leaves and fine wood than 6b.
- **Finer ramification at the very edge is limited by the budget** (see above).

**Known gaps:**

- the crown edge's fine web, which the budget limits;
- the collar ring;
- bark plates;
- the 10-year sapling's spire;
- the leaf form in close-ups.

## Round 8: the midpoint, with darker limbs (values and colour only), 2026-10-05

**Host on round 7:** overdone. Bare, the boughs writhe into a contorted mass, and in leaf pale winding limbs show through every crown. Round 7 was not sent to Astra.

**Round 8 values** (`oak.rs`):

| Value | Round 6b | Round 7 | Round 8 |
|---|---|---|---|
| Bough tropism | 0.6 | 0.35 | 0.48 |
| Bough wander | 1.0 | 1.3 | 1.15 |
| Bough abortion (each relayed) | 0.3 | 0.4 | 0.35 |
| Branch-system tropism | 0.4 | 0.25 | 0.33 |
| Branch-system wander | 0.8 | 1.1 | 0.95 |
| Branch-system abortion (each relayed) | 0 | 0.2 | 0.1 |

- **Kept from round 7:** the shorter internodes and extra twig laterals, ψ 1.5, and bough elevation 0.45.

**Colour** (`presets/oregon-white-oak.values`; host decision, colour only; catalogue identity re-pinned in `crates/telperion-core/tests/catalogue_identity.rs`):

| Row | Round 4 | Round 8 |
|---|---|---|
| Bark | 0.215 / 0.154 / 0.112 | 0.17 / 0.138 / 0.105 (greyer and darker) |
| Young wood | 0.08 / 0.066 / 0.05 | 0.075 / 0.066 / 0.055 |
| Young wood reaches up to, then blends to the bark by | 30 mm, 60 mm | 50 mm, 100 mm |

The aim is limbs in sun toward S3's grey-brown, not pale pink-tan.

**Sheet:** `raw/round8/sheet.png` (references, round 6b in leaf, round 8 in leaf, round 8 bare); probe on seeds 1 and 7 beside 6b and 7 in `probe.png`. I viewed every still.

| Seed | Height | Width | Leaves | Fine wood |
|--:|--:|--:|--:|--:|
| 1 | 21.2 m | 26.1 × 35.4 m | 1.88M | 13.9 km |
| 7 | 21.1 m | 27.3 × 27.9 m | 1.97M | 14.5 km |
| 2 | 19.3 m | 30.2 × 29.2 m | 2.59M | 19.1 km |
| 3 | 23.5 m | 30.2 × 22.5 m | 2.10M | 15.5 km |
| 4 | 21.1 m | 23.9 × 24.0 m | 1.28M | 9.4 km |

### Reading

- **Bare:**
  - The limbs and boughs now read grey-brown, much nearer S3 than 6b's pale tan. The fine web is dark against the sky.
  - The boughs bend several times and end in short twigs: calmer than round 7's coils, but still busier than 6b.
  - The crown edge is a denser, finer web than 6b's sprays.
- **In leaf:**
  - Rounded crowns: seed 7 and seed 2 broad domes, seed 3 a rounded crown, seeds 1 and 4 broad and spreading.
  - The limbs that show through the foliage are dark now, so they read as shadowed wood rather than pale streaks.
  - Some gaps between foliage lobes remain at seeds 1 and 4.
- **The trunk base keeps the flared cone with its collar band,** the known rendering gap, left as directed.

## Gate, round 8 (host and Astra, 2026-10-05)

**Host:** the colour is better and the edge finer.

**Astra: FAIL in all three samples** (`ASTRA-VERDICT-R8-1.md` to `-3.md`):

| Seed | Majority verdict |
|--:|---|
| 1 | YES |
| 2 | YES |
| 7 | BORDERLINE |
| 4 | BORDERLINE |
| 3 | NO |

All three name the same fault: "too many prominent ascending tips ... angular, upswept fans and flat-topped bowls instead of a broad, rounded envelope". Seed 3 stays narrow and upright.

**Host and Astra have now iterated three gates (rounds 5, 6b and 8) without agreeing.** Under the owner's rule (docs/tree-space.md, "How a species is judged"), the oak goes to the owner's verdict alongside the spruce. Round 9 continues in the meantime.

## Round 9: tips that settle outward (values only), 2026-10-05

**Probes on seeds 3 and 7** (`raw/round9/probe.png`, `probeB.png`):

- **Probe A** (limbs 0.75 / 0.65; bough tropism 0.35, elevation 0.25; branch-system tropism 0.25, elevation 0.05) changed little.
- **Probe B**, the stronger one, is kept.

**Kept values** (`oak.rs`):

| Value | Round 8 | Round 9 |
|---|---|---|
| Limb insertion | 0.65 | 0.8 |
| Limb elevation | 0.75 | 0.6 |
| Bough tropism | 0.48 | 0.25 |
| Bough elevation | 0.45 | 0.1 |
| Branch-system tropism | 0.33 | 0.15 |
| Branch-system elevation | 0.2 | 0 |

Round 8's colours, fine twigs and ψ 1.5 are kept.

**Sheet:** `raw/round9/sheet.png`: references, round 8 in leaf, round 9 in leaf, round 9 bare. I viewed every still.

| Seed | Height | Width | Leaves | Fine wood |
|--:|--:|--:|--:|--:|
| 1 | 20.0 m | 27.5 × 36.1 m | 1.87M | 13.8 km |
| 7 | 20.3 m | 30.6 × 27.4 m | 1.93M | 14.3 km |
| 2 | 18.5 m | 30.5 × 29.1 m | 2.60M | 19.3 km |
| 3 | 21.4 m | 31.6 × 24.9 m | 2.10M | 15.5 km |
| 4 | 19.9 m | 23.6 × 25.4 m | 1.28M | 9.4 km |

### Reading

**Not clearly better than round 8.**

- **What the tips did:** the boughs' tips settle outward and the crowns are lower and broader, 1 to 1.5 m lower and up to 6 m wider than round 8. Seed 3 is 31.6 m across against 30.2.
  - The outlines spread flatter rather than rounding over: seed 1 is a wide, star-shaped spread, and seed 7 a broad, flat-topped mass.
  - The lowest boughs now sweep down toward the ground at seeds 1 and 2.
- **Spikes remain:** a few ascending tips still stand out of the crown at seeds 3 and 4.
  - These are the long shoots' and twigs' own pull (their tropism toward 0.2), which this round did not change.
- **Bare:** the scaffold is lower and more spreading, the web as fine as round 8's, the colour unchanged.
- **For the owner's look, round 8 is the stronger sheet:** its crowns are rounder and fuller. Round 9's are broader and flatter, with lower-hanging boughs.

### Round 9, host reading (2026-10-05): round 8 kept

**Host:** flatter and shrubbier, with drooping lower boughs. Round 8 stays the candidate, and `oak.rs` is restored to its values.

**The next lever** for the spikes still ascending out of the crown at seeds 3 and 4: the long shoots' and twigs' own upward pull (tropism 0.8 and 0.4 toward elevation 0.2). Rounds 8 and 9 left those values unchanged.

**The trunk-base bell and collar** (a known rendering gap, not fixed):

- **The bell:** most likely the pipeline surface stage's basal flare (`flareRadius` 2.1 × radius over `flareFalloff` 0.022 of the height, defaults in `telperion-core/src/pipeline/surface.rs:145-212`; the oak's preset sets no flare row). It is applied to a trunk that the retained pipes have already thickened.
- **The collar bands:** probably the bark's plate texture (`plateScale` 0.055) seen across the flare. Not checked.

## Owner's verdict on round 8 (2026-10-05)

> "structurally ok but very blob like and unnatural in subtle ways. Branches are a bit chaotic ... the reference looks calmer and more natural."

**Not a pass.**

**Host diagnosis, checked:** `form.wander` draws an independent turn at every node (`geometry/lay.rs:83-90`), so a branch's direction is a memoryless random walk. Wander can only jitter, never make a slow bend. The rounds that raised it to make limbs "tortuous" made every order chaotic instead.

The fix is its own spec, fn-207 (wander bends over a length), with this oak as its R4 trial.

## fn-207 R4 trial: slow bends on the limbs, calm fine wood (on fn-207's engine), 2026-10-05

**Values against round 8** (`oak.rs`):

| Value | Round 8 | Trial |
|---|---|---|
| Leader wander | 0.6 | 0.5 |
| Limb wander | 1.25 | 0.5 |
| Leader and limb bend length | 0 | 3 m |
| Limb abortion (each relayed) | 0.3 | 0.15: the big sympodial turns only |
| Bough wander | 1.15 | 0.35 |
| Bough bend length | 0 | 1.5 m |
| Bough abortion (each relayed) | 0.35 | 0.1 |
| Branch-system abortion | 0.1 | 0 |
| Wander of branch systems, sprigs, twigs and shoots | 0.95 / 1.0 / 1.2 / 1.5 | 0.4 each |

Everything else is round 8's: the colours, the fine twigs, ψ 1.5 and the light values.

**Sheet:** `raw/fn207/sheet.png` (references, round 8 in leaf, the trial in leaf, the trial bare); probe on seeds 1 and 7 in `raw/fn207/probe.png`. I viewed every still.

| Seed | Height | Width | Leaves |
|--:|--:|--:|--:|
| 1 | 19.7 m | 27.8 × 37.0 m | 1.92M |
| 7 | 18.8 m | 27.0 × 29.1 m | 1.97M |
| 2 | 19.4 m | 31.0 × 31.7 m | 2.60M |
| 3 | 20.0 m | 32.5 × 24.0 m | 2.07M |
| 4 | 19.4 m | 26.8 × 26.5 m | 1.27M |

The sheet and this table were rendered again on the final engine, after Codex review 1's fix: the fork, at bend length 0, now hands its last turn on to the leader. I viewed it again: the same reading, with differences only within a seed.

### Reading

- **Bare: calmer.**
  - The limbs leave low and make a few long, smooth arcs instead of round 8's repeated kinks.
  - The boughs and the fine wood are even, a regular web with no coiling.
  - The chaos the owner named is much reduced.
- **The crowns are broader and flatter than round 8's:** 1 to 2 m lower and wider. Seed 7 is an umbrella with a flat top. Seed 3 spreads wide on long limbs that show through the foliage.
- **In leaf:**
  - Seed 2 is a rounded dome; seeds 7 and 1 broad, flat-topped domes; seeds 3 and 4 open spreads with their limbs visible.
  - The foliage reads as less chaotic but still lobed.
- **The flatter tops are likely the calmer limbs not rising after leaving low** (round 8's elevation 0.75 now acts on straighter limbs). Not checked: a value for the oak's next round.

## Round 10: limbs rise again and the crown grows taller (host, 2026-10-05)

Built on the fn-207 trial's bends and calm fine wood. The values against the trial:

| Value | fn-207 trial | Round 10 | Note |
|---|---|---|---|
| Trunk lifespan | 10 | 12 | The first fork sits higher |
| Leader lifespan | 14 | 19 | Round 5 had 20 |
| Limb insertion | 0.65 | 0.5 | Round 5 had 0.45 |
| Limb elevation | 0.75 | 1.05 | Round 5 had 0.95 |
| Bough elevation | 0.45 | 0.5 | |

The limb and leader bends stay at 3 m and the bough bends at 1.5 m.

**Probes on seeds 1 and 7:**
- **Probe 1** (`raw/round10/probe.png`): trunk 11 and limb elevation 0.95. The trees were taller (23.4 and 21.4 m) but still too wide, at 1.6 and 1.4 times the height.
- **Probe 2** (`raw/round10/probe2.png`): trunk 12 and limb elevation 1.05. These are the values kept.

**Sheet:** `raw/round10/sheet.png`, with the references, the fn-207 trial in leaf, round 10 in leaf and round 10 bare. I viewed every still.

| Seed | Height | Width | Width / height |
|--:|--:|--:|--:|
| 1 | 24.8 m | 25.3 × 33.6 m | 1.0–1.35 |
| 7 | 22.3 m | 26.0 × 24.7 m | 1.1–1.15 |
| 2 | 23.4 m | 26.7 × 27.8 m | 1.15–1.2 |
| 3 | 23.9 m | 27.6 × 27.6 m | 1.15 |
| 4 | 22.5 m | 25.1 × 25.6 m | 1.1–1.15 |

### Reading

**Against the targets:** all five seeds are 22 to 25 m tall, with widths of about 1.1 to 1.2 times the height. The clear trunk is about a quarter of the height. Limbs leave at roughly 40 to 60 degrees and rise in long, smooth arcs.

**Against S1, the limbs rise but do not arch over at the top:**
- The crowns read as a vase or wineglass, broadest high up.
- Seed 1 is a V with two limbs reaching out above the crown.
- Seed 2 comes closest to a dome.
- Seeds 3 and 4 have a tuft or bough sticking above the outline.

**Still lobed:** the foliage is clumped into lobes, though calmer than round 8.

## Round 11: from vase toward dome, and finer outer wood (host, 2026-10-05)

### Values against round 10

| Value | Round 10 | Round 11 | What it does |
|---|---|---|---|
| Limb lifespan | 1,000 | 20 | Then the limb carries on as **ARCH** (below) |
| ARCH (new PA) | none | the limb's values, elevation 0.6 | Older limbs bend their new wood towards a lower elevation, so the crown's outer wood arches over (the host's "lower elevation target on the older limb ages") |
| Limb branch-system laterals | 0.45 | 0.55 | More division along limbs |
| Bough branch-system laterals | 0.3 | 0.4 | More division along boughs |
| Bough lifespan | 60 | 45 | A younger outer crown |
| Branch-system lifespan | 12 | 10 | A younger outer crown |

- ARCH sits between LIMB and BOUGH in PA order, because the engine refuses a lateral younger than its bearer. Its own limb laterals are ARCH. Every later PA's index moves up by one.
- Sag and dominance are as in round 10.

### Probes on seeds 1 and 2 (`raw/round11/probe1.png` to `probe7.png`)

1. **Probe 1, all the host's levers at once:**
   - Changes: limb sag 0.0008, bough sag 0.0002, dominance 0.3, more laterals, bough lifespan 35, branch lifespan 8.
   - Result: narrow, 18 to 19 m wide (0.85 to 1.0 times the height). Leaves fell from 1.74M to 1.30M. Seed 1 was still a V.
2. **Probe 2, lifespans restored, limb sag 0.0015:** seed 2 narrowed to a 20 m cup, and seed 1 grew a hooked limb.
3. **Probe 3, sag and dominance only:** lost vigour (1.15M leaves; seed 1 at 17 m). Dominance shrinks most sibling limbs.
4. **Probe 4, sag 0.0015 alone:** whole limbs hinge down from the base, where the moment is largest. The crowns slump to 34 to 36 m wide, seed 2 into a bush, and the clear trunk is lost. **Sag does not arch the tips.**
5. **Probe 5, older limb from age 12 at elevation 0.3, sag 0.0005:** seed 1's V is gone and its top rounds, but the crown is low (19 m) and the limbs hang to the ground.
6. **Probe 6, older limb from age 20 at elevation 0.6, round 10's sag:** close to a dome on seed 2.
7. **Probe 7, probe 6 plus the moderate fine-wood steps:** kept.

### Five seeds (`raw/round11/sheet.png`)

The sheet shows the references, round 10 in leaf, round 11 in leaf and round 11 bare. I viewed every still.

| Seed | Height | Width | Leaves |
|--:|--:|--:|--:|
| 1 | 24.0 m | 27.8 × 27.6 m | 1.58M |
| 7 | 21.3 m | 24.9 × 25.4 m | 1.78M |
| 2 | 21.5 m | 27.0 × 29.0 m | 2.77M |
| 3 | 21.8 m | 34.8 × 29.7 m | 2.71M |
| 4 | 20.1 m | 24.2 × 26.8 m | 1.27M |

### Reading

**The tops now arch over instead of rising into a vase.** Seed 2 is the nearest to a dome.

**The arch overshoots on several seeds:**
- Seeds 3 and 4 are an umbrella: a spreading cap over a bare lower crown.
- Seed 7 leans into a broad, flat-topped spread.
- The trees are about 1 to 3 m lower than round 10, and seed 3 is 1.6 times as wide as it is tall.

**Seed 1's lone rising limb remains.** It ends in a hook, probably the leader carrying on as a limb and then as an old limb. This is not checked.

**Item 2 (finer outer wood) is not visibly achieved:** heavy wood still reaches the outline and ends in tufts. Pushing the laterals or shorter lifespans further narrowed the crown and cost leaves (probes 1 and 2). This is likely the light sharing among more siblings, not checked.

### Open for the host

- **ARCH's age and elevation** are the levers for item 1. Probes 5 and 6 bracket them; a value between, such as age 20 at elevation 0.7 to 0.8, may give a dome without the umbrella.
- **Item 2 may need the dividing to come with less taper of vigour per lateral.** That is a question about how sizes share among siblings, which I leave to the host.

## Round 11 reverted; round 10 restored (host reading, 2026-10-05)

The host's reading: the arching tops overshoot into umbrellas, the outer wood still ends in tufts, and **round 10 remains the best oak**. The values in `oak.rs` are round 10's again (`54b9dff8`), and the ARCH PA has been removed.

### Round 11, kept on record as tried

| Probe | Values | Result |
|---|---|---|
| ARCH (the older limb) | After 20 years limbs carried on as a PA with elevation 0.6 | Tops arched over, but overshot to umbrellas |
| 1 | Limb sag 0.0008, bough sag 0.0002, dominance 0.3, more laterals, bough lifespan 35, branch lifespan 8 | Crown narrowed and lost leaves |
| 2 | Probe 1's laterals, lifespans restored, limb sag 0.0015 | Narrow cup on seed 2; a hooked limb on seed 1 |
| 3 | Sag and dominance only | Dominance cost vigour |
| 4 | Sag 0.0015 alone | Whole limbs hinged down from the base |
| 5 | ARCH from age 12, elevation 0.3 | Low crown; limbs hung to the ground |
| 6 | ARCH from age 20, elevation 0.6 | Near-dome on seed 2 |
| 7 | Probe 6 plus moderate division and lifespans | Kept as round 11 |

Values and sheets are in "Round 11" above and in `raw/round11/`.

### Host diagnosis

The remaining gap to S1 is the **density of the fine twig web**. It is bounded by the growth and geometry budget: the same wall as the spruce's curtains.

Round 11's probes showed that more division within that budget narrows the crown or loses leaves. The owner is asked whether to accept round 10 structurally, with the fine-web density as its main gap for phase F.

## Owner's verdict (2026-10-05): round 10 passes

> "yea i agree the oak needs more fine twigs. If that's a performance thing to be fixed in [phase F] then let's accept it for now."

The oak is accepted at round 10 (`54b9dff8`, restored in `f3384827`), on fn-197's light and carbon engine and fn-207's bends.

### Known gaps

1. **The density of the fine twig web.** S1's limbs keep dividing into an ever-finer web out to the outline; ours carries heavy wood to the edge, ending in tufts.
   - The bound is the growth and geometry budget. Round 11 showed that more division within it narrows the crown or loses leaves.
   - Phase F's levers: shedding before growth, so the budget is spent on living wood, and drawing fine twigs cheaply.
2. **The trunk-base flare and collar band.** The surface stage's basal flare (`flareRadius` 2.1, `flareFalloff` 0.022) gives a bell, and a band shows at the collar (round 9 note).
3. **Bark plates** (host). The bands on the bole are probably the bark's plate texture (round 9 note).
4. **The 10-year sapling's spire** (host).
5. **Leaf form** (host).

## Requirements

| R | Status |
|---|---|
| **R1** (the oak's values, five-seed sheets) | Met: round 10, `raw/round10/sheet.png`. |
| **R2** (the gate) | The workspace gate on the branch head and Codex review of the oak's diff: see Close below. |
| **R3** (the spruce-to-oak walk) | Split into task `.2`, blocked by fn-206 (one reference axis): the spruce and the oak are written on different physiological-age chains, so they have no per-setting midpoint. As fn-194 and fn-196 did. |
| **R4** (passed species unchanged in look) | See below. |

**R4 evidence:**
- **Beech and spruce:** unchanged.
  - fn-197's neutral hashes prove it: every new light and carbon setting is neutral at 0, with 80-year hashes identical before and after (fn-197 CLOSE.md, `tests/light.rs`).
  - fn-207's R1 proves it again: beech and spruce 80-year hashes at seeds 1 and 7 identical to `97dc2233`'s.
  - The oak's own commits change only `oak.rs` and the render example's stem flag for trunk-level PAs, which the beech and spruce pass as their own trunk lists.
- **The palm:** cannot be verified on this branch, because the palm is not in it. It falls to the branch that carries both.

## Close (2026-10-05)

- **Workspace gate** on the working branch's head (`155a596a`, fn-197 + fn-207 + the oak at round 10): 1,063 passed, 0 failed, 444 s.
- **Split into three stacked branches from origin/master:** E (`fn-197-…-pr`), fn-207 (`fn-207-…-pr`) and the oak (`fn-195-…-pr`). On each:
  - the space crate's tests pass;
  - `cargo check -p telperion-render --examples` is clean.
- **On this branch also:**
  - `space_stems`, `catalogue_identity` and `material_detail` pass;
  - the palm example takes its trunk list (`PALM_TRUNK`, its one stem) for the trunk-level stem flag.
- **Codex review** of the oak's own diff (base: fn-207's branch): SHIP in round 1, no findings. Codex reads R4 as met by the recorded pixel-identical beech and spruce renders across the conversion change.
