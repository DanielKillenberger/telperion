# fn-190 R1a, probe round 1: organogenesis by continuous physiological age, no feedback, 2026-10-03

## Summary

Cost and fine wood pass on every tree. Architecture fails on the beech and oak.

- **Time and fine wood pass at both seeds** for the beech, oak and spruce. Growth takes 0.27 to 0.51 of the time bar. Fine wood is 1.07 to 1.34 times today's.
- **The visual gate fails on the beech and oak.** Both grow a dense bush without differentiated limbs. No tree has a lateral at 0.4 of the root radius (`root04` 0 everywhere).
- **The spruce passes every numeric check.** Visually it is borderline: it has a leader and a cone, but its tiers are a haze and its lowest branches reach below the ground.
- **Without feedback, size depends on how strongly an axis reiterates.** At full reiteration the rule behaves as a branching process: at one beech point, seed 1 grew 160k nodes and seed 7 42k, and a pull change of 0.05 to 0.02 moved seed 1 to 45k. At partial reiteration the seeds agree on size, but the oak's height is still bimodal.

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/` (scratch, never merged).
  - `organ.rs` replaces `law.rs`. `light.rs` is deleted; R1b's one Beer–Lambert pass returns from round 4's history (`fbd82732`) when that step runs.
  - `params.rs` is rewritten for the new settings. `score.rs`, `bands.rs` and `tree.rs` are reused unchanged, apart from `tree.rs`'s import.
- **The rule (one unit per bud per year, trunk to twig, short shoots included):**
  - A bud's phi is its birth phi plus `drift` × the years its axis has grown.
  - phi sets every outcome of the bud's yearly unit:
    - metamers per unit: `n0` to `n1`;
    - internode length: 1 to `short` × unit;
    - laterals per metamer: `branching` × (1 − phi)^`fate`, weighted along the unit by `acrotony` and `rhythm`;
    - each lateral's phi: phi + `phiStep` + `zone` × (1 − u), where u is the lateral's place along the unit from the base;
    - departure angle: `angle0` to `angle1`;
    - lean toward horizontal-outward: `lean0` + `lean1` × phi, pulled by `eta` per metamer;
    - two-ranked bearing: `distich`.
  - The terminal bud survives the year with chance `persist0` to `persist1`, blended by phi^`persistShape`.
  - When the terminal aborts, the distal laterals relay it. Their phi moves toward the axis's birth phi by `reiteration` × (1 − birth phi)^`reiterShape`.
  - A short shoot is a bud at phi near 1. It grows `n1` short internodes a year, never branches (its lateral count is 0 at phi 1) and dies by `persist1`. It is grown wood, and every short shoot is a tip in the pipe model.
  - Integer outcomes are keyed-rounded in expectation.
- **No feedback.** There is no light, allocation, balance, shedding, reserve or envelope pull, so envelope attraction is 0 by construction. The envelope supplies only the height that scales the unit.
- **New this round** (each continuous, each with a neutral value):
  - `persistShape` and `reiterShape`: shapes of the persistence and reiteration curves.
  - Reiteration toward the axis's birth phi. Resetting toward phi 0 exploded at once.
  - Straightening is one pass at the end. Each segment turns toward up by `straighten` × (its radius ÷ the root's)² × its years, and only its own segment turns: the subtree does not rotate with it. Round 5's form, the per-cycle rotation of whole subtrees, turned the limbs vertical and made a column: at straighten 0.1, w/h was 0.33 and the first-order elevation median 87°.
  - A base-effect term was tried and deleted. Every non-zero value shrank the tree to under 25k nodes or under 1,000.
- **A tree past `maxNodes` (600k) is an error, never a capped tree.** A tree under 1,000 nodes is an error, as before.
- **Bars:** today's rows in `raw/today/` recorded in round 1, at 1.5 × warm median. Today's skeleton was also re-measured this round (`raw/r1a/today/`) and was 8 to 22% slower now, so the recorded bar is the stricter one and is the bar used here.

## Points (seed 1 tuned, seed 7 held out)

| species | settings (all others neutral) |
|---|---|
| beech | cycles 50, n0 6, n1 1.5, unit 0.009, short 0.85, drift 0.03, persist1 0.5, persistShape 6, reiteration 0.7, branching 1.6, phiStep 0.2, zone 0.7, acrotony 1, rhythm 0.3, lean0 0.4, lean1 1.0, straighten 0.4, angle0 45, angle1 60, distich 0.8 |
| oak | cycles 45, n0 6, n1 1.5, unit 0.013, short 0.85, drift 0.04, persist1 0.5, persistShape 6, reiteration 0.7, branching 1.9, phiStep 0.2, zone 1.5, acrotony −1, rhythm 0.6, lean1 0.8, eta 0.015, angle0 34, angle1 60 |
| spruce | cycles 36, n0 5, n1 1.5, unit 0.007, short 0.9, drift 0.015, persist1 0.7, persistShape 10, branching 1.6, phiStep 0.3, zone 0.6, rhythm 1.0, lean0 0.3, lean1 1.5, angle0 80, angle1 70, distich 1 |

## Required checks, per species and seed

The bole is reported, not voted, in R1a. Division and the developmental trace are listed as `bands.rs` computes them; whether they vote in R1a is decision 4 below.

| tree | bole p5 (old) | division | trace | fine km (today) | ms warm (bar) | visual gate | supplementary |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.11 (0.05), reported | 0.29 ✓ | born 36.6°, rise 13.9° ✓ | 50.9 (46.7) ✓ | 28.7 (79.5) ✓ | ✗ bush, no limbs | 2/3 ✓ |
| beech s7 | 0.10 (0.05), reported | 0.25 ✓ | born 36.7°, rise 12.8° ✓ | 51.8 (47.1) ✓ | 29.4 (79.5) ✓ | ✗ bush, no limbs | 2/3 ✓ |
| oak s1 | 0.16 (0.39), reported | 0.39 ✗ | born 44.8° ✗, rise 0° | 38.0 (31.2) ✓ | 15.3 (49.6) ✓ | ✗ fan of twigs | 2/3 ✓ |
| oak s7 | 0.11 (0.69), reported | 0.81 ✗ | born 45.9°, rise 0° ✓ | 38.2 (35.6) ✓ | 14.7 (55.3) ✓ | ✗ leader over a bush | 0/3 ✗ |

| tree | leader | elevation | no division < 0.85 | fine km (today) | ms warm (bar) | visual gate | supplementary |
|---|---|---|---|---|---|---|---|
| spruce s1 | 1.00 ✓ | 5.4° ✓ | 0.90 ✓ | 24.0 (18.8) ✓ | 21.9 (47.6) ✓ | borderline: leader and cone, no distinct tiers | 2/2 ✓ |
| spruce s7 | 1.00 ✓ | 5.5° ✓ | 0.94 ✓ | 24.1 (17.9) ✓ | 21.8 (43.0) ✓ | borderline, as seed 1 | 2/2 ✓ |

### Supplementary bands and diagnostics

| tree | junction o1 | secondaries per major | length ÷ remaining | root04 | nodes (today) | tips | w/h (authored) | H ÷ authored |
|---|---|---|---|---|---|---|---|---|
| beech s1 | 0.39 ✓ | 16 ✗ | 0.49 ✓ | 0 | 229k (213k) | 37.8k | 0.54 (0.72) | 1.01 |
| beech s7 | 0.39 ✓ | 18 ✗ | 0.58 ✓ | 0 | 233k (215k) | 38.6k | 0.51 (0.72) | 1.07 |
| oak s1 | 0.28 ✗ | 6 ✓ | 0.63 ✓ | 0 | 142k (134k) | 26.4k | 0.64 (1.10) | 1.09 |
| oak s7 | 0.23 ✗ | 18 ✗ | 1.51 ✗ | 0 | 143k (144k) | 26.8k | 0.45 (1.10) | 1.56 |
| spruce s1 | 0.18 ✓ | – | – | 0 | 254k (96k) | 31.8k | 0.85 (0.62) | 0.96 |
| spruce s7 | 0.18 ✓ | – | – | 0 | 255k (91k) | 32.0k | 0.78 (0.62) | 1.05 |

For the spruce, "lowest lateral ≤ 0.10 H" also passes, at 0.04 and 0.03.

- **Fine wood at the 0.04 and 0.06 boundaries:** beech 45.9 / 53.2 km (s1) and 46.8 / 54.5 km (s7). The beech fails at 0.04 at seed 1, against 46.7. Oak and spruce stay within 1% of their 0.05 figure.
- **Secondaries per major** are counted on `local04` majors. These are laterals off the trunk at 0.4 or more of the local trunk radius, and there are 25 to 29 of them on the beech. No lateral reaches 0.4 of the root.

## Visual gate (3 images: `raw/stills/sheet-r1a-<species>.png`)

Each sheet holds the S1 photograph, today's bare still at seed 1, and this round's bare and whole stills at seeds 1 and 7.

- **Beech, fail at both seeds.** A dense, rounded bush of fine wood from the ground up, with a thin leader standing out of its top. There are no visible limbs and no bole. It is further from the photograph than today's tree, which shows a trunk and ascending limbs.
- **Oak, fail at both seeds.** Seed 1 is a fan of thin wood around a central leader. Seed 7 is a bush with a tall bare leader carrying a small head above it: a relay complex outgrew the crown, and the height is 1.56 × authored. Today's oak shows real limbs; this round's does not.
- **Spruce, borderline at both seeds.** It has a persistent leader and a conical outline. The branches read as a haze of fine wood rather than tiers of horizontal branch systems, and the lowest branches spread on and below the ground (crown base −0.01 to −0.02 H).

## Passes kept and deleted (cold ms, beech s1 / oak s1 / spruce s1)

| pass | what it does | ms |
|---|---|--:|
| yearly sweep | one pass a year over the living buds; each grows its unit and pushes its laterals | 27.3 / 16.9 / 21.1 |
| pipe | one basipetal pass at the end: tips, then radii | 2.9 / 2.3 / 3.9 |
| straighten | one pass at the end, each segment's own turn (beech only) | 5.7 / 0 / 0 |

- **State per bud:** its node, direction, outward bearing, phi, years and terminal flag. At most 12.5k buds are live (beech).
- **State per node:** position, direction, length, parent, lateral flag and birth year. No per-node state is read back during growth.
- **Deleted, confirmed absent from the probe's code:**
  - the light field and its per-cycle rebuild;
  - per-bud light lookups;
  - Borchert–Honda allocation and the per-cycle basipetal balance;
  - upkeep and the reserve;
  - shedding of branches and of short-shoot records;
  - the records themselves (short shoots are grown wood);
  - per-cycle straightening;
  - compaction;
  - phototropism;
  - the envelope pull;
  - marker perception, already gone.
- **Warm totals:** 28.7 ms for 229k nodes on the beech (about 125 ns a node), against today's 53.0 ms skeleton at equal fine wood.

## GreenLab-style factorisation with continuous phi

This round measured the hypothesis; it did not implement factorisation.

- **Counts.** Without feedback, a lateral's expected subtree depends only on its starting phi and birth year. The rule therefore factorises in expectation wherever those keys repeat.
  - They repeat because phi is built from a lattice: drift × whole years, phiStep, and zone × (k + 1) ÷ n.
  - Laterals per distinct (phi, birth year) key: beech 42,088 / 1,218 = 34.6×, oak 28,508 / 463 = 61.6×, spruce 33,241 / 117 = 284×.
- **What does not factorise:**
  - Keyed rounding makes each instance's topology differ, so only expected counts repeat.
  - Geometry depends on the parent's direction, so directions do not repeat.
  - Straightening reads the whole tree's pipe share, so its turns do not repeat.
  - Any continuous input to phi would make every key distinct. The candidates are R1b's light and vigour feeding phi.
- **Time.** The sweep that emits geometry is the whole cost (above). Factorising counts would remove none of it while every node is drawn. The hypothesis therefore holds for counts here, buys no time, and is not expected to survive R1b's feedback.

## What the round established

- **Cost and fine wood: one open-loop organogenesis pass is fast enough.** The beech grows 229k nodes and 109% of today's fine wood in 0.36 of the bar. The oak takes 0.27 to 0.31 of its bar, the spruce 0.46 to 0.51.
- **Spread is not the problem without feedback; limbs are.** The beech reaches 0.76 of the authored spread and the spruce 1.32. Directions and lean set the outline, but no axis grows into a limb.
  - Without feedback, a lineage's mass grows with its years, so the oldest, lowest laterals carry the most wood. The crown becomes a bush with a skirt.
  - Relays from the trunk's abort start late and, with drift, already old. None reaches 0.4 of the root radius.
- **Reiteration without a bound makes size a knife-edge** (`FRICTION.md`, this round). Full renewal of phi gave either a supercritical explosion past 600k nodes or a small tree. At one beech point the seeds grew 160k and 42k nodes. Branching 2.4 against 2.6 gave 160k and 115k, the opposite of the expected direction.
  - Partial reiteration (0.7, toward birth phi) with a steep persistence curve made size reproducible: beech 229k / 233k, oak 142k / 143k, spruce 254k / 255k.
  - The oak's height stays bimodal (1.09 against 1.56 H) depending on whether a relay outgrows the leader. Walks across that boundary would jump.
- **Troll's trace passes at both seeds** with end-of-run straightening of each segment's own share (born 36.6° and 36.7°, rise 13.9° and 12.8°).
- **Rauh's born-at-45° check sits on the edge** (44.8° and 45.9°). The departure angle and the first metamer's tropism step set it, not architecture.

## Decisions this round needs from the host

1. **Limbs.** No open-loop setting tried gives a limb. That covers every trunk lateral and every relay: none reaches 0.4 of the root radius, while today's beech has 5. The candidate causes for the host to weigh:
   - age-ordered mass: older lineages are always bigger without feedback;
   - relays born old by drift.
   Should round 2 give the crown's upper axes their vigour by ontogeny (for example, reiteration readiness or lateral phi that falls with the tree's age or height)? Or is limb differentiation expected only from R1b's light, so that R1a's visual gate is unreachable by construction?
2. **Size and reiteration.** Without a resource bound, reiteration is a branching process. Should R1a bound counts the GreenLab way, scaling lateral counts by a supply ÷ demand ratio? That would bring in a form of feedback, which R1a excludes. Or does R1a accept partial reiteration, leaving size and the no-jump walks to R1b?
3. **Straightening.** The end-of-run, per-segment form passes the Troll trace and keeps the crown open. It is not the per-cycle reorientation the literature describes (round 5's form, which made columns). Is it acceptable as the law's straightening?
4. **Which checks vote in R1a.** The spec's R1a criterion names the visual gate, the supplementary bands, fine wood and time, and makes the bole reported only. `bands.rs` still votes division and the developmental trace. Do they vote in R1a, and does the oak's "division − clear bole ≤ 0.10" clause (R1 bands table, not in `bands.rs`) apply once the bole stops voting?
5. **The spruce's ground.** Low branches grow below y = 0, which round 5 also noted. Should the probe refuse such a tree as an error, or should the law's lean keep wood above the ground?

Round 1 of three for R1a is spent. Per the host's steering, round 2 is not started.
