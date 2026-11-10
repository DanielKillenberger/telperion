# fn-346: how the oak's tips are born, die and are replaced

Drafted by the worker on 11 Nov 2026 at `3ba285835`, for the host. This is `R2-ANSWERS.md`, "The measurement", steps 1 to 4. No behaviour changed. The census probe is `probes/renewal.patch` (pipes.patch plus the renewal census), and it was reverted; `git status --short crates` is empty. A's held phytomers equal R1's at every decade compared (302,606 at 39, 1,645,899 at 79, 1,598,756 at 299), so the probe changed nothing in today's tree. No rule is proposed.

Marks: **read** (from the code), **measured** (printed by a run), **arithmetic** (from the run's lines).

## Headline

- **There is no tip that lives on in place (read).** The short shoot lives 3 years and the twig 5. Neither moves on to another row, and neither has a relay. Leaves grow on the current year's wood alone (`leaf_years` 1). So every leaf-bearing tip must be replaced by a new lateral on a node that some bearer lays that year, every 3 to 5 years.
- **Turnover is large and almost balanced (measured).**
  - **Today's tree** turns over 0.5 to 0.9 million tips a decade from 79 on. Births over deaths stand at 0.91 to 1.06 in every decade and every row.
  - **P1 declines on a small imbalance:** 0.97 at 119, 0.90 at 139 and 0.92 at 199.
  - **P1 from 79 to 159:** the tips that died took about 3.52 million leaf-years. The tips born brought back about 3.42 million, and living leaves fell from about 131,000 to 49,000.
- **The loss is inside the crown (arithmetic).** In every variant and nearly every decade, the net of births minus deaths is negative inside the crown (more than 3 m of path short of the longest living path) and positive or near zero outside it.
  - **Today at 119:** −62,794 inside, +30,731 outside.
  - **P1 at 139:** −16,906 inside, +1,349 outside.
  - **The doubled instrument at 199:** −28,370 inside, +16,216 outside.
- **Light comes back where the balance sheds, and nothing grows there (measured).** In today's tree the light at the shed tips' places the year after is at or above that of the tips still living in every decade measured (0.61 against 0.33 at 119). Under the probes it rises above only once a large part goes (P1 at 139, 0.94 against 0.65). Yet the only bud that can start there is a sleeping one.
- **Sleeping buds are a trickle (read, measured).**
  - **What holds them:** only the limb row, at a tenth of its buds' places.
  - **When they wake:** they wake by a fixed timetable (delay 6 years, then 4% a year), never by light or by the loss of the wood beyond. They wake only while the limb bearing them lives.
  - **How few:** 2 to 40 wake per decade in any variant, against hundreds of thousands of births.
- **The framework stops extending, and with it the births (measured).** The new nodes come from extending bearers.
  - **The doubled instrument:** its boughs' yearly extension falls from 13.5 cm at 79 to 3.2 cm at 199 and 0 at 299, and its last living limb-row tip is gone after 239.
  - **P1:** its living boughs fall from 52 at 99 to 7 at 199, and its branches from 1,675 to 159.

## 1. From the code (read)

Sources are `oak.rs`, `chain.rs`, `species.rs`, `dormant.rs`, `grow/wake.rs` and `grow/advance.rs`.

**How a living tip comes into being.**
- **A lateral** on a node its bearer laid this year, by its zone's share per bud. It is `Origin::Lateral`, `woken: false`.
- **A sleeping bud waking** (`Origin::Lateral`, `woken: true`). Its time is drawn on its node's lineage (`dormant.rs`): the zone's `delay`, then an exponential wait at `rate`.
  - **No light or loss in it.** Nothing in that draw reads light or the loss of the wood beyond.
  - **Only on a living line.** It wakes only on an axis something still carries on ("It wakes only while the axis that bears it lives, through that axis's continuations and relays").
  - **Its age.** It ages through the years it slept, and one that slept past its last row never wakes (`dormant::aged`).
- **A relay:** a new axis of the bearer's own row at its last node. It is made when the apex aborts (`relay`), when its row ends without moving on (`relay_ended`), or when its unit fails (`relay_failed`).
- **A continuation:** the apex moving on to the next row on the chain when its life ends with `continuation` 1. It is a new axis but not a new tip.

**The oak's rows** (life in years, nodes a year, what the nodes bear, what follows).

| row | life | nodes a year (zones) | laterals per bud | sleeping buds | at the end of its life | survival a year; abortion |
|---|---|---|---|---|---|---|
| trunk (2) | 12 | 4–5 bare, 2–3, 1 cluster of 3 | sprig 0.4; top: sprig 0.6 | none | moves on (fork) | 1.0; 0 |
| fork (3) | 2 | 3–4, 2–3, 1×3 | sprig 0.3; top: limb 0.8 | none | moves on (leader) | 1.0; 0 |
| leader (4) | 6 | 3–4, 2–3, 1×3 | short 0.25, branch 0.4; top: limb 0.08, bough 0.1 | none | moves on to limb (top fork passed) | 1.0; 0 |
| limb (6) | 500 | 2–3, 2–3, 1×3 | short 0.4, branch 0.45; top: limb 0.007, bough 0.08 | branch, 0.1 per place, delay 6, rate 0.04 | ends; relay 1 (a new limb axis) | 1.0; 0.15, relayed |
| bough (7) | 60 | 2–3, 2–3, 1×3 | short 0.5, branch 0.3; top: branch 0.3 | none | ends; relay 1 | 0.999; 0.1, relayed |
| sprig (8) | 10 | 1–2, 2–3, 1×3 | short 0.3, twig 0.3; top: twig 0.4, branch 0.1 | none | moves on to branch | 0.95; 0 |
| branch (9) | 12 | 1–2, 2–3, 1×3 | short 0.6, twig 0.35; top: twig 0.1, branch 0.02 | none | moves on to shoot (branchlet and spur passed) | 0.98, a failure relayed; 0 |
| shoot (12) | 4 | 1, 2–3, 1×2 | short 0.3; top: short 0.35 | none | ends, no relay | 0.93; 0 |
| twig (13) | 5 | 1, 2–3, 1×3 | short 0.7; top: short 0.3 | none | ends, no relay | 0.98; 0 |
| short (14) | 3 | 3–5, no laterals | none | none | ends, no relay | 0.9; 0 |

**Can a short shoot go on in place?** No. It lives 3 years, at a survival of 0.9 a year, and then ends with no successor and no relay. A twig ends after 5 years the same way. Each makes leaves only on the year's new wood. Only the rows that live long and relay (limb 500 years, bough 60, both relaying on abortion and on their end) keep a line going, and every finer tip is born on their new nodes or on the nodes of rows born on them.

**What wakes a sleeping bud.** Only its drawn time. Nothing in `dormant.rs` or `grow/wake.rs` reads light, the balance or the shedding of the wood beyond. Where the limb holding it has died, the bud never wakes. A bud asleep more than 16 years (branch 12 plus shoot 4) never wakes either.
- **The share that ever wakes** (arithmetic): 1 − e^(−0.04 × 10) = 33% of sleeping buds wake within the 10 years between the end of the delay and the end of the shoot row.

## 2. The census (measured)

**The probe** (`probes/renewal.patch`, run by `probes/renewal.sh`, tables from `probes/renewal.py`).
- **Births** are counted where an axis is made (`Shoot::push`), by source, row and the path band of its base.
- **Deaths** are counted where an apex ends:
  - **balance:** shed in `balance_one`;
  - **survival:** the draw failed, including a failure that is relayed;
  - **end of row:** its life ended, including a row that moves on;
  - **abortion.**
- **Sums.** Each line sums the 10 cycles up to its year.
- **Inside** means more than 3 m of path short of the longest living path that cycle.
- **Living tips by row** are given with the length and nodes their axis laid the cycle before.
- **Sleeping buds held** are those scheduled to wake later.
- **Light at shed places** is the lattice light at the places of the tips shed in a census year, read with the field built after that year's growth, beside the light of the living tips in the same field.

**The runs.** All three runs are oak seed 1001 to 300, grow only, under `telperion-heavy 12 --gpu 2 --threads 4`. None was stopped. Logs are in `raw/renewal/`.
- **A** carries the path only (letter `L`).
- **P1** is `P`, L_max 34.8.
- **P2 with L_max doubled** is `PR` at 69.6 m.

### Births by source and deaths by cause (measured)

**A (today)**

| decade to | births: lateral | woken | relay | continuation | deaths: balance | survival | end of row | abortion |
|---|---|---|---|---|---|---|---|---|
| 39 | 80,539 | 9 | 403 | 305 | 324 | 44,640 | 22,381 | 38 |
| 79 | 454,455 | 8 | 2,312 | 2,016 | 16,971 | 283,268 | 132,801 | 62 |
| 119 | 727,250 | 29 | 4,086 | 3,356 | 87,917 | 447,693 | 231,076 | 98 |
| 159 | 531,715 | 16 | 2,614 | 2,395 | 15,364 | 327,999 | 163,497 | 65 |
| 199 | 805,643 | 17 | 5,488 | 3,737 | 107,225 | 522,055 | 240,207 | 98 |
| 239 | 898,191 | 40 | 9,858 | 4,346 | 161,697 | 675,526 | 161,586 | 137 |
| 299 | 367,913 | 4 | 1,116 | 1,670 | 16,165 | 177,935 | 173,110 | 30 |

**P1**

| decade to | births: lateral | woken | relay | continuation | deaths: balance | survival | end of row | abortion |
|---|---|---|---|---|---|---|---|---|
| 39 | 73,709 | 9 | 342 | 257 | 5,530 | 37,110 | 21,697 | 38 |
| 79 | 234,851 | 8 | 707 | 854 | 42,526 | 100,583 | 91,589 | 54 |
| 99 | 254,100 | 13 | 807 | 930 | 49,807 | 96,788 | 109,590 | 40 |
| 119 | 211,304 | 12 | 706 | 790 | 44,943 | 73,326 | 102,084 | 29 |
| 139 | 138,052 | 9 | 485 | 474 | 43,113 | 44,497 | 66,945 | 22 |
| 159 | 81,397 | 9 | 262 | 292 | 17,625 | 25,596 | 39,490 | 12 |
| 199 | 27,889 | 5 | 95 | 96 | 9,617 | 7,518 | 13,408 | 11 |
| 299 | 450 | 2 | 3 | 1 | 387 | 39 | 24 | 2 |

**P2 with L_max doubled**

| decade to | births: lateral | woken | relay | continuation | deaths: balance | survival | end of row | abortion |
|---|---|---|---|---|---|---|---|---|
| 39 | 80,546 | 9 | 403 | 305 | 0 | 44,734 | 22,539 | 38 |
| 79 | 455,363 | 8 | 2,086 | 2,012 | 16,196 | 274,753 | 143,522 | 62 |
| 119 | 741,516 | 25 | 2,726 | 3,277 | 36,310 | 385,609 | 310,606 | 74 |
| 159 | 834,779 | 16 | 2,675 | 3,615 | 69,039 | 382,662 | 394,994 | 56 |
| 199 | 648,644 | 3 | 1,940 | 2,789 | 49,361 | 258,491 | 357,660 | 18 |
| 239 | 367,594 | 2 | 995 | 1,671 | 24,726 | 140,406 | 224,308 | 7 |
| 299 | 203,296 | 0 | 552 | 946 | 5,410 | 80,577 | 121,632 | 0 |

**What these show.**
- **The balance's share of deaths.** It sheds under 1 to 16% of the tips that die in today's tree, 18 to 28% in P1 from 79 to 159, and 3 to 8% in the doubled instrument from 79 on. Most tips die by the survival draw or at the end of their row in every variant.
- **The framework rows** (`probes/renewal.py` with the per-row split).
  - **Limbs** die almost only by abortion, and every abortion is relayed.
  - **Boughs** die mostly at the end of their row, and each is relayed.
  - **Branches'** survival deaths equal their relays one for one (relay_failed 1).
  - **A branch the balance sheds is not relayed.** In P1 the balance takes 1,100 to 1,200 branches a decade from 79 to 139, and branch births fall from 2,742 (99) to 786 (159).

### Births over deaths by row, every decade (measured; full rows from `renewal.py <log> rows`)

| decade to | A branch | A twig | A short | P1 bough | P1 branch | P1 twig | P1 short | P2x2 bough | P2x2 branch | P2x2 short |
|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 1.66 | 1.45 | 1.18 | 1.15 | 1.41 | 1.29 | 1.14 | 1.19 | 1.66 | 1.18 |
| 79 | 1.16 | 1.11 | 1.05 | 0.90 | 1.00 | 1.01 | 1.01 | 1.04 | 1.11 | 1.05 |
| 119 | 0.90 | 0.93 | 0.96 | 0.97 | 0.90 | 0.95 | 0.97 | 1.04 | 1.05 | 1.02 |
| 139 | | | | 0.95 | 0.78 | 0.85 | 0.91 | | | |
| 159 | 1.15 | 1.11 | 1.05 | 0.97 | 0.93 | 0.98 | 0.99 | 0.98 | 0.97 | 0.99 |
| 199 | 0.92 | 0.92 | 0.94 | 0.98 | 0.87 | 0.90 | 0.92 | 1.00 | 0.97 | 0.98 |
| 239 | 0.87 | 0.88 | 0.92 | | | | | 0.98 | 0.86 | 0.95 |
| 299 | 1.03 | 1.02 | 1.01 | 2.00 (4/2) | 1.10 | 1.03 | 0.99 | 1.00 | 0.96 | 0.99 |

### Living tips by row, with last cycle's extension and nodes (measured)

| year | variant | limb | bough | branch | twig | short |
|---|---|---|---|---|---|---|
| 79 | A | 6 (6.1 cm, 3.0 n) | 64 (13.1, 5.5) | 3,381 (5.5, 4.5) | 16,519 (2.9, 3.4) | 90,722 (0.9, 2.0) |
| 119 | A | 10 (12.9, 4.5) | 94 (11.0, 4.6) | 4,734 (5.1, 4.5) | 23,354 (2.6, 3.4) | 129,328 (0.9, 2.0) |
| 199 | A | 12 (10.6, 4.6) | 98 (8.2, 4.3) | 4,961 (5.0, 4.5) | 23,522 (2.5, 3.3) | 118,868 (0.9, 1.9) |
| 299 | A | 2 (19.3, 5.5) | 42 (12.3, 4.2) | 2,270 (6.7, 4.6) | 12,074 (3.4, 3.5) | 80,018 (1.1, 2.3) |
| 79 | P1 | 5 (6.7, 2.4) | 44 (13.6, 5.2) | 1,580 (6.1, 4.4) | 7,243 (3.2, 3.4) | 44,102 (1.0, 2.2) |
| 99 | P1 | 7 (20.4, 6.4) | 52 (9.6, 3.8) | 1,675 (6.0, 4.4) | 7,559 (3.1, 3.4) | 48,077 (1.0, 2.3) |
| 119 | P1 | 5 (6.8, 2.4) | 42 (5.0, 1.9) | 1,322 (5.5, 4.5) | 6,294 (2.7, 3.4) | 40,778 (0.9, 2.3) |
| 139 | P1 | 4 (9.4, 4.0) | 21 (3.0, 1.4) | 547 (4.8, 4.5) | 2,535 (2.4, 3.3) | 16,925 (0.8, 2.4) |
| 199 | P1 | 2 (19.2, 6.5) | 7 (8.5, 3.4) | 159 (4.9, 4.3) | 617 (2.4, 3.1) | 3,722 (0.8, 2.3) |
| 79 | P2x2 | 5 (6.6, 2.4) | 62 (13.5, 5.4) | 3,246 (5.6, 4.5) | 16,237 (2.9, 3.4) | 94,347 (0.9, 2.1) |
| 159 | P2x2 | 8 (11.0, 3.9) | 102 (7.7, 3.0) | 4,995 (5.1, 4.5) | 25,892 (2.5, 3.5) | 172,658 (0.8, 2.3) |
| 199 | P2x2 | 2 (19.1, 6.0) | 71 (3.2, 1.2) | 3,798 (4.9, 4.5) | 19,922 (2.5, 3.5) | 138,470 (0.8, 2.4) |
| 299 | P2x2 | none | 15 (0.0, 0.0) | 1,136 (2.1, 4.6) | 6,241 (1.1, 3.5) | 45,722 (0.4, 2.4) |

- **Extension of the fine rows.** In every variant the fine rows lay about the same nodes a year at every age: branch 4.5, twig 3.4, short 2.0 to 2.4. Their extension per node shrinks slowly with age (short shoots 1.0 cm at 39, 0.4 to 0.8 cm at 199 to 299 under the probes).
- **Where the variants part:** the bough row. Today's boughs lay 4.2 to 5.5 nodes a year at every age. Under P1 and the doubled instrument they fall to 1.2 to 1.9 nodes, and in the instrument to 0 at 299.
- **The trunk, fork and leader rows** hold no living tip after 19. The main line's one tip is the limb row's from 29.

### Sleeping buds held and woken (measured)

| decade to | A held | A woken | P1 held | P1 woken | P2x2 held | P2x2 woken |
|---|---|---|---|---|---|---|
| 39 | 56 | 9 | 56 | 9 | 56 | 9 |
| 79 | 97 | 8 | 97 | 8 | 97 | 8 |
| 119 | 230 | 29 | 139 | 12 | 195 | 25 |
| 159 | 192 | 16 | 101 | 9 | 218 | 16 |
| 199 | 240 | 17 | 50 | 5 | 101 | 3 |
| 239 | 344 | 40 | | | 60 | 2 |
| 299 | 6 | 4 | 1 | 2 | 0 | 0 |

Every woken bud is a branch. "Held" counts only buds whose drawn time falls inside the request's 300 years.

### Light at the places of shed tips, the year after (measured)

| year | variant | tips shed | light at their places: mean | median | living tips: mean | median |
|---|---|---|---|---|---|---|
| 79 | A | 2,808 | 0.45 | 0.48 | 0.31 | 0.27 |
| 119 | A | 6,937 | 0.61 | 0.63 | 0.33 | 0.29 |
| 159 | A | 3,601 | 0.34 | 0.32 | 0.34 | 0.29 |
| 199 | A | 3,198 | 0.38 | 0.37 | 0.30 | 0.27 |
| 239 | A | 4,657 | 0.32 | 0.38 | 0.23 | 0.17 |
| 39 | P1 | 955 | 0.19 | 0.19 | 0.37 | 0.31 |
| 79 | P1 | 4,066 | 0.37 | 0.37 | 0.50 | 0.48 |
| 99 | P1 | 5,985 | 0.49 | 0.46 | 0.55 | 0.54 |
| 119 | P1 | 2,408 | 0.53 | 0.52 | 0.62 | 0.61 |
| 139 | P1 | 12,720 | 0.94 | 0.99 | 0.65 | 0.65 |
| 199 | P1 | 679 | 0.72 | 0.72 | 0.74 | 0.73 |
| 79 | P2x2 | 1,071 | 0.16 | 0.15 | 0.34 | 0.30 |
| 119 | P2x2 | 4,236 | 0.29 | 0.28 | 0.42 | 0.39 |
| 159 | P2x2 | 6,838 | 0.48 | 0.44 | 0.49 | 0.48 |
| 239 | P2x2 | 3,247 | 0.74 | 0.67 | 0.65 | 0.65 |

**Where the shed tips stood.**
- **In today's tree** the balance sheds whole limbs. The places light up once they are gone, to above the living tips' light (0.61 against 0.33 at 119).
- **Under the probes** the shed tips stand in less light than the living ones while the crown is young (P1 at 39 to 119, the instrument to 119), so the balance there takes shaded tips. Once a large part goes, the light at its places reaches the living tips' (P1 at 139, 0.94 against 0.65).

## 3. The arithmetic

**Births over deaths, the whole tree** (arithmetic from the tables).
- **Today:** 0.91 to 1.06 from 79 on, with the downs where it sheds limbs (119, 199, 239).
- **P1:** 1.007 at 79, 0.999 at 99, 0.966 at 119, 0.899 at 139, 0.991 at 159 and 0.919 at 199.
- **The doubled instrument:** 1.020 at 119, 0.993 at 159, 0.982 at 199, 0.951 at 239 and 0.986 at 299.
- **The decline is a few percent of a turnover that is otherwise nearly balanced, held decade after decade.**

**Inside the crown against outside** (net births minus deaths, arithmetic).

| variant | decade to | net | inside | outside | births inside / all births | deaths inside / all deaths |
|---|---|---|---|---|---|---|
| A | 79 | +25,689 | −2,352 | +28,041 | 0.73 | 0.78 |
| A | 119 | −32,063 | −62,794 | +30,731 | 0.89 | 0.93 |
| A | 159 | +29,815 | +11,341 | +18,474 | 0.87 | 0.90 |
| A | 199 | −54,700 | −67,306 | +12,606 | 0.93 | 0.95 |
| P1 | 79 | +1,668 | −16,931 | +18,599 | 0.61 | 0.68 |
| P1 | 99 | −375 | −21,175 | +20,800 | 0.71 | 0.79 |
| P1 | 119 | −7,570 | −15,530 | +7,960 | 0.81 | 0.85 |
| P1 | 139 | −15,557 | −16,906 | +1,349 | 0.85 | 0.88 |
| P1 | 199 | −2,469 | −2,412 | −57 | 0.95 | 0.95 |
| P2x2 | 159 | −5,666 | −33,227 | +27,561 | 0.87 | 0.91 |
| P2x2 | 199 | −12,154 | −28,370 | +16,216 | 0.89 | 0.92 |
| P2x2 | 239 | −19,185 | −17,954 | −1,231 | 0.95 | 0.95 |

**Most turnover is inside** (arithmetic).
- **The share inside:** most births and most deaths fall inside the crown (from 0.6 at 79 to 0.95 later). Deaths inside run 3 to 8 points above births inside in every row of the table, except where both reach 0.95.
- **Outside:** the crown's outer 3 m gains every decade until the tree as a whole has stopped extending (P1 after 159, the instrument after 199).

**P1 from 79 to 159: the leaves lost** (arithmetic).
- **The estimate.** A tip's leaves a year are its row's mean nodes the cycle before, since one leaf grows a node on the current year's wood. Over the decades to 89 through 159, the tips that died took about 3.52 million leaf-years:
  - **shed by the balance:** 753,703 (21%);
  - **the survival draw:** 1,183,667 (34%);
  - **the end of their row:** 1,582,602 (45%);
  - **abortion:** 916.
- **What came back.** The tips born in the same years brought about 3,417,361 leaf-years.
- **The deficit** is therefore about 103,000 leaf-years over 80 years, 3% of the turnover. The census's living leaves fell from 130,850 at 79 to 48,563 at 159, 82,287 leaves.
- **Which deaths go unreplaced.** Turnover from age and survival is 79% of the losses and is replaced at nearly the rate it happens. What is not replaced cannot be split between causes from these counts. A death by age is answered by a birth only where a living bearer lays new nodes.
- **The bearers fall faster than the fine tips.** Between 79 and 159 under P1:
  - **boughs:** 44 to 15 (−66%);
  - **branches:** 1,580 to 499 (−68%);
  - **short shoots:** 44,102 to 16,050 (−64%).

  The balance's branch sheds (about 1,100 to 1,200 a decade, none relayed) are the rows the fine tips are born on.

## 4. Questions for the host (none answered here; no rule proposed)

1. **Renewal and the bearers.** Every leaf-bearing tip lives 3 to 5 years and is replaced only by a lateral on a node a bearer laid that year. So the crown's leaf rests on the bearers (limb, bough, branch) still extending. Should the measure of a steady state be bearers held, rather than tips held?
2. **A shed branch is never relayed.** A branch that fails its survival draw is relayed one for one. A branch the balance sheds is not, and it carried the twigs and short shoots of the next years. Is that asymmetry intended?
3. **Sleeping buds and loss.** Sleeping buds are held only on limbs and wake on a fixed timetable, never on light or on the loss of the wood beyond. At most 40 wake per decade, and none can wake where a limb has died. The light at shed places returns to above the living tips' (today at 119, P1 at 139). Is the absence of a response to loss the gap, and is it the host's to settle what real oaks do there (epicormic release)?
4. **The boughs slowing.** Under P1 and the instrument the bough row's yearly nodes fall from about 5 to 1 to 2, and to 0 in the instrument at 299, while today's hold 4 to 5. This census does not show which term does it: the allotment's size, `kept`, or a relay inheriting an old bough's age. Should that be measured next?
5. **Inside against outside.** The net loss is inside the crown in every variant, today's included, and the outer 3 m gains. Is the crown that hollows from the inside the same finding as the stills' "unclothed limbs with tufts at their ends", and is it to be read as the renewal's failure rather than the charge's?
6. **Today's tree too.** Today's tree also runs births below deaths in its shedding decades (0.96 at 119, 0.94 at 199, 0.91 at 239). It keeps its tip count only by its limbs' growth outward. Is "a tree with a steady state" a bar today's tree also fails?

## Friction

None new.
