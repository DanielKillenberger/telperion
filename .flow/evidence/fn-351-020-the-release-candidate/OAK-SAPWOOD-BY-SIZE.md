# The oak's sapwood rows by tree height, 14 Nov 2026

By a Fable build agent on `fn-351-oak-sapwood-by-size`, branched from `fn-351-candidate` at `7685ee85b`. The finding it acts on is `TREES-AGAINST-NATURE.md`, question 2: the old oak's trunk stands 1.15 to 1.63 times the top of White's open-grown range at 300, and the girth rule read the oak's two wood rows as one value at every size.

**Tags.** [text] quoted from the paper's full text, opened today. [arithmetic] computed here. [measured] a tree grown here by the scoreboard's probe. [record] a figure from an earlier file. [not run] named and left.

## What changed

- **Seed 1001's trunk at 300 is 1.81 m, against 2.20 m before and White's 1.56 to 1.92 m.** It is 1.06 m at 120 (inside 0.83 to 1.09) and 1.53 m at 200 (White's top is 1.52). The paper lands within 3 cm of the grown tree at 120, 200 and 300.
- **The oak at 40 is thicker**, 30.8 to 39.6 cm where it stood 26.1 to 33.5 cm, inside White's 28 to 40 cm and further above *Q. garryana*'s own figures (23 cm in Phillips's 40-year plot, a 26 cm ceiling in Gould 2008).
- **Every other preset grows the same tree to the bit.** Seven tree digests are equal before and after ("Other presets").
- **Seed 1007 stays out of White's range**, 2.57 m at 300 where it stood 3.13 m, 1.34 times White's top, as the reading said it would. Its excess is its leaf and crown spread, which these rows do not touch.

## Sources, checked against their text

**Phillips, Bond, McDowell, Ryan and Schauer 2003**, Funct. Ecol. 17: 832-840, full text at `research.fs.usda.gov/download/treesearch/9298.pdf`, Table 1, "Plot and tree characteristics" [text]:

| Variable | 25 m plot | 10 m plot |
|---|---|---|
| Oak tree age (years) | 250 | 40 |
| Oak canopy height (m) | 25 | 10 |
| Oak stem density (n ha-1) | 140 | 1200 |
| Oak diameter at 1·3 m (m) | 0·56 ± 0·05 | 0·23 ± 0·01 |
| Oak sapwood depth at mid-bole (cm) | 4·2 ± 0·4 | 4·3 ± 0·1 |
| Oak leaf area: sapwood area (m2 cm-2) | 0·37 ± 0·01 | 0·23 ± 0·02 |

- The recorded 0.23 ± 0.02 and 0.37 ± 0.01 m² per cm² are the table's, each on the class `PHASE2-SURVEY.md` gave it. The ± is a standard error: "Standard errors are indicated for some variables." The table states no n; "Eight trees in each of the two plots were selected for sap flow and biometric measurements."
- The ratio is a plot's. Leaf area index over sapwood area per hectare gives it back (3.3 / 9.1 = 0.36, 3.2 / 13.9 = 0.23) [arithmetic]. The two plots differ in height, age and stem density at once, and the paper separates none of them.
- Sapwood was read from cores: "Sapwood depth was estimated visually from cores taken from at least two locations on each of the sap flow trees, at the approximate height of sap flow sensor installation." Leaf came from litter collectors.

**Haneca, Čufar and Beeckman 2009**, J. Archaeol. Sci. 36: 1-11, full text at `africamuseum.be/publication_docs/`, Table 1, "Sapwood statistics for different regions in Europe" [text]. Germany (Hollstein, 1965, 1980), average number of sapwood rings and standard deviation by tree age class: under 100, 16.00 and 4.50; 100 to 200, 20.40 and 6.20; over 200, 25.90 and 7.50; not specified, 19.00 and 7.54, range 7 to 66, n = 446. British Isles (Hillam et al., 1987): under 100, 22.35; 100 or over, 31.74. The recorded 16.0, 20.4, 25.9, 22.4 and 31.7 are the table's. Section 2.6: "In general, old trees tend to have more sapwood rings compared to younger individuals. However, models that attempt to describe the relationship between the age of the tree or average ring-width and number of sapwood rings usually have a low predictive value (ca. 10%)".

**No sapwood ring count for *Q. garryana* was found.** Haneca et al. do not name the species and two searches returned none.

## The quantity the rows are read against: tree height

Phillips states the change by size and height, never by age [text]:

- Summary: "a species in which leaf to sapwood area ratio increases with tree size."
- Summary: "25 m trees had a leaf to sapwood area ratio 1·6 times greater than that of 10 m trees."
- Discussion: "If changes in AL : AS as trees grow in height were to compensate completely for increasing tree hydraulic resistance [...] 25 m trees would need to have AL : AS 0·69 times that of 10 m trees, instead of the observed factor of 1·6."

Age appears only in the table row and in general phrases ("taller or older trees"). So the rule reads the tree's height that year, its highest axis tip above the seed's base, which the grower already holds for every tip (`passes.rs`, `charged`). Up to 10 m it reads the first row, from 25 m the second, a straight line between in the row's own unit (cm² of sapwood per m² of leaf), and nothing is extrapolated beyond either plot.

## The rings: Hollstein's two classes, an estimate

Three options, each with the paper result for seed 1001 at 300 (table below) [arithmetic]:

| Option | Rings at 10 m and 25 m | Source | Seed 1001 at 300 |
|---|---|---|---|
| A | 20 and 20 | the row as it was, Hollstein's 100 to 200 class | 1.97 m, out |
| B, taken | 16.0 and 25.9 | Hollstein's classes under 100 and over 200, the classes of Phillips's 40- and 250-year plots | 1.79 m, in |
| C | 12 and 35 | Phillips's two plots by arithmetic: sapwood depth over mean ring width, even rings assumed | 1.60 m, in |

B is taken. Its ring counts were counted on wood, in 446 trees, and the table was checked against its text today. Option C is the species' own, but nobody counted a ring: it divides a 4.2 cm depth by a mean ring width the plots' diameter and age imply, and an old tree's outer rings are narrower than its mean, so the 35 is a floor of unknown slack. C serves as the cross-check that *Q. garryana* does what European oak does, and by at least as much: equal sapwood depth (4.2 and 4.3 cm) on trunks of 23 and 56 cm, 40 and 250 years old, means more rings in the older tree.

B stays a listed estimate, as the old 20 was: Hollstein's oak is German *Q. robur* and *Q. petraea*, and he classed by age where the rule reads height. A 120-year oak of 27 m reads 25.9 rings where his table gives its age class 20.4. The comments on the rows say so.

## On paper

`oak-sapwood/paper.py` runs `girth.rs`'s ring year by year on the recorded leaf (quadratic to 40, straight lines between the five ages) and the recorded height (straight lines from the seed) [record: `TREES-AGAINST-NATURE.md`, fn-364 `MEASURES.md` "fn-363" rows]. Seeds 1007 and 1012 have no record at 40 and 80, so seed 7's and a scaled seed 1's stand in. Trunk at 1.3 m, cm [arithmetic]:

| seed | rows | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|---|
| 1001 | today: 1/0.30, 20 rings | 28 | 71 | 115 | 175 | 215 |
| 1001 | A: sapwood by height, 20 rings | 32 | 76 | 111 | 163 | 197 |
| 1001 | B: A, rings 16.0 to 25.9 (Hollstein) | 33 | 78 | 108 | 151 | 179 |
| 1001 | C: A, rings 12 to 35 (Phillips's trees) | 36 | 79 | 105 | 138 | 160 |
| 1001 | R: the reading's age classes | 34 | 87 | 129 | 183 | 206 |
| 1007 | today: 1/0.30, 20 rings | 35 | 99 | 163 | 251 | 306 |
| 1007 | A: sapwood by height, 20 rings | 40 | 101 | 153 | 230 | 279 |
| 1007 | B: A, rings 16.0 to 25.9 (Hollstein) | 42 | 102 | 146 | 211 | 250 |
| 1007 | C: A, rings 12 to 35 (Phillips's trees) | 45 | 101 | 139 | 191 | 222 |
| 1007 | R: the reading's age classes | 42 | 121 | 182 | 261 | 292 |
| 1012 | today: 1/0.30, 20 rings | 34 | 86 | 138 | 221 | 285 |
| 1012 | A: sapwood by height, 20 rings | 38 | 91 | 134 | 205 | 262 |
| 1012 | B: A, rings 16.0 to 25.9 (Hollstein) | 40 | 93 | 131 | 190 | 237 |
| 1012 | C: A, rings 12 to 35 (Phillips's trees) | 43 | 95 | 126 | 175 | 212 |
| 1012 | R: the reading's age classes | 40 | 105 | 155 | 230 | 268 |
| White | open-grown | 28 to 40 | 56 to 79 | 83 to 109 | 121 to 152 | 156 to 192 |

A straight line in the source's unit (leaf per sapwood) in place of the row's moves no figure by more than 2 cm (`A'` in the script).

The age-class run does not give back the reading's 1.89 m for seed 1001 at 300; it gives 2.06 m. The reading does not state how it ran its classes, so the difference is unexplained [unknown].

## The shape built

Four rows on an age's `form`, beside `sapwood` and `sapwood_rings` (`species.rs`):

| Row | Meaning | Neutral | Oak |
|---|---|---|---|
| `sapwood_tall` | the far height's sapwood per leaf, as a share of `sapwood` | 1 | 0.23 / 0.37 |
| `sapwood_rings_tall` | the far height's rings, as a share of `sapwood_rings` | 1 | 25.9 / 16.0 |
| `sapwood_from` | the tree height the two rows were measured at, m | 0 | 10 |
| `sapwood_to` | the tree height the shares were measured at, m | 0 | 25 |

The oak's `sapwood` is now 1 / 0.23 and its `sapwood_rings` 16.0.

- **Why shares.** The host's design asked for a second row that is dormant when equal to the first. A second row holding the far value itself has no neutral default: a species that states one value would have to restate it, and moving `sapwood` alone (a control, a walk) would stop changing any tree taller than the far height. A share of 1 is neutral without being stated, and `x × 1.0` is `x` to the bit. This is the one place the build departs from the design's wording, and it is the host's to overrule.
- **The read** (`girth.rs`, `tall` and `rows`): `row × (1 + w × (share − 1))`, `w` 0 up to the near height, 1 from the far one, `(h − from) / (to − from)` between. Two heights that are equal make a step at that height, which is the limit of a steep line.
- **Refusals** (`validate.rs`): each row on its rail, rings times their share under 1, and a far height below the near one.
- **Size.** 143 lines of Rust outside tests and comments-only lines across 11 files, 12 of them one `..Form::default()` per full `Form` literal in the beech, birch and spruce. That is at the design's "about 150 lines" stop. The walk machinery is untouched: rows blend and walk through `settings()` as every row does.

## Tests

- `girth::tests` [run, green]: the read equals the rows at the near height and their shares at the far one, is a straight line between and constant outside; shares of 1 lay the same wood to the bit at four heights; a tree below the near height lays a single-row species' wood and above the far height the far rows' wood.
- `species::scale::edges`, `tests/refusals.rs` [run, green].
- `tests/walks.rs` with `WALK=form.sapwood` [run, green]: the four new rows and the two old ones walked on sagging limbs, each by degree.
- `telperion-core`: `space_presets`, `preset_values`, `light_rows`, `parameter_reference`, `--lib catalogue` [run, green]. The oak's value file is what the writer makes of its family.

## Measured

The scoreboard's probe (`fn-352…/scoreboard/probe`) with one added line that prints `tree_digest`, built once against `.worktrees/cand` at `7685ee85b` ("before") and once against this branch ("after"), no drawing, four threads. Before-rows at 120 to 300 are fn-364's "fn-363" rows [record]; the candidate's grower equals fn-363's. Leaf here is the probe's drawn leaf, which is not the settled leaf those rows give, so leaf is compared only where both sides were grown today.

<!-- measured -->
| Age | Seed | Trunk before, cm | Trunk after, cm | On paper, before to after | White, cm | After against White | Height before to after, m | Drawn leaf before to after, m² |
|---|---|---|---|---|---|---|---|---|
| 40 | 1001 | 26.1 | 30.8 | 28 to 33 | 28 to 40 | in | 10.7 to 10.7 | 88 to 103 |
| 40 | 1007 | 33.5 | 39.6 | 35 to 42 | 28 to 40 | in | 10.4 to 10.5 | 189 to 216 |
| 40 | 1012 | 28.2 | 33.5 | 34 to 40 | 28 to 40 | in | 10.5 to 10.5 | 101 to 121 |
| 80 | 1001 | 69.5 | 74.5 | 71 to 78 | 56 to 79 | in | 18.1 to 18.1 | 774 to 774 |
| 80 | 1007 | 98.5 | 103.4 | 99 to 102 | 56 to 79 | 1.31 times its top | 18.9 to 18.9 | 1650 to 1651 |
| 80 | 1012 | 77.1 | 82.4 | 86 to 93 | 56 to 79 | 1.04 times its top | 18.1 to 18.1 | 950 to 950 |
| 120 | 1001 | 115 [record] | 106.4 | 115 to 108 | 83 to 109 | in | 27.4 [record] to 27.6 | - to 1432 |
| 120 | 1007 | 165 [record] | 150.1 | 163 to 146 | 83 to 109 | 1.38 times its top | 28.4 [record] to 28.6 | - to 2999 |
| 120 | 1012 | 134 [record] | not run | 138 to 131 | 83 to 109 | not run | 26.0 [record] to not run | - to - |
| 200 | 1001 | 180 [record] | 153.0 | 175 to 151 | 121 to 152 | 1.01 times its top | 36.8 [record] to 36.7 | - to 1541 |
| 200 | 1007 | 259 [record] | not run | 251 to 211 | 121 to 152 | not run | 37.1 [record] to not run | - to - |
| 200 | 1012 | 227 [record] | not run | 221 to 190 | 121 to 152 | not run | 39.4 [record] to not run | - to - |
| 300 | 1001 | 220 [record] | 181.3 | 215 to 179 | 156 to 192 | in | 36.0 [record] to 36.0 | - to 884 |
| 300 | 1007 | 313 [record] | 257.1 | 306 to 250 | 156 to 192 | 1.34 times its top | 38.8 [record] to 38.5 | - to 1654 |
| 300 | 1012 | 290 [record] | not run | 285 to 237 | 156 to 192 | not run | 38.2 [record] to not run | - to - |
<!-- /measured -->

`oak-sapwood/fill.py` writes this table from the logs; `read.py` prints every run with its digest.

## Other presets

Tree digests at each preset's age, seed 1001, before and after [measured]:

| Preset | Before | After |
|---|---|---|
| norway-spruce 40 | `2e06a0f1423409be` | `2e06a0f1423409be` |
| european-beech 40 | `3176135bdd68c890` | `3176135bdd68c890` |
| silver-birch 40 | `10de609c4bd3b6a9` | `10de609c4bd3b6a9` |
| date-palm 40 | `da6281e6a1faefd6` | `da6281e6a1faefd6` |
| ordinary 40 | `f250db027529c482` | `f250db027529c482` |
| laurelin 80 | `c8f46cc78f1205f9` | `c8f46cc78f1205f9` |
| telperion 82 | `3eaa62b98ee6b143` | `3eaa62b98ee6b143` |

Height, trunk, leaf, spread and node count are equal too.

## Pins

Moved, each by its own procedure:

- `crates/telperion-core/tests/catalogue_identity.rs`, three digests, re-pinned from the test's own printed value with a comment naming this file: the family (`every_preset_is_the_family_it_was`), the walk and the overrides. The family digest is one number over every preset's `Debug` text, which now holds four more fields on every age, so it moves for the rows' sake as well as the oak's; it cannot show the other presets unchanged. The tree digests above do. `every_refusal_is_the_refusal_it_was` did not move.
- `crates/telperion-core/presets/oregon-white-oak.values` and `src/browser/presets.generated.ts`, the oak's ten wooded ages: `sapwood`, `sapwoodRings` and the four new rows. `preset_values` holds the value file to the writer's output [run, green]. The TypeScript table was edited by hand to the same values and was not checked by `scripts/build-wasm.mjs` [not run].
- `src/browser/parameters.generated.ts`, four rows, rewritten by `parameter_reference` under `TELPERION_WRITE_REFERENCE=1` [run, green].

- `crates/telperion-space/src/grow/growing_tests.rs`, `a_growing_phytomer_is_read_as_it_was`: the unlaid oak at 20, seeds 1 and 7, re-pinned from the test's printed values; its beech and spruce pins hold [run, green].
- `crates/telperion-core/src/pipeline/cycle_cost_tests.rs`, `every_preset_grows_the_tree_it_grew_at_sixteen`: the oak's digest, the second of eight, re-pinned; the other seven did not move [run, green]. Its ignored sibling at each preset's age was not run, and its oak pin will have moved.

No bound was loosened. A pin among the tests not run is unknown: see "What was not run".

## What stays unmet

- **Seed 1007's excess is its leaf and crown spread.** `TREES-AGAINST-NATURE.md` puts its spread at 1.45 to 1.66 times the open-grown 95% quantile and its leaf at 1.8 times what White's ring area implies. On paper these rows bring it from 3.06 m to 2.50 m at 300, still 1.3 times White's top. Seed 1012 likewise, 2.85 to 2.37 m on paper.
- **The oak at 40 moved away from its own species' figures.** At 10 m the row is now Phillips's 1 / 0.23 where it was 1 / 0.30, so the 40-year trunk is 18% thicker: 30.8, 39.6 and 33.5 cm against Phillips's 23 ± 1 cm (a stand of 1,200 stems a hectare) and Gould's 26 cm ceiling for open growth. It stays inside White's 28 to 40 cm for *Q. robur*, at its top on seed 1007.
- **The drawn leaf at 40 rose 17 to 20%** (88 to 103, 189 to 216, 101 to 121 m²) with heights and node counts within 0.3%. Thicker wood sags less; the cause was not traced [unknown].
- **The oak's height** at 200 and 300 was 36 to 39 m before, over the species' 34.8 m potential. These rows are not expected to move it.

## What was not run

- **The trees the table marks "not run"**: seed 1007 at 200 and seed 1012 at 120, 200 and 300. The yield watcher froze every heavy scope from about 13:00 to about 14:20 (FRICTION.md, two entries of 14 Nov), and `job-b.sh` was still growing them when this was written. A finished tree is kept, and `fill.py` rewrites the table.
- **The rest of the tests that read the oak.** About 110 Rust files name it. `job-c.sh` ran the space crate's unit tests, `refusals`, `chain`, `blend_reach`, the core's `catalogue_identity`, `native_entry`, `space_stems`, `grown_total`, `capability`, `space_walks`, and the core suites `suite::species`, `suite::establish`, `suite::sag`, `suite::codominance`, `pipeline::space`, `pipeline::cycle_cost` and `family::tests`: green but for the two pins under "Pins". Every other test that names the oak was not run (the render crate's, the Wasm crate's, the browser's, the core's remaining suites and walks, and every ignored test), so a pin among them is unknown.
- **`scripts/build-wasm.mjs`**, which holds `presets.generated.ts` to the Rust presets.
- **The sheet** of the leafless oak at 120 and 300, before beside after. No GPU job was started.
- **The gate**, by instruction.
