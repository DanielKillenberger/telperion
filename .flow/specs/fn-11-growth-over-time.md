# Growth over time

## Goal

A tree is a specimen with an age: age in years is one more continuous input to the tree space, so any family at any age is a valid tree, a walk along age is the same tree growing, and family, seed and age always give the same tree. Growth is live (advance by any fraction of a year, branches keep identity, thicken, extend and self-prune), every family grows by one rule, and the owner asked for it to be deterministic, reversible as a read, and clean "from the ground up". On 2026-09-13 the owner split calibration, production routing, the age strips and the mature stills into fn-30; this spec is the machine, the chronicle, the bindings, the harness page and the cost report.

## Decisions

> user (refine, 2026-09-12): "it should be a continuous parameter like every other parameter. We're building a continuous multidimensional tree space. Trees should eventually be able to die too. But maybe we can just grow to maturity for now and leave dying to a later spec (probably needs to be optional for games anyway)"
> user (refine, 2026-09-12): "it should be deterministic so we don't need to store history, parameters+seed+age should give the exact same tree."
> user (refine, 2026-09-12): "age parameter has to apply to any kind of tree. Spruce and oak we calibrate specifically. But growing should [work] for any tree parameter set."
> user (refine, 2026-09-12): "we should try and find open-grown curves and match them as closely as possible. The closer to reality the better. We'll judge what rules over what once we get there."
> user (refine, 2026-09-12): "if we build the generator to incrementally build based on age steps efficiently enough that a mature tree still generates within reasonable time it might be the better approach?"
> user (refine, 2026-09-12): "Time should be an input like every other parameter and you should be able to step along any dimension of the tree space and have a valid tree."
> user (2026-09-13, ratifying the foliage amendment): "amendment 1 i can ratify ofc"
> user (2026-09-13): "can we make the growth rule easily reversable? => if you have tree at t you could apply the step function in reverse and get tree at t-1?"
> user (2026-09-13): "hm will this be clean the way we're building this with fn-11 and fn-28 separate? I'd like to have a clean algo that is well designed from the ground up not patched together."
> user (2026-09-13, on rewriting the architecture around the chronicle): "ok let's go"
> user (2026-09-13, on splitting calibration, routing, the strips and the stills into fn-30): "alright let's do it"

- **The chronicle** (owner, 2026-09-13): one append-only lifetime record; the simulation is its only writer (births and death stamps, never deletes); the tree at an age, the change record, the snapshot and a backward step are pure filters over stamps. Generational identities, never reused; storage never compacts except past a consumer-set history cap.
- **Stepped growth over the existing builder** with retained scaffold, attractor and local frontiers, chosen over a closed form (emitting N elements is O(N) anyway; the closed form stays the owner's reserve).
- **Yearly slices, chosen by the number**: mature oak 5.906 s monthly vs 0.775 s yearly against a 0.5 s target. Age is an integer year count plus integer remainder, never a float; one jump equals any chain of advances byte for byte.
- **Pinned pure-Rust math** (`libm`) after the parity probe found the shipped builder differed between native and wasm on all five presets.
- Thickening as radius keyframes under a resize tolerance (raised to 0.1 mm by measurement); shedding by vigour as death stamps; growth traits (Chapman-Richards rate and shape, leaf lifetime) instead of a maturity field; growth saturates so age past maturity is free.
- **Foliage by living shoot and cohort** (ratified 2026-09-13) after the lifetime rule left 13 placements on the mature oak.
- One task per spec (owner's standing rule since 2026-09-12). Persistence is the seed; snapshots are the exception and never carry the mesh.
- Boundaries: no tree death, seasons, environment/damage response (fn-16), foliage variation or light benchmark (fn-21), GPU-swept wood, or artificial rate in the core.

## Acceptance Criteria

- **R1:** A specimen develops recognizably through successive ages with coherent identity, thickening and foliage — met.
- **R2:** Supported ages and time controls (pause, advance, backward inspection), replay reproduces history, invalid requests explicit — met.
- **R3:** Update latency and memory measured across growth stages — met; validation of developmental form moved to fn-30.
- **R4:** Age is a continuous family field and every preset and blend grows by one rule at any age; past maturity costs the build to maturity — met.
- **R5:** Fresh build at t equals any sequence of advances to t, byte for byte — met, native and wasm.
- **R6:** Identity through growth; vigour shedding retires identities; each advance returns a reconciling change record — met.
- **R7:** Shoot state (birth year, bud fate, vigour); leaf cohorts within the leaf lifetime — met.
- **R8:** Calibration against open-grown curves — moved to fn-30 R1 (2026-09-13).
- **R9:** Live growth on the harness page with the age dial, rebuild and play rate — met.
- **R10:** Cost of a slice and a mature build measured beside today's build — met (see Result).
- **R11:** Age strips and owner verdict on the mature oak and spruce — moved to fn-30 R3 (2026-09-13).
- **R12:** A mature tree keeps its foliage within 10% at maturity plus twice the leaf lifetime — met.
- **R13:** Any age at or before the frontier reads back as a filter, byte-identical to a fresh build — met.

## Result

Built as the machine across 36 commits; all five gates green; native-to-wasm parity on every preset. Costs (native, three-build medians, shared machine):

| | oak | spruce |
|---|---:|---:|
| envelope build, one-shot | 67.8 ms | 37.1 ms |
| mature build through growth, 173 yearly slices | 1,273.5 ms | 932.9 ms |
| sparse yearly advance | 13.4 ms | 20.6 ms |
| sparse record plus packed read | 6.6 ms | 13.3 ms |
| read at an earlier age, mature tree | 262.3 ms | 2,314.7 ms |
| snapshot round trip, mature tree | 320.7 ms at 121.9 MB | |

A slice costs what it changed (sparse oak record plus read 88.0 ms -> 6.6 ms; spruce sparse advance 1,500.3 ms -> 20.6 ms once foliage read the wood only). Reads at an earlier age stay linear in the chronicle. The grown mature tree is not yet today's tree (oak nodes +29.3%, spruce -15.3%, bounds within 2.3% and 6.0%) because presets author zero shedding threshold and calibration had not run; production stayed on the envelope build (fn-30 R2).

## Open

- Calibration, production routing, re-pin, age strips and the owner's verdict: fn-30.
- Mature build through growth is about 17x the envelope build on the oak; the closed-form alternative remains the owner's reserve.

## Resolved via Research
<!-- provenance: refine --scope=research (docs-scout, practice-scout, docs-gap-scout, memory-scout, github-scout) on 2026-09-12; plan writes the same section when its Step 1 runs the same scouts -->

### docs-scout
- **Palubicki et al. 2009, self-organizing tree models** — one iteration is one season in the order estimate environment, determine bud fate, append shoots, shed branches, update width; the spec's slice order is this loop and should say so. Source: https://algorithmicbotany.org/papers/selforg.sig2009.html
- **Vigour and apical control, extended Borchert–Honda** — resource splits between a terminal and its laterals by one trait λ with λ above 0.5 excurrent (spruce) and below decurrent (oak); the spec's unit budget across the frontier is that split and the habit traits already carry the oak and spruce forms, so no new species branch is needed, and the paper's own space-colonization variant with binary light is the spec's crown-depth proxy. Source: same paper
- **Shedding, Takenaka 1994 via Měch and Prusinkiewicz 1996** — shed when light gathered over branch size falls under a threshold; Palubicki warns the binary-light variant sheds a branch the moment it stops growing, so the spec's tolerance period is what makes shedding by a proxy usable. Source: same paper, section on branch shedding
- **Thickening** — the pipe model as the n-th power sum of child widths with n between 2 and 3, and width is never decreased when branches are shed, so the model keeps a memory of past branches; independent support for the spec's monotone radius rule and its identity-retained frontier. Source: Macdonald 1983 via Palubicki 2009; Shinozaki et al. 1964, review https://academic.oup.com/aob/article/121/5/773/4820933
- **Plastic Trees, Pirk et al. 2012** — lazy local re-transform of a skeleton only where the environment changed, to avoid re-running the growth procedure; the closest prior art to the retained frontier plus change record. Source: https://kops.uni-konstanz.de/bitstream/123456789/21500/2/Pirk_215002.pdf
- **Functional-structural plant models (LIGNUM, GreenLab, AMAPsim)** step annually and allocate a carbon budget; the spec's unit budget from growth traits is a geometric surrogate with no photosynthesis, a simplification to record, not a novelty. Source: https://sal.aalto.fi/publications/pdf-files/rper09.pdf, https://arxiv.org/pdf/q-bio/0701026
- **Weber and Penn 1995** has no time dimension; the alternative the spec is not using. Source: the paper, and openalea/weberpenn below
- **Quercus garryana, Silvics of North America (Stein 1990)** — open-grown form described; mature 15 to 27 m at 60 to 100 cm DBH; 6 to 8 rings per cm common; the forest-grown pair 95 to 135 years to 24 m and 48 cm is stand-grown; no age curve. Source: https://research.fs.usda.gov/silvics/oregon-white-oak
- **Gould, Harrington and Devine 2011, Growth of Oregon white oak** — diameter increment against size and competition with an explicit open-grown class, and DBH to height; age is not a predictor. Source: https://cascadiaprairieoak.org/wp-content/uploads/2014/07/Gould-P.J.-C.A.-Harrington-and-W.D.-Devine.-2011.-Growth-of-Oregon-White-Oak-Quercus-garryana.pdf
- **Saltspring Island dendrochronology** — 39 savanna-grown Garry oaks crossdated 1825 to 2005; ring widths give cumulative radius against age; no heights. The closest real DBH-age series for the oak. Source: https://bioone.org/journals/tree-ring-research/volume-68/issue-1/2011-5.1/Dendrochronological-Potential-of-Quercus-garryana-Saltspring-Island-British-Columbia/10.3959/2011-5.1.short
- **GOERT** publishes stewardship material, no growth or age tables; the spec's assumption of GOERT growth data is unconfirmed. Source: https://goert.ca/species/quercus-garryana/
- **Hasenauer 1997, open-grown trees in Austria** — 435 genuinely open-grown trees of 11 species including Picea abies: crown width, DBH to height, crown base, taper; no age. Best open-grown spruce shape source. Source: https://www.sciencedirect.com/science/article/abs/pii/S0378112797000571
- **Pretzsch et al. 2015, crown size in urban centres, parks and forests** — species-specific open-grown crown-radius to stem-diameter allometry for 22 species; verify Picea abies and an oak are among them; diameter-indexed, not age-indexed. Source: https://www.sciencedirect.com/science/article/pii/S1618866715000473
- **Pretzsch and Caicoya 2025, estimating tree age from height** — a sigmoidal height-age function parameterized for Picea abies and Quercus robur/petraea, fitted on dominant trees of long-term plots, so stand-grown; the best height-age fit for the spruce and a proxy for the oak; coefficients unverified. Source: https://www.semanticscholar.org/paper/a446fa8a4696498498826f7f47185469b6140222
- **Bavarian yield-table extracts (Assmann and Franz 1963 spruce, Jüttner 1955 oak)** — stand-grown, age in ten-year steps with heights and mean diameter per site class; a single checksummable PDF, the practical R8 fallback for the age axis. Source: https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf
- **Needle turnover of Norway spruce** — roughly six to seven needle cohorts, about half a cohort shed by 5.5 years and all by about 12; supports a spruce leaf lifetime near five to six years against the oak's one; figures from a search snippet, verify in the paper. Source: https://www.researchgate.net/publication/233391503_Needle_and_branch_biomass_turnover_rates_of_Norway_spruce_Picea_abies
- **USDA Urban Tree Database (McPherson, van Doorn and Peper 2016)** — the only source found that fits age to DBH directly, plus DBH to height and crown, over about 14,000 street and park trees with age recorded; differencing gives per-year increments; whether Quercus garryana and Picea abies are among its species could not be verified through the fetch block and must be checked first. Source: https://research.fs.usda.gov/treesearch/52933, data https://www.fs.usda.gov/rds/archive/catalog/RDS-2016-0005
- **No single equation** gives per-year height and diameter increments for open-grown specimens of either species; the honest R8 composition is an age axis from a stand-grown source, converted to open-grown proportions through open-grown allometry, with the Silvics pairs and the Saltspring rings as the oak's independent checks, each provenance named. Source: the scout's synthesis over the items above
- **Functional form** — every age-indexed source is sigmoidal in height; Chapman-Richards or Korf with the asymptote tied to the authored envelope height lets one growth-trait pair, rate and shape, reproduce them; no open-grown coefficients are published. Source: https://blogg.slu.se/forest-biometrics/2017/03/11/the-chapman-richards-growth-function/

### practice-scout
- **Gotcha:** only add, subtract, multiply, divide and square root are bit-identical across targets in Rust; sin, cos, exp and powf vary by platform and version and wasm is named, so every transcendental in the growth path must go through one pinned pure-Rust implementation such as the libm crate or R5's byte-identity fails between native and wasm. Source: https://github.com/rust-lang/rust/issues/150323, https://rust-lang.github.io/rfcs/3514-float-semantics.html
- **Gotcha:** wasm arithmetic is IEEE deterministic, so parity holds if no fused multiply-add or reassociation differs between builds; keep the simulation exact and the renderer free. Source: https://rapier.rs/docs/user_guides/rust/determinism/
- **Gotcha:** the budget split across the frontier must iterate in identity order, never storage order, or compaction permutes the random draws. Source: scout synthesis
- **Gotcha:** accumulate age as an integer month counter plus an integer sub-month remainder in a fixed denominator, never a float accumulator, or partitions of an advance drift apart. Source: https://gafferongames.com/post/fix_your_timestep/
- **Gotcha:** decide shedding from state snapshotted at slice start, with a fixed comparison side at thresholds and a fixed traversal order for every reduction, or which limb sheds depends on visit order. Source: scout synthesis
- **Gotcha:** compact-and-renumber with raw index handles is the ABA bug; generational keys (slot plus version) make stale handles fail loudly, and a dense slot map is the shape the spec wants rather than a hand-rolled index map. Source: https://docs.rs/slotmap/latest/slotmap/
- **Gotcha:** derive the change record from a diff of identity sets, not from appends at mutation sites, and give resized a tolerance or every run reports resized every slice. Source: scout synthesis
- **Gotcha:** in validated self-organizing models the sensitive parameters are resource flow and shedding rate; failure modes are leader death under basitony and progressive lower-crown thinning from high shedding; apical control that weakens with age is what turns young-excurrent into old-decurrent and is a growth trait, not a constant. Source: https://pmc.ncbi.nlm.nih.gov/articles/PMC11128038/, Palubicki 2009
- **Gotcha:** oscillating vigour comes from same-slice feedback; the tolerance period is the hysteresis, and cap sheds per slice. Source: scout synthesis
- **Gotcha:** a spatial index for attractor association is the per-slice cost lever in space colonization, and the per-slice fixed costs scale with live node count, about 1,200 solves for a century at monthly slices. Source: https://algorithmicbotany.org/papers/colonization.egwnp2007.large.pdf
- **Gotcha:** Chapman-Richards asymptote and origin are ill-conditioned extrapolations; reparameterize to expected values at known ages, which R8's three ages already are. Source: http://derekogle.com/fishR/examples/oldFishRVignettes/VonBertalanffy.pdf
- **Gotcha:** open versus stand-grown is a diameter difference, not a height one; a stand-grown table under-reports DBH, so state the height-to-diameter ratio per age. Source: https://pmc.ncbi.nlm.nih.gov/articles/PMC2987550/
- **Gotcha:** urban trees are not truly open-grown (pruning, soil, irrigation); name that caveat when using the Urban Tree Database. Source: https://research.fs.usda.gov/download/treesearch/41665.pdf
- **Measure first:** the radius solve alone per slice on today's mature oak; a native-to-wasm bit-parity probe over the existing builder by hashing the node buffer; the sourced curve against today's mature presets before fitting; the crown-depth vigour distribution on today's mature tree to see whether one threshold separates shed from keep at every age. Source: scout synthesis

### github-scout
- **edisonlee0212/EvoEngine, EcoSysLab (C++, active 2026)** — a maintained Palubicki descendant whose shoot descriptor is the trait table this spec needs: growth rate, apical control, apical dominance with a loss term, bud extinction rate, thickness accumulation and age factors, end-node thickness, a pruning descriptor, and a leaf hang time that is the leaf lifetime; read for the trait vocabulary and the thickening split. Source: https://github.com/edisonlee0212/EvoEngine
- **edisonlee0212/PlantArchitect (C++, BSD-3, archived 2025)** — the same rules in a small readable form, with space colonization run as a resumable per-iteration behaviour, the pattern the monthly slice needs; the best single file before the thickening and shedding step. Source: https://github.com/edisonlee0212/PlantArchitect
- **openalea/lpy (L-Py)** — growth as deterministic timestep rewriting with seeded streams and derivation length as age; the closest maintained reference for replay-to-age equals build-at-age. Source: https://github.com/openalea/lpy
- **CPlantBox (C++ and Python, GPL-3)** — a plant model stepped by fixed simulate(dt) increments with organ birth times; its slice API and birth-time bookkeeping map onto the change record. Source: https://github.com/Plant-Root-Soil-Interactions-Modelling/CPlantBox
- **LPFG-LIGNUM2** — the only public LIGNUM code, unlicensed; read for the annual cycle, never copy. Source: https://github.com/inuritdino/LPFG-LIGNUM2
- **Arbaro and the Blender sapling generator (Weber and Penn lineage)** — mature envelope only, no age; the negative case showing age cannot be a post-hoc scale. Source: https://github.com/wdiestel/arbaro
- **ropensci/allodb (R, GPL-3)** — a curated allometry equation database with Quercus and Picea and a source per equation; the audited path to height and diameter relations. Source: https://github.com/ropensci/allodb
- **No Rust or wasm tree generator with incremental growth exists in public code**; the reuse is conceptual, the trait vocabulary and the allometry, and the implementation is new. Source: scout's code search

### docs-gap-scout
- **Docs that must change:** README.md build example, field-handle paragraph, architecture opening and the branching and foliage rows, the harness paragraph, the measurements section and its report links, the species section — age beside the seed, the specimen handle and its invalidation, the slice loop, leaf lifetime, the age dial and play rate, per-slice cost, calibration age as documentation. Source: README.md:5-16, 33-43, 47, 52, 55, 88, 128-147, 160-170
- **Docs that must change:** docs/species-onboarding.md — a profile owes open-grown curves with sources and checksums, growth traits, leaf lifetime, and a calibration age as documentation; the failure table gains a missing-curve row and a three-age miss row. Source: docs/species-onboarding.md:11-13, 45-51
- **Docs that must change:** templates/species-profile.md — calibration age and derived mature age, height-at-age and DBH-at-age rows with source ids and checksums, growth-rate and leaf-lifetime capability rows. Source: templates/species-profile.md:19, 25-39
- **Docs that must change:** STRATEGY.md wording on age as a trait and the per-slice cost beside the Build metric; scripts/benchmarks/generation.md stage table gains per-slice and mature-build rows; tests/migration/README.md retained-ownership table gains identity survival, replay equivalence, radius monotonicity, shedding slice and change-record reconciliation. Source: STRATEGY.md:15,27,35; scripts/benchmarks/generation.md:35-44; tests/migration/README.md:58-87
- **Docs that must change:** module headers of branching.rs, branching/scaffold.rs, colonization.rs, radius.rs, twigs.rs and foliage.rs state one-shot passes and must say resumable per slice; tree.rs has no header and needs one covering nodes, the crossover invariant, identities and shoot state; the wasm lib and src/browser/core.ts types gain the specimen handle, advance and the change record. Source: crates/telperion-core/src/*.rs:1, crates/telperion-wasm/src/lib.rs:1, src/browser/core.ts:20-72

### memory-scout
- **zero-width-is-not-no-constraint** — an envelope invariant bug in the old colonizer; the same per-step guard applies to the stepped loop. Source: bug/runtime-errors/zero-width-is-not-no-constraint-the-2026-09-04
- **a-trunk-region-guard-on-the-parent-node** — a growth guard placed on the wrong node; the kind of per-step invariant a slice must get right. Source: bug/runtime-errors/a-trunk-region-guard-on-the-parent-node-2026-09-04
- **interpenetrating-junctions-contain-the** — the junction and radius invariant that re-solving radii each slice must keep. Source: bug/runtime-errors/interpenetrating-junctions-contain-the-2026-09-04
- **a-slack-band-around-an-orbit-pivot** and **a-second-subject-on-stage** — harness pivot and stage-state pitfalls that live playback of a growing subject will meet. Source: bug/ui/a-slack-band-around-an-orbit-pivot-2026-09-04, bug/ui/a-second-subject-on-stage-makes-every-2026-09-04
