# fn-188 R3: the foliage adapter, identities and clumping, 2026-10-03

## Setup

- **Code:** round 7's probe (`2ec2c4b2`) plus this check, committed as `9e4b8892` and reverted in `f0a88967`.
- **Tree:** round 7's chosen tree: beech G2 leaders-only, crookedness 6, λ 0.5, ξ 0.2, η 0.1, p 1, a 50° departure angle, leaf pipes, no floor, seed 1. All four variants have the same digest (`4d2531e5…`, 49,512 nodes), so positions, topology and radii are frozen; only foliage inputs change.
- **Foliage:** the pipeline's own, through `executor::expand(tree, family).mesh()`, with the beech's shipped canopy rows.
- **Rows:** `raw/adapter/adapter.jsonl` and `adapter-clump0.json`.

## 1. Birth identities

**Before:** `to_tree` built every node from `Node::root()`, whose identity is the default (birth `u64::MAX`, null key). Short-shoot placement keys its phase and random stream by birth order (`foliage/short_shoots.rs:180–181`), so every segment drew the same stream.

**Now:**

- Each probe node gets birth = its index in the probe's parent-before-child order, with the key left null.
- The identity's fields are crate-private. It is built through its serde form (telperion-core's `json` feature): serialise the default, set `birth`, deserialise.
- The result is unique per node, deterministic, and stable for a given tree.

## 2–3. Four renders, whole, pinned pose

- **Coverage and width ÷ height:** as in rounds 4 to 6.
- **Repetition:** the share of leaves whose packed rotation word (word 0 of the 12-byte leaf: the quantised quaternion) is also held by another leaf, and the count of distinct rotation words.

| variant | identities | `limbClumping` | leaves | tree / leaf cover | width ÷ height | distinct rotations | leaves with a repeated rotation | mesh ms |
|---|---|--:|--:|---|--:|--:|--:|--:|
| (i) | default | 0.25 | 1,318,843 | 0.850 / 0.765 | 0.817 | 1,174,577 | 20.4% | 588 |
| (ii) | unique | 0.25 | 1,369,427 | 0.865 / 0.794 | 0.816 | 1,366,175 | 0.47% | 630 |
| (iii) | default | 0 | 1,566,009 | 0.879 / 0.807 | 0.818 | 1,369,216 | 23.2% | 588 |
| (iv) | unique | 0 | 1,625,505 | **0.889** / 0.834 | 0.822 | 1,620,909 | 0.56% | 639 |

The B-WHOLE reference: tree cover 0.876 to 0.918 on the right tree, 0.849 on the left; width ÷ height ≥ 0.739 (a lower bound).

- **Unique identities remove the repeated rotations:** 20.4% → 0.47% with clumping 0.25, and 23.2% → 0.56% with clumping 0.
- **They also add leaves:** +3.8% to +3.9%, because each segment draws its own phase. Tree cover rises by 0.010 to 0.015.
- **Clumping 0 adds leaves** (+18.8% on default identities, +18.7% on unique) and tree cover (+0.024 to +0.029).
- **Both together** give 0.889 tree cover, inside B-WHOLE's right-tree range.

**The crown-edge crop** uses the pinned camera position, a quarter of its field of view, and is aimed at the crown's right edge at 0.55 of the height. Both crops were viewed.

- **(i):** leaves stand in regular beaded rows along every branch, with the same spacing pattern on segment after segment, and the branch lines show bare between the rows.
- **(iv):** leaves are denser and irregular along each branch with no repeating row pattern. The foliage still sleeves the branch lines, which remain visible.
- **The whole stills were not viewed.** Whether (iii) and (iv) close the inter-leader cleft is not checked visually.

## 4. Terminal runs and laterals on this tree

- **A terminal run** goes from a grown tip back along its axis to the first node that has more than one child, is a lateral's first node, or is scaffold.
- **Laterals per metre** counts every grown lateral's first node per metre of grown wood.

| measure | value |
|---|---|
| terminal runs | 8,801 |
| run length p10 / p50 / p90 / max | 0.25 / 0.50 / 1.25 / 5.50 m |
| run length in metamers | 1 / 2 / 5 / 22 |
| grown wood | 12,298 m |
| grown laterals | 10,303 |
| laterals per metre of grown wood | 0.84 |

## Stills

- **Pinned pose, whole:**
  - `beech-adapter-i-whole.png`
  - `beech-adapter-ii-whole.png`
  - `beech-adapter-iii-whole.png`
  - `beech-adapter-iv-whole.png`
- **Crops:** `beech-adapter-i-crop.png` and `beech-adapter-iv-crop.png` (both viewed).
