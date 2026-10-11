# R1: where the birch's two executors part

Measured on F4 at `d76d14b9` with fn-254 merged, on the RTX 3080 under the GPU lock. The test is `birch_tests::the_birch_grows_the_same_wood_on_the_device_as_on_the_cpu` (render crate), run with `cargo test --profile ci`. It grows the silver birch at 30, seed 42, its overrides applied, stage by stage. The bounds are the other GPU presets' bounds from `preset_agree_tests::BOUND`: tips at most 1.75e-3 m and 3.5e-5 m RMS, radii 2e-6 m.

## Stage by stage, on F4 (red)

| Stage | How it is read | Measured |
|---|---|---|
| Growth | every root's axes by lineage, before and after shedding; presences and livings | axes exact; presences 6.68e-5 apart (bound 3e-4) |
| Girth | base and phytomer radii, matched by lineage | 3.1e-8 m |
| Lay | the birch with its sag set to none, laid once | tips 2.15e-4 m most, 4.9e-6 m RMS: within the bounds |
| Sag | the birch as it ships | tips 0.117 m most, 9.3e-3 m RMS: **apart** |

The structure above the cut, which the CPU settles on both runs, agrees to 3.0e-6 m. The roots' loads that bend it therefore agree as well.

**First stage apart: sag.** It is the roots' second lay under their levers (phase 4, `bends`).

**First lineage keys apart.** The shallowest axis apart past 1.75e-3 m is root 333's first axis. Its lineage is `17581127258315254970` and its PA is 9 (`BRANCH`, the hanging long shoot). It parts at phytomer 0, key `15222163334308185898`, by 2.0e-3 m. Nothing above it in its root differs, so the gap starts in that phytomer's own turn.

## The cause, checked against the code

To find it, the inputs to that phytomer's sag were read out on both executors. A temporary dump went into `geometry/sagged.rs`, and a temporary write of the device's private values went into the laid buffer. Both were reverted.

| Input | CPU | Device |
|---|---|---|
| lever (moment) | (8.179e-6, 1.198e-5, -6.301e-6) | (8.179e-6, 1.198e-5, -6.301e-6) |
| chord | (0.5349, 0.7727, -1.1552) | (0.5349, 0.7727, -1.1552) |
| running before the sag | (0.30731, 0.44849, 0.83930) | (0.30731, 0.44849, 0.83930) |
| ground held, support | 1, 1 | 1, 1 |
| turn | 0.2405 rad | 0.1599 rad |

Every input agrees, and the turns differ by a factor of 1.50. The two turn formulas are the same: `sag::turn` in logs and turns, `sag_turn` in `lay.wgsl` in radians. Evaluated in f64 on the CPU's inputs, the formula gives the CPU's 0.2405.

The difference is the length the turn is integrated over:

- **CPU:** `geometry/sagged.rs:22` takes `length = state.internode * phytomer.scale`. This is the internode before its node's own draw.
- **Laying:** `Layer::step` (`geometry/lay.rs:196`) lays the same phytomer at `state.internode * scale * internode(key, internode_spread)` (fn-222).
- **Device:** `step` in `lay.wgsl` sags over that varied `len`. This phytomer's factor is 0.647 (its laid float 23), and the device's turn is close to 0.647 of the CPU's.

The CPU's lever gather (`sag::gather`) also weighs each phytomer by its laid length, `|tip - base|`. Within the CPU, only the sag's turn and its landing support read the unvaried length.

The birch is the only shipped preset whose internodes vary (`internode_spread` 0.2 to 0.7). Every other preset's factor is one, so on those presets the two lengths are equal and the executors agree. `varied_forms_settle_on_the_gpu_as_on_the_cpu` varies the beech's internodes, but the beech does not sag, so that test never reached this line.

**The wrong executor is the CPU.** Its sag turns a phytomer over a length other than the one it is laid at. The device turns it over its own length. The fix belongs in the shared geometry (`geometry/sagged.rs`), which every CPU path lays through. It is neither the light lattice nor the stepped executor.
