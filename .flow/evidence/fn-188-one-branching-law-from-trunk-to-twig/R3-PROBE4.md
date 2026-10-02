# fn-188 R3, round 4: straightness, real foliage, twig size, frozen growth, 2026-10-03

## Setup

- **Code:** the speed probe (`1e36a438`), extended, committed as `fff4da72` and reverted in `afbea3fc`.
- **Runs:** release on the Ryzen 9 5950X, 16 threads, with the speed probe's exact optimisations. Round 3's beech digest is unchanged by the new code (`dbe0cdc9…`).
- **Inputs and rows:** `raw/probe4/`, which holds `walk.jsonl`, `frozen.jsonl`, `render4.jsonl`, `chosen-bench.jsonl`, the overlays and the two photo scripts.

## 1. Straightness

**The direction rule (Pałubicki 2009, §4.3)** is new direction = normalise(previous direction + ξ·V + η·T).

- V is the bud's direction to its markers.
- T is the tropism vector, now a parameter (default up).
- The previous direction carries weight 1.

**The measures:**

- **Tortuosity by order:** path ÷ chord over every axis of 1 m or more, median per order.
- **Projected limb tortuosity:** the trunk axis and the five thickest first-order axes, each resampled at six points evenly by arc length. The points are projected through the pinned camera, then path ÷ chord in the image.
- **B-BARE**, measured the same way: five limbs traced by hand off a 3x zoom of `fasy896.jpg` with a 10 px grid (left, centre and right stems from the fork near y 277, a far-right and a far-left limb). There are 4 to 6 points each; the coordinates are in `photo_tort.py`.
  - Projected tortuosity per limb: 1.0006, 1.0000, 1.0037, 1.0098, 1.0004; **median 1.0006**.
  - Hand tracing at 534 px resolves no small bends, so this is a lower bound on the photo's real tortuosity.

**Beech G2, seed 1** (all other rows as round 3):

| walk | value | tortuosity o0 / o1 / o2 / o3 | projected limbs | angle | strong | nodes | tips | ms |
|---|---|---|--:|---|--:|--:|--:|--:|
| ξ | 0.05 / 0.1 / 0.2 / 0.3 / 0.5 / 0.8 | o1 1.008–1.017, o3 1.010–1.018 | 1.0087 / 1.0091 / 1.0094 / 1.0106 / 1.0114 / 1.0216 | 36→43° ±4.6–7.4 | 13–16 | 59.8k → 45.8k | 13.0k → 8.1k | 69 → 48 |
| η | 0 / 0.05 / 0.15 / 0.3 / 0.5 / 0.8 | o0 1.045–1.052, o3 1.007 → 1.028 | 1.0093 / 1.0094 / 1.0106 / 1.0109 / 1.0111 / 1.0105 | 42→23° | 12–16 | 48.3k → 75.4k | 8.9k → 17.5k | 52 → 101 |
| scaffold `crookedness` at ξ 0.05 | 0 / 1.5 / 3 / 6 / 10 | o0 1.024 → 1.046, o1 1.001 → 1.014 | 1.0014 / 1.0024 / 1.0030 / 1.0050 / 1.0087 | 36±4.6 | 10–13 | 60.1k–60.3k | 13.0k–13.1k | 66–70 |
| η at crookedness 0, ξ 0.05 | 0 / 0.05 / 0.15 / 0.3 | o1 1.001, o3 1.000 / 1.002 / 1.014 / 1.030 | 1.0012 / 1.0012 / 1.0014 / 1.0017 | 40→31° | 10–11 | 53.8k → 74.6k | 10.9k → 17.7k | 60 → 94 |

- **ξ and η barely move the limbs.** The measured limbs are scaffold axes, whose geometry is given.
- **The scaffold's own `crookedness`** (G2: 10; the shipped beech: 3) moves the projected limb tortuosity by degree, 1.0014 → 1.0087.
- **η mainly bends the fine wood:** o3 1.000 → 1.030.
- **Chosen by the measured value nearest the photo's 1.0006:** crookedness 0, ξ 0.05, η 0 (1.0012; η 0.05 ties at 1.0012).

## 2. Real foliage

**The pipeline's own foliage is applied through a public seam:**

- The probe's tree is written as a `telperion_core::tree::Tree` and handed to `pipeline::executor::expand(tree, family)?.mesh()`, the executor interface's CPU build of a given tree.
- That runs the pipeline's own wood sweep and its foliage stage (placement and cull).
- **Node kinds:** scaffold nodes are `Structural`; grown nodes are `Twig` on an unbranched terminal shoot (one tip of demand) and `Branch` otherwise.
- **No code is copied.** One design choice remains: which grown wood is twig wood.

**Coverage method, the same on render and photo:**

- **Silhouette:** per image row, from the tree's top down to 0.8 of its height in the frame, the span between the outermost tree pixels.
- **Tree cover:** tree pixels in the spans ÷ span pixels.
- **On the render:**
  - A tree pixel differs from a render of the empty scene by more than 8 levels in a channel and is not a shadow. A shadow pixel is darker in every channel by one common factor, within 0.08.
  - Leaf cover counts pixels that differ from the bare render.
- **On B-WHOLE (`fasy951.jpg`, two photographs side by side):**
  - Sky is min(RGB) ≥ 170 with max − min ≤ 45, and tree is every other pixel.
  - The right tree is x 590–1180, with its trunk base read at y 800 (`photo_cov.py`).
  - The sky thresholds were varied (150 to 190 and 30 to 45).
- **Result:** B-WHOLE's right tree reads **0.876 to 0.918**, and its left tree 0.849.

| beech s1 at the pinned pose | leaves | tree cover | leaf cover | pipeline wood + foliage ms |
|---|--:|--:|--:|--:|
| shipped preset (`mesh::build`) | 5,828,744 | 0.847 | 0.820 | 3,221 |
| shipped G2 at crookedness 0 | 1,020,129 | 0.612 | 0.529 | 566 |
| probe, G2 canopy rows (spacing 0.08, radius 0.3) | 517,101 | 0.721 | 0.615 | 292 |
| probe, the shipped beech's canopy rows (spacing 0.03, radius 0.7) | 1,198,471 | 0.842 | 0.806 | 617 |
| **probe, short shoots every 0.02 m, radius 0.7 (chosen)** | **1,757,735** | **0.877** | **0.856** | 883 |

- With 3 mm tips, all rows are at the chosen straightness.
- **The chosen setting reaches the low end of B-WHOLE's range** with 30% of the shipped preset's leaves.

## 3. Twig size

**What sets the tip radius:** the probe's pipe pass holds the root and gives every tip one unit of demand, so a tip is R · N^(−1/e).

- With R = 0.512 m, N ≈ 10k tips and e = 2.9 (`forkExponent`), that is 2.1 cm.
- Scaffold wood also keeps its own radius as a floor.

**The change:** with a terminal radius r_t, e = ln N ÷ ln(R / r_t), and tips land at r_t with the root unchanged.

| r_t | e | tip radius | leaves | tree / leaf cover |
|---|--:|--:|--:|---|
| none | 2.9 | 20.8 mm | 513,695 | 0.791 / 0.486 |
| 4 mm | 1.916 | 4.0 mm | 517,088 | 0.724 / 0.608 |
| **3 mm (chosen)** | 1.808 | 3.0 mm | 517,101 | 0.721 / 0.615 |
| 2 mm | 1.676 | 2.0 mm | 517,140 | 0.718 / 0.620 |

- These rows use G2 canopy rows.
- **The lower exponent also thins the side branches against their parents.** The chosen beech's strong limbs fall from 14 to 7 (s1) and 4 (s7), against B-BARE's 4 to 6.
- **The chosen beech, s1 / s7:**
  - o3/o1 0.027 / 0.024, mid 0.68.
  - Projected limbs 1.0012 / 1.0009; tortuosity by order 1.026 / 1.0009 / 1.0003 / 1.0003 at s1.
  - 53,784 / 51,847 nodes; 59 ms probe, against a shipped skeleton of 16 ms and a scaffold of 2.5 to 3.6 ms.

## 4. Frozen-marker parallel growth

Within a cycle every shoot reads the same marker set and grows on its own thread. Nodes are committed in bud order, and the new wood occupies its markers afterwards. The result is deterministic, but the tree differs.

Both modes run at 16 threads with early stop, warm median of 5; rows use round 3's values.

| preset s | sequential ms | frozen ms (p95, cold) | shipped skeleton / scaffold ms | nodes | strong | fine m/m | tips |
|---|--:|---|---|---|---|---|---|
| ordinary 1 / 7 | 32.6 / 30.1 | 38.4 / 34.8 | 38.1 / 8.8, 41.5 / 8.2 | +3.5% / +2.2% | 4→4 / 6→6 | 170→175 / 249→222 | −0.2% / −1.0% |
| oak 1 / 7 | 138.5 / 122.4 | 157.4 (165, 177) / 124.9 | 27.7 / 3.1, 36.6 / 4.1 | +2.7% / +3.1% | 4→4 / 3→3 | 344→354 / 309→268 | −1.1% / −0.9% |
| spruce 1 / 7 | 21.5 / 22.6 | 25.8 / 25.7 | 33.1 / 14.0, 31.6 / 13.5 | +2.6% / +2.5% | 8→8 / 9→8 | 407→448 / 400→410 | +0.3% / +0.2% |
| birch 1 / 7 | 42.1 / 44.3 | 48.3 / 54.1 | 20.9 / 3.7, 27.2 / 3.9 | +3.4% / +4.3% | 6→6 / 8→7 | 92→97 / 116→119 | −0.9% / +0.3% |
| date palm 1 / 7 | 2.5 / 2.3 | 3.9 / 3.3 | 1.6 / 1.5, 1.6 / 1.6 | +5.6% / +3.0% | 2→2 / 1→1 | 0.9→0.8 / 0.7→0.7 | −1 / +2 |
| telperion 1 / 7 | 16.7 / 17.9 | 20.0 / 21.5 | 333.2 / 6.3, 49.1 / 4.6 | +3.4% / +4.2% | 1→1 / 3→3 | 51→69 / 47→43 | −1.5% / +0.9% |
| laurelin 1 / 7 | 154.5 / 149.9 | 182.2 / 171.1 | 92.7 / 11.7, 168.2 / 12.5 | +3.2% / +2.6% | 6→6 / 6→6 | 151→154 / 130→134 | 0.0% / −2.0% |
| beech G2 1 / 7 | 56.6 / 51.6 | 59.0 / 60.2 | 17.8 / 3.5, 19.6 / 2.7 | +2.6% / +2.6% | 14→13 / 8→8 | 70→75 / 69→68 | −1.1% / −1.6% |

- **Frozen growth is slower on every preset** (1.02 to 1.6x). On the beech s1, extend goes from 23.6 to 29.0 ms.
  - The occupancy pass after each cycle stays sequential.
  - A shoot no longer stops on its own occupancy, so more metamers run the sight check.
- **The trees differ slightly:**
  - nodes +2 to 6%, tips within 2%;
  - the strong-limb count is equal, or 1 lower on three preset-seeds;
  - mid divisions are equal.
  - Order-2 tortuosity changes on oak s7 (1.044 → 1.072) and birch s1 (1.065 → 1.038).
- No frozen still was rendered.

## Stills (pinned pose; both viewed)

- **`beech-probe4-chosen-bare.png`:**
  - The limbs and branches are straight, with a clear trunk and a few upright limbs.
  - The scaffold's limbs end in thick, blunt stubs: their own radii are held as a floor.
  - The 3 mm fine wood is a sub-pixel speckle haze around the limbs.
- **`beech-probe4-chosen-whole.png`:**
  - A dense, nearly opaque green oval. The trunk is visible below the crown, and the foliage hangs in sprays.

## Chosen values

| Setting | Value |
|---|---|
| scaffold `crookedness` | 0 |
| ξ | 0.05 |
| η | 0 |
| tropism | up |
| terminal radius | 3 mm (pipe exponent solved to 1.81, root 0.512 m) |
| short shoots | every 0.02 m on wood under 0.7 of the trunk's radius |

The other canopy rows are the shipped beech's. The scaffold takes G2 values otherwise.
