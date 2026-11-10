# fn-345 R1: what the texts say, where a shoot's yearly growth is set, H's two wood defects, the arithmetic, and B's memory

Drafted by the worker on 10 Nov 2026 at `36fb456f8`, for the host. Nothing is built and nothing is proposed. Questions about how the system should work are under "For the host".

Marks used below:
- **measured:** printed by a run.
- **read:** taken from the code.
- **arithmetic:** computed from logged numbers, not grown.
- **CHECKED:** the quoted sentence was read in the source.
- **UNCHECKED:** it was not.

## Headline

- **No source gives shoot extension as a function of path length.** Path length appears only in the proposed mechanism. Every measured trend is against height, height class or size. The one closed-form decline found is forestry's monomolecular rate, kA(1 − H/A). Its asymptote A is fitted to stand data for each site class, not taken from hydraulics.
- **Vigour never sets how many nodes a shoot makes.** The node count comes from the zone's node law alone (`grow/unit.rs:37`). Vigour acts only through the bud's size, and the size is normalised so that each PA's mean is 1 (`allocation.rs:132`). Two consequences follow:
  - A decline applied to vigour cancels out, unless it differs between tips of one PA. Even then it only moves length between those tips.
  - H shortened only the internode. The same nodes, with the same laterals and the same pipe each, were packed into less length.
- **H's girth step is today's step, made larger.** The bole is an untapered cylinder up to the node where the fork's limbs leave, and the radius drops there by a quarter. This happens in A as well: radius 0.711 → 0.546 m, ratio 0.77. In H it is 0.947 → 0.706 m, ratio 0.75 (measured at 120).
- **The open tube at 200 and 260 was not measured.** The code offers one cause (read): only relays get a drawn join. A lateral or continuation tube starts at its node with its own radius, so a bearer ring much wider than everything leaving it is drawn as an open rim.
- **A single restating of the growth values cannot hold the oak at both 40 and 120 under 1 − L/34.8** (arithmetic).
  - Multiplying every shoot's extension by 1.19 holds the height at 39 at today's 10.6 m. The tree then stands 20.5 m at 119, against today's 25.9 m (rough layout).
  - Holding 119 instead needs a factor of 1.83, which makes the tree 14.7 m at 39.
- **B's memory difference is not reproduced.** Two runs of follow-up 1's exact configuration on today's probe build gave the same tree as every earlier B run, phytomer for phytomer. Their memory matched follow-up 3's, not follow-up 1's (3.02 and 3.03 GiB at year 119, against 5.20). The requested age changes the tree by 0.2% at most, through the presence windows.

## 1. The published basis

The texts were read by a delegated reading agent. It used the research and web tools and quoted the sentences below. Most full texts were behind paywalls. Where a quote comes from an abstract, the note says so. I did not re-read the sources myself.

**The hydraulic limitation hypothesis.**
- **Ryan and Yoder 1997** (BioScience 47:235).
  - **The original:** UNCHECKED, paywalled.
  - **Its wording, as quoted by Ryan, Phillips and Bond 2006:** CHECKED in that paper's full text (Plant Cell Environ 29:367, doi:10.1111/j.1365-3040.2005.01478.x): "The hydraulic limitation hypothesis proposes that increased path length (in roots, stems and branches) decreases leaf-specific hydraulic conductance as trees grow in height. … Closure of stomata will restrict the diffusion of CO2 into the leaf and reduce net photosynthesis and tree growth and perhaps the ultimate height of the tree."
  - **The three factors**, as the 2006 paper lists them: "(1) increased resistance with increasing length of the hydraulic path; (2) increased gravitational potential opposing the ascent of water in taller trees; and (3) maintenance of a species-specific minimum water potential in leaves."
  - **What the hypothesis predicts:** path length and height enter through conductance and stomatal closure. Neither is written as a growth rate.
- **Ryan, Phillips and Bond 2006**, abstract, CHECKED:
  - "height growth slows down as trees grow taller".
  - "Where hydraulic limitations to A do occur, no evidence supports the original expectation that hydraulic limitation of carbon assimilation is sufficient to explain observed declines in wood production."
- **Koch, Sillett, Jennings and Davis 2004** (Nature 428:851, doi:10.1038/nature02417).
  - **Abstract,** CHECKED: "increasing leaf water stress due to gravity and path length resistance may ultimately limit leaf expansion and photosynthesis"; "Our regression analyses of height gradients in leaf functional characteristics estimate a maximum tree height of 122–130 m".
  - **Body text,** CHECKED from an extracted PDF:
    - "Ψ = −0.00973h − 0.712", taken to −1.9 MPa, gives 122 m.
    - "LMA = 37.43 exp(0.0255h)".
    - Maximum photosynthesis "would decline to 0 at 125 m".
  - **How the maximum height is derived:** each trait regression on height is extrapolated to an assumed limiting value.
  - **Shoot length:** none found (UNCHECKED, the body was not fully read).
- **Ishii et al. 2008** (Oecologia 156:751), abstract, CHECKED: "Leaf and shoot morphology changed continuously with height". It gives no form for shoot length.

**Shoot extension against height, path length or size.**
- **Meinzer, Bond and Karanian 2008** (Tree Physiol 28:197), CHECKED: "needle length decreased with increasing height". The variable is height.
- **Woodruff and Meinzer 2011** (Plant Cell Environ 34:1920), CHECKED: trees sampled in height classes from about 2 to 57 m; "Branch NSC was inversely correlated with midday shoot water potential … and shoot extension". It reports a correlation and no form.
- **Bond et al. 2007** (Tree Physiol 27:441), CHECKED, a grafting study:
  - "cuttings from old-growth trees showed a 10-fold increase in stem elongation rate within 2 years of grafting onto seedling rootstock";
  - "size, not age, drives developmental changes in height growth in Douglas-fir".
- **Mencuccini et al. 2005** (Ecol Lett 8:1183), CHECKED: "size, not cellular senescence, accounted for the observed age-related declines in relative growth rates".
- **Rust and Roloff 2002**, on old *Q. robur* (Tree Physiol 22:597), CHECKED: "in addition to increasing pathway length and lower conductivity of xylem in old trees, structural changes in shoot and crown architecture need to be considered". It gives no growth function.
- **Yoshimura 2011**, *Q. crispula* (doi:10.1093/treephys/tpr060), CHECKED: "Shoots in the upper crown had longer and thicker axes and lower water potential". Within one crown, the shoots farthest from the root are the longest, where light dominates.
- **Bond 2000:** UNCHECKED, not read.

**The standard height-growth forms.**
- **Lei and Zhang 2004** (Nonlinear Analysis), CHECKED. The rate equations are written in current size, not age:
  - **monomolecular:** "dy/dt = k(α − y)", which is kα(1 − y/α), a linear decline in size;
  - **Bertalanffy–Richards:** "dy/dt = ηy^m − ry";
  - **the integrated form:** "y = A[1 − B exp(−kt)]^(1/(1−m))";
  - **the asymptote,** in their words: "some final value as determined by the genetic nature of the living organism and the carrying capacity of the environment".
  - **Von Bertalanffy as m = 2/3:** UNCHECKED.
- **Where forestry takes the asymptote from:** Lumen, Natural Resources Biometrics ch. 9, CHECKED. Site-index curves come from "measuring [dominant trees'] total height, and statistically fitting the data to a mathematical equation"; "The asymptote should be some function of site index". The asymptote is fitted, per site class.
- **The oak's height:** European Atlas of Forest Tree Species, Eaton et al. 2016, CHECKED: "grow to be 30 to 40 m in height"; "More usually, these oaks achieve a height of 30 m". The 34.8 m that H used comes from fn-341's `R1-MEASURES.md`, which this R1 did not re-check.

**What the texts give, in plain terms.**
- **Path length:** no source states shoot extension, or any growth decline, as a function of it. Path length is named in the mechanism (Ryan and Yoder, Koch, Rust and Roloff), never as the variable of a measured decline.
- **Height and size:** every measured gradient is against height, a height class or size. The traits measured are water potential, turgor, leaf mass per area, needle length and shoot hydraulics, and their slopes are linear or exponential in height.
- **Age:** the grafting studies say the driver is size, not age.
- **The only closed form,** kA(1 − H/A), is a fitted stand-level curve of a tree's height. It is not a law for each shoot.
- **Within one crown,** the one oak source (Yoshimura) finds the longest shoots at the top, the opposite of a penalty on a tip's own path.

## 2. Where a shoot's yearly growth is set in the grower (read)

The steps from a tip's light to the wood it lays each year:

1. **Vigour.** `allot_one` (`grow/cycle/passes.rs:104`) splits the tree's light by Borchert–Honda, `allocation::split` (`allocation.rs:50`). Each bud's vigour is its share of the tree's light under apical control λ: 0.45 on the oak's trunk rows, `oak.rs:116`.
2. **Size.** `sizes` (`grow/cycle/working.rs:336`, the same rule as `allocation.rs:139`) turns vigour into a size, (v/p)^ψ, normalised so that every PA's presence-weighted mean size is 1. The comment on `allocation.rs:132` says: "light moves growth within an order, never from one order to another." ψ is `shade_size`: 1.5 on the trunk rows, 0.5 on limb and bough, 1.0 on the fine orders.
3. **Node count.** The node count is drawn from the zone's node law alone: `lineage::node_presences(zone.nodes, …)` at `grow/unit.rs:37`, with each node a Uniform{min, max} draw (`lineage.rs:357`). Vigour, size and light do not enter. Every node then places its buds by the zone's lateral shares (`grow/unit.rs:45`), so the laterals per year follow the node count, not vigour.
4. **Internode length.** The length is `state.internode × scale × varied` (`geometry/lay.rs`, `Layer::step`). The scale is multiplied by the unit's size before the step, in the rough layout (`grow/draw.rs:378–380`) and in the final lay (`presence::assign_one`, where `phytomer.size = size`).
5. **Leaf area** is read with the size (`grow/draw.rs:228`, `light::leaf_area(scale, size, …)`).
6. **Wood upkeep** is internode × scale × size (`grow/draw.rs:385`).
7. **Girth** is the pipe model over each phytomer's presence (`girth.rs`, `own_at`: pipe × scale × ripening). It reads neither size nor length: a phytomer adds its full pipe however short it is.

**The oak's rows, as each yearly shoot is laid at scale and size 1** (read, `oak.rs`). Mean nodes are the sum of the zones' Uniform means.

| row (PA) | internode m | mean nodes a year | nominal shoot m a year | lifespan yr | ψ |
|---|---|---|---|---|---|
| trunk | 0.045 | 4.5 + 2.5 + 1 = 8 | 0.36 | 12 | 1.5 |
| fork | 0.045 | 3.5 + 2.5 + 1 = 7 | 0.315 | 2 | 1.5 |
| leader | 0.05 | 7 | 0.35 | 6 | 1.5 |
| limb | 0.035 | 2.5 + 2.5 + 1 = 6 | 0.21 | 500 | 0.5 |
| bough | 0.03 | 6 | 0.18 | 60 | 0.5 |
| sprig | 0.031 | 1.5 + 2.5 + 1 = 5 | 0.155 | 10 | 1.5 |
| branch | 0.017 | 5 | 0.085 | 12 | 1.0 |
| twig | 0.012 | 1 + 2.5 + 1 = 4.5 | 0.054 | 5 | 1.0 |
| shoot | 0.015 | 1 + 2.5 + 1 = 4.5 | 0.068 | 4 | 1.0 |
| short | 0.006 | 4 | 0.024 | 3 | 1.0 |

**What a decline must reach so that a slowed tip makes fewer and shorter shoots** (read; the choice among these is the host's):
- **The node count.** It sits in `grow/unit.rs:37`, upstream of everything a node makes. Today nothing a tip experiences reaches it.
- **The internode length.** It sits in `Layer::step`, both lays.
- **The vigour, before `sizes`?** Not on its own. Any factor common to every tip of one PA cancels in the normalisation. A factor that varies by path only moves length from far tips to near tips of the same PA, keeping each PA's mean at 1.
- **The size, after the normalisation?** It reaches length, leaf area and upkeep, but not the node count.

**What H did, and why nodes piled up** (read, and measured in fn-344):
- **What it changed:** H multiplied the length alone, in `Layer::step`.
- **What it left as it was:**
  - every tip's node count (the zone's law);
  - the buds and laterals at every node;
  - each phytomer's pipe (girth reads presence, not length);
  - each phytomer's presence, survival and the light it reads.
- **The result:** the held phytomers grew as before or faster (B's whole-tree growth). They went into a crown whose extent the factor stopped, so tip density rose from about 1,470 to 3,040 tips per m² of ground.
- **The knots:** the limbs became short segments that each carry a full node's pipe.
- **Tips far from the root:** under `max(0, 1 − L/34.8)`, every tip whose path along the wood passes 34.8 m lays internodes of length 0 and still makes all its nodes. On a crown 25 m wide standing on a 4.4 m bole, the outer tips' path is near or past that bound (inferred from the spread, not counted).

## 3. H's two wood defects

**The run** (measured): `probes/runs.sh stem` ran inside `telperion-heavy 12 --threads 4`. It grew A and H (`ABLATE=BH`) on seed 1001 to 120, grow-only, with `probes/oldoak-stem.patch`: fn-344's patch plus a dump of the stem line's phytomers and the laterals on it below 10 m. A was added beside H as the reference; it is the same short run. The logs are `raw/oak-120-1001-{A,BH}-stem.log`. The patch is reverted, and `git status --short crates` is empty.

**The girth step** (measured, settled tree at 120):

| | A | H |
|---|---|---|
| bole radius at the base, m | 0.714 | 0.950 |
| bole radius below the fork's limbs, m | 0.711 (z 4.72) | 0.947 (z 4.43) |
| change from base to there | −0.4% | −0.3% |
| radius one internode above, m | 0.546 | 0.706 |
| ratio across the step | 0.77 | 0.75 |
| limbs leaving at that node, first radius m | 0.50, 0.29 | 0.63, 0.49 (and 0.30 at z 4.71) |
| internode on the bole at 4 m, m | 0.045 | 0.040 |
| settled phytomers | 2,370,405 | 3,599,503 |

- **Where the step is.** It is the pipe model's drop at the node of the fork's year-13 top cluster, where its limbs leave. The node and the ratio are nearly the same in A and H.
- **Why the bole is a cylinder.** Between the root and that node, nothing living leaves the bole: every sprig on the trunk is shed, and the oak's `retained` is 0, so a shed branch leaves none of its pipe (`oak.rs` state, `girth.rs` header). The bole's radius therefore stays the whole tree's pipe sum all the way up. It then drops by about a quarter at the one node where the fork's limbs leave.
- **What H changed.** H did not make the step. H holds 1.52 times A's phytomers, so its bole is 1.33 times A's radius and the drop is 24 cm of radius against A's 16.5 cm. That drop happens over one internode of 4 cm. In A, the two V limbs that diverge from that node partly hide the same step.
- **H's internodes on the bole** are shortened by only 10% at 4 m, so the bole's own length plays no part in the step.

**The open tube at about 4 m, seen at 200 and 260.** Not measured: the dispatch allowed one run, at 120. The code gives one cause (read):
- **Relays.** A relay gets a drawn join that carries the bearer's ring into it (`crates/telperion-core/src/pipeline/space/join.rs`, fn-244).
- **Laterals and continuations.** They start at their node on the bearer's line with their own first radius.
- **The rim.** Where everything leaving a node is much thinner than the bearer's last ring, that ring is drawn as an open rim, with the wood above leaving from inside it.
- **Why H shows it.** At 120 the step in H is the same proportion as A's. H's bole at 200 and 260 is 2.6 to 3.0 m across (fn-344's table). The thickness of what leaves the fork node at those ages was not measured, so it is UNCHECKED that this is the tube. The same `PROBE_STEM` dump on H at 200, one grow-only run of about two minutes, would settle it.

## 4. A beside H, and the arithmetic

**Every 10 years to 129** (measured): fn-344's 300-year grow logs on seed 1001, rough-layout heights before the settle; `probes/ah10.py`.

| year | A height m | H height m | H / A | A spread m | H spread m | H / A held |
|---|---|---|---|---|---|---|
| 9 | 3.2 | 3.1 | 0.96 | 1.4 × 1.3 | 1.3 × 1.3 | 1.00 |
| 19 | 6.7 | 6.1 | 0.91 | 3.5 × 3.2 | 3.3 × 3.1 | 0.98 |
| 29 | 8.7 | 7.8 | 0.89 | 5.0 × 5.1 | 4.8 × 4.8 | 0.96 |
| 39 | 10.6 | 9.2 | 0.87 | 6.8 × 8.0 | 6.4 × 7.6 | 0.91 |
| 49 | 12.4 | 10.7 | 0.86 | 10.5 × 9.8 | 9.8 × 8.7 | 0.92 |
| 59 | 14.0 | 11.9 | 0.85 | 13.4 × 9.0 | 11.8 × 10.4 | 0.92 |
| 69 | 15.5 | 12.9 | 0.83 | 14.1 × 12.1 | 12.6 × 8.8 | 0.94 |
| 79 | 17.4 | 14.2 | 0.82 | 16.4 × 14.1 | 14.3 × 10.8 | 0.95 |
| 89 | 19.6 | 15.1 | 0.77 | 19.5 × 16.3 | 15.4 × 13.1 | 1.01 |
| 99 | 21.9 | 16.3 | 0.74 | 22.4 × 19.0 | 17.5 × 13.7 | 1.10 |
| 109 | 23.9 | 17.4 | 0.73 | 22.7 × 21.9 | 18.1 × 15.2 | 1.13 |
| 119 | 25.9 | 18.4 | 0.71 | 24.4 × 22.4 | 17.9 × 17.2 | 1.52 |
| 129 | 27.4 | 19.3 | 0.70 | 20.2 × 23.4 | 18.0 × 18.3 | 2.58 |

**What 1 − L/34.8 costs a tip** (arithmetic):

| path L m | factor | cost | factor after the 1.19 restating |
|---|---|---|---|
| 5 | 0.856 | 14.4% | 1.02 |
| 10 | 0.713 | 28.7% | 0.85 |
| 20 | 0.425 | 57.5% | 0.51 |
| 30 | 0.138 | 86.2% | 0.16 |

**One restating of the growth values** (arithmetic, `probes/restate.py`):
- **The method.** A's leader height each year is replayed with each year's gain multiplied by k (1 − H/34.8). The path is taken as the height, a lower bound on the path.
- **The replay matches H's run.** With k = 1, the replay reproduces H's own leader within 0.1 to 0.9 m at every age from 39 to 299, so for the leader the path is close to the height.
- **Holding 39 at today's 10.6 m needs k = 1.19:** every row's internode, or its nominal shoot, 19% longer.
- **The tree it gives at later ages:**

| year | today (A) m | H as run m | k = 1.19 m |
|---|---|---|---|
| 39 | 10.6 | 9.2 | 10.6 |
| 59 | 14.0 | 11.9 | 13.2 |
| 79 | 17.4 | 14.2 | 15.6 |
| 99 | 21.9 | 16.3 | 18.4 |
| 119 | 25.9 | 18.4 | 20.5 |
| 159 | 32.8 | 22.1 | 23.5 |
| 199 | 38.7 | 24.6 | 25.8 |
| 299 | 50.2 | 27.3 | 28.8 |

- **At 119 the oak stands 20.5 m against today's 25.9 m, 21% lower.** Holding 119 instead needs k = 1.83, which stands the oak 14.7 m at 39, against today's 10.6, and 32.5 m at 299.
- **Why no single k does both.** Under this form, a factor that holds 40 cannot hold 120 as well: the decline's cost at 15 to 25 m (35 to 70%) is already larger than any single k makes up.
- **What the arithmetic leaves out.**
  - Every tip is replayed as the leader. A lateral's path is longer than its height, so its cost is higher than this table's.
  - The spread is not modelled.
- **Today's height growth already slows with age.** A's mean gain is 0.271 m a year to 39, 0.192 from 39 to 119, and 0.135 from 119 to 299 (measured). A decline on top of that slows a curve that already slows (see "For the host").
- **Today's oak overshoots the measured height.** A stands 50.2 m (rough) at 299, against the 30 to 40 m the Atlas gives.

## 5. B's memory and tip counts against the requested age and between runs

**What the logs already show** (measured, fn-344 `raw/stills`):
- **The tree is identical between the two 300-year runs of B on seed 1001.** Follow-up 1 is `oak-300-1001-Bh-census.log`; follow-up 3 is `oak-300-1001-B-grow.log`. They match in held phytomers and living tips at every year compared: 564,166 and 42,686 at 49; 4,712,939 and 307,400 at 119; 13,803,452 and 913,389 at 187. Only resident memory differs: 5.20 against 3.02 GiB at 119, and 19.97 against 8.50 at 187.
- **The 150-year request** (`oak-150-1001-Bbal.log`) grows a slightly different tree from the 300-year one: 563,924 against 564,166 held at 49, and 7,649,711 against 7,633,559 at 149 (0.2%). Its memory is within 3% of follow-up 3's at the same years (4.62 against 4.74 GiB at 149).
- **So the "9.8 against 4.6 GiB" in fn-344** compared follow-up 1 with the 150-year run. It mixed the run with the age.

**The pair of runs** (measured): `probes/runs.sh mem` ran inside `telperion-heavy 12 --threads 4`. It grew B, requested 300, on seed 1001, grow-only, twice. It used follow-up 1's exact settings (no `PROBE_BALANCE`) on today's probe build, stopped at 6 GiB. The logs are `raw/oak-300-1001-Bmem{1,2}.log`.

| year | held (both runs) | tips (both runs) | run 1 GiB | run 2 GiB | follow-up 1 GiB | follow-up 3 GiB |
|---|---|---|---|---|---|---|
| 49 | 564,166 | 42,686 | 0.48 | 0.48 | 0.53 | 0.47 |
| 99 | 3,141,562 | 212,390 | 2.04 | 2.06 | 3.07 | 2.07 |
| 119 | 4,712,939 | 307,400 | 3.02 | 3.03 | 5.20 | 3.02 |
| 149 | 7,633,559 | 500,841 | 4.74 | 4.74 | 9.76 | 4.74 |

Both runs stopped at 6 GiB in year 166, holding 10,057,415, after 84 s each.

**The read:**
- **Run to run, today's build is repeatable:** the same tree to the phytomer, and the same memory within 1%.
- **The balance record does not explain follow-up 1:** with `PROBE_BALANCE` off, as in follow-up 1, the memory is follow-up 3's.
- **What follow-up 1's run had that these do not:** the build of 18:13, before follow-up 3 extended the patch. That build was not kept and cannot be rebuilt.
- **Follow-up 1's 2.3 times memory per phytomer is therefore unexplained and not reproduced.** The candidates left are that build and that run's machine state. A different tree is excluded.

**What depends on the requested age** (read):
- **The presence windows.** `Windows::new(species, request.age)` (`grow.rs:201`, `presence.rs:47`) sets every presence window: a node's, a survival's, a stop's. Each window comes from the bud's expected wood from its cycle to the requested age, as a share of the whole tree's wood at that age (`Windows::wood`, `presence.rs:70`, which indexes `self.age + 1 − cycle`). The same bud at the same year therefore gets a different window under a 150-year and a 300-year request, and so slightly different partial presences. That is the 0.04 to 0.2% held difference above.
- **The sleeping-bud buckets.** `asleep` is `vec![…; age + 2]`, one bucket per year of the request (`grow.rs`, `Grower::seeded`): small.
- **The rule that skips certain sheds.** `shed_certain` reads `rules.age` (`grow/shoot.rs:309`). It is dormant in the one cycle, because `Paused::with` sets `lasting` to none (`grow/cycle/paused.rs:105`).
- **The last year's sketch.** It is skipped (`sketching = cycle < request.age`, `paused.rs:203`). That affects only the final year.
- **Nothing found reserves memory or sizes the light lattice by the requested age** (read; `Field::quantised` takes the cycle's leaves and tips).

## For the host

1. **The form, given the texts.** No source gives a per-tip decline in path length. Height, size and the stand-level kA(1 − H/A) are what is published. Should the decline read a tip's path, its height, or the tree's size? The one oak source within a crown (Yoshimura) finds the top shoots longest.
2. **Where it acts.** Vigour cannot carry a decline through `sizes`, because each PA's mean is 1. Should the decline act on the node law (fewer nodes), on the size after the normalisation, or on both?
3. **How the young tree is held.** One factor cannot hold both 40 and 120 under 1 − L/34.8 (section 4). Is holding 40 the bar, with 120 allowed to fall about 21%, or does the form need to cost less at small L?
4. **The age decline already in today's tree.** Today's leader already slows from 0.27 to 0.14 m a year without any decline. Is that slowing meant to stay under the new rule, or to be replaced by it?
5. **The tube at 200.** Should one `PROBE_STEM` run of H at 200 be spent to confirm the tube's cause?
6. **Follow-up 1's memory.** It is not reproducible. Is it enough to record it as an artefact of that build?

## Files

- `probes/ah10.py`: the A and H table.
- `probes/restate.py`: the restating arithmetic.
- `probes/build.sh`, `probes/runs.sh`: the build and the two runs.
- `probes/oldoak-stem.patch`: fn-344's patch plus the stem dump. It is reverted and re-applies cleanly.
- `raw/`, ignored: `oak-120-1001-{A,BH}-stem.log`, `oak-300-1001-Bmem{1,2}.log`, `build.log`.
