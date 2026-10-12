# fn-264 R1a and R2: sources, Jev's choices and the checks (worker, 2026-10-12)

Every value below was cut from a fetched source, offered to Jev as one candidate among every number in its passage (with `none`), copied by code from the passage bytes, and then checked by the worker against the fetched source text. `jev select` recalls lengths and ages only (FRICTION.md), so the candidates came from a scratch extractor run through `jev ask`. Ledger entries are under `ledger/` (ignored); fetched sources stay in the session scratchpad.

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
| *Betula pendula* | 2.03 ± 0.09 | read in the same table; see the birch below |
| *Phoenix dactylifera* | no score; "very bright" (FAO Ecocrop), "requires full sun" (UF/IFAS FR314) | none: the extractor offers numbers only |

## Not shipped: waiting on the host

- **Spruce leaf area.** No source gives needle area per length of current-year shoot. Candidates: 18 to 31 needles per cm of shoot (Bellan et al. 2019, Austrian J. For. Sci., a closed 36-year stand, the whole crown profile; `fn264-pick:18dddb318255f8d1-3e7e9f-0`), and a projected needle area of 13.7 mm² sun, 8.2 mm² shade (Gebauer et al. 2019, iForest 12: 27, 20-year-old trees, control plots; `fn264-pick:18dddb319ab80e89-3e7fab-0`, `...31b39f0079-3e8129-0`). A value needs a point inside a range and a product of two stands. Needles live 5.5 years to half a cohort (Muukkonen and Lehtonen 2004; `fn264-pick:18dddb331ff24320-3e8459-0`), but the deposit counts only the nodes laid this year, so a current-year value lends the lattice about a fifth of the crown's needles.
- **Palm leaf area.** The one measured frond area is 2.81 ± 0.64 m² (Abdel-Sattar et al. 2024, Folia Hortic., 15-year-old male palms, drip-irrigated, 8 m apart; `fn264-pick:18dddb39ffe7316b-3e89b7-0`, 0.76), computed as length x width x 0.8, an outline rather than leaflet area. A second-hand "about one square meter" per leaf (Montazar et al. 2020, Water 12: 2253, citing a USDA bulletin). The engine refuses more than 1 m² a node (`MAX_LEAF_AREA`), so 2.81 cannot be drawn without an engine change.
- **Every response row of every species** (shade hazard, shade size, apical control, upkeep, balance hazard, tolerance, leaf girth). No source gives these model coefficients for any species; the oak's are its own values from fn-195 round 5 (host decision 29), not literature. A sourced shade tolerance (above) could set them only through a mapping from a 0-5 score to the engine's coefficients, which is a design decision.
