# fn-213 R1: the silver birch's model, 2026-10-07

The silver birch (*Betula pendula* Roth), catalogue `silver-birch`, as a point in the tree space. This file holds its architecture, its long and short shoots, its pendulous outer shoots, its growth and form at 20 to 80 years, and the values that state them in `crates/telperion-space/src/birch.rs`, laid into the preset as `/space` rows. Every value cites a source or is marked **estimated** (set from the stills). Design questions for the host are at the end; none is decided here.

## Sources

Fetched texts sit in the spec's ignored `raw/lit/`; no external code or data was copied. Tags:

| Tag | Source | Kind | Read |
|---|---|---|---|
| **MBE99** | Millet, J., Bouchard, A. & Édelin, C. 1999. Relationship between architecture and successional status of trees in the temperate deciduous forest. *Écoscience* 6(2): 187-203. [PDF](http://www.arboritecture.org/pdf_uploads/millet/relationship-between-architecture-and-successional-status-of-trees-in-the-temperate-deciduous-forest--millet-bouchard-and-edelin-1999.pdf) | Architectural analysis of *Betula populifolia*, the silver birch's North American sister | Full text |
| **JH87** | Jones, M. & Harper, J.L. 1987. The influence of neighbours on the growth of trees II. The fate of buds on long and short shoots in *Betula pendula*. *Proc. R. Soc. B* 232: 19-33. [doi](https://doi.org/10.1098/rspb.1987.0059) | Paper on *B. pendula* | Abstract |
| **MA82** | Maillette, L. 1982. Structural dynamics of silver birch. I. The fates of buds. *J. Appl. Ecol.* 19: 203-218. [JSTOR](https://www.jstor.org/stable/2403005) | Paper on *B. pendula* | Abstract |
| **KU03** | Kull, O., Tulva, I. & Vapaavuori, E. 2003. Influence of elevated CO2 and O3 on *Betula pendula* Roth crown structure. *Ann. Bot.*, [doi](https://doi.org/10.1093/aob/mcg052), [PMC4242242](https://pmc.ncbi.nlm.nih.gov/articles/PMC4242242/) | Paper on *B. pendula* | Full text |
| **SU03** | Suzuki, M. 2003. Size structure of current-year shoots in mature crowns. *Ann. Bot.*, PMID 12853280 ([doi](https://doi.org/10.1093/aob/mcg144), [PMC4257506](https://pmc.ncbi.nlm.nih.gov/articles/PMC4257506/)), on *Betula maximowicziana* among three species | Paper on a congener | Full text |
| **MU26** | Muranen, S. et al. 2026. *BpLAZY1A* mediates transcriptional polarity and drives adaxial tension wood-like tissue formation in silver birch branches. bioRxiv [10.64898/2026.09.12.750306](https://www.biorxiv.org/content/10.64898/2026.09.12.750306v1.full) | Preprint on *B. pendula* | Full text |
| **LO96** | Lockow, K.-W. 1996. Ertragstafel für die Sandbirke (*Betula pendula* Roth) in Nordostdeutschland. Eberswalde. [PDF](https://planbirke.fib-ev.de/app/uploads/2019/01/Sandbirke_1996_Lockow.pdf) | Yield table, stand-grown | Full text (tables read by code) |
| **FVA** | FVA Baden-Württemberg. Baumartensteckbrief Sandbirke / Hängebirke. [PDF](https://www.waldwissen.net/assets/FVA/Waldwirtschaft/Artensteckbriefe_2.0/Sandbirke.pdf) | Forestry species account | Full text |
| **WW** | Waldwissen: Silviculture of silver birch in Finland. [page](https://www.waldwissen.net/en/learning-and-teaching/world-forestry/silviculture-of-silver-birch-in-finland) | Forestry account | Full text |
| **JRC** | Beck, P., Caudullo, G., de Rigo, D. & Tinner, W. 2016. *Betula pendula*, *Betula pubescens* and other birches in Europe. European Atlas of Forest Tree Species. [PDF](https://forest.jrc.ec.europa.eu/media/atlas/Betula_spp.pdf) (CC BY 4.0) | Atlas account | Full text |
| **OSU** | Oregon State University Landscape Plants, *Betula pendula*. [page](https://landscapeplants.oregonstate.edu/plants/betula-pendula) | University page | Full text |

The catalogue's other birch sources (`catalogue/silver-birch/sources.json`: ATKINSON-BIRCH, FNA-BIRCH, TSO-BIRCH) were not retrievable here: Atkinson 1992 is paywalled and the TSO address now answers 404.

**Jev screen (docs/typesafe.md).** Code put every number-and-unit sentence of JRC, OSU, WW and FVA to Jev's literature screen (`cargo run -p telperion-jev --bin jev -- screen`; the log is `raw/jev-screen.log`, ledger `screen:18dc3240…` to `screen:18dc3242b3df30e6-1e88c6-7`). None was a measured size at a stated age usable as an anchor: JRC's "growing up to 30 m" and OSU's "40-50(80) ft" are mature ranges (anchor 0.03, unusable); FVA's "1 bis 2 m" yearly increments in the first years is a growth rate under plantation conditions (0.32, unusable); WW's stand heights are management thresholds. The heights below are therefore read by code from LO96's table, a stand-grown proxy, as the oak's were from Jüttner's (fn-195); Jev does not judge tables.

## Architecture

| # | Rule | Quote | Setting in `birch.rs` | Status |
|---|---|---|---|---|
| 1 | **A Rauh-like hierarchy of orthotropic axes, sympodial but monopodial in effect.** | MBE99 (grey birch): the young tree is "a hierarchic unit with five categories of axes … Its organization is related to Rauh's model (Hallé, Oldeman & Tomlinson, 1978), although it is sympodial"; "the birch is a sympodium with a hierarchic development similar to that of a monopodium". | Every axis is drawn monopodial (no abortion or relay), as the beech's stem is in this engine. | Rule sourced for the congener; the drawing choice is design question 1 |
| 2 | **A 2/5 spiral, rhythmic growth, one unit a year.** | MBE99: "spiralled 2/5 index phyllotaxy and rhythmic growth"; figure 3, every category "2/5 spiralled phyllotaxy, rhythmic". | `divergence` 2π·2/5 = 2.513 rad on every age; one unit a cycle. | Sourced (congener) |
| 3 | **Deferred, acrotonic branching.** | MBE99 figure 3: A1 to A4 "deferred acrotonic axile branching". | Each unit is a bare base, medial nodes and a top zone whose laterals are likelier (`unit`). | Sourced (congener); counts estimated |
| 4 | **Five axis categories, the last an unbranched short shoot.** | MBE99 figure 3: A5 "monochasial sympodium … unbranched short shoot". | Trunk (`TRUNK`, `LEADER`, `TOP`), limbs (`LIMB`), their boughs (`BOUGH`), the hanging long-shoot systems (`BRANCH`, `SHOOT`) and short shoots (`SHORT`). | Sourced (congener) |
| 5 | **The leader dominates; the trunk stays one.** | MA82: "The dominant role of the leader in determining the growth and form in silver birch". MBE99: "The occurrence of very small forks does not detract from the general linear shape of the trunk"; total reiteration "very limited (observed maximum of two crowns per tree)". | The trunk carries on through every stem age (`continuation` 1 to `TOP`); no age reiterates the trunk. | Sourced |
| 6 | **The old top ends in small forks.** | MBE99: in the mature tree "the branches and the extremity of the main stem assume a divaricated structure with very short growth units, foretelling the senescence of the tree"; "the trunk ends by a system of small, short forks". | `TOP` from year 41: units of about 0.1 m (internode 0.02 m, 4 to 7 nodes), its top zone bearing boughs at 0.3. | Rule sourced; values estimated |
| 7 | **Few limbs persist; the young stem's branches pass.** | WW: stand birch stems "free of live branches up to a height of 5-7 meters" by first thinning (self-pruning). JH87: lower, overtopped branches lose long shoots and buds die. | The young stem (`TRUNK`, 5 years) bears only passing branches (`SPRIG`, 8 years, viability 0.97, shed 2 years after their subtree's last apex); limbs start on `LEADER`. | Rule sourced; ages estimated for an open-grown tree |

## Long and short shoots

| # | Rule | Quote | Setting | Status |
|---|---|---|---|---|
| 8 | **Long shoots build the tree; short shoots at best replace themselves.** | JH87: "The long shoots bear many daughter buds on elongated internodes and they contribute overwhelmingly to the increase in the size of the tree and to its architecture. Short shoots normally bear only one bud and at best only replace themselves each year". | `SHORT`: 2 to 3 nodes a year at 3 mm, no laterals, its one terminal bud carrying it on for up to 10 years, viability 0.92 a year. Long shoots are every other age. | Rule sourced; lifespan and viability estimated (raised from 5 and 0.85 so the limbs' inner wood stays clothed, sheet r7 to r8) |
| 9 | **A short shoot carries two or three leaves; a long shoot several.** | SU03 (*B. maximowicziana*): "Each long shoot extends sympodially, bearing three to seven leaves … Each short shoot is monopodial, usually having one terminal bud and two or three leaves". | `SHORT` 2 to 3 nodes a year; `SHOOT` 4 to 6 nodes, `BRANCH` 6 to 10. Leaves are the dressing's: one station every 15 mm of shoot (`/skeleton/twigs/twig/internodeLength`). | Rule sourced (congener); the station spacing estimated |
| 10 | **Sylleptic and proleptic long shoots; ramification falls from top to bottom.** | KU03: "CSL, current year sylleptic long shoot; CPL, current year proleptic long shoot"; "shoot length and ramification declined substantially from the top to the bottom of the crown". | Laterals grow the year after their node (prolepsis; the engine's rule). Ramification is constant per age. | Proleptic sourced; syllepsis and the vertical decline **not drawn** (design question 3) |

## Pendulous outer shoots

| # | Rule | Quote | Setting | Status |
|---|---|---|---|---|
| 11 | **The outer shoots hang.** | JRC: "its branches characteristically droop, whereas those of Betula pubescens grow upwards or horizontally". OSU: "graceful pendulous branching habit. Young twigs thin and usually pendulous". | `BRANCH` (the hanging long-shoot system) and `SHOOT` (its long shoots) bend towards an elevation below the horizontal: −1.0 rad at tropism 1.0 per metre, and −1.4 rad at 1.5. | Rule sourced; angles estimated from S3-limb-twig-curtains |
| 12 | **A branch holds its angle by its gravitropic set-point, reinforced by adaxial tension wood.** | MU26: "two distinct gravitropic responses during branch development - an early response associated with establishment of the branch apex gravitropic set-point angle, and a later response associated with polar reinforcement at the branch base"; loss of *BpLAZY1A* gives the weeping 'Youngii'. | The engine's `form.elevation` is that set-point and `form.tropism` how fast an axis returns to it. | Mechanism sourced; that the wild type's fine shoots set it below the horizontal is the reading here, not a quote (design question 2) |
| 13 | **Fine shoots also bend under their own weight.** | No birch-specific source read; MU26 shows branch posture balances gravity against reinforcement. | `sag` 2e-4 on `BRANCH`, `SHOOT` and `SPRIG`, 1e-4 on `LIMB`, 3e-5 on `BOUGH`. | Estimated |
| 14 | **Limbs ascend, then arch.** | S1 (New Forest, winter) and S3-crown-limbs-sunset: ascending limbs; S3-limb-twig-curtains: a near-horizontal limb dropping curtains. | `LIMB`: inserted at 0.6 rad, set-point 1.2 rad at tropism 0.6, internode 0.045 m, 15 years, then carried on as `BOUGH` (set-point 0.6 rad, internode 0.02 m), so old limbs slow and lean out. | Reference-visible; values estimated |

## Growth and form at 20 to 80 years

| # | Rule | Quote / table | Setting | Status |
|---|---|---|---|---|
| 15 | **Fast early height growth that slows from 25 and stalls after 50.** | FVA: "Im Sauerland wurden jährliche Höhenzuwächse von 1 bis 2 m in den ersten Jahren beobachtet"; "Das Höhenwachstum kulminiert schon früh im Alter von ca. zehn Jahren"; "das Höhenwachstum ab dem Alter 25 stark abnimmt". WW: "growth is vigorous up to an age of 40‑50 years". | Three stem ages: `TRUNK` years 1 to 5 at about 0.8 m a year (internode 0.055 m, 12 to 17 nodes), `LEADER` years 6 to 40 at about 0.4 m (0.04 m, 8 to 11 nodes), `TOP` from 41 at about 0.1 m. | Shape sourced; rates estimated against row 16 |
| 16 | **Height at age.** | LO96, top height (Oberhöhe) of stand-grown silver birch, read by code from the table: class II (28 m at 100): 13.7 m at 20, 22.4 m at 40, 26.2 m at 60, 27.6 m at 80; class III (24 m at 100): 11.7, 19.2, 22.5, 23.6 m. | Grown: 17.1 to 17.5 m at 40, 23.3 to 24.1 m at 80 (seeds 1, 7, 42; sheet 1). | Sourced as a stand-grown proxy; an open-grown birch, which spends less on height, is set just under class III at 40 and at it at 80 |
| 17 | **Mature size and life.** | JRC: "a medium-sized tree, growing up to 30 m"; "Birch trees commonly live for 90-100 years". FVA: "wird ca. 100 Jahre alt"; "Bäume können bis ca. 30 m hoch werden". OSU: "40-50(80) ft … pyramidal becoming rounded". | The ages run to 100 and more; nothing sets the 80-year tree below the 30 m bound. | Sourced (context) |
| 18 | **Pyramidal when young, rounded when old.** | OSU: "pyramidal becoming rounded". | Grown: a narrow ovoid at 40 (crown 8 to 9 m wide, 0.5 of the height), a broad rounded crown at 80 (17 to 19 m wide, 0.75 to 0.8 of the height). | Sourced; the 80-year width is unchecked against a measured crown (design question 4) |
| 19 | **White bark; a dark, fissured base with age.** | JRC: "The bark of young trees is brown in colour; when mature it turn to silvery-white, with horizontal dark grey lenticels, that with age darkens and develops fissures". OSU: "with age, trunk becomes mostly black". | The preset's material rows (fn-40, fn-55, fn-71; owner rounds 13 to 26): white stem, dark lenticel dashes, red-brown shoots under 1 cm. Unchanged. | Sourced; the dark base at 80 does not read on sheet 1 (design question 5) |

## The preset

- `crates/telperion-space/src/birch.rs`: the species, nine ages on the chain (`TRUNK` 2, `LEADER` 4, `TOP` 5, `LIMB` 6, `BOUGH` 7, `SPRIG` 8, `BRANCH` 9, `SHOOT` 12, `SHORT` 14).
- `crates/telperion-core/presets/silver-birch.values`: the same species as 206 `/space` rows (stem ages 2, 4 and 5; neutral light), written by `cargo run -p telperion-core --example space_rows -- birch 2,4,5`; `tests/space_presets.rs` holds the rows equal to `birch()`.
- Dressing rows changed: `/age` 40 (the harness opens every engine tree at 40), `/canopy/shootRadius` 0.03 and `/shellDepth` 1.0 (as the other engine trees), `/skeleton/twigs/twig/internodeLength` 0.015 (one leaf station every 15 mm, so short shoots carry leaves). The leaf, bark and canopy material rows are the owner-reviewed ones, unchanged. The habit, envelope and twig-layer rows are today's generator's; the engine reads none of them.
- Light is neutral, as the beech's and spruce's: the birch reads no light (design question 3).

## Cost (native, `--profile ci`, sheet 1)

| Age | Seed | Phytomers drawn (nodes) | Leaves | Grown | Height × width |
|---|---|--:|--:|--:|---|
| 40 | 1 | 912,380 | 597,419 | 0.50 s | 17.1 × 8.4 m |
| 40 | 7 | 1,287,491 | 849,799 | 0.47 s | 17.2 × 9.0 m |
| 40 | 42 | 1,417,415 | 879,014 | 0.40 s | 17.5 × 8.7 m |
| 80 | 1 | 2,107,929 | 1,420,876 | 0.85 s | 23.3 × 17.4 m |
| 80 | 7 | 2,574,319 | 1,736,413 | 0.95 s | 24.1 × 18.4 m |
| 80 | 42 | 2,480,536 | 1,686,585 | 0.88 s | 24.1 × 18.7 m |

Every tree is under the browser's ceiling of 4,000,000 kept phytomers (`pipeline/space.rs`).

## Design questions for the host (not decided here)

1. **Sympodial axes drawn monopodial.** MBE99 calls the birch a sympodium that develops like a monopodium. The engine can draw the sympodium (abortion with a relay at the last node, `relay_at` 1, as the beech's modules), at the cost of kinks the stills would have to show are absent. This point draws it monopodial.
2. **Pendulous shoots: set-point or weight.** The hanging shoots come mostly from a set-point below the horizontal (rule 12), with a small `sag` (rule 13). MU26 sources the set-point mechanism but not a negative wild-type set-point; a birch whose shoots hang only by weight would need `sag` large enough to fold a thin shoot, which on S3's evidence the engine's beam does not yet reach without bending the limbs that bear it. Whether the set-point reading stands is the host's call.
3. **No light.** Ramification and shoot length falling from crown top to bottom (KU03) and the birch's shade intolerance are what fn-197's light would draw; this point leaves light neutral, as the beech's.
4. **Crown width at 80.** No source read gives an open-grown birch's crown width at 80; the grown 17 to 19 m comes from the limbs' estimated rates.
5. **Bark at age.** The material draws the stem white at every girth; the black, fissured base of an old birch (JRC, OSU) does not appear at 80. Whether this is the material's rows or a capability is open.

## Round 2 (2026-10-07, after the host's reading of sheet 1)

The rules above stand; these values moved, each **estimated** from the stills against S1, S2 and S3 (SHEET-2.md). Where a row above gives the older value, this table supersedes it.

| Age or row | Sheet 1 | Round 2 | Aimed at (host's point) |
|---|---|---|---|
| Stem (`TRUNK`, `LEADER`, `TOP`) `tropism`, `wander`, `bendLength` | 1.0, 0.3, 4 m | 0.5, 0.5, 6 m | (5) the stem curves and leans |
| `LEADER` lateral `BRANCH` (medial, top) | 0.12, 0.2 | 0.06, 0.12 | (1) an open crown |
| `TOP` lateral `BRANCH` (medial, top) | 0.3, 0.3 | 0.15, 0.2 | (1) |
| `LIMB` `tropism`, set-point, `sag` | 0.6, 1.2 rad, 1e-4 | 0.9, 1.3 rad, 3e-5 | (2) limbs ascend steeply |
| `LIMB` lateral `BRANCH` (medial, top) | 0.2, 0.35 | 0.1, 0.25 | (1) |
| `BOUGH` set-point (the old limb carries on in it) | 0.6 rad | 1.0 rad | (2) old limbs no longer flatten into spokes |
| `BOUGH` lateral `BRANCH` (medial, top) | 0.25, 0.4 | 0.12, 0.3 | (1) |
| `BRANCH`, `SHOOT` lateral `SHORT` | 0.45 | 0.35 | (1) |
| `SHORT` lifespan | 10 years | 6 years | (1) rule 8: short shoots at best replace themselves |
| `/skeleton/twigs/twig/internodeLength` (leaf stations) | 15 mm | 20 mm | (1) |
| `/material/shootRadius` | 0.01 m | 0.03 m | (3) fine wood dark, white from 6 cm in radius |
| `/material/leafFront`, `leafBack` (R, G, B) | 0.10/0.19/0.06, 0.20/0.28/0.14 | 0.16/0.26/0.035, 0.25/0.33/0.09 | (4) bright yellow-green |
| `/material/ridgeScale`, `crest` (RGB, strength), `cavityStrength` | 0.08, +0.1 at 0.4, 0.3 | 0.16, −0.5 at 1.0, 0.8 | (5) and (c): a dark base on old, thick wood |

Cost at round 2 (native, `--profile ci`): 315,075 to 475,769 nodes at 40 and 621,895 to 808,944 at 80 (seeds 1, 7, 42), a third of sheet 1's; 16.6 to 16.8 m tall at 40, 23.5 to 24.1 m at 80; crowns 7 to 9 m wide at 40, 14 to 16 m at 80.

## Round 3 (2026-10-07, after the host's reading of sheet 2)

Each value **estimated** from the stills (SHEET-3.md); this table supersedes the rows above.

| Age or row | Round 2 | Round 3 | Aimed at (host's point) |
|---|---|---|---|
| `BRANCH`, `SHOOT` lateral `SHORT` | 0.35 | 0.5 | (1) more short shoots, so more leaves, along each strand |
| `/skeleton/twigs/twig/internodeLength` (leaf stations) | 20 mm (default) | 12 mm | (1) more leaves per short shoot and strand |
| `/element/length`, `/element/width` | 5.5, 4.5 cm | 6.5, 5.2 cm | (1) larger leaves, inside JRC's 3 to 7 cm |
| `BOUGH` internode, set-point (the old limb carries on in it) | 0.02 m, 1.0 rad | 0.025 m, 0.95 rad | (2) old limbs reach further, so the crown broadens with age; young limbs, still in `LIMB`, are unchanged |

A probe of 0.03 m and 0.85 rad gave a 22 m crown at 80 and a 9 m one at 40, too broad at both; round 3 takes the step between.

## Round 4 (2026-10-07, after Astra's and the host's reading of sheet 3)

Aimed at trait (j): long, straight, nearly parallel limbs ending in regularly spaced tufts. Each value **estimated** from the stills (SHEET-4.md); this table supersedes the rows above.

| Age or row | Round 3 | Round 4 | Why |
|---|---|---|---|
| `LIMB`, `BOUGH` apex death with a relay (`abortion`, `relay*`, `epitony`) | none | 0.08 a unit, relay 1, epitony 0.4 | Now and then the apex dies and the upper bud of its last unit carries the limb on: a kink, and a new direction (MBE99: the grey birch's axes are sympodia; the module drawing of rule 1 stays monopodial elsewhere) |
| `LIMB`, `BOUGH` `wander`, `bendLength` (fn-207) | 0.25, 1 m | 0.6, 3 m | Crooked limbs whose bend persists over metres |
| `LIMB`, `BOUGH` `dominance` | 0 | 0.2 | Siblings differ in size, so fewer reach full length |
| `LIMB`, `BOUGH` `roll` | 0.8 | 1.2 | Laterals leave at more varied bearings |
| `LIMB` lateral `BOUGH` (medial, top) | 0.08, 0.4 | 0.12, 0.45 | More subdivision of the mid-crown limbs |
| `BOUGH` lateral `BRANCH` (medial, top) | 0.12, 0.3 | 0.1, 0.22 | Keeps the crown open with the added boughs |
| `LIMB` internode | 0.045 m | 0.055 m | The relays shorten the limbs; this restores the crown's width |
| `BOUGH` internode | 0.025 m | 0.03 m | The 80-year width (17 to 21 m) with the kinks |
| `LIMB` `viability`, `shedding` | 1, never | 0.993 a year, 3 years | A few limbs die and are shed, the low ones among them (the host's low limb) |

A first probe with `abortion` 0.15 and `dominance` 0.4 drew a narrow column (6 m at 40, 11 m at 80); its relays rose steeply on the upper side. Codominant forks: the engine forks where a relay is as thick as the limb it continues; no separate fork setting was used.

## Round 5 (2026-10-07, after the merge of fn-221 and fn-222)

Each value **estimated** from the stills (SHEET-5.md); this table supersedes the rows above.

| Row | Round 4 | Round 5 | Aimed at |
|---|---|---|---|
| `form.internodeSpread` (fn-222): stem, `LIMB`/`BOUGH`, `SPRIG`, hanging `BRANCH`/`SHOOT` | none | 0.2, 0.7, 0.4, 0.6 | (j): laterals and strands stand at uneven spacing along each axis |
| `TOP` internode (the old stem's yearly unit) | 0.02 m | 0.015 m | (c): the 80-year crown less tall and pointed |
| `BOUGH` set-point (old limbs carry on in it) | 0.95 rad | 0.8 rad | (c): old limbs spread, the crown broadens at 80 |
| `/material/mature*` (fn-221): colour, furrow strength, radius, span | off | 0.05/0.048/0.045, 0.8, 0.23 m, 0.06 m | (b): old bark near-black and deeply furrowed from 23 cm of radius to 29 cm (JRC, OSU, rule 19): the 40-year stem white above its flare, the 80-year base black to about a metre |
| `/material/crest*`, `cavityStrength` | −0.5/−0.5/−0.48 at 1.0, 0.8 | +0.1 at 0.4, 0.3 (the owner-reviewed preset's values) | The tints act on the old colour too (fn-221): ridges a lighter grey on the black base, as in S1, no dark crest on the white stem |

A probe with old bark from 20 cm darkened the 80-year stems of seeds 7 and 42 to two metres and more; 24 cm left seed 1's base white above the flare.

## Round 7 (2026-10-07, scaffold limbs, host's decision on sheet 6)

The rows and their reasons are in SHEET-7.md, "The mechanism". In short: a young-leader age (`LOW_FORK`, years 6 to 14) bearing passing branches and a few low scaffolds; the leader (years 15 to 40) bearing few limbs among passing branches; limbs steep for 25 years (was 15), viability 0.998, then spreading as boughs; round 6's forking old stem; stem wander 0.35; old bark from 0.30 m of radius over 0.07 m. All **estimated** from the stills; the scaffold structure follows the references (S1, S3) and the oak's long-lived limbs. Stem ages are now 2, 3, 4 and 5.

## Round 8 (2026-10-07, curtains; host's decision after the owner's verdict on round 7)

The rows and their sources are in SHEET-8.md, "What moved". In short:

- The hanging axis (`BRANCH`) is an unbranched strand set straight down (set-point −π/2, tropism 3, wander 0.15). It grows 4 to 6 nodes of 0.06 m a year for up to 12 years, carrying only short shoots.
- The `SHOOT` age is no longer lived.
- Limbs, boughs and the old top bear strands at 0.3 to 0.5 per node.
- Limb and bough wood holds sleeping strand buds (0.6 a node, waking after 4 years at 0.15 a year; Cameron and Sani 1994 for epicormic sprouting in *B. pendula*).

All **estimated** from the stills and checked by `birch_measure.rs`; no value is taken from the old generator.

## Round 9 (2026-10-07, arching limbs and irregular strands)

The rows and their reasons are in SHEET-9.md, "What moved". In short:

- **Limbs:** they rise less steeply, pass sooner to their spreading age, and the old limbs bend over under their own load (`sag`).
- **Strands:**
  - They sit on the limbs' outer, younger wood; the sleeping strand buds on inner wood are removed or thinned.
  - They curve a little and spread unevenly in internode length and tip survival.
  - They fork rarely (0.005 a node).
- **Fine wood:** its colour is set from S1's crown haze, read by code.

All values are **estimated** from the stills and checked by `birch_measure.rs`.
