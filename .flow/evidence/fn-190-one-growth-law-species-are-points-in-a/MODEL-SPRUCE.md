# Spruce (Massart): model specification, fn-190 task 3, 2026-10-03

Stage A for *Picea abies*: reading only, no code. It lists the rules of Massart's model as they apply to the spruce, each with its quote, then proposes settings and values on the R1a engine, names the engine features the rules need, and lists the traits the owner judges. The values and features are proposals for the host's vocabulary (task 5); the host decides.

The source texts are in the research worktree's `.firecrawl/lit/` folder. Tags follow LITERATURE.md: **[read]** is the source's own text, **[secondary]** is a lecture or another paper stating it, **[snippet]** is a search-result snippet only, and **[unsourced]** means no passage in the folder states it. Engine claims come from reading `organ.rs` and `params.rs` at `3984bce2`; none were run. They are marked **[code]**.

## The model-name conflict and the choice

The sources disagree on the model's name:

- **Massart.**
  - CIRAD lecture [CIR, secondary], `cirad.md` l.555–557: "the common epicea the, Picea excelsa, which has a model which conforms to Massart's model, that is, horizontal branches stages even if the extremity tends to straighten up. So, we are between Massart's and Rauh's models as we saw this morning."
  - Lin et al. 2018 [LIN18, read], `pmc5826307.md` l.402, citing Prusinkiewicz & Remphrey 2000: "the Massart model relates to low P4 and low P7 values (corresponding to the PA species)". PA is *Picea abies*.
  - Petri/L-system paper [PR00, read], `petri.md` l.1796, a citing snippet: "Massart's model for spruce (Prusinkiewicz and Remphrey, 2000)".
- **Rauh.** Kędra et al. 2019 [K19, read], `biorxiv795286.md` l.83: the Rauh model encompasses "taxa of such wide-spread genera as *Quercus, Pinus, Picea* and *Acer*". The claim is genus-level.
- **Intermediate forms.** Edelin's abstract [secondary], `millet.md` l.2606: conifers grow by "RAUH, MASSART, ATTIMS et MANGENOT", but "les modalités, particulièrement nuancées, de l'expression de la plagiotropie conduisent souvent à des architectures intermédiaires entre plusieurs modèles".

LIN18 contradicts itself. Its Table 4 (l.421) gives *Picea abies* "Orthotropic, morphogenetically equivalent to the trunk | More or less continuous". That is its own description of the Attim model (l.273), not Massart's. Each row of the table carries the next model's description in the l.273 order (Massart, Rauh, Roux, Attim), so the table is probably misaligned. The text at l.402 is the assignment; the table row is not used here.

**Choice.** The rules below follow **Massart**, for three reasons:

- the species-level sources name it (CIR, LIN18 via PR00), while K19's Rauh is genus-level;
- the reference photographs show plagiotropic, near-horizontal branch tiers (S1, S3), which is Massart's defining branch type;
- the spec and the catalogue table name Massart.

The CIRAD qualifier ("the extremity tends to straighten up… between Massart's and Rauh's models") is kept, not dropped. It is the one place the spruce sits off the pure Massart point toward Rauh, in the branch tip's orthotropy (rule M6). On a continuous space this is a degree, not a second model.

## Rules of the model

| # | Rule | Quote | Source |
|---|---|---|---|
| M1 | **Monopodial, indeterminate trunk.** The leader never stops and is never replaced. | "Massart's model. The growth of the trunk is monopodial, rhythmic, and indeterminate." (`petri.md` l.982) | PR00 [read] |
| M2 | **Orthotropic trunk.** The trunk is erect, radially symmetric and branches in all directions. | "Orthotropy … refers to axes whose general orientation is vertical and whose symmetry is radial, with leaves in a spiral, opposite or verticillate disposition, and associated lateral branches arranged in all spatial directions." (`bc2007.md` l.394); spruce: "orthotropic trunk and plagiotropic branches, such as … in fir trees or spruces" (`cirad.md` l.191) | BC07 [read]; CIR [secondary] |
| M3 | **Rhythmic growth, with branches in whorls (tiers).** Each yearly growth unit of the trunk ends in one whorl of branches. | "Main branches are plagiotropic and are produced in whorls." and the production `A → O[B]nA` ("the trunk as a sequence of orthotropic segments O and the associated whorls of lateral apices B") (`petri.md` l.984–990); "horizontal branches stages" (`cirad.md` l.556) | PR00 [read]; CIR [secondary] |
| M4 | **Rhythm establishes with age.** The seedling grows almost continuously, then slows, stops and becomes rhythmic, finally one growth unit a year. | "in the spruce species you see that the first year the growth is almost continuous … As of the second year, we can see that there is a slowing down … more obvious after the third year where there is true stoppage … the fourth year when there is a true polycyclism … and finally the plant will be essentially monocyclic at the end." (`cirad.md` l.139) | CIR [secondary] |
| M5 | **Plagiotropic, bilaterally organised branches.** Main branches are horizontal to slanted and carry their own laterals in one plane. | "Plagiotropic axes … have a general horizontal to slanted orientation and a bilateral symmetry owing to leaves (distichous phyllotaxis) and branches being generally arranged in one plane." (`bc2007.md` l.394); "Branches plagiotropic but never by apposition, monopodial or sympodial by substitution … Massart's model" (`key.md` l.143) | BC07 [read]; HOT key [read] |
| M6 | **The branch tip turns up** (the Rauh side of the spruce). | "horizontal branches stages even if the extremity tends to straighten up. So, we are between Massart's and Rauh's models" (`cirad.md` l.556–557) | CIR [secondary] |
| M7 | **Monopodial branches of two orders, lateral flowering.** Branches continue from their own apex; first-order branches bear second-order branches or lateral cones, and flowering never ends an axis. | "Each branch segment P is associated with a lateral flower of inflorescence K or with apex C that will create a second-order branch. The second-order branches consist of plagiotropic segments P with lateral flowers K." with `B → P[K]B`, `B → P[C]B` (`petri.md` l.994–998); "The vegetal carries male and female cones. It is a monoic species" (`cirad.md` l.559) | PR00 [read]; CIR [secondary] |
| M8 | **About four axis categories, with strong acrotony.** Each shoot develops its new growth units and shoots mainly at its tip. | "The architectural unit is composed of a certain number of categories of exes, about 4." and "each bearing entity with a very marked acrotony will develop a new growth unit and new shoots" (`cirad.md` l.559–562) | CIR [secondary] |
| M9 | **A medial category on the growth unit.** Some axes sit in the middle of the growth unit rather than in the whorl. The passage does not say whether this means the trunk's unit or a branch's. | "you see a category found in the medial part of the growth unit or carried by the branches" (`cirad.md` l.566) | CIR [secondary]; the axis it applies to is ambiguous |
| M10 | **Draperies: sequential partial reiteration.** On adult branches, dormant buds at branchlet bases grow into new branchlets as the old ones die. This repeats in waves (up to 5 to 7), so the whole branch stays green. | "as of a certain time … buds which were sleeping will grow at the bases of the branchlets … As the branchlets get old and die and go though natural pruning this phenomenon will increase … they are replaced by partial reiteration of the branchlets" and "the draperies of the spruce with several successive waves of partial reiteration" and "up to 5, 7 times, depending on the variety" (`cirad.md` l.569–577, 593) | CIR [secondary] |
| M11 | **Natural pruning of branchlets.** Without M10, only the distal part of a branch would stay green. | "if there was not this process of reiteration on several waves there would only be the distal part of the branch that would be green and the rest would have shed the branchlets" (`cirad.md` l.578–582) | CIR [secondary] |
| M12 | **Pendant branchlets of the comb habit.** Second-order branchlets hang as curtains below the main branch. | **No passage in the folder states this for the spruce.** It is seen in the reference photographs (S1, S3, S5; the README's "comb type"). CIRAD's hanging branchlets (`cirad.md` l.591, "branchlets which become hanging") are *Araucaria hunsteinii*, not the spruce. That the curtains are M10's draperies is my reading, not a source's. | **unsourced** (reference-visible only) |
| M13 | **Branch angle 40° to 70°, steeper in the upper crown.** | "The branching angle distributions were left-skewed and the angles varied between 40° and 70° … Branch angles are steeper in upper …" (`s_b3.txt` l.23, a search snippet of the BioResources paper on young Norway spruce; the angle's reference is not stated) | [snippet], unverified in full text |
| M14 | **Number of branches per whorl.** | **No passage in the folder gives a count.** PR00 writes `[B]n` with n unstated. S4 shows three to four lateral buds clustered around the terminal bud, on the visible side. | **unsourced** |

Not built from this spec: the polycyclism of M4's fourth year (one growth unit a year is the engine's grain), and cones as organs (M7). Cones change no wood; they belong to the organ stages, not the growth rule.

## Proposed settings and values

The starting point is R1a round 3's spruce (`R1A-ROUND3.md` l.59), which was borderline: leader and cone, upper tiers, lower crown a haze, w/h 0.97 against 0.62 authored. Changes from it are marked **Δ** with the rule behind them. These are proposals for task 9, not measured values. Engine behaviour is from reading the code [code], not from running it.

| Setting | Value | Δ from R1a r3 | Rule | Note |
|---|---|---|---|---|
| `persist0` | 1.0 | | M1 | The leader is never lost. |
| `persist1`, `persistShape` | 0.7, 10 | | M7 | Branches stay monopodial until phi is old. |
| `reiteration` | 0.0 | | M1 | There is no terminal abortion to relay. |
| `eta` | 0.05 | | M2 | The trunk's pull to its tropism. The trunk's `out` is zero, so its tropism is straight up [code, `organ.rs:184–191`]. |
| `ground`, `straighten` | 0, 0 | | M2 | No secondary straightening is needed on an orthotropic trunk. |
| `rhythm` | 0.85 | **Δ** from 1.0 | M3, M9 | 0.85 sets 85% of a unit's laterals in the distal whorl and spreads the rest along the unit (M9's medial category). The share is unsourced. |
| `acrotony` | 1.0 | **Δ** from 0 | M8 | Shapes the non-whorl share toward the unit's top. "very marked acrotony" is qualitative. |
| `n0`, `n1` | 5, 1.5 | | M3 | Metamers per yearly unit. The engine's whorl is set at the unit's distal node [code, `organ.rs:203`]. |
| `branching` | 1.0 | **Δ** from 1.6 | M14 | At phi 0 the distal whorl gets about `branching × n0 × rhythm` laterals [code, `organ.rs:192,203,205`], so 4.25 here against 8 in r3. The count is unsourced; S4 suggests about 4. |
| `fate` | 3 | | M8, M7 | High-phi shoots barely branch. |
| `phiStep`, `zone` | 0.42, 0.6 | | M8 | First-order branches are born near phi 0.34 and second-order near phi 0.6, about four categories in all. |
| `angle0`, `angle1` | 65, 75 | **Δ** from 80, 70 | M13 | Degrees off the parent shoot. The values follow the snippet's 40° to 70° range, and ascending young branches (M13's "steeper in upper"). Unverified. |
| `lean0`, `lean1` | 0.3, 1.5 | | M5 | Branches turn toward horizontal by phi. `lean` is clamped to [0, 1] [code, `organ.rs:190`], so no axis can lean below horizontal (see F2). |
| `distich` | 1.0 | | M5 | Branch laterals in one plane. On a vertical trunk the bearing falls back to the golden angle [code, `organ.rs:261–266`], so the trunk stays spiral (M2) at any `distich`. |
| `life0`, `life1`, `lifeShape` | 1000, 25, 2 | **Δ** from neutral | M11 | Branchlets die after about 25 years; main branches persist (an open-grown crown to the ground, S1, S2). Unsourced count; F3 is what keeps the branch green. |
| `drift` | 0.015 | | M1 | See F1: the trunk drifts too. |
| `cycles`, `unit`, `short` | 36, 0.007, 0.9 | | | Kept from r3; they are tuning, not model rules. |

## Engine features the rules need

All are proposals; whether each is a new setting, an extension of an existing one, or out of scope is the host's call.

- **F1. The trunk ages like a branch** [code, `organ.rs:173`]. The terminal bud keeps `b.phi`, but the unit's phi drifts with `years`. At `drift` 0.015 the trunk's phi is 1 − 0.985^36 ≈ 0.42 by year 36, so the whorl count falls as (1 − phi)^`fate` ≈ 0.2 of its start. M1 and M3 need the leader to keep making full whorls. Two candidate fixes: drift scaled by the axis's phi at birth, or the spruce's `drift` near 0. The second may starve the branches of their own ageing. Not run; R1a round 3's "tiers in the upper half, lower crown a haze" may or may not come from it.
- **F2. Tropism past horizontal** (M12; S1, S3, S5). `lean` stops at horizontal, so no branchlet can hang. The candidate fix extends `lean`'s range toward straight down by phi. That is a degree on an existing setting; the host decides whether it is the same setting or a new one.
- **F3. Sag of older wood, with the tip still rising** (M6 and the drooping lower branches in S1). Two things work against it:
  - **Straightening turns the wrong part of the branch.** `straighten` turns wood up by the years since it grew [code, `organ.rs:278–283`], so a branch's base turns up more than its tip. The spruce's lower branches droop at the base with an upturned tip. The candidate fix is a downward bend of wood growing with its age or carried load, while the tip keeps its own tropism. A signed `straighten` would also bend the trunk wherever it is off vertical, so it is not a drop-in.
  - **Lean flattens the tip instead of raising it.** On one axis, lean grows with phi, and phi grows with the axis's years. The tip is therefore the most horizontal part, the opposite of M6.
- **F4. Delayed release of dormant buds** (M10, draperies). Every lateral is born in the year its unit grows [code, `organ.rs:223–226`]. There are no sleeping buds that wake years later at branchlet bases, and `reiteration` acts only when a terminal aborts. Without F4 the lower branches either keep every branchlet (`life1` high) or go bare at the base (M11).
- **F5. Rhythm that establishes with age** (M4). `rhythm` is one constant, so the 5-year still shows tiers from year 1. The source says the seedling grows continuously at first. Low priority: this is visible only in the youngest still.
- **F6. A skirt resting on the ground** (S2: "a full skirt to the ground"). Wood below y = 0 is a driver error (R1A-ROUND2 l.29), and with F2 or F3 the lowest branches will reach the ground. Whether wood may rest at y = 0, or the `ground` pull must hold it up, is a design question for the host, not a value.

## Traits the owner judges

Seen in `.flow/references/norway-spruce/`; I viewed all five photographs. The spruce is evergreen, so wood structure and tiers are judged through the needles.

1. **One straight leader to the tip, never forked** (S1, S2). The stem shows through the crown in S1.
2. **A narrow cone, widest at the base, with the live crown to the ground in open growth** (S1, S2). The authored w/h is 0.62 (R1A-ROUND3 l.18). There is no clear bole in either photograph.
3. **Readable whorled tiers along the stem**: main branches in stories with gaps between them (S1, S3; less readable in the dense S2).
4. **Main branches near-horizontal or drooping, with upturned tips**: drooping lower in the crown, ascending near the top (S1, S3, S5; M6).
5. **Comb habit: second-order branchlets hang as curtains below each main branch** (S1, S3, S5).

S2 shows the other habit, the **brush habit**: short, stiff, near-horizontal branches without curtains. The owner picks which habit the preset targets. Trait 5 applies to the comb only, and needs F2.
