# fn-213 sheet 6: round 6, the crown at 80 only (worker, 2026-10-07)

Round 6 tests the host's reading of (c) on sheet 5 (SHEET-5.md, "The host's decisions"): the leader that never ends keeps the 80-year crown elongated. It uses existing mechanisms only, leaves the leaves alone (fn-223), and is judged on crown shape only.

## What changed (against round 5, `e12645b1`)

| Row | Round 5 | Round 6 | Why |
|---|---|---|---|
| `TOP` (the stem from year 41) `abortion`, `relay*`, `relayAt`, `epitony` | none | 0.8 a unit, relay 1, at 0.6 of the unit, epitony 0.2 | The old stem's tip dies most years and a bud below it, barely turned upward, carries on: the top becomes small forks (MBE99) |
| `TOP` `tropism`, set-point | 1.0, π/2 | 0.4, 1.0 rad | Its relays slant rather than straighten into a new leader |
| `TOP` lateral `BOUGH` at the unit's top | 0.3 | 0.5 | More small forks at the old top |
| `LIMB` lifespan (then `BOUGH`) | 15 years | 10 | Limbs pass sooner into their older, flatter age |
| `BOUGH` set-point | 0.8 rad | 0.5 rad | Limbs flatten with age |

A first probe with `abortion` 0.5, the `TOP` changes alone and `BOUGH` at 0.6 rad (`raw/probe/q6a`) left the crowns as tall as round 5's; the table's values are the second. The 40-year tree keeps its leader (`TOP` starts at 41). The species diff is `raw/round6.diff`, the regenerated rows `raw/round6-values.diff`.

## The stills

`raw/sheet6/sheet-80.png` and `sheet-40.png`, the earlier sheets' layout (references, then seeds 1, 7, 42: bare, in leaf, crown, limb). Viewed by the worker.

| Age | Seed | Height | Crown width |
|---|---|--:|---|
| 40 | 1 | 16.8 m | 7.8 × 8.5 m |
| 40 | 7 | 16.8 m | 8.7 × 7.3 m |
| 40 | 42 | 16.7 m | 9.1 × 9.9 m |
| 80 | 1 | 21.5 m | 17.0 × 21.1 m |
| 80 | 7 | 23.6 m | 18.8 × 18.5 m |
| 80 | 42 | 21.8 m | 18.8 × 21.0 m |

**The worker's reading.** The 80-year tops now end in small slanting forks and the trees stand 1 to 2 m lower. The crowns are still taller than wide in their upper half, though: the width comes from long low and mid limbs, while the upper crown stays a narrow column around the stem. Shortening the limbs' first age also made the crowns sparser. (c) does not read as fixed.

## Astra's verdict (crown shape only)

`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-sheet6.md`. The prompt said leaf seating is a known gap under fn-223 and asked about crown shape alone. Verbatim:

> | Age | Seed | Verdict | Crown-shape evidence |
> |---|---|---|---|
> | 40 | 1 | PASS | `birch-40-1-whole.png` shows an elongated, narrow egg overall, with tapering shoulders despite an uneven lower-right extension. |
> | 40 | 7 | PASS | `birch-40-7-whole.png` has a narrow ovoid envelope, with its middle spreading beyond the tapering upper crown. |
> | 40 | 42 | FAIL | `birch-40-42-whole.png` has a pronounced separate right-hand limb lobe and a tall left-hand apex, making a divided, lopsided outline rather than a narrow egg. |
> | 80 | 1 | FAIL | `birch-80-1-bare.png` shows upper forks, but their steep, projecting tips cap a long, narrow crown without broad, rounded shoulders. |
> | 80 | 7 | FAIL | `birch-80-7-bare.png` retains a strongly vertical central axis and an isolated high terminal shoot above a columnar crown. |
> | 80 | 42 | FAIL | `birch-80-42-bare.png` shows a tall, leaning crown with low lateral spread and a projecting upper shoot, leaving the upper outline narrow and pointed. |
>
> **Overall crown shape: FAIL.** All three 80-year specimens lack the broad, rounded, limb-built upper crown required for mature open-grown birch; spreading low branches do not supply that upper breadth. The 40-year seed 42 also misses the narrow-egg target. Leaf seating and leaf mass were excluded from these judgments.

## Wrong-path stop: (c) is architecture

(c) has failed on sheets 4, 5 and 6, and round 6 tested the host's new cause with existing mechanisms. Per the host's instruction the worker stops; the call is the host's.

What the sheet shows about the cause: ending the leader does fork the top, but nothing then builds a broad upper crown in its place. The width at 80 sits low, in long limbs from the stem's middle. The upper limbs come from the stem's last decades and stay short and steep, because a limb's length is set by its age's lifespan and internode, not by where it stands or how much light it gets. In S1, S2-Oslo and S3 the upper crown is made by a few large limbs that rose from mid-stem and kept growing for decades into the top.

Candidate directions for the host, none tried:

- A few dominant scaffold limbs from the leader's middle years that live and grow longest, as the oak's reiterated limbs do.
- Light (fn-197's `shade_size`, `apical_control`), so upper, sunlit limbs outgrow lower ones.

**The preset is left at round 5** (`e12645b1`, the strongest sheet; round 6 also made 40-year seed 42's crown lopsided). Round 6's values live only in the two diffs in `raw/`. No product file has changed since round 5, so round 5's gate (SHEET-5.md, "Gate") stands for the branch tip.
