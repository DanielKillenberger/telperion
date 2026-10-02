# fn-182 task 1: the twig shell (R3 to R6), 2026-10-02

Branch `fn-182-branching-hierarchy-few-strong-limbs` from master `9cbca483`. Generator commit `a046d8da`; tests follow in `768feab5`.

## What was built

- **Two rows in `/skeleton/twigs`:** `twigShell` (wire 257, rail 0 to 1, default 1) and `twigShellSoftness` (wire 258, rail 0 to 1, default 0.1, dormant at a shell of one). Both are shares of the crown's widest radius. They are declared once in the catalogue, blended linearly, on the dials, and refused by name off their rails (`twig shell`, `twig shell softness`).
- **The law** (`pipeline/branching/local/shell.rs`, gate in `local/seed.rs`): a scaffold station's lateral buds are decided once, when they first fall due.
  - A station within `twigShell` of the outline bears as today.
  - Past it, the share of stations that bear falls linearly to zero over `twigShellSoftness`. Each station's draw is keyed by its birth order and the seed, as `detail::fate` keys its draws.
  - A station that does not bear spends its lateral buds unborn, so it leaves the pending set and is never asked again.
- **Depth is shedding's:** the smaller of the horizontal room under the smooth outline and the distance to `Envelope::profile()`. Wood on or outside the outline has depth zero or less. The profile is read once per seeding pass.
- **Unset is today's law:** at a shell of one, `Shell::new` returns none and reads nothing.
- **Other changes:** `TwigAnatomy` moved to `pipeline/twigs/anatomy.rs`, so `twigs.rs` is 372 lines. Snapshot schema 8. New query purpose `twig_shell`. Two entries in the limits inventory. The sweep holds the two rows. `docs/parameters.md` and the browser metadata and presets are regenerated.

## Tests

- `suite::twig_shell`, on a synthetic crown of 96 level limbs from the axis to the outline at the widest height, a station every 0.25 m:
  - `deep_wood_bears_nothing_and_shallow_wood_bears_as_today`: at shell 0.3 with no softness, the stations that bear are exactly those at depth ≤ 0.3. Their laterals are bit-identical to the shell-of-one tree.
  - `the_fade_bears_a_falling_share_of_stations`: at shell 0.2 and softness 0.4, the bearing share is 1, then about 0.75, then about 0.25, then 0 across the depth bands.
  - `the_tree_moves_by_degree_as_the_rows_move`: shell and softness are each walked in 0.01 steps. Each step only adds stations, at most two per limb, and never takes one back.
  - `the_rows_are_refused_by_name_off_their_rails`, and `the_depth_is_the_profile_distance_where_it_is_nearer`.
- **`tests/dormancy.rs`:**
  - A new case: `twigShellSoftness` is asleep at shell 1 and awake at shell 0.2 on Ordinary, across every artifact.
  - `an_unset_twig_shell_is_todays_law_on_every_preset` covers 8 presets at seeds 1 and 7.
- **Updated:**
  - Catalogue counts are 259.
  - The identity digests are re-pinned. With the two rows stripped from the text, the family, walk and override digests are fn-179's. The refusal digest's pairs move.
  - Snapshot schema 8; the JSON-default test; the sweep's held rows; the frozen geometry protocol; the Jev dial counts (235 authored, 140 visible); the conformance jitter set holds the shell at 0.8, off its rail's top.

## R3: identity with master

Master `9cbca483` was built in its own detached worktree with its own `target/`. Every shipped preset was compared at seeds 1 and 7, on the skeleton tree with its shed count, the wood, and the leaves (instances, placed, retained), digested over their `Debug` text. All 16 rows are identical. The record is `unset-shell-vs-master.json`.

## Queries (`query-count`, seed 1)

| Tree | Shell | `twig_shell` queries | Scaffold stations | `scaffold_containment` | Nodes |
|---|--:|--:|--:|--:|--:|
| Beech | 1 (today) | 0 | – | 35,726 | 213,044 |
| Beech | 0.4 / 0.25 / 0.15 | 3,937 each | 3,849 | 35,726 | 147,336 / 105,069 / 77,313 |
| Plane | 1 (today) | 0 | – | 2,471,460 | 247,571 (reduced) |
| Plane | 0.4 / 0.25 | 313,551 | 45,530 | 2,883,370 | 244,163 / 247,788 (reduced) |
| Plane | 0.15 | 44,793 | 45,530 | 411,910 | 172,874 |

- **One read per station on the beech:** 3,937 is 129 profile points plus one read for each of the 3,808 stations whose buds fell due.
- **The plane:** its default 250k budget still binds at shells 0.4 and 0.25. fn-180's within-budget search then regrows the tree about seven times, and each regrowth seeds once. The extra containment queries come from those regrowths.
- **Timing without counters** (4 runs each):
  - Beech: 79 ms today, 55 ms at 0.4, 41 ms at 0.25, 33 ms at 0.15.
  - Plane: 740 ms today, 1,080 ms at 0.4, 930 ms at 0.25, 135 ms at 0.15. The middle two are slower than today because of the budget search.

## R5: per order, seed 1 (candidates are experiments, not shipped)

- **Beech:** the shipped `european-beech`.
- **Plane:** `oregon-white-oak` with fn-180's `plane-candidate.json`, which is fn-170's candidate with `girthHold` 0.75 and `girthFall` 4, at the default node budget.
- **Softness:** 0.1 throughout.
- **Units:** twig-law laterals per metre of scaffold wood, by the wood's depth under the smooth outline: outer ≤ 0.15, mid 0.15 to 0.4, inner > 0.4 of the widest radius. Wood metres per bin are in brackets.
- **Scaffold laterals per metre** are unchanged by the shell: beech 0.69 / 0.59 / 0.58, plane 2.43 / 0.70 / 0.79 at parent orders 0 / 1 / 2.

| Tree, parent order [wood m outer/mid/inner] | Today | 0.4 | 0.25 | 0.15 |
|---|---|---|---|---|
| Beech 0, the stem [4/3/19] | 0 / 0 / 1.64 | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |
| Beech 1, the limbs [1/18/148] | 0 / 4.97 / 3.16 | 0 / 4.97 / 0.62 | 0 / 0.65 / 0 | 0 / 0 / 0 |
| Beech 2 [78/255/300] | 8.15 / 6.78 / 6.73 | 8.15 / 6.78 / 1.00 | 8.15 / 3.82 / 0 | 8.15 / 1.05 / 0 |
| Beech 3 [325/541/432] | 8.46 / 7.25 / 7.12 | 8.46 / 7.25 / 1.44 | 8.46 / 4.61 / 0 | 8.46 / 1.52 / 0 |
| Plane 1 [121/839/1118] | 5.72 / 5.80 / 5.41 | 6.55 / 6.62 / 1.54 | 6.55 / 2.76 / 0 | 6.55 / 0.65 / 0 |
| Plane 2 [653/3160/2837] | 8.46 / 7.68 / 7.59 | 9.72 / 8.77 / 2.03 | 9.72 / 4.86 / 0 | 9.72 / 1.30 / 0 |
| Plane 3 [1558/4693/3762] | 9.85 / 9.25 / 9.10 | 11.23 / 10.58 / 2.53 | 11.23 / 6.07 / 0 | 11.23 / 1.83 / 0 |

- **Today:** both trees bear evenly at every depth, as R1 found.
- **With a shell:** the inner bin goes to zero once the shell plus its softness falls below 0.4, and the mid bin thins with it.
- **The plane's outer density rises,** from 5.72 to 6.55 at order 1. With fewer twigs on inner wood, the budget's reduction keeps more detail. At 0.15 the plane fits its budget unreduced.
- **Beech limbs:** 148 of the beech's 167 m of first-order limbs lie deeper than 0.4. A shell at or below 0.25 leaves its limbs bare of twig-law shoots.

**Stills.** 960x720 headless at the hero pose, whole and bare, seed 1, under `raw/stills/` (gitignored):

- `beech-today-s1-{whole,bare}.png` and `beech-shell{0.4,0.25,0.15}-s1-{whole,bare}.png`
- `plane-today-s1-{whole,bare}.png` and `plane-shell{0.4,0.25,0.15}-s1-{whole,bare}.png`
- The overlays are in `raw/families/`.
- All 16 differ by checksum. Two were opened only to confirm they are not blank: beech 0.25 bare and plane 0.25 whole.
- Nothing was opened in the harness beside the photographs; no winner is picked.

## R6: gates and costs

- **`cargo test --profile ci --workspace --no-fail-fast`:** one full run at `3d8ac476` took 314 s: 1,107 passed, 4 failed, 22 ignored. The four failures were new-row bookkeeping: the frozen geometry protocol, two Jev dial counts, and the conformance jitter set. They were fixed in `768feab5`, and the three failing targets were re-run green: telperion-core lib `geometry_benchmark`, telperion-jev `tuning_engine` (39 passed) and telperion-render `conformance` (6 passed). The full gate was not re-run.
- **`npm test`** (after `npm ci`): 14 files and 141 tests passed, in 148 s.
- **Build:** `npm run build` took 58 s. The headless example built in 67 s.
- **Peak RSS of one skeleton build:** beech 76.1 MB today against 39.3 MB at shell 0.25; plane 284.4 MB against 236.8 MB.
- **Artifacts** (`node scripts/artifact-budgets.mjs`, ceilings only, since this is not a PR run). The base column is fn-183's reported final sizes, not a fresh master build.

  | Artifact | Base | Branch | Growth | Ceiling |
  |---|--:|--:|--:|--:|
  | `telperion.wasm` | 1,421,083 | 1,421,976 | +0.06% | 1,600,000 |
  | `telperion-render.wasm` | 1,996,108 | 1,996,787 | +0.03% | 2,200,000 |
  | `telperion-field.wasm` | 369,935 | 369,940 | +0.00% | 400,000 |
  | `telperion.js` | 119,870 | 120,276 | +0.34% | 150,000 |
  | `field.js` / `voxelize.js` | 2,720 / 4,526 | unchanged | 0 | 16,000 each |

## Choices the spec did not settle (for the host)

1. **Smooth outline, not lobed.** This follows shedding, the cull and the fill, which read "one polyline for depth" (`envelope.rs`, `profile`). On the beech, whose irregularity is 0.18, a lobed depth puts 28 / 59 / 92 of 3,849 stations on the other side of shells 0.15 / 0.25 / 0.4: 0.7 to 2.4 percent.
2. **The fade lies past `twigShell`.** Stations within the shell bear as today, and none bear past shell plus softness. R4's "deeper than `twigShell` bears no shoots" holds exactly only at softness 0. The alternative, a fade ending at the shell, would make the neutral depend on softness, because no depth exceeds 1.
3. **"Unset" is a value.** The neutral is 1 at the top of the rail, not an absent option.
4. **Laterals only.** A deep scaffold tip still grows its terminal twig continuation.
5. **Curtains.** A deep station bears no hanging shoots either. No shipped hanging table sets a shell.
6. **Below the crown base,** wood is outside the outline, so it is at depth zero and bears.
7. **On the growth path,** depth is measured against that year's outline.

## R5 follow-up (host, 2026-10-02): values for the hierarchy

Only existing rows were used. The overlays are under `raw/families/grid/`, and no production code or preset changed. Seed 1, at the default node budget. The scratch tool was never committed; its rows are in `raw/hier-grid.jsonl`.

**How the generator maps to the photographs:**
- The photographs' leaders are the generator's order-0 axes: the trunk plus its codominant parts.
- The photographs' second order is the scaffold laterals on those stems.

**The columns:**
- **lead:** the number of order-0 axes.
- **root forks:** forks at the ground, which make a clump rather than a trunk.
- **fork h:** the median order-0 fork height, as a share of the tree's height.
- **lead girth:** a codominant part's start radius ÷ its parent's radius, as a median.
- **limbs/lead:** scaffold laterals per order-0 axis, with their girth as a share of the parent (median).
- **o1 /m:** scaffold laterals per metre of stem.

**The metric:** d is the sum, over the targets, of the distance outside each target range ÷ that range's midpoint. It is 0 when every target is met.
- **Beech targets** (B-BARE): 4 to 6 leaders, fork height 0.3 to 0.4, leader girth 0.4 to 0.6, 5 to 10 limbs per leader.
- **Plane targets** (photograph): 3 to 4 leaders, fork height 0.15 to 0.25, leader girth 0.5 to 0.7, 0.3 to 0.5 laterals per metre on the leaders, their girth 0.3 to 0.5.

**Beech** (shipped table plus `codominance` c, `forkHeight` h, `forkWays` w; 27 sets):

| Set | d | Lead | Fork h | Lead girth | Limbs/lead (girth) | Nodes | ms |
|---|--:|--:|--:|--:|---|--:|--:|
| today | 3.67 | 1 | – | – | 18.0 (0.57) | 213,044 | 79 |
| h0.35 w4.0 | 0.02 | 4 | 0.37 | 0.61 | 10.0 (0.62) | 201–238k | 76–89 |
| h0.35 w3.2 | 0.04 | 4 | 0.37 | 0.62 | 9.0 (0.62) | 204–225k | 77–87 |
| h0.30 w3.2 | 0.06 | 4 | 0.30 | 0.63 | 9.2 (0.64) | 237–245k | 85–382 |
| h0.30 w4.0 | 0.13 | 4 | 0.30 | 0.61 | 10.8 (0.64) | 238–245k | 90–372 |
| h0.25 w3.2 / w4.0 | 0.23 / 0.33 | 4 | 0.26 | 0.66 / 0.61 | 10.0 / 11.5 | 243–249k | 91–400 |
| w2.5, any h | 0.40–0.75 | 3 | 0.26–0.37 | 0.70–0.73 | 10–11.3 | 188–242k | 72–104 |

- **Codominance** (0.3, 0.6 or 0.9) changes none of these numbers. It adds forks on the limbs (4 to 48), and those forks add nodes.
- **ms:** the runs near 400 ms are fn-180's budget search, at 237k to 249k nodes.
- **Out of reach:** five or six leaders. The stem forks once at seed 1, and `forkWays` stops at 4, so leaders are at most 4. That is the low end of B-BARE's 4 to 6.
- **Leader girth** is 0.61 to 0.63 at best, just above 0.6.

**Plane** (fn-180's candidate plus `lateralSpacing` s and `lateralsPerStation` l on a 4×4 grid, then a fork sub-grid at s 3, l 1 or 2, on `forkWays` w and `forkHeightSpread` sp):

| Set | d | Lead | Root forks | Fork h | Lead girth | Limbs/lead (girth) | o1 /m | Nodes | ms |
|---|--:|--:|--:|--:|--:|---|--:|--:|--:|
| today (s1.6 l5) | 6.36 | 7 | 3 | 0.31 | 0.44 | 24.3 (0.19) | 2.43 | 247,571 | 717 |
| l3, any s | 3.76–3.79 | 7 | 3 | 0.31 | 0.44–0.47 | 14.6 (0.24) | 1.46 | 176–245k | 73–679 |
| l1, any s | 1.17–1.21 | 7 | 3 | 0.31 | 0.47–0.49 | 4.9 (0.42–0.47) | 0.49 | 12–245k | 6–465 |
| l1 w2.2 sp0.05 | 0.15 | 3 | 0 | 0.13 | 0.63 | 5.7 (0.34) | 0.52 | 78,731 | 28 |
| l1 w2.0 sp0.05 | 0.39 | 2 | 0 | 0.13 | 0.64 | 7.5 (0.32) | 0.50 | 65,670 | 22 |
| l1 w2.35 sp0.05 | 0.45 | 4 | 0 | 0.13 | 0.35 | 4.8 (0.42) | 0.54 | 91,487 | 33 |
| l1 w2.5 sp0.05 | 0.55 | 5 | 0 | 0.25 | 0.40 | 4.2 (0.50) | 0.54 | 101,902 | 40 |
| l1 w2.5 sp0.15 | 0.75 | 3 | 2 | 0.00 | 0.66 | 6.7 (0.41) | 0.47 | 92,013 | 35 |
| l2, any fork set | 1.76–2.43 | 2–7 | 0–3 | – | – | 8.4–16 | 0.95–1.10 | – | – |

- **`lateralsPerStation`** sets the laterals on the stems: 2.43, 1.46, 0.97 and 0.49 per metre at 5, 3, 2 and 1. At 1 it reaches the photograph's 0.3 to 0.5 per metre and limb girth 0.42 to 0.47.
- **`lateralSpacing`** moves only the second order on the limbs: 0.70, 0.40, 0.26 and 0.18 per metre at 1.6, 3, 5 and 8.
- **Leaders are fork rows, not lateral rows.** The candidate's `forkHeightSpread` of 0.15 draws three forks at the root, which makes a clump of seven stems. A spread of 0.05 removes them.
- **With one fork,** `forkWays` 2.2 gives 3 leaders and 2.35 gives 4. Above 2.2 the fractional part is thin: its girth is 0.35 to 0.40.
- **Fork height:** the first fork falls at 0.13 against the photograph's roughly 0.2.

**Picks (code-proposed by d; the host confirms):**
- **Beech hier1:** `codominance` 0.3, `forkHeight` 0.35, `forkWays` 4.0 (d 0.02).
- **Beech hier2:** `codominance` 0.3, `forkHeight` 0.35, `forkWays` 3.2 (d 0.04).
- **Beech tie-break:** codominance ties on d, so the lowest, 0.3, is picked; it has the fewest forks on the limbs.
- **Plane hier1:** fn-180's candidate with `lateralsPerStation` 1, `lateralSpacing` 3.0, `forkWays` 2.2, `forkHeightSpread` 0.05 (d 0.15).
- **Plane hier2:** the same with `forkWays` 2.0 (d 0.39).

**Stills.** 960x720, seed 1, under `raw/stills/`:
- `beech-hier{1,2}-{today,shell0.4,shell0.25}-s1-{whole,bare}.png`
- `plane-hier{1,2}-{today,shell0.4,shell0.25}-s1-{whole,bare}.png`
- Shell softness is 0.1, and the overlays are `raw/families/<tree>-hier<N>-<shell>.json`.
- All 24 are distinct by checksum. Two were opened to confirm they are not blank: beech hier1 shell 0.25 bare, and plane hier1 today whole.
