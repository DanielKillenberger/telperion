# fn-264 R1a and R2: sources, Jev's choices and the checks (worker, 2026-10-12)

Every value below was cut from a fetched source, offered to Jev as one candidate among every number in its passage (with `none`), copied by code from the passage bytes, and then checked by the worker against the fetched source text. `jev select` recalls lengths and ages only (see the spec's Friction), so the candidates came from a scratch extractor run through `jev ask`. Ledger entries are under `ledger/` (ignored); fetched sources stay in the session scratchpad.

## Shipped

| Row | Value | Source, age and condition | Jev | Checked |
|---|--:|---|---|---|
| site `extinction` (all presets) | 0.5 | Bréda 2003, J. Exp. Bot. 54: 2403, on individual tree crowns (leaf area density x path): "k equals 0.5 if leaf-angle distribution is assumed to be spherical (Ross, 1981)". Species-free. | `fn264-pick:18ddda653291e939-3dbf09-0`, 1.00 | yes; the oak's 0.5, "cited from memory" in fn-197 STEP3.md, holds |
| beech `leafArea` (and the Two Trees) | 0.0015723 m² | Hladká and Čaňová 2005, J. For. Sci. 51(4), Table 2: mean of 25 sunlit leaves, 15.723 cm², 1998; model tree 80 years, 35 m, mixed beech-fir-spruce stand at Poľana, 850-860 m. Shaded leaves 21.648 cm² (`fn264-pick:18dddb168cb357ec-3e6648-0`). | `fn264-pick:18dddb1675e2f05d-3e652d-0`, 0.99 | yes |

- **One leaf a node.** The beech module already bears one leaf a node (inferred, fn-193); its short shoot's cluster of four stands for its three to five metamers' leaves, which the same source supports ("There were mainly 4 leaves on shoots, 7 to 9 leaves on terminal ones", `fn264-pick:18dddb16a58d9b75-3e677f-0`). So every living age carries one leaf's area, as the oak's do.
- **Sunlit, not shaded** (worker's reading, open to the host): the source has no whole-crown mean; the generator draws an open-grown crown, which is sunlit.
- **The Two Trees take the beech's rows** (decision named): their engine species is `beech()` (fn-218, fn-219; `tests/space_presets.rs`), "with the group off he is that beech, very old" (`telperion.values`). Their nearest real analogue is therefore the beech, and its leaf area reaches them with no row of their own.

## Shade tolerance (candidates for the response rows, not shipped)

Niinemets and Valladares 2006, Ecol. Monogr. 76: 521, Appendix A, 0 (none) to 5 (maximal), species-level:

| Species | Score | Jev |
|---|--:|---|
| *Fagus sylvatica* | 4.56 ± 0.11 | `fn264-pick:18dddb17a32041f1-3e698a-0`, 1.00 |
| *Picea abies* | 4.45 ± 0.5 | `fn264-pick:18dddb33049974a2-3e8332-0`, 1.00 |
| *Quercus robur* (the oak's engine species) | 2.45 ± 0.28 | `fn264-pick:18dddb17ba557bcd-3e6a8f-0`, 1.00 |
| *Quercus garryana* (the oak preset's name) | 2.4 ± 0.3 | read in the same table, not judged |
| *Betula pendula* | 2.03 ± 0.09 | `fn264-pick:18dddb15cf3d5bc7-3e6351-0`, 1.00 (1.85 beside it is drought) |
| *Phoenix dactylifera* | no score; "very bright" (FAO Ecocrop), "requires full sun" (UF/IFAS FR314) | none: the extractor offers numbers only |

## Not shipped: waiting on the host

- **Birch leaf area.** No fetchable source states a measured mean area of one *B. pendula* leaf in its authors' text. The design that would (Hagemeier and Leuschner 2019, Forests 10: 265, stands of 45 and 67 years) gives leaf size only in its Figure 1; a scraper's table of 10.0 / 8.8 / 7.2 / 6.3 cm² by crown height is reconstructed from that figure, not the authors' numbers. Akinyemi et al. 2024 (Trees) likewise in a figure, 5-year-old trees. Paywalled and likely to hold it: Kull and Niinemets 1993 (Tree Physiol. 12: 311), Niinemets 1994 (For. Ecol. Manage. 70), Atkinson 1992 (J. Ecol. 80). The short shoot's "two or three leaves" is sourced (Kull et al., Ann. Bot., PMC4242242), so one leaf a node holds as for the beech once an area is.

- **Spruce leaf area.** No source gives needle area per length of current-year shoot. Candidates: 18 to 31 needles per cm of shoot (Bellan et al. 2019, Austrian J. For. Sci., a closed 36-year stand, the whole crown profile; `fn264-pick:18dddb318255f8d1-3e7e9f-0`), and a projected needle area of 13.7 mm² sun, 8.2 mm² shade (Gebauer et al. 2019, iForest 12: 27, 20-year-old trees, control plots; `fn264-pick:18dddb319ab80e89-3e7fab-0`, `...31b39f0079-3e8129-0`). A value needs a point inside a range and a product of two stands. Needles live 5.5 years to half a cohort (Muukkonen and Lehtonen 2004; `fn264-pick:18dddb331ff24320-3e8459-0`), but the deposit counts only the nodes laid this year, so a current-year value lends the lattice about a fifth of the crown's needles.
- **Palm leaf area.** The one measured frond area is 2.81 ± 0.64 m² (Abdel-Sattar et al. 2024, Folia Hortic., 15-year-old male palms, drip-irrigated, 8 m apart; `fn264-pick:18dddb39ffe7316b-3e89b7-0`, 0.76), computed as length x width x 0.8, an outline rather than leaflet area. A second-hand "about one square meter" per leaf (Montazar et al. 2020, Water 12: 2253, citing a USDA bulletin). The engine refuses more than 1 m² a node (`MAX_LEAF_AREA`), so 2.81 cannot be drawn without an engine change.
- **Every response row of every species** (shade hazard, shade size, apical control, upkeep, balance hazard, tolerance, leaf girth). No source gives these model coefficients for any species; the oak's are its own values from fn-195 round 5 (host decision 29), not literature. A sourced shade tolerance (above) could set them only through a mapping from a 0-5 score to the engine's coefficients, which is a design decision.

## Host decisions (2026-10-12) and what each met in the code

1. **Birch leaf area: derived** from an open flora's leaf length and width times a stated shape factor for an ovate-rhombic leaf, labelled derived, dimensions quoted and checked. The paywalled papers above stay listed as better sources. Basis: no measured per-leaf mean is fetchable. Shipped: one leaf a node of 13.19 cm² (`birch.rs`, `LEAF_AREA`), the mid-points (as decision 2 takes the spruce's) of Flora of China 4's "leaf blade triangular-ovate or rhombic-ovate, 3-7.5 × 1.5-6 cm" (eFloras, `fn264-pick:18ddde5f1ac656e6-21e2c-0` and `...5f2fe0d943-21f65-0`, 0.95 each) times 0.67, "Ovate (sensu stricto) ... Ovate leaf with proximal two-fifths with L : W < 3 : 1" (Schrader et al. 2021, Ann. Bot. 128: 395, Table 1; area over length x width; `fn264-pick:18ddde5f47692fa6-220ab-0`, 1.00). Both checked against the fetched text. Readings for the host: the source gives k by shape class, not for *B. pendula*; "ovate sensu lato" is 0.68. Flora of North America's "3--7 × 2.5--5 cm" was seen in search only.
2. **Spruce leaf area: derived**, the mid-point of Bellan et al.'s 18 to 31 needles per cm times Gebauer et al.'s needle area. Shipped: 24.5 x 100 x 13.7 mm² = 0.033565 m² a metre of current-year shoot, times each age's internode (`spruce.rs`, `NEEDLE_AREA`); the sun needle (13.7 mm²) was used, as the beech's sunlit leaf was. Basis: the two stands are the fetchable measurements.
   - **The retention ruling was not built (stopped, reported).** The host ruled that the deposit count retained foliage by the drawing's retention (fn-248's `leaf_years`) if that is one contained rule. It is not. The drawing bears a leaf where the wood's retention keeps it *and* the wood is within the slender bound (`foliage/plan.rs:484`, a girth threshold read from the finished tree's radii), and the cycle has no finished radii. The oak, birch, spruce and palm state `leaf_years` infinite, so a deposit read from retention alone would put leaf area on every phytomer they ever grew, trunks included. The spruce's 5.5-year needle life (Muukkonen and Lehtonen 2004) is in no row. The deposit therefore still counts the year's needles alone.
3. **Palm leaf area: not built (reported).** The ruling divides the frond's 2.81 m² by its 196 ± 12 pinnae (Abdel-Sattar et al. 2024, Table 2) and deposits that per leaflet. But `leaf_area` is the area a *node* bears, the deposit lends it once per phytomer laid (`grow/draw.rs:253`), and a palm phytomer is one frond (`palm.rs`). Per leaflet, a frond would lend 0.0143 m², 1/196 of itself. The frond's own area stays above `MAX_LEAF_AREA` (1 m² a node).
4. **Response rows:** one monotone mapping from Niinemets and Valladares's T, anchored on the oak at *Q. garryana*'s T = 2.4: each hazard-type row is the oak's x (5 - T) / 2.6, and the size response scales the same way. The Two Trees take the beech's T (4.56). The palm's rows stay zero: it has no laterals to shed and no score, and its single axis makes them dormant. Basis: no source gives the coefficients, and the owner judges the calibration on the stills. **Not yet applied (reported):**
   - The oak's shade size differs by age (1.5 at ages 2-4 and 8, 0.5 at 6-7, 1.0 at 9 and 12-14), and the spruce lives in ages 0, 1, 10 and 11 and the beech and birch in age 5, where the oak has no value to scale.
   - The ruling names the hazards and the size, not upkeep (oak 0.35), apical control (0.45), leaf girth (1.0) or tolerance (0). With upkeep at 0 a balance never falls below a tolerance of 0, so the balance hazard the mapping sets would shed nothing.
5. **The ordinary:** grows at the site's 0.5; an old family carrying 1/6 is refused by name, not reinterpreted (as shipped, `an_old_wire_light_reads_only_as_the_sites`).

## Host answers (2026-10-12, second round)

- **Retained foliage: a stated approximation.** The deposit counts the year's foliage alone. For an evergreen (the spruce keeps about 5.5 years of needles) it lends the lattice that year's share. Counting retained foliage needs a leaf-lifespan row and a girth bound inside growth, and becomes its own spec (host); not this one.
- **Palm: the whole frond, 2.81 m² a node.** `MAX_LEAF_AREA` is raised to 5 m², the palm frond being the largest shipped leaf, refused past it by name. Basis: a palm phytomer is one frond, and `leaf_area` is per node.
- **Shade size:** the oak's mean over its populated ages times the mapping factor, at every age the species lives in.
- **Upkeep 0.35, tolerance 0, leaf girth 1.0:** shared physiology (wood's maintenance cost, the balance threshold, the pipe-to-leaf relation); every species takes the oak's values unscaled, so the balance hazard bites.
- **Apical control:** architecture, not light; each species keeps its own.

## Host rulings on the birch (2026-10-12, third round; BIRCH.md)

1. **The size response (ψ) is not mapped from shade tolerance.** At any ψ near the oak's, the oak-anchored mapping turns the birch's long shoots into bare whips, and the tolerance score has no basis for setting ψ. ψ stays what each species had before fn-264: zero, dormant, for every species that had none. A species gets a size response only from a source or an owner verdict. The shade hazard's mapping stays.
2. **Leaf girth stays 1.0, shared.** Under the pipe model disused pipes persist as heartwood, so a limb keeping girth from leaves it once bore is botanically defensible; it is the smaller cause.
3. **Wood lying on the ground** predates fn-264, and the owner likes the ground-running feature. fn-252 (ground contact by degree) owns it; the finding is filed there (`fn-252-.../FN264-BIRCH.md`).
