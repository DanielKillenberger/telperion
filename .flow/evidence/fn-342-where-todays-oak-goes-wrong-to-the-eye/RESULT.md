# fn-342: where today's oak goes wrong to the eye

Drafted by the worker on 10 Nov 2026 at `665b7a0d6`, for the host. Today's cycle, unmodified. Nothing here proposes a fix.

Marks: **measured** (a probe run, log named), **tagged** (the probe recorded the cause at the joint as the lay made it), **neutral run** (one cause's value set to neutral on the limb and bough PAs and the turns measured again), **read** (from the code, not run), **eye** (seen on a still).

## How it was measured

- **The probe** is `probes/turns.patch`. It is dormant unless its environment variables are set, and it is reverted from the crates (`git status --short crates` is empty).
  - **Lay tags.** `geometry/lay.rs` `Layer::step` records, for every phytomer, the degrees it turned by tropism, by wander, by sag and by the pulls. The pulls are straightening, erection, pulls a relay inherits, take-over and landing. The last lay wins, which is the settle's sagged lay. All 0.3 to 2.9 million phytomers per tree were tagged (`TREE ... tagged=`).
  - **Yearly record.** `grow/cycle/probe.rs` is called from `balance_one` each year inside the one grow. It writes the height, the stem line's limbs and boughs, and each one's balance terms. The stem line is the seed plus every continuation or relay of it.
  - **Joint walk.** `pipeline/turns_probe_tests.rs::turns_measures` walks every limb (PA 6) and bough (PA 7) lateral along its thickest wood, from its base and each joint once, and records the turn between successive segments. With `TURN_RMIN=0.01` it also walks every lateral of any PA that is 1 cm or more at its base. It walks the trunk path too.
- **Runs.** All runs went through `telperion-heavy 8 --threads 4` using `probes/runs.sh`; logs are in the ignored `raw/`. The reducers are `probes/turns.py`, `kinks.py`, `years.py`, `trunk.py` and `marks.py`.
- **A joint's cause.**
  - **Axis change.** Where the path changes axis, the cause is the new axis's origin:
    - **relay:** a relay bud carries on after the apex stopped.
    - **fork:** a lateral is thicker than the bearer's own continuation. It is split by that continuation's radius over the lateral's: under 0.7 is **fork-kink**, where the lateral reads as the line turning; 0.7 or more is **fork-Y**.
    - **lateral_at_end:** the bearer has no wood past the node.
  - **Same axis.** Within one axis, the cause is the largest tagged component: tropism, wander, sag, or the change in pull. That pull is **straightening** on a lateral and **take-over / inherited pull** on a relay.
- **Thresholds.** A joint counts at 10° or more. Chords count at 15° or more, over 0.3 m and 1 m of arc, and each chord's cause is the cause holding most of the summed joint turn inside it.
- **Width on screen.** The whole still frames the tree's height in about 0.78 of 720 px. That is 54 px per m at 40 and 22 px per m at 120, read off `sheet-bare.jpg`.

## R1: the jaggedness, measured

Limb and bough paths only, seeds 1001 and 1007 (`raw/t-40.log`, `raw/t-120.log`; tagged):

| cause | 40: share of joints ≥ 10° | 40: share of summed turn | 120: share of joints ≥ 10° | 120: share of summed turn |
|---|---|---|---|---|
| relay | 0.71 | 0.26 | 0.92 | 0.31 |
| fork | 0.22 | 0.11 | 0.06 | 0.02 |
| lateral_at_end | 0.07 | 0.04 | 0.02 | 0.01 |
| wander | 0 | 0.35 | 0 | 0.25 |
| tropism | 0 | 0.14 | 0 | 0.14 |
| take-over / inherited pull (boughs) | 0 | 0.10 | 0 | 0.26 |
| sag | 0 | 0.01 | 0 | 0.01 |
| straightening | 0 | 0.00 | 0 | 0.00 |
| joints ≥ 10° per m | 0.26 | | 0.26 | |
| summed turn, ° per m | 17.9 | | 21.9 | |

**Sharp turns come only from changes of axis.**
- **Not one joint turned by 10° or more from wander, tropism, sag, straightening or take-over.** Each of those turns a joint by 0.2 to 1.0° on average. Over a metre they add up to a bend: at 120, wander is the dominant cause of 34% of the 1 m chords that turn 15° or more, and take-over of 64% of the 0.3 m chords that do.
- **Relays.** A relay turns the path by 9° on average, and 15° on a limb at 40 (13° at 120).
- **Forks.** A fork or a lateral at the axis end turns it by its insertion angle: 34° from a bough lateral, 57° from a branch lateral.

**Neutral runs (40, both seeds, `raw/t-40-<neutral>.log`)** confirm the tags:
- **No relays.** With `abortion` 0 on limbs and boughs, joints of 10° or more fall from 0.26 to 0.07 per m, and none of them is a relay. The 0.3 m chords of 15° or more fall from 0.15 to 0.07 per m.
- **No wander.** With wander 0, joints of 10° or more are unchanged (0.27 per m). The median 1 m chord turn falls from 10.0° to 5.9°.
- **No sag, or no straightening.** Either leaves the counts unchanged.

**All wood 1 cm or more** (`raw/t-40-r0.01.log`, `raw/t-120-r0.01.log`; `raw/turns-*-all.md`) changes the picture:
- **Branch wood.** Most visible wood is branch wood (PA 9): 639 of 1,046 m at 40, and 1,892 of 2,923 m at 120 on wood 2.5 cm or more.
- **Its kinks are forks.** Branch wood kinks 0.58 (40) and 0.70 (120) times per m, mostly at forks. Forks are 74% of the joints of 10° or more at 40 and 67% at 120, and 86% and 84% of the summed turn of those joints.
- **Stopped apexes.** At 40, 1,025 of 1,382 forks of 10° or more stand where the bearer's apex had stopped (`apex_end` set). The continuation left beyond the fork is 10.5 phytomers on average.
- **The kink is the lateral.** The lateral that takes over turns the line by its insertion angle.

**On screen** (`raw/kinks.md`; joints of 10° or more per tree, by their wood's width in the whole still):

| | 40, ≥ 2 px | 40, ≥ 8 px | 120, ≥ 2 px | 120, ≥ 8 px |
|---|---|---|---|---|
| fork-kink | 57 | 7 | 210 | 8 |
| relay | 30 | 7 | 116 | 18 |
| fork-Y | 30 | 3 | 42 | 2 |
| lateral_at_end | 10 | 1 | 18 | 2 |
| per m of wood that wide | 0.49 | 0.47 | 0.49 | 0.26 |
| per 1,000 px of drawn line | 9.1 | 8.7 | 22.1 | 11.6 |

**Why 40 is worse: the measurement does not show it.**
- **Per drawn pixel, 120 has more kinks.** On the same tree at 120, kinks per drawn pixel are 1.3 to 3 times those at 40, and kinks per tree are 3 times as many.
- **Only the thickest wood is worse at 40.** On wood 8 px or wider, 40 kinks 0.47 per m against 0.26 at 120. At 40 that thick wood is still young limbs and boughs with relays and fork-kinks. At 120 it is old limb wood, where relays are 18 of 30 kinks over 116 m.
- **So the claim is unproven.** The table does not establish that 40 is worse to the eye. What the eye reads at 40 is in R2.

## R2: the jaggedness, seen

- **Stills.** `sheet-bare.jpg` holds the whole leafless trees: 40 on top, 120 below, seeds 1001 and 1007 left to right. `sheet-limb.jpg` holds the mid close-ups in the same order. The single stills are `raw/stills/oak-{40,120}-{1001,1007}-{bare,limb}.png`, from the clean `space_oak` example (sha1 `e75014a2…`, built before the probe was applied).
- **Marked sheet.** `sheet-marks.jpg` draws the walked wood of seed 1001 at 40 (left) and 120 (right) from the still's azimuth, with every joint of 10° or more on wood 2 px or wider ringed in red, one panel per cause.
  - **What it does not show.** Wander, tropism, sag, straightening and take-over have no panel: they make no joint of 10° or more.
  - **Fit.** The projection matches the bare stills' silhouettes.

What the worker sees, and where the table and the eye disagree:

1. **The elbows agree with the table.** The visible elbows on mid-size wood are the forks and relays the table ringed. Examples:
   - the Z-shaped branch in the 120/1001 close-up;
   - the elbow beside the trunk in the 40/1007 close-up;
   - the zigzag twig off the lower left limb in the 40/1001 close-up.
   On the marked sheet they carry fork-kink or relay rings.
2. **At 40, the eye reads the low limbs.** On 40/1001 the lowest limbs bend down onto the ground and run along it in angular steps. That bend is landing and sag, spread over many joints of under 10°, so the table rings none of it. This is the largest visible crookedness at 40 on that seed.
3. **At 40, the crown is a pole with short spiky laterals.** The eye reads the whole tree at 40 as twiggy and jagged: branch-wood kinks drawn 2 to 4 px wide in a small frame, around a straight trunk with few limbs. At 120 the same kinks are hairlines inside a crown dominated by smooth, thick limb arcs. **This is a reading of the stills, not a measure.** The kink density per pixel says the opposite (R1).
4. **Long bends are smooth to the eye.** The broad meander of the limbs that wander makes, about 10° per metre, reads as gently curved rather than jagged.

## R3: the oak at 300, its history

Seeds 1001 and 1007 (`raw/y-300.log`, reduced in `raw/years.md` and by `probes/trunk.py`). Yearly heights are the rough layout's, before the settle's sag. The settled tree stands about 10 m lower: 40.0 against 50.2 m on seed 1001, and 31.7 against 42.4 m on seed 1007.

**Height and its yearly gain (m per year):**

| years | seed 1001 | seed 1007 |
|---|---|---|
| 1–20 | 0.35 | 0.35 |
| 20–40 | 0.18 | 0.17 |
| 40–80 | 0.17 | 0.21 |
| 80–120 | 0.22 | 0.19 |
| 120–160 | 0.17 | 0.10 |
| 160–200 | 0.14 | 0.23 |
| 200–250 | 0.13 | 0.06 |
| 250–299 | 0.11 | 0.00 |
| stem line's top at 299, m | 46.1 | 37.7 |

After 120 the stem line's top goes on rising at 0.11 to 0.17 m a year on seed 1001 and 0.08 to 0.13 m on seed 1007, and nothing in the record slows it. Seed 1007's overall height stops at 42.4 m from year 200 while its stem top climbs from 26.4 to 37.7 m: something else stood higher, its stem line. The settled trunk path of seed 1007 ends at 23.6 m, where its thickest wood leaves the stem line for a bough.

**The living limbs (PA 6) on the stem line, and when they die (measured):**

- **Seed 1001.**
  - **The fork's limbs.** The fork's three limbs, born in years 13 and 14 at 4.7 to 5.0 m, die in years 111, 120 and 127.
  - **Replacements.** Later limbs are born only at the top: year 85 at 18.1 m, 119 at 22.5 m, 136 at 25.4 m, 180 at 32.2 m. The lowest living limb climbs: 4.7 m to 120, then 18.1, 22.5, 25.4 and 32.2 m. One limb lives at 299.
- **Seed 1007.**
  - **The fork's limbs.** Its five fork limbs and the one born in year 17, all at 4.5 to 6.0 m, die between years 90 and 176.
  - **Later limbs.** The limbs born in years 28, 95 and 117, at 8.0, 16.8 and 19.3 m, die in years 141, 210 and 243. None lives from year 243.
- **Boughs.** Boughs on the stem line behave the same way. Every one born before year 180 is gone by 299. Those living at 299 were born in year 180 or later on seed 1001 and 221 or later on seed 1007, all at 29 m and up.

**When and why the low limbs die (measured).**
- **The balance kills them all.** Every limb and bough that died on the stem line, on both seeds, died the year the balance walk shed it (`shed=1`), with its remembered balance just below 0 (`tolerance` 0). None died otherwise.
- **The light grew, the upkeep grew faster.** In its last five years, a dying limb's light was still rising, or flat. What had grown was the upkeep. The upkeep of its **dead wood standing** (closed subtrees whose upkeep the working set froze) was 8 to 10 times the upkeep of its living wood, as these rows from the record show:

| limb | its last year | light | upkeep of living wood | upkeep of dead wood standing | memory |
|---|---|---|---|---|---|
| 1001, born 13 at 4.7 m | 120 | 12,421 | 1,077 | 11,948 | −0.02 |
| 1001, born 13 at 4.7 m | 127 | 10,866 | 1,108 | 12,636 | −0.11 |
| 1007, born 17 at 6.0 m | 176 | 39,854 | 4,753 | 40,950 | −0.07 |
| 1007, born 95 at 16.8 m | 210 | 49,901 | 6,227 | 57,698 | −0.12 |

- **Dead wood stays charged.** The dead wood stays in the subtree's upkeep for as long as the grower holds it. Shedding is applied only at the settle (`grow/settle.rs`), so during growth a dead twig is never dropped from its bearer's account.
- **The cycle that kills a limb.** A limb's dead wood mounts year after year until upkeep equals light. The memory falls through 0, `balance_hazard` 2.0 fires, and the whole limb is shed.
  - **Code:** `grow/cycle/passes.rs` `balance_one` (Q, C, the D16 walk) and `grow/cycle/working.rs` `totals`/`closed`, which add a closed child's frozen upkeep.
  - **What is set and what follows:** the rate (`upkeep` 0.35), the hazard 2.0 and the tolerance 0 are prescribed in `oak.rs`. Q and C follow the tree's state.
  - **Unmeasured.** Whether the upkeep a dead subtree is charged ever falls during growth was not checked beyond this record, in which it only rises.

**Why nothing replaces them (read; supported by the births above).**
- **Where limbs come from.** A limb is borne only:
  - by the fork's top cluster, for two years (`fork` row: 3 buds at p 0.8);
  - by the leader's, for six years (LIMB p 0.08);
  - and on the stem line above the leader, which is itself the LIMB PA (`leader` continues into `limb`), by its top cluster at LIMB p 0.007 and BOUGH p 0.08.
- **They all start at the apex.** All of these stand at the growing apex, so every replacement is born at the top of the tree.
- **The bole cannot grow one.** The trunk, fork and leader rows hold no sleeping buds. The limb row's sleeping buds wake as BRANCH (`sleeping`, p 0.1), never as limb or bough. The bole below the crown therefore has no bud that could grow a new limb.
- **Prescribed.** These values are prescribed in `oak.rs`.

**Why the trunk snakes (measured, trunk path at 300).**
- **Above 7 m the trunk is a chain of limb relays.** The trunk path is the trunk PA to 4.2 to 4.4 m, the fork to 5 m and the leader to 7 m, all straight: under 1° at every joint and 31 to 34° summed. Above 7.0 to 7.1 m it is the **limb PA**: 67 axes on seed 1001 (65 relays, 2 laterals, born in years 20 to 296) and 35 on seed 1007.
- **Relays make the sharp turns.** On seed 1001, 40 of the 65 relay joints turn 10° or more, 903° summed. The largest are 31° at 17 m and 25.2 m, and 57° at the top, where a lateral takes over.
- **Within-axis turn bends it as much again.** Within the relay axes the path turns 837° more, every joint under 10°.
- **Not split at 300.** That within-axis turn was not split by cause at 300, because the run at 300 was not tagged. On the limbs at 120 the same split is wander 32%, tropism 14% and sag 5% of the summed turn (`turns.py --pa=6`).
- **Seed 1007.** 14 joints of 10° or more and 981° summed over 16 m of limb stem.
- **The pulls on that wood (read).** The stem above 7 m is laid with the limb row's `wander` 0.5, `bend_length` 3 and `tropism` 0.5 toward `elevation` 1.05 rad, about 60° above horizontal. That is the limb's ascent, not the vertical, so the leader's continuation leans toward a limb's angle. Its relays come at `abortion` 0.15 a year with `relay` 1.0.

## R4: causes, the code that sets them, prescribed or state

| cause | what it does to the eye | code | prescribed or state |
|---|---|---|---|
| relay | limb and bough elbows of 9 to 15° every 2 to 4 m; the trunk's kinks above 7 m | `oak.rs` limb `abortion` 0.15, bough 0.1, branch 0; `relay` 1.0; the relay's frame in `geometry.rs` `framed` (blend, `relay_at`, `epitony` 0.2) | rates and angles prescribed; which apex stops is a keyed draw at a prescribed rate |
| fork-kink and lateral_at_end | 34 to 57° elbows, mostly on branch wood (PA 9), most where the bearer's apex had stopped | insertion angle per PA in `oak.rs` (limb 0.5, bough 0.6, branch 1.0, twig 0.8 rad), framed in `geometry.rs` `lateral`; which wood is thicker comes from the pipe model, `girth.rs` | angle prescribed; that the lateral takes the line follows state (girth, the bearer's stop) |
| fork-Y | true forks, no kink | as above | as above |
| wander | the long meander, ~10° per metre; no sharp joint | `Form::wander`, `bend_length`, `geometry/lay.rs` `Layer::step` | prescribed (keyed noise) |
| tropism | a steady pull toward the PA's elevation; on the stem above 7 m, toward 60° | `Form::tropism`, `elevation`, `lay.rs` `toward_elevation` | prescribed |
| take-over and inherited pull | short bends back onto the line after each bough relay | bough and branch `takeover` 0.75 (fn-286); `geometry.rs` `taken`, `Layer::handed` | prescribed |
| sag and landing | the low limbs bending onto and along the ground at 40 (eye); 1% of the summed turn | `Form::sag` (limb 3·10⁻⁴), `sag.rs`; landing in `lay.rs` (`support`, `rested`) | rate prescribed; the bend follows the load (state) |
| straightening | none measurable | limb `straightening` 0.15, `geometry.rs` `bend` | prescribed |
| low limbs dying (300) | bare bole to 18 to 32 m | `passes.rs` `balance_one` and `working.rs` `totals`: dead standing wood charged at `upkeep` 0.35; `balance_hazard` 2.0, `tolerance` 0 | the rate, hazard and tolerance prescribed; the shedding follows state |
| no replacement limbs (300) | nothing new low down | `oak.rs` fork, leader and limb zones; `sleeping` wakes BRANCH only | prescribed |
| height gain after 120 | goes on at 0.1 to 0.17 m a year on the stem line | the limb row's 500-year life and constant internode (`oak.rs`); nothing ends the stem line | prescribed |

## R5: verification

- **No product behaviour changed.** The probe lives in `probes/turns.patch` and was reverted with `git apply -R`. `git status --short crates` is empty.
- **Clean stills.** The stills came from the example built from the clean tree before the patch was applied.

## Open for the host

- **Is 40 worse to the eye?** The measurement does not say 40 kinks more. The candidates the eye points at are not decided here: the landing of the low limbs, and how wide the branch wood's fork-kinks are drawn in a small frame.
- **The trunk's within-axis turns at 300 were not tagged.** Their split rests on the 120 limb split.
