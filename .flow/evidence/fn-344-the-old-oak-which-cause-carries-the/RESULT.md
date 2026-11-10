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
