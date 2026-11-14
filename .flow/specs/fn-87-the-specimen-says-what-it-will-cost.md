# The specimen says what it will cost

## Goal
Nothing bounded what the species suite held: each heavy test kept `SEEDS_IN_FLIGHT = 4` specimens, cargo ran the four heavy tests together, and on a small box the kernel decided (fn-84's OOM, `signal: 9` read as a failing test). The constant's comment ("a spruce placement is about 350 MB") had drifted three times (941, 470, 88 MB), because the number was written down instead of computed. Baseline at 38e6c57f (ci, whole species binary, VmHWM): 5,695 MB / 49.12 s at 32 threads, 5,602 / 47.76 at 4, 3,486 / 74.45 at 2, 2,589 / 104.03 at 1. This spec makes the specimen say what it will cost, computed from the tree it just grew, and admits seeds against one budget the whole binary shares.

## Decisions
- Owner, 2026-09-19: the memory fix is fn-86; this scheduling work is separate, with the figure calculable from the tree instantly rather than a constant maintained over time.
- The first dispatch returned `BLOCKED: DESIGN_CONFLICT`: the spec claimed the tree carries station and vertex counts and `tree.rs:76` says otherwise; two dispatches lost. Architecture bullets now cite file and line.
- Plan review (gpt-6-astra high, two rounds, 2026-09-19): admitting at least one specimen breaks a universal bound, and four tests each calling `in_flight` would each spend the same budget; the budget became three numbers: the machine's reserve (larger of 2 GB or a quarter of `MemAvailable`), the process ceiling, and a charge limit at two thirds of it, the headroom covering the 15% underestimate and the named transient allocations.
- Counting logic lifted out of the builders into one function each that builder and predictor share; `Tree` gains no field (its bincode feeds the digests). A static thread cap was rejected.

## Acceptance Criteria
- **R1:** A core function predicts resident bytes from the grown tree, building no mesh or matrix, O(nodes) scratch; no nodes predicts zero — met (`footprint::predict`).
- **R2:** One implementation of the counting; builder counts equal predicted per species for the four with presets — met (clumped families asserted at or under).
- **R3:** Prediction within 15% of measured capacity from live objects — met exactly (asserted equal on every seed).
- **R4:** No literal byte count; changing the leaf's size moves the prediction — met.
- **R5:** One binary-wide budget, charge limit two thirds of the ceiling; concurrent mixed admissions never exceed it, panics release, admission never zero — met.
- **R6:** `SEEDS_IN_FLIGHT` gone; peak VmHWM under the process ceiling, a saturated budget included; a lone over-limit specimen is named — met.
- **R7:** Wall time within 15% of 49.12 s at 32 cores; smaller hosts run narrower, not killed — met (26.46, 39.80, 35.47 s).

## Result
Commits 33429197, 75153b12. `surface::extent`, `station::walk`/`station_count` and `short_shoots::count` are shared by builder and predictor; `foliage::place` reserves the crown once, exactly. Three desk runs as free memory fell: ceiling 15,304 / 13,960 / 13,352 MiB, peak VmHWM 12,539 / 11,750 / 11,222 MiB, wall 26.46 / 39.80 / 35.47 s; charge limit saturated within half a percent; resident 1.23-1.27x charged against the 1.5 allowed. Peak is about twice the old 5,695 MB because the suite now spends what the machine can spare. No digest moved.

## Open
Under nextest each test is its own process, so four heavy tests are four budgets reading the same `MemAvailable`; whether the bound should be shared across processes was escalated as a design question.

## Friction
- Admission policy and prediction inputs needed host decisions (two worker blockers) -> answered by the spec's three-number budget and shared counting functions.
- An unchecked architecture claim cost two bridge dispatches -> rule: architecture claims cite the code (now in AGENTS.md).
- The workspace gate ran twice because no command named the gate's profile -> one gate command and profile (now in AGENTS.md).
