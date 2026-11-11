# fn-354 R5: the leaf account, survey and design, 11 Nov 2026

The survey is a Fable agent's, read-only, condensed by the host; the design under it is the host's. [code] read in source, [evidence] a recorded number, [inferred] arithmetic, [U] not determined.

## A correction to the host's verdict

`PROBE-VERDICT.md` item 5 said the grower's leaf count "leaves out the short-shoot clusters that carry most of a beech's and a birch's leaf". **That is wrong.** A short shoot is its own axis and its nodes deposit leaf every year it lives: the beech's lays 3 to 5 nodes a year for 4 years against a drawn cluster of 4 (`beech.rs:325-349`), and fn-339's twelve-seed table gives the beech the same current-year and standing leaf (31.64 and 31.62 m²) [code, evidence]. `cluster_leaves` is a drawing row; adding it to the grower would count those leaves twice.

## Where the drawn leaf and the grower's leaf part, by species [inferred from the probe's G2, seed 1001]

| Tree | Drawn per leaf | Grower per leaf | Size ratio | Count ratio |
|---|---|---|---|---|
| Beech 40 | 33.7 cm² | 15.7 cm² | 2.14 | 1.43 |
| Birch 40 | 11.6 cm² | 13.2 cm² | 0.88 | 3.03 |
| Oak 40 | 26.1 cm² | 18.1 cm² | 1.44 | 0.52 |
| Oak 120 | 26.1 cm² | 18.1 cm² | 1.44 | 0.98 |
| Spruce 40 | 3.1 needles per cm, each about 4 times the grower's | 24.5 per cm | | |

- The oak from 80 on: the drawn blade is 1.44 times the measured leaf, nothing else.
- The beech: two thirds blade size, the rest count.
- The birch: all count; its `leaf_years` is "never", which leafs every slender run of any age.
- **Why the drawn leaf moved with the girth:** leaves are placed on wood under `stem_radius × canopy.shootRadius`, a share of the root's radius (`foliage/plan.rs:462-472`). A slimmer trunk lowers the bound. [code]

## What the grower's account really lacks

- **Leaves older than a year.** Only the phytomers laid in the current cycle deposit (`grow/draw.rs:213-234`); the lattice is rebuilt every year. For the oak and beech that is their standing leaf. For the spruce it is a fifth of it: 82.0 m² current against 441.6 m² standing at 40 (fn-339, twelve seeds), so the spruce shades itself with a fifth of its needles.
- `leaf_years` is the row that should say how long a phytomer bears its leaf. Today only the drawing reads it, and its "never" on the birch, spruce, palm and ordinary tree means "no gate, the girth bound decides", which the grower cannot evaluate. `leaf_life` is presentation only (the played year). fn-267 asks for this same change for the light.
- **The balance reads no leaf area:** income is `size × light` at living tips (`passes.rs:50-62`). The leaf account moves it only through the light.

## The design

One definition, in the grower, of the leaf a phytomer bears in year y:

    leaf(p, y) = leaf_area × p.scale × p.size × clamp(leaf_years − (y − p.cycle), 0, 1) × fade(p's axis, y)

- `leaf_years` is a trait with a value for every species: the years a phytomer keeps its leaf. The light's deposit and the wood rule both read `leaf(p, y)`; the drawing keeps reading the same row for its stations.
- `fade` is the kept share a closed subtree is already drawn at while it is held, so a dead shoot's needles go by degree and not in a step.
- No cluster term. `cluster_leaves` stays the drawing's.
- The deposit walks every held axis, not only the open ones.

Values:

| Species | `leaf_years` | Source |
|---|---|---|
| Oak, beech, Laurelin, Telperion | 1 | as today |
| Birch | 1 | deciduous; today "never" |
| Spruce | 5.5 | Muukkonen & Lehtonen 2004: half a cohort shed at 5.5 years. Estimate in form: a cohort thins gradually to 12 years, and one life stands in for that curve |
| Ordinary tree | 1 | estimated: a deciduous broadleaf; today "never" |
| Palm | as today | its fronds are drawn by the rosette rows and its `sapwood` is 0; nothing reads its deposit |

**Expected on paper** (first order, the light's feedback ignored):

- Oak, beech and the Two Trees: nothing moves.
- Spruce: the lattice over its own ground goes from a leaf area index of 2.3 to 12.3; an interior shoot's yearly survival falls from about 0.875 to 0.68 at its shade hazard. The crown should thin from the inside and make fewer shoots, which is the direction real spruces differ from it (shoots per metre of branch looked high, `LEAF-SURVEY.md`). Its trunk under the wood rule already used six years of needles in the probe (39 to 42 cm at 40).
- Birch: the grower is unchanged (it deposits one year today); the drawing stops leafing old runs, so the drawn birch loses leaf wherever it stood on old wood. How much is unknown: most of a birch's leaf is on short shoots, which the drawing shows as clusters whatever the wood's age.
- Ordinary tree: as the birch.
- Cost: the spruce deposits about 5.4 times as many phytomers a year (about 50 ns each today); fewer shoots should give more than that back.

**What would refute it:** the spruce stripped to a pole or a tuft; the birch or the ordinary tree visibly barer than the base with no cause in their own leaf; a walk over `leaf_years` failing its bound.

**The walks:** the walk tree is `leafy` with `leaf_years` never. It takes 1, which is what it has tested until now, and `leaf_years` gets its own walk.

## What this does not settle

- **The beech's missing leaf** is in how many shoots it makes (R7) and nowhere else: its trunk under the wood rule stays 9 to 12 cm at 40 until that is fixed.
- **The drawn leaf is not yet the grower's leaf:** blade size (the beech draws 2.1 times its measured leaf, the oak 1.4), the oak's station spacing, the spruce's needle count, and the girth-relative bearing bound. All of it is the drawing's and is rebuilt in Phase 5 from the grower's counts. Until then the bearing bound moves with the trunk, and the scoreboard shows what that does.

## The Two Trees' leaf (host, 11 Nov 2026, late evening)

**Seen** (`product2/two-trees.jpg`, seed 1, base beside product): under the wood rule Telperion at 82 and Laurelin at 80 are worse than the base, plainly. A thin trunk under long thin limbs that arch out and lie on the ground 19 to 20 m from the trunk, where the base has a thick trunk and a round crown of heavy limbs. 153 thin runs stand under 0.5 m against 1; `ground_reach` is red on 14 flowers under the ground. Telperion at 40 is a slim stem in both.

**Cause, read in the presets:** the Two Trees are the beech with internodes 2.6 times and primary radii 3 times on their coarse ages, and leaves drawn at `/supernatural/canopy/size` 2.3 against the beech's 1.1. The grower's leaf stays the beech's 15.7 cm². The old girth summed primary radii, so the thick primaries made the thick wood. The wood rule reads the leaf, and the grower's leaf does not know these trees bear larger leaves. So their wood is a beech's leaf-worth on levers 2.6 times as long, and it bends.

**Design:** the supernatural canopy size reaches the grower. Where the group sets its canopy size, the species' `leaf_area` is multiplied by (that size / the natural `/canopy/size`)², at the same place the group's internode and primary overrides are applied to the species. For the Two Trees that is (2.3 / 1.1)² = 4.37. One leaf for the light, the wood and the drawing, which is what R5 is. Dormant with the group off or the size unset.

**Expected on paper:** sections 4.37 times wherever leaf sets them, so radii 2.1 times; the bend of a limb under its own weight goes with 1/r², about a quarter. The grower's shade deepens 4.37 times inside these crowns (their hazard is the beech's weak 0.169), so some interior shoots go. R7 then adds the beech's long shoots on top. The limbs should leave the ground and `ground_reach` should pass.

**What would refute it:** `ground_reach` still red; the Two Trees still a web on a thin trunk at 80; or their crowns hollowed by their own shade. If refuted, the builder returns with the sheet; the sag of a supernatural tree is the host's next question, not the builder's.
