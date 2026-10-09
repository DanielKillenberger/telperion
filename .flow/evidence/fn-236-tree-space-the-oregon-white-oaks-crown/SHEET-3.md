# fn-236 sheet 3: round 3, the fine orders fill the crown (worker, 2026-10-09)

Round 3 follows the host's decisions after the leaf rounds (SHEET-2.md) and Astra's review (ASTRA-CROWN.md). It is built on round 1's structure with L3's leaves, and judged at 80 from four azimuths. **At 80 the three seeds read as broad, closed crowns.** Round 3 is committed in `2c6854f3`: the preset's rows and `oak()` together, with `every_engine_preset_carries_the_engines_species` green.

## What moved

| Row | 1 | 3 | Control (host decision) |
|---|--:|--:|---|
| `BRANCH` medial / top `TWIG` share, `/space/ages/9/zones/{1,2}/lateral/13` | 0.18 / 0.25 | 0.35 / 0.10 | 3, placement: the twig budget moves from the top cluster to the medial nodes |
| `BRANCH` insertion, `/space/ages/9/insertion` | 0.8 | 1.0 rad | 3 |
| `TWIG` lifespan, viability, `/space/ages/13/{lifespan,viability}` | 3, 0.95 | 5, 0.98 | 3 |
| Fine orders' light share ψ, `/space/ages/{9,12,13,14}/shadeSize` | 1.5 | 1.0 | 2 |
| `FORK` top-bud `LIMB` share, `/space/ages/3/zones/2/lateral/6` | 0.55 | 0.8 | structure: four or five fork limbs, so no seed's crown stands on two (see "Seed 1") |
| `/canopy/size`, `outward`, `scatter`, `shootRadius` | 1, 0, 0, 0.06 | 1.4, 0.3, 30°, 0.1 | L3 kept (SHEET-2.md) |

- **Values:** every value is **estimated**, chosen by probe against the stills and the gap measure. The blade is 140 mm, the largest FNA gives (MODEL-OAK.md L1).
- **Reserve buds (control 1):** no row moved. Earlier release on the limbs (Ra), modest reserves on the boughs (Rb) and more limb buds (Rc) each left the stills and gaps where they were. The probes are below.

## The gap measure (`oak_views.rs`, `views.sh`)

- **Leaves:** each leaf is a square 0.6 × its blade length, on a 0.1 m grid.
- **Views:** 8 side views, 45° apart.
- **Gaps:** in each grid row, the share of the outline's cells (leftmost to rightmost covered cell) that no leaf covers; the mean of the 8 views.
- **Stills:** the hero shot with the tree turned 0, 90, 180 and 270°, camera and sun turned together.
- **Source:** the value file is read from disk, so a probe needs no rebuild.
- **Corrected after review (2026-10-09):** the first version painted every leaf as a 0.2 m square on a 0.1 m grid, which closed real gaps (impl-review finding, three reviewers). Every gap number on this sheet comes from the corrected raster: each leaf covers the 2 cm cells its own square overlaps, re-measured with `OAK_NO_STILLS=1` (`raw/views-g-<tag>.jsonl`). The ranking of the probes is unchanged: round 3 has the fewest gaps at every seed.

## Probes at 80 (`raw/views-<tag>.jsonl`, stills `raw/views-<tag>/`), seeds 1 / 7 / 42

| Probe | Change, on round 1 + L3 unless named | Width / height | Gaps | Nodes |
|---|---|---|---|---|
| r1 | round 1, leaves as before | 0.92 / 1.15 / 1.14 | 0.317 / 0.228 / 0.301 | 677,910 / 1,493,410 / 753,114 |
| L3 | the L3 leaves | 0.92 / 1.15 / 1.14 | 0.251 / 0.185 / 0.251 | as r1 |
| Ra | limb reserves released at delay 2, rate 0.2 | 0.85 / 1.15 / 1.19 | 0.252 / 0.181 / 0.254 | 673,288 / 1,509,761 / 765,956 |
| Rb | Ra + bough reserves 0.1, delay 2, rate 0.2 | 0.96 / 1.20 / 1.30 | 0.243 / 0.203 / 0.244 | 761,972 / 1,558,639 / 825,178 |
| Rc | limb reserves 0.4, delay 2, rate 0.2 | 0.95 / 1.25 / 1.31 | 0.257 / 0.199 / 0.251 | 695,637 / 1,500,746 / 799,549 |
| F | fine ψ 0.5 | 0.79 / 0.97 / 1.05 | 0.292 / 0.181 / 0.237 | 703,746 / 1,805,189 / 797,325 |
| P | placement: twigs medial, insertion 1.0, twigs 5 years at 0.98 | 0.87 / 1.26 / 1.19 | 0.204 / 0.209 / 0.179 | 843,437 / 1,689,695 / 957,953 |
| P2 | P without the insertion | 0.84 / 1.39 / 1.25 | 0.242 / 0.224 / 0.196 | |
| P3 | P2 + short shoots on the basal nodes | 1.04 / 1.41 / 1.32 | 0.268 / 0.227 / 0.222 | |
| PF | P + fine ψ 0.5 | 0.78 / 0.94 / 1.01 | 0.237 / 0.168 / 0.196 | 905,152 / 2,147,914 / 1,051,749 |
| PF2 | P + fine ψ 1.0 | 0.81 / 1.17 / 1.02 | 0.203 / 0.162 / 0.170 | 879,023 / 2,023,838 / 1,020,331 |
| PF3 | PF + limb elevation 0.95, insertion 0.6 | 0.80 / 1.01 / 1.06 | 0.236 / 0.187 / 0.216 | |
| **PF4 = round 3** | PF2 + fork limbs 0.8 | **1.09 / 1.14 / 1.08** | **0.158 / 0.156 / 0.143** | 1,407,033 / 2,197,974 / 1,166,095 |

**Sheets viewed:**
- `raw/compare/r3-P-80.png`, `r3-PF-80.png`, `r3-PF2-80.png`, `r3-PF4-80.png`: four azimuths per seed;
- `raw/compare/compare3-40.png`, `compare3-80.png`: O-WHOLE, 0.1.4, round 1 and round 3 in leaf, bare and crown.

**How the controls read on the stills:**
- **Placement (P):** first turned seed 42 into a closed crown.
- **Fine ψ:** at 0.5 it closed seed 7 but narrowed seeds 1 and 42 into columns. At 1.0 it kept the closure and the width.
- **Seed 1:** stayed a narrow, upright, lobed column through every fine-order probe (width over height 0.78 to 0.87). It is a crown on two fork limbs; at 0.8 for the fork's limbs it opens into a dome as broad as the others.
- **Reserve buds:** changed nothing visible.

## Round 3's measures (`raw/measure-r3.jsonl`, `raw/views-r3-40.jsonl`)

| Measure, seeds 1 / 7 / 42 | 40 | 80 | Target (derived) |
|---|---|---|---|
| Height, m | 10.6 / 11.0 / 9.9 | 16.3 / 20.6 / 17.0 | 8–12; 14–24 |
| Crown width, m | 8.9 / 10.1 / 9.5 | 18.0 / 23.0 / 18.5 | 6–11; 10–20 |
| Width / height | 0.84 / 0.92 / 0.96 | 1.11 / 1.12 / 1.09 | 1.0–1.3 |
| Crown base, m | 2.1 / 4.1 / 2.4 | 7.0 / 5.5 / 6.8 | low |
| Projected foliage gaps | 0.257 / 0.204 / 0.250 | 0.158 / 0.156 / 0.143 | |
| DBH, cm | 58 / 74 / 59 | 120 / 221 / 120 | 20–41; 41–81 |
| Nodes | 280,342 / 371,391 / 284,467 | 1,407,033 / 2,197,974 / 1,166,095 | |
| Leaves | 343,180 / 523,138 / 350,320 | 1,755,000 / 3,424,929 / 1,486,800 | |

- **Seed 7** (the host's ask): 2.2 M nodes, 3.4 M leaves, 20.6 m tall, a 23.0 m crown just past the derived 20 m, DBH 2.2 m. It is the largest tree, as in every round.
- **DBH:** at 80 it stands 1.5 to 2.7 times the derived range; round 1's trunk took back half of round 0's. The stills' trunks read heavy but not swollen.
- **Budget:** the 20 M grown budget held; no probe was refused.

**My reading of the stills:**
- **At 80** (`compare3-80.png`, `r3-PF4-80.png`), each seed is a broad, dense, closed crown on a short trunk from all four azimuths. Little sky and almost no wood shows through. The tops are flatter than O-WHOLE's dome; seed 7's is the roundest.
- **At 40** each is an upright, fuller, shrubby young oak. That is the host's accepted young form, not the target.

## R3, the site's scale (`raw/field/`, fn-214's harness copied: `render.mjs`, `run.mjs`)

**Setup:**
- **Module:** the slim field module built from round 3 (`field-r3.wasm`) beside 0.1.4's (`field-v014.wasm`, fn-214's copy).
- **Request:** age 40, wood 0.08, tuft 0.4.
- **Seeds:** the grove's `seedOf(date#layer.i)` for 2026-10-09 #0.0, #1.0, #2.1 and 2026-10-10 #0.1 (7432, 7919, 7737, 6137), and fn-214's 1234 and 5678.
- **Round 0's module** (`field-r0.wasm`, built from `22d29326`'s oak rows): rendered beside both for 1234 and 5678.

| Sheet | Contents |
|---|---|
| `raw/compare/field-r3-n36.png`, `field-r3-n48.png` | per seed, 0.1.4 (left) and round 3 (right) |
| `raw/field/oak-r0-{1234,5678}-n36.png` | 0.1.4, round 0, round 3 |

All four were viewed.

**My reading:**
- **Against round 0** (the owner's "weird" oak), round 3 at 40 is fuller and rounder, with fewer sky holes.
- **Seeds 7919, 7737 and 6137** read as round oaks.
- **Seeds 7432, 1234 and 5678** are irregular. They have patches of wood showing, and low foliage runs down towards the ground, which is the shrubby 40-year form.
- **None is 0.1.4's** dense lollipop on a clear trunk.

**At the site's scale, round 3 does not yet read as a round oak for every seed.**

## R4, cost

**Native** (`--profile ci`, one build each, rounds 0 and 3 back to back at a one-minute load of 7 to 13; `raw/measure-t-r0.jsonl`, `raw/measure-t-r3.jsonl`):

| ms, seeds 1 / 7 / 42 | Round 0 | Round 3 | × |
|---|---|---|---|
| 40 | 724 / 1,010 / 758 | 1,854 / 2,294 / 1,846 | 2.3–2.6 |
| 80 | 7,812 / 15,909 / 9,611 | 18,630 / 39,672 / 11,700 | 1.2–2.5 |

**The site's field** (Wasm under Node, one thread, n 48 and 36, medians of three; load 3 to 6; `raw/field/timing.jsonl`):

| Seed | Build | grow ms | query ms n48 / n36 | Memory MB | Cubes n48 (wood + foliage) |
|---|---|--:|---|--:|---|
| 1234 | 0.1.4 | 176 | 146 / 98 | 73 | 634 + 10,265 |
| 1234 | round 0 | 5,458 | 243 / 179 | 440 | 3,776 + 2,652 |
| 1234 | round 3 | 9,609 | 520 / 360 | 783 | 4,116 + 4,526 |
| 5678 | 0.1.4 | 210 | 168 / 111 | 116 | 649 + 10,074 |
| 5678 | round 0 | 2,058 | 101 / 73 | 157 | 2,028 + 1,702 |
| 5678 | round 3 | 7,614 | 474 / 313 | 649 | 3,930 + 5,390 |

**What this costs the site:**
- **Against round 0:** round 3's site grow is 1.8 to 3.7 times slower and takes 1.8 to 4.1 times the memory.
- **Against 0.1.4:** it is 36 to 55 times slower and takes 6 to 11 times the memory.

This is for fn-214 to choose its cut against (its coarse field still refuses the lit oak). The cost comes with the denser fine orders: the twigs live longer and more of them survive.

## Gate (round 3), limits 4/4/4

- **Render half, under the GPU lock first:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0: 205 passed in 47 binaries (`raw/gate3-render.log`).
- **Workspace half:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 101 at first: 1,060 passed, 4 failed (`raw/gate3-core.log`). All four are pins of the oak's own values:
  - `catalogue_identity`'s family, walk and override digests, which hash every preset's family and the walks between them;
  - `closed_form::boundary::the_rows_hold_their_values`: the oak's ten pinned ages changed, on one core and on four.

  The beech's, the spruce's and the palm's rows held their pins (measured). That is the measured part of "other species byte-identical". That no other preset's family moved is inferred: no other value file and no other species function changed.
- **Re-pinned:** the three digests (with a line in the file's header) and the oak's ten row hashes. Re-run, both pass: `catalogue_identity` 4 of 4, `closed_form::boundary` 2 passed (`raw/gate3-repin-*.log`).
- **Generated file:** `presets.generated.ts` regenerated with `TELPERION_WRITE_REFERENCE=1 npm run wasm:build`. All 29 hunks fall in the oak's block.
- **`npm test`:** exits 0, 143 of 143 in 13 files (`raw/npm3.log`).

## Review

Codex impl-review was run against F4's tip (`e37348f0`), so the merge is not in the diff. Three reviewers returned NEEDS_WORK on one shared P2: the gap raster painted every leaf as a 0.2 m square. That is fixed in `6627b40f` and every gap number above is re-measured. The re-review returned **SHIP**. R5, the owner's verdict, stays open by design.

## For the owner's verdict

**Stills:**
- `raw/compare/compare3-80.png` and `compare3-40.png`: O-WHOLE, 0.1.4, round 1 and round 3;
- `raw/compare/r3-PF4-80.png`: round 3 at 80 from four azimuths;
- `raw/compare/field-r3-n36.png` and `field-r3-n48.png`: the site's grids, 0.1.4 beside round 3.

**Open with it:**
- **The site at 40:** three of six grove seeds still read irregular at the site's scale.
- **Cost:** the site's oak grows 1.8 to 3.7 times slower than round 0's and takes 650 to 780 MB.
- **Stems:** DBH at 80 runs 1.5 to 2.7 times the derived range.
