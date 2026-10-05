# fn-196 round 1: the date palm on the engine

A dispatched worker wrote this. It reads the stills; it decides no design. The questions at the end go to the host.

## History

The first pass stopped at two design questions: the leaf bases could not reach the pipeline from an engine tree (D1), and the pipe model could not give a columnar stem (D2). The host answered both on 2026-10-05, and they were built first as their own specs. fn-204 moves leaf-base clothing to the start of expansion, with every preset byte-identical. fn-205 adds `form.secondary`, which is byte-identical at 1. Both are done, the workspace gate is green and Codex passed each (SHIP).

## What round 1 draws

- **Values** (SOURCES.md): `crates/telperion-space/src/palm.rs`. One physiological age, readiness 0, 12 phytomers a year at 0.031 m, `secondary` 0, `pipe` 0.297 m (today's trunk radius). At 50 years the stem is 600 phytomers and 18.6 m to the apex, and one width from foot to crown (`palm::tests::one_unbranched_stem_of_one_width`).
- **Dressing:** the date-palm preset's rows, unchanged and with no overlay (`examples/space_palm.rs`). The fronds, skirt, 256 retained leaf bases in the lattice, acanthophylls, the foot's flare and the material all come through the conversion and `executor::expand`. No inflorescence is drawn today (fn-111), so none is owed.
- **Today's palm** is the preset's own `mesh::build` at the same seeds (`--today`), framed and lit the same way.

## Sheet

`.flow/evidence/fn-196-tree-space-d-the-date-palm-as-a-point/raw/round1/` (gitignored):

- `SHEET-round1.png`: the three references (P-WHOLE, P-TRUNK, P-BASE, from `.flow/references/date-palm/`), then today's palm (top row) above the new palm at 50 years (bottom row), at seeds 1, 7, 2, 3 and 4, whole.
- `seed-<s>-all.png` for each seed: today (top) above new (bottom) in six shots, whole, bare, crown close-up, base, limb and spray.
- `ages-all.png`: the new palm at ages 5, 10 and 20 for seed 1 (rows 1 to 3) and seed 7 (rows 4 to 6), in the same six shots.
- `zoom-1.png`: whole, seed 1, today beside new at full resolution, and bare, seed 7, today beside new.
- Every single still, `today-<seed>-<shot>.png` and `palm-<age>-<seed>-<shot>.png`, 96 in all.

Every still was viewed, through these composites.

## Reading against today's palm

| Seed | Whole | Bare and base | Crown close-up |
|---|---|---|---|
| 1 | The same silhouette: a narrow rosette of arching grey-green fronds over a single latticed trunk, crown base near half height. The new trunk is straight from foot to crown; today's carries a slight kink and wave below the crown. | The new stem is a straight column, latticed from just above the flare to the crown, the flare as today. Today's tapers slightly and wanders near the top. The base close-ups are near identical. | The same frond count, arch and droop. |
| 7 | As seed 1. The new crown sits a little rounder. | Today's stem shows its kink at about 85 % of the height; the new stem has none. | Equivalent. |
| 2 | As seed 1. | As seed 1. | Equivalent. |
| 3 | As seed 1. | As seed 1. | Equivalent. |
| 4 | As seed 1. Today's trunk wanders a little more. | As seed 1. | Equivalent. |

**No visible regression at the mature age.** The one visible change is the stem: a straight column against today's slightly tapered, kinked one. Both are near-vertical. Mesh heights are 25.2 to 25.4 m new against 25.9 to 27.2 m today, frond tips included.

**Across seeds the new crowns vary less than today's.** The frond draws are keyed by the family's seed and the apex's identity, and the space examples keep the preset's seed for every engine seed. So the five new crowns are one crown, turned a little by each stem's last segment. Today's crowns differ seed to seed. The same holds for the beech and the spruce sheets.

**The limb and spray shots** frame empty sky or loose frond tips. A palm has no limb, so these two shots carry nothing for it.

## Against the references (shared by today's palm and the new one)

- **Present:** one trunk and no laterals; the diamond lattice of flat-faced bases; a compact rosette of arching pinnate fronds; the flared foot (P-WHOLE).
- **Absent in both:**
  - The lean of about 15° and the curve near the top in P-WHOLE.
  - The lower fronds drooping well below the crown base into a dark skirt; the drawn skirt barely reads at this framing.
  - The hanging date clusters (fn-111).
  - The bases weathering smooth toward the foot (P-TRUNK).
  - The trunk's dark fibrous brown: it renders pale tan under this sun.

## Young palms (ages 5, 10, 20)

- The stem is a short column at full width, 1.9, 3.7 and 7.4 m tall, carrying the **full-size adult crown**. At 5 and 10 years the whole tree reads as a 19 m wide ground rosette with a stump. The frond rows (`rachisLength` 7 m, 42 fronds and the skirt) do not follow the stem's age.
- **All 256 leaf bases are packed onto the short stems.** At 5 and 10 years they are a fine scale pattern, many to the trunk's width, unlike the mature lattice. The base count and size are rows too, not a function of the stem's length.

## Framing (still.rs)

The camera is now fitted to the mesh bounds, fronds and leaves included, so the palm's crown is in shot. There is also a `crown` close-up for every tree, and `--today` for the preset's own build. Beech and spruce at 80 years, seed 1, were rendered by the old and new binaries (`raw/framing/*-compare.png`):

- **Whole and bare:** no visible change. The tree's size and place in frame are the same; the mesh bounds are about 1 % taller than the node bounds (beech 18.6 against 18.8 m, spruce 21.9 against 22.0 m). The pixel RMSE between old and new is 0.058.
- **Limb close-up:** moves visibly. It aims a third of the bounds' width off the stem, and the mesh's width differs. The beech's limb shot shifts about a metre sideways and frames a different part of the same limb (RMSE 0.17); the spruce's shifts less (RMSE 0.10).

## Questions for the host

1. **Young palms.** The frond crown and the 256 leaf bases are preset rows, the same at every age, so a 5 or 10 year palm wears an adult crown on a stump. Should the dressing's organ sizes and counts follow the engine tree's age or stem, or is the palm judged at maturity only?
2. **Seed variety.** The space examples keep the preset's seed, so the frond crowns vary across engine seeds only through the stem. Should the example key the family's seed to the engine seed? That would change the beech and spruce sheets' leaves as well.
3. **Lean.** The engine has no lean for an orthotropic stem; `tropism` pulls it back to the vertical. P-WHOLE's 15° lean is absent here as it is today. Is this round's bar today's palm (met) or the reference (a gap)?
4. **fn-204's open P2 (Codex).** A tree that already carries its bases, handed back to `expand`, is clothed again. The spec kept the step's behaviour unchanged, so it was left as it is.

## Host decisions on round 1 (2026-10-05)

1. **Fronds and leaf bases by age:** the palm is judged at maturity for D3. Young palms wearing the adult crown, and the 256 bases packed onto a short stem, are a known gap. The fix belongs in fn-198's station contract (phase F): organs placed from the engine's phytomers, so that frond size and count and leaf-base count follow the stem the engine grew. The young stills stay on the sheet (`raw/round1/ages-all.png`).
2. **Seed:** the space examples now set the preset's seed from the engine seed for every species (`still.rs`, `seeded`).
3. **Lean:** judged against today's palm, which meets it. The reference's 15° lean is a known gap; a site-driven lean of an orthotropic stem is a later setting, not this spec.
4. **fn-204's P2:** fixed. A tree that already ends in exactly the bases its table hangs is not clothed again. The test was red first (1,064 nodes against 552), all eight preset digests are unchanged, and Codex re-reviewed it: SHIP.

## The seed keyed to the engine's: beech and spruce

Beech and spruce at 80 years, seed 7, were rendered before (the preset's seed) and after (the engine's): `raw/seeded/{beech,spruce}-seeded.png`, whole above spray, before on the left. The structure is the same, so the silhouette, the limbs and the crown's mass do not move.

- **Beech:** the leaves sit in other places and their hue jitter falls on other leaves, so the light olive patches move. The leaf count is the same (2,833,880). Nothing changes beyond leaf placement.
- **Spruce:** the needle sprays are indistinguishable at both framings. The after run placed 16,483,690 needles; the before run's count was not logged. Nothing visible changes beyond placement.

## Round 1b: the mature sheet with the seed keyed

`raw/round1b/SHEET-round1b.png` has the three references at the left. To their right, today's palm (top row) above the new palm at 50 years (bottom row), at seeds 1, 7, 2, 3 and 4, each as a whole shot cropped to the tree (480 x 720 of the 960 x 720 still) beside its crown close-up. `view-a.png` (seeds 1, 7, 2) and `view-b.png` (seeds 3, 4) are the same panels at full size. Every still was viewed.

- **The crowns now vary across seeds as today's do.** Each seed draws its own fronds: the spacing and droop of the lower fronds and the number standing above the apex differ seed to seed.
- **At every seed** the new palm reads as today's: the same narrow rosette of arching grey-green fronds over a single latticed trunk, the crown base near half height.
- **The one visible difference is still the stem:** a straight column against today's slightly wandering one.
- **Close-ups:** the new crown shows its upper fronds a little more upright and open at seeds 1, 3 and 4, as its apex axis is near-vertical while today's tilts with the stem's last kink. The difference is slight, and is a reading, not a measure.
- **Heights:** the new palm is 25.3 to 25.7 m to the frond tips, against today's 25.9 to 27.2 m.

No visible regression against today's palm at maturity.
