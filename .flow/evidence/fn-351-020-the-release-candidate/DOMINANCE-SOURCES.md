# Sources for sibling-branch inequality on oaks ("dominance" d)

Search date: 2026-11-13. About 40 tool calls (paper index, web search, page reads).
No recommended d is computed here. This file lists what was measured, by whom, and
which statistic of the share distribution each measurement could inform.

Share model being sourced: s = 1 - d(1 - u^4), u uniform on [0,1]. Minimum 1 - d,
mean 1 - 0.8 d, median 1 - 0.9375 d, about 16% of siblings above 1 - d/2. Share is a
linear size scale relative to an undominated sibling (s = 1).

## Headline

- **No source found gives the distribution of sibling branch sizes on any white oak
  as a table of ratios to the largest sibling.** Nothing measured was found for
  Quercus garryana at all.
- The closest oak measurements are: one group-mean length ratio between the top
  cluster and the rest of the shoot (Q. petraea, abstract only), one set of
  branch-to-parent diameter classes on 140 to 160 year old oaks (Q. robur / petraea,
  read in text), and one whole-crown diameter frequency exponent on oaks over 200
  years old (Q. petraea, read in text).
- Every mapping to d below is indirect. The ambiguities are listed per source.
- Two paywalled papers (Buck-Sorlin and Bell 2000, Harmer 1991 and 1992) are the
  most likely places for the missing per-position numbers. They were not read.

How "read" is used below: "read in text" means the passage was read in the paper's
own full text, but only the passages located by keyword search of that text, never
the whole paper end to end. Figures were not viewed, so any value that exists only
in a figure is marked as such.

---

## 1. Branch sizes among siblings / first-order branches

### 1.1 Quercus garryana (Oregon white oak)

**Not found.** Searched: web search for "Quercus garryana Oregon white oak open-grown
branch diameter number of branches crown architecture measurements"; web search for
"Oregon white oak OR Quercus alba OR bur oak open-grown branch diameter, largest
branch, scaffold branches, number of primary branches, trunk diameter ratio"; paper
index for oak branch diameter distribution within crown.

What turned up is descriptive or about crown width and stem growth, not branch sizes:

- OWIC species page, search snippet only (page not opened): "Open-grown trees develop
  very broad, rounded crowns (crown width may equal total height) with massive,
  crooked branches on short, massive boles."
  https://owic.oregonstate.edu/oregon-white-oak-quercus-garryana
- Devine and Harrington 2006, "Changes in Oregon white oak ... following release":
  snippet mentions crown diameter and height only.
  https://research.fs.usda.gov/download/treesearch/29659.pdf (not opened)
- Gould, Harrington and Devine 2011, "Growth of Oregon White Oak": snippet mentions
  crown width and open-grown diameter growth only (not opened).

Q. alba and Q. macrocarpa: **not found** (same searches). FEIS bur oak page is
descriptive only (snippet): "Branches in the upper portion of the crown are
ascending; branches are larger and horizontal ..."

### 1.2 Branch-to-parent diameter ratio on old oaks (Q. robur / Q. petraea)

Kędra K. 2019. Architectural analyses of tree forking habit with single-image
photogrammetry: a case study of old-growth temperate oaks. bioRxiv.
DOI 10.1101/795286. Version 1 read: https://www.biorxiv.org/content/10.1101/795286v1.full
(a version 3 exists, not compared). **Read in text.** Preprint, not peer reviewed.

- Species and condition: 30 oaks of the Q. robur / Q. petraea complex in a mixed
  forest remnant near Krakow. "The target trees median DBH was 53 cm. The exact age
  of those trees is not known, however, they are estimated to be between 140 and 160
  years old." Stand grown: "mean density of trees and shrubs ... 24±7 individuals
  per plot (480±140 ... per hectare)". Not open-grown.
- Definition used (attributed to Ningre 1997 and Colin et al. 2012): "BR lower than
  1/2 denotes a small branch; BR between 1/2 and 2/3 denotes a large branch; and BR
  larger than 2/3 defines a fork, while the values closer to the 2/3 threshold
  reveal asymmetric forks, and BRs close to 1:1 indicate a true fork". BR is branch
  diameter over parent stem diameter, both "measured at the distance of 1 m from the
  branch insertion point".
- Sample: only the two "main branches" per tree, "those which contributed to the
  horizontal crown extent". "a set of 38 branches underwent measurement of all
  described traits ...; which included 24 non-forked branches and 14 forked
  branches (as determined qualitatively)."
- "for five branches ... despite the main axis was clearly vertical ..., the
  branching ratio well exceeded the 2/3 threshold (for two such branches it was
  higher than 0.8)." Agreement of the qualitative and the 2/3 rule: "rather high
  (87%)".
- "The most bimodal-like distribution was found in the case of branching ratio".
- Spearman: BR with DBH rs = 0.380, with branch height rs = 0.578, with parent stem
  diameter rs = -0.569.
- Per-branch BR values exist only in the paper's Fig. S2 and box plots (Fig. 6):
  not extracted.

Mapping to the share distribution:
- Statistic: at best a **fraction above a level** (14 of 38 main branches are forks,
  that is above about 2/3 of the parent stem), and it describes the **top** of the
  sibling distribution only.
- Ambiguities: (a) the ratio is to the parent stem above the junction, not to the
  largest sibling, so it constrains how big the s = 1 branch is relative to its
  parent, not d; (b) the sample is the two largest branches per tree by design, so
  nothing is said about the small siblings or the minimum; (c) these are surviving
  branches after 140 to 160 years of shedding in a closed stand, where the model's
  share is applied to a young shoot; (d) diameter ratios compound over decades of
  secondary growth, so an old-branch ratio is not the linear size scale of the
  shoot at birth; (e) single-image photogrammetry, projected diameters.

### 1.3 Frequency distribution of all branch diameters in old oaks (Q. petraea)

Dassot M., Fournier M., Deleuze C. 2019. Assessing the scaling of the tree branch
diameters frequency distribution with terrestrial laser scanning: methodological
framework and issues. Annals of Forest Science 76:66. DOI 10.1007/s13595-019-0854-7.
https://link.springer.com/article/10.1007/s13595-019-0854-7 **Read in text.**

- Species and condition: "Three large oak trees (Quercus petraea (Matt.) Liebl.)
  were selected in a coppice with standards stand ... These trees were more than
  200 years old, higher than 25 m with a diameter at breast height between 65 and
  95 cm ... and presented large branching systems with several unpruned dead
  branches at the crown base."
- Method: "The branch diameters given by the 3D tree mock-up were grouped into 1-cm
  diameter classes"; diameters sampled "every 25 cm" along the structure.
- Result: "The allometric scaling exponent (i.e. the slope of the relationship) was
  very close for the three trees and was respectively − 2.41, − 2.37 and − 2.44 for
  tree A, B and C." With dead branches added the pooled exponent was "− 2.61".

Mapping:
- Statistic: none of the four directly. It is the slope of count against diameter
  over the **whole crown, all orders mixed**.
- Ambiguities: (a) it counts 25 cm segments, not branches, so long branches are
  counted many times; (b) siblings are not grouped, so the exponent reflects
  branching order, taper and shedding as much as sibling inequality; (c) it can only
  be compared with a generated tree by taking the same statistic on that tree.
- Value: an oak-specific, old-tree, measured whole-crown figure.

### 1.4 First-order branches and shoot populations on an open-grown oak (Q. crispula)

Suzuki M. 2003. Size structure of current-year shoots in mature crowns. Annals of
Botany 92(3):339-347. DOI 10.1093/aob/mcg144.
https://pmc.ncbi.nlm.nih.gov/articles/PMC4257506/ **Read in text.**

- Species and condition: Quercus crispula (a white oak of section Quercus, East
  Asia), one tree: height 9.6 m, DBH 23.3 cm, crown 6.2 x 5.9 m, 22 first-order
  branches, all examined. "The enclosed trees were growing naturally on a sunny
  roadside or in an open space, being unshaded by other trees."
- "In Q. crispula, current-year shoot populations on first-order branches had an
  extremely positively skewed frequency distribution of L0 (Fig. 3)."
- "neither mean nor CV of L0 or D0 was significantly correlated with the basal stem
  diameter of first-order branches (Fig. 7; P > 0·05)."
- "the number of current-year shoots on a first-order branch was proportional to
  the basal stem cross-sectional area of the branch."
- The CV values and the 22 first-order basal diameters exist only in Fig. 7 and
  Fig. 2: not extracted. The abstract's "three mature trees of each of three
  species" conflicts with Table 1, which lists one tree per species; Table 1 is
  what was read.

Mapping:
- Statistic: the **direction of skew** only. u^4 puts most siblings near 1 - d and
  few near 1, which is a positively skewed size distribution; the paper reports the
  same direction for current-year shoot length on an open-grown oak. No number.
- Ambiguities: (a) the population is all current-year shoots on a first-order
  branch, not the siblings of one parent shoot; (b) the CV would be the usable
  number and is in a figure; (c) one small tree, a different white oak.
- Lead: Fig. 2 plots the basal diameters of all 22 first-order branches of an
  open-grown oak, which is exactly priority 1 if digitised.

### 1.5 Branch-to-stem and sibling ratios in non-oak or unverified sources

- Minamino and Tateno 2014, PLOS ONE, DOI 10.1371/journal.pone.0093535, read in
  text: measured daughter inequality ("daughters' degree of deviation") on Fagus
  crenata and Abies homolepis only. **Not oak; not usable.**
- Bentley et al. 2013, "An empirical assessment of tree branching networks ...",
  PMID 23800188: the abstract page could not be fetched. Species and radius ratios
  **not verified**; listed as a lead only.
- Colin et al. 2012 (sessile oak forks in three plantations up to 23 years old) and
  Ningre 1997 (the 1/2 and 2/3 thresholds): known only through Kędra's citations.
  **Not read.**

---

## 2. Gradient of lateral shoots within an annual shoot (acrotony)

### 2.1 Whorl versus interwhorl branch length, Q. petraea

Harmer R. 1991. The effect of bud position on branch growth and bud abscission in
Quercus petraea (Matt.) Liebl. Annals of Botany 67:463-468. DOI not captured.
https://www.researchgate.net/publication/31248446 **Abstract only** (as reproduced
on an aggregator page; full text not read). Growing condition and tree age are not
stated in the abstract.

- "Approximately 60% of the buds on each shoot began expanding in spring but only
  about half of these formed branches."
- "Approximately 42% of 'whorl' buds and 28% of 'interwhorl' stem buds formed
  branches."
- "'Whorl' branches were approx. 60% longer that 'interwhorl' stem branches; buds
  on the lower surface of the shoot produced longer branches than those on the
  upper surface."
- "by Sep. 45 % of buds originally present had abscised."

Mapping:
- Statistic: a **ratio of two group means** (interwhorl mean length is 1/1.6, about
  0.63, of whorl mean length). It is closest to the model's mean share, but taken
  against the mean of the top group, not against the largest sibling.
- Ambiguities: (a) s = 1 is the undominated sibling, so a ratio to the whorl mean
  overstates shares relative to the maximum; (b) the within-group spread, the
  minimum and the per-rank gradient are not given in the abstract; (c) 58% of whorl
  buds and 72% of interwhorl buds form no branch at all, an outcome outside the
  model's range of 1 - d to 1, so whether d or a separate branching probability
  carries that is a modelling question; (d) one season's extension length, where
  the model scales length and diameter together; (e) abstract only.

### 2.2 Longest lateral against the terminal shoot, Q. rubra (red oak, not white)

Oleksak B., Kmetz-Gonzalez M., Struve D.K. Terminal bud cluster pruning promotes
apical control in transplanted shade tree whips. Journal of Arboriculture (year and
volume not captured; measurements dated 1993 to 1996).
https://joa.isa-arbor.com/request.asp?JournalID=1&ArticleID=2767&Type=2
**Read in text** (whole article).

- Condition: container-grown red oak whips, 1 to 1.2 m tall, 20 per treatment, one
  season (1995).
- Table 2, intact terminal bud cluster: terminal shoot length 48 cm; lateral shoots
  elongating 4.7; distance from the base of the terminal shoot to the lowest
  elongating lateral bud 32 cm.
- "Plants where the terminal bud cluster was removed had the fewest number of
  elongating buds. These plants also had long lateral shoots; the longest averaged
  84% of the length of the terminal shoot. The lowest lateral to terminal shoot
  length ratio, 61%, occurred in plants where the lateral buds in the terminal bud
  cluster were removed."
- "Elongating buds in control plants were almost exclusively cluster buds".
- The same ratio for the intact treatment is not stated.
- Citing Harmer 1991: "branches resulting from intercluster bud elongation tended
  to be shorter than branches resulting from lateral buds in the terminal bud
  cluster."
- Ash only, not oak: terminal to average lateral shoot length ratio 1.1 (intact),
  and "the 1.8 ratio found in established green ash".

Mapping:
- Statistic: the **largest sibling relative to the leader**, which fixes what s = 1
  means against the parent axis. It says nothing about d.
- Ambiguities: red oak; nursery whips; pruning treatments, with the untreated value
  missing; one season.

### 2.3 Where branches sit along the growth unit, Q. rubra

Heuret P., Guédon Y., Guérard N., Barthélémy D. 2003. Analysing branching pattern in
plantations of young red oak trees (Quercus rubra L., Fagaceae). Annals of Botany
91(4):479-492. DOI 10.1093/aob/mcg046.
https://pmc.ncbi.nlm.nih.gov/articles/PMC4241068/ **Read in text.**

- Condition: main axis of 5-year-old trees, plantation in south-western France,
  "2 m spacing between trees".
- Table 1: monocyclic growth unit (n = 24): length 53.6 cm, 29.9 nodes, 9.4
  branches. First unit of a bicyclic shoot (n = 68): 44.0 cm, 28.9 nodes, 3.2
  branches. Second unit (n = 55): 51.3 cm, 21.1 nodes, 10.0 branches.
- "Branches were more often found towards the tip ... The maximum probability of
  observing a branched node occurred at around the seventh node below the apex;
  this probability was zero after the 26th node".

Mapping:
- Statistic: none. This is the **number and position of siblings** (about 9 to 10
  per growth unit), not their sizes. Branch types (delayed by a year or by a cycle,
  monocyclic or polycyclic) are recorded; branch lengths were not found in the text
  searched.

### 2.4 Other architectural sources checked

- Collet C., Colin F., Bernier F. 1997. Height growth, shoot elongation and branch
  development of young Quercus petraea grown under different levels of resource
  availability. Annales des sciences forestières 54(1):65-81.
  https://hal.science/hal-00883132 **Read in text.** Seedlings 2 to 4 years old,
  semi-controlled conditions. Counts of buds becoming shoots, no branch length or
  diameter by rank. Relevant only as a statement that the hierarchy is unstable:
  "In 60% of the seedlings, the longest GUs produced during the different growth
  flushes were found on different shoots"; "high frequency of apical bud death (20%
  of all apical buds, each year)".
- Buck-Sorlin G.H., Bell A.D. 2000. Models of crown architecture in Quercus petraea
  and Q. robur: shoot lengths and bud numbers. Forestry 73(1):1-19.
  DOI 10.1093/forestry/73.1.1. **Abstract only; paywalled.** "The shape of
  shoot-length frequency distributions in oak can be described for each branching
  order using a limited number of mathematical models"; snippet: "shoot lengths
  decreased with increasing branching order". Mature trees. The fitted
  distributions are the most likely source for a measured sibling spread.
- Buck-Sorlin G.H., Bell A.D. 2000. Crown architecture in Quercus petraea and
  Q. robur: the fate of buds and shoots in relation to age, position and
  environmental perturbation. Forestry 73(4):331-349.
  DOI 10.1093/forestry/73.4.331. **Abstract only.** Shedding ratios, no sizes in
  the abstract.
- Harmer R. 1992. Relationships between shoot length, bud number and branch
  production in Quercus petraea. Forestry 65:61-72. **Not read** (citation only).
- Ward W.W. 1964. Bud distribution and branching in red oak. Botanical Gazette
  125:217-220. **Not read** (citation only).
- k-th lateral relative to the longest, per rank, for any oak: **not found.**
  Searched: paper index for "Quercus petraea annual shoot architecture lateral
  branch length gradient acrotony position along parent shoot"; web searches on
  oak branching pattern, polycyclism and lateral shoot length by rank, and on whorl
  and interwhorl branch length.

---

## 3. Height culmination and retrenchment (secondary; at most five)

No source with a measured age or size at which oak crowns stop adding height was
found. Searched: "veteran oak crown retrenchment height stops age years Quercus
robur maximum height reached"; "oak height growth culminates age ... optimum crown
size".

1. Lonsdale D. (ed.) 2013. Ancient and other veteran trees: further guidance on
   management. Ancient Tree Forum / Tree Council.
   https://ancienttreeforum.co.uk/wp-content/uploads/2015/02/ATF_book.pdf
   Read in text (keyword-located passages only). "After maturity, growth and
   shedding both continue, but the overall effect is often the gradual diminution
   of crown size that is widely known as retrenchment. During this process, many
   trees also produce numerous epicormic growths, which are capable of developing
   into new branches when older ones are shed." No number.
2. Same book, caption of fig. 4.10: "ancient oak with epicormic growth on lower
   stem; this provides capacity for retrenchment". No number.
3. Kędra 2019 (section 1.2), oaks of 140 to 160 years, read in text: "Five trees
   (55%) of the R.D model had some minor or dead branches (seemingly once being
   major branches) below the fork". A count of lost major branches, not a height.
4. Unverified, search snippet of a secondary text only (a county natural history
   magazine on archive.org, page not opened, sentence truncated, not oak specific):
   "Once optimum crown size is reached, usually after forty to a hundred ...".
   The primary is probably White 1998 on estimating the age of large and veteran
   trees; **not read**, do not cite as measured.
5. Fay N., survey methods paper, search snippet only: "The crown begins to reduce
   in size (crown retrenchment) and the annual increment (CAI) eventually reduces,
   compared to earlier developmental stages in the trees."
   http://www.arborecology.com/resources/Reduced_SurveyMethods%20_Arboricultural_Technique_TURIN_280a.pdf

---

## What each usable measurement could inform

| Measurement | Species, condition | Share statistic | Main ambiguity |
|---|---|---|---|
| Interwhorl branches about 0.63 of whorl branch length (whorl "approx. 60% longer") | Q. petraea; condition not in abstract | mean, as a ratio of group means | reference is the whorl mean, not the largest sibling; abstract only |
| 42% of whorl buds, 28% of interwhorl buds form a branch | Q. petraea; same source | none in range; a zero-size outcome | outside 1 - d to 1 |
| Branch / parent diameter classes below 1/2, 1/2 to 2/3, above 2/3; 14 of 38 main branches forks; two non-forks above 0.8 | Q. robur / petraea, 140 to 160 y, stand | fraction above a level, top of the distribution | ratio to parent, not to sibling; largest branches only |
| Diameter-class frequency exponent -2.41, -2.37, -2.44 | Q. petraea, over 200 y, coppice with standards | none directly | all orders, counted by segment |
| Shoot length "extremely positively skewed"; CV independent of branch size | Q. crispula, open-grown, 9.6 m | direction of skew | values in figures only; not siblings of one parent |
| Longest lateral 84% or 61% of terminal shoot | Q. rubra whips, pruned | size of the s = 1 sibling against the leader | red oak, pruned, untreated value missing |
| 9.4 to 10.0 branches per growth unit of about 21 to 30 nodes | Q. rubra, 5 y, plantation | none; sibling count | no sizes |

## Leads not followed (budget)

- Digitise Fig. S2 and Fig. 6 of Kędra 2019 for the per-branch ratios.
- Digitise Fig. 2 and Fig. 7 of Suzuki 2003 for the 22 first-order basal diameters
  and the shoot-length CV of an open-grown oak.
- Full text of Harmer 1991 and 1992 and of Buck-Sorlin and Bell 2000 (paywalled).
- Colin et al. 2012 and Ningre 1997 for fork frequency and branching ratios in
  sessile oak plantations.
- Bentley et al. 2013 for measured daughter radius ratios; species unverified.
