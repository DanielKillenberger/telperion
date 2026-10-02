# fn-188 R3, round 6: a leaders-only scaffold, spray rows, the lobed envelope, 2026-10-03

## Setup

- **Code:** round 5's probe (`e04db211`) plus a leaders-only switch and limb counts against the trunk. Committed as `e0ca1ed1` and reverted in `b7528eb7`.
- **Kept from round 5:**
  - leaf-pipe radii (exponent 2.9, root 0.512 m), no scaffold floor;
  - scaffold crookedness 6, ξ 0.2, η 0.1;
  - the beech's shipped canopy rows on G2;
  - the pipeline's own wood and foliage through `executor::expand(tree, family).mesh()`.
- **Runs:** release on the Ryzen 9 5950X, 16 threads.
- **Rows:** `raw/probe6/`: `walk.jsonl`, `walk-limbs.jsonl` and `r6-flat.json`.

## A. Scaffold reduced to the trunk and its leaders

- **What is kept:** only stem wood (`Node::stem`), the trunk and the four leaders of G2's fork (`forkWays` 4). Every scaffold lateral is dropped.
- **Growth:** competition grows every side branch from the buds along the resampled leaders.

**Limb counts.** Round 5's "strong" count compares a lateral with its parent at the junction. Along a thinning leader that counts many laterals, so two counts against the trunk itself (the root radius) are added: lateral axes born at 0.4 or more, and at 0.3 or more, of the root radius. B-BARE reads 4 to 6 limbs at 0.4 to 0.6 of the trunk.

| λ | scaffold | strong (junction ≥ 0.4) | limbs ≥ 0.4 trunk | ≥ 0.3 trunk | o1 / o2 / o3 | mid / end | o3/o1 | tortuosity o0–o3 | projected limbs | nodes | leaves | probe ms | tip |
|---|---|--:|--:|--:|---|---|--:|---|--:|--:|--:|--:|--:|
| 0.45 | leaders | 21 | 3 | 3 | 0.249 / 0.496 / 0.929 | 0.610 / 0.864 | 0.160 | 1.037 / 1.002 / 1.003 / 1.006 | 1.0033 | 38,180 | 1,357,437 | 51.5 | 7.1 mm |
| 0.475 | leaders | 22 | 3 | 3 | 0.245 / 0.534 / 0.907 | 0.582 / 0.866 | 0.161 | 1.036 / 1.002 / 1.003 / 1.007 | 1.0043 | 38,480 | 1,382,184 | 49.5 | 6.9 mm |
| **0.5** | leaders | 22 | 3 | 5 | 0.243 / 0.581 / 0.859 | 0.554 / 0.870 | 0.162 | 1.036 / 1.003 / 1.004 / 1.010 | 1.0035 | 50,695 | 1,383,846 | 61.8 | 6.4 mm |
| 0.525 | leaders | 24 | 3 | 9 | 0.201 / 0.521 / 0.783 | 0.523 / 0.866 | 0.155 | 1.036 / 1.003 / 1.003 / 1.006 | 1.0037 | 36,761 | 1,362,462 | 50.6 | 7.5 mm |
| 0.55 | leaders | 20 | 3 | 4 | 0.259 / 0.492 / 0.738 | 0.502 / 0.865 | 0.144 | 1.036 / 1.002 / 1.003 / 1.005 | 1.0049 | 37,961 | 1,363,045 | 51.1 | 7.2 mm |
| 0.45 to 0.55 | full G2 (round 5) | 24–30 | 7–8 | 13–16 | – | – | – | – | – | 40,015–52,550 | – | – | – |

- **With leaders only,** three limbs reach 0.4 of the trunk at every λ: the fork's other three leaders, so four leaders including the primary. Grown side branches stay under it.
- **Wood at 0.3 of the trunk** varies with λ (3, 3, 5, 9, 4).
- **With the full G2 scaffold,** 7 to 8 limbs reach 0.4 and 13 to 16 reach 0.3.
- **Seed 7 (λ 0.5, leaders):** no limb reaches 0.4 of the trunk and 8 reach 0.3 (48,621 nodes).
- **λ 0.5 grows 32 to 38% more nodes** than its neighbours on both scaffolds: 50,695 against 36,761 to 38,480 with leaders, and 52,550 against 40,015 to 41,398 with G2. This was not traced.
- **Chosen λ 0.5.** I viewed one contact sheet of the five bare thumbnails beside B-BARE's crown. All five show the trunk forking low into a few thick leaders running to the top, inside a fine twig mesh.
  - λ 0.5 has the most even mesh with well-separated leaders.
  - λ 0.55 kinks one leader near the top.

## B. Spray orientation: what exists

| Row | Where | Beech value | What it does |
|---|---|--:|---|
| `canopy.divergence` | `pipeline/foliage/canopy/params.rs:36`; used in `foliage/station.rs:91–95` | **180°** | Turns each successive leaf around a twig shoot. 180° is already two-ranked. |
| short-shoot clusters | `pipeline/foliage/short_shoots.rs:110–126` | – | Fan their `shortShootLeaves` leaves across ± `shortShootSpread` (beech 90°). The fan turns around the vertical from the cluster's level bearing, so it is already in the horizontal plane. |
| `canopy.upward` | `canopy/params.rs:73` | 0.3 | Tilts leaves toward the sky; rosettes ignore it. |
| `canopy.outward` | `canopy/params.rs:66` | 0.0 | Turns leaves away from the trunk. |
| `canopy.forwardLean` | `canopy/params.rs:80` | 0.1 | Leans leaves along the shoot. |
| `canopy.leanRise` | `canopy/params.rs:87` | – | Adds lean on radials that face up. |
| `canopy.scatter` | `canopy/params.rs:107` | 80° | Turns each leaf at random. |
| `twigs.divergence` / `twigs.angle` | `pipeline/twigs.rs:211` / `:181` | – | The twig layer's shoot phyllotaxis and branching angle. The probe grows no twig layer, so they do not apply here. |

- **No pipeline row lays a shoot or a spray toward horizontal** (plagiotropy). The orientation of a probe shoot comes from the probe's own direction rule: previous direction + ξ·markers + η·up.
- **A flat-spray render using only existing rows,** `upward` 0 and `scatter` 20° (`r6-flat.json`):
  - Coverage falls: tree 0.857 → 0.810, leaf 0.779 → 0.715.
  - Width ÷ height is 0.805 → 0.804.
  - The still (`beech-probe6-l0.5-flat-whole.png`, viewed) shows two leader masses with a cleft between them, and foliage as combed streaks along rising shoots.

## C. Envelope

**The probe already filled markers into the lobed envelope.** The fill keeps a draw where `Envelope::contains(p, 0, seed)` holds. That calls `radius_toward` (`envelope.rs:152`), which applies the lobes whenever `irregularity` is not 0 (beech 0.18, `lobeScale` 0.7). Nothing was changed.

## Result at λ 0.5, leaders only, seed 1 (pinned pose)

Coverage and width ÷ height are measured as in rounds 4 and 5.

| | leaves | tree cover | leaf cover | width ÷ height | probe ms | pipeline mesh ms | probe + mesh | shipped `mesh::build` |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| B-WHOLE right / left tree | – | 0.876–0.918 / 0.849 | – | ≥ 0.739 / ≥ 0.655 | – | – | – | – |
| round 5 (c6, full G2) | 1,167,304 | 0.836 | 0.739 | 0.837 | 56.0 | 589 | 645 | 1,341 |
| **λ 0.5, leaders** | 1,383,846 | **0.857** | 0.779 | 0.805 | 61.8 | 618 | **680** | 1,360 |
| λ 0.5, leaders, flat-spray rows | 1,383,846 | 0.810 | 0.715 | 0.804 | 65.7 | 612 | 678 | 1,313 |

- **Probe ms** covers marker fill and growth, warm median of 3, without the scaffold build (2.5 to 3.6 ms).
- **Mesh** is the pipeline's wood sweep and foliage, warm median of 2.
- **Shipped** is the same family's `mesh::build`, warm median of 3.

## Stills

**Pinned pose:**

- `beech-probe6-l0.5-bare.png`
- `beech-probe6-l0.5-whole.png`
- `beech-probe6-l0.5-flat-whole.png`

**Thumbnails:** `beech-probe6-thumb-l<λ>.png` for all five λ.

**Viewed:** the λ contact sheet and the flat-spray whole still.
