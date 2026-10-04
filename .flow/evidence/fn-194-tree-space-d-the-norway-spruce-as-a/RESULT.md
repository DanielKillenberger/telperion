# fn-194: the Norway spruce as a point (worker)

## Round 1 (2026-10-04)

**Values only, no engine change.** `crates/telperion-space/src/spruce.rs` places Massart's model on today's engine, using six physiological ages (PAs):

- **Seedling:** five bare years.
- **Trunk:** 40 years at about 0.35 m a year.
- **Crown leader:** after that, about 0.25 m a year.
- **Main branch, branchlet, shoot.**

How the model's rules map onto the engine:

| Rule | How the engine draws it |
|---|---|
| M1, M2: the trunk | Erect, monopodial, the golden angle |
| M3: the whorls | A whorl of 5 buds (`Zone::buds`), each at p 0.85, at the top of each yearly unit |
| M9: the medial category | Interwhorl branches at p 0.2 |
| M5, M7: plagiotropic branches | The trunk's `plane` π/2 lays the branchlets left and right in the branch's plane |
| M6: the tip turns up | The branch inserts near horizontal (1.5 rad) and bends towards 0.55 rad |
| M12: comb curtains | Branchlets bend to −0.85 rad |
| M11: natural pruning | Branchlets live up to 25 years, viability 0.98, and are shed when they die |

**Stills:**
- `crates/telperion-render/examples/space_spruce.rs` draws them. The beech's runner moved into `examples/space/still.rs` unchanged and both examples share it.
- The stills overlay rows on the norway-spruce preset:
  - no clusters and no clumping;
  - one needle every 2.5 mm, the preset's own spacing;
  - darker, render-matched colours.

**Colours, matched by render as the beech was** (mean linear colour):

| Target | Photograph | Render |
|---|---|---|
| Bark: S1's trunk base (non-green pixels) | 0.070 / 0.065 / 0.042 | 0.071 / 0.066 / 0.042 |
| Needles: a block of S1's lower crown, against the needle pixels of the 80-year render | 0.053 / 0.073 / 0.051 | 0.049 / 0.067 / 0.043 |

The colours sit in the example's `ROWS` and the preset is untouched. Moving them into the preset would change today's spruce and re-pin the catalogue identity, as the beech's change did; that is the host's call.

**Sheets** (`raw/round1/`, ignored, on disk). I viewed every still.

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | S1, S2, S3 and today's spruce (seeds 1 and 7, `raw/today/`), above the 80-year trees in leaf and bare at seeds 1, 7, 2, 3, 4 |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7, in leaf and bare |
| `close-ups.png` | Trunk bases and limbs, every age and seed |

Measures are in `raw/round1/run.log`:

| Age | Height | Width | Grown in | Kept nodes |
|--:|--:|--:|--:|--:|
| 80 | 22.1 to 22.9 m | 11.2 to 11.7 m (w/h about 0.52) | 3.6 to 4.7 s | 1.69M to 1.76M |
| 40 | 12.1 to 12.3 m | 5.4 to 5.6 m | | |
| 20 | 5.4 to 5.7 m | 2.2 to 2.3 m | | |
| 10 | 2.1 to 2.3 m | 0.5 to 0.6 m | | |

At 80 years the needles number 12.2M to 12.7M, against 7.4M for today's spruce.

### Reading, trait by trait (MODEL-SPRUCE.md's traits)

| Trait | Reading | Spruce? |
|---|---|---|
| 1. One straight leader, never forked | All ages and seeds; a sharp spire at the top | **Yes** |
| 2. Narrow cone, widest at the base, live crown near the ground | A clean cone, about 0.52 w/h, foliage to about 0.5 m. S1 is broader (about 0.75) and its lower crown spreads wider | **Yes, narrower than S1** |
| 3. Readable whorled tiers | Clear at 20 and 40 years. At 80 years they read only bare; in leaf they merge into a uniform cone | **Yes young; weak mature** |
| 4. Branches near-horizontal or drooping, tips upturned | Branches leave near horizontal and curve up gently (limb close-ups). The lower crown does **not** droop: there are no sagging lower limbs as in S1 | **Partly** |
| 5. Comb curtains below each branch | Fine hanging fringes under every branch in the limb close-ups. From the whole-tree distance they read as a fine haze, not S1's heavy dark curtains | **Partly** |
| Mass and colour | In leaf: a dark, fairly dense, teal-green cone, much denser and darker than today's spruce. S1 is yellower-green and blotchier | Close |
| Seed variety | The five seeds are nearly identical: one regular "Christmas tree". S1 and S2 are irregular | **No** |
| 10 years | A thick tapering pole with five tiny whorls: **fails**. The five bare seedling years and short young branches leave no sapling crown, and the girth is far too heavy for a 2 m tree | **No** |

Overall, from 20 years up it reads as a conifer of the spruce/fir kind: a single leader, whorled tiers and a conical crown. Beside S1 it is too regular, too narrow and too even across seeds, it has no drooping lower limbs, and its curtains are faint. The 10-year tree fails.

### Engine gaps against MODEL-SPRUCE.md's F1 to F6 (for the host; not designed here)

- **F1, the trunk ageing like a branch:** gone. The PA is fixed per axis.
- **F2, tropism past horizontal:** available. `form.elevation` reaches −π/2 and the curtains use it.
- **F3, sag of older wood with the tip still rising: not expressible.** Insertion and elevation are fixed per PA at birth, so a branch cannot droop more as it ages or carries load. The lower crown therefore cannot droop the way S1's does without young branches drooping too.
- **F4, delayed release of dormant buds (draperies, M10): not expressible.** Inner branches go bare once their branchlets are shed, so only the branchlets' 25-year lifespan keeps them green.
- **F5, rhythm establishing with age:** expressible through the seedling PA's `rhythm`, though the seedling is bare this round (see F6).
- **F6, the ground: binding.** The comb's curtains on the lowest branches reached below z = 0 five times (`BelowGround`). Values only kept them off the ground: a seedling with no laterals (the first branches at about 0.5 m), branches lifted (insertion 1.5, elevation 0.55, tropism 0.6), curtains held at −0.85 and branchlets shortened. That same bare seedling is why the 10-year tree fails. Whether wood may rest on the ground, or what lifts the lowest limbs, is the host's question.
- **Not yet expressed:** seed-to-seed irregularity (a values question, such as `dominance` or more wander and roll on branches), and girth on young trunks (the trunk's pipe is 0.006).

Not run, as directed: R3 (the beech-to-spruce walk), R4 (the beech's sheet re-rendered), Astra, review, the workspace gate.

## Host answers to round 1 (2026-10-04)

| Question | Answer |
|---|---|
| Ground and droop (Q1, Q2) | Built as fn-200 (`form.sag` and ground support); this branch is rebased onto it |
| Dormant buds (Q3) | Not now; recorded as a known gap |
| Habit (Q4) | Comb, as S1 shows |
| Colours (Q5) | Moved into the norway-spruce preset |
| Variety, girth, sapling, width (Q6) | Go ahead: more unevenness between seeds and branches, a thinner young trunk, sapling branches from year 2 or 3, a crown widened toward S1's 0.75 |

## Round 2 (worker)

**Values only** (`crates/telperion-space/src/spruce.rs`):

- **Colours:** in `crates/telperion-core/presets/norway-spruce.values` with their matching notes. They are gone from the example's rows. `catalogue_identity`'s three digests are re-pinned; this changes today's spruce colours.
- **Sapling:**
  - a two-year unbranched seedling, then a new sapling PA: 8 years, shoots of about 0.2 m a year, whorls from year 3, rhythm 0.7 while it establishes (M4);
  - the trunk PA follows for 35 years, then the crown leader.
  - The ground now supports the curtains, so they no longer need lifting.
- **Thinner trunk:** the trunk's pipe is 0.0025 (it was 0.006) and its wander 0.1.
- **Young branches:** a new PA for a branch's first six years, with longer yearly shoots (internode 0.05), so young tiers spread and the crown widens.
- **Main branches:**
  - Sag 7.5e-5 (fn-200), taken from the R5 strip where width peaks.
  - Insertion 1.45, wander 0.8, roll 0.5, dominance 0.25.
  - Viability 0.998 with shedding after 3 years, so a few branches die and leave gaps.
  - Two branchlets between the whorl pairs (one before).
- **Interwhorl branches:** probability 0.3 (0.2 before).

**Sheets** (`raw/round2/`, on disk). I viewed every still. The layout is as in round 1; the reference row holds round 1's seed 1 in place of today's seed 7.

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, then the 80-year trees in leaf and bare at seeds 1, 7, 2, 3, 4 |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases and limbs, every age and seed |

Measures are in `run.log`:

| Age | Height | Width | w/h | Grown in | Kept nodes |
|--:|--:|--:|--:|--:|--:|
| 80 | 21.9 to 22.2 m | 12.9 to 16.1 m | 0.59 to 0.73 (S1 about 0.75; round 1 0.52) | 5.6 to 7.6 s | 2.16M to 2.31M |
| 40 | 11.8 to 11.9 m | 6.5 to 8.0 m | | | |
| 20 | 5.2 m | 3.2 to 4.1 m | | | |
| 10 | 1.7 to 1.9 m | 1.2 to 1.5 m | | | |

### Reading against round 1's short list

| Trait | Round 2 | Better? |
|---|---|---|
| Width | A broader cone at every seed: 0.59 to 0.73 of height. Seed 4 is the narrowest | **Yes**, near S1 at seeds 1, 2, 3 |
| Lower crown droop | Lower branches sweep down from the trunk and their outer parts lie on the ground as a skirt. Tips still rise (limb close-ups). The upper crown stays near horizontal and ascending, as in S1 | **Yes** |
| Seeds alike | The crowns now differ: seed 1 lopsided, seed 4 narrow and tall-shouldered, seeds 2 and 3 broad, with gaps and uneven tiers. Still one family, more alike than S1 and S2 are | **Partly** |
| 10 years | A sapling with spreading, uneven tiers, no longer a pole. Needles are sparse, and the last two yearly shoots stand bare above the top whorl | **Yes, borderline** |
| 20 and 40 years | Young spruces with clear tiers and a full lower crown | Yes |
| Mass | The upper crown is dense and dark. The lower crown is airy: a see-through skirt, where S1's lower crown is a dark curtain | No change |

### Open, for the host

- **Bare inner lower branches.** In the trunk-base close-ups, the lowest branches' inner lengths are bare wood: their branchlets have died and been shed. They sweep down and lie on the ground, and where the rest begins each one bends at an angle, so the base reads as a ring of bare tentacles. Two things cause it: no draperies (F4, recorded as a known gap), and the ground support's rest. A rest is a corner, not a curve, at the internode where wood meets the ground.
- **The airy lower crown**, against S1's dark curtains. This is likely the same missing draperies; denser or longer-lived branchlets are a values option.

Known gap: draperies, the delayed release of dormant buds (F4; host, 2026-10-04).

## Host round 3 directions (2026-10-04)

The main defect at 80 years: the base reads as bare tentacles on the ground, and the lower crown is see-through where S1's is a dark curtain.

- **Ground contact (host design):** a curve, not a corner, built in fn-200's geometry as a follow-up commit on this branch.
- **Density (host decision):** values first. If they cannot fill the inner lengths, stop and report; draperies then become their own gap spec.

## Round 3 (worker)

### Ground contact (`b3ef700c`, `crates/telperion-space/src/geometry.rs`)

**The contact zone:** four of the wood's radii plus six internodes above the ground.
- Within it, non-trunk wood's downward pitch eases with the square root of its height, so the wood lands tangent and runs along the ground. The hard rest stays as a backstop.
- Outside the zone nothing changes; the beech's stills are byte-identical (`raw/beech-r3/`).
- **Why the six internodes:** a zone of radii alone is thinner than one internode on coarse wood, so a step jumped across it and the corner came back (1.49 rad, then 0.81 at two internodes). The landing turn has to be spread over several steps.

**Two changes to fn-200's sag were needed on the way:**
1. **Sag bends an axis down in its own vertical plane**, by the magnitude of its moment. Before, it turned about the carried torque's axis. That axis is ill-defined when an axis has turned almost opposite to its pre-sag direction, and the landing walk showed it as a jump.
2. **Sag stops 0.15 rad short of straight down.** Otherwise an axis bent exactly vertical leaves tropism no side to turn to, and the side flips. That vertical-direction singularity is pre-existing, in `toward_elevation`.

**Girth before placement:** girth is now computed before placement, so the contact zone knows each radius on the first lay. A relay's node is now set with the scales, because girth reads it.

**Tests (`tests/sag.rs`):**
- `wood_lands_on_the_ground_without_a_corner`: was red on the corner version (1.49 rad), now green; its threshold is 0.35 rad.
- `landing_on_the_ground_walks_by_degree`: walks limb sag onto the ground and checks slope and refinement.
- The walk's limbs bear twigs, not limbs. A limb inserted square off a level limb can stand exactly straight down at sag 0, where the same pre-existing tropism singularity is a jump of its own (recorded for the host).
- All crate tests pass; the walk's worst sag slopes are 0.00, 3.47 and 0.07.

### Density by values (`spruce.rs`)

- **Long-lived spurs:** a branchlet now grows 10 years, then becomes a new SPUR PA. A spur barely lengthens (6 mm a year), lives 45 years at viability 0.99, and makes short shoots every year. Branchlets therefore stand along about 55 years of each main branch instead of 25.
- **More branchlets:** branches carry paired branchlets at their two medial nodes (one each before).
- **Sag 5e-5** (7.5e-5 before), so fewer of the lowest branches lie flat.
- **Needle spacing:** the stills' needle spacing is 3.5 mm (2.5 before), to stay under the renderer's dispatch limit (FRICTION.md).

**Spur lifespans tried at seed 1:**

| Spur lifespan | Grown in | Kept nodes | Result |
|--:|--:|--:|---|
| 30 years | 8.6 s | 2.9M | Inner lengths still bare |
| 70 years | 9.5 s | 3.8M | Inner lengths filled with hanging branchlets, but the crown no denser from the whole-tree view; first crashed the renderer at 2.5 mm spacing |
| 45 years (kept) | | | |

### Sheets (`raw/round3/`, on disk; I viewed every still)

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | S1, S2, S3, round 2's seed 1 and round 2's seed 1 base above the five 80-year trees in leaf and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases and limbs, every age and seed |

Measures are in `run.log`:

| Age | Height | Width | Grown in | Kept nodes |
|--:|--:|--:|--:|--:|
| 80 | 21.9 to 22.2 m | 13.7 to 15.9 m (w/h 0.63 to 0.73) | 7.5 to 9.1 s (three seeds over 8 s) | 3.3M to 3.6M |

Younger ages are as in round 2.

### Reading

| Trait | Round 3 | Met? |
|---|---|---|
| Ground contact | The lower branches now arch down and land in smooth curves with no corner. The tentacle ring is gone: inner lengths near the trunk carry hanging branchlets, and fewer branches lie flat | **Yes** |
| Seed 4's base | A dark vertical band runs down the trunk (a branch hanging against it, or a shading artefact; not chased) | Note |
| Lower crown density, whole-tree view | Somewhat fuller than round 2, but still see-through: thin hanging lines over a light underlayer, never S1's dark curtain. The bare-wood view shows the reason: the comb's curtains are long thin branchlets with sparse foliage, not dense hanging sprays | **No** |
| Width, droop, seed variety, 10 to 40 years | As in round 2; the young trees are unchanged in look | Kept |
| Cost | 7.5 to 9.1 s at 80 years, against round 2's 5.6 to 7.6 s, past the 8 s guide at three seeds | Over |

**Values do not reach S1's dark lower curtain.** Longer-lived branchlets fill the inner lengths but read as strings: each old branchlet adds only a few short shoots a year. Going further (70-year spurs) costs 9.5 s and over the renderer's instance limit without darkening the whole-tree view. As the host directed, I stop here. Dense lower curtains look like draperies: a branchlet's dormant buds releasing new sprays along its length (M10, F4). That is a gap spec for the host to design.


## Host decisions after round 3 (2026-10-04)

- **Round 3's density values are kept.** Up to about 9 s for an 80-year tree is accepted; phase F owns cost.
- **Known gap: the airy lower crown.** Dormant-bud draperies (M10, F4) become a later species-fidelity gap spec, by the host.
- **Known gap: needle spacing.** The renderer's dispatch limit forced 3.5 mm needle spacing (FRICTION.md), which may be part of the see-through look.
- **The straight-down tropism jump** is fixed in fn-200, and every engine change moved onto the fn-200 branch. This branch now carries only values and stills.

## Round 4 (worker)

**Same values as round 3, on fn-200's current engine** (`1f491cfb`, `dd5cc47d`): the tropism taper near straight down, sag about the frame-carried torque, and a landing that keeps its lean.

**Sheets** (`raw/round4/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 3's seed 1 and round 3's seed 4 base, above the five 80-year trees in leaf and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases and limbs, every age and seed |
| `seed4-r3-vs-r4.png` | Seed 4's base and whole tree, round 3 beside round 4 |

**Measures** (`run.log`) are identical to round 3: the same node counts, heights and widths, and growth in 7.4 to 9.1 s at 80 years. The engine changes move wood but grow none.

**Reading:** round 4 reads as round 3.
- The stills differ in pixels (the trunk-base close-ups most), but by eye the trees are the same: smooth landings, the skirt, width 0.63 to 0.73, seed variety, and the same airy lower crown.
- No still shows a new artefact.

**What the straight-down taper changes:** nothing visible in the spruce.
- I rebuilt once with the taper's cone at 1e-12 (effectively off), rendered 20 and 80 years at seeds 1 and 4 into `raw/round4-notaper/`, then restored the code.
- Every still is identical to round 4 to within 30 levels of colour per pixel.
- Sag stops 0.15 rad short of straight down, three times the 0.05 rad cone, so sagging wood never enters it.
- In the beech, 43 of 2.8M pixels differ, which is a few twigs.

**Seed 4's dark band at the trunk base:** not a limb hanging straight down, and the fix did not remove it.
- At 80 years, seed 4 has no wood thicker than 1 cm below 4 m steeper than heading z −0.85.
- No main-branch wood thicker than 1.5 cm comes within 0.6 m of the trunk's axis below 3 m, except the trunk itself.
- It is most likely a low limb running toward the close-up camera, which stands 6 m from the trunk. Seen nearly end-on from below, its shaded underside reads as a dark band over the trunk. It is the same in round 3 and round 4. I did not chase it further.

## Gate, round 4 (2026-10-04)

**The host is confident in the frame.** Astra failed all three samples (`ASTRA-VERDICT-R4-{1,2,3}.md`); every seed was BORDERLINE or NO. The common fault:

> "the missing curtain of densely needled, pendulous branchlets: across all five seeds, foliage exposes the branch scaffolding instead of forming the hanging masses visible in the references" (R4-1); "the foliated crowns remain too close to their bare-wood silhouettes" (R4-2); "insufficient hanging, needle-bearing branchlet mass" (R4-3).

Their second complaint: branches that look upswept and stiff, "stiff, ascending fans", "rigid, upward-pointing branch sprays".

## Round 5 (worker)

**Host decision for round 5:** hanging curtains and less upswept boughs, by values. fn-201 is merged into this branch, so the renderer's dispatch limit is gone and needle spacing is back to 2.5 mm. Up to about 10 s per 80-year tree is accepted.

**Values** (`spruce.rs`):

| Part | Round 4 | Round 5 |
|---|---|---|
| Main branches, older wood | elevation 0.55, tropism 0.6, insertion 1.45 | elevation 0.15, tropism 0.4, insertion 1.5 |
| A branch's first six years (the young upper crown) | elevation 0.55 | elevation 0.35 |
| Branchlets | sag 0; elevation −0.85, tropism 3.0 | sag 2e-4; elevation −0.7, tropism 1.5 |
| Shoots per node on branchlets | up to 2 | 2 at the medial nodes, 3 at the top (p 0.55, 0.8) |
| Shoots per node on spurs | 2 | 3 (p 0.35) |
| Shoots | insertion 0.8, elevation −0.6, tropism 0.5, lifespan 4 | insertion 1.1, elevation −0.5, tropism 1.0, lifespan 3 |
| Spurs | 45 years | 40 years |

- **Why the lower lifespans:** cost. With lifespan 4 and 45-year spurs, seed 7 grew past the 20M phytomer budget.
- **Needle spacing:** I also tried 1.5 mm at seed 1: 23.6M needles and 18.7 s of dressing, only slightly denser from the whole-tree view. The sheet uses 2.5 mm.

**Sheets** (`raw/round5/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 4's seed 1 and its limb close-up, above the five 80-year trees in leaf and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases and limbs, every age and seed |

Measures are in `run.log`:

| Age | Height | Width | Grown in | Dressed in | Needles |
|--:|--:|--:|--:|--:|--:|
| 80 | 21.9 to 22.2 m | 13.8 to 16.3 m | 7.1 to 8.9 s | about 12 s | 14.0M to 15.1M (round 4: 12.3M to 13.2M at 3.5 mm) |

### Reading against Astra's fault, per seed (80 years)

| Seed | Upswept, stiff boughs | Hanging needled curtains; whole tree vs its bare silhouette |
|---|---|---|
| 1 | **Fixed.** Boughs leave level and the lower ones arch to the ground; tips turn up slightly | Fringes hang under each bough in the limb close-up. From the whole-tree view the lower crown is still see-through: the in-leaf tree tracks its bare silhouette |
| 7 | **Fixed.** Level boughs, a lower skirt, no ascending fans | Fuller middle crown than round 4, but the lower crown is still lines over a light underlayer |
| 2 | **Fixed** | The densest of the five, a fuller lower-left mass; still not S1's dark curtain |
| 3 | **Fixed** | As seed 2, slightly sparser |
| 4 | **Fixed.** Narrow, with level boughs | See-through lower crown; the base close-up no longer shows round 4's dark band |

**Overall:**
- **Astra's second complaint is answered** at every seed: no upswept fans remain, and the upper crown stays level to ascending as in S1.
- **The main fault is not answered.** The needled trees are a little fuller than round 4, but still close to their bare-wood silhouettes. In the limb close-ups, the curtains are fringes of thin hanging shoots, not dense sprays with width.
- **Values reach their limits here:** cost (more shoots overran the phytomer budget), and needle density (1.5 mm needles barely changed the look). What remains looks like draperies (M10, the recorded gap) and possibly needle size and placement in the dressing.
- **Young trees:** they keep round 4's look with level tiers. 10 years reads as a sapling; 20 and 40 years read as young spruces.

## Probe 6: needle stance (worker, 2026-10-04)

**The host's reading of round 5:** each shoot dresses as a thin string because its needles hug the wood. The host asked for a probe of the dressing rows that set the needle's angle.

**What the code does** (`crates/telperion-core/src/pipeline/foliage/station.rs`, `axis` and the station loop):
- Each needle's axis is the radial off the wood, plus the tangent times (`forwardLean` + `leanRise` × the radial's upward part), plus `outward` from the trunk, plus `upward`.
- With the preset's `forwardLean` 0.05, needles already stand at about 90° all round the shoot. Only the upper ones lean forward, by `leanRise` 1.2.
- `surfaceContact` moves only the needle's seat, from the shoot's axis (0) to its surface (1), not its angle.
- `shootRadius` sets which wood is clothed: wood thinner than that share of the trunk's radius.

**A finding about the stills:** the "limb" close-up in every round so far was rendered **bare** (`View::Bare`). It shows no needles at all, so the strings in the earlier limb close-ups were wood. The stills runner now adds a `spray` shot: the same camera, in leaf.

**Probe sheet:** `raw/probe6/probe6.png` holds S3 and six in-leaf limb close-ups of seed 1 at 80 years. `raw/probe6/crop.png` holds full-size crops of the lower crown for four of them.

| Variant | Rows changed |
|---|---|
| a | the preset |
| b | `leanRise` 0, `forwardLean` 0.3 |
| c | `leanRise` 0, `forwardLean` 0.7 |
| d | b with `surfaceContact` 0 |
| e | b with `shootRadius` 0.06 |
| f | b with `upward` −0.4 |

**Reading:**
- **The variants are indistinguishable.** Stance is not the lever.
- **The bough tips are needled sprays:** dense, flat, dark green, close to S3.
- **The inner curtains are long hanging strands with sparse dots of needles.** They are the old branchlets and spurs. A spur makes about one short shoot a year (3 buds at p 0.35), and each shoot lives 3 years, so a spur carries only about three live shoots at a time. Its own wood is a single strand, and at 9 m a needled strand is a dotted line.
- **`shootRadius` 0.06 changes nothing,** so the strands are already clothed.
- **What a spray needs:** a branchlet of many lateral shoots alive at once, which S3 shows.

**What decides this:** the structure, not the dressing. It is the live shoot count along old branchlets in `crates/telperion-space/src/spruce.rs` (the spur and shoot rows): values limited by the phytomer budget, or the draperies gap (M10). I did not apply a variant and did not render round 6.

## Round 6 (worker)

**Host decisions for round 6:** sourced live shoots on old branchlets, spurs bearing sprays, a budget raise if needed, and boughs widened toward 0.7 of the height.

**Values** (`spruce.rs`):

| Part | Round 5 | Round 6 |
|---|---|---|
| Shoots | grow 3 years, shed 1 year after their apex stops | grow 2 years, kept 4 more |
| Spurs | 3 buds a year at p 0.35, often one shoot | a pair a year in the spur's own plane, at p 0.9 |
| Main branches' internode | 0.03 | 0.032 |

- **Source for shoot life:** Muukkonen, P. and Lehtonen, A. (2004), "Needle and branch biomass turnover rates of Norway spruce (*Picea abies*)", *Canadian Journal of Forest Research* 34: 2517–2527.
  - From its abstract: "At the age of 5.5 years, 50% of the needles in the needle cohort have been shed", and all are shed at 12 years. The mean annual needle turnover is 0.10.
  - The numbers are read from the published abstract (via its search snippets); the full text was not reachable (HTTP 403).
- **Why a pair on each spur:** an old branchlet then hangs as a small needled spray, not a strand.
- **Width:** 0.035 gave w/h 0.85 at seed 1, too wide, so the internode is 0.032.
- **Budget:** no seed came near the 20M phytomer budget. Growth fell: shorter-growing shoots grow fewer phytomers, while the longer-kept ones add kept nodes. **No raise was needed.**

**Sheets** (`raw/round6/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 5's seed 1 and its in-leaf spray, then the five 80-year trees in leaf, as in-leaf sprays, and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases, bare limbs, and the young trees' bases and sprays |

**Measures** (`run.log`) at 80 years:

| Height | Width | w/h | Grown in | Dressed in | Kept nodes | Needles |
|--:|--:|--:|--:|--:|--:|--:|
| 21.9 to 22.2 m | 14.6 to 17.2 m | 0.67 to 0.79 | 5.9 to 6.9 s | about 15.5 s | 3.4M to 3.7M | 18.3M to 19.8M (round 5: 14.0M to 15.1M) |

### Reading against Astra's fault, per seed (80 years)

| Seed | In-leaf spray close-up | Whole tree against its bare silhouette |
|---|---|---|
| 1 | Boughs are dense flat needled sprays to the tip. Under them the curtains are now short sprays, not dotted strands | The upper and middle crown is a dark mass. The lower third is still see-through, with boughs as stripes over light |
| 7 | The densest sprays of the five: thick, dark boughs, close to S3 | A fuller middle crown. The lower left still shows separate boughs with gaps |
| 2 | Dense sprays and heavy lower boughs | Fuller than round 5 throughout. The lower crown is still see-through between the boughs |
| 3 | As seed 7 | As seed 2 |
| 4 | Dense sprays on a narrow tree | See-through lower crown on both sides |

**Overall:**
- **The boughs themselves are now right:** dense, flat, dark needled sprays, as in S3. The in-leaf trees no longer track their bare silhouette in the upper and middle crown.
- **The inner and lower crown is still see-through, plainly.** The space between the boughs is empty, where S1 hangs dark curtains, and the bare base close-ups still show bare inner bough wood. Sourced needle retention and spur sprays fill each bough but not the space between boughs. That fits the missing mechanism: draperies (M10), the waves of new branchlets released along old branches.
- **Young trees:** they look as in round 5 but fuller in the 20- and 40-year sprays. 10 years is still a sparse sapling.

## Gate, round 6 (2026-10-04)

The host found round 6 clearly better than round 5, with sprays that read like S3. Astra failed all three samples, mostly NO (`ASTRA-VERDICT-R6-{1,2,3}.md`). Two faults:

1. **Branchlet habit:** "foliage spreads into thin horizontal shelves instead of hanging beneath the boughs in substantial comb-like curtains" (R6-3). The lower boughs read as "flat fans rather than descending with upturned tips".
2. **Staging:** the tree's ground shadow, the dark diagonal left of every tree, read as "a conspicuous upward lower limb" and a "lower-left bulge". The photographs have no such shadow.

## Round 7 (worker)

### Staging (`crates/telperion-render/examples/space/still.rs`)
- **The change:** the stills runner now sets the sun to azimuth 115°, elevation 60° for every shot. The renderer's default was 135° and 30°.
- **Why 115°:** the hero camera looks from azimuth 115°, so the sun now stands behind the camera and high. The shadow falls behind the tree, away from the camera, and short.
- **Scope:** this applies to every species' stills, the beech's included; the beech has not been re-rendered under it.
- **Side effect:** the ground and the sunlit sides read brighter, so the colours, matched by render under the old sun, are not re-matched.

### Branchlet habit (`spruce.rs`)

| Part | Round 6 | Round 7 |
|---|---|---|
| Branchlets (first ten years) | elevation −0.7, tropism 1.5, sag 2e-4 | elevation −1.0, tropism 2.0, sag 5e-4. They keep their flat spray, since the bough tips are right |
| Spurs (a branchlet's later decades, the curtain) | as branchlets, pairs in one plane | divergence 2.4, so each year's pair of shoots turns by the golden angle and the shoots stand all round, not in one plane; elevation −1.35, tropism 3.0, so they hang near straight down |
| Shoots | roll 0 | roll 1.2, so siblings turn about their bearer |
| Main branches | sag 5e-5 | sag 7e-5 |

**How plane and roll act:**
- A lateral's side is its bearer's side turned by the bearer's `plane`.
- Its azimuth about the bearer is the bearer's divergence times the node's rank, plus its slot's share of the whorl, plus its own `roll`, drawn either way by its lineage.
- So the branchlets' 0 plane kept every pair of shoots in one plane. The spur's golden divergence and the shoots' roll take the spray out of it.

**Sheets** (`raw/round7/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 6's seed 1 and its spray, then the 80-year trees in leaf, as in-leaf sprays, and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases, bare limbs, and the young trees' bases and sprays |

**Measures** (`run.log`) at 80 years: grown in 5.9 to 7.1 s and dressed in about 15.5 s. Height 21.9 to 22.2 m, width 14.3 to 17.3 m (w/h 0.65 to 0.79), 18.3M to 19.8M needles.

### Reading per seed (80 years)

| Seed | Staging | Branchlet habit (spray close-up and whole tree) |
|---|---|---|
| 1 | **Fixed.** No shadow beside the tree; the ground is clear to its left | Boughs carry darker hanging masses below their flat tips. From the whole-tree view the crown reads as a continuous cone with darker underlayers; the lower third is less see-through than round 6 but still shows light between boughs |
| 7 | **Fixed** | Curtains under the middle boughs read as dark hanging masses, the closest to S3 yet; flat shelves only at the tips |
| 2 | **Fixed** | Heavy lower boughs with masses beneath; the lower crown is fuller |
| 3 | **Fixed** | As seed 7 |
| 4 | **Fixed** | Narrower; the masses are fewer and the lower crown is the most see-through of the five |

**Overall:**
- **Staging is fixed at every seed.** Round 6's lower-left "bulge" and "upward limb" were the shadow, and they are gone.
- **Branchlet habit is better but not fully S1.**
  - The old branchlets now hang as masses all round, and the tips stay flat sprays, as asked.
  - At whole-tree distance the boughs still read mostly as layered shelves with a darker underside, not as S1's deep hanging curtains.
  - The trunk-base close-ups show the lowest boughs swept down steeply to the ground and bare inside: a skirt of rods, the draperies gap.
- **Young trees:** the 40-year sprays are still flat shelves, as in round 6.

## Gate, round 7, and host decision (2026-10-04)

Host: shadow-free staging; conic spruces with dense tip sprays; confident in the frame, not the mass. Astra, three samples: FAIL in all, but mostly BORDERLINE (round 6 mostly NO): "the boughs read as thin, flat shelves rather than substantial, drooping sprays with hanging comb branchlets" (`ASTRA-VERDICT-R7-{1,2,3}.md`). Values were pushed three rounds (retention, spur sprays, hang). Host decision: build dormant-bud release (draperies, MODEL-SPRUCE.md F4, M10) as its own engine spec, fn-202, which this spec now depends on.

## Round 8 (worker): sleeping buds on the main branches (fn-202)

**Values** (`spruce.rs`):
- **Sleeping buds:** every zone of both main-branch stages (the young six-year branch stage and the branch) carries a sleeping branchlet bud: probability 0.45 per bud place, delay 1 year, yearly waking rate 0.3. These are fn-202's release law from its R5 strip; the strength is tuned here.
- **Needle mass, moved not added:**
  - Spur shoots p 0.6 (0.9 before).
  - Shoots keep their needles 3 years after they stop growing (4 before): about 5 years in all, near Muukkonen and Lehtonen's half-life of 5.5.
  - Needle spacing 3.2 mm (2.5 before).

**Strength probe** (seed 1 at 80 years, `raw/probe8/`, one tree per process):

| Sleeping probability | Needles | Result |
|--:|--:|---|
| 0 | 15.0M | No change |
| 0.3 | 19.6M | Visibly denser lower and middle crown |
| 0.5 | | GPU out of memory |
| 0.45, at 3 mm needles | 18.5M | Denser again |

**Sheets** (`raw/round8/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 7's seed 1 and its spray, then the 80-year trees in leaf, as in-leaf sprays, and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases, bare limbs, and the young trees' bases and sprays |

**Needles per tree at 80 years** (`run.log`):

| Seed | Needles | Grown in |
|--:|--:|--:|
| 1 | 17.1M | 9.8 s |
| 7 | 18.1M | 10.4 s |
| 2 | 18.1M | 10.3 s |
| 3 | 17.5M | 10.1 s |
| 4 | 16.9M | 9.8 s |

- Dressing takes about 15 to 16 s, and wood is 54 to 58 km.
- Seeds 1 and 7 first failed with GPU out of memory and rendered on a retry with the same binary and values (FRICTION.md).

**Measures:** height 21.9 to 22.2 m, width 14.6 to 17.2 m (w/h 0.67 to 0.79). Staging, width and colours are as in round 7.

### Reading against Astra's round-7 fault (inner curtains, not flat shelves with gaps)

| Seed | Whole tree | Spray close-up |
|---|---|---|
| 1 | Middle and lower crown darker and more continuous than round 7; the gaps between lower boughs are narrower but still show ground | Boughs carry dense dark masses beneath their flat tips, reaching further in towards the trunk |
| 7 | The fullest of the five: a dark cone almost to the ground, with only slivers of light between the lowest boughs | Heavy hanging masses under every bough, the nearest yet to S1 and S3 |
| 2 | Full and dark through the middle; the lower crown has a few light gaps | Dense masses under the middle boughs |
| 3 | As seed 7 | As seed 7 |
| 4 | Narrow; still the most see-through low on both sides | Masses present but thinner |

**Overall:**
- **Sleeping buds move the tree towards S1's curtain.**
  - The middle and lower crown no longer read as separate shelves over light at seeds 7, 3 and 2.
  - At seeds 1 and 4 light still shows between the lowest boughs.
  - The bough tips stay flat sprays.
- **The trunk-base close-ups still show the lowest, oldest boughs bare near the trunk.** They are fewer, and more hanging branchlets show among them.
- **Limits:**
  - The GPU's memory stops the sleeping probability near 0.45.
  - Growth takes about 10 s, within the 12 s accepted.
  - Denser curtains would need needle mass moved again, or more memory.
- **Young trees:** much as in round 7; the sleeping buds barely show at 20 and 40 years.

## Gate, round 8 (2026-10-04)

Host: denser at every seed, 7 and 3 near-continuous cones; confident in the frame and much of the mass. Astra, three samples: FAIL, mostly BORDERLINE (as round 7): "flattened horizontal sprays dominate instead of sweeping lower boughs carrying long, hanging comb branchlets" (`ASTRA-VERDICT-R8-{1,2,3}.md`). Host decision: round 9 targets the bough curve, downward sweep from the trunk with tips recovering upward (named in rounds 6 to 8), by values on sag and tip tropism.

## Round 9 (worker): the bough curve

**Probes** (one limb close-up and whole tree of seed 1 at 80 years each):

| Probe | Variants (tropism / elevation / sag) | Reading |
|---|---|---|
| `raw/probe9/probe9.png` | a: round 8's 0.4 / 0.15 / 7e-5; b to e: 0.8 / 0.6 / 1.2e-4 up to 2.2 / 1.0 / 2.5e-4 | b to e upswept, upward fans, worse than round 8: low sag with high tropism only lifts the boughs |
| `raw/probe9b/probe9b.png` | g: 0.8 / 0.7 / 8e-4; h: 0.5 / 0.5 / 1.5e-3; i: 1.0 / 0.9 / 1.5e-3 | g and i show the down-then-up hook, with the inner bough drooping and the tip recovering. h droops too far. f (0.6 / 0.6 / 4e-4) never rendered: GPU out of memory three times |
| `raw/probe9c/`, `probe9e/` | g with the young six-year stage held at tropism 0.4, elevation 0.35, sag 1e-4 (8e-4 and 6e-4 on older wood) | Picked 6e-4: the clearest hook, with the upper crown level to ascending |

**Values** (`spruce.rs`):
- **Older branch wood:** tropism 0.8, elevation 0.7, sag 6e-4, internode 0.036 to hold the width.
- **Young six-year stage:** tropism 0.4, elevation 0.35, sag 1e-4.
- **Irregularity:** wander 1.0 (was 0.8), roll 0.8 (was 0.5), dominance 0.35 (was 0.25).
- **Kept from round 8:** sleeping buds, needle spacing 3.2 mm, staging and colours.

**Stills runner:** it now prints each tree's mesh size (wood vertices, triangles, needles) before the GPU sees it, so a refused allocation says what it was asked to hold.
- At 80 years the wood is 99M to 107M vertices and 196M to 211M triangles. That is likely the memory pressure, more than the needles.
- Renders retry a tree up to four times after 30 s; none needed it this round (FRICTION.md).

**Sheets** (`raw/round9/`, on disk; I viewed every still):

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 8's seed 1 and its limb, then the 80-year trees in leaf, as bare limb close-ups, as in-leaf sprays, and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases, and the young trees' bases and sprays |

**Measures at 80 years** (`run.log`):

| Seed | Needles | Grown in | Width |
|--:|--:|--:|--:|
| 1 | 15.5M | 9.8 s | 15.7 × 14.9 m |
| 7 | 16.5M | 10.3 s | 14.8 × 14.2 m |
| 2 | 16.4M | 10.6 s | 15.1 × 14.7 m |
| 3 | 16.0M | 10.4 s | 14.2 × 15.7 m |
| 4 | 15.3M | 9.8 s | 12.9 × 16.7 m |

w/h is about 0.6 to 0.76.

### Reading against "flat shelves, no tip recovery"

| Seed | Limb close-up | Whole tree |
|---|---|---|
| 1 | Every middle and lower bough sweeps down from the trunk and its outer third curls back up: the hook S1 shows | A conical crown with drooping lower boughs whose tips rise. The silhouette is narrower at the base and more "weeping" than S1 |
| 7 | As seed 1, the clearest hooks | Full lower crown, the skirt spreading on the ground |
| 2 | As seed 1 | Heavy lower crown, irregular tiers |
| 3 | As seed 1 | As seed 2 |
| 4 | Hooks, with a few long lower boughs | Narrow and irregular, one long bough to the right |

**Overall:**
- **"Flat shelves, no tip recovery" is answered** in the limb close-ups at every seed. The tiers are less mechanical: bough lengths and angles vary between neighbours.
- **New fault, plainly: loops at the base.**
  - In the trunk-base close-ups, the lowest boughs droop so steeply that their recovering tips curl back up past the trunk, drawing loops and coils of bare wood around the base. This shows at every seed and is clearly wrong.
  - It comes from strong sag on long, old lower boughs combined with tip tropism: the drooping part reaches near straight down, and the tip still turns up.
  - Whether the lowest boughs should take less sag, or sag should scale differently with length, is a host design question. I stopped here.
- **The whole-tree look also changed:** the crown narrows at the base, with drooping lower boughs and upturned tips, closer to a weeping form than S1's broad base.
- **Young trees:** 20 and 40 years keep level tiers. The young stage barely sags, so the hooks appear only on older wood.

## Round 10 (worker): the hook on fn-203's bent-lever sag

**Probes** (seed 1 at 80 years; limb, whole tree and trunk base per variant; tropism / elevation / sag on the older branch wood):

| Probe | Variants | Reading |
|---|---|---|
| `raw/probe10/probe10.png` | 1.2–2.0 / 0.9–1.1 / 2e-4 to 4.5e-4 | Upswept: tropism wins and the boughs rise as fans |
| `raw/probe10b/probe10b.png` | 0.6–1.0 / 0.6–0.9 / 1.2e-3 to 4e-3 | The boughs hang near straight down and spread on the ground: a column-shaped tree with a skirt |
| `raw/probe10c/probe10c.png` | i 1.0 / 0.8 / 6e-4; j 1.4 / 1.0 / 9e-4; k 1.2 / 0.9 / 7e-4; l 1.8 / 1.1 / 1.2e-3 | **j and k show the hook:** down from the trunk, outer third recovering. i is nearly level; l recovers too early, near the trunk |

**Values** (`spruce.rs`, older branch wood only):
- **Picked j:** tropism 1.4, elevation 1.0, sag 9e-4, internode 0.034 (was 0.036, to bring the width towards 0.7).
- **Unchanged:** the young six-year stage, the dormant curtains, the irregularity, needle spacing 3.2 mm, staging and colours.

**Sheets** (`raw/round10/`, on disk, one tree per process with retries; none were needed). I viewed every still.

| Sheet | What it holds |
|---|---|
| `five-seeds.png` | The references, round 9's seed 1 and its limb, then the 80-year trees in leaf, as limb close-ups, as in-leaf sprays, and bare |
| `young.png` | 10, 20 and 40 years at seeds 1 and 7 |
| `close-ups.png` | Trunk bases, and the young trees' bases and sprays |

**Measures at 80 years** (`run.log`):

| Seed | Needles | Grown in | Width |
|--:|--:|--:|--:|
| 1 | 15.5M | 10.0 s | 15.4 × 17.5 m |
| 7 | 16.5M | 10.3 s | 13.5 × 15.3 m |
| 2 | 16.4M | 10.9 s | 16.6 × 16.0 m |
| 3 | 16.0M | 10.3 s | 15.7 × 17.1 m |
| 4 | 15.3M | 10.2 s | 15.1 × 13.1 m |

w/h is 0.6 to 0.8.

### Reading against "flat shelves, no tip recovery"

| Seed | Limb close-up and spray | Whole tree | Trunk base |
|---|---|---|---|
| 1 | Every middle and lower bough sweeps down and its outer third turns up: the S1 hook | A broad-based cone, lower boughs drooping with upturned tips | Smooth arcs out to the ground; no loops |
| 7 | Clear hooks, dense sprays | Broad base, full lower crown | Smooth; the boughs run out along the ground with no kinks |
| 2 | As seed 1 | Broad, irregular tiers | Smooth; one bough crosses near the trunk |
| 3 | Clear hooks, long recovering tips | Broad base, the widest skirt | Smooth |
| 4 | Hooks, slightly steeper | The narrowest of the five, but not weeping | Smooth |

**Overall:**
- **The hook is back at every seed** without round 9's loops: the bent lever lets heavy wood hang instead of coiling. From the whole-tree view the boughs read as descending sweeps with upturned ends, not flat shelves, and the base is broad again.
- **Trunk bases:** no loops and no visible kinks where wood meets the ground. Some of the lowest boughs make a rounded knee just out from the trunk, as fn-203 recorded.
- **Upper crown:** level to ascending, as in S1.
- **Young trees:** 20 and 40 years keep level tiers, with the 40-year boughs drooping slightly. 10 years is unchanged.
- **Still open:** light between the lowest boughs at seeds 1 and 4, as in round 8.
