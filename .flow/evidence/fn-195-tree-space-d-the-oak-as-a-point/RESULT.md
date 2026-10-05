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
