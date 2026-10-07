# fn-213 sheet 4: round 4 beside the references (worker, 2026-10-07)

Round 4 aims at trait (j), as Astra and the host read sheet 3 (SHEET-3.md): long, straight, nearly parallel limbs with drooping tufts at regular spacing, and seed 7's lowest limb reaching the ground at 80. The values are in MODEL-BIRCH.md, "Round 4". The worker viewed every still named here.

## The stills

Same layout as sheets 1 to 3, in the ignored `raw/sheet4/`:

| File | What |
|---|---|
| `raw/sheet4/sheet-40.png` | Row 1: references S1, S3-crown, S2-Oslo, S2-Uusimaa (SHEET-1.md lists sources and licences). Rows 2 to 4: age 40, seeds 1, 7, 42: hero bare, hero in leaf, crown (in leaf), limb close-up (bare) |
| `raw/sheet4/sheet-80.png` | The same at age 80 |
| `raw/sheet4/bases.png` | Trunk bases: 40 seed 1; 80 seeds 1, 7, 42 |
| `raw/sheet4/birch-<age>-<seed>-<shot>.png`, `render.log` | Every still; sizes and times |

| Age | Seed | Nodes | Leaves | Height | Crown width |
|---|---|--:|--:|--:|---|
| 40 | 1 | 384,791 | 257,229 | 16.6 m | 8.0 × 9.0 m |
| 40 | 7 | 540,519 | 356,040 | 16.8 m | 9.3 × 7.2 m |
| 40 | 42 | 604,397 | 402,867 | 16.6 m | 9.8 × 9.5 m |
| 80 | 1 | 922,942 | 599,246 | 23.4 m | 17.1 × 21.1 m |
| 80 | 7 | 1,098,413 | 720,326 | 25.1 m | 18.0 × 18.6 m |
| 80 | 42 | 1,134,719 | 782,824 | 23.7 m | 19.8 × 19.9 m |

## The worker's reading

- **(j) Regularity.** The limbs bend and kink and subdivide unevenly; the limb close-ups no longer show parallel runs with evenly spaced tufts. Bare at 80, the crowns read close to S1: a crooked white stem, irregular ascending limbs and a dark fine haze.
- **Low limb.** Seed 7's lowest limb no longer reaches the ground at 80. Seed 1 at 80 carries one long low limb reaching far to one side.
- **Kept.** Crown shape (an egg at 40, a broad dome at 80), openness, leaf mass and colour hold from sheet 3; the leaf count fell by about a fifth.

## Astra's verdict (2026-10-07)

Submitted with sheet 3's prompt, the paths changed to `raw/sheet4/` and SHEET-4.md, at commit `bf4d5971` (`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-sheet4.md`). Its answer, verbatim:

> The trees are recognisable as silver birches, but the mature open-grown likeness is uneven.
>
> | Age | Seed | Judgment | Visible reason |
> |---|---|---|---|
> | 40 | 1 | nearly | White curved stem and hanging foliage convince; the crown separates into a narrow upper tuft and a side cluster. |
> | 40 | 7 | nearly | Birch character is present, but the crown looks unusually compressed and drawn upward. |
> | 40 | 42 | yes | The fuller, irregular egg-shaped crown, slender stem and drooping foliage make the strongest likeness. |
> | 80 | 1 | nearly | The crown broadens, but its tall central mass and isolated long lower limb weaken the mature dome. |
> | 80 | 7 | nearly | The crown remains pointed and vertically concentrated, with sparse projecting lower branches. |
> | 80 | 42 | nearly | The rounded upper crown approaches the target, but long exposed lateral extensions interrupt its outline. |
>
> Trait verdicts cover the set; a fail identifies a visible shortfall rather than absence from every specimen.
>
> | Trait | Verdict | Evidence |
> |---|---|---|
> | (a) White, slender, curving or leaning stem | **pass** | `sheet-40.png` and `sheet-80.png` show slender white trunks with visible curves and differing lean. |
> | (b) Dark base on the old tree | **fail** | `bases.png` shows increased black speckling at 80, but the bases remain predominantly white, unlike the substantial dark basal area in S1. |
> | (c) Narrow egg at 40, broader dome at 80 | **fail** | `sheet-40.png` broadly supplies the narrow egg, but `sheet-80.png` retains tall, irregular crowns, especially seed 7, rather than convincing broad domes. |
> | (d) Ascending main limbs | **pass** | The bare views in both sheets show main limbs rising from the stem before spreading outward. |
> | (e) Hanging fine outer shoots | **pass** | `birch-80-42-limb.png` clearly shows fine shoots arching downward from the outer branches. |
> | (f) Dark fine wood in the bare crown | **pass** | Both sheets show a dark fine-branch haze distinct from the white trunk and thicker limbs. |
> | (g) Openness, sky between limbs | **pass** | The bare views in `sheet-80.png` preserve clear gaps between major limbs, and its leafy crown views retain openings. |
> | (h) Leaf mass in leaf | **pass** | `sheet-40.png` and `sheet-80.png` carry substantial leafy crowns within the density range suggested by the dense Oslo and more open Uusimaa references. |
> | (i) Leaf colour | **pass** | The crown views in both sheets show a plausible fresh yellow-green, though brighter and more uniform than the reference foliage. |
> | (j) Anything that reads as wrong for birch | **fail** | `birch-80-42-limb.png` still shows repeated comb-like arrangements of drooping shoots along long, sparsely subdivided branches, giving the branching a constructed appearance beside S1 and S3. |
>
> **Overall verdict: FAIL.** The single most useful change would be to break up the repeated outer-branch comb pattern with more irregular subdivision and shoot spacing, so the crown reads as interwoven birch branching.

**In short:** 40 seed 42 reads *yes*, the other five *nearly*. Traits (a), (d) to (i) pass; (b) fails as before (waits on "Bark that matures with girth"); (c) now fails at 80 (tall, vertically concentrated crowns, seed 7 pointed; it passed on sheet 3); (j) fails again: "repeated comb-like arrangements of drooping shoots along long, sparsely subdivided branches". Overall **FAIL**.

## For the host: (j) is the spec's wrong-path stop

(j) has now been named by Astra on sheets 3 and 4, and by the host on sheet 3, after round 4's change aimed at it: the spec's wrong-path stop (two rejections naming the same trait after a change aimed at it). The worker stops tuning here. What round 4 moved and what it did not:

- **Moved:** the limbs themselves are crooked and subdivide unevenly (kinks by relays, persistent bends, unequal siblings); Astra no longer names the limbs as straight or parallel.
- **Not moved:** the comb, now located by Astra on the outer branches: hanging shoot systems leave each bough at evenly spaced nodes, one bud a node on the 2/5 spiral, every internode of an age the same length, so along a bough the strands stand at near-equal spacing and near-equal length.

Levers values still hold, untried: `BRANCH` apex death with relays (kinked strands), `BRANCH` `dominance` (unequal strands), Poisson node counts in the bough's zones (an uneven count per unit), a lower `rhythm` (laterals spread over the unit). None changes the spacing within a unit.

What the engine has no setting for (unknown whether it is needed; the host's judgement): **internode length that varies from node to node** within an age. Every internode of a PA is `internode` metres (`PaState::internode`, one scalar), times the phytomer's `scale` (`structure.rs`: its presences multiplied, which is a draw growing in, and with light the unit's `shade_size` vigour); nothing varies it node to node within a fully grown unit (`geometry/lay.rs:148`). Options:
1. Try the untried levers above as round 5, accepting the stop's risk.
2. A capability spec: a per-node internode variation (a keyed draw, dormant at zero), so lateral spacing along an axis is irregular.
3. Turn on light (fn-197's `shade_size`, `apical_control`) for the birch, so units and laterals vary with the light they get, as KU03 reports for birch crowns; a design question already in MODEL-BIRCH.md (3).
