# fn-236 sheet 1: round 1, and a stop for the host (worker, 2026-10-09)

Round 1 moves the oak's `/space` rows towards the sources in MODEL-OAK.md and is mirrored into `oak.rs`. It fixes the stem, the tuft and most of the size, but the crown still does not close into a dome. Twenty-one probes on values did not close it either, so this round stops for the host: the closed, round crown (T1) is the trait in question.

## What moved

| Row | 0 | 1 | Aimed at |
|---|--:|--:|---|
| `LEADER` lifespan, `/space/ages/4/lifespan` | 19 | 6 | the tuft above the dome; width over height at 40 |
| `LIMB` internode, `/space/ages/6/internode` | 0.045 | 0.035 | the 80-year size (T6 to T9) |
| `LIMB`, `BOUGH` light share ψ, `/space/ages/{6,7}/shadeSize` | 1.5 | 0.5 | lit limbs outgrowing the crown |
| `BOUGH` elevation, `/space/ages/7/form/elevation` | 0.5 | 0.9 | boughs spreading flat past the edge |
| `retained`, every age | 0.5 | 0 (row removed) | DBH two to four times Silvics' |
| Medial `SHORT` share on `LIMB` / `BOUGH` / `BRANCH` / `TWIG` | 0.25 / 0.3 / 0.3 / 0.4 | 0.4 / 0.5 / 0.6 / 0.7 | crown density |
| `SHORT` pipe, `/space/ages/14/form/pipe` | 0.00095 | 0.0006 | the added short shoots' girth |

All **estimated**: each was chosen by probe against the measures and the stills, inside the sources' ranges where the sources give one. The `retained` value was a fn-197 host value (decision 29); it is the oak's own row, and 0 is today's neutral.

## Measures (`oak_measure.rs`; `raw/measure-r0.jsonl`, `raw/measure-r1.jsonl`)

Seeds 1 / 7 / 42.

| Measure | r0 @40 | r1 @40 | r0 @80 | r1 @80 | Target (derived) |
|---|---|---|---|---|---|
| Height, m | 12.5 / 13.5 / 12.4 | 10.8 / 11.4 / 9.9 | 22.3 / 26.9 / 24.2 | 17.0 / 21.4 / 16.7 | 8–12; 14–24 |
| Crown width, m | 10.3 / 11.1 / 10.3 | 10.4 / 11.2 / 10.7 | 21.8 / 29.6 / 25.4 | 15.5 / 23.0 / 18.9 | 6–11; 10–20 |
| Width / height | 0.83 / 0.82 / 0.84 | 0.97 / 0.98 / 1.09 | 0.98 / 1.10 / 1.05 | 0.91 / 1.07 / 1.13 | 1.0–1.3 |
| Crown base, m | 2.6 / 4.7 / 3.3 | 2.4 / 4.6 / 2.8 | 5.4 / 5.6 / 8.7 | 4.9 / 5.6 / 7.8 | low |
| Fill | 0.65 / 0.80 / 0.74 | 0.66 / 0.79 / 0.70 | 0.87 / 0.88 / 0.87 | 0.84 / 0.88 / 0.82 | 0.99 (O-WHOLE) |
| Solid | 0.66 / 0.63 / 0.72 | 0.71 / 0.65 / 0.74 | 0.73 / 0.84 / 0.81 | 0.81 / 0.83 / 0.82 | 0.92 (O-WHOLE) |
| Spikes | 10 / 2 / 3 | 6 / 6 / 3 | 4 / 3 / 1 | 3 / 6 / 1 | 0–1 |
| Top tuft, m | 1.2 / 1.7 / 0.6 | 0.7 / 0.8 / 0.5 | 2.2 / 4.8 / 4.9 | 1.1 / 2.1 / 1.9 | small |
| DBH, cm | 73 / 91 / 80 | 49 / 70 / 49 | 177 / 295 / 238 | 85 / 201 / 96 | 20–41; 41–81 |
| Nodes | 167,123 / 291,579 / 218,923 | 172,527 / 281,789 / 192,194 | 947,012 / 2,222,419 / 1,481,549 | 677,910 / 1,493,410 / 753,114 | |
| Grown, ms (`space_oak`) | 631 / 881 / 730 | 779 / 1,095 / 800 | 6,663 / 13,346 / 8,133 | 7,335 / 15,765 / 5,843 | |

- **Size:** inside the derived ranges at 40 and, but for seed 7's 23 m crown, at 80.
- **Stem:** DBH falls by a third to a half; seed 7 at 80 is still 2.0 m, the rest near the top of the range.
- **Tuft:** gone at 40, halved at 80.
- **Closure and roundness:** fill does not move; solid rises a little; spikes do not fall.
- **Cost:** nodes at 80 fall by 28 to 49 %. The build times ran at loads of 4 to 12 (other sessions) and are noise at this size; they are re-measured on a quiet machine on the final round (R4).

## The stills

`raw/compare/compare1-40.png`, `compare1-80.png` (per seed: O-WHOLE, 0.1.4, round 0, then round 1 in leaf, bare and crown). Round 1's own stills: `raw/sheet1/`. I viewed both sheets, and every probe sheet named below.

- **At 80:** the trunks are now a believable girth and the old leader no longer stands above the crown. Seed 7 reads nearest a dome, though limbs show through its lower half. Seed 42 is a rounded outline that is see-through, with the wood visible across it. Seed 1 is lopsided and open, with a clump of foliage on its left and bare limbs on its right.
- **At 40:** each tree is an upright, open, shrubby young oak, with splayed low branches at its foot. None is a dome, and each is narrower to the eye than its 1.0 width over height says, because the width comes from the low branches.
- **Against O-WHOLE and 0.1.4:** both are closed masses of foliage. Round 1's foliage still sits in separate dark tufts at the ends of a modest number of axes.

## What the probes showed (`raw/measure-*.jsonl`; stills `raw/sheetc1` to `sheetc5`)

| Probe | Change (on the base named) | What it did |
|---|---|---|
| p1 | `LEADER` 6 years | height and the tuft fall; w/h 0.90–0.96 at 40 |
| p2 | ψ 0.5 on `LIMB`, `BOUGH` | seed 7's crown 30 → 23 m; spikes at 80 fall |
| p3 | `LIMB` elevation 0.8, insertion 0.7 | wider at 80 only; fill falls |
| p4 | more branches and boughs on limbs | **refused at 80: the 20 M grown-phytomer budget** |
| c1 | p1 + p2 + limb internode 0.035 | taken (sheet `comparec1-*`) |
| c2 | c1 + more boughs | DBH to 6 m, spikes rise |
| g1, g3 | c1 with `retained` 0; c1 with stem and limb pipe halved | `retained` halves DBH; own pipe barely moves it; g1 taken |
| d1, d2 | more boughs; limbs fork at 0.03 | ragged and wider; d2 **refused at 80: budget** |
| e1, e2 | branch systems longer (internode 0.025) or living 20 years | wider and lower fill |
| f1 | the light's shedding off (`balanceHazard`, `shadeHazard` 0) | fill unchanged: the gaps are not shed shoots |
| h1–h4 | limb sag 1e-4, limb tropism 0.8, ψ 0, limbs carrying on as boughs at 30 | seed 7 still 25 m wide |
| k1–k3 | bough elevation 0.9; bough internode 0.02 | seed 7 25 → 20 m; k1 taken |
| s1 | more short shoots (taken) | +10 % nodes, fill unchanged |
| c4 | limb elevation 0.85, insertion 0.8, straightening 0.05 | low limbs splay at 80 (seed 7 w/h 1.62); reverted |
| t1, t2 | trunk 8 years (an earlier fork) | broad at 40 (w/h 1.1–1.2), too wide at 80 (21–35 m) |
| b1 (c5) | boughs end at 30 years | smaller and sparser, lopsided seed 1; reverted |

A limb census of seed 7 at 80 (`oak_limbs`, scratch): 175 limb axes (144 of them relays), 846 boughs (613 relays), and the farthest tips are boughs', 12.4 m out against limbs' 8.9 m. Boughs relay at every abortion and at their lifespan's end, so they grow for the tree's life.

## For the host

1. **The closed dome (T1) is not reached by the values tried.** The crown's foliage stands at the ends of the axes, and more axes either break the 20 M grown-phytomer budget (p4, d2) or widen the crown and lift DBH without closing it (c2, d1, e1, e2). Turning the light's shedding off did not close it (f1). The gaps are between the limbs' foliage clouds. The 0.1.4 oak filled an envelope, and the engine draws none. Whether another set of values can close the crown, or the trait needs a capability, is the host's call. One untried lever outside this brief is the dressing's leaf rows (leaf size, `/skeleton/twigs/twig/internodeLength`, `/canopy/*`), which are not `/space` rows.
2. **Width at 40 against size at 80.** An earlier fork (t1) gives the 40-year oak a dome's proportions, but the same limbs make the 80-year crown 21 to 35 m wide, past PH82's 10 to 20 m. Values move both ages together.
3. **The ordinary tree.** `the_ordinary_tree_is_the_centroid_of_the_branching_species` fails on round 1, as MODEL-OAK.md foresaw: the ordinary preset is the centroid of the beech, the oak and the spruce. The fix is to regenerate the ordinary value file with `examples/centroid.rs`, which changes the ordinary tree (R6 says other species stay byte-identical), or to leave the test red. Not done; the host's call. `every_engine_preset_carries_the_engines_species` passes: `oak()` equals the preset.
4. **Seed 7** grows twice the nodes of seeds 1 and 42 at 80 under every probe, and is the widest each time.

Not run on round 1: the gate, `npm test`, the catalogue digests and `presets.generated.ts` (the round is not ready for a verdict), and the site's field stills at n = 36 and 48 (R3).

**My reading:** round 1 is better than round 0 at 80 (stem, tuft, size), but it is not the target, and I would not put it to the owner.
