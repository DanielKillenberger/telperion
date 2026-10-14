# fn-275 R1: the date palm's establishment (worker, 2026-10-14)

Fetched copies are in `raw/sources/` (ignored); the sha256 prefix is the fetched file's. Each value below is copied from the quoted text.

## The law (general to palms)

- Tomlinson and Huggett 2012, "Cell longevity and sustained primary growth in palm stems", Am. J. Bot. 99: 1891 (`tomlinson2012.md`, 6f41e72c32cbcb72): "An important feature of palm growth is the 'establishment phase' of early stem development in which the stem progressively widens as the crown grows to its mature size. Usually this occurs before the stem becomes visible above ground".
- Broschat, "Palm Morphology and Anatomy", UF/IFAS EP473 (`ifas-ep473.md`, ca93fc22f5d8a212): "Stems of juvenile palms (those which have not yet achieved their maximum diameter) elongate very slowly, expanding incrementally in diameter with each new leaf. ... Once a juvenile palm stem reaches its maximum diameter, substantial vertical elongation of the stem begins".
- The date palm: "The girth does not increase once the canopy of fronds has fully developed" (Zaid and de Wet 2002, FAO 156 ch. I, `fao156-ch1.md`, ca0ec9aa36dac408).

The shape the spec states holds: fronds grow to their mature size over establishment, and the stem's rise waits for it.

## Values

| Row | Value | Source text | Status |
|---|---|---|---|
| `frondEstablishment` | 6.75 years | Tissue-culture palms "5 years old" against "OS-grown plants (20–40 years old)": leaf length 121.71 / 130.29, 140.43 / 155, 133.43 / 140 in. (Frontiers in Plant Science 16: 1516983, 2025, Table 1; `frontiers2025.md`, 5cf62ac0575ec01c), shares 0.934, 0.906, 0.953, mean 0.931. The logistic course from 0.1 reaches 0.931 at year 5 over 6.746 years (code fit), rounded to 6.75. | sourced |
| cross-check | 4 to 5 years | "In 4 or 5 years, the crown clears the ground and the 1st flowering can be expected" (Orwa et al. 2009, Agroforestree Database, via growables.org, `orwa2009.md`, d54cdc39c40abadf) | consistent: the course stands at 0.75 at 4 and 0.93 at 5 |
| `frondEarlyShare` | 0.1 | Laboratory plantlets "are usually about 35 to 45 cm long with 4 to 5 leaves" (FAO 156 ch. V, `fao156-ch5.md`, 989d34975cc7e7e2), 0.40 m against "3 to 6 m long (4 m average)" (ch. I) | declared stand-in: the plantlet's age is not stated, and its length is the plant's, not a leaf's |
| curve | logistic course | Pallardy 2008: "seasonal and lifetime growth of shoots ... generally conform to such a [sigmoid] pattern" (already `unfold::course`) | sourced (fn-270) |

No source on disk or found gives a seedling's or a young palm's leaf length at a stated early age (FRICTION.md).

## Shipped age

The palm ships at 40. Its drawn crown is 42 living and 16 dead fronds at 8.4 a year, so its oldest drawn frond was made at about year 33, past 6.75: the shipped palm is unchanged (test `establishment_shows_on_a_young_palm_alone`).

## Reported, not built

- **Juvenile leaves.** "Leaves of seedling date plants are characterised by a slightly developed petiole and a juvenile leaf which develops during the first three years after seed germination ... also called primordia, non-pinnae or entire leaves" (FAO 156 ch. I). Plantlets carry "4 to 5 leaves among which are 0 to 2 pinnae leaves" (ch. V); "Plants at 12–18 months old begin to produce feathered (compound) leaves" (Date Palm Biotechnology, 2011, search snippet only). For a seedling the entire leaf is the dominant read at ages 1 and 3; an offshoot-grown palm (the catalogue's orchard palm) is planted with pinnate leaves.
- **The stem's rise and girth.** The general law has the stem widen with each new leaf and rise only after establishment. In the stills the stem at 1 is a stump at nearly its full girth under eight small fronds, at 3 about a frond base high, and hidden in the crown at 6; whether the engine's rise and girth follow establishment was not measured. Not part of R2.

## Stills and recording (R5, seed 1, `examples/unfold.rs`, the whole year)

`raw/stills/{whole,crown}-{1,3,6,40}/`, viewed: at 1 eight small fronds (0.10 to 0.12 of a mature one by the law) on the stump; at 3 a knee-high rosette under the scale figure's height; at 6 a wide ground rosette of near-mature fronds; at 40 the shipped palm. The crown camera sits a quarter of the height off, so at 1 and 3 it frames single leaflets. The grow-in, `examples/grow_in.rs` on the device a whole year a frame, is `raw/grow-in.mp4` (crossfaded, 6 frames a second); its year-3 frame (`raw/grow-in/date-palm-00002.png`, viewed) matches the CPU still, and its year 40 draws the CPU's 6,554 leaves.

After the review's fix (a frond is sized by the age it emerged at, not its place in the year's count), the stills and the grow-in were drawn again and viewed at 1, 3 and 6 (`raw/stills/`, `raw/grow-in.mp4`; the first drawing is kept beside them as `*-first`). The palm at 40 is byte-identical to the first drawing's still.

## The film's palm at 1 (host's check, fn-272 FILM.md defect 3)

A probe on this build (engine path, seed 7, whole age, `raw/probe.log`; the probe test was removed after): at 1 the palm stands 1.11 m with a crown 1.6 m across; with `frondEstablishment` cut to zero it stands 7.19 m and 15.9 m across, the film's 7.3 m and 16.3 m. The film's tall seedling is the full-length fronds this spec makes small. The stem itself rises from year 1: its top stands at 0.37 m at 1, 1.12 m at 3 and 14.9 m at 40, so it does not wait for establishment as the general law has it (above). The grow-in, begun for 40 and drawn at 1 on the device (`raw/grow-in/date-palm-00000.png`, viewed), stands below the scale figure.
