# Date palm (Corner): model specification and today's palm bar

fn-190 task 4, stage A (R1). Reading only, plus one render of today's palm. A dispatched worker wrote this file: the settings and engine features below are proposals for the host's vocabulary (task 5), and the four questions at the end go to the host. Nothing here decides a design.

Sources are the research worktree's `.firecrawl/lit/` texts, cited by file and line, and LITERATURE.md (`.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/LITERATURE.md`).

## 1. Which model, and a source conflict

LITERATURE.md §1.2 assigns the date palm to **Corner** on [CIR] (a secondary lecture transcript) and a [TOM79] search snippet that is about palms in general, not *Phoenix*.

- [CIR] `cirad.md:456`: "the date palm, which is most often monocole (monoecious), Phœnix Dactylifera, a monocole tree. With respect to the previous question, it has lateral flowers. This would be the Corner architectural model."
- [TOM79] `s_a3.txt:11`: "Corner's model is common and represents single-stemmed palms with lateral inflorescences."

**Conflict.** A key built on Hallé, Oldeman and Tomlinson 1978, pp. 84–97, names *Phoenix dactylifera* as its example of **Tomlinson's** model, the pleonanthic variant, not Corner's:

- `key.md:5`: "The following key is based on Halle, Oldeman and Thomlinson (1978) “Tropical Trees and Forests” (pp.84–97)."
- `key.md:35`: "4\. Basitony, i.e., branches at the base of the module, commonly subterranean, growth usually continuous, axes either hapaxanthic or pleonanthic ...... **Tomlinson’s model**."
- `key.md:45–49`: "(b) Pleonanthy, i.e., each module not determinate, with lateral inflorescences … Monocotyledon: _Phoenix dactylifera_ (date palm—Palmae)."
- Tomlinson 1983 [T83], `miami.md:84`: "Branching of major axes is then either restricted to the base of the trunk, as in McClure's (bamboos) and Tomlinson's (for example, many palms) models".

Under Tomlinson's reading, each module is a Corner axis (unbranched, lateral inflorescences, indeterminate), and the plant adds new modules from the base. [CIR] gives the other reading: a multi-trunked date palm is reiteration (`cirad.md:456`: "You can see that it's not natural for it to have several trunks. … So an individual can replicate its architecture."). Both readings give the same single stem above the base, and all three reference photographs show one trunk (§4). The difference is limited to basal shoots. **Question Q1 for the host.**

## 2. Corner's rules, quoted

| # | Rule | Quote | Source | Standing |
|---|---|---|---|---|
| C1 | One axis, never branched | "Corner's model (A) concerns unbranched plants with lateral inflorescences." | [BC07] `bc2007.md:430` | sourced (model) |
| C1 | | "1\. Stem strictly unbranched (Monoaxial trees) … — Inflorescences lateral ...... **Corner’s model**." | `key.md:7,17` | sourced (model) |
| C2 | Monopodial, from one vegetative meristem, indeterminate | "A tree conforming to this model has a single, monopodial, orthotropic and non-branching trunk constructed by one vegetative meristem. Inflorescences are axillary and growth is therefore indeterminate." | [PR00] `petri.md:912–916` | sourced (model) |
| C3 | Orthotropic: vertical, radially symmetric | "the orthotropic axis such as the trunk of a coconut tree and the orthotropy is characterised by a growth direction which is essentially vertical and symmetry which is radial" and "Orthotropy is characteristic of many trees or vegetals such as palm trees" | [CIR] `cirad.md:187–189` | sourced (palms in general) |
| C4 | Flowering lateral (axillary), so the apex never ends | the C2 quote, and [CIR] `cirad.md:456` "it has lateral flowers" (*Phoenix*) | [PR00]; [CIR] | sourced (*Phoenix*, secondary) |
| C5 | The axis is modules in a straight line | "Whereas the axis in Corner’s model is formed as a sequence of modules in straight position with respect to each other, the axis in the Chamberlain’s model is formed as a sequence of modules in a lateral position." | [PR00] `petri.md:634–638` | sourced (model) |
| C6 | Growth continuous | "(a) Growth continuous: … Cocos nucifera (coconut palm—Palmae), Elaeis guineensis (African oil palm—Palmae)"; "continuous growth … most of the palm trees … there is also a continuous organogenesis" | `key.md:19–23`; [CIR] `cirad.md:119` | sourced for palms; **unchecked for *Phoenix*** |
| C7 | Several stems are reiteration, not the model | [CIR] `cirad.md:456` (quoted in §1); "a non-branched Corner model and if there is a trauma the plant will reproduce the structure … total traumatic reiteration" (said of *Fouquieria*) | [CIR] `cirad.md:456,498` | sourced (secondary); conflicts with §1 |
| C8 | No secondary thickening restricts branching | "Plants without secondary thickening, a process in which the diameter of the trunk increases as the tree grows taller and the crown becomes larger, have an obvious restriction on their ability to branch. Most woody monocotyledons conform to this pattern." | [T83] `miami.md:84` | sourced (monocots) |
| C9 | A palm conforms to its model all its life | "only trees of either very precise or very simple organization—for example, palms and many conifers—grow in such a way that they always conform to a described tree model." | [T83] `miami.md:33` | sourced (palms) |
| C10 | The stem's girth is set at establishment and does not grow with the crown | none in the source set; it is what C8 implies for a monocot stem | — | **unsourced**: not to be built on C8 alone |
| C11 | The stem's lean and curvature | none in the source set; only P-WHOLE's observation (§4) | — | **unsourced** as a rule; observed in one photograph |

C9 means the mature palm should show the model, so stage D can judge Corner directly at the mature age.

## 3. Mapping to settings (proposal for task 5)

The names are placeholders until the host's vocabulary exists. Each value is the setting's value at the palm's point.

| Setting (provisional) | Palm value | Rule | Note |
|---|---|---|---|
| Branching readiness (chance that a lateral vegetative bud develops) | **0 at every physiological age** | C1, C8 | LITERATURE §4.2 has "Corner: Branching probability 0 at all φ", but it marks that table as the author's reading, not a source's. At 0, the rules about where laterals go along a shoot (acrotony to basitony), branch angle, two-ranked against spiral laterals, and relay all have nothing to act on. |
| Apical fate (chance the apex continues, against aborting or flowering at the tip) | **continues: 1** (determinacy 0) | C2, C4 | Monopodial and indeterminate |
| Orthotropy along the axis (base to tip) | **1 over the whole axis**, no secondary reorientation | C3, C5 | Mixed or plagiotropic axes are not used |
| Flowering position | **lateral (axillary): 1**; terminal: 0 | C4 | Today draws no inflorescence: capability `infructescence` is Absent (`crates/telperion-core/tests/capability.rs:134`), owned by fn-111 |
| Growth rhythm | **continuous** (rhythm strength 0) | C6 | Unchecked for *Phoenix*. No tier is visible, because nothing branches. |
| Phyllotaxis | spiral, **137.508°** (`ranges::default_rosette_divergence`; date-palm.values does not override it) | — | No source in the set gives a frond divergence for *Phoenix*: **unsourced**. The fronds and the leaf-base lattice share this one spiral (`leaf_bases.rs` header). |
| Reiteration readiness | **0** under Corner; basal-only under Tomlinson | C7 / §1 | Waits on Q1 |
| Stem girth against carried load | **no coupling** (girth independent of the crown) | C8, C10 | C10 is unsourced. Today the girth comes from the radius rows below, and the leaf bases are hung after the radius solve so they never thicken the trunk (`leaf_bases.rs:11–14`). |

Today's preset rows that set the stem (`crates/telperion-core/presets/date-palm.values`). These are tuning values the owner accepted on 2026-09-24 (fn-82), not literature: `habit/lateralOrders = 0`, `apicalDominance = 0.85`, `crookedness = 1.5`, `bias/gravitropism = 0.25`, `bias/lean = 0.02`, `supernatural/writheWavelength = 0.35`, `envelope/height = 22.86`, `crownBase = 0.45`, `spread = 0.1`, `radii/trunkRadius = 0.013` (share of height), `lengthTaper = 0.25`, `surface/flareRadius = 2.0`. `lateralOrders = 0` is an integer that switches laterals off. The vocabulary's branching readiness would reach that state by degree instead (spec, "Edge Cases": "Corner's unbranched stem is the point where branching readiness reaches zero, reached by degree").

## 4. Reference-visible traits (what the owner judges)

From `catalogue/date-palm/packet/references.json`:

1. **One trunk, no laterals anywhere.** P-WHOLE: "A single leaning trunk … no laterals."
2. **The trunk is clothed in retained leaf bases, from the crown base down, in a tight spiral diamond lattice.** P-TRUNK: "Retained petiole bases in a tight spiral clothe the trunk from the crown base down". P-BASE: "the cut, weathered petiole bases in a phyllotactic spiral, each a wedge a hand wide".
3. **A compact apical rosette of about thirty arching pinnate fronds, the lower ones drooping well below the crown base.** P-WHOLE observation. The crown base is about half the height (P-WHOLE shot `crownBase 0.5`, P-TRUNK `0.55`).
4. **The whole stem leans, about fifteen degrees in P-WHOLE.** "leaning about fifteen degrees … the lean is the tree's, not the camera's". P-TRUNK adds that the trunk "narrows slightly at the bottom of the frame where older bases have weathered off".
5. **Hanging date clusters on orange stalks among the fronds.** P-WHOLE: "two hanging date clusters on orange stalks". This is Corner's lateral inflorescence (C4). Nothing draws it today (fn-111).

## 5. The stem features the palm's organs rely on

These were checked in the code at base `3984bce2`. A new stem must keep each of them, or the organs stop drawing as they do today.

| # | Feature | Who reads it | Where |
|---|---|---|---|
| S1 | Every stem node carries the `stem` flag, and each stem has exactly one childless apex node. One rosette stands at every stem apex. | rosette, fronds, leaf bases | `tree.rs:190` (`stem_apices`); `foliage/rosette.rs:108` |
| S2 | The direction of the apex's last segment sets the crown's axis. A wobble in the final step tilts the whole crown and its spiral frame. | rosette, skirt, leaf-base frame | `foliage/rosette.rs:113–121`; `leaf_bases.rs:111` |
| S3 | Fronds and the skirt hang along the straight apex axis, not along the stem polyline: `at: rosette.at - rosette.axis * depth`. A stem that curves within `rosetteDepth` (0.35 m) plus the skirt's depth would part the fronds from the bark. This consequence is inferred, not run. | fronds, skirt | `foliage/rosette.rs:193` |
| S4 | A polyline from the apex down to the last stem node above the root, with the root excluded. Its length must exceed `rosetteDepth`, or the stem gets no bases. | leaf bases | `leaf_bases.rs:105`, `:297` |
| S5 | A radius on every stem node after the radius solve. Each base reads the girth at its depth, the lattice is laid on the stem's mean girth, and the bases are appended after the solve. | leaf bases, lattice | `leaf_bases.rs:151`, `:172–175`, `:350` |
| S6 | Stem wander that shows on scales longer than the trunk's width. Turns shorter than the width are smoothed out of the centreline that the lattice wraps. | lattice | `leaf_bases.rs:316` |
| S7 | Draws are keyed by the apex node's identity (birth order) and the seed, so a stem whose apex identity changes redraws its crown. | fronds | `foliage/plan/fronds.rs:51`; rosette stream |
| S8 | A stem that bears a frond crown bears nothing along its length: the rosette is the tree's only placement source. | foliage | `foliage/rosette.rs:9–11` |

## 6. Engine features the palm needs (proposal)

- **E1.** Branching readiness that reaches zero by degree and grows one order-zero stem flagged `stem`, with no lateral nodes (C1; S1, S8).
- **E2.** A monopodial apex that keeps growing to the site's height, at a step fine enough for the lattice. Today's stem is about 40 nodes, inferred from 552 nodes minus 256 bases of two nodes each (C2, C6; S4).
- **E3.** Stem girth that does not follow carried load: a girth set at establishment, a flared foot, and a gentle taper (C8, C10 unsourced; S5). Today this comes from `trunkRadius`, `lengthTaper` and `flareRadius`. A pipe-model radius on a single axis would need checking against this. **Q2.**
- **E4.** Lean and smooth curvature of the whole stem at scales above the trunk's width, with a straight apex segment (trait 4; S2, S3, S6).
- **E5 (conditional on Q1).** Basal reiteration, meaning new stems from the base.
- **E6 (conditional on Q3).** Lateral (axillary) inflorescence positions below the living crown (C4, trait 5). This is fn-111's organ, not stem growth.

## 7. Today's palm: the bar

Rendered with `target/ci/examples/growth_law today date-palm <seed> <out> <prefix>`. The probe was built in this worktree with `cargo build --profile ci -p telperion-render --example growth_law` (exit 0). Stills and JSON are in `raw/palm-today/` (gitignored):

| Seed | Stills | Height | Skeleton nodes | Skeleton time, warm median (cold) |
|---|---|---|---|---|
| 1 | `date-palm-1-whole.png`, `date-palm-1-bare.png` | 19.451 m | 552 | 1.597 ms (1.694) |
| 7 | `date-palm-7-whole.png`, `date-palm-7-bare.png` | 19.970 m | 553 | 1.505 ms (1.657) |

What the four stills show. I viewed each one. This is a description, not a verdict.

- **Whole, both seeds:** one trunk, the diamond lattice of bases from just above the flared foot up to the crown, a dense rosette of arching green fronds, and a pale dead skirt hanging against the trunk below the living crown. No inflorescence.
- **Bare, both seeds:** one unbranched stem, latticed for its whole length, flared at the foot, near-vertical, with a visible kink about 85% of the way up the stem.
- **Against the traits:** traits 1 to 3 are present. Trait 4 is not: the stem is near-vertical (`lean = 0.02`), where P-WHOLE leans about 15°. Trait 5 is absent.
- **Framing defect in the bar stills.** The `today` command frames the camera on the skeleton's node bounds (`camera()` in `growth_law/main.rs`). For the palm, those bounds are the stem and its bases, so the top half of the frond crown is cut off at the frame's upper edge at both seeds. The bar shows the trunk and the lower crown, not the crown's outline. This is a tooling matter for task 19 ("the palm's organs"). This task's Touches do not allow editing the probe. **Q4.**
- **The probe's metrics** read the palm as a bare stem: `fine_m 0`, one path, `lowest_lateral_axis 1.0`. `wood_m` (207.6 m at seed 1) also counts the 256 retained bases' runs (inferred). It is not stem length.

## 8. Questions for the host

- **Q1. Corner or Tomlinson for *Phoenix*?** The source set disagrees (§1). The answer decides whether the palm's point has basal reiteration readiness above zero. The reference photographs show one trunk either way.
- **Q2. Stem girth.** Does the vocabulary carry a "girth follows load" setting that is 0 for a monocot stem (C8), or does the palm keep today's radius rows outside the growth rule? C10 is unsourced.
- **Q3. Inflorescence.** Corner's defining feature (C4, trait 5) is not drawn today. Does the palm point need it in this spec, or does it stay with fn-111, with the bar judged without it?
- **Q4. Framing of the bar.** Should task 19's tooling frame the palm on the mesh bounds, or on the reference shot (P-WHOLE `fill 0.72`), so the whole crown is in the bar?
