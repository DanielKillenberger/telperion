# The oak at 40, seed 7: the long, zigzag low limbs (cause)

The owner (2026-10-15, species review): "seed 7 specifically doesn't look like a real tree. Lower branches that grow to long and zigzag weirdly." This note covers measurement and cause only. No fix is made here; choosing one is the host's design decision.

Base: F4 at c09f9798a, preset `oregon-white-oak` at age 40, engine CPU cycle. Probe: `seed7/oak_lines.rs`, copied into `crates/telperion-core/examples/` for a run and removed afterwards. It walks `telperion_space::grow`'s axes. A *stem* is the seed's axis and every axis carried on from it. A *limb* is a lateral off a stem. A limb's *line* is the limb plus the axes it carries on through (continuations, and relays at their node). Summary script: `seed7/summary.py`. Raw lines: `raw/seed7/*.jsonl` (ignored). Stills: `seed7/probes-az0.jpg`, which shows base, probe A and probe B left to right, viewed.

## What the low limbs are

They are not the scaffold. The scaffold limbs (PA 6, `limb`) leave the trunk at 4.6-5.7 m and rise to 9-10 m. The long low limbs are the young stem's laterals:

- PA 8, `sprig`, born at cycles 1-14 on the juvenile trunk (`oregon-white-oak.values:218,222,252`) at 0.4-4.9 m.
- Each sprig grows its 0.07 m internodes for 10 cycles (`:400,403`), about 3.5 m of nearly unbranched wood.
- It then carries on into PA 9, `branch` (`oak.rs:292-325`, `state(10, 1.0, …)`, continuation 1).
- PA 9 relays (`:445`, relay, relayEnded and relayFailed all 1), then ends in PA 12.

At seed 7 there are 23 such lines. 19 of them are 3 m or longer (4-6.5 m), and 17 end beyond the crown's 95th-percentile radius (3.12 m). They reach up to 4.99 m, 1.60 times that radius. They carry 127-8,563 phytomers each, against 25,000-110,000 for a scaffold limb. So they are long, thin wands with leaf only at their current year's shoots (`leafYears` 1, `shootRadius` 0.007).

## Seed 7 is the tail of a distribution, not an outlier (seeds 1-50, age 40)

| per tree | median | p90 | max | seed 7 (rank of 50) |
|---|---|---|---|---|
| sprig lines (PA 8) | 20 | 25 | 28 | 23 (10) |
| … 3 m or longer | 15.5 | 20 | 24 | 19 (10) |
| … tip beyond crown95 | 13 | 16 | 20 | 17 (3) |
| farthest tip / crown95 | 1.40 | 1.57 | 1.63 | 1.60 (3) |
| sprig-line wood (m) | 85 | 109 | 126 | 110 (4) |
| relay turns ≥ 40° on them | 1 | 2 | 4 | 3 (2) |
| kinks > 45° (0.5 m chords) | 0 | 1 | 2 | 2 (1) |

Every seed grows 13-28 of these wands. Seed 7 sits near the top on reach and on relay turns, and it has the most measured kinks.

## The zigzag: a relay heads off its axis by its PA's full insertion angle

- When an apex stops (it ends, fails or aborts), a relay bud of the same PA is made (`grow/stop.rs:260-297`, `fail` at `:100-135`). Its blend is the presence of the stop (`stop.rs:125,280`), which is 1 for a whole stop.
- Its heading is a mix of the parent's line at the tip and "its own bud … facing where a bud there would" (`geometry.rs:192-230`). At blend 1, that heading is the bud direction alone.
- The bud direction is `turned` (`geometry.rs:343-361`) by `angle()`, which is the relay's own PA's `insertion` (`geometry.rs:307-310`), then tilted up by epitony.
- Nothing afterwards turns the relay back toward the line it replaced. `straightening` acts towards the vertical, and PA 9 has no straightening. PA 9's tropism pulls it towards elevation 0.2 rad (`:443` and on).

Measured: each relay's turn from the axis it replaces equals its PA's insertion.

| PA | insertion | measured turn |
|---|---|---|
| 9, branch | 1.0 rad | 49-57° |
| 6, limb | 0.5 rad | ~29° |
| 7, bough | 0.6 rad | 34° |

On the sprig lines, the 57° relay is a hook about a metre long in wood that later thickens. In the still it is the dark near-vertical piece on the left limb.

**Probe A**, no PA-9 relays (`/space/ages/9/relay`, `relayEnded` and `relayFailed` all 0):

- Relay turns of 40° or more drop to 0 at every seed, and kinks drop to 0.
- In the still, the left hook is gone.
- The right limb's elbows remain. Those are PA 6 relays at 29°: the scaffold's intended "kinks at each relay: the oak's tortuous course" (`oak.rs:196-201`, CIR).
- Side effects: seed 7 falls from 371,210 nodes to 302,426 and from 70,244 leaves to 55,587, and the crown narrows from 9.5 m to 8.4 m.

## Why the wands are long and stay

- **Length:**
  - A sprig's internode is 0.07 m, twice the scaffold's 0.035 m and the bough's 0.03 m. `oak.rs:292`: "long shoots of about 0.35 m a year for ten years". The source for that is not cited; SOURCES.md is not checked here.
  - After ten years the sprig carries on as a branch instead of ending.
  - **Probe B**, sprig internode 0.035:
    - Median lines beyond crown95 drop from 13 to 1, and seed 7 from 17 to 5.
    - Farthest reach drops from 1.40 to 1.11 times crown95.
    - Sprig wood drops from 85 m to 46 m.
    - Seed 7's leaf width drops from 9.5 m to 7.2 m.
    - Node count barely moves (381,527).
    - The still loses the long low wands.
- **Persistence:**
  - The line keeps a living apex because PA 9 relays when its age ends. `shed.rs:17-19`: "Life is unbounded … where an age that ends relays".
  - Nothing removes a low branch that is overtopped by the crown. This is the mechanism fn-282 names for the birch, where low branches read 0.85-0.98 light because they stand outside the crown.
  - The oak's low-wand light was not measured: `Phytomer::light` is crate-private. It is **unknown**, not assumed.

## Candidate fixes (general, by degree; the host chooses)

1. **A relay takes over its axis's line by degree.**
   - Today a relay at blend 1 heads at its full insertion angle (`geometry.rs:212-226`). The fix makes its heading run back toward the line it replaces as it grows: Troll's take-over, in which a sympodial axis straightens into a near-monopodial line as it thickens. This would be either a new per-PA row (how far a relay turns off the line, neutral at today's value) or the relay's straightening read towards its parent's line instead of the vertical.
   - Dormant where no PA relays: the spruce and palm.
   - It changes the geometry of every preset that relays: beech, birch, ordinary, Telperion, Laurelin and oak (relay rows in 6 of 8 value files).
   - With a neutral default, only rows that are set change anything.
   - It cures the hook without removing relays. Probe A shows that removing them costs about 20% of the oak's leaves.
2. **Low branches under the crown die by degree (fn-282, post-release by the owner's word).**
   - This is the general route for wands that persist.
   - It would also shed many of the oak's sprig lines.
   - It is larger and needs sources first (fn-282 R1).
3. **Oak values only: sprig length.**
   - The sprig's internode, 0.07 m (estimated, `oak.rs:292`), set from a sourced annual extension of a young Q. garryana's laterals.
   - Probe B shows that halving it removes the wands' reach.
   - No other species changes. It is a value, not a mechanism, so it needs a source rather than an effect.
   - It does not cure the 57° relay hook. That is milder in probe B because the relays stand nearer the trunk, but kinks remain at seed 7 (1).

The hook and the reach are separate causes, and each probe cures one of them. A release fix for "too long and zigzag" needs one candidate from {1} and one from {2, 3}.
