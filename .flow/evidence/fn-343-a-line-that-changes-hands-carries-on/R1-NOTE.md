# fn-343 R1: why the bearer's tip stops, what takes over, and why the relay's geometry does not reach it

Drafted by the worker on 10 Nov 2026 at `08b7d7f1a`, for the host. Today's oak, unmodified. No code is built for R2. This note describes what was found; it proposes no design.

Marks: **tagged** (the probe recorded it in the grower at the moment it happened), **measured** (a probe run, log named), **read** (from the code, not run), **inferred** (arithmetic on measured numbers, not run).

## How it was measured

- **The probe** is `probes/stops.patch`: fn-342's `turns.patch` plus stop tags. It is reverted from the crates (`git status --short crates` is empty).
  - **Stop tags.** Each place in the grower that sets `apex_end` records why, keyed by the axis's lineage, PA and birth. A relay shares its parent's lineage, so the key includes the PA and the birth. The tags are:
    - `fail_relayed:*` and `fail:*`: the survival draw failed (`grow/advance.rs`), with or without a relay draw. **`:shade`** means the draw would have passed at full light (`state.viability` unshaded) and failed only because shade lowered the survival (`shade_hazard`). **`:base`** means it would have failed at full light too.
    - `abort`: the abortion draw (`advance.rs`, `persists`).
    - `moved_on`: the apex spent its PA's lifespan and moved on to its next age as a `Continuation` axis (`grow/stop.rs` `move_on`).
    - `ended:no_successor` and `ended:not_moved`: the lifespan was spent and the apex did not move on.
    - `shed`: the balance walk shed it (`grow/balance.rs`, `grow/cycle/passes.rs`).
    - `slept`: an axis of no length (`grow/wake.rs`).
  - **Joint walk.** At each take-over joint, the fn-342 joint walk looks up the bearer's tag at its `apex_end` cycle and the lateral's place on the bearer. A take-over joint is a `fork` or `lateral_at_end` that turns 10° or more.
- **Runs.** `probes/build.sh` and `probes/runs.sh` ran through `telperion-heavy 8 --threads 4`. They are the oak at 40 and 120 on seeds 1001 and 1007, every lateral 1 cm or more at its base (`TURN_RMIN=0.01`). The logs are `raw/t-40-r0.01.log` and `raw/t-120-r0.01.log`.
  - **Same trees as fn-342.** Heights, axis counts and phytomer counts match fn-342's logs.
  - **Same joints.** At 40 the walk finds fn-342's 1,382 take-over joints, 1,025 of them with the bearer's `apex_end` set.
  - **Every stop tagged.** No stopped bearer lacks a tag. A few bearers carry more than one record (8 at 40, 126 at 120), and the record at the `apex_end` cycle is the one used.
- **Reducers.** `probes/stops.py`, `probes/succ.py` and `probes/project.py`. Their outputs are in `raw/stops.md`, `raw/succ.md` and `raw/project.md`.

**Correction to fn-342.** `apex_end` is set when an apex dies, and also when it **changes PA** (`structure.rs`: "died, ended, or changed PA"). fn-342 said that 1,025 of 1,382 forks "stand where the bearer's growing tip had stopped". That count includes tips that moved on to their next age, and that is the largest single cause below.

## 1. What stopped the bearer's tip (tagged)

**Take-over joints whose bearer is 1 cm or more at the joint**, both seeds. "living" means the bearer's apex had not stopped:

| cause | 40 (451 joints) | 120 (3,662 joints) |
|---|---|---|
| moved on: lifespan spent, line carried on as the next age | 0.32 | 0.52 |
| survival failed, relay made (`fail_relayed`) | 0.17 (shade 0.07, base 0.10) | 0.24 (shade 0.16, base 0.08) |
| bearer living, no stop | 0.35 | 0.12 |
| abortion | 0.09 | 0.02 |
| lifespan spent, no successor (`ended:no_successor`) | 0.04 | 0.09 |
| survival failed, no relay | 0.02 | 0.01 |
| shed by the balance | 0 | 0.005 |

**By the bearer's rank**, on wood 1 cm or more:

| bearer | 40 | 120 |
|---|---|---|
| branch (PA 9) | 343 joints: moved on 0.39, living 0.38, failed and relayed 0.23 (base 0.13, shade 0.10) | 3,138 joints: moved on 0.61, failed and relayed 0.27 (shade 0.18, base 0.09), living 0.11, shed 0.01 |
| bough (PA 7) | 55 joints: abortion 0.53, living 0.47 | 136 joints: living 0.44, abortion 0.41, ended 0.12, failed 0.02 |
| twig (PA 13) | 24 joints: ended 0.83, failed 0.16 | 364 joints: ended 0.88, failed 0.12 |
| sprig (PA 8) | 16 joints: moved on 0.50, failed 0.50 | none |
| limb (PA 6) | 13 joints: abortion 0.85, living 0.15 | 22 joints: abortion 0.86, living 0.14 |

**By wood thickness** (bearer radius at the joint, all 1,382 and 8,742 take-over joints):

| bearer radius | 40: n | 40: main causes | 120: n | 120: main causes |
|---|---|---|---|---|
| under 1 cm | 931 | ended 0.44, living 0.21, moved on 0.17, failed 0.17 | 5,080 | ended 0.60, moved on 0.16, living 0.11, failed 0.11 |
| 1 to 2.5 cm | 279 | living 0.44, moved on 0.22, failed 0.21 | 2,255 | moved on 0.45, failed 0.25, ended 0.14, living 0.13 |
| 2.5 to 5 cm | 110 | moved on 0.39, living 0.29, failed 0.19, abortion 0.13 | 864 | moved on 0.61, failed 0.24, living 0.11 |
| 5 cm and more | 62 | moved on 0.61, failed 0.15, abortion 0.18 | 543 | moved on 0.68, failed 0.21, living 0.06 |

**What this answers.** The host asked why branch tips stop so often when the branch row has `abortion` 0.
- **The branch tip mostly does not stop.** Its lifespan is 12 years (`oak.rs` branch `state(12, 1.0, ...)`), after which it moves on with `continuation` 1.0 to the shoot age (PA 12).
- **The shoot age is weak.** It lives 4 years, has viability 0.93, pipe 0.0009 and no relay, and bears short shoots only. The branch's line therefore carries on thin.
- **Real deaths come second.** These are survival failures at the branch's `viability` 0.98 a year, shaded by `shade_hazard` 1.0. They are 23 to 27% of the branch take-overs on thick wood, and every one made a relay (`relay_failed` 1.0).
- **Twigs end at 5 years with nothing to carry them on.** They have `continuation` 0 and `relay_ended` 0. That is the main cause on wood under 1 cm.
- **The balance plays no part:** shed is 0 to 1%.

## 2. Which lateral took the line, and by what rule (tagged and measured)

**The rule is girth, not a grower event.**
- The walk follows the thickest wood, and a lateral "takes the line" only because the pipe model (`girth.rs`: a section is the sum of the sections it carries) makes it thicker than the bearer's own wood past the node.
- **No grower decision names a lateral as the line (read).** The grower carries a line on only through a `Relay` or a `Continuation` axis.

**What the bearer's own successor is**, on take-overs with the bearer 1 cm or more (measured, `raw/succ.md`):

| bearer's stop | 40 | 120 | its successor's base radius over the lateral's |
|---|---|---|---|
| moved on, continuation (shoot) with wood | 109 | 1,497 | median 0.14 (40), 0.13 (120) |
| moved on, continuation without wood | 34 | 415 | none |
| failed or aborted, relay with wood | 104 | 619 | median 0.52 (40), 0.43 (120) |
| failed or aborted, no successor with wood | 26 | 350 | none |
| living | 158 | 423 | none |
| ended, no successor | 20 | 324 (17 with a relay, 0.88) | none |

**Which lateral.**
- **Pairs, all take-overs:**
  - branch to branch: 557 at 40, 3,893 at 120;
  - twig to short shoot: 487 and 3,655;
  - branch to twig: 218 and 853;
  - bough to branch: 75 and 231;
  - limb to bough: 11 and 13.
- **On thick wood the bearer is almost always a branch.** On wood 1 cm or more, 343 of 451 bearers (40) and 3,138 of 3,662 (120) are branches. The branch row bears branch laterals only from the top cluster of each yearly unit (`BRANCH` p 0.02) and twigs below.

**Where it sits.** Take-overs with the bearer 1 cm or more:

| | 40 | 120 |
|---|---|---|
| bearer's wood still standing past the lateral, median (quartiles) | 0.39 m (0.17, 0.58), 18 phytomers | 0.25 m (0.06, 0.51), 16 phytomers |
| the same, bearer moved on | 0.43 m | 0.31 m |
| the same, bearer failed or aborted | 0.22 m | 0.16 m |
| the same, bearer living | 0.43 m | 0.46 m |
| no lateral with wood above it on the bearer | 0.27 | 0.45 |
| lateral born the cycle its node was laid | 1.00 | 1.00 |
| years from the lateral's birth to the bearer's stop, quartiles | 1, 3, 7 | 1, 3, 6 |
| bearer stopped within 1 year of the lateral's birth | 0.38 | 0.35 |
| bearer's own wood past it over the lateral's radius, median | 0.70 | 0.52 |

**The lateral is not a subapical bud that grew out after the stop.**
- **Born with its node, years before.** Every one was born with its node, in the year that node was laid. None woke later. The median is 3 years before the bearer stopped.
- **Well down the axis.** It sits 0.25 to 0.39 m down the bearer, 16 to 18 phytomers.
- **The bearer's wood past it still stands.** That wood is half to seven tenths as thick as the lateral.
- **It is a side branch that out-grew the line above it.** On fine wood (all take-overs) it sits closer: 0.04 to 0.07 m and 2 to 5 phytomers.

## 3. Why `relay`, `takeover` and the relay's frame do not apply (read, checked by the run)

**Read.**
- **The frame.** `geometry.rs` `framed` frames an axis by its origin. A `Lateral` goes to `lateral()`, which uses the insertion angle of its own PA (branch 1.0 rad, twig 0.8 rad). Only a `Relay` or `Continuation` gets the carried-on frame: blended from the bearer's tip line toward its `relay_at` stand by `blend`, and turned up by `epitony`.
- **The pull.** `taken` returns `NO_TAKE` unless the axis is a `Relay`, so `takeover` reaches only relays.
- **The carried pulls.** `Layer::handed` and `handed_take` are passed on only through `Axis::carried_on` (`geometry/place.rs`), which is none for a lateral.
- **So a lateral never gets any of these, whatever its girth turns out to be.**

**Checked by the run.**
- **No pull.** On the taken lateral's first phytomer the lay recorded no take-over pull (`over` 0) at any take-over joint: 0 of 1,382 at 40 and 0 of 8,742 at 120.
- **Insertion angle.** The lateral's heading against the bearer's last segment has a median of 51.6° on all take-overs and 57.3° on wood 1 cm or more. 57.3° is the branch row's `insertion` of 1.0 rad.

**Where the mechanism ran, it was not the line.**
- **Failed or aborted bearers.** The relay was made and framed as a relay, but its base is 0.43 to 0.52 of the lateral's radius, so the thickest wood runs into the lateral.
- **Bearers that moved on.** The continuation is laid on the line, but it is a shoot at 0.13 to 0.14 of the lateral's radius.

## 4. The host's direction on paper

**What it implies in the existing mechanism (read).**
- **Which axes.** `framed` and `taken` would treat a `Lateral` like a `Relay` where it out-carries its bearer's wood past its node and the bearer's `apex_end` is set.
  - It would get a blend, framed toward the bearer's line at its tip, with `epitony`.
  - It would get the take-over pull back onto that line at its PA's `takeover`.
- **Girth first.** The choice is made by girth, so the girth has to be known before the lateral is framed. Whether the layer's order allows that was not checked: **unknown**.
- **Settings it touches:**
  - `takeover`: branch 0.75, bough 0.75, limb 0, twig 0;
  - the relay frame's `relay_at` and `epitony` (branch 0.2), as read through `framed`.

**Can it be dormant at a neutral value?** Not through the existing settings.
- **`takeover` is already non-zero.** Branch and bough use 0.75 for their relays (fn-286), so applying it to laterals would change today's tree at today's values.
- **The frame has no value of its own.** It is keyed by origin.
- **So dormancy needs a value neutral at 0 that is not there today.**

**Expected effect on joints of 10° or more per metre (inferred from `raw/project.md`).** Walked wood 1 cm or more, joint 1 cm or more:

| | 40 | 120 |
|---|---|---|
| today, joints ≥ 10° per m | 0.443 (465 over 1,050 m) | 0.620 (4,274 over 6,889 m) |
| take-overs, bearer moved on | 0.126 | 0.251 |
| take-overs, bearer stopped otherwise | 0.101 | 0.143 |
| take-overs, bearer living | 0.119 | 0.046 |
| relays | 0.097 | 0.181 |
| relays turning ≥ 10° on this wood | 0.50 (median 9.7°) | 0.54 (median 10.6°) |

- **If every stopped-bearer take-over turned as a relay does:** about 0.33 per m at 40 (−26%) and 0.44 at 120 (−29%).
- **If only true stops counted,** leaving out the tips that moved on: about 0.39 (−11%) and 0.55 (−11%).
- **Pull alone changes no count.** If only the take-over pull applied and the frame did not, the count of joints of 10° or more would not change. fn-342 measured that take-over makes no joint of 10° or more and bends the line over 0.3 m chords instead. The 57° turn is in the lateral's base frame.

**Where the direction does not fit what was found:**
1. **Most of the line's change on thick wood is no stop.** It is the branch moving on to the shoot age at 12 years (32% at 40, 52% at 120). The line does carry on, in line, as the continuation. It is thin because the shoot age is a 4-year short-shoot bearer with a fine pipe, so the lateral below out-carries it.
2. **A third of the take-overs at 40 have no stopped tip at all:** 35% at 40, 12% at 120.
3. **The lateral that takes over is a side branch well down the axis.**
   - **Where and when.** It sits 0.25 to 0.39 m (16 to 18 phytomers) below the bearer's end and was born with its node, a median 3 years before the stop. Only 35 to 38% stopped within a year of its birth.
   - **What the relay's frame would do.** The frame stands a bud between the tip's line and `relay_at`. Applied here, it would lay this branch along the bearer's line from 0.25 to 0.39 m down, beside the 0.25 to 0.39 m of the bearer's own wood that still stands past it, which is half to seven tenths the lateral's radius.
4. **Where it comes closest: failed or aborted bearers** (27 to 28% of thick take-overs). Their lateral sits closer, 0.16 to 0.22 m down. Even there a relay already exists in the continuation's geometry, and it is about half the lateral's girth.

## Verification

- **Probe reverted.** The probe is saved as `probes/stops.patch`, which reapplies cleanly, and was reverted with `git apply -R`. `git status --short crates` is empty.
- **No still was made.** R1 needs none.
