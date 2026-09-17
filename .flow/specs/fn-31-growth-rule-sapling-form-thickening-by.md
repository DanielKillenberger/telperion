# Growth rule: sapling form, thickening by age, a shedding floor

> **Shelved 2026-09-18.** Growth over time is a hidden feature since fn-65: the mature tree is the product and the harness draws the direct build. Round 13 returned the species-neutral mechanism (`growth.saplingRate`, the young-phase hand-over, the no-spurt invariant) and the pedunculate oak brief with its Jev-weighted curve; the rename and the Q. robur fit wait with the rest. This spec resumes only when the owner un-hides growth. The reason is recorded in CLAUDE.md on master under "Mature trees are the product".

## Conversation Evidence

> user (on the fn-30 age strips, 2026-09-14): "a bare pole doesn't sound very appealing?"
> user (2026-09-14): "the 10yo one is way too low quality and i'm not convinced that the 26yo looks like that either? This calibration task seems to have missed the mark by not being able to touch the generator"
> user (2026-09-14, asked whether to record the rejecting verdict and capture a growth-rule spec that may touch fn-11's machine): "yes"
> user (2026-09-14, asked how the spec relates to fn-21): "New spec, fn-21 untouched"
> user (2026-09-14, on the visual digest of this spec): "we also want 1yo saplings to look like actual saplings yea?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 45% [paraphrase], 30% [inferred] -->

Calibrated growth (fn-30) fitted the oak's and the spruce's growth traits to open-grown height references and rendered each species at six ages. The owner judged the strips and rejected the young ages: the ten-year oak is one stem with a handful of leaves, the twenty-six-year oak is a bare scaffold with sparse foliage, and the five-year spruce is a stem with no needles. In the owner's words, the calibration task "missed the mark by not being able to touch the generator". [user]

fn-30 could set only trait values, and its boundary forbade any change to fn-11's growth rule. Fitting rate and shape moved how fast each tree reaches its envelope; it could not change what the tree looks like on the way. The calibration therefore met its height numbers and exposed three defects in the rule itself. [paraphrase]

- **Early form.** Laterals and foliage lag the stem by decades, so a young tree reads as a pole and a middle-aged one as a scaffold. [paraphrase]
- **Thickening.** The trunk stays a fixed fraction of the live envelope height, so the height-to-diameter ratio never changes with age; fn-30 measured a ratio near 28 for the oak at every age against references that fall from 125 young to 44 mature, and near 35 for the spruce against 47 to 35. [paraphrase]
- **Shedding.** The vigour proxy decays with node age with no floor, so any nonzero shedding threshold strips every preset to about thirty nodes by maturity through the growth path. fn-30 hid this by zeroing the threshold on the ordinary tree and the Two Trees, which the owner did not accept as a fix. [paraphrase]

This spec owns those three defects. It may change fn-11's growth rule, and it is judged on the same strips at the same ages, with fn-30's checksummed references as its curves. fn-30 stays open until this spec's strips earn the owner's accepting verdict. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The rule stays one rule.** Every change lands inside fn-11's yearly slice, as numeric traits every family carries and the blend walks; no species branch and no preset-specific path enters the growth code. The chronicle, the stamps, the read-at-age contract and byte-identical replay are unchanged in kind: a tree at any age at or below the frontier is still a filter over the record. [strategy:Growth and botanical fidelity]
- **Sapling form from year one.** A one-year specimen is a seedling at seedling scale, a stem of tens of centimetres carrying its first leaves for the oak and a few centimetres with a whorl of needles for the spruce, and from there a young specimen carries laterals and foliage in proportion to its size, so the tree is a branched, leafed sapling at every age the strips show, and the crown fills as the envelope expands rather than arriving decades after the stem. The mechanism is the implementer's to choose within the rule; the strips judge it. [inferred]
- **Thickening by age.** Trunk and limb radius follow the tree's age and history as well as the live envelope, so the height-to-diameter ratio falls with age toward open-grown values. Radius keyframes stay monotone and the pipe-model split at forks stays as fn-11 pinned it. [inferred]
- **A floor under vigour.** The vigour proxy keeps a floor for a shoot that still holds light, so a mature exposed branch never sheds by age alone, while a shaded interior shoot still falls below a nonzero threshold and dies as a stamp. The presets' authored nonzero thresholds return; a zero threshold remains a valid row that disables shedding by choice, not as a workaround. [inferred]
- **Derived ages move.** The derived mature age of every preset follows from the changed rule and is documented, never authored; fn-30's routing of production through growth to that age is kept as is. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **fn-30's instrument is the judge.** The age strips open at year one and then run fn-30's ages (oak 1, 10, 26.7, 56.1, 112, 200 and the derived mature age; spruce 1, 5, 14.1, 26.6, 36.9, 60 and the derived mature age), the year-one frame framed to its own size so a seedling is not lost in a mature tree's camera with fn-30's command, seed and protocol, within the owner's capture budget: small before large, at most four images viewed per capture, and one full capture per commit. [paraphrase]
- **The references are fn-30's.** The composed curves, their sources and checksums are reused, not re-sourced. The owner's doubt about the young-age diameter composition stands and is recorded; Stein's mature open-grown figures are the independent check the owner may prefer. [paraphrase]
- **One re-pin.** Identity, audit and look pins move once, after convergence against fn-30's numbers is stated, with the reason recorded; a pin moving by an amount the numbers do not explain is a defect. [paraphrase]
- **Cost is bounded by fn-30's measurement.** A mature build through growth costs no more than fn-30 measured (oak 2,463 ms, spruce 867 ms at seed 7), and the report records the result against the half-second dial-move target that both species already miss. [inferred]
- **Every family still grows.** The Two Trees and the ordinary tree build and grow at every age under the same rule with their nonzero thresholds restored, and their crowns hold at maturity. [paraphrase]
- **Renderer fixtures grow too.** fn-30 left four renderer tests red because two fixture parameter sets grow no wood or no placements through the growth path. That defect is fn-30's to close before its PR; this spec inherits a green tree and keeps it green. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R0, the convergence rule (owner, 2026-09-18), first on resume:** at the preset's age the specimen matches the direct build from the same value table within a per-metric tolerance the owner sets, on every catalogue species and every protocol seed, for node count, primary limbs, branches by order, twigs, placed leaves, leaf area, height and crown box. Growth never gets rows of its own that the mature build does not read. The growth-parity report is the scoreboard; it is measured before any other criterion below is touched. [user]

- **R1:** The oak at 1, 10 and 26.7 years and the spruce at 1, 5 and 14.1 years, rendered with fn-30's strip protocol, read as actual saplings: at year one a seedling at seedling scale with its first leaves or needles, and at the later young ages branched saplings carrying foliage, and every later age reads as a continuation of the same tree; the owner judges the two strips beside fn-30's and records an accepting verdict in the owner's own words. [paraphrase] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R2:** The measured height-to-diameter ratio of the oak and the spruce falls with age, and trunk diameter at fn-30's three fit ages lands within 15 percent of the reference the owner accepts for that age, with any accepted deviation recorded beside the number. [inferred] Errors: a miss outside the tolerance stops the spec with the number unless the owner's recorded judgment accepts it.
- **R3:** With the presets' authored nonzero shedding thresholds restored, the ordinary tree and the Two Trees hold their mature crowns (node counts at the derived mature age within the convergence the report states, never a collapse toward tens of nodes), and shading still sheds interior shoots as death stamps; a test asserts both on the fixture presets. [inferred] Errors: a preset that collapses, or a shaded shoot that never sheds, fails the test naming the preset.
- **R4:** fn-11's contracts hold: same seed and parameters give a byte-identical chronicle on native and wasm, every reader is a filter over stamps, growth traits stay numeric rows the blend walks, and no species or preset branch enters the growth code; the existing determinism and parity tests pass unchanged. [strategy:Growth and botanical fidelity] Errors: a parity or determinism failure names the preset and the age.
- **R5:** Production routing through growth is kept, the derived mature ages are re-documented, the pins are re-pinned once with the convergence numbers stated before the move, and the mature build cost stays at or under fn-30's measurement for both species. [paraphrase] Errors: a cost over fn-30's number stops the spec with the number.
- **R6:** The report, in fn-14's shape, carries the re-rendered strips beside fn-30's, the ratio and diameter tables, the shedding evidence, the cost, and the owner's verdict slots; fn-30's rejecting R3 slots are re-judged from this report. [inferred] Errors: no error surface beyond the protocol.

## Boundaries
<!-- scope: business -->

- No within-species variation, bud-fate modelling, apical-dominance traits or light-response scenarios; those stay with fn-21, which is untouched. [user]
- No environment, competition, pruning, breakage, death or seasons; those stay with fn-16 and the lifecycle specs. [paraphrase]
- No fork anatomy, root collar or junction geometry; that is fn-20's. [paraphrase]
- No smoothing between years and no page work; that is fn-28's. [paraphrase]
- No new reference sourcing; fn-30's curves, sources and checksums are reused. [paraphrase]
- No change to the chronicle's shape or the read-at-age contract; the rule changes inside the slice. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner rejected fn-30's young ages on sight and named the cause: the calibration could not touch the generator. The realism the owner values first lives in the rule, not in the trait values. [user]
- fn-30's diagnosis, routing, references and strips are kept as the instrument; this spec is the fix they call for. [paraphrase]

### Implementation Tradeoffs

- Fixing the rule over hiding the defects in the value tables: fn-30's zero-threshold workaround disabled shedding for three presets and left the ageing proxy in place; the owner did not accept it, so the threshold returns and the proxy gains a floor. [paraphrase]
- A new spec over extending fn-21: the owner chose a new spec so fn-21 keeps its scope as the variation and light-response proof and this spec stays a defect fix judged on existing strips. [user]
- Building on fn-30's branch: fn-30's routing, report and strips land first with its spec left open on the rejecting verdict, and this spec continues from that state; fn-30 closes when this spec's strips are accepted. [inferred]

## Strategy Alignment

- Follows "Growth and botanical fidelity": one continuous tree structure whose lifecycle behaviour is judged against real trees, with reference-based visual QA exposing structural mistakes that counts alone cannot catch. [strategy:Growth and botanical fidelity]

## Parked unknowns

- Whether the composed young-age diameter reference (stand-grown yield table plus an open-grown integration from a 7 cm anchor) is trustworthy, or whether Stein's mature open-grown figures should anchor the diameter judgment; the owner decides at R2 with the numbers in front of them.

## Resolved via Research
<!-- provenance: refine --scope=research (docs-scout, practice-scout, docs-gap-scout, memory-scout, github-scout) on 2026-09-14; plan writes the same section when its Step 1 runs the same scouts -->

### docs-scout
- **National Christmas Tree Association** — Norway spruce plantation: growth in the first ten years after planting is slow; 8 to 11 years to reach a 1.8 to 2.1 m tree. The young end of the spruce curve is slower than the model's 4.8 m at 14 years suggests. Source: https://realchristmastrees.org/education/tree-varieties/norway-spruce/
- **Iowa State University Extension** — Norway spruce in the landscape: moderate to fast when young, about 23 m in 50 years (roughly 0.45 m a year averaged); "stiff when young, becoming more graceful with age". Source: https://naturalresources.extension.iastate.edu/forestry/iowa_trees/trees/norway_spruce.html
- **New Hampshire State Forest Nursery stock list** — Norway spruce two-year seedlings 15 to 20 cm, three-year seedlings about 14 cm mean and 10 cm minimum; a nursery anchor for years 2 and 3. Source: https://buynhseedlings.com/product/spruce-norway-3-0/
- **Oregon Wood Innovation Center (Oregon State)** — Oregon white oak: wild seedlings put down a deep taproot and the shoot stays small and shrubby for many years, then a distinct faster sapling stage follows; cultivated or planted seedlings grow rapidly and skip the shrub stage; stump sprouts reach up to 0.9 m a year for the first three years. No height-by-age table for ages 1 to 25 was found for Q. garryana in any source; the open-grown planted case is the fast one. Source: https://owic.oregonstate.edu/oregon-white-oak-quercus-garryana
- **Open Oregon Forest Measurements (OER textbook)** — spruce whorl counting works to about 15 years; add 2 to 4 years for the gap between germination and the first countable whorl on the main stem, so a five-year spruce carries one to three whorls and a one-year seedling none. Source: https://openoregon.pressbooks.pub/forestmeasurements/chapter/4-4-field-technique-tips-for-counting-whorls/
- **US plant patent PP28693 (Norway spruce cultivar)** — a 50 to 60 cm trunk carries seven whorls of four to five branches; a cultivar datapoint, not a species norm, for whorl density at seedling scale. Source: https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/PP28693
- **Meier et al. 2012, Tree Physiology review (USFS)** — epicormic buds lie dormant under bark and sprout on increased light or stress; leaves on the trunk of a healthy young oak are not normal crown behaviour, so the rule keeps trunk wood bare without an epicormic mechanism. Source: https://www.fs.usda.gov/nrs/pubs/jrnl/2012/nrs_2012_meier_001.pdf
- **Arboriculture and Urban Forestry, structural development of trees** — oaks are decurrent: apical dominance suppresses laterals in the first year, after which laterals often outgrow the leader; a young open-grown oak is branched low and early, never a bare pole. Source: https://auf.isa-arbor.com/content/6/4/105
- **Ancient Tree Forum (Gimber)** — open-grown trees keep branches in every direction from low on the trunk to the top and carry a full billowing crown; photo-illustrated and usable beside a rendered still. Source: https://www.ancienttreeforum.org.uk/recognising-open-grown-trees-megan-gimber/
- **Telewski 1986, Physiologia Plantarum (Abies fraseri)** — unflexed seedlings lean; wind flexing keeps a sapling nearer to vertical. Lean is the default state of an unflexed slender stem. Source: https://pubmed.ncbi.nlm.nih.gov/11538654/
- **Telewski and Pfeffer 1997, Tree Physiology (Ulmus americana seedlings)** — stem flexure lowers height, internode length and slenderness and raises diameter against staked controls; slenderness and sway are coupled. Source: https://academic.oup.com/treephys/article/18/1/65/1622486
- **Wang, Peng and Zhu 1998, Canadian Journal of Forest Research** — the slenderness coefficient (height over diameter) falls with diameter, height, crown length and age; R2's falling ratio is the general forestry pattern. Source: https://cdnsciencepub.com/doi/10.1139/x98-092
- **Rose, Chachulski and Haase 1990, Target Seedling Symposium** — the nursery sturdiness ratio, height over root-collar diameter, is the standard measure of how slender a seedling reads: high is spindly, low is stout; the metric to state for the rendered seedlings. Source: https://rngr.net/publications/proceedings/1990/rose.pdf/at_download/file
- **GrowIt BuildIt, the shape of trees** — illustrated open-grown against forest-grown crown forms by species, a visual key for the crown shape at 10 and 25 years. Source: https://growitbuildit.com/tree-shapes-crowns/
- **No age-labelled photographic series** of either species at 1, 10 or 26 years was found; the illustrated guides above are the nearest visual reference. Source: the scout's search over institutional, extension and nursery sites

### practice-scout
- **Practice:** self-organizing tree models grow from one or a few initial shoots that compete for light from year one, with no separate pole stage; crown shape emerges under the same rule at every age. Source: https://algorithmicbotany.org/papers/selforg.sig2009.html
- **Practice:** whorl-branched conifers show determinate growth: one flush a year yields one whorl of lateral buds at the base of that year's leader, so whorl count is an age proxy and a young spruce shows zero or one whorl, never a bare whip. Source: https://openoregon.pressbooks.pub/forestmeasurements/chapter/4-3-young-trees/
- **Practice:** Weber and Penn, and Arbaro, encode leader bend as a per-segment curvature angle applied across the segments of a branch plus a separate attraction-up tropism vector, so curvature scales with segment count, never with absolute segment length. Source: https://courses.cs.duke.edu/cps124/fall01/resources/p119-weber.pdf, https://github.com/wdiestel/arbaro
- **Practice:** real stems counter self-load bending with reaction wood laid down over years; gravitropic correction is a secondary-growth process racing the bending moment, which is why lean self-corrects on a thickening stem and persists on a slender one. Source: https://nph.onlinelibrary.wiley.com/doi/10.1111/nph.13968
- **Practice:** stem-posture models combine gravitropic sensing with proprioceptive curvature sensing, so a stem corrects its own curvature rather than only its verticality. Source: https://www.sciencedirect.com/science/article/abs/pii/S0022519308005389
- **Practice:** cantilever bending under a static foliage load has an elastic part and a delayed viscoelastic part of 30 to 50 percent of the instant deflection. Source: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6370663/
- **Gotcha:** a crookedness term authored for a mature growth step over-bends a centimetre seedling step unless scaled, and the current rule scales it down with step length; the scaling must not over-straighten at small shoot steps, and bend imparted in thin years must not be replayed unchanged into thick trunk wood or it kinks at the year boundary. Source: the repo's timeline slice and the Weber and Penn per-segment convention above
- **Gotcha:** a tropism applied as a constant pulse per year rather than scaled by how far off vertical the tip already is snaps the stem back to vertical in the year the tropism first dominates; apply it continuously per segment. Source: https://courses.cs.duke.edu/cps124/fall01/resources/p119-weber.pdf
- **Practice:** leaf-bearing stations are restricted to current-year or current-flush shoots so an older trunk segment is bare once its shoots pass the leaf-bearing window. Source: https://www.cof.orst.edu/cof/fs/kpuettmann/CJFR%2048%202018.pdf
- **Practice:** epicormic shoots are a separate bud source on older wood triggered by light or disturbance, with their own faster growth; fn-31 keeps trunk wood bare by gating leaf stations, never by adding that pathway, which is fn-21's territory. Source: https://pmc.ncbi.nlm.nih.gov/articles/PMC2803000/
- **Gotcha:** a single-shape sigmoid fitted to older ages under-predicts young growth; forestry site-index work documents the bias and cures it with a young anchor, polymorphic curves or a multi-anchor fit. Source: https://iforest.sisef.org/contents/?id=ifor1548-008, https://www.sciencedirect.com/science/article/abs/pii/S0378112798005015
- **Practice:** juvenile growth is three-phase, establishment, exponential, plateau, so an explicit establishment term blended into the sigmoid is the standard cure for a first decade that reads too slow; the current rule blends one into height only. Source: https://zslpublications.onlinelibrary.wiley.com/doi/10.1111/jzo.12770, https://lieth.ucdavis.edu/research/phasic/3phas.htm
- **Gotcha:** an establishment term that reaches height alone leaves a correctly tall pole; it must also reach whatever gates lateral recruitment and leaf stations, or the sapling stays unbranched. Source: the scout's cross-reference of the two findings above

### docs-gap-scout
- **Docs that must change:** README.md — the growth paragraph still says the spruce's leaf lifetime is provisionally six; the presets now carry one for every family. Source: README.md:175-176, crates/telperion-core/src/growth.rs:53
- **Docs that must change:** templates/species-profile.md — the growth table asks for three reference ages only; add a juvenile-references row (height by age for years 1 to 25, sapling form and sturdiness ratio) so the next species supplies what R1 judges. Source: templates/species-profile.md:28-29
- **Docs that must change:** docs/species-onboarding.md — extend the growth-evidence paragraph to require the same juvenile references, and revise the fn-31 paragraph that states the round-3 outcomes as settled once R1 resolves. Source: docs/species-onboarding.md:21-29,77
- **No update expected:** STRATEGY.md metrics, scripts/benchmarks/generation.md, tests/migration/README.md, the growth, scaffold, survival and foliage module headers, and the generated browser presets, which regenerate from the Rust traits. Source: the scout's grep over each file

### memory-scout
- **Zero width is not no constraint** — envelope containment stays a hard bound below the crown base; sapling form must not treat a zero-width band as unconstrained. Source: bug/runtime-errors/zero-width-is-not-no-constraint-the-2026-09-04
- **A trunk-region guard on the parent node** — guards test the destination of a step, not the growing node; the same applies to any bare-trunk rule that keeps leaves off trunk wood. Source: bug/runtime-errors/a-trunk-region-guard-on-the-parent-node-2026-09-04
- **Interpenetrating junctions** — junction radius containment is measured against the lobed surface as drawn; thickening by age must keep it. Source: bug/runtime-errors/interpenetrating-junctions-contain-the-2026-09-04
- **A slack band around an orbit pivot** — harness pivot clamps use the actual mesh bounds, which matters when re-pinned trees change size. Source: bug/ui/a-slack-band-around-an-orbit-pivot-2026-09-04
- **An AC that enumerates the default state is the test** — identity and re-pin tests enumerate the default render state; a comment licenses nothing. Source: bug/ui/an-ac-that-enumerates-the-default-state-2026-09-04
- **A second subject on stage** — a capture that compares two subjects accounts for both in every read of what is on screen. Source: bug/ui/a-second-subject-on-stage-makes-every-2026-09-04

### github-scout
- **edisonlee0212/EvoEngine (C++, licence unasserted, active 2026-09)** — the maintained successor of EcoSysLab: tropism rotates by the angle to target scaled by a tropism weight and clamped to that angle, so bend never scales with step length; a sagging cantilever term whose strength falls with segment thickness, so lean freezes out of a thickening trunk; pipe-model thickness accumulation gated by age; foliage placed only where internode thickness is under a bound, keeping older wood bare; a pine growth model with one whorl per leader flush, whorl dormancy years and needle age with senescence. Read for the rules, never copy. Source: https://github.com/edisonlee0212/EvoEngine
- **cran/ForestFit (R)** — fits height or diameter against age with Chapman-Richards, Korf, Weibull, logistic, Gompertz and Prodan forms with starting-value heuristics; read for the functional forms and how a young anchor enters a fit. Source: https://github.com/cran/ForestFit
- **aitorvv/height-diameter_models_Spain (R, MIT)** — height-diameter fits for real species with bibliography; a cross-check on which curve family fits the juvenile and mature portions. Source: https://github.com/aitorvv/height-diameter_models_Spain
- **wdiestel/arbaro (Java, GPL-2.0)** — the Weber and Penn per-segment curve and attraction-up implementation, background for the tropism convention. Source: https://github.com/wdiestel/arbaro
- **edisonlee0212/PlantArchitect (C++, BSD-3, archived 2025)** — superseded by EvoEngine; read that instead. Source: https://github.com/edisonlee0212/PlantArchitect
- **No findings** for root-focused CPlantBox, L-Py examples or the Blender sapling add-on beyond what EvoEngine covers. Source: the scout's search
