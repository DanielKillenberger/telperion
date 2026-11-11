# fn-346: what the old oak under Q loses, how its limbs lie, and Q on every preset

Drafted by the worker on 11 Nov 2026 at `97c680da7` plus the host's R4 answers, for the host. This is `R4-ANSWERS.md`, "The measurement", steps 1 to 6, on Q: the charge by size × path / L_max with L_max doubled, dead wood charging nothing, and the main line tested. No behaviour changed. The census probe is `probes/oldtree.patch`, and it was reverted; `git status --short crates` is empty. No rule is proposed.

Marks: **measured** (printed by a run), **arithmetic** (from the run's lines), **read** (from the code), **eye** (read off a still).

## Headline

- **Limbs and boughs are lost whole while part of them still pays (measured).** From 119 to 299 the balance shed lines whose own balance stood only just below zero (−0.003 to −0.24 remembered), and most of them still held paying children the year before.
  - **Seed 1001:** 3 of the 4 limbs and 66 of the 78 boughs it shed had at least one own bough or branch above zero.
  - **Seed 1007:** 8 of 13 limbs and 176 of 198 boughs.
  - **The share of children paying:** about a third of a shed line's own children were above zero (mean 0.32 to 0.44).
  - **Lines lost with their bearer:** as many again were lost with their bearer as the bearer was shed (1001: 6 limbs, 69 boughs; 1007: 29 limbs, 254 boughs).
- **Almost nothing is born where a limb went (measured).** In the 20 years after each limb loss, the framework born on the same bearer within 3 m of path of its base was mostly nothing.
  - **Seed 1001:** 9 of 14 losses were followed by no new limb, bough or branch at all.
  - **Seed 1007:** 47 of 58.
  - **The stem's sleeping buds run out.** It holds 28 to 29 at 122 and none from 219 (1001) or 243 (1007).
- **The droop is the sag rule's, and the stem line itself lies down (measured).** In the settled tree the fork limbs' tips stand −3.2 to +17.4 m against their bases at 160 on 1001 and −4.5 to +2.2 m at 200 on 1007; with the sag rule off the same limbs stand +6 to +25 m and +7 to +17 m.
  - **Down to the ground:** at every age from 160, at least one fork limb's subtree reaches 0.0 to 1.3 m above the ground.
  - **The stem line too:** a limb born on the stem line at 16.4 m of height (unsagged) stands at 3.6 m sagged on 1007 at 200. The stem line that carries the crown on after the leader has bent down under the crown it bears.
  - **Refused on 1007:** the settled tree is refused at 240, 280 and 300 because a limb-row axis reaches 1 to 2 cm below the ground. The rough layout, which has no sag, stands 40 to 42 m tall there.
- **The limbs' tips stop extending (measured).** The fork limbs' lines lay 0.14 to 0.20 m a year to about 200 on 1001 (to about 160 on 1007), then nothing. Their lines' last apexes die by abortion: limbs whose length is 14 to 52 m end with 1 to 6 living tips.
- **The leaf sinks as the tree ages (measured, rough layout).** At 159 on 1001, 30% of tips stand at 25 to 30 m of height. By 259, 57% stand at 15 to 20 m and 6% above 30 m, while the rough layout's height holds at 41.6 m: the upper crown empties. On 1007 the leaf stays high (40% at 35 to 40 m at 259), and the tips fall from 266,000 at 139 to 40,000 at 299.
- **The other presets under Q are the same as today's (eye, measured).** Height and spread are unchanged on every preset. Leaf moves 0 to +5% and held phytomers 0 to +9%.

## How it was made

**The probe** (`probes/oldtree.patch`, which is RENEWAL's census plus the following).

**A line** is a limb (PA 6) or bough (PA 7) lateral and the relays and continuations of its row that carry it on (read: limbs and boughs relay on abortion, on a failed unit and at the end of their row, `oak.rs`).

**Each year in `balance_one`, for each living line, the probe keeps a record** (all printed with the line's end):
- its remembered balance, light and charge, as the walk read them;
- its length (its longest living tip's path less its base path) and its tip's path;
- its living tips, the shed apexes of that year included;
- its own boughs (for a limb) or branches (for a bough) and how many of them have light above charge.

**How a line ends:**
- **balance:** the walk shed the line's head itself, and its bearer was not shed;
- **with bearer:** shed as part of a bearer that was shed;
- **died:** no living apex left in it. The cause is that of the line's last apex: abortion, survival or end of row.

**The losses.** For each limb that ended, the probe counts the framework births (limb, bough, branch) whose base lies within 3 m of path of the limb's base on the same bearer line (the stem line, or the bearer limb or bough line) in the 20 years after. It also counts the sleeping buds held on the stem line in the year of the loss.

**Reach.** Every 20 years the probe records the longest living path and the living tips by 5 m band of height (rough layout).

**Shape** (`probe_ablate::settled`). The tree is grown to 120, 160, 200, 240 and 280, then settled.
- **What is measured, for each living limb on the stem line:** its line's length, base radius, horizontal reach from its base, its tip's height against its base, the lowest point of the line and of its whole subtree, phytomers below the ground, and the length laid in the last ten cycles.
- **Sag off for comparison.** The same is measured with the sag rule off (`--unsagged`, which sets each row's `form.sag` to 0). Growth never reads sag (`sag.rs`), so the two settled trees hold the same axes, matched by lineage.

**Runs.** All runs went under `telperion-heavy`. Logs are in `raw/oldtree/` and `raw/stills/`; the tables come from `probes/oldtree.py`.
- **History:** `probes/oldtree.sh history <seed>`, both seeds to 300. Neither was stopped by its census. The 1007 run ended with the settle's refusal at 300 after the census to 299 was complete.
- **Shape:** `probes/oldtree.sh shape <seed>`.
- **The other presets:** drawn with `probes/pipes.sh`, and the one-thread seconds measured alone (no other heavy job running).
- **The first build of this probe failed.** It panicked reading a bearer axis from inside a grow share, and every run of it exited 101 at once (FRICTION.md). The runs here are the second build's.

## 1. Every limb and bough line's end, 119 to 299 (measured)

| seed | row | shed by its balance | shed with its bearer | died by abortion | died at the end of its row | died by survival |
|---|---|---|---|---|---|---|
| 1001 | limb | 4 | 6 | 3 | 0 | 0 |
| 1001 | bough | 78 | 69 | 16 | 65 | 0 |
| 1007 | limb | 13 | 29 | 10 | 0 | 0 |
| 1007 | bough | 198 | 254 | 42 | 132 | 2 |

**Lines the balance shed: their balance and whether part of them paid the year before** (arithmetic):

| seed | row | shed by its balance | remembered balance | own children above zero: lines with at least one | mean share of own children above zero |
|---|---|---|---|---|---|
| 1001 | limb | 4 | −0.055 to −0.029 | 3 | 0.44 |
| 1001 | bough | 78 | −0.177 to −0.004 | 66 | 0.35 |
| 1007 | limb | 13 | −0.115 to −0.004 | 8 | 0.32 |
| 1007 | bough | 198 | −0.241 to −0.002 | 176 | 0.37 |

**Every limb line that ended on seed 1001** (from `oldtree.py raw/oldtree 1001 ends`):

| year | cause | born | on the stem | base path m | base height m | memory | light | charge | length m | tip path m | living tips | own boughs | paying |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 122 | balance | 84 | no | 15.8 | 13.9 | −0.029 | 740 | 774 | 6.1 | 21.9 | 3,146 | 3 | 1 |
| 122 | with bearer | 98 | no | 18.0 | 15.1 | −0.036 | 360 | 376 | 3.9 | 21.9 | 1,510 | 3 | 1 |
| 150 | balance | 133 | no | 20.7 | 17.2 | −0.036 | 157 | 177 | 0.3 | 21.1 | 4,452 | 6 | 0 |
| 170 | with bearer | 157 | no | 37.7 | 30.6 | +0.019 | 258 | 240 | 3.2 | 41.0 | 518 | 0 | 0 |
| 170 | with bearer | 161 | no | 38.7 | 31.4 | +0.139 | 116 | 100 | 2.2 | 41.0 | 201 | 1 | 1 |
| 180 | with bearer | 157 | no | 21.1 | 17.5 | −0.058 | 213 | 247 | 0.4 | 21.6 | 5,766 | 9 | 0 |
| 219 | with bearer | 213 | no | 48.6 | 37.7 | +0.109 | 114 | 110 | 1.3 | 49.9 | 163 | 4 | 4 |
| 225 | balance | 208 | no | 44.4 | 35.5 | −0.055 | 485 | 538 | 3.8 | 48.2 | 870 | 4 | 3 |
| 226 | balance | 208 | no | 44.4 | 35.5 | −0.035 | 712 | 738 | 3.7 | 48.1 | 1,314 | 3 | 2 |
| 246 | died, abortion | 130 | no | 31.6 | 26.5 | +0.157 | 3 | 2 | 16.1 | 47.7 | 4 | 1 | 1 |
| 256 | with bearer | 256 | no | 54.3 | 41.6 | +1.000 | 1 | 1 | 0.0 | 54.3 | 1 | 0 | 0 |
| 281 | died, abortion | 103 | no | 25.1 | 21.7 | +0.239 | 0 | 0 | 16.6 | 41.7 | 3 | 1 | 1 |
| 284 | died, abortion | 85 | yes | 20.8 | 18.1 | +0.150 | 4 | 3 | 28.1 | 48.9 | 6 | 1 | 1 |

**What the table shows.**
- **The fork limbs never end.** Those born in years 13 and 14 on the stem at 4.7 to 5.0 m are not in the table on 1001. Their lines live to 300, with their tips stopped (shape below).
- **The limbs that end** are limb-row laterals born high on other limbs (bases at 14 to 54 m of height). Most are shed at a balance of −0.03 to −0.06 with half their light against their charge, or with their bearer.
- **Seed 1007's 52 limb ends** (`oldtree.py raw/oldtree 1007 ends`) show the same.
  - **A limb shed at near zero:** the largest, born in 121, carried 27,222 living tips and 12 own boughs, 9 of them paying. It was shed in 202 at −0.004, with light 8,270 against charge 8,226, and 6 limbs went with it the same year.
  - **The stem's own oldest limbs die by abortion:** those born in 13 and 17 at 4.5 to 6.0 m die in 162, 262 and 294. Their lines are 21 to 52 m long and end with 3 to 5 living tips.

**Read** (from `contract/shed.rs`). A lateral's balance is one number for its whole subtree, (Q − C)/(Q + C), remembered as a running mean. The hazard reads that number alone, and a shed takes every apex in the subtree (`walk`). Nothing in the walk lets the paying part of a shed line stay.

## 2. Each stem limb's shape in the settled tree (measured)

**Seed 1001.** These are the fork limbs born in 13 and 14 at 4.7 to 5.0 m, and the stem's later limb laterals. The last two columns are the same limb with sag off.

| age | born | base m | base radius cm | length m | reach m | tip − base m | lowest of the line m | lowest of the subtree m | below ground | m a year, last 10 years | sag off: tip − base m | sag off: lowest of the line m |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 120 | 13 | 4.7 | 34.4 | 16.2 | 8.8 | +7.5 | 4.8 | 4.8 | 0 | 0.138 | +12.9 | 4.8 |
| 120 | 13 | 4.7 | 42.9 | 23.2 | 13.6 | +16.2 | 4.8 | 4.8 | 0 | 0.195 | +18.9 | 4.8 |
| 120 | 14 | 5.0 | 20.4 | 8.4 | 7.6 | +1.0 | 5.1 | 3.2 | 0 | 0.000 | +6.3 | 5.1 |
| 160 | 13 | 4.7 | 29.9 | 21.8 | 11.0 | +0.9 | 2.7 | 0.9 | 0 | 0.168 | +17.6 | 4.8 |
| 160 | 13 | 4.7 | 46.7 | 30.5 | 19.1 | +17.4 | 4.8 | 4.8 | 0 | 0.188 | +25.4 | 4.8 |
| 160 | 14 | 5.0 | 20.4 | 8.4 | 4.8 | −3.2 | 1.8 | 0.0 | 0 | 0.000 | +6.3 | 5.1 |
| 200 | 13 | 4.7 | 30.4 | 28.0 | 15.6 | +2.2 | 3.9 | 1.2 | 0 | 0.160 | +22.4 | 4.8 |
| 200 | 13 | 4.7 | 43.1 | 38.7 | 21.9 | +10.0 | 4.8 | 4.8 | 0 | 0.194 | +30.4 | 4.8 |
| 200 | 14 | 5.0 | 20.6 | 8.7 | 3.2 | −4.8 | 0.3 | 0.0 | 0 | 0.000 | +6.5 | 5.1 |
| 240 | 13 | 4.7 | 27.6 | 13.1 | 5.4 | −2.5 | 2.2 | 0.3 | 0 | 0.000 | +10.8 | 4.8 |
| 240 | 13 | 4.7 | 29.1 | 16.6 | 13.6 | −2.1 | 1.0 | 0.1 | 0 | 0.000 | +14.5 | 4.8 |
| 240 | 14 | 5.0 | 22.0 | 8.4 | 2.5 | −4.9 | 0.1 | 0.0 | 0 | 0.000 | +6.3 | 5.1 |
| 280 | 13 | 4.7 | 23.1 | 12.5 | 4.6 | −4.6 | 0.1 | 0.0 | 0 | 0.000 | +10.4 | 4.8 |
| 280 | 13 | 4.7 | 9.5 | 10.1 | 6.2 | +7.4 | 4.7 | 4.7 | 0 | 0.000 | +9.0 | 4.7 |
| 280 | 14 | 5.0 | 19.6 | 8.4 | 1.6 | −4.8 | 0.1 | 0.0 | 0 | 0.000 | +6.3 | 5.1 |

**A limb born in 85 on the stem line** (base unsagged 18.4 m) stands at 17.1 m at 120, 15.4 m at 160 and 13.0 to 13.1 m from 200: the stem line under it bends down.

**Seed 1007** (the settled tree refused from 240; selected rows, full table from `oldtree.py raw/oldtree 1007 shape`):

| age | born | base m | base radius cm | length m | reach m | tip − base m | lowest of the line m | lowest of the subtree m | m a year | sag off: tip − base m | sag off: lowest of the line m |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 120 | 13 | 4.5 | 63.7 | 22.9 | 17.8 | +9.8 | 4.6 | 4.0 | 0.169 | +16.3 | 4.6 |
| 120 | 13 | 4.5 | 37.4 | 17.4 | 7.1 | +1.7 | 3.1 | 2.7 | 0.000 | +10.2 | 4.6 |
| 160 | 13 | 4.5 | 54.3 | 31.5 | 25.1 | +6.1 | 4.6 | 1.3 | 0.189 | +22.6 | 4.6 |
| 160 | 13 | 4.5 | 26.4 | 16.3 | 9.4 | −4.3 | 0.2 | 0.0 | 0.000 | +9.3 | 4.6 |
| 160 | 28 | 7.9 | 50.2 | 25.6 | 15.6 | −0.1 | 6.7 | 4.7 | 0.175 | +16.1 | 8.1 |
| 200 | 13 | 4.5 | 49.0 | 24.1 | 16.6 | +2.2 | 4.6 | 0.0 | 0.000 | +17.0 | 4.6 |
| 200 | 13 | 4.5 | 37.7 | 23.5 | 14.9 | −4.5 | 0.1 | 0.0 | 0.000 | +17.3 | 4.6 |
| 200 | 28 | 7.7 | 43.0 | 23.0 | 5.8 | −3.5 | 1.8 | 0.7 | 0.000 | +14.8 | 8.1 |
| 200 | 95 | 3.6 | 32.8 | 9.3 | 6.5 | +6.0 | 3.7 | 3.7 | 0.000 | +6.7 | 16.4 |

**The refusals** (measured), each "reaches … below the ground":
- **at 240:** axis 260, PA 6, born in 83, base at 0.313 m, 0.013 m below;
- **at 280:** axis 128, PA 6, born in 79, base at 0.286 m, 0.019 m below;
- **at 300:** axis 85, PA 6, born in 74, base at 0.657 m, 0.015 m below.

**"Below ground" counts 0 in every tree drawn.** The settle clamps a phytomer at the ground, so a drooping limb lies on the ground (lowest of the subtree 0.0 m from 160 on). The 1007 trees from 240 are refused instead.

**What the sag rule reads** (read: `sag.rs`, `geometry/sagged.rs`).
- **The turn.** Each phytomer turns down by `sag` × the bending moment of the load beyond it ÷ its radius⁴, per metre of its length.
- **The load** is the wood beyond it (length × radius², sized by presence) and its foliage (its own pipe's section × length), the axes it bears included.
- **The moment** is taken base to tip, the load beyond each phytomer turned by the bends already made, so a branch "hangs and stops".
- **The oak's values:** `sag` is 0.0003 on the limb row, 0.00005 on boughs, and 0 elsewhere (`oak.rs`). Growth never reads it.

**The share of the droop it gives** (arithmetic). On the fork limbs the tip stands 2 to 22 m lower with sag on than off: from the tables, 16.7 m at 160 and 20.2 m at 200 on 1001's limb born in 13 (+0.9 against +17.6, +2.2 against +22.4). The unsagged limb's tip stands 6 to 30 m above its base at every age measured. So the droop to the ground is the sag rule's alone. Without it the limbs still rise from their bases.

## 3. After a limb is lost (measured)

**Framework born within 3 m of path of the lost limb's base, on the same bearer, in the 20 years after** (`oldtree.py <dir> <seed> losses`):

| seed | limbs lost (all years) | followed by nothing | followed by something: limbs, boughs, branches born (largest case) |
|---|---|---|---|
| 1001 | 14 | 9 | 5 cases; the largest after the loss in 78: 5 limbs, 3 boughs, 14 branches; in 150: 3, 3, 35 |
| 1007 | 58 | 47 | 11 cases; the largest after 223: 3, 5, 40; after 243: 4, 0, 22 |

**Sleeping buds held on the stem line in the year of a limb loss:**
- **1001:** 28 at 122, 17 at 150, 6 at 170, 5 at 180, then 0 from 219.
- **1007:** 29 at 122, 28 at 123, 7 at 144, 2 to 4 from 155 to 220, 1 to 241, then 0 from 243.

The stem line holds sleeping buds only on its limb-row stretch (read: only the limb row holds them, `oak.rs`), and they expire after 16 years asleep (RENEWAL.md).

## 4. The tree's reach and where the leaf sits (measured, rough layout before the settle)

**Seed 1001.**

| year | longest path m | height m | spread m | living tips | living tips by height |
|---|---|---|---|---|---|
| 79 | 19.7 | 17.4 | 17.6 × 14.3 | 114,482 | 5–10 m 24%, 10–15 m 63%, 15–20 m 13% |
| 119 | 29.6 | 25.4 | 24.3 × 22.3 | 187,539 | 5–10 m 5%, 10–15 m 31%, 15–20 m 38%, 20–25 m 26% |
| 159 | 38.7 | 31.5 | 27.8 × 30.3 | 204,753 | 10–15 m 18%, 15–20 m 33%, 20–25 m 15%, 25–30 m 30%, 30–35 m 4% |
| 199 | 46.5 | 37.7 | 32.0 × 34.5 | 163,124 | 10–15 m 10%, 15–20 m 35%, 20–25 m 12%, 25–30 m 11%, 30–35 m 26%, 35–40 m 6% |
| 239 | 50.7 | 40.1 | 37.8 × 36.7 | 90,173 | 10–15 m 11%, 15–20 m 48%, 20–25 m 17%, 25–30 m 9%, 30–35 m 7%, 35–40 m 7% |
| 279 | 53.8 | 41.6 | 34.3 × 27.8 | 61,435 | 10–15 m 16%, 15–20 m 56%, 20–25 m 18%, 25–30 m 4%, 30–35 m 3%, 35–40 m 3% |
| 299 | 55.5 | 41.6 | 34.4 × 29.0 | 53,401 | 10–15 m 17%, 15–20 m 56%, 20–25 m 15%, 25–30 m 8%, 30–35 m 3% |

**Seed 1007.**

| year | longest path m | height m | spread m | living tips | living tips by height |
|---|---|---|---|---|---|
| 79 | 20.6 | 17.6 | 20.4 × 21.3 | 161,215 | 5–10 m 21%, 10–15 m 68%, 15–20 m 10% |
| 119 | 31.3 | 24.4 | 29.1 × 29.2 | 247,162 | 5–10 m 6%, 10–15 m 21%, 15–20 m 49%, 20–25 m 24% |
| 159 | 38.7 | 32.0 | 34.2 × 39.1 | 243,464 | 15–20 m 21%, 20–25 m 39%, 25–30 m 28% (and 13% below 15 m) |
| 199 | 47.5 | 38.7 | 41.1 × 42.2 | 180,300 | 20–25 m 23%, 25–30 m 29%, 30–35 m 27%, 35–40 m 6% (and 15% below 20 m) |
| 239 | 52.2 | 42.0 | 40.9 × 43.6 | 127,559 | 25–30 m 31%, 35–40 m 34% (and 22% below 25 m) |
| 279 | 54.9 | 40.0 | 40.2 × 43.2 | 57,820 | 30–35 m 32%, 35–40 m 23%, 5–10 m 12% |
| 299 | 56.0 | 40.1 | 37.1 × 43.0 | 39,976 | 35–40 m 33%, 5–10 m 22%, 30–35 m 21% |

**What the bands show** (arithmetic).
- **Seed 1001's leaf moves down after 159.** The share above 25 m falls from 34% (159) and 43% (199) to 11% (299), and the share at 15 to 20 m rises to 56%.
- **Seed 1007's leaf stays high but thins.** Its share at 5 to 10 m rises from 4% at 199 to 22% at 299.
- **All rough-layout heights.** These heights stand before the settle. The settled tree stands 23 m tall at 300 on 1001 (RENEW-RESULT.md).

## 5. Q on every preset and the oak's cost (measured)

**Sheet.** `q-others.jpg`: today beside Q, leafless then leafed; the beech at 40 and 80, and the birch, spruce, palm and ordinary at 40.

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |
|---|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 | 1.2 |
|  | Q | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,432 | 328,331 | 0.7 | 1.8 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 | 10.5 |
|  | Q | 17.5 | 18.8 × 14.5 | 109 | 4.7 | 3 | 217,924 | 1,719,507 | 2.2 | 14.6 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 | 32.4 |
|  | Q | 24.3 | 27.0 × 25.8 | 131 | 4.7 | 5 | 375,802 | 3,120,543 | 3.9 | 42.0 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 | 70.6 |
|  | Q | 29.3 | 50.0 × 50.7 | 128 | 4.7 | 5 | 344,012 | 2,845,517 | 3.8 | 111.6 |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 | 0.4 |
|  | Q | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,365 | 0.4 | 0.5 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 | 7.6 |
|  | Q | 17.4 | 19.6 × 20.2 | 132 | n/a | n/a | 478,977 | 2,049,430 | 2.8 | 8.6 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 | 1.4 |
|  | Q | 16.7 | 7.7 × 8.4 | 30 | n/a | n/a | 140,946 | 378,165 | 0.7 | 1.8 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 | 5.1 |
|  | Q | 12.4 | 7.8 × 7.3 | 35 | n/a | n/a | 4,406,389 | 1,211,003 | 2.0 | 6.5 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
|  | Q | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.6 |
|  | Q | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.8 |

**The one-thread seconds are not comparable as they stand** (measured; a caveat, not a finding).
- **Where today's come from:** fn-345's runs on 10 Nov, with fn-345's probe build.
- **Where Q's come from:** this build on 11 Nov.
- **The ordinary preset shows the gap.** Its tree is the same to the phytomer under both (92,345 held), yet it takes 0.8 s against 0.6.
- **What is needed:** a same-session, same-build pair to state Q's cost. That was not run in this pass.

**The read per preset under Q (eye, `q-others.jpg`).**
- **Beech 40: the same.** The same tree, leaves equal to the leaf (16,614).
- **Beech 80: the same.** The same dome, with 3% more leaves and 9% more held phytomers.
- **Birch 40: the same.** The same weeping birch with a clear white trunk (+1% leaves).
- **Spruce 40: the same.** +5% leaves, +6% held phytomers.
- **Palm 40: the same,** to the phytomer.
- **Ordinary 40: the same,** to the phytomer.

## 6. Questions for the host (none answered; no rule proposed)

1. **Lost whole while part pays.** A line is shed on its subtree's one balance. Most lines shed between 119 and 299 held paying children, about a third of them, and stood only 0.002 to 0.24 below zero. Is the unit the balance decides on (the whole lateral subtree) the cause of the whole-limb losses the stills show?
2. **The fork limbs never end; their tips do.** The fork limbs born in 13 and 14 live to 300 on 1001, but lay nothing from about 200 (1001) or 160 (1007). Their lines' apexes end by abortion, and each abortion is relayed until the relay's subtree goes. Is the limb whose tip has stopped, rather than the limb shed, the old tree's main loss?
3. **The droop is sag alone.** With sag off, the same limbs' tips stand 6 to 30 m above their bases at every age. With it on they lie on the ground from 160 to 200 on, and the stem line carrying the crown bends from about 16 m down to 4 m on 1007 at 200. On 1007 the settled tree is refused from 240 for a limb reaching under the ground. Is the sag's response to a heavy load out of scale in an old tree, or is the load (limbs 25 to 39 m long) the thing out of scale? Girth and sag are outside this spec's scope.
4. **Nothing comes back where a limb went.** 9 of 14 and 47 of 58 limb losses are followed by no framework birth on the bearer near the base in 20 years. The stem line's sleeping buds run out by about 220 to 243. Is renewal near a loss a matter for this goal, given that rule B (buds woken by light) changed almost nothing?
5. **The one-thread cost.** Should Q's cost be measured as a same-session pair against today's build before any of this is judged on cost?

## Friction

One entry in FRICTION.md: the first probe build panicked reading a bearer from inside a grow share.
