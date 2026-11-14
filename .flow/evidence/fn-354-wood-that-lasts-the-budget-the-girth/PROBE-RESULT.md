# fn-354 R4: the probe's result, Stage 1, 11 Nov 2026

The rule in PROBE-BRIEF.md is built on `fn-354-probe` (`b52bd3654`, never merged) and drawn on the brief's panel under B, G1 and G2, beside the base run of 11 Nov. The oak at 40 on seed 1001 draws a trunk of 27 cm at 1.3 m under the rule, against the paper's 30 cm. The rest of the panel was drawn after that check. Two trees were refused for wood below the ground: one under B and one under G1. G2 drew every tree. The yearly pass takes 6 to 14% of a one-thread grow, measured under load. Nothing here calls a tree better or worse; the host views the sheets.

Tier: a probe; no gate and no codex review (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; a probe, no product code lands)

## Where everything is

- **Probe code:** branch `fn-354-probe`, and as a patch in `probe/girth-rings.patch`. Its new module is `crates/telperion-space/src/girth/rings.rs`.
- **Variant values:** `probe/variant.sh`, the brief's R, N, leaf years and `4ρg/E` per preset, unchanged.
- **Sheets:** `probe/sheets/`, with columns base, B, G1, G2.
  - `oak-whole` and `oak-bare`: ages 40, 80, 120, 200 and 300, on seeds 1001, 1007 and 1012.
  - `beech-`, `birch-` and `spruce-whole` and `-mid`.
  - `oak-close`: seed 1001 at 120 and 300. It shows the trunk base leafed (`low`), the trunk base leafless (`foot`), and the largest limb's junction leafless (`joint`).
- **Every measure, a column per run:** `probe/VARIANTS.md`.
- **Yearly traces:** `probe/traces/*.csv`, from the first grow of each tree under G1 and G2.
- **Run records:** `probe/runs/` holds each run's RUN.md (commit, load), `measures.jsonl`, `cost.jsonl` and `ringpass.txt`.
- **Runner:** fn-352's scoreboard on the integration branch (`d0e390f2b`), extended by options only. Raw stills and logs are in its ignored `raw/fn354-*`.

## The oak's trunk at 1.3 m against White's open classes, cm

G1 and G2 lay the same wood: sag bends the tree but does not change its girth.

| age | White (three open classes) | seed 1001: base / B / G | seed 1007: base / B / G | seed 1012: base / B / G |
|---|---|---|---|---|
| 40 | 28 to 40 | 56 / 56 / 27 | 75 / 75 / 34 | 65 / 65 / 28 |
| 80 | 56 to 79 | 110 / 116 / 68 | 207 / 199 / 93 | 150 / 150 / 76 |
| 120 | 83 to 109 | 143 / 138 / 106 | 150 / 190 / 139 (G1 refused) | 176 / 161 / 118 |
| 200 | 121 to 152 | 129 / 82 / 132 | 226 / 89 / 171 | 115 / refused / 153 |
| 300 | 156 to 192 | 120 / 36 / 145 | 106 / 52 / 185 | 170 / 59 / 172 |

- **Seeds inside White's range under G:**
  - At 40: 2 of 3.
  - At 80: 2 of 3.
  - At 120: 1 of 3.
  - At 200: 1 of 3 (153 cm is 1 cm over).
  - At 300: 2 of 3.
- **Outside under G:**
  - Above the range: seed 1007 at 80, 120 and 200, and seed 1012 at 120.
  - Below the range: seed 1001 at 40 (27 cm) and at 300 (145 cm).
- **Trace from seed 1001 at 300 under G2,** the rule's trunk at each age on the way: 26, 68, 105, 134 and 145 cm at 40, 80, 120, 200 and 300. The survey's paper arithmetic, on the cycle before the budget fix, gave 30, 72, 103, 144 and 175 cm.

## The other species at 40, trunk at 1.3 m, cm (base / B / G)

| seed | beech | birch | spruce |
|---|---|---|---|
| 1001 | 26 / 26 / 9 | 30 / 30 / 19 | 34 / 35 / 41 |
| 1007 | 29 / 29 / 10 | 26 / 27 / 17 | 33 / 34 / 39 |
| 1012 | 30 / 30 / 12 | 30 / 31 / 19 | 37 / 37 / 42 |

## Limbs

Full rows are in VARIANTS.md. Under G1 and G2 the limbs' base diameters are the same, since sag does not change girth.

- **Oak, the three largest limbs' base diameter in cm** (B against G):
  - At 40: 24 to 38 against 10 to 15.
  - At 80: 44 to 134 against 17 to 48.
  - At 120: 37 to 142 against 34 to 76.
  - At 200: 29 to 74 against 56 to 81.
  - At 300: 10 to 53 against 55 to 95.
- **The largest limb's chord from vertical, in degrees.**

  | species | base | B | G1 | G2 |
  |---|---|---|---|---|
  | Oak at 40 to 120 | not measured | 23 to 64 | 40 to 108 | 21 to 47 |
  | Oak at 200 to 300 | not measured | 48 to 104 | 37 to 55 | 13 to 46 |
  | Beech | not measured | 37 to 40 | 1 to 8 | 1 to 8 |
  | Birch | not measured | 24 to 51 | 67 to 87 | 32 to 49 |
  | Spruce | not measured | 87 to 88 | 88 to 90 | 83 to 85 |

  - The base run had no limb measure; it was added for this probe.
  - Under G, the beech's largest limb stands within 1 to 8° of vertical. I did not check which limb that is.

## Leaf: the rule's account against the drawn tree's, last year, m²

The rule's figure is the whole tree's `A` at its base. The drawn figure is the scoreboard's leaf area under G2.

| tree | rule | drawn |
|---|---|---|
| oak 40 (1001, 1007, 1012) | 122, 185, 129 | 91, 197, 103 |
| oak 80 | 491, 920, 613 | 724, 1,388, 908 |
| oak 120 | 739, 1,190, 902 | 1,050, 1,736, 1,304 |
| oak 200 | 229, 393, 486 | 314, 534, 713 |
| oak 300 | 146, 254, 326 | 248, 377, 498 |
| beech 40 | 17, 24, 29 | 52, 69, 110 |
| birch 40 | 69, 55, 76 | 183, 143, 199 |
| spruce 40 | 446, 422, 466 | 272, 258, 281 |

- **For the oak from 80 on,** the drawn leaf is 1.4 to 1.7 times the rule's.
- **For the beech and birch,** the drawn leaf is 2.6 to 3.8 times the rule's.
- **For the spruce,** the rule's leaf is 1.6 times the drawn. Its drawn figure is whole needle surface, and its six leaf years are the brief's estimate.

## Cost

One thread pinned to one core. Seed 1001, grown, expanded and dressed. The machine was at load 18 to 60 from other sessions (`probe/runs/fn354-cost-*/RUN.md`), so the seconds are this evening's and noisy.

| oak | B seconds | B peak GiB | G2 seconds | G2 peak GiB | G2's yearly pass | pass share |
|---|---|---|---|---|---|---|
| 40 | 1.60 | 0.42 | 2.21 | 0.45 | 0.14 s | 6% |
| 120 | 77.3 | 3.33 | 49.6 | 3.53 | 5.57 s | 11% |
| 300 | 199.4 | 2.01 | 221.8 | 2.29 | 31.2 s | 14% |

- **The noise:** B at 120 being slower than G2 is noise from the load; a quiet rerun is needed before any number here is quoted.
- **The pass against the brief and the design:**
  - It does not double the grow, so the brief's early stop does not apply.
  - It is over the design's tenth at 120 and 300.
  - It runs on one thread and builds two flat vectors a year. It was not optimised.
- **B's memory.** B in this table is the probe binary with the rule off. That binary carries the new 8-byte `wood` field on every phytomer, about 0.03 GB at 120 by its phytomer count.

## Refused trees

Both are refused by the settle's ground check, on an axis of the oak's PA 6:

- **B, oak at 200, seed 1012:** "axis 2363 (PA 6, born in cycle 135, base at 0.141 m) reaches 0.005 m below the ground". The base run drew this tree, so the budget fix on this branch changed it.
- **G1, oak at 120, seed 1007:** "axis 4692 (PA 6, born in cycle 83, base at 1.247 m) reaches 0.016 m below the ground".

## What is drawn, per tree

From the sheets, all viewed. These are descriptions, not verdicts.

### Oak at 40

- **Base and B** are the same tree: a single stem with laterals splaying low and a thick base, inside a dense crown.
- **G1 and G2** draw a slim stem (27 to 34 cm) with fine, wiry laterals. The low laterals of seeds 1001 and 1012 read as a thin skirt of twigs rather than wood.
  - **G1:** the crown is more open and wider than G2's.
  - **G2:** close to B's outline.

### Oak at 80

- **Base and B:** heavy, many-limbed crowns on a thick bole.
  - Seed 1007's trunk is 2 m across, with a flared, collar-like base.
- **G:** the same outlines, drawn with slender limbs and many fine branches, and a stem near a third of B's.
  - **G1:** wider.
  - **G2:** the crown is fuller and rounder than G1's on seeds 1007 and 1012.

### Oak at 120

- **Base and B:**
  - Seed 1001 is a V of two thick leaders.
  - On seeds 1007 and 1012, B is a broad, low dome of heavy limbs.
- **G1:**
  - Seed 1001: the V is flattened out wide, its limbs leaning out.
  - Seed 1007 was refused.
- **G2:**
  - Seed 1001 keeps the V upright.
  - Seeds 1007 and 1012 are rounded crowns on a single trunk.

### Oak at 200

- **Base:** a long bare curving stem with a crown at the top. Seed 1007's stem lies along the ground first.
- **B:**
  - Seeds 1001 and 1007 draw a tangle of thin, crooked limbs on a short trunk (82 and 89 cm), some trailing on the ground.
  - Seed 1012 was refused.
- **G1:** a spreading, open crown on a 1.3 to 1.7 m trunk. Seed 1012 leans hard to one side.
- **G2:** taller (28 to 32 m) and more upright crowns of fine branches. Seed 1012 has one whip-like shoot standing above the crown.
- **Under G on seed 1007,** the trunk shows a band, a step in its outline, a little above the flare. No close view was taken at 200.

### Oak at 300

- **Base:** tall sinuous stems carrying small crowns. Seed 1007's stem loops near the ground.
- **B:**
  - Seed 1001 is 7.7 m tall: a 36 cm stem with a thick limb bent back in a loop, and a long limb lying on the ground.
  - Seeds 1007 and 1012 are low tangles about 15 to 17 m tall, on 52 to 59 cm stems.
- **G1 and G2:** a short, very thick base on every seed: 145 to 185 cm, with a broad flare.
  - **Seed 1001:** above the base, a step to a much narrower curving stem, carrying a small crown at about 22 m.
  - **Seed 1007:** a squat, barrel-like trunk with one long leaning limb.
  - **Seed 1012:** a thick trunk with a visible sleeve and step a few metres up, under a rounded crown.
  - **G2 against G1:** slightly taller and more compact.

### Oak close views, seed 1001

- **At 120, trunk base (`low`, `foot`):** a smooth flared base in every run. B's is the widest. G's is narrower, with the flare's rim lower.
- **At 120, the largest limb's junction (`joint`):** in B, G1 and G2 alike, the trunk below the fork ends in a flat-topped cylinder. The fork's limbs rise from inside its rim, a visible step of a few centimetres all round. Under G the cylinder is narrower and the step is the same in kind.
- **At 300, trunk base under G:** a wide flare with a raised band, a ring-like step, part way up it. B's base is a thin stem.
- **At 300, the junction under G:** a wide lower cylinder with a flat rim. The stem above is narrower and banded with several ring steps, one above another.
  - It is the shoulder the design's "What the drawing needs" says the close view should show.
  - In B the largest limb is a thick arch bent back across the stem, with a dark seam where it meets the stem.

### Beech at 40

- **Same outline in every run:** a straight stem under an upper crown.
- **Under G** the stem is a wire: 9 to 12 cm, against 26 to 30 cm. The branches are thin, so the crown reads as leaves on fine twigs.

### Birch at 40

- **Same form in every run:** a white stem, with hanging shoots in a narrow crown.
- **Under G:**
  - The stem is 17 to 19 cm, against 26 to 31 cm.
  - G1's crown is wider and its hanging shoots longer (spread up to 13 m).
  - G2's is narrower than B's.

### Spruce at 40

- **Same cone in every run.**
- **G1:** the stem is thicker (39 to 42 cm) and the tiers spread wider.
- **G2:** the tiers are narrower, a slimmer cone (spread 6 to 7 m against 7 to 8 m).

## What could not be built as written, and what was done instead

1. **The last year's leaf.**
   - The grower lays no rough layout in the last cycle (`paused.rs`: "the last cycle's layout and light are read by no bud"). That year's new phytomers have no deposit and no scale.
   - Under the rule, the last cycle's layout is drawn, with no light read. A check variant (`SKETCH`) draws it without the rule. On the oak at 40, seed 1001, that tree is the same as B in every measure.
2. **Where `T` and `S` live.**
   - The design puts them in the phytomer's unused `radius` field. During growth the rough layout's landing reads that field (`geometry/lay.rs:343`), so a packed value would move the layout.
   - A new field, `wood: [f32; 2]`, holds them instead. It adds 8 bytes per phytomer.
3. **Relay joins.**
   - As the brief says, a relay's leaf enters its bearer at the tip. So the bearer's head above the join carries the relay's wood too.
   - `Join::below` and `above` are both the radius of the bearer's phytomer at the join's node. The brief does not say which radii to use.
4. **A fading branch's share.**
   - "Its kept share" is read as the share its scale is drawn at: its own fade times the fade of every axis it stands on, as `vigour` passes down through scale. The section is drawn as `T × share`.
5. **A retired subtree.**
   - Under the rule nothing reads a retired subtree's radius. It is not drawn, and its bearer's `T` is untouched.
   - So nothing was stored for it. The settle's tree-as-grown work (`keeps`, `freeze`) is skipped while the rule is on.
6. **The new phytomer's `T`.** `π × established²` uses the scale the rough layout gave it in the year it was laid.
7. **The trace's 1.3 m point and height** are read on the rough layout, unsagged. The trunk path follows the largest `T`.
8. **Leaf years** are the brief's (1, or 6 for the spruce), not the presets' `leaf_years`.

## For the host to read

These are observations, not decisions.

- **Drawn leaf.** Under G the drawn leaf moves with the girth, though the leaf the grower counts does not: the oak at 40 on seed 1001 draws 91 m² against B's 125 m². That tree holds fewer phytomers under 0.2 mm (8,044 against 52,634), so the 0.2 mm cut is not the cause. The leaf drawn follows the radius some other way, which I did not trace.
- **The leaf the rule reads** is 0.3 to 0.4 of the drawn leaf for the beech and birch. It is the grower's `leaf_area × scale × size` per phytomer.

## Friction

Three entries were added to FRICTION.md during this task:

- The scoreboard's probe did not compile after fn-353 deleted `Growth::scale`.
- The hook refused heredoc edit scripts and moves under the home directory.
- Other sessions held the load at 18 to 60 while the probe timed its pass.
