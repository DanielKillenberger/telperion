# fn-188 R3, round 7: plagiotropy as one parameter, the departure angle, 2026-10-03

## Setup

- **Code:** round 6's probe (`e0ca1ed1`), changed, committed as `2ec2c4b2` and reverted in `9e768dd8`.
- **Kept from round 6:**
  - the leaders-only G2 scaffold with crookedness 6;
  - λ 0.5, ξ 0.2, η 0.1;
  - leaf pipes (exponent 2.9, root 0.512 m), no floor;
  - the shipped canopy rows;
  - the pipeline's own wood and foliage via `executor::expand(..).mesh()`.
- **Runs:** release on the Ryzen 9 5950X, 16 threads.
- **Rows:** `raw/probe7/`: `angle.jsonl`, `walk.jsonl` and `timing.jsonl`.

## 1. Plagiotropy p and the departure angle

**Plagiotropy, one continuous parameter p ∈ [0, 1]:**

- Each node carries its branching depth from the scaffold. Scaffold wood is depth 0, a lateral adds 1, and a continuation keeps its parent's depth.
- Each node also carries its shoot's horizontal outward direction h. At a lateral's first metamer, h is the bud direction with its component along the parent axis removed, projected to horizontal and normalised; continuations inherit it.
- **Tropism:** T = normalise((1 − w)·up + w·h), with w = p · (1 − 0.5^depth).
- **Growth step:** new direction = normalise(previous + ξ·markers + η·T).
- **p = 0 reproduces round 6 byte for byte:** digest `b396983f…` with 50,695 nodes at seed 1, and `d8738a49…` with 48,621 nodes at seed 7.

**The departure angle a new lateral bud takes:**

- It is the preset's `twigs.angle` (beech **40°**, `european-beech.values`), read by `params()` and used in `grow::bud_dir`.
- `bud_dir` turns the parent direction by that angle about a perpendicular axis, at a golden-angle bearing.
- It is now overridable (`P_ANGLE`).

**Angle to vertical by order:** the median over lateral axes of the angle between the axis chord (base to tip, 0.5 m or more) and +y.

| angle | p | angle to vertical o1 / o2 / o3 / o4 | tortuosity o0–o3 | limbs ≥ 0.4 trunk | nodes | leaves |
|---|---|---|---|--:|--:|--:|
| 40° | 0 | 40.4 / 51.1 / 65.3 / 85.5 | 1.036 / 1.003 / 1.004 / 1.010 | 3 | 50,695 | 1,383,846 |
| 40° | 0.5 | 42.5 / 55.5 / 70.3 / 90.6 | 1.036 / 1.003 / 1.004 / 1.007 | 3 | 50,073 | 1,360,977 |
| 50° | 0 | 45.2 / 58.2 / 71.2 / 89.1 | 1.036 / 1.004 / 1.006 / 1.011 | 3 | 49,677 | 1,334,543 |
| 50° | 0.5 | 48.9 / 63.6 / 76.4 / 91.6 | 1.036 / 1.003 / 1.005 / 1.007 | 3 | 49,698 | 1,341,509 |
| 60° | 0 | 52.3 / 65.1 / 77.3 / 90.2 | 1.036 / 1.005 / 1.010 / 1.012 | 3 | 50,025 | 1,348,344 |
| 60° | 0.5 | 55.4 / 68.8 / 82.0 / 93.2 | 1.036 / 1.005 / 1.009 / 1.007 | 3 | 50,036 | 1,348,808 |

**Chosen 50°.** At p 0.5 its first-order median, 48.9°, is nearer B-BARE's two traced side limbs than 60° gives (55.4°).

- Those limbs come from round 4's hand traces of `fasy896.jpg`, base to last point, projected:
  - far left (123,233) → (27,123), 41°;
  - far right (147,240) → (233,117), 35°.
- The photo's are projected angles and the probe's are 3D chords, so the comparison is rough.
- Order 1 here mixes the fork's three leaders with grown laterals off the primary leader.

## 2. The p walk at 50°, seed 1 (pinned pose)

| p | angle to vertical o1 / o2 / o3 / o4 | tortuosity o0 / o1 / o2 / o3 | limbs ≥ 0.4 / ≥ 0.3 trunk | nodes | leaves | tree / leaf cover | width ÷ height |
|---|---|---|---|--:|--:|---|--:|
| 0 | 45.2 / 58.2 / 71.2 / 89.1 | 1.036 / 1.004 / 1.006 / 1.011 | 3 / 3 | 49,677 | 1,334,543 | 0.850 / 0.764 | 0.806 |
| 0.25 | 46.2 / 58.6 / 72.8 / 89.7 | 1.036 / 1.004 / 1.005 / 1.010 | 3 / 4 | 49,382 | 1,335,339 | 0.855 / 0.773 | 0.820 |
| 0.5 | 48.9 / 63.6 / 76.4 / 91.6 | 1.036 / 1.003 / 1.005 / 1.007 | 3 / 3 | 49,698 | 1,341,509 | 0.858 / 0.775 | 0.821 |
| 0.75 | 51.2 / 66.6 / 83.5 / 95.6 | 1.036 / 1.004 / 1.005 / 1.005 | 3 / 3 | 49,524 | 1,327,016 | 0.854 / 0.766 | 0.816 |
| **1** | 55.4 / 74.2 / 88.3 / 97.6 | 1.036 / 1.005 / 1.005 / 1.005 | 3 / 3 | 49,512 | 1,318,843 | 0.850 / 0.765 | 0.817 |

- **p moves the angles by degree:** order 2 from 58.2 to 74.2° from vertical, order 3 from 71.2 to 88.3°. Orders 4 and up pass horizontal at p ≥ 0.5.
- **The rest barely moves:** leader count (3 limbs at 0.4 of the trunk, so 4 leaders), nodes (49.4k to 49.7k), leaves and coverage (0.850 to 0.858) are nearly constant.
- **Width ÷ height:** 0.806 to 0.821; B-WHOLE ≥ 0.739 (right tree) and ≥ 0.655 (left tree), both lower bounds.

## 3. Thumbnails and the chosen p

**Two contact sheets were viewed**, one bare beside B-BARE's crown and one whole beside B-WHOLE's right tree, each with all five p.

- **Bare:** higher p spreads the fine wood toward horizontal at the crown's edge; p 1 most, p 0.75 next. The four leaders and their low fork read the same at every p.
- **Whole:** all five show the same texture, vertical combed streaks in two leader masses with a central cleft. p changes it little. None reads as B-WHOLE's broad, lumpy, layered masses.
- **Picked p 1 and p 0.75,** judged on the bare sheet: fine wood spreading toward horizontal at the edge, as B-BARE's side branches do. The whole sheet did not separate them.

## 4. Timing, p 1 at 50°

- **Probe:** marker fill and growth, warm median of 3 (cold in brackets).
- **Mesh:** `executor::expand(..).mesh()`, warm median of 2.
- **Shipped:** `mesh::build` of the same family, warm median of 3.
- **Not included:** the scaffold build, about 3 ms.

| seed | nodes | leaves | probe ms | pipeline mesh ms | probe + mesh | shipped `mesh::build` | limbs ≥ 0.4 trunk |
|---|--:|--:|---|---|--:|--:|--:|
| 1 | 49,512 | 1,318,843 | 61.2 (68.1) | 595 (626) | 656 | 1,358 | 3 |
| 7 | 47,862 | 1,712,723 | 66.0 (80.5) | 666 (667) | 732 | 1,366 | 0 |

At seed 7 no limb reaches 0.4 of the trunk, as in round 6.

## Stills

- **Full, pinned pose:**
  - `beech-probe7-p1-bare.png`
  - `beech-probe7-p1-whole.png`
  - `beech-probe7-p0.75-bare.png`
  - `beech-probe7-p0.75-whole.png`
- **Thumbnails:** `beech-probe7-thumb-p<p>-<bare|whole>.png` for p 0, 0.25, 0.5, 0.75 and 1.
- **Viewed:** the two contact sheets only.
