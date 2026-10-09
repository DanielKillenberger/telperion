# fn-213 sheet 9: round 9, arching limbs and irregular strands (worker, 2026-10-07)

Round 9 follows the host's reading of round 8:

1. The curtain should fall from arching outer limbs, outside the crown.
2. The bare interior should open so the white limbs show through a grey-brown haze.
3. The strands should be less parallel and even.

The same principles as round 8 apply: values only, none from the old generator, no new knobs.

## What moved (against round 8, `09279fba`)

| Row | Round 8 | Round 9 | Aimed at |
|---|---|---|---|
| `LIMB` set-point, `tropism`, `sag` | 1.3 rad, 0.9, 3e-5 | 1.15 rad, 0.7, 6e-5 | (1) the limb rises less steeply |
| `LIMB` lifespan (then `BOUGH`) | 25 years | 20 | (1) it passes sooner to its spreading age |
| `BOUGH` set-point, `tropism`, `sag` | 0.8 rad, 0.6, 3e-5 | 0.5 rad, 0.3, 8e-5 | (1) the old limb spreads and bends over under its load (the existing beam, `sag.rs`) |
| `LIMB` lateral `BRANCH` (medial, top); sleeping strand buds | 0.4, 0.5; 0.6 a node | 0.15, 0.4; none | (2) fewer strands on the limb's inner, older wood |
| `BOUGH` sleeping strand buds | 0.6 a node | 0.2 | (2) the same on old bough wood |
| Strand `wander`, `bendLength`, `internodeSpread` | 0.15, 0, 0.3 | 0.4, 1 m, 0.7 | (3) strands curve and their nodes spread unevenly |
| Strand `viability` | 0.96 | 0.92 | (3) more strands stop early: uneven ends |
| Strand lateral `BRANCH` (a fork) | none | 0.005 a node | (3) a rare fork, not branching systems |
| `/material/shootRed`, `Green`, `Blue` (wood under 3 cm) | 0.095 / 0.07 / 0.055 | 0.12 / 0.09 / 0.09 | (2) the haze's colour: code read S1's crown haze, the non-sky pixels of its upper crown, at sRGB 93/70/76, a purple-grey brown with blue above green; the row takes that hue, a little lighter (S3's upper crown reads 127/111/107) |

A first probe with `BOUGH` at 0.3 rad and sag 2e-4, and a fork at 0.02 a node (`raw/probe/q9a`), bent the limbs to the ground. It also cost 3.38M nodes and 4.6M leaves at 80 (seed 7), close to the browser's cap. The table's values are the second probe's (`q9b`).

## Measures (`birch_measure.rs`, seed 1; raw in `raw/measure-round9.txt`)

| Measure | Old | R8 @40 | R9 @40 | R8 @80 | R9 @80 |
|---|--:|--:|--:|--:|--:|
| Hanging strands / fine runs | 32,630 / 44,225 | 2,174 / 30,847 | 2,022 / 24,166 | 13,904 / 178,427 | 10,466 / 112,255 |
| Strand length p10 / p50 / p90 / max, m | 0.45 / 0.45 / 1.32 / 3.00 | 0.35 / 0.75 / 1.26 / 2.68 | 0.23 / 0.66 / 1.21 / 2.64 | 0.39 / 0.84 / 1.56 / 2.77 | 0.22 / 0.69 / 1.43 / 2.91 |
| Strand droop p10 / p50 / p90, deg | 40 / 40 / 90 | 35 / 53 / 69 | 37 / 57 / 73 | 38 / 60 / 76 | 44 / 68 / 81 |
| Fine runs per metre of wood ≥ 1 cm | 12.5 | 4.7 | 2.0 | 4.2 | 1.9 |
| Fine wood (< 5 mm), km | 30.1 | 4.5 | 3.5 | 27.6 | 17.3 |
| Leaves | 231,665 | 399,714 | 297,479 | 3,118,139 | 2,027,911 |
| Leaves nearest a hanging strand | 67% | 34% | 37% | 34% | 33% |

The strands now spread wider in length (p10 0.22 m against 0.35 to 0.39 m) and hang steeper. Twigs per metre of limb fell, as intended: the strand buds were thinned on inner limb wood, so the count per metre of all thick wood is lower.

## Cost (native, `--profile ci`, limits 8 threads)

| | Round 8 | Round 9 |
|---|--:|--:|
| Nodes at 40, seeds 1 / 7 / 42 | 353,160 / 688,759 / 425,062 | 280,893 / 541,478 / 329,116 |
| Nodes at 80, seeds 1 / 7 / 42 | 2,331,473 / 2,746,150 / 2,695,068 | 1,526,701 / 1,745,880 / 1,736,147 |
| Grown at 80, s | 0.94 / 1.16 / 1.00 | 0.78 / 0.90 / 0.96 |
| Leaves at 80 | 3.12M / 3.41M / 3.55M | 2.03M / 2.17M / 2.28M |

Round 9 is cheaper than round 8 at both ages and about round 7's node count at 80; the most is 1.75M nodes, under the 4M cap.

## The stills

`raw/compare/compare9-40.png` and `compare9-80.png`. For each of seeds 1, 7 and 42 there are three rows: the old birch, round 8 and round 9 (bare, in leaf, crown, limb). Round 9's stills are in `raw/sheet9/`. The worker viewed every still.

**The worker's reading.**

- **At 80:** the crown is a broad dome, 18 to 20 m across. The curtain falls from the outer parts of spreading limbs to near the ground, and in places pale limbs show through the bare crown (seed 7).
- **At 40:** the crown is still a narrow column with its fine wood bunched high on the stem.
- **Strands:** they curve slightly and end at varied lengths, but still hang in ranks.

## Astra's verdict

`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-round9.md`, asked as for round 8 (curtains, bare fine wood, crown shape; round 9 against round 8 and the old birch; leaf seating aside). Verbatim:

> ROUND 9 improves curtain coverage and variation, but still looks too combed and densely weeping against the photographs. These verdicts cover all three seeds; OLD is the pictured age-70 baseline, so its shape comparison is not age-matched.
>
> | Age | Trait | ROUND 9 | Evidence | vs ROUND 8 | vs OLD |
> |---|---|---|---|---|---|
> | 40 | Curtains | **Fail** | In `compare9-40.png`, ROUND 9 has long strands and more uneven crown coverage, but the limb views retain repeated hooked starts and straight, parallel drops, more mechanically arranged than the Uusimaa curtains in `refs.png`. | **Better** | **Worse** |
> | 40 | Bare fine wood | **Fail** | In `compare9-40.png`, all three ROUND 9 bare crowns concentrate dark fine wood around the upper stem, with sparse hanging frameworks below, lacking the distributed fine haze and readable pale limbs of S1 and S3 in `refs.png`. | **Same** | **Worse** |
> | 40 | Crown shape | **Fail** | In `compare9-40.png`, ROUND 9 is suitably narrow overall, but seed 7 has a forked upper outline and seed 42 remains a thin, uneven column rather than a coherent narrow egg. | **Same** | **Better** |
> | 80 | Curtains | **Fail** | In `compare9-80.png`, ROUND 9 clothes more of the crown with varied, long hanging tiers, but `sheet9/birch-80-1-limb.png` still shows ranks of nearly parallel strands and heavily drooping supporting wood that make the result more uniformly weeping than the references. | **Better** | **Worse** |
> | 80 | Bare fine wood | **Fail** | In `compare9-80.png`, ROUND 9 opens some views of pale limbs, especially seed 7, but `sheet9/birch-80-1-bare.png` confirms a dense descending web that overwhelms the airy branching haze visible in S1 and S3. | **Better** | **Worse** |
> | 80 | Crown shape | **Fail** | In `compare9-80.png`, ROUND 9 seeds 1 and 42 approach broad rounded crowns, but seed 7 remains tall, lopsided and divided into hanging tiers, so the rounded mature form is not consistent across seeds. | **Better** | **Worse** |
>
> The single most useful change would be to **break up the repeated comb of pendulous fine wood into unevenly spaced, unequal-length sprays with more directional variation**, keeping long curtains while allowing pale limbs and air to show between them.

## For the host

- **Crown at 80:** not narrow once the limbs arch (18 to 20 m across, a dome at seeds 1 and 42 by Astra's own words), so the round did not stop on it.
- **Improvement:** Astra rates round 9 better than round 8 on curtains at both ages, and on bare fine wood and crown shape at 80; the same on bare fine wood and crown at 40.
- **Still failing:** every trait, including crown shape at 40, which passed on sheets 7 and 8 and now fails for seeds 7 and 42.
- **The comb, named again** (sheets 8 and 9): "repeated hooked starts and straight, parallel drops", "ranks of nearly parallel strands". What gives them ranks, in the engine's terms:
  - **Hooked starts:** each strand leaves its bearer at the same insertion (0.7 rad) and turns down at the same tropism (3 per metre), so every strand starts with the same hook.
  - **Ranks:** the strands stand at the bearer's evenly spaced nodes along one bough.

  Values could vary each of these by degree (insertion, tropism, roll). Whether to try that, or to treat the comb as the stop it now is, is the host's call. No further values were tried.

## Gate (round 9, commit `a35e4f3b`)

Render half first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0. Then `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0. `harness/parity.test.ts` passes 8 of 8. Limits at 8; logs in the ignored `raw/gate9-*.log`.

## Code check before any round 10 (2026-10-07, host's question; no renders)

Does the engine draw a per-axis variation, keyed by lineage, for the strands' leaving angle, turning rate and roll? Read on the branch tip (`916b44ec`):

| Trait | Row | Per axis? | Where |
|---|---|---|---|
| Roll about the bearer | `form.roll` (per PA) | **Yes**: a uniform spread of ±`roll` radians, drawn per axis from `Key(lineage).child(ROLL)` | `geometry.rs:177-178`, `geometry/single.rs:84`; documented at `species.rs:220-223` |
| Leaving angle | `insertion` (per PA) | **No**: one constant per age, passed unchanged to `turned(...)` for every lateral and relay | `geometry.rs:183`, `geometry.rs:208`, `geometry/single.rs:89`; also read as a constant by the closed form (`closed_form/events.rs:156`, `236`, `268`; `kept/path.rs:256`) |
| Turning rate | `form.tropism` (per PA), toward `form.elevation` (per PA) | **No**: `share = 1 - exp(-tropism × length)` with the PA's constant; only `length` varies, through `internodeSpread` | `geometry/lay.rs:148-150` |

Per-axis or per-node keyed variation that does exist, and touches the strands only indirectly:

- Internode length (`form.internodeSpread`, per node, `lay.rs:148`).
- Wander (`form.wander`, a keyed turn per node, `lay.rs:155-160`) and its remembered bend (`form.bendLength`).
- Sibling size (`form.dominance`).
- Survival (`viability`).

Round 9 already uses roll at 1.2 rad on the strands. A per-axis spread of the leaving angle and of the turning rate does not exist; per the host's instruction the worker stops here.
