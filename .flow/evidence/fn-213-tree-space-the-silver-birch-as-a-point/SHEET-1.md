# fn-213 sheet 1: the birch beside its references (worker, 2026-10-07)

The first sheet of the birch point in MODEL-BIRCH.md, for the host to judge (R3). The worker viewed every still named here. Nothing below is a verdict; the worker's reading is offered for the host's.

## The stills

Rendered by `cargo run --profile ci -p telperion-render --example space_birch -- <dir> 40 80 --seeds 1,7,42` at commit `0b2074d7`, through the one pipeline and the headless renderer, the camera fitted to each tree. They are in the ignored `raw/sheet1/` folder:

| File | What |
|---|---|
| `raw/sheet1/sheet-40.png` | Row 1: the references S1, S3-crown, S2-Oslo, S2-Uusimaa. Rows 2 to 4: age 40, seeds 1, 7, 42, each as hero bare, hero in leaf, crown (in leaf) and limb close-up (bare) |
| `raw/sheet1/sheet-80.png` | The same at age 80 |
| `raw/sheet1/bases.png` | Trunk bases: 40 seed 1, 80 seeds 1 and 7 |
| `raw/sheet1/birch-<age>-<seed>-<shot>.png` | Every still: `bare`, `whole`, `base`, `limb`, `spray`, `crown` |
| `raw/sheet1/run.log` | Sizes and times (also in MODEL-BIRCH.md, "Cost") |

The still runner's crown shot is in leaf only; a bare crown view is not drawn.

## The references

The fn-188 set in `.flow/references/silver-birch/` (README.md there holds each file's checksum and resize). All are Wikimedia Commons photographs under Creative Commons licences that permit redistribution with attribution; none is a copyrighted copy without one.

| Id | Source | Author | Licence |
|---|---|---|---|
| S1-newforest-heath-winter | https://commons.wikimedia.org/wiki/File:Betula_pendula_NewForest.jpg | Treehouse1977 on Flickr | CC BY-SA 2.0 |
| S2-oslo-lawn-leaf | https://commons.wikimedia.org/wiki/File:Silver_Birch_(Betula_pendula)_-_Oslo,_Norway_2020-08-04.jpg | Ryan Hodnett | CC BY-SA 4.0 |
| S2-uusimaa-field-leaf | https://commons.wikimedia.org/wiki/File:B._pendula,_Koivu_Birch_end_of_Sept.jpg | Abc10 | CC BY-SA 4.0 |
| S3-crown-limbs-sunset | https://commons.wikimedia.org/wiki/File:Betula_penuda_at_sunset_in_winter.jpg | Trance Light | CC BY 3.0 |
| S3-limb-twig-curtains | https://commons.wikimedia.org/wiki/File:Hengebj%C3%B8rka_ved_gamlevegen_i_Grythengen.jpg | Øyvind Holmstad | CC BY-SA 4.0 |
| S4-pendant-shoots-budbreak | https://commons.wikimedia.org/wiki/File:Betula_pendula_Brzoza_brodawkowata_2023-04-15_So%C5%82tysowice_05.jpg | Agnieszka Kwiecień, Nova | CC BY-SA 4.0 |

## The worker's reading

What reads as a silver birch:

- A single, slender, white stem carried through the crown (MA82's dominant leader).
- A narrow ovoid crown at 40 (about half as wide as tall, as S2-Uusimaa) becoming broad and rounded at 80 (OSU's "pyramidal becoming rounded").
- Long fine shoots hanging in strands from ascending limbs, in leaf and bare; at 40 the strands match S2-Uusimaa's hanging leafy shoots.
- Height on LO96's curve: 17 m at 40, 23 to 24 m at 80.
- Seeds 1, 7 and 42 differ in limb placement and crown lumps; none is an outlier.

What does not yet read, against the references:

1. **The bare crown is a dense dark haze.** S1 and S3-crown show an open lattice of ascending limbs and twigs with sky through it; at 40 the upper crown is a near-opaque grey-brown mass. The short shoots kept ten years and the hanging systems' count make it. The likeliest fix is values (fewer hanging systems, shorter-lived short shoots), traded against rule 8.
2. **The old limbs are long straight horizontal spokes at 80.** In the limb close-ups at 80 they run near-horizontal and parallel; S1's and S3-crown's limbs ascend and fork. The limb's continuation as `BOUGH` (set-point 0.6 rad) flattens them. Values.
3. **Limbs are white to their tips.** In S1 and S3-crown limbs are white only where thick and dark towards their ends. The shoot colour reaches only wood under 1 cm (`/material/shootRadius`). Values.
4. **The in-leaf crown is bluish and uniform.** S2 is yellow-green with sky between leaf masses. The leaf colour rows were tuned for today's generator under its lighting. Values (material).
5. **The stem is straight and white to the ground.** S1 and S2-Uusimaa show a gently curving or leaning stem with a black, fissured base; the base close-ups at 80 are white with lenticel marks (MODEL-BIRCH.md design question 5).
6. **No fork in the stem.** S2-Oslo forks at about a quarter of its height into two ascending stems; this point never forks (MBE99 records such twin crowns as occasional, at most two).

## Gaps for the host

- **Native and Wasm topologies differ for the birch.** `harness/parity.test.ts` ("native/wasm topology bytes and positions to rounding: silver-birch") fails: at the preset's own tree (age 40, seed 42) the native build keeps 1,417,415 nodes and the Wasm build 1,417,246. The other four engine presets pass. Probes that each changed one thing and rebuilt both sides kept the gap at exactly 169 nodes: a thicker short-shoot pipe (0.0007 to 0.001 m), a longer short-shoot internode (3 to 5 mm), no `sag` anywhere, no `bendLength` anywhere. So it is not the conversion's thinnest-wood or shortest-internode cut, nor the beam or the remembered bend. Unknown: which engine draw rounds differently. Options: (a) find the draw (an engine or conversion fix, its own spec); (b) the host's pending decision to pin the engine's math (fn-198) settles it; (c) values that avoid it, found by search, which hides rather than fixes it.
- **Pendulous shoots by set-point, not by weight** (MODEL-BIRCH.md design question 2): drawable now; whether it stands is the host's call.
- **Dark old bark** (question 5): unknown whether material rows or a capability.

## The host's reading (2026-10-07, both sheets viewed)

Reads as birch: the white stem, the hanging strands, the narrow crown at 40 rounding by 80. Not yet:

1. The crown is far too dense, bare and in leaf; the references show an open crown with sky between ascending limbs.
2. Main limbs at 80 run out as flat horizontal white spokes; a birch's main limbs ascend steeply and the weeping strands hang from them.
3. All wood is white to the tips; a birch is white on the stem and thick limbs only, its fine wood dark brown to purple-grey (that dark fine wood is much of how a bare birch reads).
4. Leaves bluish; they should be a bright yellow-green.
5. The stem never curves or leans, and there is no dark fissured base at 80.

## The host's decisions

- (a) Merge `fn-198-tree-space-f-the-engine-in-production` (pinned math, F2b, faster tests) first and re-run `harness/parity.test.ts`; if the 169-node gap remains, stop that thread and report it for its own spec.
- (b) Hanging by a target angle below the horizontal stands if it reads right.
- (c) Bark white by girth and dark on fine wood and on the old fissured base: try the existing material rows first; if they cannot express colour by radius or age, report a capability gap with options, never fake it.
- (d) Monopodial axes and the estimated crown width stand.

Round 2 aims at (1), (2), (4) and (5)'s lean by values, then a second sheet in the same layout (SHEET-2.md).
