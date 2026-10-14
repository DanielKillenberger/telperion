# fn-276 fixes 1 and 3: the floor and the node table, measured (worker, 2026-10-14)

Code: 4ee9bf0d6. Raw output is in `raw/` (ignored): `f1-walks-3.log`, `f1-presets.log`, `f1-presets-rest.log` (running), `f1-ab.log`, `f3-bt.log`, `f3-r1.log`.

## The closed form (`telperion_space::most_units`)

It bounds the units one job grows in a cycle:

- **Ages:** from each age with a lifespan, along its successors, an age is entered while those entered before it sum to one cycle at most on the tick grid.
- **Per age:** each age grows one unit, and one more where its unit can fail and a relay take the failure over (`relay_failed > 0`, `viability < 1`).
- **Where an age ends in the cycle:** its ended relay's whole unit and that unit's own failure. These count beside a continuation that is not certain (`go < 1`), or in its place where the apex stops.

The device's limits take it as a floor:
- units rounds: 2u - 1;
- layout and hierarchy generations: u.

A floor past the 6 axes a units job holds (`job.wgsl`'s `FRAMES`) is refused by name before submit.

The CPU census now counts the most units a living job and a waking job grew each cycle (`Cycle::jobs`).

## The check against the CPU's count (`tests/units_bound.rs`)

No case is under.

| Cases | Bound | Most counted, living / waking |
|---|---:|---|
| Ordinary 1 to 80, seeds 1, 7, 42 | 6 | 3 / 3 |
| Oregon white oak 1 to 80 | 4 | 2 / 2 |
| Norway spruce 1 to 78 | 2 | 1 / 1 |
| The film's six walks at t = 0, 0.01, 0.1, 0.5, 0.9, 1, three seeds (108 frames) | 1 to 15 | at most 4 / 4 |

The rest of the presets is still running: the spruce from 78, the beech, birch and palm, and the Two Trees. The first run was OOM-killed at the spruce at 78 (FRICTION.md).

## Open for the host: the bound is about twice the count, and that costs

**Where the looseness comes from:** each age's failure term. The bound without it ("ages and ended relays") equals the CPU's count on every walk frame where the two meet (4 and 4, 3 and 3). It is not a bound, though: one failure relay on a unit in the chain adds a unit to the CPU's count.

### Cost

Warm build time, median of three runs alternated with each other (`f1-ab.log`), in ms:

| Preset | No floor | Floor | Floor without failure term |
|---|--:|--:|--:|
| Telperion 82 | 1,319 | 1,403 (+6.4%) | 1,318 |
| Beech 40 | 206 | 203 | 220 |
| Oak 40 | 241 | 242 | 246 |
| Ordinary 40 | 489 | 476 | 502 |

- **Telperion:** its floor is 4 units, so 7 rounds every cycle, where the rounds used plus slack give 3. That costs 84 ms, more than fn-256 task 12 saved across every preset (2 to 3.5%). The Laurelin is the same species.
- **The other presets:** within the runs' spread.

### The cap

The floor passes the 6 frames on six walk frames. The CPU grows each of them with at most 4 units a job:

| Walk frame | Floor |
|---|--:|
| Date palm to Laurelin at 1, t = 0.01 | 15 |
| Date palm to Laurelin at 1, t = 0.1 | 10 |
| Spruce to birch at 40, t = 0.01 | 10 |
| Spruce to birch at 40, t = 0.1 | 9 |
| Beech to spruce at 40, t = 0.9 | 10 |
| Oak to palm at 40, t = 0.9 | 10 |

Under the ruling they are refused by name before submit, which conflicts with R3 and R4.

### Options for the host

- **(a) Keep the bound and the 6-frame cap.** Those six frames are refused by name, and Telperion and the Laurelin pay about 6%.
- **(b) Raise `FRAMES` to the most floor a parameter set reaches.** Each frame is 35 words a job: 9.2 MB a frame at a full range of 65,536 jobs, and less where a build's jobs are fewer. 16 frames would cost about 92 MB at full range. The cost stays.
- **(c) Take the floor without the failure term.** No cost was measured on the presets. A failure relay can then pass the floor, and the check stops being a bound check; the device refuses by name at UnitRounds where the jobs also pass the observed rounds and slack.

  It still passes the cap on three frames: palm to Laurelin t = 0.01 (11), spruce to birch t = 0.01 (7) and oak to palm t = 0.9 (7).

  Even the bare chain of ages is 9 at palm to Laurelin t = 0.01: nine sliver ages within one cycle. An apex that reaches them in an older tree needs more than 6 frames on any reading.
- **(d) A tighter bound on failures.** For example, only a unit whose share leaves room for the relay's rest. This needs work in the schedule's time model.

## Fix 3: the node table

The node table now grows by the box the dispatch span projects (`given::boxed`), at `ROOM`. The Laurelin at 20 builds on the device: 2,610 instances, the CPU's 2,610.

Peak MB (`build_time`, `f3-bt.log` against `f1-bt-floor.log`):
- **Telperion:** 2,484 to 2,585 (+101), light depths 66 to 149.
- **Laurelin:** 2,281 to 2,374 (+93).
- **Every other preset at its age:** unchanged.

The ledger is not touched. These tables are charged through the existing budget path. The fn-270-grow-budget branch's `buffer.rs` fix is uncommitted there and is not merged here.

R1's two tests pass (`f3-r1.log`): the six walk frames and the Laurelin match the CPU's instance counts.
