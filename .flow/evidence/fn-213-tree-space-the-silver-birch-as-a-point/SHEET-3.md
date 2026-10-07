# fn-213 sheet 3: round 3 beside the references (worker, 2026-10-07)

Round 3 aims at the host's two remaining points on sheet 2 (SHEET-2.md, "The host's reading"): (1) in leaf the crown read as sparse strands, not a leafy mass; (2) at 80 the crown was narrower than S2-Oslo's and S3's. The values are in MODEL-BIRCH.md, "Round 3". The worker viewed every still named here.

## The stills

Same layout as sheets 1 and 2, in the ignored `raw/sheet3/`, at the round-3 commit:

| File | What |
|---|---|
| `raw/sheet3/sheet-40.png` | Row 1: references S1 (New Forest, winter), S3-crown (limbs at sunset), S2-Oslo (lawn, in leaf), S2-Uusimaa (field, in leaf). Rows 2 to 4: age 40, seeds 1, 7, 42, each as hero bare, hero in leaf, crown (in leaf), limb close-up (bare) |
| `raw/sheet3/sheet-80.png` | The same at age 80 |
| `raw/sheet3/bases.png` | Trunk bases: 40 seed 1; 80 seeds 1, 7, 42 |
| `raw/sheet3/birch-<age>-<seed>-<shot>.png`, `render.log` | Every still; sizes and times |

The references, their sources and licences are SHEET-1.md's ("The references"); the files are in `.flow/references/silver-birch/`.

| Age | Seed | Nodes | Leaves | Height | Crown width |
|---|---|--:|--:|--:|---|
| 40 | 1 | 389,597 | 323,883 | 16.7 m | 8.1 × 7.5 m |
| 40 | 7 | 560,812 | 475,429 | 16.8 m | 8.9 × 7.6 m |
| 40 | 42 | 588,577 | 480,787 | 16.6 m | 8.6 × 9.1 m |
| 80 | 1 | 762,193 | 653,301 | 24.0 m | 17.8 × 18.0 m |
| 80 | 7 | 984,749 | 837,695 | 24.7 m | 20.0 × 17.1 m |
| 80 | 42 | 936,851 | 804,587 | 24.7 m | 18.6 × 19.4 m |

## The worker's reading

1. **Leaf mass.** About 1.7 times sheet 2's leaves, larger blades: the in-leaf crowns read as a leafy mass along hanging strands, with sky still between limbs at 80. At 40 the crown is near S2-Uusimaa's; it is still lighter than S2-Oslo's full mass.
2. **Width at 80.** 17 to 20 m against sheet 2's 14 to 16; the 80-year crown is a broad dome, the 40-year one still a narrow egg (8 to 9 m, about half the height). Seed 7's lowest limb at 80 hangs to the ground beside the stem.
3. The bare crowns at 40 and 80 keep sheet 2's look: white stem, ascending limbs, dark fine haze, hanging strands.

## Astra's verdict (2026-10-07)

Submitted as `codex exec -m gpt-6-astra --sandbox read-only -C <worktree> -o raw/astra-sheet3.md "<prompt>" < /dev/null`, at commit `25986b14`. The prompt gave it `raw/sheet3/sheet-40.png`, `sheet-80.png`, `bases.png`, the individual stills, this file and MODEL-BIRCH.md, and asked, per age and seed, whether the tree reads as a mature open-grown silver birch beside the references (yes, nearly, no), then pass or fail per trait (a) to (j) with evidence, an overall verdict and the single most useful change. Its answer, verbatim:

> **All six are nearly convincing.** The white stems, ascending limbs and hanging foliage establish birch, but the repeated branch structure weakens the likeness beside the photographs.
>
> | Age | Seed | Judgment | Visible reason |
> |---|---|---|---|
> | 40 | 1 | **Nearly** | Slender curved stem and narrow crown; foliage forms conspicuously repetitive feather-like sprays. |
> | 40 | 7 | **Nearly** | Convincing narrow silhouette and pendulous foliage; bare branching looks crowded and mechanically regular. |
> | 40 | 42 | **Nearly** | Plausible asymmetric crown; repeated upright limbs and hooked tips remain conspicuous. |
> | 80 | 1 | **Nearly** | Broader, rounded crown reads as mature; exposed lower branches form an unusually regular fan. |
> | 80 | 7 | **Nearly** | Strong leafy crown, but the isolated low branches and orderly internal framework distract. |
> | 80 | 42 | **Nearly** | Broad asymmetric crown suggests an older birch; long parallel branches with repeated drooping ends weaken it. |
>
> | Trait | Verdict | Visible evidence |
> |---|---|---|
> | **(a) White, slender, curving or leaning stem** | **Pass** | `sheet-40.png` and `sheet-80.png` show slender white stems with visible bends in every seed. |
> | **(b) Dark base on the old tree** | **Fail** | `bases.png` shows more dark markings at 80, but all three old bases remain predominantly white with black speckling rather than a convincing dark, fissured lower trunk. |
> | **(c) Narrow egg at 40, broader dome at 80** | **Pass** | `sheet-40.png` shows narrow elongated crowns, while `sheet-80.png` shows wider, more rounded crowns, although seed 7 retains a pointed top. |
> | **(d) Ascending main limbs** | **Pass** | The bare views in both sheets clearly show main limbs rising outward from the stem. |
> | **(e) Hanging fine outer shoots** | **Pass** | `birch-80-42-limb.png` clearly shows fine terminal shoots arching downward, also visible as hanging foliage in both sheets. |
> | **(f) Dark fine wood in the bare crown** | **Pass** | The bare crowns in both sheets carry a dark brown-grey twig haze around the pale structural wood. |
> | **(g) Openness, with sky between limbs** | **Pass** | Both sheets show gaps between limb systems, especially in the lower and outer crowns at 80. |
> | **(h) Leaf mass in leaf** | **Pass** | The leafy whole-tree views in both sheets form coherent crown masses, with seed 7 at 80 especially full, though less continuous than the Oslo reference. |
> | **(i) Leaf colour** | **Pass** | `birch-40-1-crown.png` shows plausible fresh yellow-green foliage, albeit brighter and more uniformly lime-coloured than the reference crowns. |
> | **(j) Other features wrong for birch** | **Fail** | `birch-80-42-limb.png` shows long, nearly parallel branches ending in repeated umbrella-like sprays, and `birch-40-1-crown.png` gives those sprays a fern-like regularity absent from the photographs. |
>
> **Overall verdict: FAIL.**
>
> **Single most useful change:** break up the repeated long-branch-and-drooping-tuft pattern into more irregular, subdivided branch systems, so the crown loses its comb-like structure.

**In short:** every age and seed is *nearly*; traits (a) and (c) to (i) pass; (b) the dark old base fails (the host's decision keeps the tint until the mature-bark capability spec); (j) fails on a repeated, comb-like branch structure: long nearly parallel limbs ending in regular drooping tufts. Overall **FAIL**; its one change: break the repeated long-branch-and-tuft pattern into more irregular, subdivided branch systems.

## Gate (round 3, commit `25986b14`)

In two parts, as the host set (2026-10-07), with `CARGO_BUILD_JOBS=8` and `RUST_TEST_THREADS=8`: `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` without the GPU lock, 89 targets ok and none failed; `cargo test --profile ci -p telperion-render --no-fail-fast` under the lock, 41 targets ok and none failed. `harness/parity.test.ts` passes 8 of 8 on round 3's values (the Wasm rebuilt). Logs in the ignored `raw/gate3-*.log`.

## The host's reading (2026-10-07, sheet-80 viewed)

The host agrees with Astra's (j): the limb close-ups show long, straight, nearly parallel limbs with drooping tufts at regular spacing, where the references show crooked, irregularly subdivided limbs. Seed 7's lowest limb at 80 reaches the ground.

Round 4: break the regularity by values (more variation in lateral angles and lengths, more subdivision of the mid-crown limbs, the persistent curvature of fn-207's `form.bend_length`, occasional forks where the engine allows, the low limb raised or shed); if values cannot, report a capability gap with options. (b) waits on the new spec "Bark that matures with girth"; the birch passes only when it lands. Sheet 4 in the same layout (SHEET-4.md), then Astra with the same prompt.
