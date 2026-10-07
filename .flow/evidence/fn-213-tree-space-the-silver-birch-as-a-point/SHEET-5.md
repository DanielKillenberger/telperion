# fn-213 sheet 5: round 5 beside the references (worker, 2026-10-07)

Round 5, after merging F4 at `2c416efd` (fn-222's internode spread, fn-221's old bark): internode spread against (j), old bark against (b), and the broad crown at 80 restored against (c), as the host decided. The values are in MODEL-BIRCH.md, "Round 5". The worker viewed every still named here.

## The stills

Same layout as sheets 1 to 4, in the ignored `raw/sheet5/`:

| File | What |
|---|---|
| `raw/sheet5/sheet-40.png` | Row 1: references S1, S3-crown, S2-Oslo, S2-Uusimaa (sources and licences in SHEET-1.md). Rows 2 to 4: age 40, seeds 1, 7, 42: hero bare, hero in leaf, crown (in leaf), limb close-up (bare) |
| `raw/sheet5/sheet-80.png` | The same at age 80 |
| `raw/sheet5/bases.png` | Trunk bases: 40 seed 1; 80 seeds 1, 7, 42 |
| `raw/sheet5/birch-<age>-<seed>-<shot>.png`, `render.log` | Every still; sizes and times |

| Age | Seed | Nodes | Leaves | Height | Crown width |
|---|---|--:|--:|--:|---|
| 40 | 1 | 384,717 | 258,147 | 16.8 m | 8.3 × 9.1 m |
| 40 | 7 | 540,483 | 358,833 | 16.8 m | 9.4 × 7.3 m |
| 40 | 42 | 604,024 | 403,937 | 16.7 m | 9.7 × 9.9 m |
| 80 | 1 | 922,783 | 602,161 | 22.7 m | 17.5 × 21.7 m |
| 80 | 7 | 1,095,781 | 723,157 | 24.4 m | 21.0 × 18.8 m |
| 80 | 42 | 1,134,169 | 784,617 | 23.4 m | 19.5 × 20.8 m |

## The worker's reading

- **(j) Regularity.** Along limbs and strands the laterals now stand at uneven spacing; with round 4's kinks the limb close-ups read as crooked, irregularly subdivided wood. Some outer sprays still read feathery in the in-leaf crown close-ups.
- **(b) Bark.** The 40-year stem is white to the ground but for a dark rim at the flare; at 80 the base is black and deeply fissured with lighter ridges to about a metre (seed 1), two metres (seed 42, the thickest), greying into the white stem above, as S1's base does.
- **(c) Crown.** An egg at 40 (8 to 10 m wide); at 80 a broader crown, 19 to 22 m wide, its top less pointed than sheet 4's, though seed 7 still carries its mass high.
- **Kept:** the white curving stem, ascending limbs, dark fine wood, hanging strands, sky between limbs and the yellow-green leaf mass.

The worker is confident the sheet reads as a mature open-grown silver birch at both ages, most strongly bare; the weakest views are the in-leaf crown close-ups at 80, whose sprays are finer and more feathery than S2-Oslo's leaf mass.

## Astra's verdict (2026-10-07)

Submitted with sheets 3 and 4's prompt, the paths changed to `raw/sheet5/` and SHEET-5.md, at commit `e12645b1` (`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-sheet5.md`). Its answer, verbatim:

> The trees are recognizably birches, but the 80-year specimens do not yet convincingly match the mature, open-grown crowns in the photographs.
>
> | Age | Seed | Reads as mature open-grown silver birch? | Visual reason |
> |---|---|---|---|
> | 40 | 1 | **Yes** | `sheet-40.png` shows a slender curved white stem, an irregular narrow crown and hanging foliage. |
> | 40 | 7 | **Nearly** | `sheet-40.png` shows convincing bark and drooping shoots, but the crown is compressed into a tall, narrow plume. |
> | 40 | 42 | **Yes** | `sheet-40.png` shows a plausible asymmetric egg-shaped crown with ascending forks and pendulous margins. |
> | 80 | 1 | **Nearly** | `birch-80-1-whole.png` reads as birch, but its sparse middle and long exposed branches make the crown look depleted. |
> | 80 | 7 | **Nearly** | `birch-80-7-whole.png` has convincing birch cues but retains a tall, pointed crown instead of a broad mature dome. |
> | 80 | 42 | **Nearly** | `sheet-80.png` shows a plausible leaning stem and wider spread, but the foliage remains concentrated around a tall central crown and thin projecting sprays. |
>
> Trait verdicts apply across the submitted set; exceptions are identified.
>
> | Trait | Verdict | Evidence |
> |---|---|---|
> | **(a) White, slender, curving or leaning stem** | **PASS** | `sheet-40.png` and `sheet-80.png` show slender white stems with visible bends, particularly the leaning 80-year seed 42. |
> | **(b) Dark base on the old tree** | **PASS** | `bases.png` clearly shows dark, vertically fissured bases on all three 80-year stems, with white bark above. |
> | **(c) Narrow egg at 40, broader dome at 80** | **FAIL** | `sheet-40.png` largely supplies the narrow egg, but `sheet-80.png` retains elongated crowns, especially seed 7, without a convincing broad dome. |
> | **(d) Ascending main limbs** | **PASS** | The bare views in both sheets show main limbs rising from the stem before spreading outward. |
> | **(e) Hanging fine outer shoots** | **PASS** | The limb close-ups in `sheet-80.png` show fine terminal shoots bending downward, also visible as hanging leafy strands in `sheet-40.png`. |
> | **(f) Dark fine wood in the bare crown** | **PASS** | Both sheets show dark brown-grey fine branches clearly differentiated from the white trunk and thicker limbs. |
> | **(g) Openness, with sky between limbs** | **PASS** | Both sheets show gaps between limb systems, although `birch-80-1-whole.png` carries that openness into an excessively empty middle crown. |
> | **(h) Leaf mass in leaf** | **FAIL** | `birch-80-42-crown.png` and `birch-80-1-whole.png` show foliage following thin sprays with much exposed framework, lacking the fuller hanging masses visible in the leafy references. |
> | **(i) Leaf colour** | **PASS** | `sheet-40.png` and `sheet-80.png` show a plausible yellow-green, though brighter and paler than much of the reference foliage. |
> | **(j) Other features that read as wrong** | **FAIL** | `birch-80-42-crown.png` shows repeated feather-like leafy sprays attached to long exposed axes, giving parts of the crown a fern-like texture. |
>
> **Overall verdict: FAIL.**
>
> The single most useful change would be to **make the foliage form fuller, irregular hanging masses instead of narrow feather-like sprays**, while preserving gaps between the main limbs.

**In short:** 40 seeds 1 and 42 read *yes*; 40 seed 7 and all three at 80 *nearly*. (b) the dark base now **passes**, as do (a), (d), (e), (f), (g) and (i). (c) fails at 80 (elongated crowns, seed 7 pointed); (h) fails (foliage along thin sprays, framework exposed); (j) fails, now on "repeated feather-like leafy sprays attached to long exposed axes". Overall **FAIL**.

## For the host: two wrong-path stops, with their causes

Both (c) and (j) are now named twice after changes aimed at them: (c) on sheets 4 and 5 (round 5 aimed at it), (j) on sheets 3, 4 and 5 (rounds 4 and 5 aimed at it). Per the host's rule the worker reports the causes and stops; no further values were tried.

**(c), the crown at 80 stays tall.** The stem never loses its lead: `TOP` carries the leader on at 0.1 m a year to 80 and beyond (`continuation`, lifespan 100), while the limbs, set to ascend steeply in round 2 (set-point 1.3 rad, host point 2), climb beside it, so the crown's mass rises with the leader. MBE99 describes the opposite in the mature grey birch: "the extremity of the main stem assume[s] a divaricated structure with very short growth units", the trunk ending "by a system of small, short forks", the crown's top made by the limbs. Untried value candidates, for the host to choose among: end `TOP` (a lifespan of 10 to 20 years with no continuation, so the leader stops in the 50s and the limbs overtop it), or let its apex die and relay often (`abortion` with relays, as round 4 gave the limbs), and lower the limbs' set-point with age. Whether that is values or a decision on the architecture (rule 5's dominant leader against rule 6's forking top) is the host's call.

**(j) and (h), feather-like leafy sprays.** The leaves are the dressing's: one station every 12 mm along every run thinner than `shootRadius` (`placement.rs`, `runs`), so a long hanging shoot is clothed evenly from base to tip and reads as a two-sided feather. A birch carries most of its leaves two or three together at its short shoots (rule 9, SU03; JH87), so its foliage hangs in clumps along the strand. The engine grows those short shoots (`SHORT`, a few millimetres a year), but the dressing does not seat a cluster at them; the canopy's own short-shoot clusters (`short_shoots.rs`, the `/canopy/shortShoot*` rows the beech's preset states) seat clusters on wood by their own draw, not at the engine's short shoots. Options: (i) a capability spec: leaves seated as a cluster at the tip of each engine short shoot (the engine's `SHORT` axes), the stations along long shoots kept for their own leaves; (ii) try the canopy's short-shoot cluster rows on the birch as values (they place clusters along slender wood, though not at the engine's short shoots), with stations along long shoots made sparse; (iii) accept the sprays as a known gap, as the owner's verdict rule allows when host and Astra do not agree.

## Gate (round 5, commit `e12645b1`)

Render half first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0. Then `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0. `harness/parity.test.ts` passes 8 of 8. All with `CARGO_BUILD_JOBS`, `RUST_TEST_THREADS` and `RAYON_NUM_THREADS` at 8; logs in the ignored `raw/gate5-*.log`.
