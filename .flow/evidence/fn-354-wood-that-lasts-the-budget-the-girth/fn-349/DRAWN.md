# fn-349: the product's trees drawn (BUILD-ANSWERS.md 5)

Builder's record, 11 Nov 2026, from the build at `868278f07`: the restated rule, under which a part earns what its paying children leave over. Every draw ran through `telperion-heavy`, sag on. Marks: **measured**, **eye**.

## The sheets

| sheet | what it shows |
|---|---|
| `drawn-oak-1001-bare.jpg`, `drawn-oak-1001-whole.jpg` | today (A), the probe's SR and the product (N): the oak at 40, 80, 120, 200 and 300 |
| `drawn-oak-1007-*.jpg`, `drawn-oak-1012-*.jpg` | the same three at 120, 200 and 300 |
| `drawn-others.jpg` | the beech at 40 and 80; the birch, spruce, palm and ordinary at 40; A, SR and N, leafless then leafed |
| `drawn-two-trees.jpg` | laurelin at 82 and telperion at 80, A beside N, supernatural off and on, whole and mid views |

- **Where the stills come from.** Today's and SR's are fn-345's and fn-346's, read from the fn-319 worktree. The Two Trees' A was drawn from the base commit (`f88a21c31`, built in `.flow/tmp/base`).
- **Scripts:** `draw.sh`, `sheets.sh` and `cost.sh`.
- **One tile is grey.** The product's oak at 200 on seed 1012 is refused, as an engine error: "axis 2363 (PA 6, born in cycle 135, base at 0.141 m) reaches 0.005 m below the ground" (measured). The same seed draws at 120 and 300.
- **Bare sheets.** The read below was taken from the leafed sheets and from `drawn-others.jpg`. The bare oak sheets were made but not viewed.

## Measures (measured)

**The oak, leaves and height, today / SR / product:**

| tree | today (A) | SR | product (N) |
|---|---|---|---|
| 1001 at 40 | 47,279; 10.6 m | 47,532; 10.6 m | 47,532; 10.6 m |
| 1001 at 80 | 209,845; 17.5 m | 227,393; 17.5 m | 227,955; 17.5 m |
| 1001 at 120 | 304,559; 25.1 m | 363,719; 24.7 m | 367,982; 24.7 m |
| 1001 at 200 | 270,545; 34.3 m | 97,454; 22.6 m | 97,398; 22.8 m |
| 1001 at 300 | 233,403; 40.2 m | 66,310; 8.3 m | 65,460; 7.7 m |
| 1007 at 120 | 386,363; 25.5 m | 599,507; 24.9 m | 607,369; 25.1 m |
| 1007 at 200 | 619,213; 37.1 m | 156,803; 23.0 m | 154,025; 22.9 m |
| 1007 at 300 | 94,815; 31.9 m | 102,800; 15.3 m | 108,586; 14.9 m |
| 1012 at 120 | 528,242; 23.9 m | 436,807; 23.5 m | 442,393; 23.5 m |
| 1012 at 200 | 210,219; 33.9 m | 198,824; 23.9 m | refused (below the ground) |
| 1012 at 300 | 379,622; 48.1 m | 160,626; 16.5 m | 161,507; 16.9 m |

**The other presets at 40 (and the beech at 80).** Each matches SR's leaf count: beech 16,614 and 480,147; birch 140,946; spruce 4,406,389; palm 6,554; ordinary 7,957. Heights match today's: 9.8, 17.4, 16.7, 12.4, 21.8 and 13.3 m. Against today's, the leaf counts move by +3.5% (beech at 80), +0.7% (birch), +5.3% (spruce), and 0 for the palm and the ordinary.

**The Two Trees** (nodes, leaves; today → product):

| tree | today | product | stands |
|---|---|---|---|
| laurelin at 82, off | 1,815,608 nodes, 499,591 leaves | 1,921,949, 519,025 | 17.8 m |
| laurelin on | 2,205,609, 620,464 | 2,225,941, 623,299 | 44.4 m |
| telperion at 80, off | 1,765,887, 480,151 | 1,671,245, 463,750 | 17.4 m |
| telperion on | 2,038,543, 574,023 | 2,022,408, 572,292 | 43.6 m |

Height and spread are unchanged on every Two Trees row. The census lines are in `raw/stills/*-N.log` and `*-A.log`.

**The census to 300** (`r5.sh`, `early live`, the record at each cycle's end; held phytomers and living tips every 20 years):

| year | 1001 held | 1001 tips | 1007 held | 1007 tips |
|---|---|---|---|---|
| 39 | 354,758 | 24,702 | 509,957 | 40,184 |
| 79 | 2,199,132 | 128,268 | 3,911,894 | 238,735 |
| 119 | 4,025,632 | 199,874 | 6,216,072 | 321,751 |
| 159 | 3,287,324 | 151,717 | 4,940,227 | 238,139 |
| 199 | 2,361,376 | 112,107 | 3,571,083 | 169,564 |
| 219 | 2,090,067 | 102,736 | 3,165,660 | 151,129 |
| 239 | 2,097,882 | 107,086 | 2,736,988 | 132,438 |
| 259 | 2,080,679 | 108,787 | 2,659,227 | 129,668 |
| 279 | 2,091,967 | 109,781 | 2,804,334 | 136,429 |
| 299 | 2,174,319 | 118,375 | 2,859,783 | 137,500 |

- **The steady state holds under the restated rule.** From 219 on seed 1001 and 239 on seed 1007, living tips stay within 15% and held phytomers within 8%; SR's living tips held at 103,000 to 117,000 and 136,000 to 139,000.
- **Not measured: height and longest path every 20 years.** The product's census does not carry them, and adding them would need code in the cycle. Heights at the drawn ages are in the table above.

**Cost, one thread, today (A) and the product (N), alternated in one session** (`cost.sh`, `early live`, grow and settle):

| oak, seed 1001 | today: seconds, peak GiB | product: seconds, peak GiB |
|---|---|---|
| 40 | 0.9, 0.20 | 1.0, 0.21 |
| 120 | 30.9, 1.62 | 35.8 (+16%), 2.12 (+31%) |
| 300 | 130.4, 2.54 | 117.9 (−10%), 1.99 (−22%) |

The product costs more at 120, where its crown is larger, and less at 300, where it holds less. That is SR's pattern: +17% at 120 and −11% at 300 against today's.

## The read per tree (eye)

- **The oak at 40 and 80 (1001):** the same as today's. At 80 the crown is a little broader, as SR's was.
- **The oak at 120 (1001, 1007, 1012):** the product is SR's tree. It is better than today's on all three seeds: a many-limbed crown clothed through its depth. On 1007 it is a broad full oak on a short bole where today's leans; on 1012 it is broad and even where today's is lopsided.
- **The oak at 200:**
  - **1001: worse than today's,** as SR was: straggly, a third of today's leaf, limbs down to the ground on one side.
  - **1007: neither better nor worse,** as SR: an upright 23 m tree, thinly clothed, with limbs clear of the ground. It is fuller in the crown than SR's.
  - **1012:** refused, where SR drew a dense 24 m bush.
- **The oak at 300:**
  - **1001: worse than today's,** and worse to the eye than SR's. The stem above about 8 m loops over in a full ring to the ground.
  - **1007: better than today's,** as SR was: a small upright old tree on a straight trunk, thin but standing, where today's is a twisted pole with a limb on the ground.
  - **1012:** a broad spreading old tree on a short leaning trunk, as SR's. It is not worse than today's, which is a 48 m bare pole with a crown at the top.
- **The other presets:** the same as today's to the eye, and identical to SR's.
- **Laurelin and telperion (no charge under the corrected rule):** the same as today's to the eye in every view, off and on. Node counts move by up to 6% each way, and the height and spread are unchanged.
- **Against SR:** the product draws SR's trees. The leaf counts agree within 1% at 40 to 120 and within 6% at 200 and 300. The 300 trees differ in their details, chiefly 1001's ring of stem. The restated rule changes the size of a failing part's balance, not its sign, and the trajectories part after 120.
