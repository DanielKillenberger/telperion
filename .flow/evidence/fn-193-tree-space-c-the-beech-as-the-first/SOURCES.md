# fn-193 R1: the beech's values and their sources

The beech is `crates/telperion-space/src/beech.rs`, one growth cycle a year. Source tags follow fn-188's LITERATURE.md; texts in the fn-188 research worktree's `.firecrawl/lit/`.

- **[M98]** Millet, Bouchard & Édelin 1998, *Can. J. Bot.* 76:2100–2118 (*Fagus grandifolia*; Troll's model in the genus), `millet.md`.
- **[LET]** Letort et al., GreenLab beech (arXiv 1010.5145), `letort.md`: Fig. 2 and Table 2. Understorey trees aged 21 and 46, not open-grown.
- **[DTT86]** Dupré, Thiébaut & Teissier du Cros 1986, `dupre.md`.
- **[OSU]**, **[TSO]**, **[PACKHAM]**: catalogue/european-beech sources (height, spread, dbh).

**Estimated** means no read source gives the value for a mature open-grown beech; it was set by the host-screened stills, never by a number alone.

## Reference axis (nine physiological ages)

| PA | Role | Source |
|---|---|---|
| trunk | A1, the young stem: orthotropic, bearing A2 systems | [M98] "orthotropic along almost all its length". A stack of Troll modules since round 5 (next row). |
| trunk modules (round 11) | abortion 0.5 a growth unit, relay 1, `relay_at` 0.15 of the last growth unit, epitony 0.6, insertion 0.3, module tips toward elevation 1.4 rad | [M98] the relay bud stands in the curvature zone, on the upper side; the numbers are **estimated** from the stills |
| fork | the stem's top over 2 cycles: one limb at its top node, boughs or a second limb below it (round 12) | [M98] "a fork is created from subterminal buds"; "the relays do not differentiate themselves from one another" |
| leader, limb, bough, spur | total reiterates, each forking into the next, each shorter-lived | [M98] "Each reiterate forks and reiterates in its turn, producing reiterates that are increasingly smaller and less branched"; "the most peripheral being the shortest". Four levels: estimated. |
| branch | GreenLab PA 2, long ramified shoot | [LET] Fig. 2: Z20 bare, Z24 short shoots, Z23 PA 3, Z22 partial reiteration, base to tip |
| shoot | GreenLab PA 3, long shoot bearing short shoots | [LET] Fig. 2 |
| short | GreenLab PA 4, short shoot | [LET] "usually 3 or 4. The constant value of 3 was uniformly chosen"; [DTT86] short shoots "3 à 5 entre-nœuds courts", buds "qui ne se développent jamais" |

## Values

| Value | Beech | Source or status |
|---|---|---|
| Zone order in every growth unit: bare base, laterals toward the top | all PAs | [LET] acrotony "as described by Rauh for beech" (Nicolini 1998) |
| branch zones (nodes; lateral probability) | Z20 1–2; Z24 2–3, short 0.7 (round 13, the top of [LET]'s A24 range); Z23 1–2, shoot 0.65; Z22 1, branch 0.08 | node counts from [LET] Fig. 2 maxima (3, 3, 2, 1) and Table 2 M2 ranges; probabilities: A24 ∈ [0.5, 0.7] read as a probability, A23 ∈ [0, 0.2] **raised to 0.65 (estimated)**, A22 ∈ [0, 0.2] set at 0.08 so branch self-renewal stays below one per lifetime (estimated) |
| branch growth unit, 5–8 nodes | | [DTT86] long shoots "6 à 10 entre-nœuds" (seedlings) |
| shoot zones | Z30 1; Z34 2–3, short 0.6 | [LET] Table 2: M30 ≈ 1, A34 ∈ [0.55, 0.6] |
| short shoot, 3 nodes, never branches | | [LET], [DTT86] |
| branch systems rolled about their parent by up to 0.7 rad, reiterates and branch systems by up to 1.2 rad (`form.roll`, rounds 13, 14 and 17) | | **estimated**, from the stills: breaks the one plane the systems of a steep limb stacked in |
| phyllotaxis of branch, shoot, short | distichous (π) | [M98] "plagiotropic monopodia with distichous phyllotaxy" |
| branch systems in one plane | trunk `plane` π/2, others 0 | [M98] "arranged in a single horizontal plane" |
| trunk divergence 2.4 rad | | **estimated**: stands in for [M98] "the orientation of the branching plane may change from one module to the other" |
| branch and shoot plagiotropic (elevation 0.25 and 0.15 rad) | | [M98] plagiotropic; the slight rise is **estimated** (lower values put low branches into the ground) |
| reiterates oblique (round 12): the leader toward 1.25 rad; a limb toward 0.55 rad for 15 cycles, then 1.15; a bough toward 0.5 for 8, then 1.0; a spur toward 0.75. The leader and limbs bear boughs, boughs bear spurs | | [M98] "oblique, have a large diameter"; the crown is a succession of reiterates "increasingly smaller and less branched", the most peripheral the shortest. Angles, phase lengths and rates **estimated** |
| limbs and boughs as Troll module stacks (round 16): abortion 0.3 (limb) and 0.35 (bough) a growth unit, relay 1, `relay_at` 0.15, epitony 0.6, insertion 0.7, straightening 0.6, tips toward 0.3 and 0.2 rad at tropism 0.6 (round 17; the leader a module stack too, tips toward 0.8); limb 70 units bearing boughs 0.16, bough 30 bearing spurs 0.14; reiterates bear short shoots 0.3 and branch systems 0.35 | | [M98] trunk and limbs are stacks of plagiotropic modules straightening at the base, the relay in the curvature zone; numbers **estimated** from the stills |
| insertion 0.7 rad (reiterates), 1.0 (branch, shoot), 0.9 (short) | | **estimated**; [RAIS21] 30.7° branch angle (abstract, reference axis unknown) not used |
| fork after 12 cycles at about 5 m (about 0.25 H) | | **estimated** to the photographs S1 (about 0.2 H to 0.33 H); [M98] forks at about 20 m in forest |
| internodes: trunk 7 cm, reiterates 4 cm, branch 3 cm, shoot 2.5 cm, short 6 mm (round 12) | | **estimated**: LITERATURE.md "internode lengths were not found in readable sources" |
| lifespans and viabilities (round 16: leader 45 at viability 1; limb 70 and bough 30 as module stacks, spur 25 at 0.995; branch 10, shoot 5; short 3 at 0.9; shedding after 1 idle cycle) | | **estimated**; [KINT10] beech branches "die late but shed fast" (abstract) |
| sinuosity (`wander`) and bending (`tropism`) rates (round 17: branch 0.5, shoot 1.0) | | **estimated** |
| pipe model | each phytomer's section added below it | the pipe model as [PAL09] uses it (LITERATURE.md); pipe radii **estimated** |
| height about 20 m and width 16 to 22 m at 80 cycles | | result, against [OSU] 15–23 m tall, 12–18 m spread (landscape) |
