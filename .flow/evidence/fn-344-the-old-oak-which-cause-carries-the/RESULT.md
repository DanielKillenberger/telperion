# fn-344: the old oak, one cause off at a time

Drafted by the worker on 10 Nov 2026 at `eb773f78b`, for the host. Today's oak, with one of fn-342's three causes switched off at a time. These variants are instruments, not candidates. The reads below are the worker's eye on the stills, not the owner's. Nothing here proposes a fix.

Marks: **measured** (printed by a run), **eye** (read off a still), **read** (from the code, not run).

## Headline

- **The spec's census rule stopped every variant except C at 200 and 300.** "Four times today's count at that age" is measured against A, whose held phytomers halve in the year it sheds its fork limbs (2.8 million at year 115, 1.03 million at 132 on seed 1001). A variant that keeps those limbs passes four times A's count at that point without having grown to four times A's peak. On 1001, B and D stopped in year 132 for this reason. The question this leaves for the host is under "Open for the host".
- **Memory, not the rule, stops B, E and F.** On seed 1007 they reached the 8 GiB resident stop (years 103, 129 and 83). On 1001 they stood at 6.6 to 7.2 GiB when stopped. None of these three fits the desk to 300 in any reading.
- **At 120 (all drawn but F):**
  - **E reads nearest an old open-grown oak:** a broad, low, heavy-limbed dome, 43 × 47 m wide and 33 m tall.
  - **B is A with a fuller crown.**
  - **C is A.**
  - **D is a column.**
- **At 300, only C and the second reading of D could be drawn.** Both are further from an oak than A.

## How it was made

- **The probe.** `probes/oldoak.patch` builds on fn-343's `ablate.patch` (its trunk measure is kept). It is reverted, and `git status --short crates` is empty. `ABLATE` is a set of letters:
  - **B, dead wood not charged.** In `grow/cycle/working.rs` `totals`, a closed child subtree adds no upkeep to its bearer. A closed subtree has no living apex and no sleeping bud, so it is the dead standing wood fn-342 named. Open wood is charged as today, both its own upkeep and the upkeep of open axes that carry no light.
  - **C, the stem line pulled toward vertical as the leader is.** In `geometry/lay.rs` `Layer::step`, on the stem line (the lay's `trunk` flag: the seed and its continuations and relays) the form's `tropism` is 0.6 and its `elevation` π/2, the leader row's values. This leaves the trunk, fork and leader rows unchanged, since they already have those values. On the limb axes of the stem line it replaces the limb row's 0.5 toward 1.05 rad. Wander, bend length, sag and straightening are as today.
  - **D, the leader lives past the run.** This is a preset value only: the leader row's (PA 4) `lifespan` goes from 6 to 1000 years.
  - **E, sleeping buds on the trunk line wake as limbs.** This needed two parts.
    - **Preset values.** The trunk, fork and leader rows (PA 2 to 4), which hold no sleeping buds today, are given sleeping buds of PA LIMB at the limb row's sleeping values (p 0.1, delay 6, rate 0.04) in every zone.
    - **What the probe had to add.** The limb row's sleeping buds wake as BRANCH. On the stem line's limb axes only, `grow/wake.rs` `sleep` now draws them as LIMB at the same probability. Their expected-wood window is recomputed for LIMB. The stem line is tracked by a set of lineages, which a continuation or relay of the seed's line joins (`grow/shoot.rs` `push`).
    - **No new decision was needed.** A woken limb takes the age row the existing rule gives a sleeping bud: it ages in the bark through the years it slept (`dormant::aged`), and since the limb row's life is 500 years it wakes as a limb.
  - **F is B, C and E together.**
- **The census (R2).**
  - **Each year.** `grow/cycle/probe.rs` prints the held phytomers (`CENSUS`), the stem line's living limbs and the lowest one's base (`YEAR`, after fn-342's record).
  - **The stop.** The process stops (exit 42, `STOP`) when held passes four times A's count in the same year, read from the `raw/curve-<age>-<seed>.txt` file A's run wrote. It also stops at 8 GiB resident.
  - **Why 8 GiB.** The resident stop was 10 GiB for the 120-year runs. It was lowered after E at 120 ended its grow at 6.6 GiB and peaked at 9.1 GiB while dressed, which a 12 GB cap would not survive at a larger tree (`FRICTION.md`).
  - **No run was killed by the cap.**
- **The settled measures.** `grow.rs` prints the settled tree's living stem-line limbs and the lowest one's base (`SETTLED`).
- **The bole shot.** `still.rs` adds `bole`, a leafless mid view of the lower trunk: aimed at a quarter of the height, from 0.9 of the height away.
- **Runs.**
  - `probes/stills.sh` ran inside `telperion-heavy 12 --gpu 2 --threads 4` (17:32 to 17:59). `probes/stills-peak.sh` ran D's second reading (17:59 to 18:10).
  - Logs are in `raw/stills/oak-<age>-<seed>-<v>.log`, reduced by `probes/measures.py`.
  - The sheets are `sheet-B.jpg` to `sheet-F.jpg`, made by `probes/sheets.sh`. Each sheet has one row per tree and six columns: A beside the variant, leafless, leafed and the lower trunk. A grey tile carries the stop line.
- **A is today.** Its 120 figures equal fn-343's A: 25.1 m, 26.6 × 25.5 m, 142.6 cm and 304,559 leaves. Its 300 settled record matches fn-342: one living limb on 1001, none on 1007.

## The measures beside A's (measured)

The trunk is measured at 1.3 m. Lowest limb and living limbs are the settled tree's stem-line limbs. Leaves are the current year's. Peak is the whole process's resident peak, and s its wall seconds. Held/A is the held phytomers in the variant's last year against A's in that year. "Left A" is the first year the variant held 1.25 times A's count or more and never fell back under it.

| tree | v | height m | spread m | trunk cm | lowest limb m | limbs | leaves | peak GiB | s | held/A | left A | stopped |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 120/1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 3.0 | 25 | 1.00 | | |
| | B | 26.0 | 28.6 × 27.6 | 182 | 4.7 | 5 | 529,374 | 5.7 | 34 | 1.94 | 94 | |
| | C | 26.9 | 26.5 × 27.0 | 141 | 4.7 | 4 | 333,224 | 3.2 | 25 | 1.02 | never | |
| | D | 40.1 | 21.8 × 20.9 | 198 | 11.0 | 10 | 584,740 | 4.7 | 44 | 1.53 | 42 | |
| | E | 33.0 | 42.9 × 46.9 | 427 | 1.1 | 54 | 1,347,239 | 9.1 | 89 | 3.87 | 30 | |
| | F | | | | | | | 7.2 | 28 | 4.06 | 30 | year 85, 4 × A |
| 200/1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2.8 | 51 | 1.00 | | |
| | B | | | | | | | 6.9 | 38 | 5.75 | 93 | year 132, 4 × A (A halved) |
| | C | 42.0 | 21.4 × 21.5 | 132 | 26.0 | 3 | 287,276 | 2.9 | 48 | 1.04 | never | |
| | D | | | | | | | 3.2 | 49 | 4.52 | 42 | year 132, 4 × A (A halved) |
| | D, second reading | 65.8 | 25.1 × 26.7 | 248 | 40.9 | 21 | 617,044 | 5.3 | 116 | | | |
| | E | | | | | | | 6.8 | 79 | 4.11 | 30 | year 121, 4 × A |
| | F | | | | | | | 7.2 | 28 | 4.05 | 30 | year 85, 4 × A |
| 300/1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 2.9 | 96 | 1.00 | | |
| | B | | | | | | | 6.9 | 38 | 5.77 | 94 | year 132, 4 × A (A halved) |
| | C | 59.6 | 18.5 × 20.4 | 100 | none | 0 | 121,300 | 2.4 | 84 | 0.55 | never | |
| | D | | | | | | | 3.1 | 48 | 4.53 | 42 | year 132, 4 × A (A halved) |
| | D, second reading | 98.3 | 31.6 × 39.8 | 338 | 70.7 | 11 | 506,983 | 8.4 | 236 | | | |
| | E | | | | | | | 6.7 | 79 | 4.09 | 30 | year 121, 4 × A |
| | F | | | | | | | 7.2 | 28 | 4.05 | 30 | year 85, 4 × A |
| 300/1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 3.9 | 138 | 1.00 | | |
| | B | | | | | | | 8.2 | 38 | 1.68 | 84 | year 103, 8 GiB |
| | C | 47.3 | 19.4 × 26.4 | 106 | none | 0 | 119,076 | 3.3 | 116 | 1.13 | never | |
| | D | | | | | | | 6.5 | 206 | 6.21 | 244 | year 248, 4 × A (A halved) |
| | D, second reading | 99.1 | 34.0 × 25.0 | 265 | 70.0 | 23 | 712,193 | 6.7 | 272 | | | |
| | E | | | | | | | 8.2 | 104 | 2.11 | 33 | year 129, 8 GiB |
| | F | | | | | | | 8.3 | 32 | 2.34 | 32 | year 83, 8 GiB |

**D, second reading** (`stills-peak.sh`, outside R2): the stop is four times the most A had held by that year, with the 8 GiB stop kept. It was run for D alone, because only D was stopped by A's halving while far from the desk's memory. B on 1001 stood at 6.9 GiB in year 132 and would have met the 8 GiB stop soon after.

**What the stopped variants held in their last year (measured, `YEAR` records).**

| variant | tree | last year | stem-line limbs living | lowest m (rough layout) | A that year |
|---|---|---|---|---|---|
| B | 300/1001 | 131 | 5 | 4.7 | 2 living, lowest 18.1 |
| B | 300/1007 | 102 | 9 | 4.5 | |
| E | 300/1001 | 120 | 54 | 1.1 | 4 living, lowest 4.7 |
| E | 300/1007 | 128 | 68 | 1.0 | |
| F | 300/1001 | 84 | 44 | 0.7 | |
| F | 300/1007 | 82 | 51 | 0.3 | |

The `YEAR` heights are the rough layout's, before the settle.

- **B keeps the fork's limbs alive past the years A sheds them.** A's fork limbs die in years 111 to 127 (fn-342). In B all five are still living in year 131.
- **E fills the bole with limbs.** By 120 they stand down to about 1 m.

## The read, per variant (eye)

The bar is an old open-grown oak: a short thick bole, heavy low limbs, and a crown about as wide as tall. A at 120 is a two-limb V from about 5 m. At 200 and 300 it is a bare, snaking pole with a tuft. On 1007 at 300, its stem coils in a loop a few metres up and lies out along a long lean.

**B, dead wood not charged (`sheet-B.jpg`).**
- **At 120: nearer than A, a little.**
  - **Gained.** The same low fork with a fuller, rounder and denser crown. The trunk is thicker (182 against 143 cm), and more limbs and boughs leave low.
  - **Lost.** The V's open look: the two main limbs are partly hidden in clutter.
- **At 200 and 300: not drawn.** B outgrows the desk. It held twice A's count from year 94 and reached 6.9 to 8.2 GiB by year 103 to 132.
- **What it shows.** The yearly record shows B keeps the fork limbs that A loses (above). The eye has no 300 still to judge.

**C, stem line pulled toward vertical (`sheet-C.jpg`).**
- **At 120: the same as A.** The trunk above the fork is a little straighter.
- **At 200 and 300: further from an oak than A.**
  - **The pole.** The bare pole stays bare and gets taller and narrower: 59.6 m and 18.5 × 20.4 m on 1001, against 40.2 m and 26.0 × 37.2 m. The tuft is smaller, with about half the leaves on 1001.
  - **The bends.** C removes A's large bends: the S of 1001 and the loop of 1007 are gone. The stem still wanders in a gentle wave, and 1007 still leans from its base.
  - **Lost.** Spread and crown.
  - **The limbs.** No low limb comes back: none is living on the stem line at 300 on either seed.
- **What it shows.** The snaking is carried by the limb row's pull toward 60°. The bare pole is not.

**D, the leader lives past the run (`sheet-D.jpg`, the second reading at 200 and 300).**
- **At 120: further from an oak than A.** A single straight excurrent trunk, 40 m tall, with a narrow spire-like crown and no low fork limbs left standing (lowest at 11 m).
- **At 200 and 300: much further.** A pole of 66 to 99 m with a small tuft at the top and its limbs above 40 to 70 m. The stem still meanders, from the leader's own wander.
- **Lost.** The fork and every low limb.
- **Broke.** Height. Nothing ends the leader's rise, and it climbs at about 0.3 m a year.
- **What it shows.** Keeping the leader does not keep the limbs. It moves the rise from the limb axes to the leader.

**E, trunk-line sleeping buds wake as limbs (`sheet-E.jpg`).**
- **At 120: nearest an old open-grown oak of any tile, though not one.**
  - **Gained.** A broad, heavy, low dome, wider than tall (43 × 47 m by 33 m), with many thick limbs from about 1 m up and a massive bole flaring into them.
  - **Broke.** The crown is a dense, cluttered mass rather than a few heavy limbs, more a bush than an oak. The trunk measures 4.3 m at 1.3 m, about three times A's, as 54 limbs feed it. The bole is hidden.
- **At 200 and 300: not drawn.** E passes four times A's count in year 121 (1001) and reaches 8 GiB in year 129 (1007). It left A's curve in year 30 to 33, a few years after the first sleeping buds can wake.

**F, B, C and E together (`sheet-F.jpg`).**
- **Not drawn at any age.** It passes four times A's count in year 85 on 1001 and reaches 8 GiB in year 83 on 1007. By then it holds 44 to 51 living stem-line limbs, down to 0.3 to 0.7 m.
- **The sheet shows A alone,** beside grey tiles carrying the stop.

**Summary (eye and measured, not a proposal).**
- **E** (limbs born low on the bole) is the only cause that, switched off alone, gives a tree whose silhouette approaches an old open-grown oak. At 120 it overshoots into a dense bush, at four times the phytomers.
- **B** (dead wood charged) keeps the fork's limbs alive, by the record.
- **C** (the stem line's 60° pull) carries the snaking and nothing else.
- **D** makes a column.
- **Neither B nor E can be grown to 300 on this desk as they stand.**

## R5: verification

- **No product behaviour changed.** The probe lives in `probes/oldoak.patch` and was reverted with `git apply -R`; it re-applies cleanly (`git apply --check`). `git status --short crates` is empty.
- **The probe is dormant without its variables.** A's figures equal fn-343's. Every run set `ABLATE_CENSUS`, so A's census and records come from the same build.

## Open for the host

- **The census rule against a falling A.** R2 stops at "four times today's count at that age". A's count halves in year 132 on 1001 (and later on 1007), the year its fork limbs are shed. So any variant that keeps its limbs is stopped there, whatever its memory. Whether the stop should read A's count that year or the most A had held, as the second reading does, is the host's call. Only D's second reading was run, because B, E and F stop on memory either way.
- **What the eye still lacks.** There is no 300-year still of B or E, the two variants whose record keeps or makes low limbs. Drawing them at 300 needs either a larger memory scope than 12 GB, or a variant that grows less. Both are host decisions.
- **E's trunk girth (4.3 m at 120)** follows from the pipe model feeding 54 limbs. Whether that is right for an oak is not judged here.

## Follow-up: B grown as far as the desk allows (host, 10 Nov 2026)

**The host's answers.**
- **The stop rule** is the most A had held up to that year. The spec's wording, A's count that year, was the host's mistake.
- **B is the variant that matters.** A sheds its fork limbs in year 132 and B keeps them. The open question is whether B's tree levels off or grows without bound.

**The run.**
- **The patch.** `probes/oldoak.patch` was extended:
  - the census reads its factor from `ABLATE_FACTOR` and its resident stop from `ABLATE_RSS_GIB`;
  - `ABLATE_GROW_ONLY` ends the process once the tree is settled, so a tree too big to dress is never dressed.
- **The command.** `probes/b-census.sh 300 1001` ran under `GROW_ONLY=1 telperion-heavy 22 --gpu 2 --threads 4`, from 18:13 to 18:15. It stops at eight times A's running peak or at 20 GiB resident. The still runner asks the device at start, so the job went through the GPU admission rather than around it.
- **The log** is `raw/stills/oak-300-1001-Bh-census.log`, reduced by `probes/btable.py`. The patch is reverted and `git status --short crates` is empty.

**The result (measured).**
- **B stopped at 20 GiB, not at eight times A.** It hit the resident stop in year 188, holding 14,043,820 phytomers at 20.33 GiB. That is 4.7 times the most A had held by then. The run was not killed.
- **It never levels off.** B left A's curve in year 94 (1.25 times A's count). Its yearly gain in held phytomers has risen every decade since:
  - about 79,000 a year over years 109 to 119;
  - about 100,000 over 129 to 149;
  - about 145,000 over 159 to 169;
  - 196,000 a year over 179 to 187.
- **It is still rising, and faster, when stopped.**
- **Memory per phytomer grows too.** B holds 1.5 KB of resident memory per held phytomer at 187, against A's 0.7 KB. If the gain stayed flat at its last measured rate, B would hold at least 36 million phytomers at 300 and need about 50 GiB at its last memory per phytomer (an extrapolation, not a run). That is far past this desk.
- **No stills.** B did not reach 200, so there are no stills at 200 or 300 and no wood-point figure from the device. Seed 1007 was not run, as the host ordered, because 1001 did not finish inside the cap.

A beside B, every 10 years. Lowest limb and height are the rough layout's, before the settle (fn-342: the settled tree stands about 10 m lower at 300). Limbs are the living limbs on the trunk line.

| year | A held | A limbs | A lowest m | A height m | A GiB | B held | B limbs | B lowest m | B height m | B GiB | B / A's peak | B gain a year |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 9 | 7,773 | 0 | - | 3.2 | 0.1 | 7,773 | 0 | - | 3.2 | 0.1 | 1.00 | |
| 19 | 54,782 | 3 | 4.7 | 6.7 | 0.2 | 54,782 | 3 | 4.7 | 6.7 | 0.2 | 1.00 | 4,701 |
| 29 | 136,873 | 3 | 4.7 | 8.7 | 0.2 | 136,901 | 3 | 4.7 | 8.7 | 0.2 | 1.00 | 8,212 |
| 39 | 302,606 | 3 | 4.7 | 10.6 | 0.3 | 304,227 | 3 | 4.7 | 10.6 | 0.3 | 1.01 | 16,733 |
| 49 | 538,894 | 3 | 4.7 | 12.4 | 0.5 | 564,166 | 3 | 4.7 | 12.4 | 0.5 | 1.05 | 25,994 |
| 59 | 839,535 | 3 | 4.7 | 14.0 | 0.6 | 894,358 | 3 | 4.7 | 14.0 | 0.8 | 1.07 | 33,019 |
| 69 | 1,197,933 | 3 | 4.7 | 15.5 | 0.9 | 1,319,597 | 3 | 4.7 | 15.5 | 1.1 | 1.10 | 42,524 |
| 79 | 1,645,899 | 3 | 4.7 | 17.4 | 1.0 | 1,890,785 | 3 | 4.7 | 17.4 | 1.7 | 1.15 | 57,119 |
| 89 | 2,033,740 | 4 | 4.7 | 19.6 | 1.3 | 2,475,077 | 4 | 4.7 | 19.6 | 2.3 | 1.22 | 58,429 |
| 99 | 2,371,803 | 4 | 4.7 | 21.9 | 1.5 | 3,141,562 | 4 | 4.7 | 22.0 | 3.1 | 1.32 | 66,648 |
| 109 | 2,825,545 | 4 | 4.7 | 23.9 | 1.8 | 3,922,116 | 4 | 4.7 | 24.1 | 4.0 | 1.39 | 78,055 |
| 119 | 2,506,773 | 4 | 4.7 | 25.9 | 1.6 | 4,712,939 | 5 | 4.7 | 26.3 | 5.2 | 1.61 | 79,082 |
| 129 | 1,685,315 | 2 | 18.1 | 27.4 | 1.5 | 5,639,597 | 5 | 4.7 | 28.2 | 6.5 | 1.92 | 92,666 |
| 139 | 1,291,752 | 3 | 18.1 | 29.2 | 1.4 | 6,639,118 | 6 | 4.7 | 30.0 | 8.0 | 2.27 | 99,952 |
| 149 | 1,541,601 | 3 | 18.1 | 31.2 | 1.4 | 7,633,559 | 6 | 4.7 | 32.2 | 9.8 | 2.61 | 99,444 |
| 159 | 1,958,386 | 3 | 18.1 | 32.8 | 1.6 | 9,047,534 | 6 | 4.7 | 34.0 | 11.9 | 3.09 | 141,398 |
| 169 | 2,311,831 | 3 | 18.1 | 34.5 | 1.7 | 10,496,842 | 6 | 4.7 | 36.7 | 14.4 | 3.58 | 144,931 |
| 179 | 2,618,444 | 3 | 18.1 | 36.4 | 1.8 | 12,234,880 | 6 | 4.7 | 39.0 | 17.3 | 4.18 | 173,804 |
| 187 | 2,937,737 | 4 | 18.1 | 38.1 | 1.9 | 13,803,452 | 7 | 4.7 | 40.9 | 20.0 | 4.70 | 196,072 |

**The read (measured, no proposal).**
- **What B keeps.** B keeps the fork's limbs: its lowest living limb stays at 4.7 m to year 187, while A's rises to 18.1 m at 128. B's living limbs on the trunk line rise from 4 to 7. Its height runs only 1 to 3 m above A's.
- **Where the excess grows.** It is not upward: B holds more wood in the crown it keeps. The excess is already visible before A sheds anything (1.22 times A at 89, 1.39 at 109). So charging dead standing wood was already restraining the whole crown in A, not only its doomed limbs (inferred from the counts).
- **What this means.** Without that charge, nothing in the record restrains the total. The yearly gain grows with the tree and shows no sign of a ceiling by 188.

## Follow-up 2: the balance over the whole tree, A and B to 150 (host, 10 Nov 2026)

**The probe.** `probes/oldoak.patch` was extended with these numbers, each taken from the balance pass itself.
- **Income** is `w.light[0]` after `Working::totals`: the root's subtree light, which is the sum of every lit tip's own light.
- **Light per tip** is the `own` light of each living apex that `balance_one` uses.
- **A tip's balance** is its axis's q − c (`w.light[i] − w.cost[i]`), the raw value the balance walk is given, not the remembered one.
- **Upkeep** is summed inside `totals`.
  - **Living:** each open axis's own upkeep.
  - **Dead:** each closed child's frozen upkeep (`closed`). Under B it is still computed for the record, but not added to the cost.
  - **Charged** is `w.cost[0]`.
- **Tip deaths** are counted where the grower ends an apex:
  - **balance:** shed in `balance_one`;
  - **survival:** the viability draw failed (`advance.rs`, both branches);
  - **life:** the apex reached its PA's lifespan, so it moved on to the next PA or stopped.

  Abortions (7 to 17 in a year) are left out of the table.
- **Leaves** are the cycle's leaves the light reads (`Grower::leaves`).
- **Spread** is the extent of every phytomer tip on the two horizontal axes.
- **Ground area.** The lattice gives no ground area cheaply, so the area is the count of 1 m² ground cells holding any phytomer tip. That is closer to the projected crown than spread squared.
- **Heights and spreads** are the rough layout's, before the settle.

**The run.** `probes/balance.sh` ran under `telperion-heavy 14 --gpu 2 --threads 4`, grow-only, from 18:18 to 18:20. The tree was grown to age 150, with stops at eight times A's running peak or 12.5 GiB.
- **Neither variant was stopped.** A peaked at 1.9 GiB in 31 s, and B at 5.1 GiB in 64 s.
- **The 150 grow and the 300 grow agree.** Held phytomers at 149 match within 0.2% (B: 7.65 against 7.63 million). The 300-year request held 9.8 GiB at that year against 4.6 GiB here.
- **Logs** are `raw/stills/oak-150-1001-{A,B}bal.log` (`BAL` lines). The patch is reverted and `git status --short crates` is empty.

Every 10 years, the whole tree. Light and upkeep are in the balance's own units. "Neg %" is the share of living tips whose axis's q − c is below 0.

| v | year | income | living tips | q mean | q median | neg % | upkeep living | upkeep dead | charged | leaves | spread m | ground m² | income / m² | tips / m² | died balance | died survival | died life |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A | 9 | 607 | 1,191 | 0.510 | 0.488 | 0.00 | 20 | 6 | 26 | 3,099 | 1.4 × 1.3 | 4 | 152 | 298 | 0 | 130 | 95 |
| A | 19 | 1,985 | 4,122 | 0.482 | 0.475 | 0.02 | 94 | 155 | 249 | 13,137 | 3.5 × 3.2 | 14 | 142 | 294 | 1 | 655 | 681 |
| A | 29 | 3,952 | 10,751 | 0.368 | 0.354 | 0.00 | 227 | 554 | 781 | 32,238 | 5.0 × 5.1 | 28 | 141 | 384 | 1 | 1,986 | 1,505 |
| A | 39 | 6,373 | 24,658 | 0.258 | 0.196 | 0.00 | 521 | 1,464 | 1,985 | 68,724 | 6.8 × 8.0 | 47 | 136 | 525 | 52 | 6,670 | 2,961 |
| A | 49 | 10,112 | 41,497 | 0.244 | 0.186 | 0.01 | 885 | 3,195 | 4,080 | 116,159 | 10.4 × 9.8 | 76 | 133 | 546 | 447 | 11,630 | 4,986 |
| A | 59 | 15,101 | 62,146 | 0.243 | 0.197 | 0.01 | 1,332 | 5,898 | 7,230 | 176,519 | 13.3 × 9.0 | 106 | 142 | 586 | 348 | 17,104 | 7,783 |
| A | 69 | 20,916 | 86,179 | 0.243 | 0.202 | 0.00 | 1,808 | 9,860 | 11,668 | 246,652 | 14.0 × 12.1 | 132 | 158 | 653 | 576 | 22,971 | 11,088 |
| A | 79 | 26,786 | 112,459 | 0.238 | 0.195 | 0.01 | 2,344 | 15,362 | 17,706 | 321,687 | 16.4 × 14.0 | 183 | 146 | 615 | 2,919 | 31,133 | 14,584 |
| A | 89 | 33,422 | 140,350 | 0.238 | 0.191 | 0.01 | 2,997 | 22,151 | 25,148 | 402,472 | 19.6 × 16.3 | 240 | 139 | 585 | 3,660 | 39,647 | 18,585 |
| A | 99 | 38,141 | 163,084 | 0.234 | 0.192 | 0.01 | 3,334 | 30,530 | 33,864 | 463,019 | 22.3 × 19.1 | 324 | 118 | 503 | 1,349 | 45,397 | 20,888 |
| A | 109 | 44,006 | 189,037 | 0.233 | 0.197 | 0.01 | 3,871 | 40,038 | 43,910 | 547,011 | 22.8 × 22.1 | 350 | 126 | 540 | 2,457 | 50,324 | 25,270 |
| A | 119 | 41,004 | 163,119 | 0.251 | 0.220 | 0.01 | 3,391 | 50,625 | 54,016 | 475,968 | 24.6 × 22.5 | 381 | 108 | 428 | 6,556 | 45,173 | 22,680 |
| A | 129 | 23,317 | 62,396 | 0.374 | 0.342 | 0.00 | 1,638 | 59,447 | 61,084 | 192,924 | 20.2 × 23.7 | 319 | 73 | 196 | 542 | 12,098 | 9,713 |
| A | 139 | 28,552 | 88,230 | 0.324 | 0.290 | 0.00 | 2,205 | 64,313 | 66,518 | 269,232 | 21.0 × 23.8 | 286 | 100 | 308 | 3,347 | 20,059 | 13,373 |
| A | 149 | 29,021 | 103,429 | 0.281 | 0.241 | 0.01 | 2,509 | 70,528 | 73,037 | 300,784 | 18.4 × 26.6 | 270 | 107 | 383 | 554 | 26,951 | 13,900 |
| B | 9 | 607 | 1,191 | 0.510 | 0.488 | 0.00 | 20 | (6) | 20 | 3,099 | 1.4 × 1.3 | 4 | 152 | 298 | 0 | 130 | 95 |
| B | 19 | 1,985 | 4,122 | 0.482 | 0.475 | 0.02 | 94 | (155) | 94 | 13,137 | 3.5 × 3.2 | 14 | 142 | 294 | 0 | 655 | 681 |
| B | 29 | 3,952 | 10,750 | 0.368 | 0.355 | 0.00 | 227 | (554) | 227 | 32,268 | 5.0 × 5.1 | 28 | 141 | 384 | 1 | 1,988 | 1,513 |
| B | 39 | 6,390 | 24,701 | 0.259 | 0.196 | 0.01 | 527 | (1,461) | 527 | 69,009 | 6.8 × 8.0 | 49 | 130 | 504 | 0 | 6,706 | 3,000 |
| B | 49 | 10,313 | 42,665 | 0.242 | 0.184 | 0.01 | 929 | (3,189) | 929 | 120,103 | 10.4 × 10.2 | 81 | 127 | 527 | 5 | 12,046 | 5,239 |
| B | 59 | 15,474 | 64,717 | 0.239 | 0.192 | 0.00 | 1,423 | (5,946) | 1,423 | 184,707 | 13.3 × 13.5 | 123 | 126 | 526 | 50 | 18,000 | 8,236 |
| B | 69 | 21,299 | 93,918 | 0.227 | 0.183 | 0.01 | 2,001 | (9,980) | 2,001 | 267,027 | 15.3 × 14.8 | 147 | 145 | 639 | 22 | 26,069 | 11,980 |
| B | 79 | 28,611 | 130,372 | 0.220 | 0.175 | 0.01 | 2,826 | (15,614) | 2,826 | 371,393 | 17.9 × 14.6 | 191 | 150 | 683 | 24 | 37,668 | 16,861 |
| B | 89 | 36,255 | 165,341 | 0.219 | 0.172 | 0.00 | 3,609 | (23,165) | 3,609 | 473,950 | 20.6 × 17.2 | 255 | 142 | 648 | 167 | 48,226 | 22,036 |
| B | 99 | 43,769 | 211,885 | 0.207 | 0.162 | 0.01 | 4,543 | (32,584) | 4,543 | 600,695 | 23.9 × 19.6 | 333 | 131 | 636 | 174 | 63,157 | 27,463 |
| B | 109 | 52,743 | 256,521 | 0.206 | 0.164 | 0.01 | 5,639 | (44,217) | 5,639 | 732,111 | 26.2 × 22.1 | 411 | 128 | 624 | 440 | 77,296 | 33,720 |
| B | 119 | 61,539 | 308,555 | 0.199 | 0.159 | 0.01 | 6,697 | (58,214) | 6,697 | 876,058 | 26.3 × 24.5 | 472 | 130 | 654 | 213 | 93,613 | 40,186 |
| B | 129 | 72,258 | 364,709 | 0.198 | 0.159 | 0.01 | 7,988 | (74,733) | 7,988 | 1,032,926 | 28.4 × 27.8 | 582 | 124 | 627 | 234 | 111,571 | 47,241 |
| B | 139 | 82,915 | 428,262 | 0.194 | 0.155 | 0.01 | 9,521 | (94,286) | 9,521 | 1,212,102 | 30.1 × 31.5 | 695 | 119 | 616 | 460 | 131,787 | 55,369 |
| B | 149 | 94,826 | 501,712 | 0.189 | 0.149 | 0.01 | 11,250 | (117,271) | 11,250 | 1,397,186 | 32.1 × 34.4 | 772 | 123 | 650 | 614 | 157,960 | 62,033 |

Under B, the dead upkeep in brackets is what would have been charged; it is not added.

**The read (measured; nothing proposed).**
- **Income rises with the tips, not past them.**
  - **In B,** total income rises without levelling, from 10,313 at 49 to 94,826 at 149. Its income per square metre of crown ground stays flat, at 119 to 150 throughout (123 at 149). So does the number of living tips per square metre, at 616 to 683 from 69 on.
  - **The light per tip falls only slowly** (mean 0.24 at 49, 0.19 at 149).
  - **Why income keeps rising.** The crown fills each square metre to about the same density and income per square metre flattens. But the crown's ground area does not stop growing: 81 m² at 49, 333 at 99 and 772 at 149, spreading 10 m to 33 m across. Total income tracks that area. Living tips rise in step with it (42,665 to 501,712).
  - **What B lacks.** Nothing in the measured balance makes B's crown stop widening.
- **The balance never binds in B.** B's charged upkeep is the living wood alone, about 9 to 12% of income every decade (11,250 of 94,826 at 149). The share of living tips with a negative q − c is 0.01% or less in both variants. The balance sheds 614 tips in B's year 149, against 157,960 tips lost to the survival draw.
- **A's upkeep is nearly all dead wood.** Of A's total upkeep in each year, dead wood is:
  - 78% at 49 (3,195 of 4,080);
  - 88% at 89 (22,151 of 25,148);
  - 91% at 109 (40,038 of 43,910);
  - 97% at 129 (59,447 of 61,084).
- **A's whole-tree charge passes its income in year 109** (43,910 against 44,006) and stands at 2.6 times its income by 129. That is the year after its fork limbs are shed, when its living tips fall from 189,037 at 109 to 62,396.
- **Tips die mostly by the survival draw, in both variants.** In A in year 119 the balance sheds 6,556 tips against 45,173 lost to the survival draw. The balance's work in A is to shed whole limbs at once, not tips one by one.

## Follow-up 3: variant H, B with internodes shortened toward a height limit (host, 10 Nov 2026)

H is an instrument, not a candidate, and nothing here proposes one. **H = B** (dead wood not charged) plus one change: every new internode, on every growing tip of every rank, is multiplied by max(0, 1 − L / 34.8 m). L is the path along the wood from the root collar to that tip at the start of the year. 34.8 m is the oak's published potential maximum height (fn-341, R1-MEASURES.md).

**How L is carried (the probe; read and measured).**
- **Where it is stored.** The grower did not carry a path length. The probe stores it on the lay's walk (`geometry/lay.rs` `Layer`):
  - **base path:** the path at the axis's base;
  - **cumulative path:** the path after each phytomer laid, one f32 per phytomer;
  - **year path:** the path at the start of the year being laid.
- **An axis's base path** is its bearer's cumulative path at the node it stands on, for a lateral or a relay, or at the bearer's end, for a continuation. The seed starts at 0.
- **When the year path is taken.** It is taken when the first phytomer of a new cycle is laid on that axis. For an axis born that year, that is its base path.
- **The shortening.** The factor multiplies the internode's length in `Layer::step`, and only that. Node counts, the scale that laterals inherit, the wood upkeep (`internode × scale`), rates and angles are unchanged.
- **Both lays get the same lengths.** The same walk runs in the rough layout grown with the tree (`grow/draw.rs`) and in the final lay (`geometry/place.rs`, through a seventh field of the lay's start). So both lay the same lengths.
- **Cost.**
  - **No extra pass.** The probe adds one f32 per laid phytomer.
  - **Time.** H's grow at 120 took 28 s against B's 30 s.
  - **Time at 300.** H took 406 s to 300 with the 10-year whole-tree record on, and that record's ground-cell count over every phytomer is itself a cost.
- **The patch.** The stop factor and memory stop are as in follow-up 1. `ABLATE` takes the letter H, run as `BH`. Everything is in `probes/oldoak.patch`, which is reverted and re-applies cleanly; `git status --short crates` is empty.

**Runs.**
- **Grow-only first, from 18:24 to 18:51,** under `telperion-heavy 22 --gpu 2 --threads 4`, stopping at eight times A's running peak or 20 GiB:
  - H on seed 1001 at 40, 120, 200 and 300, and on seed 1007 at 300;
  - A and B on seed 1001 at 300 again, so all three carry the same whole-tree record.
- **Then drawn, from 18:51 to 19:07:** A and H at 40, H at 120, 200 and 300, then A and H at 260.
- **Scripts and sheet.** The runs are `probes/h.sh`, the table is `probes/htable.py`, and the sheet is `probes/sheet-h.sh`, giving `sheet-H.jpg`. Its rows are 40, 120, 200, 260 and 300; its columns are A beside H, leafless, leafed and the lower trunk.

**What stopped (measured).**
- **H on 1001** reached 300 inside the cap: 16.44 million held, grow peak 14.8 GiB.
- **The device refused H at 300.** "the wood points buffer needs 300932256 bytes, above the device's limit of 268435456". The mesh had 189.6 million wood vertices. The largest age drawn is 260, at 7.1 million nodes after settling.
- **H on 1007** stopped at 20 GiB in year 257, holding 24.68 million. It held 17.5 million at 199 (twice 1001's 8.3 million) and was not drawn: its grow alone would have neared the cap while dressed.
- **B on 1001** stopped at eight times A's running peak in year 237, holding 29.27 million at 18.9 GiB.
- **B's memory is not repeatable between runs.** In follow-up 1, B on the same seed held 13.8 million at 20 GiB in year 188. Here it held 22.3 million at 14.2 GiB in year 219. The same B build gave different resident memory per phytomer between the two runs. That is measured, not explained.

**Every 20 years, seed 1001.**
- **Held** is the phytomers the grower holds, and **tips** its living apexes.
- **Height and lowest limb** are the rough layout's, before the settle.
- **Spread** is the extent of phytomer tips on the two horizontal axes.
- **Ground** is the 1 m² ground cells holding a phytomer tip.
- **Limbs** are the living limbs on the trunk line.
- **GiB** is the resident memory in that year.

| year | v | held | living tips | height m | spread m | ground m² | lowest limb m | limbs | GiB |
|---|---|---|---|---|---|---|---|---|---|
| 19 | A | 54,782 | 4,121 | 6.7 | 3.5 × 3.2 | 14 | 4.7 | 3 | 0.2 |
| 19 | B | 54,782 | 4,122 | 6.7 | 3.5 × 3.2 | 14 | 4.7 | 3 | 0.2 |
| 19 | H | 53,905 | 4,071 | 6.1 | 3.3 × 3.1 | 13 | 4.4 | 3 | 0.2 |
| 39 | A | 302,606 | 24,622 | 10.6 | 6.8 × 8.0 | 46 | 4.7 | 3 | 0.3 |
| 39 | B | 304,227 | 24,702 | 10.6 | 6.8 × 8.0 | 48 | 4.7 | 3 | 0.3 |
| 39 | H | 274,278 | 23,016 | 9.2 | 6.4 × 7.6 | 41 | 4.4 | 3 | 0.3 |
| 59 | A | 839,535 | 61,634 | 14.0 | 13.4 × 9.0 | 106 | 4.7 | 3 | 0.7 |
| 59 | B | 894,358 | 64,651 | 14.0 | 13.3 × 13.1 | 119 | 4.7 | 3 | 0.7 |
| 59 | H | 773,529 | 58,373 | 11.9 | 11.8 × 10.4 | 89 | 4.4 | 3 | 0.6 |
| 79 | A | 1,645,899 | 111,239 | 17.4 | 16.4 × 14.1 | 184 | 4.7 | 3 | 1.0 |
| 79 | B | 1,890,785 | 130,741 | 17.4 | 18.0 × 14.6 | 193 | 4.7 | 3 | 1.1 |
| 79 | H | 1,570,854 | 115,310 | 14.2 | 14.3 × 10.8 | 115 | 4.4 | 3 | 1.1 |
| 99 | A | 2,371,803 | 160,988 | 21.9 | 22.4 × 19.0 | 316 | 4.7 | 4 | 1.5 |
| 99 | B | 3,141,562 | 212,390 | 22.0 | 23.9 × 19.5 | 335 | 4.7 | 4 | 2.1 |
| 99 | H | 2,605,435 | 188,277 | 16.3 | 17.5 × 13.7 | 175 | 4.4 | 4 | 1.9 |
| 119 | A | 2,506,773 | 158,355 | 25.9 | 24.4 × 22.4 | 375 | 4.7 | 4 | 1.7 |
| 119 | B | 4,712,939 | 307,400 | 26.3 | 26.0 × 25.0 | 463 | 4.7 | 5 | 3.0 |
| 119 | H | 3,813,670 | 268,321 | 18.4 | 17.9 × 17.2 | 231 | 4.4 | 5 | 2.9 |
| 139 | A | 1,291,752 | 84,904 | 29.2 | 20.6 × 23.7 | 288 | 18.1 | 3 | 1.4 |
| 139 | B | 6,639,118 | 426,991 | 30.0 | 29.9 × 31.2 | 681 | 4.7 | 6 | 3.8 |
| 139 | H | 4,941,820 | 347,399 | 20.3 | 19.3 × 20.1 | 284 | 4.4 | 6 | 3.3 |
| 159 | A | 1,958,386 | 132,432 | 32.8 | 19.5 × 27.1 | 304 | 18.1 | 3 | 1.6 |
| 159 | B | 9,047,534 | 590,873 | 34.0 | 33.4 × 35.9 | 846 | 4.7 | 6 | 5.6 |
| 159 | H | 6,245,124 | 446,982 | 22.1 | 21.8 × 19.3 | 304 | 4.4 | 6 | 4.7 |
| 179 | A | 2,618,444 | 173,430 | 36.4 | 24.7 × 30.1 | 423 | 18.1 | 3 | 1.8 |
| 179 | B | 12,234,880 | 803,025 | 39.0 | 37.6 × 44.9 | 1118 | 4.7 | 6 | 7.5 |
| 179 | H | 7,416,206 | 530,270 | 23.4 | 20.4 × 20.9 | 342 | 4.4 | 6 | 5.5 |
| 199 | A | 2,179,327 | 148,220 | 38.7 | 26.0 × 22.9 | 332 | 22.5 | 3 | 1.9 |
| 199 | B | 16,406,972 | 1,094,506 | 44.3 | 41.8 × 47.8 | 1363 | 4.7 | 7 | 10.2 |
| 199 | H | 8,301,197 | 605,900 | 24.6 | 21.5 × 22.1 | 361 | 4.4 | 7 | 6.3 |
| 219 | A | 2,919,458 | 205,858 | 42.4 | 26.4 × 25.6 | 345 | 22.5 | 3 | 2.1 |
| 219 | B | 22,307,241 | 1,513,309 | 49.7 | 48.6 × 57.8 | 1751 | 4.7 | 7 | 14.2 |
| 219 | H | 10,094,980 | 753,153 | 25.0 | 21.0 × 22.6 | 351 | 4.4 | 7 | 7.5 |
| 239 | A | 2,225,565 | 169,759 | 44.9 | 19.2 × 25.8 | 239 | 25.4 | 2 | 2.2 |
| 239 | H | 11,502,894 | 855,716 | 25.8 | 22.2 × 22.4 | 369 | 4.4 | 6 | 9.1 |
| 259 | A | 3,104,390 | 222,546 | 44.9 | 22.4 × 25.8 | 336 | 25.4 | 2 | 2.5 |
| 259 | H | 13,098,983 | 1,002,479 | 26.4 | 23.2 × 22.5 | 389 | 4.4 | 6 | 10.3 |
| 279 | A | 2,956,941 | 77,909 | 46.9 | 25.4 × 29.1 | 456 | 32.2 | 1 | 2.9 |
| 279 | H | 14,689,964 | 1,131,991 | 26.9 | 24.4 × 23.2 | 404 | 4.4 | 6 | 11.8 |
| 299 | A | 1,598,756 | 94,929 | 50.2 | 27.2 × 29.0 | 419 | 32.2 | 1 | 2.0 |
| 299 | H | 16,440,929 | 1,269,211 | 27.3 | 25.0 × 23.7 | 417 | 4.4 | 6 | 13.5 |

B stopped in year 237 (eight times A's peak), so it has no rows from 239.

Seed 1007 (H, stopped at 20 GiB in year 257):
- **Year 199:** 17.50 million held, 1.39 million tips, 25.5 m, 640 m², lowest limb 4.3 m, 10 limbs.
- **Year 239:** 23.05 million held, 1.87 million tips, 26.9 m, 30.7 × 29.3 m, 654 m², 10 limbs.
- **A on 1007 at 299:** 0.80 million held, no living limbs.

**The drawn trees (measured).** Spread is the mesh's two horizontal axes. Leaves are the current year's. The time is wall seconds for growing, dressing and drawing.

| tree | v | height m | spread m | trunk cm | leaves | s |
|---|---|---|---|---|---|---|
| 40/1001 | A | 10.6 | 7.6 × 8.0 | 56 | 47,279 | 1.5 |
| | H | 9.3 | 6.6 × 7.5 | 57 | 40,414 | 1.4 |
| 120/1001 | A | 25.1 | 26.6 × 25.5 | 143 | 304,559 | 25 |
| | H | 18.8 | 19.7 × 18.6 | 190 | 366,245 | 31 |
| 200/1001 | A | 34.3 | 22.1 × 22.1 | 129 | 270,545 | 51 |
| | H | 25.0 | 22.9 × 22.7 | 261 | 937,616 | 137 |
| 260/1001 | A | 41.2 | 25.9 × 16.7 | 130 | 246,852 | 83 |
| | H | 26.8 | 25.5 × 24.5 | 302 | 1,500,853 | 282 |
| 300/1001 | A | 40.2 | 26.0 × 37.2 | 120 | 233,403 | 96 |
| | H | refused: wood points 300.9 MB over the device's 268.4 MB | | | | 422 |

**The read.**

**Does H level off? In extent, yes; in held wood, no (measured).**
- **Height levels.** It stands at 22.1 m (rough) at 159, 24.6 at 199, 25.8 at 239 and 27.3 at 299, and the settled height is 26.8 m at 260. The gain falls from 0.06 m a year at 159 to 199 to about 0.03 at 239 to 299.
- **Ground area levels.** It is 304 m² at 159, 361 at 199 and 417 at 299. Spread holds near 22 to 25 m a side from 199 on.
- **Held wood and tips do not level.**
  - **Held** goes 6.25 million at 159, 8.30 at 199, 11.50 at 239, 16.44 at 299. The yearly gain rises from 44,000 over 179 to 199, to 70,000 over 219 to 239, to 88,000 over 279 to 299.
  - **Living tips** go 0.45, 0.61, 0.86 and 1.27 million at the same years.
- **So H stops widening and rising at about 25 to 27 m tall and 22 to 25 m across, from about year 200,** and then fills the same ground ever more densely.
  - **Tip density** rises from about 1,470 tips per m² at 159 to 3,040 at 299.
  - **B's density held near 650** (follow-up 2).
- **The balance starts to bind in H** (`BAL` lines, `raw/stills/oak-300-1001-H-grow.log`).
  - The share of living tips with negative q − c rises: 0.07% at 159, 1.3% at 199, 3.1% at 239 and 4.7% at 279.
  - The median light per tip falls from 0.062 to 0.026.
  - Total income still rises: 42,058 at 159, 94,012 at 279.
- **Seed 1007** grows the same way and denser still: 23 million held at 239, on 654 m².

**H at 40 against A (measured).**
- **Height:** 9.3 against 10.6 m, 12% lower.
- **Spread:** 6.6 × 7.5 against 7.6 × 8.0 m.
- **Wood:** 2.03 against 2.72 km.
- **Leaves:** 40,414 against 47,279.
- **Trunk:** the same, 57 against 56 cm.
- **The look (eye):** the same young tree, slightly smaller. Its form, its leader and its low limbs along the ground are unchanged.

**H at 120 against A at 120 (eye): not as good an oak.**
- **Gained.**
  - A rounder, more compact crown about as wide as tall (19.7 × 18.6 m by 18.8 m).
  - More, thicker limbs from the fork.
  - A heavier trunk (190 against 143 cm).
- **Lost.** A's open two-limb V and its airy, spreading crown. H's crown is a crowded mass of limbs.
- **Broke.**
  - **The trunk.** A hard step in girth on the bole a few metres up: a cylinder sleeve where the trunk meets what it bears (`oak-120-1001-H-bole.png`).
  - **The limbs.** Stubby, knotted wood along them.

**H at 200 and 260 against A (eye): nearer an old open-grown oak in silhouette, broken in its wood.**
- **The silhouette is nearer.**
  - **A** is a bare, snaking 34 to 41 m pole with a tuft.
  - **H** has a short, very thick bole (2.6 to 3.0 m at 1.3 m) and heavy limbs from about 4 m. Its crown is about as wide as tall: 22.9 × 22.7 m by 25.0 m at 200, and 25.5 × 24.5 by 26.8 m at 260.
  - That is the old open-grown oak's proportion, which A has lost by 200.
- **What it lost.** Every limb's length and taper. The crown is a dense knot of short, swollen limbs ending in tufts, more a pollard or a bonsai than the long reaching limbs of an old oak.
- **What it broke (eye, `oak-200-1001-H-bole.png`, `oak-260-1001-H-bare.png`).**
  - **The trunk top is an open tube.** At about 4 m it ends as a hollow cylinder rim, and the wood above leaves from inside it.
  - **Sleeves.** Abrupt girth steps along the limbs.
  - **Knots.** Short, bulbous, kinked segments, where internodes laid near the height limit are nearly zero long and the wood is drawn thick over them.
  - **No tree on the sheet should be judged by its wood at this distance until these are understood.** That is a host matter.
- **H at 300 could not be drawn** (device refusal above). Seed 1007 was not drawn.
