# fn-236 sheet 2: the leaf rows, and a stop for the host (worker, 2026-10-09)

This round follows the host's decisions after round 1 (SHEET-1.md). F4 is merged (`ba813bb2`), which brings in fn-231's skip of the ordinary tree's centroid test; the ordinary preset is unchanged. Round 1's structure is kept, and only the leaf rows move. Size is judged at 80.

## Round 1 on the merged code

`raw/sheet1m/` holds round 1 rendered on the merged code. It is node for node and leaf for leaf what round 1 was before the merge (677,910 / 1,493,410 / 753,114 nodes at 80, seeds 1 / 7 / 42). The merge does not move the oak.

## The leaf probes (stills `raw/sheetL1` to `raw/sheetL4`, at 40 and 80)

Each probe was rendered from round 1's preset with these rows changed. The preset was restored after each one, so it still stands at round 1.

| Probe | Rows | Source | Leaves at 80, s1 / s7 / s42 |
|---|---|---|---|
| r1 | (round 1) | | 679,655 / 1,968,252 / 889,400 |
| L1 | `SHORT` (`/space/ages/14`) `clusterLeaves` 6, `clusterSpread` 1.0 rad | L3: crowding at the shoot tip is **estimated** from acrotony | 673,241 / 1,826,656 / 836,381 |
| L2 | L1 + blade 120 × 90 mm (`/element/length`, `/width`) | L1, L2: FNA's 25-120(-140) × 15-85 mm, OSU's 5-15 cm | as L1 |
| L3 | r1 + `/canopy/size` 1.4 (a 140 mm blade, FNA's largest), `/canopy/outward` 0.3, `/canopy/scatter` 30°, `/canopy/shootRadius` 0.1 | blade from L1 at its upper bound; the rest **estimated** | 714,009 / 2,026,587 / 925,506 |
| L4 | L3 + `shootRadius` 0.2, a leaf every 6 mm of shoot (`/skeleton/twigs/twig/internodeLength`), `SHORT` clusters of 5 at 1.2 rad | **estimated** | 835,312 / 2,300,668 / 1,045,056 |

Wood, nodes, height and crown width do not move under any leaf row; the leaf rows dress the grown tree only.

**Sheets, all viewed:**
- `raw/compare/leafL12-80.png`: O-WHOLE beside round 1 (merged), L1 and L2, every seed at 80.
- `raw/compare/leafL3-80.png`: round 1 against L3, in leaf and crown close-ups.
- `raw/compare/leafL4-80.png`: O-WHOLE beside L3 and L4.

**What the stills show:**
- **L1 and L2:** no visible change at the hero distance. Clusters move leaves from along a short shoot to its tip without adding surface.
- **L3:** at 140 mm the leaves fill each clump more fully. In the crown close-ups the foliage reads denser and the sky shows through less within a clump. Seed 42's whole view is fuller.
- **L4:** 17 % more leaves than L3, and in the whole view barely different from it.
- **The crowns are unchanged in kind.** In every probe each one is still a set of dark clumps with sky and ground between them:
  - seed 1 is open in its lower half;
  - seed 7 has limbs standing out of its upper left;
  - seed 42's wood shows across its middle.

  O-WHOLE is one continuous, closed surface. The leaf rows thicken each clump but do not join the clumps.

## Stop (host decision 4 after round 1)

The crown is still clumps with gaps after the leaf rounds, so this stops here as the host's rule says. The blade is already at the largest size its source allows (140 mm). The rest of what closed a clump was estimated rows, and it gave no visible step past L3. Round 1's preset stands; no leaf row was committed.

Seed 7 at 80: 1,493,410 nodes, 21.4 m tall, crown 23.0 m wide, DBH 2.0 m (round 1). It is the widest seed and has the most nodes, as under every probe.

Not run: the site's field stills (R3), the timing (R4), the gate, `npm test` and impl-review. These wait for a round that reads as the dome.
