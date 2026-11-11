# fn-346: the last measurement: SR and SRB with the settle's sag on and off, a third seed, and the leaf gap

Drafted by the worker on 11 Nov 2026 at `2fd9a788a` plus the host's R8 answers, for the host. This is `R8-ANSWERS.md`, "The last measurement", steps 1 to 5. No new rule was built.
- **The probe:** `probes/last.patch`, which is `stays.patch` with G never set, plus one census line for the leaf-gap check. It was reverted; `git status --short crates` is empty.
- **The variants:** SR is letters `PRMU` (the square charge with R); SRB is `PRMUS` (SR with rule B as built, no G). Both run at L_max 34.8 m undoubled.
- **Sag off** is the example's existing `--unsagged`, which sets every row's `form.sag` to 0. It draws: growth never reads sag, so the sag-off tree holds the same axes and leaves. Its stills are an instrument.

Marks: **measured** (printed by a run), **read** (from the code), **eye** (read off a still), **inferred**.

## Headline

- **The leaf gap is rule B's, not G's (measured, read).** SRB at 300 on seed 1001 draws 4,501 leaves (SRBG 7,509) and only 0.60 km of wood, against SR's 3.49 km.
  - **What the settled tree holds:** 2,221,267 of its 2,306,836 settled phytomers (96%) have a scale under 0.001, and 122,697 of those a scale of 0. Under SR it is 109,164 of 1,655,424 (7%), and under today's tree 11,722 of 1,537,433 (0.8%).
  - **Why (read):** a woken bud's presence is the product of its bearer's survival and persistence over every unit since the node grew (`dormant::carried`, `Sleeper::made`). Under rule B a bud waits decades and never expires, so that product falls toward 0, and every shoot it grows inherits it as its base scale. The census counts each phytomer's leaf whatever its scale, while the drawn tree draws wood and leaves by scale.
  - **What was not done:** the exact place where near-zero scale drops a leaf from the drawn tree was not traced within the twenty minutes. The drop is measured, and its link to the conversion is inferred.
- **Sag off stands the trees up (measured).**
  - **The sag-off heights:** SR's oak stands 30.2 m at 200 and 24.3 m at 300 on seed 1001, against 22.6 and 8.3 m with sag on. On 1007 it is 28.7 and 18.2 m against 23.0 and 15.3. Today's stands 39.2 and 52.2 m on 1001 against 34.3 and 40.2.
  - **The crown:** with sag off, SR at 300 on 1001 is a thin 24 m pole with a small crown at its top, not a crown of limbs (eye).
- **Seed 1012 (measured, eye).**
  - **At 40:** SR is today's tree.
  - **At 120:** SR is a broader, more open crown on a short trunk (38.2 × 29.6 m against 32.2 × 26.2 m), with 17% fewer leaves than today's.
  - **At 200 and 300:** SR is a low, wide tree, 23.9 m and 16.5 m tall, against today's leaning 33.9 m and 48.1 m pole with a crown at its top.
- **SRB at 40 and 80 and on the other presets: the same as SR and today's,** with 4% to 16% more leaves.
- **SRB's census equals SRBG's line for line on both seeds** (`YEAR` and `BAL` lines identical), as the host expected: growth reads no radius.

## 1 to 3. Measures (measured)

**Columns.**
- **Variants:** A is today's tree, SR and SRB as above. A trailing `u` is the same tree with the settle's sag off.
- **Height and spread:** the mesh's.
- **Trunk:** the diameter at 1.3 m.
- **Lowest living limb and limbs:** the settled stem line's limb row.
- **Leaves:** the current year's.
- **Held:** settled phytomers.

**Seeds 1001 and 1007, sag on beside sag off**

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|---|---|
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 |
|  | SR | 24.7 | 27.4 × 25.9 | 137 | 4.7 | 5 | 363,719 | 3,159,003 | 3.8 |
|  | SRu | 26.3 | 25.0 × 23.9 | 137 | 4.7 | 5 | 363,719 | 3,159,003 | 4.1 |
|  | SRB | 25.0 | 26.9 × 25.0 | 145 | 4.7 | 5 | 413,023 | 3,573,380 | 4.3 |
|  | SRBu | 26.4 | 24.7 × 23.4 | 145 | 4.7 | 5 | 413,023 | 3,573,380 | 4.5 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 |
|  | Au | 39.2 | 19.0 × 23.2 | 129 | 23.1 | 3 | 270,545 | 2,108,291 | 2.9 |
|  | SR | 22.6 | 29.1 × 37.2 | 82 | 4.7 | 3 | 97,454 | 1,541,098 | 2.3 |
|  | SRu | 30.2 | 28.0 × 23.0 | 82 | 4.7 | 3 | 97,454 | 1,541,098 | 2.4 |
|  | SRB | 23.4 | 29.3 × 42.0 | 102 | 4.7 | 4 | 170,581 | 2,192,188 | 2.9 |
|  | SRBu | 30.2 | 27.6 × 26.9 | 102 | 4.7 | 4 | 170,581 | 2,192,188 | 3.0 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 |
|  | Au | 52.2 | 26.3 × 29.1 | 120 | 33.0 | 1 | 233,403 | 1,537,433 | 3.1 |
|  | SR | 8.3 | 13.7 × 20.2 | 36 | 4.7 | 2 | 66,310 | 1,655,424 | 2.4 |
|  | SRu | 24.3 | 14.1 × 9.9 | 36 | 4.7 | 2 | 66,310 | 1,655,424 | 2.6 |
|  | SRB | 18.7 | 15.5 × 16.7 | 40 | 4.7 | 3 | 4,501 | 2,306,836 | 2.6 |
|  | SRBu | 22.5 | 21.4 × 13.7 | 40 | 4.7 | 3 | 4,501 | 2,306,836 | 2.5 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 |
|  | SR | 24.9 | 35.4 × 35.2 | 189 | 4.5 | 10 | 599,507 | 4,881,176 | 5.5 |
|  | SRu | 27.3 | 32.2 × 30.5 | 189 | 4.5 | 10 | 599,507 | 4,881,176 | 5.5 |
|  | SRB | 25.2 | 34.4 × 35.0 | 200 | 4.5 | 10 | 676,832 | 5,561,522 | 6.1 |
|  | SRBu | 27.5 | 31.8 × 30.7 | 200 | 4.5 | 10 | 676,832 | 5,561,522 | 6.1 |
| oak-200-1007 | A | 37.1 | 29.7 × 34.0 | 226 | 14.4 | 2 | 619,213 | 4,121,440 | 5.0 |
|  | Au | 40.8 | 28.7 × 30.3 | 226 | 17.3 | 2 | 619,213 | 4,121,440 | 4.9 |
|  | SR | 23.0 | 29.2 × 24.9 | 89 | 4.5 | 8 | 156,803 | 2,823,600 | 3.4 |
|  | SRu | 28.7 | 26.2 × 21.7 | 89 | 4.5 | 8 | 156,803 | 2,823,600 | 3.5 |
|  | SRB | 23.4 | 38.9 × 30.7 | 122 | 4.5 | 9 | 232,295 | 4,355,513 | 4.2 |
|  | SRBu | 32.5 | 27.5 × 33.0 | 122 | 4.5 | 9 | 232,295 | 4,355,513 | 4.2 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 |
|  | Au | 42.3 | 14.5 × 30.3 | 106 | none | 0 | 94,815 | 681,125 | 4.2 |
|  | SR | 15.3 | 16.2 × 13.6 | 51 | 4.9 | 4 | 102,800 | 2,290,382 | 3.5 |
|  | SRu | 18.2 | 13.9 × 15.3 | 51 | 4.9 | 4 | 102,800 | 2,290,382 | 3.5 |
|  | SRB | 14.2 | 13.8 × 14.7 | 52 | 4.5 | 5 | 125,059 | 2,984,933 | 4.2 |
|  | SRBu | 17.4 | 13.1 × 17.6 | 52 | 4.5 | 5 | 125,059 | 2,984,933 | 4.1 |

Today's sag-off tree at 120 was not drawn (asked at 200 and 300 only). Sag changes no leaf, no girth and no held count, only the pose.

**Seed 1012, sag on**

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|---|---|
| oak-40-1012 | A | 10.3 | 8.7 × 7.5 | 65 | 4.7 | 4 | 49,715 | 412,291 | 0.7 |
|  | SR | 10.3 | 8.7 × 7.5 | 65 | 4.7 | 4 | 50,121 | 418,877 | 0.7 |
| oak-120-1012 | A | 23.9 | 32.2 × 26.2 | 176 | 4.7 | 3 | 528,242 | 4,058,037 | 4.5 |
|  | SR | 23.5 | 38.2 × 29.6 | 160 | 4.7 | 4 | 436,807 | 3,823,631 | 4.3 |
| oak-200-1012 | A | 33.9 | 21.5 × 18.1 | 115 | 21.4 | 3 | 210,219 | 1,416,891 | 3.2 |
|  | SR | 23.9 | 38.2 × 27.7 | 114 | 4.7 | 4 | 198,824 | 3,192,874 | 3.3 |
| oak-300-1012 | A | 48.1 | 36.0 × 26.5 | 170 | 25.4 | 4 | 379,622 | 3,151,362 | 4.4 |
|  | SR | 16.5 | 20.2 × 17.9 | 59 | 4.7 | 2 | 160,626 | 2,345,145 | 3.4 |

**The young oak and the other presets, sag on**

| tree | v | height m | spread m | trunk cm | leaves | held | peak GiB |
|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 47,279 | 325,454 | 0.7 |
|  | SR | 10.6 | 7.6 × 8.0 | 56 | 47,532 | 329,158 | 0.7 |
|  | SRB | 10.6 | 7.7 × 8.0 | 57 | 49,451 | 341,526 | 0.7 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 209,845 | 1,593,559 | 2.0 |
|  | SR | 17.5 | 19.1 × 14.9 | 115 | 227,393 | 1,856,352 | 2.3 |
|  | SRB | 17.6 | 19.5 × 15.3 | 124 | 250,343 | 2,035,357 | 2.5 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | 16,614 | 83,359 | 0.4 |
|  | SR | 9.8 | 5.1 × 7.0 | 26 | 16,614 | 83,365 | 0.4 |
|  | SRB | 9.8 | 5.1 × 7.0 | 26 | 17,420 | 86,014 | 0.4 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | 463,750 | 1,887,629 | 2.6 |
|  | SR | 17.4 | 19.6 × 20.2 | 132 | 480,147 | 2,055,505 | 2.8 |
|  | SRB | 17.4 | 19.6 × 20.2 | 136 | 509,854 | 2,196,334 | 2.9 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | 139,929 | 374,880 | 0.7 |
|  | SR | 16.7 | 7.7 × 8.4 | 30 | 140,946 | 378,165 | 0.7 |
|  | SRB | 16.8 | 8.1 × 8.8 | 31 | 152,343 | 412,191 | 0.8 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | 4,185,635 | 1,137,653 | 2.0 |
|  | SR | 12.4 | 7.8 × 7.3 | 35 | 4,406,389 | 1,211,003 | 2.1 |
|  | SRB | 12.4 | 7.7 × 7.2 | 37 | 4,793,592 | 1,288,524 | 2.2 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | 6,554 | 480 | 0.3 |
|  | SR | 21.8 | 19.3 × 18.7 | 59 | 6,554 | 480 | 0.3 |
|  | SRB | 21.8 | 19.3 × 18.7 | 59 | 6,554 | 480 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | 7,957 | 92,345 | 0.4 |
|  | SR | 13.3 | 9.1 × 8.5 | 22 | 7,957 | 92,345 | 0.4 |
|  | SRB | 13.3 | 9.1 × 8.4 | 23 | 9,207 | 104,604 | 0.4 |

**The SRB census.** It equals SRBG's to 300 on both seeds (`raw/remain/oak-300-<seed>-SRB.log`; SRBG's table is in STAYS-RESULT.md).

## 4. The leaf gap (measured, read)

**The settled phytomers by scale at 300 on seed 1001.** Grow only, `PROBE_SCALES` (`probe_ablate::settled`), logs `raw/stills/oak-300-1001-{SRBs,SRs,As}-census.log`.

| variant | 0 | under 0.001 | 0.001 to 0.01 | 0.01 to 0.1 | 0.1 to 0.5 | 0.5 or more | settled phytomers |
|---|---|---|---|---|---|---|---|
| today | 0 | 11,722 | 25,456 | 98,563 | 318,788 | 1,082,904 | 1,537,433 |
| SR | 3,648 | 105,516 | 43,641 | 139,901 | 1,307,974 | 54,744 | 1,655,424 |
| SRB | 122,697 | 2,098,570 | 3,735 | 9,980 | 19,826 | 52,028 | 2,306,836 |

The year's own phytomers, those that bear this year's leaves, show the same split under SRB: 452,538 of 465,693 have a scale under 0.001.

**What the numbers show.**
- **The drawn leaf follows the scale.** The scales of 96% of SRB's settled phytomers are under 0.001, against 7% under SR. Its drawn wood is 0.60 km against SR's 3.49 km, and its drawn leaves 4,501 against 66,310. The census's leaf count (about 468,000) counts every phytomer laid that year whatever its scale (`grow/draw.rs`: a leaf per phytomer, its area `scale × size × leaf_area`), so it does not see that.
- **The cause (read; the last link inferred).** A woken bud's presence is its own draw's times what carried its bearer on: the product of every later unit's survival and persistence since the bud's node grew (`dormant::carried`; `Sleeper::made`). It enters as the new axis's base scale (`grow/draw.rs` `pencil`, `base_scale = inherited × vigour × share`).
  - **Why rule B makes it near zero:** under B a bud is held without ageing until light lets it wake. On a limb, bough or branch that has grown for decades, that product reaches near 0, and every unit the woken shoot and its laterals grow carries it.
  - **What was not traced:** the conversion that drops wood and leaves at near-zero scale.
- **The same holds with G.** SRBG at 300 on 1001 draws 7,509 leaves and 0.60 km of wood, so G is not the cause.

## 5. The read per tree (eye)

**The sheets.**
- **`last-oak-1001.jpg` and `last-oak-1007.jpg`:** rows 120, 200 and 300, each leafless then leafed. Columns today, today sag off, SR, SR sag off, SRB, SRB sag off. Today's sag-off at 120 is a grey tile (not drawn).
- **`last-oak-1012.jpg`:** today beside SR, sag on, leafless then leafed, at 40, 120, 200 and 300.
- **`last-others.jpg`:** today, SR and SRB, sag on, for the oak at 40 and 80 and the other presets.

**Seed 1001, sag on.**
- **120: SR and SRB better than today's.** The many-limbed clothed crown; SRB is a little fuller (+14% leaf).
- **200: SR and SRB worse than today's.**
  - **SR:** the thin, straggly crown with limbs to the ground on one side.
  - **SRB:** fuller (170,581 leaves against SR's 97,454), but low and sprawling (29 × 42 m on 23 m), with low limbs on the ground.
- **300: worse than today's.**
  - **SR:** the stem arcs over to the ground.
  - **SRB:** a crooked, near-bare 19 m stem with straggling bare wood at its foot (the leaf gap above).

**Seed 1001, sag off (instrument).**
- **200:** SR and SRB stand as upright 30 m trees with open crowns, narrower than with sag.
- **300:** SR is a 24 m bare pole with a small tuft of crown at its top; SRB a crooked bare stem.
- **Today's tree** stands 39 and 52 m, a pole with a crown at its top.

**Seed 1007, sag on.**
- **120: SR and SRB better than today's.** The broad, full oak on a short bole.
- **200: SR not worse than today's, not clearly better** (an upright 23 m tree, thinly clothed). SRB is fuller and wider (38.9 × 30.7 m).
- **300: SR better than today's.** A small upright tree on a straight trunk. SRB is the same, a little fuller (125,059 leaves).

**Seed 1007, sag off (instrument).** SR and SRB stand 3 to 9 m taller and a little narrower at every age. At 300 the crown leans to one side.

**Seed 1012.**
- **40: the same.**
- **120: SR better than today's in form.** Today's leans, with its crown weighted to one side over a bare stretch of trunk. SR is a broad, open, many-limbed crown on a short trunk, with 17% fewer leaves than today's.
- **200: SR different, not clearly worse.** A compact, gnarled 24 m crown of crooked limbs, 38 m wide on one axis, against today's 34 m leaning pole with a crown at its top.
- **300: SR different, not clearly worse.** A 16.5 m tree whose open, one-sided crown spreads from a short trunk, against today's 48 m leaning pole with a crown at its top.
- **What is wrong under SR:** at 200 and 300 the crown is one-sided and thin, and its wood is crooked.

**The young oak and the other presets: the same as today's and SR's under SRB.**
- **The oak at 40 and 80:** +4% and +10% leaf over SR.
- **The beech at 80:** +6%.
- **The birch:** a little wider (8.1 × 8.8 m), with its white trunk clear.
- **The ordinary preset:** +16% leaf.

## Friction

None new in this run.
