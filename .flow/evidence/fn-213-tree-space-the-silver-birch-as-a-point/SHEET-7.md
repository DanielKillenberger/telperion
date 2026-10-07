# fn-213 sheet 7: round 7, scaffold limbs (worker, 2026-10-07)

Round 7 follows the host's decision on sheet 6: a few dominant limbs from the leader's middle years that live and grow longest, rise steeply at first and spread with age, with round 6's old-stem forking kept. Only crown shape is judged, at 40 and 80; leaf seating is fn-223's known gap.

## The mechanism: existing, value-level

The oak's repeated limbs (`oak.rs`) are a long-lived limb age (`LIMB`, lifespan 500, sag) borne sparsely, at a high probability over a short stretch of the stem (its fork age), each limb kept by high viability. No code: every piece is a value of an existing age. The birch reaches the same structure with these rows:

| Rows | Round 5 | Round 7 | Role |
|---|---|---|---|
| New stem age `/space/ages/3` (`LOW_FORK`, the young leader, years 6 to 14): `lifespan` 9, zones bearing `SPRIG` 0.25 / 0.6, `LIMB` 0.1 at the top, `BRANCH`, `SHORT`; `/space/ages/3/trunk = true` | (the leader began at year 6) | new | The young crown: passing branches the bole sheds, a few low scaffolds |
| `/space/ages/4` (`LEADER`, years 15 to 40) `lifespan` | 35 | 26 | The stem's middle years |
| `/space/ages/4/zones/1/lateral/6`, `.../zones/2/lateral/6` (`LIMB` from the leader) | 0, 0.3 | 0.04, 0.22 | Fewer limbs, the scaffolds, with passing branches (`SPRIG` 0.25, 0.45) between them |
| `/space/ages/6/lifespan` (`LIMB`: steep at its 1.3 rad set-point) | 15 | 25 | Scaffolds rise steeply for 25 years, then carry on as `BOUGH` (0.8 rad), spreading with age |
| `/space/ages/6/viability` | 0.993 | 0.998 | Scaffolds survive for decades |
| `/space/ages/8/lifespan` (`SPRIG`) | 8 | 14 | Passing branches keep the 40-year crown full between the scaffolds |
| `/space/ages/5/*` (`TOP`): `abortion` 0.8, relays at 0.6, `epitony` 0.2, set-point 1.0 rad at `tropism` 0.4, top-zone `BOUGH` 0.5 | none | as round 6 | The old stem ends in small forks (MBE99) |
| Stem (`/space/ages/{2,3,4,5}/form/wander`) | 0.5 | 0.35 | A gentler lean at 40 |
| `/material/matureRadius`, `matureSpan` | 0.23, 0.06 m | 0.30, 0.07 m | The fuller crown thickens the stem; old bark again starts above the 40-year flare and reaches a metre at 80 |

Round 6's shorter-lived limbs, which made 40-year seed 42 lopsided, are gone (`LIMB` lives 25 years). Probes: `raw/probe/q7a` (scaffolds alone, `LIMB` 0.12: a sparse tree), `q7b` (more scaffolds and longer-lived passing branches: a broad dome, but a stem black to the crown), `q7c`, `q7d` (the stem's old-bark radius moved with its new girth).

## The stills

`raw/sheet7/`: `sheet-40.png`, `sheet-80.png` (references, then seeds 1, 7, 42: bare, in leaf, crown, limb) and `bases.png`. Viewed by the worker.

| Age | Seed | Nodes | Height | Crown width |
|---|---|--:|--:|---|
| 40 | 1 | 420,906 | 16.9 m | 7.8 × 7.6 m |
| 40 | 7 | 753,616 | 17.4 m | 8.5 × 8.3 m |
| 40 | 42 | 519,294 | 16.8 m | 8.6 × 9.4 m |
| 80 | 1 | 1,573,152 | 25.5 m | 18.1 × 20.3 m |
| 80 | 7 | 1,818,916 | 27.1 m | 16.5 × 17.7 m |
| 80 | 42 | 1,848,829 | 25.5 m | 17.8 × 15.2 m |

**The worker's reading.** At 40 the crowns are narrow eggs at every seed, seed 42 leaning but no longer lopsided. At 80 the scaffolds now carry the crown's breadth up into its top, and seed 1 reads near a rounded dome bare; seed 7 keeps a tall upper axis and seed 42 stays narrow (15 m across one way). The bases hold: white at 40, black and fissured to about a metre at 80. The 80-year trees are 25.5 to 27.1 m tall, between LO96's classes II and I.

## Astra's verdict (crown shape only)

`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-sheet7.md`, told that leaf seating is fn-223's known gap. Verbatim:

> | Age | Seed | Verdict | Image evidence |
> |---|---|---|---|
> | 40 | 1 | PASS | `sheet-40.png`, seed 1, shows a compact, narrow oval crown with tapering shoulders, consistent with the narrow-egg target. |
> | 40 | 7 | PASS | `sheet-40.png`, seed 7, shows an elongated egg-shaped crown whose uneven upper tips remain within a suitably narrow outline. |
> | 40 | 42 | PASS | `sheet-40.png`, seed 42, shows a narrow, leaning egg-shaped crown, with projecting lateral branches adding asymmetry without changing its overall form. |
> | 80 | 1 | FAIL | `birch-80-1-bare.png` shows broad lateral spread but a central upper extension rising above the shoulders, leaving a peaked rather than rounded crown. |
> | 80 | 7 | FAIL | `birch-80-7-bare.png` shows a tall, rightward upper axis overtopping the other limbs and producing an elongated, pointed crown. |
> | 80 | 42 | FAIL | `birch-80-42-bare.png` shows several upper forks, but their tightly upright arrangement retains a narrow oval crown rather than the required broad, rounded canopy. |
>
> **Overall crown shape verdict: FAIL.** The 40-year specimens meet the narrow-egg target; the 80-year specimens lack the broad, rounded mature silhouette, with seeds 1 and 7 retaining conspicuous top extensions and seed 42 remaining too narrow despite its forks.
>
> Leaf seating and leaf mass were excluded.

## Stop: (c) fails again; the host puts the strongest sheet to the owner

40 passes at every seed for the first time; 80 fails at every seed. Per the host's instruction the worker stops here.

What remains at 80, as Astra and the worker read it: an upper axis that still overtops the limbs (seeds 1, 7), because a relay of the old stem, or a scaffold set at 1.3 rad, keeps rising for up to 25 years; and upper forks held tightly upright (seed 42), because every scaffold rises at the same set-point before it spreads. No further values were tried.

**The preset carries round 7**: 40 now passes, the 80-year crowns are broader in their upper half than round 5's, and the base holds. It is the worker's candidate for the strongest sheet; the host decides which sheet goes to the owner (round 5's is `raw/sheet5/`).

## Gate (round 7, commit `3ff611e9`)

Render half first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0; then `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0; `harness/parity.test.ts` passes 8 of 8. Limits at 8; logs in the ignored `raw/gate7-*.log`.
