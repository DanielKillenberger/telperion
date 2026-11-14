# A5 diagnosis: why the settled old oak's limb line draws at scale 0, 12 Nov 2026

Oregon white oak, seed 1001, unchanged preset (`values/base.values` = census-p5's values file, checked by `cmp` before each build). Checkout `census-p5` at `487266f8f` plus uncommitted readers. Measurement only; no design judgment is made here.

How each number was read:

- [assign]: `presence::assign` run on a settled copy of the grower (`Grower::settled_copy`, then `mark`), as `settle_shed` runs it.
- [shed]: `shed::shed_past` run on those assigned axes with the retirement's fates, exactly as `settle_shed` runs it, and its `lived` and `alive` vectors captured from `fades`.
- [settled]: `Paused::settle()`, the tree the pipeline draws.
- [grower]: the grower's own stores.
- All from one grower grown year by year to 300 (`Retiring::AtAge`), read between years. `windows.rs` says that nothing a draw reads depends on the age asked for. This run confirms the 250 values of the earlier run asked at 250 (vigour 1.0 at 250).

The line: the limb lateral born 14 (lineage `15990d2e291cd478`), its relays born 15, 26 and 27, on the fork (born 12) and the trunk.

## 1. The term that takes the line from 1.0 to 0.13

It is not `assign_one`. Every factor `assign_one` and `assign` read stays the same from 240 to 300 [assign]:

| axis | birth draws (raw = after `mark`) | units with survive or persist < 1 | woken (`mark`'s kept) | parent's end presence | own end | vigour assigned |
|---|---|---|---|---|---|---|
| lateral 14 | 0.7888 x 1.0 | none | not woken | (lateral: not read) | 1.0 | 0.7888 |
| relay 15 | 1.0 x 1.0 | none | not woken | 1.0 | 1.0 | 1.0 |
| relay 26 | 1.0 x 1.0 | none | not woken | 1.0 | 1.0 | 1.0 |
| relay 27 | 1.0 x 1.0 | none | not woken | 1.0 | 1.0 | 1.0 |

These were the same in all 61 years read (244 axis-years) [assign]. Fork and trunk: 1.0 throughout. The windows enter only through the stored unit and birth presences, and those do not move. The lateral's 0.7888 is its birth draw.

The value moves in the shed pass that the settle runs after assign:

- `shed.rs:139`: `axis.vigour *= fade[i];` (`grow/settle.rs:62-71` runs `assign`, then `shed_past`, then `settle_lay` → `geometry::scale`).
- `contract/presence.rs:102-105`: `fade = clamp((delay + 1) - idle, alive.min(1), 1)`, with `idle = age - birth - lived`.
- `lived` and `alive` are carried up from children:
  - `shed.rs:242-253`: weight = the child's vigour, times its node's phytomer scale for a lateral (`:246-249`).
  - `shed.rs:216-232`: `up`. The lived time is carried by `carry(before, weight, gap, lived) = before + weight * (gap + lived)` (`contract/presence.rs:94-96`). The living apex's presence is carried by `alive x weight`. Each axis takes the maximum over its children and stands.
- Delay: limb `/space/ages/6/shedding = 5.0`, so delay + 1 = 6. Bough (age 7) 5.0, so 5.

The fade terms by year [shed]:

| age | axis | lived | idle | (delay+1) - idle | alive | fade | vigour after shed [settled] |
|---|---|---|---|---|---|---|---|
| 250 | lateral 14 | 236 | 0 | 6 | 1.0 | 1.0 | 0.7888 |
| 250 | relay 15 | 235 | 0 | 6 | 1.0 | 1.0 | 1.0 |
| 250 | relay 26 | 224 | 0 | 6 | 1.0 | 1.0 | 1.0 |
| 250 | relay 27 | 223 | 0 | 6 | 1.0 | 1.0 | 1.0 |
| 260 | 14 / 15 / 26 / 27 | 246 / 245 / 234 / 233 | 0 | 6 | 1.0 | 1.0 | 0.7888 / 1 / 1 / 1 |
| 270 | 14 / 15 | 256 / 255 | 0 | 6 | 1.0 | 1.0 | 0.7888 / 1.0 |
| 270 | 26 / 27 | 236 / 235 | 8 | -2 | 0.6080 | 0.6080 | 0.6080 |
| 280 | 14 / 15 | 266 / 265 | 0 | 6 | 1.0 | 1.0 | 0.7888 / 1.0 |
| 280 | 26 / 27 | 236 / 235 | 18 | -12 | 0.6080 | 0.6080 | 0.6080 |
| 290 | 14 / 15 | 274 / 273 | 2 | 4 | 0.6080 | 1.0 | 0.7888 / 1.0 |
| 290 | 26 / 27 | 236 / 235 | 28 | -22 | 0.6080 | 0.6080 | 0.6080 |
| 300 | 14 / 15 | 274 / 273 | 12 | -6 | 0.1320 | 0.1320 | 0.1042 / 0.1320 |
| 300 | 26 / 27 | 236 / 235 | 38 | -32 | 0.1320 | 0.1320 | 0.1320 |

So each axis's fade has fallen to its floor, `alive`. That floor is the presence of its most present living apex, relative to the axis. At 300 it is 0.1320 on every axis of the line [shed, descent along the child carrying the most `alive` up]:

- bough lateral born 148: vigour 0.6080 (its birth draws [assign]), node scale 1.0;
- then relays at weight 1.0;
- then the branch lateral born 237: vigour 0.3004 [assign], node scale 0.7230;
- 0.6080 x 0.3004 x 0.7230 = 0.1320;
- below it, weight 1.0 to a living short born 300, own presence 1.0.

The lived time stops because the living wood below the line sits behind weights under 1:

- A bough lateral born 148 (weight 0.6080) whose subtree lived to 292 carries into relay 146 only `2 + 0.608 x 144 = 89.55` cycles past 146 [shed: relay 146 lived 89.55, idle 56.45 at 292].
- The line's lived time therefore follows only full-weight wood. When that wood stops living, idle grows by one each year, even though the subtree still holds living tips.

Phytomer scale then compounds down the relays in `geometry::scale` (question 4) [settled, 300]:

| axis | scale |
|---|---|
| lateral 14 | 0.1066 |
| relay 15 | 0.0134 |
| relay 26 | 0.0017 |
| relay 27 | 0.0002 to 0.0003 |

## 2. The year it leaves 1.0, and what changed

Settled vigour on the line, read every year from 240 to 300 [settled]:

| years | lateral 14 | relay 15 | relays 26, 27 |
|---|---|---|---|
| 240 to 267 | 0.7888 | 1.0 | 1.0 |
| 268 to 292 | 0.7888 | 1.0 | 0.6080 |
| 293 | 0.7888 | 1.0 | 0.1320 |
| 294 to 300 | 0.1042 | 0.1320 | 0.1320 |

What changed:

- **268 (relays 26 and 27).** Their subtree's full-weight living time stopped at 26 + 236 = 262 [shed: lived 236, unchanged from 267 to 300]. Idle reached 6 at 268, the fade's time term reached 0, and the fade fell to the floor `alive` = 0.6080. That is the bough lateral born 148, now their most present living descendant.
  - Which apex ended at 262 was not read [U]. The reader prints only the most-`alive` and latest-`lived` descents, not the axis whose death set lived = 236.
- **288, felt at 294 (lateral 14 and relay 15).** On relay 15, the bough line born 21 had a living apex at presence 1.0 up to 280 [shed: alive 1.0, lived 259 at 280]. At 290 nothing in it lives: alive 0, lived 267, so its last living year was 21 + 267 = 288. Its descent at 290 runs through 39 bough relays, born 26 to 116 and on, all weight 1.0. That was the line's last full-weight living subtree.
  - From then on the line's lived time stays at 14 + 274 = 288 and idle grows one a year.
  - At 294 idle = 6, the time term = 0, and the fade = `alive`.
- **293 (the floor itself drops from 0.6080 to 0.1320).** Inside the bough lateral born 148, the most present living apex fell from 1.0 at 292 to 0.2172 at 294 [shed]. 0.2172 = 0.3004 x 0.7230 (branch born 237).
  - So a living apex at full relative presence below bough 148 stopped in year 293 [shed; which apex: U].
  - That year relays 26 and 27, already on the floor, go to 0.6080 x 0.2172 = 0.1320. Lateral 14 and relay 15 follow at 294, once their own idle reaches 6.
- **No other trigger on the line [assign, grower].**
  - All four axes stay `Open` in the retirement through 300. None is a retired root, and no stand sits on them (stands on them: 2, 31 to 33, 2, 18 to 19).
  - No birth draw, unit presence or woken share moves.
  - In the grower the line keeps base scale 0.7888 and running and kept presence 1.0 (earlier GCHAIN log).

## 3. Other trees: living tips drawn under 0.01 while the grower holds them above 0.1

Living tips are matched to the settled tree by (lineage, birth) [grower, settled].

- "unmatched" counts tips with no settled axis under that key. They are about 40% everywhere, and the earlier probe matched 60% by lineage alone.
- Why they do not match is unknown [U]: shed in the settle, or a birth that differs.
- "ambiguous" counts keys held by more than one settled axis; they are left out.
- "both" counts tips whose grower last-phytomer scale is > 0.1 and whose settled last-phytomer scale is < 0.01. It is a share of the matched tips.

| seed | age | living tips | matched | unmatched | ambiguous | grower > 0.1 | settled < 0.01 | both | both / grower > 0.1 |
|---|---|---|---|---|---|---|---|---|---|
| 1001 | 120 | 189,672 | 112,953 | 76,579 | 140 | 101,518 | 8,607 | 2,411 | 2.4% |
| 1001 | 160 | 126,891 | 76,691 | 50,119 | 81 | 68,175 | 8,101 | 2,849 | 4.2% |
| 1001 | 200 | 55,499 | 33,697 | 21,766 | 36 | 30,803 | 2,690 | 952 | 3.1% |
| 1001 | 220 | 35,685 | 21,459 | 14,193 | 33 | 19,017 | 2,238 | 694 | 3.6% |
| 1001 | 240 | 15,259 | 9,506 | 5,743 | 10 | 7,725 | 1,874 | 414 | 5.4% |
| 1001 | 250 | 8,955 | 5,702 | 3,249 | 4 | 4,313 | 3,605 | 2,369 | 55% |
| 1001 | 260 | 7,982 | 4,922 | 3,056 | 4 | 3,628 | 3,543 | 2,265 | 62% |
| 1001 | 280 | 5,711 | 3,599 | 2,109 | 3 | 2,520 | 3,383 | 2,304 | 91% |
| 1001 | 300 | 4,334 | 2,630 | 1,702 | 2 | 1,963 | 2,630 | 1,963 | 100% |
| 1007 | 120 | 295,913 | 170,011 | 125,593 | 309 | 149,849 | 18,972 | 5,296 | 3.5% |
| 1007 | 160 | 209,471 | 122,130 | 87,146 | 195 | 103,834 | 17,145 | 4,834 | 4.7% |
| 1007 | 200 | 132,423 | 77,762 | 54,559 | 102 | 62,409 | 13,782 | 2,996 | 4.8% |
| 1012 | 120 | 244,453 | 141,026 | 103,178 | 249 | 122,859 | 14,630 | 3,912 | 3.2% |
| 1012 | 160 | 206,151 | 118,485 | 87,479 | 187 | 101,342 | 14,043 | 4,349 | 4.3% |
| 1012 | 200 | 171,947 | 98,581 | 73,184 | 182 | 79,844 | 15,960 | 4,500 | 5.6% |

Seeds 1007 and 1012 at 200 show the same kind of loss, at 3 to 6% of their large living tips: a living tip held large by the grower and drawn under 0.01. That is the share seed 1001 had before 240. Neither reaches seed 1001's 55 to 100% after 250. Whether their lost tips sit behind the same idle-at-the-floor fade was not read [U]; the descent reader was run only on seed 1001's limb line.

## 4. What the code does today (no answer to whether it should)

**Where a relay's scale takes vigour, and whether it compounds.**

- `geometry.rs:76-81` (`geometry::scale`): `base_scale[i] = inherited x vigour x share`, with `Origin::Continuation | Origin::Relay => base_scale[parent]` (`:79`).
  - A relay inherits its parent's base scale, which already holds the parent's vigour, and multiplies its own vigour onto it.
  - A relay of a relay multiplies again: with n relays at vigour v, the base scale holds v^(n+1) of the lateral's.
- The phytomers are then `p.scale x base` (`:89`) and `x size` (`:97`).
- `presence.rs:183-192` (`assign`): a relay's assigned vigour is `end[parent] x made`, its parent's end presence (the running product of its unit presences, `assign_one`, `presence.rs:215-241`), not its parent's vigour. So `assign` itself does not compound vigour down a relay chain.
- The compounding factor is the shed fade: `shed.rs:139` multiplies every kept axis's vigour by its own fade, before `scale` reads it (`grow/settle.rs:62-71`, then `:121`).
  - Each relay of a line whose subtrees share one living takes the same fade, here 0.1320.
  - `geometry::scale` multiplies it once per relay.
- In the fades, a relay's weight carried up is its vigour alone, with no node scale (`shed.rs:246-249`). A lateral's weight is its vigour times its node's phytomer scale.

**What the documents say a relay's presence and vigour are meant to be (quoted).**

- `structure.rs:126-128`, `Axis::vigour`: "The axis's size at birth relative to what bears it, 0 to 1: a branch a setting has just made, or is about to unmake or shed, is small."
- `structure.rs:129-133`, `blend`: "A relay's place between the continuation it replaces (0) and its own bud's place and heading (1): the presence of the stop that made it."
- `presence.rs:1-8`: "Presence (fn-192 R2): how grown each element a draw made is, from 0 where a setting crosses the draw to 1 once the setting is a window past it. … a branch made anywhere moves no other draw's window."
- `grow/stop.rs:171-177`, `relay_bud`: "It carries on the axis's lineage, its time in its PA and the time the axis has grown, which its abortion hazard counts, so a relay laid as the continuation grows what the apex would have (host decision 11)."
- `closed_form.rs:8-13`: "Abortion after a growth unit is a second survival factor; a stopped apex's relay bud is one more substructure of its PA, one that carries on the growth units its axis spent. … Shedding is outside the formula."
- `closed_form.rs:41-42`: "an apex that all but stopped moving on grows its continuation and its relay, which count whole (host decision 13)."
- `shed.rs:1-7`: "How big a kept one is fades with its subtree's living time, each growth unit's time weighted by its presence, so a branch whose last growth a setting is unmaking shrinks away before it is shed; it is dropped once its fade reaches nothing, which for whole presences and delays is the cycle GreenLab drops it in (fn-206)."
- `shed.rs:157-160`, `fades`: "Each axis's size factor from shedding: 1 for a living subtree, falling to 0 as its idle time, measured in presence-weighted cycles, passes the delay, but never below its most present living apex. Where every presence is whole it is 1 for every kept axis."
- `contract/presence.rs:98-101`, `fade`: "D17: an axis's fade, its delay plus a cycle `delay`, born in `birth`, its subtree living `lived` cycles past its birth and its most present living apex `alive`, at the tree's `age`: the delay left, clamped to that apex and to one. Shed where it is none."
- `docs/pipeline.md:63`: "A closed lateral whose fade at the age grown is none is retired as the tree grows (fn-255) … Nothing reads a retired root but its bearer's fade …"
- `docs/pipeline.md:67`: "a phytomer's radius is `sqrt(section × share / π)`, `share` the fade its axis is drawn at, and a relay joins at the radius of the wood it stands in".
- `FINE-CONTRACT.md` (fn-209 evidence), host decision 8, lists "each axis's vigour" among the floored products. Its D17 is the fade above.
- `docs/tree-space.md` holds no passage defining a relay's presence, vigour or fade (searched for "presence", "relay", "fade", "shed": none on those terms).

Observed against the `fades` doc [shed]: the line's subtree holds living apexes at every year read. Its fade is still below 1 from 268 (relays 26 and 27) and 294 (lateral 14 and relay 15), because the presence-weighted idle time passes the delay.

## Commands

```
# readers added (uncommitted, census-p5):
#   crates/telperion-space/src/grow/cycle/a5diag.rs        (new: chain factors, fade terms, descents, living tips)
#   crates/telperion-space/src/grow/cycle.rs               (+ mod a5diag)
#   crates/telperion-space/src/grow/cycle/paused.rs        (+ Paused::a5diag, Paused::a5_tips)
#   crates/telperion-space/src/shed.rs                     (+ A5_ON / A5_CAPTURE: fades' lived/alive copied when asked)
# probe: $S/probe/src/diag.rs (new), main.rs (+ "diag")
bash $S/build.sh diag        # telperion-heavy 8 --threads 16 -- cargo build --profile ci --manifest-path $S/probe/Cargo.toml --target-dir $S/probe-target
bash $S/batch-f.sh           # diag.sh: telperion-heavy 16 --threads 16 -- python3 $S/peak.py $S/bin/probe-diag diag oregon-white-oak <age> <seed> <lineage|-> <diag years> <count years>
                             #   1001 to 300, diag 240..300 yearly, counts 120,160,200,220,240,250,260,280,300
                             #   1007 and 1012 to 200, counts 120,160,200
bash $S/batch-g.sh           # rebuilt with the fade reader; 1001 to 300, diag 250,260,267,268,270,280,290,292,293,294,300
python3 -I $S/a5d.py $S/logs/diag-1001-300.log sline 14,15,26,27
python3 -I $S/a5d.py $S/logs/diag-1001-300.log axes 0,12,14,15,26,27 267,268,292,293,294
grep '^A5F\|^A5P' $S/logs/diag-1001-300-fade.log
```

Logs: `$S/logs/diag-1001-300.log`, `diag-1007-200.log`, `diag-1012-200.log`, `diag-1001-300-fade.log` (and `.err`).

Cost:

| job | seconds | peak memory |
|---|---|---|
| build | 24 | - |
| rebuild | under 1 min | - |
| 1001 to 300, 70 settles | 117 | 2.1 GiB |
| 1007 to 200 | 143 | 3.4 GiB |
| 1012 to 200 | 107 | 2.7 GiB |
| fade run | 48 (growth and reads) | 1.4 GiB |

- One queue wait of 30 s beside the running gate.
- Nothing was killed.
- Friction: the shell hook refused a perl one-liner in a pipe (dcg `heredoc.posix:pipeline-consumer`), so the table was produced with a script file (`a5d.py`). It cost about a minute.
