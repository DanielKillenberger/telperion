# What holds a crown's spread, and why the old oak has none, 14 Nov 2026

By a Fable reading agent. Nothing was built, run or changed. Findings and a proposal for the host; every design judgment here is the host's to make.

**Tags.** [code] read in `.worktrees/cand/crates/telperion-space/src`. [record] a file under `.flow/`. [source, full text] opened in this reading through a passage-quoting extractor (spot-check a number before it reaches a preset). [source, abstract only] an abstract or search excerpt. [arithmetic] computed here from tagged inputs. [unknown] not determined.

## The answer in short

- **No term in the grower reads how far a limb reaches, and none slows a tip.** The charge is a yearly chance of shedding, zero at break-even, tested on a whole subtree. Every living tip lays its row's whole unit every year until it is shed [code].
- **No source gives a limit on lateral reach near the measured radius.** A hydraulic charge with a height term and a path term, in any ratio a source supports, lets a fully lit low limb run 33 to 60 m of path before it breaks even. Open-grown oaks stand at 9 to 17 m of radius [arithmetic].
- **The spread is not wide for the trunk.** Against the species' own open-grown equation (Paine and Hann 1982) the candidate's spread is 0.86 to 1.05 of the maximum crown width for its own trunk at 40, 80 and 120. Crown and trunk are both 1.4 to 2.1 times ahead of the fastest open-grown Oregon white oak for their age. The disagreement is growth per year from 40 on, one fact seen twice [arithmetic].
- **Every charge and share tried left the spread at 120 where it was** (23 to 38 m under five variants: the square and straight path charges, fn-363, fn-364, and the square charge with the balance counting what would remain) [record]. A limit cannot move it. Only the limbs' yearly travel can.
- **The evidence supports no new limit rule.** It supports extension that falls with size, which this grower has refuted four times for a reason of its own. Section 4 names the difference a fifth form would need, the measurement missing from the literature, and a read-only census as the smallest probe.

## 1. What limits reach and height in the grower as it stands

| Term | What it reads | What it does | Bounds reach? |
|---|---|---|---|
| The charge [code, `grow/cycle/passes.rs:153-163`] | The tip's last node's z above the seed's base; `inverse_height` | `size × (height × inverse_height)²`, none below the collar | No. A level limb's charge never rises |
| The earning [code, `passes.rs:57-60`] | `size = base_scale × running`; the light at the tip | `size × light` | No |
| Light [code, `light.rs:1-8`] | Beer-Lambert through last cycle's leaves | "a node's light is the sky's directions' transmittances", each weighted by its share: at most 1 | Only by self-shade |
| Subtree totals [code, `working.rs:198-224`] | Own line's tips whole; each child lateral's surplus "where that is above none" | A part's Q is its own tips' light plus its children's surpluses; its C is its own tips' charge alone | No: it subsidises |
| Shedding [code, `contract/shed.rs:47-77`; `oak.rs:146-147`] | Memory = (memory + (Q − C)/(Q + C)) / 2; `balance_hazard` 2.0, `tolerance` 0.0 | Yearly hazard 2 × max(0, −memory); shed with chance 1 − e^(−hazard) | Height only, late |
| The unit's size [code, `allocation.rs:145-155`, `working.rs:341-374`; record, fn-345 R1-NOTE §2] | Vigour per presence to ψ, "normalised so each PA's mean unit stays whole" (`species.rs:215-221`) | Moves length between tips of one row; the row's mean never falls | No |
| Node count [record, fn-345 R1-NOTE §2: `grow/unit.rs:37`] | The zone's node law alone | Limb: 6 nodes × 0.035 m = 0.21 m a year at scale 1; bough 0.18 m | No |
| Scale down a relay chain [code, `grow/draw.rs:272-290`] | The parent's `base_scale × running` | A relay starts at its parent's scale: no loss along a limb | No |
| Lifespan and relays [code, `oak.rs:243-245, 264-273, 293-295, 310-318`] | Limb 500 years; bough 60, then `relay_ended` 1.0; abortion 0.15 and 0.1 with `relay` 1.0 | A stopped apex is always carried on by a relay at its last node, one unit a year [record, RELAYS.md] | No |
| Shade death [code, `species.rs:203-207`, `oak.rs:93, 290`] | Survival = viability^(light^−φ) | Limb viability 1.0: dormant. Bough 0.999 | No |
| Ground [code, `oak.rs:139`] | `ground_hazard` 0.0 | A limb on the ground runs on | No |
| Sag [code, `species.rs:300-305`, `oak.rs:29`] | Bending moment over radius⁴, 4ρg/E | Turns wood down; breaks nothing | No. It lays ascending limbs level, which turns path into radius |
| Sleeping buds [record, fn-346 RENEWAL.md] | A drawn time only: delay 6, rate 0.04 | "Nothing in `dormant.rs` or `grow/wake.rs` reads light, the balance or the shedding of the wood beyond" | No renewal from within |

**Why tips pass 1/inverse_height.** Three causes, all in the code; none is light above 1 or stored surplus.

1. **The comparison does not stop extension.** Size is set by `allot_one` from light and presence [code, `passes.rs:123-146`] and reads no charge. A tip above break-even lays the same unit as one below.
2. **At break-even the hazard is zero and it rises slowly.** With tolerance 0 the hazard is 2 × |memory|. A lone tip at light 1 and h = 1.1 H has (1 − 1.21)/(1 + 1.21) = −0.095, a hazard of 0.19, a 17% chance a year, and its memory closes half the gap each year [arithmetic on `shed.rs:53-54`]. At 0.2 m a year it rises a metre before it is likely to go.
3. **A bearer is tested with its children's surpluses and without their charges.** A limb whose own tip stands above H keeps a positive balance while paying laterals hang below it [code, `working.rs:217-219`]. The recorded light at the top is about 0.6 (tips under fn-364 stalled at 0.77 H, and 0.77² = 0.6) [record, log 14 Nov 03:24], so a lone tip breaks even at 26.9 m. The tips stand at 36 to 39 m, 9 to 12 m above their own break-even [arithmetic].

**So the charge is a deadline.** Extension is constant, so any threshold on height or path becomes an age at which wood dies. It is never a size at which wood settles.

## 2. What the literature gives

| Candidate | Source and passage | The relation, and its variable |
|---|---|---|
| Open-grown crown width, the species itself | Paine and Hann 1982, Maximum crown-width equations for southwestern Oregon tree species, OSU For. Res. Lab. Res. Pap. 46 [source, full text]. Oregon white oak row "3.0786 \| 1.9242 \| 0.0"; 75 trees, D "0.6 - 37.3" in, adjusted R² "0.9633". Selected for "A crown presently free of surrounding crown competition", "Limbs extending to the ground or nearly to the ground", "Lowest limbs as long or longer than limbs above". "The Model [1] equations should not be applied to D values larger than the quantities given in Table 2." | MCW ft = 3.0786 + 1.9242 D in: radius gains 0.1155 m per cm of trunk [arithmetic]. Crown width against **trunk diameter**; linear, no ceiling inside 95 cm |
| Open-grown crown radius, *Q. robur* | Pretzsch et al. 2015, 95% quantile, ln(cr) = −1.4370 + 0.8157 ln(d) [record, fn-341 R1-MEASURES.md] | Radius against **trunk diameter**; still rising at every d |
| How fast the trunk can grow | Gould et al. 2008: "The maximum predicted diameter growth of an open-grown oak (SI, 100) is 1.27 in./5years" [record]. Gould et al. 2011: "The maximum growth rate is predicted when DBH is about 40 cm, stand BA is low (e.g., open-grown trees) and SI is high" [source, abstract only: search excerpt] | 0.645 cm a year at most, peaking near 40 cm: a **rate against size** |
| Height against size | Gould, Harrington and Devine 2011: "potential height was found to increase with DBH at an almost constant rate for trees < 30 cm DBH and then level-off so that little additional height growth is predicted for trees > 60 cm" [record, fn-341] | Height growth ends by **trunk size**, by degree |
| Hydraulics of this species | Phillips et al. 2003 [source, full text]: "Values for KL ranged from ≈0·1–0·5 and 0·2– 0·8 mmol m−2 s−1 MPa−1 for 25 m and 10 m trees"; "Midday Ψl was not different between 10 m and 25 m trees, reaching approximately −3·0 MPa in both"; predawn "reaching −0·9 MPa in October" at 25 m, "about −0·3 MPa" at 10 m; "25 m trees had a leaf to sapwood area ratio 1·6 times greater" | Conductance per leaf falls by 0.5 to 0.63 from 10 to 25 m [arithmetic]. Stated against "tree height and associated hydraulic path length"; the two are not separated |
| Friction against gravity | Cruiziat, Cochard and Améglio 2002, Hydraulic architecture of trees: main concepts and results [source, full text]: "the gradient of xylem pressure is much smaller in the trunk (roughly 0.02 MPa m–1) than in the branches, at least ten times larger" (*Thuja*); "the leaf resistance expressed as a percentage of the total resistance between trunk and leaves is 80 to 90% for Quercus"; junctions less conductive "by a factor of 1.1–1.5". Burgess et al. 2006: "Vertical tension gradients due to gravity are ∼0.01 MPa m−1 ... Frictional tension gradients vary" [source, abstract only] | Friction is per metre of **path**, about equal to gravity in a trunk and ten times it or more in branches (a conifer). In oak most of the resistance is in the leaf, which no length changes |
| Branch length itself | Waring and Silvester 1994: "aspect of exposure and branch length accounted for up to 6‰ δ13C difference within the foliage of individual trees of Pinus radiata". Walcroft et al. 1996: "artificial manipulation of branch length by pruning produced evidence consistent with an increase in hydraulic conductance" [both source, abstract only] | The one measured effect of **horizontal path**: lower stomatal conductance on long branches. A pine; no growth function |
| The hypothesis | Ryan, Phillips and Bond 2006: "increased path length (in roots, stems and branches) decreases leaf-specific hydraulic conductance"; "no evidence supports the original expectation that hydraulic limitation of carbon assimilation is sufficient to explain observed declines in wood production" [record, fn-345 R1-NOTE] | Path and height through conductance; not written as a growth rate, and found insufficient |
| Size, not age | Bond et al. 2007: "size, not age, drives developmental changes in height growth"; Mencuccini et al. 2005 [record, fn-345] | Extension declines with **size** |
| Old oak crowns | Rust and Roloff 2002: "in addition to increasing pathway length and lower conductivity of xylem in old trees, structural changes in shoot and crown architecture need to be considered" [record, fn-345]. Not reached again here | No function |
| Mechanics | Bertram 1989, through "Tree Branching: Leonardo da Vinci's Rule versus Biomechanical Models" (PMC3979699): distal stems "disproportionately slender", older elements near "elastic self-similarity" [source, abstract only, secondary] | Length against diameter. No maximum length, no safety factor of a horizontal oak limb reached [unknown] |
| Renewal | Ishii, Ford and Kennedy 2007: "Crown maintenance by epicormic sprouting is a normal life-history trait"; "delayed adaptive reiteration may help maintain crown productivity and increase longevity" [source, abstract only] | Old crowns are held by death and replacement from within. No rate |
| Branch autonomy, carbon export; increments of old limbs against their length | None opened | [unknown] |

**What the sources agree on.** Crown radius is given against trunk diameter and rises with it without a stated ceiling. Rates fall with size. Nothing gives a length at which a limb stops, and nothing gives yearly limb extension against limb length for an open-grown oak [unknown].

## 3. Was the path charge wrong in kind, or in size

**In kind it was half right.** Friction is real and runs along the path; gravity runs with height. A charge on height alone leaves out a term at least as large per metre as the one it keeps.

**The ratio.** In a trunk friction is about 0.01 MPa per m beside gravity's 0.01 (the *Thuja* figure of 0.02 taken as the two together) [arithmetic on Cruiziat]. In branches it is ten times more. For this oak, gravity at 25 m is 0.245 MPa of a soil-to-leaf drop of 2.1 to 2.7 MPa, 9 to 12%; at 34.8 m it is 0.34 MPa, 13 to 16% [arithmetic on Phillips]. Height is a small share of this species' water budget. The split of the rest between roots, wood and leaf in *Q. garryana* is [unknown].

**On paper.** Take the head as h + ρL metres, ρ the friction per metre of path over gravity's, and normalise so a straight vertical tip at H is charged 1: `reach = (h + ρL) / ((1 + ρ) H)`. A tip at light Λ breaks even at reach = √Λ. Take a fully lit edge tip, Λ = 0.9, on a low limb at h = 6 m, with a bole path of 4.7 m [record, fn-346: "limbs from 4.7 m"]:

| ρ | Break-even path L | Reach from the trunk, path straight to 1.5 times tortuous | The age it is reached at 0.21 m a year from year 14 |
|---|---|---|---|
| 0 (today) | never | no bound | never |
| 1 (a trunk's ratio) | 60 m | 37 to 55 m | 276 |
| 2 | 46 m | 28 to 42 m | 212 |
| infinite (the old path charge) | 33 m | 19 to 28 m | 149 |

Open-grown radius: 8.7 to 10.9 m at 120 and 14.6 to 17.3 m at 300 (Pretzsch at White's trunks), 10.1 to 13.0 m at 120 (Paine and Hann at White's trunks) [arithmetic].

**So:** the low limb survives under every ratio, until an age. Its break-even reach is 1.1 to 6 times the measured radius, so no ratio holds the crown at its measured size. The record agrees: under the square path charge (V0) the spread at 120 was 24 and 32 m, under the straight one (PR) 26 and 33 m (rough layout), beside fn-363's 25.6 and 36.2 [record, SURPLUS-PROBE.md, SQUARE-RESULT.md]. The path charge then killed every limb between 150 and 300, on the table's schedule: leaf 739, 199, 0.9 m² at 120, 200, 300 on seed 1001 [record].

**The path charge was wrong in two sizes.** Its scale took all of the head as friction, which halves a low limb's allowance against ρ = 1. And it was the only thing that ever ended a limb, in a grower where extension never slows and nothing regrows nearer the trunk. A sourced ratio moves the deadline from 149 to 212 or 276 years. It does not remove it.

## 4. The proposal

### 4a. The reading that changes the question

The reading of this morning compared the spread with the *Q. robur* quantile at White's trunk for the age. Against the candidate's own trunk, on the species' own equation:

| Age, seed | Trunk cm | Spread m, recorded | Paine and Hann at that trunk | Ratio | In the equation's range (to 94.7 cm) |
|---|---|---|---|---|---|
| 40, 1 | 27 | 7.5 | 7.2 | 1.05 | yes |
| 40, 7 | 33 | 7.4 | 8.6 | 0.86 | yes |
| 80, 1 | 73 | 17.8 | 17.8 | 1.00 | yes |
| 80, 7 | 98 | 24.0 | 23.6 | 1.02 | 3% past |
| 120, 1001 | 115 | 25.6 | 27.5 | 0.93 | extrapolated |
| 120, 1007 | 165 | 36.2 | 39.0 | 0.93 | extrapolated |
| 120, 1012 | 134 | 30.7 | 31.9 | 0.96 | extrapolated |

[arithmetic; trunks and spreads from fn-363 and fn-364 MEASURES.md]. Against Pretzsch at the same trunks the ratios are 0.90 to 1.20. The radius gained per cm of trunk from 40 to 80 is 0.112 and 0.128 m on seeds 1 and 7, beside the equation's 0.1155.

**The crown fits its trunk. The tree is ahead of its age.** At Gould's fastest open-grown growth the trunk is 52 cm at 80 and 77 cm at 120, and the equation gives 12.9 and 18.7 m of spread. The candidate stands at 1.4 to 1.9 times that spread and 1.4 to 2.1 times that trunk [arithmetic]. Against White's trunks the spread at 120 is inside on seed 1001 (20.1 to 26.1 m) and 1.39 and 1.18 times the top on the other two.

**Where the years are gained.** Radius per year [arithmetic]:

| Span | Candidate | White's trunk growth × Paine and Hann | Gould's fastest × Paine and Hann |
|---|---|---|---|
| 0 to 40 | 0.093 | 0.093 to 0.128 | 0.075 |
| 40 to 80 | 0.129 and 0.208 | 0.081 to 0.112 | 0.075 |
| 80 to 120 | about 0.10 to 0.15 (other seeds) | 0.078 to 0.086 | 0.075 |
| 120 to 200 | 0.068, 0.106, 0.111 | 0.055 to 0.062 (extrapolated) | |

To 40 the limbs ascend at about 60° (`elevation` 1.05) and half their travel is radius. From 40 on, seed 7's radius advances by the limb row's whole nominal 0.21 m a year: the limbs lie level and nothing has slowed them. A measured oak's radius advances at 0.08 to 0.11 m a year throughout and slows after 120.

### 4b. Verdict: the evidence supports no rule that limits reach

- **No charge in a sourced ratio bounds the crown** (section 3), and no charge ever tried moved the spread at 120 [record].
- **Adding the friction term alone is not worth building.** On paper ρ = 1 leaves spread, leaf and trunk at 120 as they are, lowers the top by the factor (1 + ρ)/(1 + ρτ), τ the top tips' path over their height (0.87 at τ = 1.3; τ is [unknown], the path is still carried in `Pencil.path` and read by nothing [record, fn-363 R1-READING]), and sets a deadline of about 276 years on the lowest limbs. The young tree would not move: at 40 the reach is under 0.35.
- **What the sources do support is extension that falls with size**: height growth ending by 60 cm of trunk (Gould), conductance per leaf halving from 10 to 25 m (Phillips), size and not age (Bond, Mencuccini). That is the family this grower has refuted four times: fn-345's H (internodes × (1 − L/34.8)) and N (node count × the same), fn-364's share, and R9 [record].

### 4c. What the four refuted forms share, and what a fifth would need

Each failed inside the grower, not in its botany:

1. **The slowed tip does not die.** A limb's viability is 1, its abortion is always relayed, and the balance's hazard is zero at break-even. fn-364's share reached zero exactly where the hazard does, so the tip was pinned there for centuries [record, log 14 Nov 06:10]. N's limbs "live on and keep making a node or two a year" [record, fn-345 RESULT.md].
2. **A relay's kink is paid per year, its straightening per metre**, so shorter units coil [record, CURVATURE.md, RELAYS.md]. R9 moved tortuosity from 57.7 to 40.3 where fn-363's own is 47.1.
3. **Leaf stands only on nodes an extending bearer laid that year** [record, RENEWAL.md, "For the host" 1], so a slow bearer is a bare one.
4. **Nothing regrows from within.** Sleeping buds wake by a timetable. Waking them without a bound was also tried: "At 300 all four are wreckage", and one seed "does not stop growing" [record, RENEW-RESULT.md].

A real crown at its size does what item 1 lacks: tips at the limit die and are replaced from behind (Ishii et al. 2007). In this grower only fn-363 has that turnover, at the top: "fn-363's tip rises, is shed and is renewed from below" [record].

**The one form the evidence points to, for the host to judge.** A tip's unit falls linearly with its head and the balance stays as it is: `unit × (1 − reach)`, reach as in section 3, with ρ as the one new row and a row that scales the fall, zero being today's rule. It is general, by degree and has no species term beyond a potential height and a friction ratio.

- **Why linear:** the measured water potential is linear in height ("Ψ = −0.00973h − 0.712", Koch et al. 2004 [record, fn-345]).
- **What it does to a tip at its limit:** the balance sheds from reach² = light, so from reach = √light. There the unit is 1 − √light of whole: 0.23 at the recorded top light of 0.6, 0.05 at 0.9. A shaded tip keeps moving, crosses its break-even, gains a hazard, dies and is replaced from below, as fn-363's does. A fully lit edge tip nearly stalls, which is items 1 to 3 again at the crown's edge.
- **On paper** (ρ = 1, limb unit 0.21 m, low limb at h = 6 m from year 14, top at 0.2 m a year with path 1.2 times height) [arithmetic, crude]:

| | Today | With the fall | Measured |
|---|---|---|---|
| Height at 40 | 10.7 to 11.1 m | about 9.9 m (−8%) | none for open-grown |
| Low limb's path at 40 | 10.5 m | 9.4 m (−10%) | |
| Height at 120 | 26 to 28 m | about 18.5 m; asymptote 31.6 m | mature trees 15 to 27 m (Silvics) |
| Low limb's path at 120 | 27 m | 21 m (0.77) | |
| Spread at 120 | 25.6 / 36.2 / 30.7 m | about 20 / 28 / 24 m | 20.1 to 26.1 (Paine and Hann at White's trunks) |
| Leaf at 120, with ground area | 968 / 1,974 / 1,389 m² | about 570 / 1,170 / 820 m² | |
| Trunk at 120, section following the summed leaf | 1.15 / 1.65 / 1.34 m | about 0.95 / 1.35 / 1.10 m | 0.83 to 1.09 (White) |
| Low limb's path at 300 | 65 m, shed before | 39 m | radius 14.6 to 17.3 m (Pretzsch) |

- **Where it stands against the record:** at the top it is H with half the slope. H and N were run at L/34.8 with the growth rows restated by 1.19 to hold the tree at 40, and N's oak at 120 was "a thin, sparse, radiating crown", trunk 69 cm against White's 83 to 109, with vines from 200 [record]. This form differs in three ways: half the slope, the balance left free to shed and renew, and a target at 120 that is smaller than today's tree where fn-345 judged against today's. Whether that is enough is not shown by anything here. I do not recommend building it before the census below.

### 4d. The smallest probe: a read-only census, no rule

One oak, seeds 1001 and 1007, grown once to 300 on the candidate with a reader patch, printing every decade:

- **A.** Spread, trunk at 1.3 m, and radius gained per cm of trunk.
- **B.** For the living limb and bough tips: units laid in the decade, mean size, mean elevation of the outer 2 m, and radial distance gained.
- **C.** Each living tip's path over its height, and over its radial reach.
- **D.** Light and leaf by tip, in bands of reach (h + L)/(2H).

**Bars, set beforehand.**

- **The rate reading holds** if A stays within 0.85 to 1.15 of 0.1155 m per cm from 40 to 120 on both seeds, and B shows limb tips gaining at least 0.15 m of radius a year in some decade after 40 at a mean size within 10% of 1. Then the excess is the limbs' level travel and no limit is missing. It fails if size has already fallen below 0.8 while radius still gains: then the cause is elsewhere and this report's reading is wrong.
- **The linear form is worth a build** only if D shows under a tenth of the leaf at 120 on tips with light above 0.85 and reach above 0.7 (few tips would stall at the edge), and C's top-tip ratio is under 1.5 (the top would not fall below about 25 m). Otherwise it is refuted on paper and is not built.

### 4e. What is missing

- **From the literature:** yearly extension of an open-grown oak's limb leaders against limb length or tree size; extension of level against ascending shoots; the split of the oak's hydraulic resistance between root, wood and leaf; a horizontal limb's safety factor. None was found.
- **From the grower:** the four census lines above. Path, elevation and size by tip are carried and unrecorded.
- **A decision that is the host's:** whether real limbs keep ascending at the tip (so half their travel is radius for life) is a question about the lay and sag, and nothing in this reading settles it.

## Sources opened here

- Phillips, Bond, McDowell, Ryan and Schauer 2003, Funct. Ecol. 17: 832-840, https://research.fs.usda.gov/download/treesearch/9298.pdf [full text].
- Paine and Hann 1982, https://ir.library.oregonstate.edu/downloads/b2773w92s [full text].
- Cruiziat, Cochard and Améglio 2002, https://hal.science/hal-01190863v1/file/70532_20091203104054054_1.pdf [full text].
- Walcroft et al. 1996, Tree Physiol. 16: 281; Waring and Silvester 1994, Tree Physiol.; Ishii, Ford and Kennedy 2007, Tree Physiol. 27: 455; Burgess et al. 2006, Plant Cell Environ. (doi 10.1111/j.1365-3040.2005.01415.x); Gould, Harrington and Devine 2011 (growth sentence); PMC3979699 on Bertram 1989 [abstract or excerpt only; the two Tree Physiology PDFs did not load].
- Not reached: Ryan and Yoder 1997; Tyree and Ewers 1991; West, Brown and Enquist; Rust and Roloff 2002 (search returned nothing); Mäkelä 1986 and Valentine 1985 on the pipe model's own bound on height growth, which would be the next to read.
