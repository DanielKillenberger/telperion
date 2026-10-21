# fn-303: the rounds frames' counts on the release branch

All runs were on 2026-10-21, at `telperion-heavy 8 --threads 2`, with `--profile ci`. Raw logs are in `raw/`. The ignored probes are in `crates/telperion-render/src/generation/cycle/fn303_probe_tests.rs`, and they leave the tree before the first commit.

## R1: the bisect

| Commit | spruce to birch s7 f63 | oak to palm s1 f36 | f37 | f48 |
|---|--|--|--|--|
| 55c19adef (fn-298 tip) | 388,725 / 388,725 nodes | 3,789 / 3,789 | 5,793 / 5,793 | 948 / 948 |
| c3ed89f12 (+ fn-283, fn-299) | 382,987 / 382,999 | 20,205 / 20,250 | 19,985 / 20,032 | 4,785 / 4,910 |
| bb20f3877 (+ fn-301) | as c3ed89f12 | as c3ed89f12 | as c3ed89f12 | as c3ed89f12 |
| 6198fbc9a (+ fn-302) | as c3ed89f12 | as c3ed89f12 | as c3ed89f12 | as c3ed89f12 |

Counts are device / CPU (`raw/r1-<sha>.log`).

- **c3ed89f12 parts the counts.** It brings fn-283 (d600fa12c, a frond draws its rachis) as well as fn-299. fn-298's tip has neither. fn-299's blend changed the frames themselves: oak to palm frame 36 grew from 3,789 nodes to 20,250.
- **Growth never parts.** On all four frames, the device census and `grow_census` hold the same axes and phytomers grown at every cycle from 1 to 40 (`raw/r1-probe.log`). Matching the grown state at each cycle by lineage leaves no axis unmatched.

## Oak to palm 36, 37 and 48: the rachis tail (host ruling 1)

On each frame, the CPU's node count less its hung rachis nodes equals the device's count exactly (`raw/r1-nodes-1.log` to `raw/r1-nodes-3.log`).

| Frame | CPU nodes | Rachis nodes | CPU less rachis | Device |
|---|--:|--:|--:|--:|
| 36 | 20,250 | 45 | 20,205 | 20,205 |
| 37 | 20,032 | 47 | 19,985 | 19,985 |
| 48 | 4,910 | 125 | 4,785 | 4,785 |

- The CPU hangs the rachis chains after the radius solve (`leaf_bases.rs:195,248`, `rachis.rs:93`) and marks where they begin with `tree.clothing`.
- The device draws the rachis in another pass (`rachis.wgsl`).

## For fn-300: spruce to birch at 40, seed 7, frame 63

- **The lineage.** The CPU draws a 10-node twig the device lacks, on axis lineage `0xe7a34cf416e9d0b0`: radius 0.00022 m, phytomers of cycles 14 to 18, CPU nodes 2295 to 2304. The rest of the gap is single-node swaps on other axes, 12 CPU-only nodes against 8 device-only.
  - Through all 40 cycles the axis holds no discrete difference.
  - From cycle 20 on, one of its units survives 0.5301237 on the device against 0.53016186 on the CPU, and its phytomers' scale (0.4574817 against 0.4575146) and grown share follow (`raw/r1-watch-sb.log`).
  - Shade light and positions differ by about 1 ulp from cycle 1.
- **The `FINEST` link.** The conversion drops wood thinner than `FINEST` = 2e-4 m, and all it bears (`telperion-core/src/pipeline/space.rs:48,424`; the device's `dress.wgsl:122,490`).
  - The twig's first node is 0.00022 m on the CPU.
  - The largest radius gap on matched nodes is 2.4e-5 m.
  - The device's radius for that first phytomer was not read, so the threshold crossing is inferred.
  - The single-node swaps are the `SHORTEST` merge (`space.rs:49,470`; `dress.wgsl:621`) falling on another phytomer.
- **The `below` word lead, oak to palm frame 48.** At cycle 29, axis lineage `0xcaa177247cb8e8a2` holds `below` = 768 on the device against 3,712 on the CPU, with every other word equal (`raw/r1-probe-f48.log`).
  - `below` is a failed unit's relay's rest below a tick, in 2^-32 of a cycle (`decide::lived_rest_at`; the device's `advance.wgsl:282`).
  - At cycle 37 a second axis parts by 5 ticks in `spent`.
- **The 2 leaves on shared wood (not traced).** The frame's instances are 187,997 on the device against 187,999 on the CPU. The 2 CPU-only leaves stand on wood both executors draw (`raw/r1-leaves-0.log`):
  - CPU leaf 62549 stands 0.0049 m from node 152053 (identity `0xf5083c193ec61be7`).
  - CPU leaf 145787 stands 0.0003 m from node 357950 (identity `0xc477c151561637c4`).
  - Within 6 cm of the dropped twig, each side holds 2 leaves.
- **The whole frame goes to fn-300 (host, 2026-10-21):** nodes and instances, under one ignored test whose reason names fn-300.

## Oak to palm frame 36's 3 leaves: a relay join's t at its parent's tip (host, 2026-10-21: traced here)

- **Which placement.** They are no rosette frond.
  - The rosette places 35 leaves on both executors: 13 fronds, with equal slots, `made` and year (`raw/r1-fronds-cpu.log`).
  - The 3 are one compound leaf on a wood run.
  - Per run, the device's segments and the CPU's kept stations agree on every run but one, key 1828110802 (`raw/r1-runs-cpu.log`, `raw/r1-runs-dev.log`).
- **The run.** CPU node 9545 is a twig tip, r 0.0002177 m, 1.554 mm of grown wood. Its run's cauline draw is keyed by that node's key (`telperion-core/src/pipeline/foliage/plan.rs:453-455`) and keeps one station of 3 leaflets.
  - The device draws a node at the same place, with radius 0.00021769898 against the CPU's 0.0002177 and the same parent key 3513484491.
  - Its identity and key differ:
    - Device: phytomer 59 of engine axis 393, key 1186578072.
    - CPU: the join of relay axis 978 (`join_key`, identity 0x6d10a25704c758d8, key 1828110802).
  - The device's run draws under the other key, and it keeps no leaves (`raw/r1-key-1828110802.log`).
- **The inputs.** Relay 978 joins axis 393 at node 59 with `above` 0.00019943 m, under `FINEST` = 2e-4 m, on both executors. The join's t differs (`raw/r1-twig-join.log`):

  | Relay | CPU t | Device t |
  |---|--:|--:|
  | 978 on 393 | 0.9999992 (0x3f7ffff3) | 1.0 (0x3f800000) |
  | 393 on 100 | 0.9999978 | 1.0 |
  | 100 on 27 | 0.99999964 | 1.0 |

  - The relays' blends agree to 1 f32 ulp (0.7036435604 on the CPU against 0.7036435008 on the device).
- **Where the paths part.**
  - **The join's t.** The CPU's `attach` (`telperion-space/src/geometry.rs:283-296`) blends the stand toward the tip by scaled length. `stand::node_x` (`stand.rs:166-176`) then returns the last node with t = (length − before) / scale = 1 − 8e-7.
  - The device's `attach` (`lay_stand.wgsl:267-281`) has the same shape, and `attach_main` (`settle_thick.wgsl:59-67`) writes t = 1.0.
    - Inferred, not read: its length reaches past the summed tip, so the loop falls through to (n, 0) and `settle_thick.wgsl:62-66` sets t to 1.
  - **The draw.** On the CPU, 0 < t < `join::ONE` makes the join inside (`space.rs:436`). The own draw is capped at `above` and drops under `FINEST` (`space.rs:440-443`), so the node is the join's draw, keyed by the join.
  - On the device, `next_join` skips a join `at_end` (`dress.wgsl:251-253,709`), so the own phytomer's node is drawn under its own key (`dress.wgsl:789-797`).
- **Proposed cause (for the host; no fix planned).** For a relay standing at its parent's node, the join's t is 1 by geometry. Each executor then lands on one side of 1 by last-bit rounding of a summed f32 length. Inside-or-at-end is decided by `t < 1`, so that rounding chooses the node's identity, which keys the run's cauline draw.
- **Frames 37 and 48.** Every run's kept leaves agree once the rosette's fronds are set aside (37: 394 + 38 = 432; 48: 11 + 111 = 122). They show no gap of this kind below "within 1". Frame 37's device does draw a join node (key 1828110802) that frame 36's does not.

## The instance gaps (host ruling 3): neither is the cut nor the twig

- **Oak to palm frame 36, 157 against 160.**
  - All 157 device leaves have a CPU leaf within 2 cm.
  - The CPU has 3 large leaves with none: CPU leaves 3, 4 and 5, of scales 0.43, 1.27 and 1.24.
  - They stand 0.16 to 0.39 m from the nearest wood and 0.35 to 1.34 m from the clothing tail, so they are not on the rachis (`raw/r1-leaves-1.log`).
- **Spruce to birch frame 63, 187,997 against 187,999.**
  - Within 6 cm of the dropped twig, each side holds 2 leaves, so the twig's leaves are not the gap.
  - The 2 CPU-only leaves (62549 and 145787) stand 0.0049 m and 0.0003 m from nodes the device also draws (`raw/r1-leaves-0.log`).

## The premise check for the end-join rule (host, 2026-10-22): it fails

The CPU's relay joins were surveyed from `telperion_space::grow`'s structure (`Axis::joined`): every shipped preset at its default age, and at 12 and 40 where younger, at seeds 1, 7 and 42 (`raw/r1-relay-survey-presets.log`).

| Relay joins | t < 1 − 1e-4 | Joined short of the parent's last node | Least t |
|--:|--:|--:|--:|
| 17,962 | 15,167 | 12,727 | 0.00037 (telperion at 82, seed 1, axis 5539) |

- Relays genuinely join inside their parent. On the ordinary at 40, seed 1:
  - axis 9478 joins at t 0.039;
  - axis 4605 joins at node 4 of 5, t 0.24;
  - axis 6370 joins at node 0 of 4, t 0.87.
- The parent PA's `relay_at` stand, blended by the relay's `blend` (`geometry.rs:283-296`), puts the join anywhere along the parent's last unit.
- 2,795 joins sit at or above 1 − 1e-4. Frame 36's relays (t 0.9999992, 0.9999978 and 0.99999964) are among them.
- The sweep frames were not surveyed. The presets alone refute "a relay continues its parent from the parent's apex", so the rule was not applied.

## Task 1: a span's share is 1 where the length reaches its end (2026-10-22)

`stand::span_share` (CPU) and `span_share` in `lay_stand.wgsl` (device) return 1 where the length reaches the span's summed end, decided by the comparison that chose the span. Otherwise they return the division's share.

- **Unit tests.**
  - CPU: `a_length_at_a_span_end_is_the_whole_span` (`raw/t1-unit-cpu.log`).
  - Device: `a_length_at_a_span_end_is_the_whole_span_on_the_device` (`raw/t1-unit-device.log`).
  - Both pass on scales [0.3, 0.3, 0.044714], whose last span's division reads the tip as 0.9999994.
- **The four frames** (`raw/t1-counts.log`). Device / CPU:

  | Frame | Instances | Nodes | Matched |
  |---|--:|--:|--:|
  | oak to palm 36 | 157 / 157 (was 157 / 160) | 20,205 / 20,250 | 20,205 |
  | oak to palm 37 | 432 / 432 | 19,985 / 20,032 | 19,985 |
  | oak to palm 48 | 122 / 122 | 4,785 / 4,910 | 4,784 |
  | spruce to birch 63 | 187,997 / 187,999 | 382,987 / 382,999 | — |

  - The node gaps on 36, 37 and 48 are the rachis tail (task 2, ruling 1).
  - Frame 36's CPU now draws relay 978's twig tip as the phytomer's own node, as the device does. The run keyed by that node draws no leaves (`raw/t1-leaves-36.log`: no leaf without a counterpart within 2 cm).
- **The growth census** (`raw/t1-census.log`). On all four frames, the device census and `grow_census` hold equal axes and phytomers grown at every cycle from 1 to 40.
- **The relay survey** over the presets at their R1 ages and seeds 1, 7 and 42, 17,962 joins (`raw/t1-relay-survey-base.log`, `raw/t1-relay-survey.log`):

  | Rule | t == 1 | t in [1 − 1e-4, 1) | t < 1 − 1e-4 |
  |---|--:|--:|--:|
  | Before (the division) | 1,490 | 1,305 | 15,167 |
  | After (`span_share`) | 2,771 | 24 | 15,167 |

  - 1,281 joins whose length reached their span's end now read exactly 1.
  - The 24 left in [1 − 1e-4, 1) stand short of their span's end, since `span_share` returns 1 whenever the length reaches it.
- **The gate** (`raw/t1-gate.log`, `telperion-heavy 12 --threads 2`, as AGENTS.md declares it): 1,474 passed, 1 failed, 117 ignored.
  - The failure is `walk_sizes`' named exception oak to birch at 80, seed 7, t 0.01. Its kept phytomers rose from the recorded 3,961,379 to 3,961,473 (+94, 0.002%), because the span share moves relay bases and with them growth. Its grown count stays inside its record.
  - The row is re-measured in `crates/telperion-core/tests/walk_sizes.rs` (host ruling 2026-10-22: outputs change where the CPU landed at 1 − δ, measured), as fn-299 re-measured it. `walk_sizes` passes on the re-measured row (`raw/t1-walk-sizes.log`).
