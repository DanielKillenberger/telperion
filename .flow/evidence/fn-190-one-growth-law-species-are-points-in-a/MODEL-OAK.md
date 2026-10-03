# fn-190 R1: the oak's model specification (Rauh), 2026-10-03

Stage A for the oak (catalogue `oregon-white-oak`, *Quercus garryana*; *Q. robur* photographs stand in where *Q. garryana* ones are lacking). Reading only: no code changed. Every value below is a **proposal for the host** (task .5 owns the vocabulary and the point's values). Design questions are listed at the end and are not decided here.

**Tags.** Each rule cites the passage it rests on, quoted from the source text in the research worktree's `.firecrawl/lit/` folder:

- **[BC07]** `bc2007.md`.
- **[T83]** `miami.md` (Tomlinson 1983).
- **[PR00]** `petri.md` (Prusinkiewicz & Remphrey 2000, full text; LITERATURE.md lists only its abstract).
- **[K19]** `biorxiv795286.md` (Kędra 2019).
- **[KEY]** `key.md` (the HOT key after Hallé et al. 1978).
- **[CIR]** `cirad.md`.
- **[HOT]** `halle.md` (Hallé, Oldeman & Tomlinson 1978).
- **[HEIN]** `hein.md` (abstract only).

A value tag says where a number comes from:

- **[source]**: the passage gives it.
- **[photo]**: read from a reference photograph.
- **[R1a r3]**: carried over from the oak point of `R1A-ROUND3.md`, which failed the visual gate.
- **[unsourced]**: a starting guess.

## Source findings that change LITERATURE.md's reading

- **`hot_ch3.md` holds no *Q. rubra* passage.** It is Springer's landing page for HOT chapter 3, with the abstract only. The *Q. rubra* statement is in [T83] (`miami.md`): "Although the rubber tree Hevea brasiliensis is a good example of the widely distributed Rauh's model in the tropics, a more familiar example such as an oak (Quercus rubra) would have to be used for the temperate zone."
- **`halle.md`'s model pages are images.** Its Rauh's model text was not extracted, so HOT is citable here only through its axis table and through secondary sources.
- **The oak-specific "clustered laterals at the top of each annual shoot" is not in [T83] or [K19].**
  - Acrotony as such is sourced for Rauh's model ([BC07], Rauh 1939, "primary acrotony").
  - The only oak-specific branch-position passage is [CIR]'s, which puts the largest branches on the second growth unit of a polycyclic year.
  - The clustering at the shoot tip is seen in S4, not read in a source (rule 5).
- **"Forks from the terminal bud cluster" has no source as a mechanism.**
  - [CIR] says apex death "can be very frequent in an oak tree".
  - [K19] reports that mature oaks fork and that forks formed early.
  - No text says a fork arises from the terminal bud cluster (rule 6).

## Rules of Rauh's model for the oak

| # | Rule | Quote | Settings (today's `params.rs` names; new ones in *italics*) | Proposed oak value | Engine feature missing |
|---|---|---|---|---|---|
| 1 | **Rhythmic growth in height**: the trunk grows in yearly growth units. | [BC07] Fig. 16: "Rauh's model (C) is represented by numerous woody plants where growth and branching are rhythmic, all axes are monopodial and sexuality is lateral." [KEY] 21: "Trunk with rhythmic growth in height ...... Rauh's model." | `cycles`, `n0`, `n1`, `unit`, `short`; the engine grows one unit per bud per year | cycles 45 [R1a r3; mature age unsourced], n0 6, n1 1.5, unit 0.008, short 0.85 [R1a r3] | None for one unit a year. |
| 1a | **Polycyclism**: an oak may make more than one growth unit a year, and the largest branches sit on the later unit. | [CIR]: "the oak which has several cycles per year"; "the largest branches that will be mono- or polycyclic moreover will be found mainly on the second growth unit which will therefore lead to what you call an acrotony." | *flushes per year* (neutral 1) | *flushes* 1.0 for now; the frequency is [unsourced] | **Polycyclic years.** Several growth units per year, with the branching weighted to the last one. Today one unit a year is fixed. |
| 2 | **Monopodial axes**: the terminal bud continues every axis. | [PR00]: "Rauh and Attim's models. The trunk is monopodial." [K19] Fig. 1: "monopodial trunk and rhythmic, orthotropic branching." | `persist0`, `persist1`, `persistShape` | persist0 1.0 [source: monopodial trunk]; see rule 6 for persist1 | None. Its conflict with rule 6 is design question 1. |
| 3 | **Orthotropic branches equivalent to the trunk**: every axis is erect, radially symmetric and spirally arranged, and a branch repeats the trunk. | [PR00]: "Branches are orthotropic and morphogenetically equivalent to the trunk. … Temperate examples of Rauh's model include ash, oak, pine, maple, and larch." [KEY] 18: "Vegetative axes all orthotropic". [CIR]: "a model of vegetal the axes of which are all orthotropic". [HOT] axis table: strict orthotropy has "spiral or decussate" phyllotaxis and "radial" symmetry. | `distich`, `lean0`, `lean1`, `eta`, `phiStep`, `angle0`, `angle1` | distich 0 [source: spiral, radial]; lean0 0 [source: orthotropic]; lean1 0.5 [unsourced; R1a r3 used 0.8; design question 3]; eta 0.015 [R1a r3]; phiStep 0.05 [R1a r3; small, so laterals stay young like the trunk]; angle0 25, angle1 60 [photo S1, S3; no source gives an oak angle] | None. Oak phyllotaxis (2/5 spiral) is in none of the texts read, so the spiral itself is [unsourced] beyond the HOT table. |
| 4 | **Lateral flowering**: flowering never ends an axis, so it does not shape the architecture. | [BC07] Fig. 16: "sexuality is lateral". [PR00]: "Flowering is always lateral." [K19]: "oak trees exhibit lateral flowering (not affecting branching)". | *flowering position* (terminal share; neutral 0) | 0 [source] | None; nothing to build at 0. |
| 5 | **Acrotony**: laterals develop preferentially at the distal end of each growth unit or annual shoot, and the distal ones are the most vigorous. | [BC07]: "Acrotony … is the prevalent development of lateral axes in the distal part of a parent axis or shoot, and depending on whether branching is monopodial or sympodial, Rauh (1939) … termed it, respectively, 'primary' or 'secondary' acrotony." [BC07]: "there is neither automatic nor direct correlation between the privileged position and the relative vigour of lateral branches." Oak: [CIR] as rule 1a. | `acrotony` (count profile), `rhythm` (share at the distal node), `zone` (vigour by place: distal laterals born younger) | acrotony 2.0 [unsourced; R1a r3 used 1]; rhythm 0.3 [photo S4: a cluster at the shoot tip; value unsourced]; zone 2.5 [R1a r3] | None for the profile. **Naming:** `rhythm` in `params.rs` is the share of laterals at the distal node (tiers), not HOT's rhythmic growth (rule 1); the vocabulary should not carry both meanings under one word. |
| 6 | **Forking in the mature oak**: the apex dies often, and distal laterals take over as near-equal arms. Not a single mature oak keeps the pure model. | [CIR] (on apex abortion and the relay by a lateral bud): "These abscissions here are very frequent … And it can be very frequent in an oak tree." [K19]: "Not a single tree fully conformed to the original Rauh's model"; "Many of the R.D trees exhibited repeated forking of the axes coming from a fork below"; "the forked junctions had been formed since the early times of branch emergence"; "the apical control (a mechanism that promotes a single leader shoot) may be weakened in older trees (Wilson, 2000)". Fork criterion: branching ratio above "the 2/3 threshold". | `persist1`, `persistShape`, `reiteration`, `reiterShape`, `rhythm` (arms at the last node) | persist1 0.8, persistShape 3 [unsourced; R1a r3 used 0.95 and 6, and its leader never yielded]; reiteration 0.9 [unsourced; R1a r3 0.7]; reiterShape 1 [neutral] | **Apical control that weakens with the tree's age** (K19 citing Wilson 2000). Persistence today depends only on the axis's phi, and the trunk's phi stays near 0, so the leader never yields (R1A-ROUND3, decision 2). "From the terminal bud cluster" is [unsourced] as a mechanism. |
| 7 | **Reiteration builds the mature crown**: the model is visible in the sapling, while the mature crown is reiterated complexes. | [T83]: "Most trees conform to their model for a limited period as saplings"; "Crown shape in trees is determined entirely by the twin processes of architecture (a deterministic process) and reiteration (an opportunistic process)." [K19]: these traits "make the original architectural 'blueprint' (Rauh's model) hardly recognisable at the scale of whole mature and older oak trees (Oldeman, 1990)." [BC07] (general, not oak): "sequential reiteration … proved to be a very common and major morphogenetic process underlying crown construction in most forest trees." | `reiteration`, `reiterShape`, `vigourJump` | As rule 6; vigourJump 1 [R1a r3] | **Reiteration not tied to apex abortion**, either sequential (by ontogenetic stage) or opportunistic from reserve buds ([T83]: "the tree's ability to produce reserve buds"). The engine reiterates only when a terminal aborts. |
| 8 | **Main branches at the crown base, and branch death below**: the lowest branches form the horizontal crown, and lower ones die in low light. Oak branches die early and shed slowly. | [K19]: "the lowest branches (of equal insertion height) are the main branches, that contribute to the horizontal crown extent"; "the main branches are of unequal insertion height (e.g. because of branch mortality in low light conditions)". [HEIN] abstract: "early branch dying and slow shedding for oak". | `life0`, `life1`, `lifeShape` (a stand-in until light, stage F) | life0 100, life1 30, lifeShape 3 [R1a r3] | **Shading mortality** (stage F's light and carbon balance; already planned). Slow shedding (dead stubs kept) is unbuilt; it is out of R1 scope, and listed only so it is not lost. |
| 9 | **Proleptic branching**: a lateral grows after a rest, in the following year. | [T83] Fig. 3: "in prolepsis the branch develops after a period of rest as a lateral bud-in the case of temperate trees, in the second year." [HOT] axis table: strict orthotropy originates "mainly by prolepsis". | None: the engine grows a lateral bud in the following year's sweep. | n/a | None (checked in `organ.rs`: laterals are pushed as buds and grow next cycle). |
| 10 | **Forked arms turn upright away from the junction.** | [K19]: "the forked branches reached their upright positions further away from the branch insertion points." | `straighten`, `ground` | straighten 0.01 [unsourced; R1a r3 0]; ground 1 [R1a r3] | None. |

**Settings carried from R1a round 3 that no rule above sets:**

- drift 0.025, est0 0.3, estYears 8;
- branching 1.85, fate 1.5;
- maxNodes at the run's limit.

Each is [R1a r3].

**Unsourced, listed so nothing is built on them silently:**

- the oak's branch angles;
- its phyllotaxis fraction;
- the frequency of polycyclic years;
- the clustering of laterals at the shoot tip (seen in S4, never read in a source);
- the fork-from-the-bud-cluster mechanism;
- lower-limb droop (rule 3's lean, design question 3);
- the mature verdict age.

## Traits the owner judges (reference-visible)

All in `.flow/references/oak-quercus-robur/`. I viewed each photograph named below.

1. **A short bole that divides low.** A single trunk to about 0.15 to 0.35 of the height breaks into several comparable, ascending limbs. There is no dominant leader through the crown; a weak central stem may persist. Read from `S1-robur-field-november.jpg` (two codominant stems and limbs from about a third of the height), `S1-robur-old-open-winter.jpg` (low division into several crooked leaders) and `S2-garryana-wheatfield-leaf.jpg` (a stout bole splitting into three or four heavy limbs).
2. **A broad dome.** The crown is about as wide as tall or wider (R1-BANDS measured w/h 1.07 to 1.22). The lowest limbs spread near horizontal and their tips hang toward the ground. Read from `S1-robur-field-november.jpg`, `S1-robur-old-open-winter.jpg` and `S2-garryana-wheatfield-leaf.jpg`.
3. **Heavy, forking, kinked limbs.** Limbs are thick at the junction and split repeatedly into near-equal arms rather than carrying a spine of side branches. Their course is angular and zigzag. Read from `S3-robur-limbs-from-trunk.jpg` (three heavy limbs off the bole, kinked secondaries) and `S3-garryana-tortuous-limbs.jpg` (tortuous zigzag limbs with short gnarled secondaries).
4. **A full twig mesh to the edge.** Fine twigs fill the crown, densest at the rim but present inside, with no hollow shell. Read from `S1-robur-field-november.jpg` and `S1-robur-old-open-winter.jpg`.
5. **Laterals bunched at the shoot tip.** Short laterals sit at the distal end of each year's shoot, each ending in a bud cluster. This is visible only at close view. Read from `S4-robur-frosted-twig-spray.jpg`.

## Design questions for the host (not decided here)

1. **Monopodial model, forked tree.** Rauh's model is monopodial (rule 2), yet every mature oak in K19 forks (rule 6), and the owner judges the mature tree (trait 1). Two ways to express the division are possible:
   - a terminal-persistence chance at the trunk;
   - lateral vigour catching up with a leader whose apical control weakens with tree age.

   Which one belongs in the vocabulary is the host's call. R1a's oak never divided with persistence by phi alone (R1A-ROUND3, decision 2).
2. **Reiteration.** Should reiteration be its own trigger (rule 7), or stay a consequence of abortion only?
3. **Spreading crown from orthotropic axes.** The model's axes are erect (rule 3), while the photographs' low limbs spread and hang (trait 2). No source says whether that comes from lean by age or from bending under load, which the engine does not model.
4. **Polycyclism.** Is polycyclism (rule 1a) needed for the oak's look at all, or does acrotony alone carry it? No source gives its frequency.
