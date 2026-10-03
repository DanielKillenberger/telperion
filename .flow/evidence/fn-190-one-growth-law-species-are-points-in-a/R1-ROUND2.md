# fn-190 R1, probe round 2: light as the resource, 2026-10-03

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/`, changed per the spec's "Host decisions after R1 round 1".
  - Every bud reads its light from the shadow grid with one lookup, and its direction from the grid's gradient. Each new metamer casts its shadow. The space markers, perception cones and `space.rs` are gone.
  - The envelope is only an optional pull toward the authored crown (`envelope`, neutral 0, left at 0 this round).
  - A bud's lateral development is `branching · (1 − φ)^fate · min(1, v ÷ vRef)^vigourFate`, so a weak or old bud makes an unbranched short shoot, from the same law.
  - The tree grows from a seedling at the root in fixed space. The bole comes from shedding on remembered light per tip (`shed`, `memory`).
- **Votes:** `bands.rs` implements item 5. Two probe definitions are mine and need the host's word:
  - The Troll trace passes when the first-order axes off the stem are born at a median of 45° elevation or less and their base segments rise by 10° or more over the run.
  - The Rauh trace passes when those axes are born at 45° or more and fall by no more than 5°.
  - The spruce's "lowest lateral" is read as the band "lowest lateral with an axis ≥ 0.05 H, at ≤ 0.10 H".
- **Runs:** release, 16 threads, warm median of 3. Round 2 tuning sweeps the points on seed 1, then fixes them; seed 7 is held out. Rows are in `raw/r2/`; today's rows were re-run with fine wood at 0.04 and 0.06 (`raw/today/`).

## Points

| species | settings off neutral |
|---|---|
| beech | cycles 44, shed 0.2, memory 0.1, distich 0.8 |
| oak | cycles 40, shed 0.2, memory 0.1, acrotony 1.5, rhythm 0.3, angle0 40, lambda 0.5 |
| spruce | cycles 36, shed 0.15, memory 0.1, lambda 0.58, alpha 1.2, lean1 1.4, phiStep 0.6, rhythm 0.85, angle0 70, angle1 85 |

The beech point has no Troll lean or straightening. Every setting that gave the trace (lean0 0.2–0.5, straighten 0.03–0.08) shrank the tree to 6–9k nodes. It also took fine wood to near zero and lost the limbs (sweep in the probe session, beech seed 1).

## Results

**Required checks** (all must pass) and **supplementary** (beech and oak 2 of 3, spruce 1 of 2). Today's fine wood and 1.5 × today's skeleton time are the bars.

| tree | bole | division | trace | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.30 ✗ | 0.31 ✓ | born 54°, rise 0° ✗ | 4.6 (46.7) ✗ | 50.9 (79) ✓ | 2/3 ✓ (o1 0.43 ✓, sec 14 ✗, rest 0.57 ✓) | fail |
| beech s7 | 0.33 ✗ | 0.43 ✓ | born 55°, rise 0° ✗ | 3.9 (47.1) ✗ | 48.3 (79) ✓ | 2/3 ✓ (sec 17 ✗) | fail |
| oak s1 | 0.17 ✓ | 0.18 ✓ | born 55°, rise 0° ✓ | 8.5 (31.2) ✗ | 118 (50) ✗ | 2/3 ✓ (o1 0.34 ✗, sec 6 ✓, rest 0.89 ✓) | fail |
| oak s7 | 0.10 ✗ | 0.11 ✗ | born 55°, rise 0° ✓ | 7.8 (35.6) ✗ | 125 (55) ✗ | 2/3 ✓ (o1 0.31 ✗) | fail |

| tree | leader | elevation | no division < 0.85 | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| spruce s1 | 1.0 ✓ | 29.4° ✓ | 0.83 ✗ | 4.2 (18.8) ✗ | 38.3 (48) ✓ | 2/2 ✓ (lowest 0.02, o1 0.19) | fail |
| spruce s7 | 1.0 ✓ | 30.0° ✓ | 0.0 ✗ | 64.5 (17.9) ✓ | 792 (43) ✗ | 1/2 ✓ (o1 0.25 ✗) | fail |

- **Fine wood at 0.04 and 0.06 of the root:** within 2% of the 0.05 figure on the oak and spruce. On the beech it is 34% lower at 0.04 and 12% higher at 0.06, with the same verdict.
- **Visual gate (3 images: `raw/stills/sheet-r2-<species>.png`, reference S1 | today bare | round 2 bare | round 2 whole, seed 1):**
  - **Beech, fail.** A clear bole carries a narrow, upright broom crown. There are no differentiated limbs and no dome; it reads as a poplar beside Entzia's spreading crown.
  - **Oak, fail.** Several near-parallel uprights rise from a low division: the repeated paired uprights the gate rejects. The crown is too narrow and short (15 m against the envelope's 24 m).
  - **Spruce, nearest to passing.** A cone with a persistent leader and tiers of near-horizontal branches, closer to the photograph than today's. It is sparse: thin fine wood and little foliage, and no division only because the measure finds a crossing at 0.83 H.
- **Bole by shedding works on the beech.** Shedding on remembered light took the lowest substantial lateral from 0.02 H (no shedding) to 0.21–0.33 H. But the response is steep: shed 0.15 or 0.2 with memory 0.1 or 0.3 gave 0.03, 0.21, 0.67 and 0.21 H.

## Where time goes (cold run, ms)

| tree | total | light lookups | allocate | shed | extend: bud work | extend: shadow casts (voxel updates) |
|---|--:|--:|--:|--:|--:|--:|
| beech s1 | 54 | 11.6 | 7.3 | 1.7 | 21.9 | 9.5 (24.8M) |
| oak s1 | 122 | 25.3 | 16.4 | 3.9 | 48.3 | 22.2 (59.1M) |
| spruce s1 | 40 | 6.6 | 4.0 | 0.8 | 16.9 | 9.2 (23.1M) |
| spruce s7 | 826 | 127 | 200 | 93 | 292 | 18.3 (10.1M) |

Shadow casting is a fifth to a third of the time. Bud work dominates: the bud list, its lookups, allocation and new metamers. Dormant buds are never dropped now that nothing marks them dead, so the bud list only grows.

## Decisions this round needs from the host

1. **Nothing bounds the tree's size.** Each bud's light adds to the resource, so the resource grows with the bud count. The same spruce point grew 49k nodes at seed 1 and 597k at seed 7 (it hit the node cap, 792 ms). Alpha 1.2 against 1.5 gave 49k against 598k nodes. That is the jump the walks forbid. What sets size: a resource normalised to the crown, a height or age model, the envelope as more than an optional pull, or a bud mortality rule?
2. **Shedding is a cliff.** A lateral carrying the main line can be shed, taking the tree with it. With lambda 0.48, or persistence 0.9 on the beech, the tree fell to 5–10 nodes. Should shedding weigh light against size (Takenaka), or spare the main line?
3. **Fine wood stays a quarter to a tenth of today's** where growth is bounded (3.9 to 8.5 km against 18 to 47 km). The short-shoot settings (more laterals, short shoots, drift) share the vigour more thinly, so the shaded shoots are shed and the tree gets smaller rather than finer. The grid's shade per shoot (`shade` 0.003–0.01) changes this only by trading it for time.
4. **The Troll trace and the visual gate cannot pass together with the bands yet.** The lean and straightening that produce the trace also shrink the tree. Confirm the trace definitions above, or give the host's own.

Round 2 of six is spent.
