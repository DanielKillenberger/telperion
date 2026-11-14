# fn-350 result: does a kept pipe hold the old oak up

Measured on 11 Nov 2026 on the fix branch (`fn-349-the-corrected-balance-in-the-product`, 210f97006), built in its own checkout. The probe was reverted afterwards; that checkout is clean at 210f97006 and has nothing committed to it. This file gives the measures and a plain read of them. The verdict is the host's, given from the sheets.

**In short:** at `retained` 0.5 and 1 the old oak keeps its stem. The 300-year oak on seed 1001 stands 23 m tall where it was 7.7 m, its stem no longer loops to the ground, and the oak at 200 on seed 1012 is drawn instead of refused. The trunk at 1.3 m no longer falls from 120 to 300 on any seed; it grows. But the trunk is 3 to 5.4 m across at 1.3 m, and the base of the old trees is drawn as a stepped collar (see R3). The setting costs nothing measurable in time or memory.

## R1: how the value reaches the oak

An existing override was enough; no product code changed. The still runner's `--girth retained,chi` flag (`crates/telperion-render/examples/space/tuning.rs`) sets `retained` and `leaf_girth` on every PA. Chi was held at the preset's own value, which is 1 on every PA of both the oak and the beech. `tolerance.rs` sets it on the preset; the 0 in `beech.rs` is overwritten.

- **Check (measured):** at `retained` 0 every tree reproduces fn-349's own run node for node: oak 40/80/120/200/300 on seed 1001, 120/200/300 on seeds 1007 and 1012, and the beech at 40 (57,910 nodes). The oak at 200 on seed 1012 is refused with the same error.
- **Measurement-only probe:** `probes/kept.patch` was applied uncommitted and then reverted with `git apply -R`. It does two things:
  - `still.rs` prints the trunk diameter at 1.3 m and the largest limb's base diameter. The trunk is found by following the thickest child from the root. The largest limb is the thickest other child leaving that line.
  - `early.rs` reads `PROBE_RETAINED`, which is used for the one-thread cost run.
- **Scripts:** `probes/run.sh` holds the runs, `probes/measures.py` builds the table, `probes/sheets.sh` builds the sheets. The raw logs are under the ignored `raw/stills/`.

## R2: the old oak, measured

Diameters are in cm. "Grown s" is the grow time at 8 threads in the drawn run. Peak GiB is the drawn run's peak, rendering included. "Today" is the variant A trunk from fn-345 and fn-346's logs.

| tree | today trunk | retained | height m | spread m | trunk at 1.3 m | largest limb base | nodes | leaves | grown s | peak GiB |
|---|---|---|---|---|---|---|---|---|---|---|
| oak 120 1001 | 142.6 | 0 | 24.7 | 27.4 x 26.0 | 138.1 | 90.6 | 2,435,562 | 367,982 | 20.3 | 3.57 |
| | | 0.5 | 25.8 | 25.6 x 24.2 | 256.8 | 164.7 | 2,439,591 | 379,480 | 20.6 | 3.92 |
| | | 1 | 26.0 | 25.4 x 24.1 | 327.0 | 209.1 | 2,439,640 | 380,132 | 20.5 | 3.92 |
| oak 200 1001 | 128.8 | 0 | 22.8 | 31.1 x 37.9 | 82.3 | 51.4 | 941,162 | 97,398 | 44.6 | 2.30 |
| | | 0.5 | 28.6 | 28.4 x 32.5 | 284.4 | 187.0 | 940,476 | 110,678 | 45.5 | 2.36 |
| | | 1 | 29.3 | 28.5 x 30.8 | 366.0 | 240.8 | 939,303 | 110,689 | 43.9 | 2.35 |
| oak 300 1001 | 120.4 | 0 | 7.7 | 16.1 x 16.6 | 35.9 | 18.2 | 1,041,547 | 65,460 | 66.8 | 2.28 |
| | | 0.5 | 23.3 | 16.1 x 13.4 | 306.2 | 205.6 | 1,040,683 | 66,206 | 68.6 | 2.44 |
| | | 1 | 23.7 | 15.4 x 14.0 | 394.7 | 263.2 | 1,040,145 | 66,206 | 66.6 | 2.47 |
| oak 120 1007 | 149.9 | 0 | 25.1 | 35.7 x 35.0 | 190.5 | 116.4 | 3,456,537 | 607,369 | 37.0 | 5.11 |
| | | 0.5 | 26.5 | 33.0 x 33.7 | 377.6 | 209.9 | 3,466,607 | 625,134 | 37.9 | 5.52 |
| | | 1 | 26.8 | 32.7 x 32.9 | 481.0 | 272.0 | 3,467,525 | 625,648 | 36.9 | 5.57 |
| oak 200 1007 | 225.8 | 0 | 22.9 | 30.1 x 27.6 | 89.5 | 52.4 | 1,434,589 | 154,025 | 80.5 | 3.43 |
| | | 0.5 | 28.4 | 31.6 x 23.3 | 380.7 | 211.9 | 1,440,038 | 170,695 | 80.5 | 3.46 |
| | | 1 | 28.5 | 28.9 x 23.1 | 487.8 | 271.3 | 1,440,435 | 170,709 | 79.7 | 3.46 |
| oak 300 1007 | 106.1 | 0 | 14.9 | 16.3 x 13.1 | 51.6 | 22.2 | 1,495,238 | 108,586 | 108.7 | 3.43 |
| | | 0.5 | 16.1 | 14.8 x 16.4 | 419.8 | 216.1 | 1,493,239 | 113,528 | 109.0 | 3.49 |
| | | 1 | 16.7 | 15.3 x 16.6 | 544.0 | 276.8 | 1,492,353 | 113,540 | 110.3 | 3.48 |
| oak 120 1012 | 176.4 | 0 | 23.5 | 38.1 x 29.7 | 161.0 | 95.5 | 2,638,477 | 442,393 | 26.3 | 3.96 |
| | | 0.5 | 25.4 | 32.9 x 31.2 | 306.9 | 207.3 | 2,646,747 | 460,493 | 26.2 | 4.33 |
| | | 1 | 25.6 | 32.3 x 30.0 | 389.7 | 266.3 | 2,647,928 | 461,445 | 26.2 | 4.34 |
| oak 200 1012 | 115.3 | 0 | refused | | | | | | | |
| | | 0.5 | 30.9 | 36.5 x 27.2 | 358.6 | 240.6 | 1,679,660 | 219,443 | 65.9 | 3.53 |
| | | 1 | 31.9 | 35.6 x 25.3 | 458.2 | 308.3 | 1,682,107 | 219,474 | 66.6 | 3.53 |
| oak 300 1012 | 169.8 | 0 | 16.9 | 19.1 x 18.6 | 59.2 | 35.2 | 1,723,399 | 161,507 | 104.5 | 3.31 |
| | | 0.5 | 20.7 | 18.2 x 14.8 | 386.9 | 185.1 | 1,722,944 | 165,715 | 102.1 | 3.55 |
| | | 1 | 20.9 | 18.1 x 15.5 | 500.5 | 237.1 | 1,722,047 | 165,715 | 103.6 | 3.58 |

The oak at 200 on seed 1012 at `retained` 0 is refused with: `engine = axis 2363 (PA 6, born in cycle 135, base at 0.141 m) reaches 0.005 m below the ground`. No other tree is refused at any value. On every tree the stem line stood 1.3 m up, so each trunk reading is taken at 1.3 m.

**One-thread cost, oak 300 on seed 1001** (`early live`, pinned to one core, as fn-349's `cost.sh` measured it; `raw/cost.log`):

| retained | seconds | peak GiB |
|---|---|---|
| 0 | 120.6 | 1.99 |
| 0.5 | 122.0 | 1.99 |
| 1 | 121.7 | 1.99 |

fn-349 measured 117.9 s and 1.99 GiB for the same build at 0 in its own session. The 1 % spread between values is within run-to-run noise.

**The setting also reaches growth, slightly (measured):** node and leaf counts move at 0.5 and 1, for example +0.3 % nodes and +3.0 % leaves for the oak at 120 on seed 1007. So `retained` is not only read when the tree settles. Why it reaches growth was not traced; my guess is the carbon balance, which charges living wood (fn-346).

## R3: the sheets

These sheets have one column per variant: today, then the fix branch at `retained` 0, 0.5 and 1. Rows are ages. Every tile is labelled with height, spread and the trunk at 1.3 m. The refused tree is a grey tile labelled with its error.

- `kept-oak-1001-bare.jpg`, `kept-oak-1001-whole.jpg`: ages 40, 80, 120, 200 and 300 (R5's young oak included).
- `kept-oak-1007-bare.jpg`, `kept-oak-1007-whole.jpg`, `kept-oak-1012-bare.jpg`, `kept-oak-1012-whole.jpg`: ages 120, 200 and 300.
- `kept-beech-40-1001.jpg`: leafless, then leafed.

Where the stills come from:

| column | source |
|---|---|
| today | fn-345 and fn-346's raw stills |
| retained 0 | fn-349's own stills; this spec's run at 0 grew those trees for measures only, and they matched fn-349 node for node |
| retained 0.5 and 1 | drawn here |

**What I saw in the 1001 leafed sheet and the 1012 leafless sheet (plain description, not a verdict):**

- At 0.5 and 1, the oak at 200 and 300 stands upright with a single thick stem; at 0 it lies along the ground or loops.
- At 200 and 300, the base of the kept-pipe oak is drawn as a wide flared disc with stepped rings above it, like a pot or a collar. At 300 on seed 1012 the lower stem looks like a hollow, broken-open cylinder.
- At 120 the kept-pipe trunk is visibly much thicker than today's.
- The 300-year crown at 0.5 and 1 is a small tuft on a tall, leaning stem.

## R4: what the setting does not reach

**From the code (inferred from reading it, at 210f97006):**

- **What `retained` reaches.** `girth/disused.rs` gives a `Disused` pipe to the nearest kept bearer of every *sheddable* axis: laterals and relays.
  - A shed axis leaves `retained` × its base radius as grown. That radius is computed over its whole subtree as grown (`attachments_with`).
  - A kept axis that is fading leaves `retained` × (1 − its fade) of that radius.
  - If the bearer was shed too, it is skipped (`shed.index[parent] == usize::MAX`). That bearer's own grown radius already contains the child's pipe, and the bearer passes it down to the next kept ancestor.
- **Subtrees let go in `retire.rs` are reached.** A retired root stands for its subtree by its frozen girth in `attachments_past` (`r.frozen_root(i)`, `settle.rs`). The frozen girth is the subtree's radius when it closed: dead wood does not ripen any further (fn-270 R4). The retired root is also shed (its index is `usize::MAX`), so `disused` gives its bearer `retained` × that frozen radius, scaled by its bearers' fades. At `retained` 1 the pipe of a retired subtree stays whole in the girth below it.
- **Tips that die inside a kept axis.** Continuations are not sheddable and never leave a `Disused` pipe. But a continuation is only removed together with a shed or retired ancestor, whose grown radius already carries it. A phytomer whose apex died on a kept axis keeps its own pipe, because its phytomers stay in the structure.
- **The one place the code loses section at `retained` 1: a kept branch that is fading.** The branch's own wood is drawn scaled by its fade f, and its bearer keeps (1 − f) of its section. At exponent 2 the bearer then carries f²r² + (1 − f)r², which is short of r² by f(1 − f)r² while the branch fades. The loss ends when the branch is shed. This is inferred from `disused` and `thicken_given` and was not measured.
- **Not checked:** whether `leaf_girth` moves old wood. It rescales each phytomer's own pipe by its light at the final age against the tree's mean.

**The measurement: the trunk at 1.3 m, from 120 to 200 to 300 (measured):**

| seed | retained 0 | retained 0.5 | retained 1 |
|---|---|---|---|
| 1001 | 138 → 82 → 36 | 257 → 284 → 306 | 327 → 366 → 395 |
| 1007 | 191 → 90 → 52 | 378 → 381 → 420 | 481 → 488 → 544 |
| 1012 | 161 → refused → 59 | 307 → 359 → 387 | 390 → 458 → 501 |

At `retained` 1 the trunk no longer falls on any seed: it grows by 13 to 28 % from 120 to 300. It also grows at 0.5. At 0 it falls to a third or less.

## R5: the younger trees

| tree | today trunk | retained | height m | spread m | trunk at 1.3 m | largest limb base | nodes | leaves |
|---|---|---|---|---|---|---|---|---|
| oak 40 1001 | 55.7 | 0 | 10.6 | 7.6 x 8.0 | 55.8 | 31.3 | 274,826 | 47,532 |
| | | 0.5 | 10.6 | 7.6 x 8.0 | 67.1 | 33.0 | 275,312 | 51,990 |
| | | 1 | 10.6 | 7.6 x 8.0 | 77.5 | 38.3 | 275,370 | 55,200 |
| oak 80 1001 | 110.2 | 0 | 17.5 | 19.1 x 15.0 | 115.6 | 74.6 | 1,414,069 | 227,955 |
| | | 0.5 | 17.6 | 18.8 x 14.9 | 173.0 | 105.3 | 1,416,784 | 243,311 |
| | | 1 | 17.7 | 18.8 x 14.9 | 213.2 | 128.2 | 1,417,666 | 247,204 |
| beech 40 1001 | 25.6 | 0 | 9.8 | 5.1 x 7.0 | 25.6 | 14.3 | 57,910 | 16,614 |
| | | 0.5 | 9.8 | 5.1 x 7.0 | 31.8 | 16.6 | 58,127 | 16,990 |
| | | 1 | 9.8 | 5.1 x 7.0 | 37.0 | 18.8 | 58,223 | 16,984 |

The young trees keep their height and spread, and their shape on the sheets looks the same. Only the trunk thickens:

| tree | trunk at `retained` 1 against 0 |
|---|---|
| oak at 40 | +39 % |
| oak at 80 | +84 % |
| beech at 40 | +45 % |

On the leafed sheet the oak at 80 shows a flared base at 0.5 and 1.

## A plain read per tree

| tree | read |
|---|---|
| Oak 300, 1001 | Keeping the pipe turns the looped, 7.7 m stem into a 23 m upright tree. Its trunk is 3.9 m across at `retained` 1, against 1.2 m today. |
| Oak 200, 1012 | Refused at 0. Drawn at 0.5 and 1, at 31 to 32 m. |
| Oak 300, 1007 and 1012 | Upright at 0.5 and 1, but 16 to 21 m against today's 32 and 48 m. The trunks are 4.2 to 5.4 m across. |
| Oak 120 on all seeds | Close to today in height and spread. At `retained` 1 the trunk is 2.2 to 3.2 times today's. |
| Young oak and beech | The same trees with thicker trunks. |

## Friction

One entry, in `FRICTION.md`: the `--girth` flag sets two values at once. Because of it, the first beech run held chi at 0; the beech was re-run with chi 1.
