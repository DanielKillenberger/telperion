# fn-270 task 1: the premises on the CPU cycle (worker, 2026-10-13)

R1 (a) to (d), R2a and R2a', on fn-255's CPU cycle, before any API or device work. The branch is cut from F4 with the merged GPU engine (`e914f2e0`).

- **Hook:** `grow/cycle/paused.rs`. `Paused` is the existing cycle loop split between years: `begin(species, horizon, retiring, census)`, `grow_to(age)` (refused by name past the horizon, `age.horizon`, and backward, `age.backward`), `settle()` (the tree at the age grown, settled from a copy of what the settle reads), and `digest()` (everything the next cycle and the settle read, to the bit). `grow_cycle` now runs through it: `begin`, `grow_to(age)`, settle. Nothing else changed in the cycle.
- **Retirement without the final age (R3), as a hook:** `Retiring::AtAge` (`Working::continuable`, `retire.rs`) reads a subtree's fade at the age grown so far, `cycle - 1`, not at the horizon. A kept lateral that sheds waits among the fading and is evaluated again once its fade there reaches none. Today's rule stays the default (`Retiring::AtHorizon`); task 2 makes the new one the rule.
- **Tests:** `grow/cycle/paused/tests.rs` (the gate, the engine's oak, spruce and beech at ages 24 to 41) and `crates/telperion-core/examples/premises.rs` (every shipped preset at its default age, one process a tree, `RAYON_NUM_THREADS=4` on cores 0 to 3, one at a time, 8 GB free first). Raw output is in `raw/` (ignored).
- **Byte-equal** means one digest of the Debug text of the whole tree (every float to its last bit, -0 apart), `tree_digest`.

## Verdicts

| Premise | Verdict |
|---|---|
| (a) Every read of the final age | Listed below. Every read is fixed by a horizon given at `begin`, except retirement's, which (c) replaces. |
| (b) The tree at a year is the horizon's prefix | **Holds.** On 6 presets at seed 42, every year's tree of one run, retired at the age grown, is the tree of a run that retires nothing, 40 of 40 years, and on Telperion 82 of 82. |
| (b) No pop in a walk | **Does not hold at shedding (host).** The horizon itself adds none: in years without shedding no old organ moves at all. But each year boundary drops shed branches drawn at a quarter or more whole, as today's trees of each age do. Below. |
| (c) Retirement at the age grown keeps today's tree | **Holds**, 24 of 24 (8 presets, seeds 1, 7, 42). |
| (c) The live set | **Peak:** +11.6% to +18.1%, under 25%. **Per cycle:** up to +90.9% (spruce, cycle 24), over 25% on every preset but the palm. Which one the stop reads is the host's call. |
| (d) 0→H is 0→a→H | **Holds** on 8 presets at seed 42, a = 10, 40, H−1, with two settles and a zero-year advance at a; in the gate also around the first waking, apex end, retirement and change of age. |
| No snapshot mutation | **Holds:** the grower's digest is the same after every settle, at every split and every year of the walks. |
| R2a: fractional settles leave the state; the whole year is today's tree | **Hold.** |
| R2a: just past a year joins the previous year's tree | **Does not hold for old organs (host).** New organs start from nothing; but the tree drawn just past 40 is year 41's in its old organs, and 94,909 of the oak's year-40 organs have moved or gone there (shedding and fades step a whole year). |
| R2a' | **Holds** with the stand-in presentation. |

No same-executor mismatch and no snapshot mutation. **Two things go to the host:** the live-set stop holds on its peak reading and fires on a per-cycle reading; and the year boundary pops where the year sheds, in the walk (b) and in the join (R2a). The preset runner now ends with exit 2 on a live-set reading over 25% and fails on any mismatch, mutation or prefix break (review round 1).

## (a) Every read of the final age, against code

The cycle path is `grow_cycle` → `start` → `Paused::grow_to` → `Grower::year` → `step` (the one-cycle branch) → `settle`. Paths the cycle does not take (`couple`, `oracle`, `stepped`, `realized`, `subtree`, `alone`, the captures) are left out.

| Where | What reads the final age | Under a horizon H |
|---|---|---|
| `grow.rs:255`, `presence.rs:47-50` | `Windows::new(species, age)`: the expected wood of every draw, as a share of the whole tree's at the age | Fixed at H |
| `presence.rs:67, 77, 88, 110, 145` | `wood`, `log_wood`, `wood_left`, `dormant`, `window`: cycles left to the age | Fixed at H |
| `grow/cycle/passes.rs:56` | the balance's shedding window per open axis | Fixed at H |
| `grow/unit.rs:114, 153, 155` | a lateral's window, a unit's expected wood, the sleeping buds' expected wood | Fixed at H |
| `grow/stop.rs:66, 165, 236, 276` | the stop, relay and move windows | Fixed at H |
| `grow/wake.rs:216-217`, `dormant.rs:77` | a sleeping bud is drawn only if it wakes within the age | Fixed at H |
| `grow.rs:392` | `asleep` sized to age + 2 | Fixed at H |
| `grow/cycle/paused.rs:87` (was `cycle.rs:82`) | the last cycle sketches no layout and reads no light | At H; a pause before H has its layout and light (the state's boundary flag, R2) |
| `grow/cycle/retire.rs:136, 251` | **retirement's fade, at the age** | **Replaced:** at the age grown (R3); today's at H drops subtrees a year before H still draws |
| `grow/cycle/retire.rs:248, 316` | a closed subtree's living; a woken bud's kept share | Read no age on this path: the living takes none, and the share reads it only in the waking's own cycle, three before any verdict |
| `grow/settle.rs:18, 30`, `dormant.rs:344` | `mark`: a bud that woke in the last cycle keeps the share its bearer lives on | The settle's age (`settled_copy` sets it) |
| `grow/settle.rs:75, 77, 84, 93` | the tree as grown, the fades and the tree's age | The settle's age |
| `grow/shoot.rs:316` | `shed_certain` | Never reached: the cycle grows every lateral (`lasting` is none) |
| `grow/sketch.rs:130` | the coupling pass's last field | Never reached: the cycle couples nothing |

So the growth law reads H through the windows (the council's finding), and the settle reads the settle's age. Under a horizon the tree at year a is a state of the H run, not today's a-year tree: today's a-year request has windows of a.

## (b) The horizon's prefix, and pops

`premises walk <preset> 42`: one run settled at every year, against a run that retires nothing (`Retiring::Never`) and one retiring as today (`AtHorizon`).

| Preset (H) | At the age grown = none retired | Today's retirement: first year apart | Its settles that failed |
|---|--:|--:|--:|
| date-palm (40) | 40 of 40 | none | 0 |
| european-beech (40) | 40 of 40 | 7 | 0 |
| silver-birch (40) | 40 of 40 | 8 | 0 |
| ordinary (40) | 40 of 40 | 5 | 29 |
| oregon-white-oak (40) | 40 of 40 | 6 | 24 |
| norway-spruce (40) | 40 of 40 | 8 | 30 |
| telperion (82) | 82 of 82 | 7 | 0 |

Today's retirement is not continuable: a year's settle reads a subtree let go while it still fades, and either draws a different tree or panics (`geometry.rs:65`, a lateral on a bearer whose phytomers were let go).

**Pops.** How far each year moves the phytomers of the year before, each known by its axis's identity (`identities`; a relay shares its bearer's lineage) and its place on the axis (`movement`): the most an old one's scale moved, how many moved by over a quarter, and how many over a quarter went. Against today's trees of each age (one request per age), on the 40-year presets:

| Preset | Horizon: most / moved > ¼ / gone > ¼ (of them shed whole) | Today's ages: most / moved > ¼ / gone > ¼ |
|---|---|---|
| european-beech | 0.998 / 2,205 / 197,093 (197,093) | 0.986 / 2,636 / 204,567 |
| silver-birch | 1.000 / 8,194 / 501,397 (501,397) | 1.000 / 11,131 / 503,034 |
| ordinary | 1.017 / 140,203 / 110,874 (110,874) | 1.017 / 136,016 / 107,989 |
| oregon-white-oak | 1.833 / 13,498 / 783,225 (783,225) | 1.720 / 15,579 / 784,079 |
| norway-spruce | 0.972 / 8,947 / 1,578,540 (1,578,540) | 0.971 / 8,951 / 1,564,765 |

Telperion's walk (82 years, horizon only): most 1.000, 140,222 moved over a quarter, 10,883,811 gone over a quarter, every one shed whole.

- **The horizon adds no pop of its own.** In the beech's years 2 to 11, old phytomers move by over a quarter in one year only (year 8, three of them) and otherwise not at all (most 0.000 in eight of the ten years); today's ages move them every year by up to 0.71 as the windows move with the age.
- **The year boundary still pops where shedding steps.** A fade falls by a whole cycle at a time, so a branch drawn at a quarter or more is gone the next year, its whole axis with it (every gone phytomer under the horizon, on every preset). That is today's law, the same in both walks, and it is what R5's presentation within the year meets at every year boundary: a tree drawn just past a year is next year's tree with its new organs at nothing, and its shed branches already gone. **For the host:** whether "no pop" covers shedding's whole-year steps, and if so whether R5 fades shedding within the year as well.

## (c) Retirement at the age grown

`premises kept <preset> <seed>`: today's `grow_census` against `Paused` retiring at the age grown, at the preset's default age.

| Preset (H) | Kept tree the same (seeds 1, 7, 42) | Live peak, today → at the age (seed 42) | Peak rise (1 / 7 / 42) | Worst cycle's rise (1 / 7 / 42) |
|---|---|--:|---|---|
| date-palm (40) | yes, yes, yes | 480 → 480 | 0 / 0 / 0 | 0 / 0 / 0 |
| european-beech (40) | yes, yes, yes | 312,699 → 361,172 | 15.8% / 14.1% / 15.5% | 40.8% / 47.9% / 39.9% |
| silver-birch (40) | yes, yes, yes | 351,806 → 400,965 | 14.5% / 16.1% / 14.0% | 33.6% / 28.0% / 30.2% |
| ordinary (40) | yes, yes, yes | 163,296 → 182,261 | 11.8% / 11.8% / 11.6% | 39.3% / 39.0% / 38.4% |
| oregon-white-oak (40) | yes, yes, yes | 361,337 → 423,055 | 16.2% / 15.8% / 17.1% | 74.0% / 66.2% / 75.5% |
| norway-spruce (40) | yes, yes, yes | 1,104,734 → 1,247,515 | 12.9% / 12.7% / 12.9% | 90.0% / 90.3% / 90.9% |
| laurelin (80) | yes, yes, yes | 2,788,511 → 3,252,119 | 18.1% / 17.1% / 16.6% | 42.0% / 47.9% / 40.7% |
| telperion (82) | yes, yes, yes | 2,967,969 → 3,453,984 | 18.1% / 17.0% / 16.4% | 42.0% / 47.9% / 40.5% |

The live set is the phytomers held at each cycle's end (`Census::cycles`). Retiring at the age grown holds a subtree until its fade reaches none, where today's let it go as soon as it closed if its fade at H would be none; so every cycle holds more, most in the middle cycles (the spruce worst at cycles 24 to 28), and least at the end, where the age grown nears H. The peak is in the last cycle on every preset, both ways. The per-cycle lists are in `raw/kept-*.txt` (`LIVE`).

**The stop, for the host:** "a live-set rise over 25%" holds on the peak (at most +18.1%, Telperion seed 1), which is what an arena sized once is sized by. Read per cycle, it fires on seven presets (the palm retires nothing). This worker does not choose the reading.

## (d) Grown on from a pause

`premises split <preset> 42`, a = 10, 40, H − 1 (40 is the horizon of the 40-year presets): grown to a, settled, advanced by no year, settled again, grown on to H, settled.

| Preset (H) | Splits | Settles repeat | Grower moved | Grown on = grown at once |
|---|---|---|---|---|
| date-palm, european-beech, silver-birch, ordinary, oregon-white-oak, norway-spruce (40) | 10, 39 | yes | no | yes |
| laurelin (80) | 10, 40, 79 | yes | no | yes |
| telperion (82) | 10, 40, 81 | yes | no | yes |

The gate (`a_tree_paused_and_grown_on_is_the_tree_grown_at_once`) also splits the oak, spruce and beech at H = 24 the year before and the year of the first waking, apex end, retirement and change of age past the third year, each with repeated settles and a zero-year advance; every one is the tree grown at once, and none moves the grower.

## R2a and R2a', with a stand-in presentation

`a_tree_drawn_within_its_year_unfolds_from_the_last`, the oak at horizon 41, seed 7. The stand-in draws this year's phytomers at the fraction of the year, every other one whole; the curves are task 4's. A request within the last year grown is met (committed − 1 < a ≤ committed); one a year or more back is refused (`age.backward`).

- 40 → 40.25 → 40.5 → 41 commits cycle 41 once, at 40.25; the later requests commit nothing.
- No settle at 40.25, 40.5 or 41 moves the grower's digest.
- The settle at 41 is the tree `grow_cycle` grows to 41, to the bit.
- Just past 40 (40 + 1e-9), this year's phytomers stand at 1e-9 of their whole size or less: new organs start from nothing. The old organs are year 41's (asserted), so they differ from year 40's by the year's moves above: 94,909 of the oak's year-40 organs moved or went (`JOIN`, `raw/t1-r2a.log`). The stand-in scales only `scale`; new shoots keep their whole-year tips and radii, which task 4's curves replace.

## Cost

| Run | Seconds | Peak |
|---|--:|--:|
| `kept` Telperion (today's then at the age, seed 42) | 28.4 | 3.78 GB |
| `split` Telperion (three splits) | 96.3 | 4.15 GB |
| `walk` norway-spruce (three walks of 40 settles, 40 of today's ages) | 246.0 | 2.35 GB |
| `walk` Telperion (three walks of 82 settles) | 404.0 | 7.64 GB (the run retiring nothing) |
| Gate tests (`grow::cycle::paused`, five, one thread) | 60 | |
