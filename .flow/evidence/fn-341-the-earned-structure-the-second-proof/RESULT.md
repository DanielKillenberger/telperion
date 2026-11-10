# fn-341 result: the earned structure, the second proof

Drafted by the worker on 10 Nov 2026. The work behind it:
- `R1-MEASURES.md` and `R2-RULES.md`, answered in `R2-ANSWERS.md`;
- `R3-G-PAPER.md`, answered in `R3-G-ANSWERS.md`;
- `R3-RESULT.md`, answered in `R3-ANSWERS.md`.

R4 and R5 are below. Nothing further is proposed.

**SHEETS_STATUS**

## What the probe is

`probes/earned.patch`, switched by the environment (`TELPERION_RINGS`, `TELPERION_POSTURE`). The crates are unchanged.

- **G, girth by rings.**
  - **The rule.** Each year every section lays a ring: the standing leaf it carries above it, over R × N. A section is the sum of its rings and never loses one. It replaces every age's `pipe`, `ripening`, `retained`, `exponent` and `leaf_girth`.
  - **Traits** (the nearest measured values, not the species' own):
    - the oak's R is Phillips et al. 2003's two *Q. garryana* groves (0.23 and 0.37 m² cm⁻²), read at the tree's height;
    - the oak's N is European oak's sapwood rings by age (16, 20.4, 25.9; Haneca et al. 2009, Table 1);
    - the beech's R is Bartelink 1997's 0.331 m² cm⁻²; its N is Gebauer et al. 2008's 70 conducting rings by dye. Bartelink found no heartwood at all.
  - **Primary wood under the rings:** each phytomer's existing `established` pipe, an estimate.
- **P, posture by gravitropic performance.** The sag acts only on the share 1 − min(1, sin φ_max / sin φ), from Alméras and Fournier's eq. 39. Its coefficient, 1,241 m, comes from the paper's Fig. 8 angiosperm means, an estimate for every species. H is the straight chord to the far tip of what a section bears, and D is G's diameter. `straightening` and `erection` stay as they were.
- **U, upkeep on living wood, is in the patch and switched off.**
  - **Why:** it held twice the cycle's structure at 120 and outgrew 24 GB at 300 (`R3-RESULT.md`).
  - **The measured finding:** the cycle's only bound on how much structure a tree holds is the charge for wood already dead.
  - **What a rule needs:** real units on both sides of the carbon balance, what a leaf earns and what a volume of sapwood respires. That is a redesign of the balance (`R3-ANSWERS.md`), the next piece of the structure.
  - **What the sources state:** respiration scales with living cells (Ryan 1990) and with sapwood volume (Ryan et al. 1994). No text read says outright that heartwood does not respire.
- **Not in the probe:** vigour across orders, rank by vigour and height by path length.
  - **No rule here limits height or sets the crown's base.** Both are open.

## R1's measures before and after

Medians over seeds 1001, 1007 and 1012, from `raw/r1-*.log` (the cycle) and `raw/r3-*-GPr4*.log` (the probe), through `probes/r1.py` and `probes/ranges.py`. Distance is the model over the nearer end of the published range, 1 inside. Ranges and sources are as in `R1-MEASURES.md`.

### The oak

| measure | age | cycle | probe | published | moved |
|---|---|---|---|---|---|
| DBH at 1.3 m | 40 | 0.65 m (1.63) | **0.23 m** (0.82) | White 0.28–0.40; Phillips' 40-year *Q. garryana* 0.23 | **toward**: equal to Phillips' measured tree, below White's range |
| | 80 | 1.50 m (1.89) | **0.67 m** (inside) | 0.56–0.79 | toward, now inside |
| | 120 | 1.50 m (1.38) | **1.05 m** (inside) | 0.83–1.09 | toward, now inside |
| | 300 | 1.20 m (0.77) | **1.71 m** (inside) | 1.56–1.92 | toward, now inside |
| DBH over the years | 40→300 | rises, then falls | rises every age | "a constant non-reversible feature" (White) | toward |
| DBH over the *Q. garryana* ceiling | 40 / 80 / 120 | 2.53 / 2.90 / 1.94 | 0.89 / 1.29 / 1.36 | ≤ 1 | toward |
| height | 40 | 10.3 m | 10.3 m | forest values in range | unchanged |
| | 80 | 17.7 m | 18.0 m | | about unchanged |
| | 120 | 24.9 m | 26.2 m | forest 24 m at 95–135 yr | slightly away |
| | 300 | 40.0 m (1.15) | **51.9 m** (1.49; 42.1–56.3) | potential maximum 34.8 m; record 36.6 m | **away** |
| crown spread for its age | 40 | 7.7 m (inside) | 7.6 m (inside) | 7.2–9.6 | unchanged |
| | 80 | 20.4 m (1.21) | 19.2 m (1.14) | 12.7–16.8 | slightly toward |
| | 120 | 25.9 m (1.19) | 23.6 m (1.08) | 17.4–21.8 | toward |
| | 300 | 30.4 m (inside) | 24.9 m (0.85) | 29.3–34.6 | **away** |
| crown spread for the trunk | 40 / 80 / 120 / 300 | 0.54 / 0.72 / 0.92 / 1.28 | 1.25 / 1.31 / 1.11 / 0.79 | 1 (Pretzsch, open-grown *Q. robur*) | toward at 40 and 80; on the far side at 120 and 300 |
| junction section ratio, median | all | 0.99–1.00 | 1.00–1.00 | Leonardo, about 1 | inside both |
| lateral over parent diameter, median | all | 0.33 | 0.25–0.26 (0.20 at 40) | no checked value | unranked |
| limbs' base angle from vertical | 40 / 80 / 120 / 300 | 28° / 31° / 29° / 32° | 26° / 26° / 26° / 27° | *F. crenata* 10–50° (another genus) | unranked |
| the stated limb's chord, seed 1001, 80 | 80 | 41° | 18.5° (R3) | holds its growth angle (P's paper) | as on paper |
| lowest living lateral on the bole | 40 | 0.3 m | 0.3 m | open *Q. garryana* crown ratio 0.80 | unchanged |
| | 80 / 120 | 4.7 / 4.7 m | 4.7 / 4.7 m | | unchanged |
| | 300 | 23.2 m (2 seeds); first fork 20.9–29.6 m | 33.0 m (1 seed; none on the bole on 2); first fork 4.7–33.0 m | short boles (Silvics) | away on seed 1001; unread on two |
| living limbs from the stem | 40 / 80 / 120 / 300 | 5 / 5 / 3 / 1 | 5 / 5 / 3 / 1 | no count | unchanged |
| bole taper, cm per m | 40 / 80 / 120 / 300 | 2.0 / 0.1 / 0.1 / 0.3 | 1.2 / 0.5 / 0.4 / 3.0 | no checked number | unranked |

### The beech

| measure | age | cycle | probe | published | moved |
|---|---|---|---|---|---|
| DBH at 1.3 m | 40 | 0.29 m (0.90) | **0.037 m** (0.12) | White 0.32–0.48 | **away**: its trunk is what its leaf earns (20 m² above it at 40; its leaf area index is about a fifth of a real beech's, fn-340) |
| | 80 | 1.31 m (1.37) | **0.21 m** (0.34) | 0.63–0.96 | away |
| height, spread, limbs | 40, 80 | 9.8 / 17.3 m, 6.0 / 19.8 m, 8 / 11 | identical | — | unchanged: P is dormant on the beech (no sag), and G does not move its lay. The crown-base read differs (3.4 → 3.0 m at 40) only because the fork that bounds the bole is found by diameter |

**Toward real trees:**
- the oak's trunk at every age, now inside the published range from 80 to 300, and never thinning;
- the oak's crown spread for its age at 80 and 120;
- the oak's limbs, which hold their growth angle (P).

**Away:**
- the oak's height at 300, from 40 to 52 m against a 34.8 m potential;
- the oak's spread at 300, which falls below the range;
- the beech's trunk, which follows its too-small leaf.

**Unchanged:** the crown base and limb count, which no rule in this probe touches.

## Every other preset

At 40, seed 1001, whole view. No checked R and N for the spruce, the birch, the palm or the ordinary tree. The spruce's R is checked (0.36, McDowell et al. 2002) but its sapwood rings were seen only in an abstract snippet (Sellin 1991), and the birch has neither. So each runs on the cycle's girth under P alone, and the sheet says so. P is dormant where a preset has no sag.

## Time

One thread, seed 1001, one grow each (`probes/r4-runs.sh time`, `raw/r4-time-*.log`):

| oak | cycle | P alone | G and P | the ring step's share |
|---|---|---|---|---|
| 40 | 0.98 s | 0.97 s | 2.13 s | +1.16 s, 2.2 times |
| 120 | 31.9 s | 31.8 s | 74.2 s | +42.4 s, 2.3 times |

- **At 300, four threads:** 303 to 394 s for G and P, against 86 to 125 s for the cycle in R1. Peak 6.7 GB on seed 1001 and 11.3 GB for seeds 1007 and 1012 in one process; the cycle's oak at 300 peaks at 2.0 to 5.0 GB (fn-336, other seeds).
- **P costs nothing measurable.**
- **The ring step's cost is the probe's way of reading the leaf above:** a shed, scaled copy of the tree settled every year. The rule itself needs one sum carried up the tree a year, as the balance carries its sums. That was not built (`R3-ANSWERS.md` item 4).

## R5: the question for the owner

SHEETS_QUESTION

The verdict is the owner's, and only his written word counts.
