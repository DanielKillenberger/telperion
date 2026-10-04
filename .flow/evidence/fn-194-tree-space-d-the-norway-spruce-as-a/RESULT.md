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
