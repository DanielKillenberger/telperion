# fn-190 R1: pass bands for the beech, oak and spruce, 2026-10-03

Written before tuning, for Astra's review. Measures are `score.rs` (probe9) as R3-PROBE9 defines them; H is the top node's height. "lat/m" and "generations" are R3-PROBE8's: laterals per metre of non-structural wood, and lateral starts counted from structural wood along each terminal path.

**Viewed (7 images, each with a 100 px grid drawn on a scratch copy; pixel readings are in the original file's pixels, read by eye from the grid, about ±15 px):** beech `S1-entzia-open-winter`, `S1-rostock-copper-winter`; oak `S1-robur-field-november`, `S1-robur-old-open-winter`, `S2-garryana-wheatfield-leaf`; spruce `S1-lawn-solitary-comb`, `S2-beskid-meadow-brush`. No S3–S5 image was viewed, so secondaries, junction ratios and fine-wood counts rest on S1 readings, literature or model inference. Preset envelopes (`packet/species.json`): beech H 32 m, spread 0.5 (W/H 1.0), crownBase 0.12; oak H 24 m, spread 0.55 (W/H 1.1), crownBase 0.16, 5 scaffold limbs; spruce H 15 m, spread 0.31 (W/H 0.62), crownBase 0.04, 16 tiers × 5.

**Photo readings (base y → top y, H in px; landmark fractions upward from the base):**

| photo | base / top / H | lowest substantial limb | 2 comparable stems from | W/H | caveat |
|---|---|---|---|---|---|
| beech Entzia | 1120 / 125 / 995 | y 915 → 0.21 | y 800–860 → 0.26–0.32 | 1.3 (x 600–1900) | wind-leaned, flattened on the right |
| beech Rostock | 1707 / 15 / 1690 | y 1220–1230 → 0.28 | y 800–860 → 0.50–0.54 | ≥ 0.9 (cut at both sides) | strong upward view inflates lower fractions (true values lower, estimated 0.25 and 0.45); buildings overlap the lower right |
| oak robur field | 1186 / 126 / 1060 | y 990 → 0.18 (a thin drooping lateral at 0.12) | y ~1000 → 0.18 (two codominant stems) | 1.07 | lower branch tips hang to 0.05 H |
| oak robur old | 1236 / 164 / 1072 | y 1000 → 0.22 | y 950 → 0.27; a central stem persists to ~0.65 H | 1.21 | flat light; tips hang to 0.13 H |
| oak garryana (S2, in leaf) | 704 / 14 / 690 | ~y 500 → 0.30 | y 500 → 0.30 | 1.22 | foliage hides lower laterals; leaning trunk |
| spruce S1 comb | 1577 / 6 / 1570 | skirt to the ground; lowest attachment ≈ y 1490 → 0.06 | none: one stem to the tip | 0.66, widest at 0.24 H | wall and pole behind the lower crown |
| spruce S2 brush | ~1670 / 300 / ~1370 | crown to the ground (≈ 0) | none | 0.57 | slope hides the stem base |

## European beech (Troll; Fagus sylvatica)

| measure | band | source | confidence | note |
|---|---|---|---|---|
| clear bole (lowest substantial trunk lateral / H) | **0.18–0.30** (spec 0.18–0.28, widened up) | Entzia 0.21; Rostock 0.28 image (≈ 0.25 true); Astra 0.20–0.23 on fasy896 | medium | Widened to admit Rostock's park-grown bole. Rejects the probe balls (0.07–0.08). Preset crownBase 0.12 sits below the band. |
| division (`leaders2` / H) | **0.28–0.50** (spec 0.30–0.40, widened up) | Entzia 0.26–0.32; Rostock 0.50–0.54 image (≈ 0.45 true); Astra 0.30–0.39 | medium-low | The two S1 trees disagree: Entzia breaks up near the bole top, Rostock carries one stem to mid-height. Rejects the balls (0.09). Does not separate a comb (base A 0.37); the secondaries row does. |
| major axes (union of root04, local04) | **4–7** (spec 4–6, widened by one) | Entzia ~5 traceable limbs; Rostock 6–8; Astra 4–6 | medium for root04, low for the union | local04 counts every lateral ≥ 0.4 of the local trunk, including those off a thin upper leader: today's probes have union ≥ 9 with root04 6. If the union cannot fall to 7 while the image reads right, score root04 against 4–7 instead and report local04 unscored (open question 1). |
| substantial secondaries per major (median) | **4–10** (spec 5–10, lower end widened) | Entzia right-hand limbs show ~4–8 readable secondaries; Astra 5–10 | low | Rejects the comb (27). Not a topology quota. |
| secondary length / parent remaining (p50) | **0.3–0.8** (kept) | Astra on fasy896; not re-measurable on S1 at this scale | low | Shipped beech 1.52 fails it (secondaries longer than what remains); probes 0.53–0.70 pass. |
| shell / interior fine wood | **≥ 1.3** (kept) | both S1: twig haze at the periphery, dark limbs readable inside | direction high, number low | 1.0 is uniform density. The 1.3 figure is not photo-measured; probes reach 0.9–1.07, shipped 0.19. |
| upper / lower fine wood | **≥ 1.1** (kept) | Rostock's upper crown visibly denser; Entzia's lower spray more open | direction medium, number low | Probes 1.00–1.09. |
| first-order junction ratio (median, axes ≥ 1 m) | **0.35–0.60** (spec 0.4–0.6, lower end widened) | Entzia limbs at the bole top ≈ 0.4–0.6 of trunk width; Rostock's lowest limbs ≈ 0.3–0.45 | low (limbs are 5–20 px wide) | The median includes small trunk laterals high in the crown, which pulls it below the limb reading. Rejects shipped (0.16). |
| laterals per m of fine wood | **≥ 1.3** (kept) | shipped beech diagnosis 1.57 (R3-PROBE8); beech short shoots never branch, long shoots branch acrotonically [DTT86, LET] | low | Not visible at S1 scale. Probes 0.77–1.05 fail. |
| generations p50 | **≤ 3** (kept) | shipped 1/3; mature Fagus has 3 axis categories, young 5 [M98] | low | Depends on the structural / fine boundary, which the new law may remove (open question 2). |

## Oak (Rauh; Oregon white oak preset, Q. robur S1 photographs, Q. garryana S2)

| measure | band | source | confidence | note |
|---|---|---|---|---|
| clear bole | **0.14–0.32** | robur field 0.18; robur old 0.22; garryana ≈ 0.30 (in leaf) | medium | Preset crownBase 0.16 is inside. Lower branch tips hang below the bole top in both robur photos; the measure takes attachment, so they do not count. |
| division (`leaders2` / H) | **0.15–0.35**, and division − clear bole **≤ 0.10** | robur field 0.18; robur old 0.27; garryana 0.30 | medium | The oak breaks into comparable limbs at or just above the crown base, unlike Rostock's beech. The gap condition uses only measures 1 and 2. |
| major axes (union) | **4–7** | robur field: 2 codominant stems + ~3 limbs; robur old ~5–6; garryana ~4–5; preset 5 scaffold limbs; Rauh branches equivalent to the trunk [T83, K19] | medium | Same local04 caveat as the beech. A persistent central leader is allowed (robur old) but not required. |
| substantial secondaries per major | **3–10** | model inference: mature oaks fork and reiterate [K19]; limbs carry a few heavy forks rather than many laterals | low | No S3 viewed. |
| secondary length / parent remaining (p50) | **0.4–1.0** | model inference (Rauh: branches morphogenetically like the trunk, forks comparable to their continuation) | low | Higher than the beech: oak limbs split rather than throw side branches. |
| shell / interior fine wood | **≥ 1.1** | robur field: twigs fill the interior too, denser at the edge; robur old: edge haze | low | Weaker gradient than the beech; oak crowns stay full inside. |
| upper / lower fine wood | **0.9–1.5** | robur field dense top, robur old spreading low limbs carry much spray | low | A sanity band; does not separate a ball. |
| first-order junction ratio | **0.40–0.70** | garryana right limb ≈ 0.6–0.7 of the trunk; robur old left limb ≈ 0.5 | low-medium | Heavier limbs than the beech. Rejects a comb of thin laterals. |
| laterals per m of fine wood | **≥ 1.3** | model inference: acrotonic clusters of short laterals at each annual shoot's tip [T83]; robur README S4 describes short laterals each ending in a bud cluster | low | Borrowed floor from the beech; no oak-specific count exists. |
| generations p50 | **≤ 3** | model inference, as the beech | low | Same boundary caveat. |

## Norway spruce (Massart; Picea abies)

| measure | band | source | confidence | note |
|---|---|---|---|---|
| clear bole as defined | **not scored** | — | — | Spruce branches are ≈ 0.1–0.25 of the stem, so no trunk lateral meets "substantial" (ratio ≥ 0.3) and the measure returns 1.0 for a correct tree. |
| lowest lateral (`t1_bole.lowest_any_lateral`, already output) | **≤ 0.10** | S1 ≈ 0.06; S2 ≈ 0; preset crownBase 0.04 | medium-high | Open-grown spruce keeps a skirt to the ground. Stands in for the clear bole. |
| division (`leaders2` / H) | **none below 0.85 H** (null, or ≥ 0.85) | S1 and S2: one stem to the tip; Massart monopodial orthotropic trunk [PR00, CIR] | high | Near the tip the stem thins and the top whorl may reach half its radius, so 0.85–1.0 is allowed. |
| major axes (union) | **0–1** | stem ≫ branch in both photos; Massart | medium | Under the 0.4 rules a correct spruce has none. Two or more means the law grows broadleaf limbs. |
| substantial secondaries per major; length / remaining | **not scored** | — | — | No majors to measure along. |
| shell / interior fine wood | **≥ 1.2** | both photos: foliage and shoots on the outer branch ends, bare branch wood inside | low | The star-shaped crown from mid-height fits a cone; the narrow tip bins are noisy. |
| upper / lower fine wood | **0.7–1.6** | open-grown skirt (S1, S2) | low | Sanity band only. |
| first-order junction ratio | **0.08–0.25** | S1: lower branches ≈ 0.2 of the stem width; model inference (strong apical control, 5 branches per whorl sharing the pipe) | low-medium | Rejects a broadleaf habit (≥ 0.35). |
| laterals per m of fine wood | **≥ 1.3** | spruce branch sprays carry second- and third-order shoots in one plane (README S5) | low | Borrowed floor; no spruce count exists. |
| generations p50 | **≤ 3** | README S5: branch → 2nd → 3rd order shoots | low | Same boundary caveat. |
| **added: first-order branch elevation** | **median −5° to +30°**, and share above 45° **≤ 0.15** | S1 chords of three edge branches: +15°, +21°, +24°; S2 stiff, near-horizontal; Massart plagiotropic branches | medium | Definition: for every lateral whose parent node is on the trunk axis and whose axis length is ≥ 0.05 H, the chord from the parent node to the axis's last node; elevation = atan2(Δy, horizontal length of the chord), in degrees. Median and share > 45° over those laterals. Catches an oak-like spruce whose branches ascend. |

## Open questions

1. **Majors, union or root04.** local04 counts laterals off a thin upper leader, so the union reads 9–15 on today's probes against a visual count of 5–8. Should the beech and oak band apply to root04 alone? That is a scorecard design choice for the host.
2. **Generations and lat/m need a fine-wood boundary.** Both count from structural wood. If the one law removes the scaffold, "structural" must be redefined (for example by birth radius against the root) or these two measures lose meaning. Host decision.
3. **No S3–S5 view was used.** Secondaries per major, length ÷ remaining and junction ratios for all three species rest on S1 readings at 5–20 px per limb or on model inference. One S3 per species would firm them up.
4. **The beech division band** spans two different trees (0.28–0.50). If the owner wants the Entzia habit (early break-up) rather than Rostock's (one stem to mid-height), the band narrows to 0.28–0.40.
5. **Envelope width is not on the list.** Entzia reads W/H 1.3 against the beech preset's 1.0; the oak (1.07–1.22 against 1.1) and spruce (0.57–0.66 against 0.62) agree with their presets.
6. **Fine-wood numbers have no photographic anchor** for any species (shell/interior 1.3, lat/m 1.3). They are directions with guessed thresholds, and a run that fails only these should be judged on the sheets.
