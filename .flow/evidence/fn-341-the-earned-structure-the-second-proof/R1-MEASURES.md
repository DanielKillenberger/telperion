# fn-341 R1: today's structure against published real trees

Drafted by the worker on 10 Nov 2026 at `da8e28194`, for the host. Nothing here is decided.

Marks:
- **measured**: a probe run, with the log named;
- **checked**: the number's sentence or table row was read in the source's own text, and is quoted in the sources table;
- **unchecked**: seen only in a snippet, an abstract or a secondary page;
- **inferred**: arithmetic on the above, done in `probes/ranges.py`.

Literature: the full-text reads came from a literature pass by a subagent and from the worker. The worker re-read three sources itself: White 1998's Table 1a, Table 2 and paragraphs 6 and 7, Gould et al. 2008's growth sentence, and Pretzsch et al. 2015's *Q. robur* crown row.

## How it was measured

- **The probe.** `probes/structure.patch` adds one ignored test, `pipeline::structure_probe_tests::structure_measures`. It grows the preset once through `telperion_space::grow`, the full cycle at fidelity 0, the tree the pipeline draws. It then reads the settled engine `Structure`: z up, metres, every phytomer's tip and pipe radius.
  - **Run.** Each preset and age ran once inside `telperion-heavy 8 --threads 4` (`probes/r1-runs.sh`). Logs are in the ignored `raw/r1-<preset>-<age>.log`, and `probes/r1.py` reduces them to the medians below.
  - **Seeds.** 1001, 1007 and 1012, as the spec asks.
  - **Reuse.** fn-326's, fn-339's and fn-340's breast-height read (`shape`, the widest node 1.0 to 1.6 m up) is kept beside the new reads as `dband`. Their `limb_count` was not used, because it counts systems on the drawn tree by start radius and says nothing about which wood is a limb. fn-332's `examples/trunk.rs` reads lean, not these measures, so it was not run.
- **The trunk** is the thickest wood from the root, taken at each junction across laterals, relays and continuations. The **bole** is that trunk up to its first junction whose second-thickest wood is half the thickest or more: the fork.
- **Where the trunk diameter is measured.** At 1.3 m, the pipe radius of the trunk internode that spans 1.3 m.
  - **No root flare in it (read).** The engine's radius carries no flare. The drawn mesh adds one in `pipeline/surface.rs` (`flare_radius`, default 2.1 times, over `flare_falloff`, default 0.022 of the height). Whether the oak's or beech's preset sets its own was not checked.
  - **fn-340's read agrees (measured).** The widest node in 1.0 to 1.6 m, on the pipeline's drawn tree, equals this engine read to the printed digit on oak seeds 1001 to 1003 at 40 (`raw/sanity2.log`: 0.578, 0.850, 0.641 m on both). It is at most 4% above the stem's own 1.3 m value, at 40, and identical from 80 on.
  - **Why fn-340's oak is thicker.** fn-340's 0.79 m at 40 and 2.0 m at 80 are medians over seeds 1001 to 1006. On seeds 1001, 1007 and 1012 the same read gives 0.65 m and 1.50 m. The cycle's trunk varies 1.6 to 2.3 times from seed to seed (fn-340).
- **Lowest living limb.** The lowest junction on the bole whose lateral carries any of this year's growth. The oak is deciduous on current-year wood, so "living" is "grew this year". The lowest junction with a lateral of 5 cm or more is read too.
- **Limb count.** Living axes of the limb physiological age (chain index 6) borne by a stem age (2 to 5). Also: living laterals off the bole, of any size and of 5 and 10 cm or more.
- **Limb angle.** For each lateral of 5 cm or more off the trunk: the chord over its first metre of thickest wood, relays included, from vertical and from the trunk's internode at the junction. The angle is binned by the junction's height over the tree's height. A limb is a chain of relay axes about 0.2 m long (`FRICTION.md`), so each limb's whole length and its tip's reach out and up are read too.
- **Junctions.** Every junction in wood of 2 cm or more. Each lateral's radius is read over the internode below the junction, and the summed section above (r²) over the section below.
- **Taper.** Centimetres of diameter lost per metre of bole, from 1.3 m to the fork.

## The table

Medians over the three seeds, with the per-seed values in `raw/r1-tables.txt`.

Ranges:
- **Oak DBH, *Q. robur* (checked).** White 1998's rule, run forward from Table 1a's three open-grown categories: champion, "good site, open grown, sheltered" and "average site, garden, parkland". Its stated method: even rings to the mature state, then rings of constant area.
- **Oak DBH, *Q. garryana* (checked).** The ceiling if every year grew at Gould et al. 2008's maximum predicted open-grown rate, 1.27 in per 5 years. It is an upper bound, not a range.
- **Distance.** The model over the nearer end of the range, 1 inside. Below 1 the model falls short.

### The oak (`oregon-white-oak`)

| measure | 40 | 80 | 120 | 300 | published | distance at 40 / 80 / 120 / 300 |
|---|---|---|---|---|---|---|
| DBH at 1.3 m, m | 0.65 | 1.50 | 1.50 | 1.20 | *Q. robur* open-grown: 0.28–0.40, 0.56–0.79, 0.83–1.09, 1.56–1.92 (White, checked). *Q. garryana* ceiling: 0.26, 0.52, 0.77, 1.94 (Gould 2008, checked) | **1.63 / 1.89 / 1.38 / 0.77**; over the *Q. garryana* ceiling 2.53 / 2.90 / 1.94 / 0.62 |
| DBH from one age to the next | — | rises | flat | **falls** (seed 1007: 2.07, 1.50, 1.06) | "Thickness of the stem is a constant non-reversible feature" (White ¶6, checked) | direction wrong after 80 |
| height, m | 10.3 | 17.7 | 24.9 | 40.0 (31.7–47.8) | *Q. garryana*, forest: 14 at 47–70 yr, 24 at 95–135 yr; mature on favourable sites 15–27; record 36.6; potential maximum on a good site 34.8 (Silvics, Gould 2011; checked) | inside / inside / inside / **1.15** (seed 1012 1.37 over the potential, 1.31 over the record) |
| DBH for height, *Q. garryana* measured trees | 0.65 m at 10.3 m | | | 1.20 m at 40.0 m | Corvallis, Oregon: 0.23 ± 0.01 m at 10 m and 40 years; 0.56 ± 0.05 m at 25 m and 250 years (Phillips et al. 2003, Table 1, checked; growing condition not in the read) | **2.8** at 40; 2.1 against the 250-year trees |
| height for DBH | 17.7 m at 1.50 m | | 24.9 m at 1.50 m | 40.0 m at 1.20 m | *Q. garryana*: "little additional height growth is predicted for trees > 60 cm" (Gould 2011, checked) | direction wrong: height goes on rising with the DBH past 1 m |
| crown spread, m | 7.7 | 20.4 | 25.9 | 30.4 | open-grown *Q. robur* crown for White's DBH at that age: 7.2–9.6, 12.7–16.8, 17.4–21.8, 29.3–34.6 (Pretzsch 2015 95% quantile, checked; inferred) | inside / 1.21 / 1.19 / inside |
| crown spread for the trunk | 7.7 m on 0.65 m | 20.4 on 1.50 | 25.9 on 1.50 | 30.4 on 1.20 | the same relation at the model's own DBH: 14.4, 28.3, 28.3, 23.7 m | **0.54** / 0.72 / 0.92 / 1.28 |
| junction section ratio (Σr² above / r² below), quartiles | 0.99 / 1.00 / 1.02 | 0.98 / 0.99 / 1.00 | 0.99 / 0.99 / 1.00 | 0.99 / 1.00 / 1.00 | Leonardo exponent 1.8–2.3 across species (Eloy 2011, citing others); *Fagus crenata* daughters "a little larger" than the mother (Minamino and Tateno 2014, checked) | inside |
| lateral over parent diameter, median (5 cm laterals: limbs) | 0.33 (limbs 0.56) | 0.33 (0.54) | 0.33 (0.65) | 0.33 (0.30) | no checked value for oak or beech | unranked |
| lowest living lateral on the bole, m (crown ratio) | 0.3 (0.97) | 4.7 (0.74) | 4.7 (0.81) | 23.2, one seed none below 29.6 (0.42) | open *Q. garryana* woodland, mean crown ratio 0.80 (Gould 2008, checked; a stand mean, not a range); open-grown trees "have short boles bearing very large, crooked branches" (Silvics, checked) | inside / 0.92 / inside / **0.52** |
| living limbs from the stem (limb age) | 5 (4–8) | 5 (4–7) | 3 (3–4) | 1 (1 on every seed) | no count found | unranked |
| living laterals on the bole, all / ≥ 5 cm / ≥ 10 cm | 12 / 10 / 8 | 2 / 2 / 2 | 2 / 2 / 2 | 2 / 2 / 2 | — | unranked |
| limb angle at the base from vertical, limbs (age 6) | 28° | 31° | 29° | 32° (8 of 9 dead, held until shed) | *Fagus crenata*, forest, ~30 yr: lateral angles 10–50°, basal slope 30.5 ± 15° (Minamino and Tateno 2014, checked; another genus, slope's reference not read) | unranked: no oak or beech value |
| angle from vertical by height, lower / middle / upper third | 43° / 31° / 46° | 26° / 50° / 60° | 26° / 61° / 61° | 28° / 70° / 53° | none checked | unranked |
| bole taper, cm per m | 2.0 | 0.1 | 0.1 | 0.3 | a taper model for Danish oak, no ratio read (Tarp-Johansen 1997) | unranked |

### The beech (`european-beech`)

| measure | 40 | 80 | published | distance |
|---|---|---|---|---|
| DBH at 1.3 m, m | 0.29 | 1.31 | *F. sylvatica* open-grown: 0.32–0.48, 0.63–0.96 (White, checked) | **0.90 / 1.37** |
| height, m | 9.8 | 17.3 | none checked for an open-grown beech | unranked |
| crown spread, m | 6.0 | 19.8 | Pretzsch's *F. sylvatica* row not read (unchecked) | unranked |
| junction section ratio, quartiles | 1.00 / 1.05 / 1.15 | 1.00 / 1.00 / 1.08 | as the oak's | inside |
| lowest living lateral on the bole, m (crown ratio) | 3.4 (0.65) | 4.5 (0.74) | none for open-grown beech | unranked |
| living limbs from the stem | 8 | 11 | none | unranked |
| limb angle at the base from vertical | 25° | 25° | *F. crenata* 10–50° (another species) | unranked |
| bole taper, cm per m | 0.3 | 6.7 (1.6 to 17.7 across seeds) | none | unranked |

## Ranked by distance from the published range

1. **The oak's trunk at 80: 1.9 times the open-grown *Q. robur* upper end, 2.9 times the *Q. garryana* ceiling.** At 40 the figures are 1.6 and 2.5 times, and at 120 1.4 and 1.9 times. On fn-340's seeds (1001 to 1006) the figures are 2.0 times at 40, 2.6 at 80 and 1.8 at 120 the *Q. robur* upper end. Against the one measured *Q. garryana* of its age and height (Phillips 2003: 0.23 m at 10 m and 40 years) the oak at 40 is 2.8 times. The host's belief of "two to three times" holds against *Q. garryana* and against *Q. robur* on fn-340's seeds. It holds within 1.4 to 1.9 times on R1's seeds. [measured; inferred]
2. **The old oak's crown base: crown ratio 0.42 at 300, against 0.80 in an open oak woodland.** Only one limb of the limb age is still alive at 300 on every seed. The bole runs bare to 21 to 30 m, which is a forest tree's form. Silvics says open-grown trees have short boles. The 0.80 is one stand's mean, so this distance is weaker evidence than the trunk's. [measured; checked source]
3. **The trunk thins with age: 1.50 m at 80 and 120, 1.20 m at 300, falling on seed 1007 from 2.07 to 1.06 m.** At 300 it ends 0.77 times the lower end of White's range. Real stems only thicken (White ¶6). This is a direction, not a size. [measured]
4. **The beech's trunk at 80: 1.37 times the upper end**, after 0.90 times the lower end at 40: thin at 40, thick at 80. [measured]
5. **The crown spread for the trunk: 0.54 at 40.** It is a consequence of item 1. For its age, the spread sits inside the range at 40 and 300 and 1.2 times it at 80 and 120. [measured; inferred]
6. **The oak's height at 300: 1.15 times the potential maximum** (seed 1012: 47.8 m). Height goes on rising past 1 m of DBH, where *Q. garryana* levels off past 0.6 m. Height for age is inside the forest values at 40 to 120. [measured]
7. **Inside:** the junction section ratio for both species, and the oak's spread for age at 40 and 300.
8. **Unranked, no checked number:** limb count, limb angle by height, bole taper, a lateral's diameter over its parent's, and the beech's height, spread and crown base. Each is named in "Measures with no source".

## What sets each measure today

**Prescribed** means a timer, a set angle, or a value chosen by eye or fitted to a target. **State** means it follows from the tree's present state. File references are to `crates/telperion-space/src/`.

- **Trunk diameter (girth).** `girth.rs` is the pipe model. Each phytomer adds its age's own pipe (`own_at`: `Form::pipe` × scale × ripened over `Form::ripening` years × the leaf factor). The sections are summed upward at each age's `Form::exponent` (`thicken_given`). The section counts only wood held at the settle: a shed branch takes its pipe with it (`PaState::retained`, 0 on the oak).
  - **State:** the sum over the wood the tree holds, and `leaf_girth` (χ 1 on the oak), which thickens lit wood against the tree's mean.
  - **Prescribed:** the per-phytomer pipes and exponents. The oak's trunk is 0.009 at 2.6, its limb 0.009 at 2.8 with 15-year ripening, its bough 0.004 at 2.6; the beech's trunk and reiterates are 0.008 at 2.6. `SOURCES.md` marks them "Estimated from the stills; dbh 0.6 to 1.0 m at 80 years".
  - **Fitted:** the oak's `retained` 0, chosen because "at 0.5 the stem stood two to four times Silvics' DBH" (`oak.rs`).
  - **Why it thins, and why the girth follows wood rather than leaf (inferred from the code).** The radius is the sum of what is held now. Shedding a limb takes its whole pipe away, so the trunk thins. A phytomer adds pipe whether or not it bears leaves.
- **Height.**
  - **Prescribed:** each age's internode and nodes per unit; the oak's trunk age's 12-year life (`TRUNK` lifespan 12), the fork's 2 and the leader's 6; then the limb age, which lives 500 years at a constant internode. Nothing limits height.
  - **State:** light scales a unit's size against its own age's mean (`shade_size`, normalised within each age; `allocation.rs`), so light never moves growth from one age to another.
- **Crown spread.** The limb and bough rows: internode, nodes, `elevation`, `tropism`, `insertion`, `straightening` and `sag`, all estimated. `shade_size` is 0.5 on limbs and boughs. Limbs and boughs are lost by the light hazards (`shade_hazard`, `balance_hazard`, `upkeep`: state, with rates chosen).
- **Junction ratios.** State: the pipe model's sum. Prescribed: its exponents, 2.0 to 2.8. At those exponents the sections across a junction sum close to 1.
- **Lowest living limb.**
  - **Prescribed:** the fork's height, by the trunk age's timer. Twelve years at 0.32 to 0.40 m a year puts it near 4.5 m whatever the light (RETHINK). The bole's laterals are the sprig age, with a 10-year life and `viability` 0.95.
  - **State:** clearing, by the shade and balance hazards. At 300 they have taken all but one fork limb.
- **Limb count.** Prescribed. The fork age lasts 2 years, and each year's top cluster of 3 buds grows limbs at p 0.8, about 4.8 expected. The leader adds about 1.4 more over its 6 years (3 buds, p 0.08). After that, shedding by state.
- **Limb angle.**
  - **Prescribed:** `insertion` (the oak's limb 0.5 rad from its bearer), `elevation` (1.05 rad), `tropism` 0.5, `straightening` 0.15 and `wander`. `SOURCES.md`: "angles estimated (MODEL-OAK: no source gives an oak angle)".
  - **Sag** (`Form::sag`, 3·10⁻⁴ on the limb) is a beam bent once at the settle, about 80 times an elastic beam's (fn-338 R1-NOTE). Growth never reads it, and nothing holds or loses an angle by the wood's state.
  - **What that gives (measured):** the limbs' base angle is 25 to 31° from vertical at every age. Their whole chord leans further out: at 80 the median limb is 11.6 m long, with its tip 5.5 m out and 7.3 m up.
- **Taper.** State: the pipe model. Below the fork the bole bears almost nothing, so from 80 on it is a cylinder (0.1 cm per m). At 40 it is 2 cm per m while the sprigs still stand on it.

## Measures with no source

- A lateral's diameter over its parent's at the junction, oak or beech. Only *F. crenata*'s area ratios were read.
- The count of first-order limbs on a mature oak or beech: one search found only pruning guides.
- Limb angle by height in the crown, oak or beech. Kint et al. 2010 (young planted *Q. robur* and *F. sylvatica*) has none in its abstract. Juchheim et al. 2017 was paywalled. Bayer et al. 2013 was read without finding angles. *F. crenata*'s 10 to 50° stands in only.
- Crown base for open-grown beech. Hasenauer 1997's abstract did not load.
- A taper ratio for oak or beech. Tarp-Johansen et al. 1997 give a model form for Danish oak, and its coefficients were not read.
- Pretzsch et al. 2015's *F. sylvatica* crown row: two extraction passes missed it.
- Beech height and DBH at age outside White's core table.

## Sources

Each row is checked unless marked.

| source | what was read | the source's words |
|---|---|---|
| White 1998, Forestry Commission Information Note 12, https://cdn.forestresearch.gov.uk/1998/01/fcin012.pdf (read by the worker) | Table 1a, open-grown rows; Table 2; ¶4, ¶6, ¶7 | "Good site, open grown, sheltered \| 80/4 \| … \| 70/5" (oak, beech); "Champion tree potential … \| 70/5 \| … \| 80/6"; "Average site, garden, parkland \| 100/3.5 \| … \| 70/4"; "annual rings in the stem which are of the same cross-sectional area but they progressively decline in width"; "Thickness of the stem is a constant non-reversible feature of tree growth in so far as it has to increase each year that the tree lives"; "Provided there are no branches, swellings, buttresses or abnormal lumps, girth should be measured with a tape at breast height (1.3 m …)" |
| Gould, Marshall and Harrington 2008, West. J. Appl. For., https://research.fs.usda.gov/download/treesearch/29814.pdf (read by the worker) | growth equation result; stand 1 | "The maximum predicted diameter growth of an open-grown oak (SI, 100) is 1.27 in./5years"; "dbh has a curvilinear relationship with the maximum predicted growth at 18.3 in."; "Stand 1 was an open oak woodland … mean CR of 0.80, and mean diameter of 16.3 in." |
| Pretzsch et al. 2015, Urban For. Urban Green. 14: 466, https://repositorio.uchile.cl/bitstream/2250/135852/1/Crown-size-and-growing-space.pdf (row read by the worker) | Table 2 row, ln(cr) = a + α ln(d), cr in m, d in cm, 95% quantile | "Quercus robur \| 4839 \| -1.4370 \| 0.0353 \| <0.0001 \| 0.8157 \| 0.0103 \| <0.0001 \| 3.3"; "the species specific expected open-grown crown radius at a stem diameter of 25 cm, as estimated from the fitted allometric equation for the 95%-quantile" |
| Stein 1990, Silvics of North America, Oregon white oak, https://research.fs.usda.gov/silvics/oregon-white-oak | text (OCR prints "in" for m) | "Under favorable conditions, mature Oregon white oak trees are 15 to 27 in (50 to 90 ft) tall and 60 to 100 cm (24 to 40 in) in d.b.h."; "A maximum height of 36.6 m (120 ft) …"; "Typically, open-grown trees have short holes bearing very large, crooked branches that form dense, rounded crowns" |
| Gould, Harrington and Devine 2011, Northwest Sci. 85: 159, https://research.fs.usda.gov/download/treesearch/39910.pdf | text (OCR) | "potential height was found to increase with DBH at an almost constant rate for trees < 30 cm DBH and then level-off so that little additional height growth is predicted for trees > 60 cm. The maximum potential height for OWO on a good site was 34.8 m" |
| Minamino and Tateno 2014, PLoS ONE 9: e93535 (PMC3979699) | text | "The sum of the cross-sectional areas of daughter branches was a little larger than the cross-sectional area of the mother branch at most branching points in Fagus crenata"; "the angles are 50–80° for Abies homolepis and 10–50° for Fagus crenata"; "The slope at the basal section of the Fagus crenata branches was 30.5±15.0°" |
| Eloy 2011, Phys. Rev. Lett. 107: 258101 (arXiv:1105.2591) | text; the number is Eloy's citation of others | "the available data indicate that the Leonardo exponent is in the interval 1.8<Δ<2.3 for a large number of species" |
| Phillips, Bond, McDowell and Ryan 2003, Funct. Ecol. 17: 832 (treesearch 9298) | Table 1 | *Q. garryana*, Corvallis: 250-year-old trees 25 m tall, 0.56 ± 0.05 m at 1.3 m, leaf to sapwood area 0.37 ± 0.01 m² cm⁻²; 40-year-old trees 10 m tall, 0.23 ± 0.01 m, 0.23 ± 0.02 m² cm⁻² |
| Paine and Hann 1982, OSU Res. Pap. 46 | table row; units not stated in the read, so **unchecked** for use | "Oregon white oak \| 3.0786 \| 1.9242 \| 0.0" |
| Mitchell's 2.5 cm of girth a year, via a veteran-tree page | **unchecked** (secondary) | — |
