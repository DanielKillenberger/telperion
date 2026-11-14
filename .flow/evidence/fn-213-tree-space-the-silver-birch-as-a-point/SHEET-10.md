# fn-213 sheet 10: round 10, strand-to-strand variation (worker, 2026-10-07)

F4 was merged at `b31a4f65` (fn-224: `form.insertionSpread`, `form.tropismSpread`), with the birch stating both at 0 in the merge commit (`2c3281f6`, digests re-pinned).

Round 10 then aimed at three things, values only and no old-generator numbers:

- **The comb:** the new spreads on the strands, with roll and the existing internode spread.
- **The 40-year egg:** restore round 8's.
- **Round 9's arching:** keep it.

Per the host, this sheet goes to the owner whatever Astra says.

## What moved (against round 9, `a35e4f3b`)

| Row | Round 9 | Round 10 | Aimed at |
|---|---|---|---|
| Strand (`/space/ages/9`) `form/insertionSpread` | 0 (absent) | 0.5 rad | Each strand leaves at the age's 0.7 rad ± up to 0.5 |
| Strand `form/tropismSpread` | 0 (absent) | 0.6 | Each strand turns down at 3 × (1 ± 0.6) per metre: some hook tight, some arch wide |
| Strand `form/roll` | 1.2 rad | 1.6 rad | Strands leave at more varied bearings about the bough |
| `LIMB` set-point, `tropism`, lifespan | 1.15 rad, 0.7, 20 years | 1.3 rad, 0.9, 25 years (round 8's) | The 40-year egg: young limbs steep again; the old limbs still spread and bend over as `BOUGH` (round 9's 0.5 rad, sag 8e-5) |

Strand internode spread stays 0.7. All values are **estimated**.

## Measures (`birch_measure.rs`, now with each strand's leaving elevation and its drop over its first 0.3 m; raw in `raw/measure-old-r10.txt`, `measure-round9-r10.txt`, `measure-round10.txt`)

Seed 1.

| Measure | Old | R9 @40 | R10 @40 | R9 @80 | R10 @80 |
|---|--:|--:|--:|--:|--:|
| Hanging strands / fine runs | 32,630 / 44,225 | 2,022 / 24,166 | 1,912 / 24,159 | 10,466 / 112,255 | 12,335 / 136,349 |
| Strand length p10 / p50 / p90, m | 0.45 / 0.45 / 1.32 | 0.23 / 0.66 / 1.21 | 0.23 / 0.67 / 1.22 | 0.22 / 0.69 / 1.43 | 0.23 / 0.71 / 1.46 |
| Leaving elevation p10 / p50 / p90, deg | −90 / −40 / −40 | −51 / −9 / 35 | −48 / −7 / 37 | −62 / −29 / 20 | −64 / −28 / 22 |
| Drop over first 0.3 m p10 / p50 / p90, deg per m | 0 / 0 / 0 | 83 / 170 / 258 | 83 / 171 / 268 | 53 / 125 / 246 | 46 / 127 / 253 |
| Strand droop (chord) p50, deg | 40 | 57 | 56 | 68 | 67 |
| Fine runs per metre of wood ≥ 1 cm | 12.5 | 2.0 | 2.0 | 1.9 | 2.0 |
| Fine wood (< 5 mm), km | 30.1 | 3.5 | 3.5 | 17.3 | 21.1 |
| Leaves on hanging strands | 67% | 37% | 36% | 33% | 33% |

**What the measures say:**

- **The old strands:** they leave straight at one angle and never turn (drop 0 everywhere). Its curtain varies only in length.
- **Round 10 against round 9:** the strand-to-strand spread of leaving angle and drop rate is nearly unchanged: p10 to p90 widens by a few degrees. Round 9's strands were already widely spread by their bearers' own orientation and roll, which the measure includes. So the new spreads move the distribution little at these values.

## Cost (native, `--profile ci`, limits 8)

| | Round 9 | Round 10 |
|---|--:|--:|
| Nodes at 40, seeds 1 / 7 / 42 | 280,893 / 541,478 / 329,116 | 280,885 / 535,595 / 335,228 |
| Nodes at 80, seeds 1 / 7 / 42 | 1,526,701 / 1,745,880 / 1,736,147 | 1,845,346 / 2,155,356 / 2,108,348 |
| Grown at 80, s | 0.78 / 0.90 / 0.96 | 1.07 / 1.52 / 1.33 |
| Leaves at 80 | 2.03M / 2.17M / 2.28M | 2.48M / 2.68M / 2.79M |

The most is 2.16M nodes, under the 4M cap. The rise at 80 comes from the longer-lived steep limbs (25 years against 20), not the spreads: at 40 the node counts are unchanged.

## The stills

`raw/compare/compare10-40.png`, `compare10-80.png`. For each of seeds 1, 7 and 42 there are three rows: the old birch, round 9 and round 10 (bare, in leaf, crown, limb). Round 10's stills are in `raw/sheet10/`. The worker viewed every still.

**The worker's reading.**

- **At 80:** much as round 9. Seed 1 is a weeping dome with fuller overlapping curtains; seed 7 is tall; seed 42 is columnar.
- **At 40:** the crown did **not** come back to round 8's egg. It is 12 to 13 m wide at seeds 1 and 42 against round 8's 7.5 m: restoring the limb rows alone did not do it. The cause is not established. Round 9's other changes all act at 40 too:
  - the strands' wander, bend and rare forks;
  - the thinned limb strands;
  - the bough's set-point and sag.

  The worker tried no further values.
- **Strands:** they still hang as similar hooks and drops in the limb close-ups.

## Astra's verdict

`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-round10.md`, asked as for rounds 8 and 9 (curtains, including hooks and drops varying strand to strand; bare fine wood; crown shape; round 10 against round 9 and the old birch; leaf seating aside). Verbatim:

> ROUND 10 still fails the three traits as a set across seeds 1, 7 and 42. Leaf seating is excluded. The OLD rows are labelled age 70, so their shape comparison is against the requested silhouette, not an age-matched control.
>
> | Age | Trait | ROUND 10 | Evidence | vs ROUND 9 | vs OLD |
> |---|---|---|---|---|---|
> | 40 | Curtains | **Fail** | `compare10-40.png` shows long, straight drops clothing the upper crown, but the limb views repeat similar rounded hooks and parallel strands, with less irregular layering than the Uusimaa photograph in `refs.png`. | **Same** | **Worse** |
> | 40 | Bare fine wood | **Fail** | In `compare10-40.png`, all three bare crowns concentrate into dark upright tangles with exposed hanging wires below, whereas S1/S3 in `refs.png` show readable pale limbs spreading through an airy fine-wood haze. | **Same** | **Worse** |
> | 40 | Crown shape | **Fail** | `compare10-40.png` is suitably narrow, but seeds 1 and 42 read as elongated, uneven columns and seed 7 as a top-heavy fan, rather than a consistently filled narrow egg. | **Same** | **Better** |
> | 80 | Curtains | **Fail** | `compare10-80.png` shows fuller overlapping curtains, especially on seed 1, but `sheet10/birch-80-1-limb.png` still exposes repeated arch-and-vertical-drop geometry rather than the varied hanging clusters in the Uusimaa reference. | **Better** | **Worse** |
> | 80 | Bare fine wood | **Fail** | In `compare10-80.png`, the dark fine wood overwhelms much of the pale framework, particularly on seed 42, while OLD preserves clearer white branching and S1/S3 show more open space between fine sprays. | **Same** | **Worse** |
> | 80 | Crown shape | **Fail** | `compare10-80.png` brings seed 1 toward a rounded crown, but seed 7 remains tall and lopsided and seed 42 remains columnar, missing the broad rounded mass shown by the Oslo photograph in `refs.png`. | **Same** | **Worse** |
>
> The single most useful change would be **greater strand-to-strand variation in pendant geometry**: vary where each strand hooks downward and how far it drops, breaking the repeated comb pattern into irregular, overlapping curtains.

**In short:** every trait fails at both ages. Against round 9 it is the same on everything but curtains at 80, which are better. Astra's change is the one this round aimed at: "greater strand-to-strand variation in pendant geometry".

## For the host and the owner

- **The spreads:** fn-224's insertion spread (0.5 rad) and tropism spread (0.6) made little measurable or visible change at these values. Round 9's strands already spread from −50° to +35° in leaving elevation and 80 to 260° per metre in drop.
- **The repeated look:** what the stills still show as repeated is the strands' shape, not their parameters. Each one leaves, turns down within about 30 cm (base tropism 3 per metre) and hangs straight. Larger spreads are within the rows' bounds (insertion to π/2, tropism to 1) and untried.
- **The 40-year egg:** not restored. The cause is unknown (see the stills section).
- **The preset:** carries round 10. Round 9's values are in its commit `a35e4f3b` if the owner prefers that sheet.

## Gate (round 10, commit `bdfe3ff7`)

Render half first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0. Then `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0. `harness/parity.test.ts` passes 8 of 8. Limits at 8; logs in the ignored `raw/gate10-*.log`. The scratch worktree for the old birch's measures was created again and removed.
