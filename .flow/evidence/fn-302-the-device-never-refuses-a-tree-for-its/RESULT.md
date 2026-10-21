# fn-302: a stepped-limit overflow rebuilds once at the bound

**Verdict: every frame of the film's walk builds on the device, frame 41 by one retry; every other build is bb20f3877's to the byte.** All runs were on 2026-10-21, at `telperion-heavy 8 --threads 2`. Raw logs are in `raw/`.

## R1: the repro

- On bb20f3877, both tests refuse beech to spruce at 40, seed 1, frame 41 of 90: "the build passed its sizes at cycle 12: HierGens (0x1)" (`raw/r1-repro.log`).
- After task 1, `the_film_walks_build_on_the_device` passes. Frame 41 builds by one retry and matches the CPU exactly: 108,218 instances and 307,728 nodes on both.
- `the_rounds_frames_have_the_cpus_counts` passes once four frames move out (host ruling, 2026-10-21). They go into the ignored `fn303_the_four_rounds_frames_have_the_cpus_counts`, which names fn-303.

| Frame | Instances, device / CPU | Nodes, device / CPU |
|---|--:|--:|
| spruce to birch at 40, seed 7, frame 63 | 187,997 / 187,999 | 382,987 / 382,999 |
| oak to palm at 40, seed 1, frame 36 | 157 / 160 | 20,205 / 20,250 |
| oak to palm at 40, seed 1, frame 37 | within 1 | 19,985 / 20,032 |
| oak to palm at 40, seed 1, frame 48 | within 1 | 4,785 / 4,910 |

The digest lines of these four frames equal bb20f3877's, and none retries. On the base, frame 41's panic, first in the test's list, hid them.

## R2: the bound's cost, and the design it led to

Sizing the hierarchy rounds at their bound costs more than noise, per build (`raw/u1-cost.log`):

| Preset | Cost |
|---|--:|
| beech 40 | +7.8% |
| palm 12 | +12.4% |
| spruce 40 | +1.7% |

That is about 20 to 33 µs per idle round-pass of host recording.

The census survey covered 70,284 censuses (`raw/survey-both.log`, `raw/survey-units.log`):
- gens(c) = deep(c − 1) holds on every census.
- The rise from the census two cycles back is 3 at most, whichever word it is read from.
- The units rounds sit at zero headroom on 812 censuses.

So no projection is a bound, and the host chose one retry at the bound (Design revised).

## R3: output unchanged

- **Preset digests:** `digest` at seeds 1, 7 and 42, and `palm_digest`. All 34 lines equal bb20f3877's (`raw/base-digests.log`, `raw/t2-digests.log`).
- **The film-walk sweep:** 1,533 frames.
  - All 1,532 frames that bb20f3877 built have digest lines equal to the base's.
  - Frame 41, which bb20f3877 refused, now builds.
  - Sources: `raw/t2-survey.log` and `raw/t2-survey-telperion.log` against `raw/base-digests.log`.
- **The 145 budget refusals:**
  - In the first survey run, 145 Laurelin to Telperion frames at 80 were refused by the device budget: seed 7, frames 36 to 90, and every frame at seed 42.
  - The budget follows free VRAM, and other applications on the machine held about 7.3 of the GPU's 10 GiB (FRICTION.md).
  - Rerun on a free GPU, all 270 Laurelin to Telperion frames built, with no retry and equal digests.

## R7 and R8: the stepped limits and the retries

`stepped_survey` covers 1,605 device builds and 69,966 censuses (`raw/t2-survey.log`): the 1,533 walk frames, plus every shipped preset at its default age and at 12 and 40 where younger, at seeds 1, 7 and 42. In the table, rise is a cycle's need over the census two cycles back; headroom is what the cycle recorded less what it needed.

| Stepped limit | Most rise | Least headroom |
|---|--:|--:|
| units ranges | 1 | 0 |
| draw ranges | 1 | 0 |
| deposit ranges | 1 | 0 |
| sweeps | 1 | 2 |
| hierarchy rounds | 3 (beech to spruce, seed 1, frame 1, cycle 16) | −1 (frame 41, seed 1, cycle 12: the retry) |
| placing passes | 1 | 0 |
| living units rounds | 6 (beech to spruce, seed 1, frame 1, cycle 15) | 0 (ordinary at 40, seed 1, cycle 18) |
| wake units rounds | 5 (beech to spruce, seed 1, frame 18, cycle 36) | 0 (beech to spruce, seed 1, frame 34, cycle 8) |

**Retries:**
- One build of 1,605 began again: beech to spruce at 40, seed 1, frame 41.
- No preset build retried, and the 270-frame rerun had no retry.
- A headroom of 0 is a limit used to its last round. The build passes, and one more would retry.

**Frame 41's retry cost:** frame 41 alternated in one process, 7 pairs (`raw/t2-frame41-timing.log`).

| Build | Median | Range |
|---|--:|--:|
| As built, retrying once | 317.7 ms | 315.4 to 320.0 ms |
| Hierarchy rounds pinned at the bound from the first cycle | 273.9 ms | 271.7 to 275.7 ms |

- The retry costs 43.8 ms, the refused first attempt's 12 cycles. Both sides build 108,218 instances.
- Builds that do not retry record exactly what they recorded before, so their cost is unchanged (Design revised: no timing pair).
