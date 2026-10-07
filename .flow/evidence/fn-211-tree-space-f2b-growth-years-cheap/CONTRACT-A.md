# fn-211 task 7: option A's ceiling, then its arithmetic contract (worker, 2026-10-07)

**Revised in task 8 (2026-10-07)** after Astra's review (ASTRA-REVIEW-CONTRACT-A.md, binding): Part 2 is rewritten point by point, each change cited to the review, and its first proof is built (2.13). Part 1 is unchanged.

Option A (TASK5-GPU.md; ASTRA-REVIEW-OAK-GPU.md, point 1): the light-reading oak's fine roots step on the device, and identity by lineage holds by construction because every value that can reach a decision is computed in one specified integer arithmetic on both executors. This file measures what A could move to the device, then writes the contract for the host's and Astra's review. It builds nothing.

## Answer in brief

- **Measured: A could move about half of the stepped oak's 80-year build on all cores, so the contract is written.** The task's stop is a movable share under 30 % on all cores.
  - **All cores:** 50 to 56 % (seed 1: 53.6 and 56.3 %; seed 7: 49.7 and 51.2 %).
  - **One core:** 57 to 74 %.
  - **Without the re-lay**, which A moves only if the fine roots' re-lay and sag go to the device too, the share is still 34 to 38 % on all cores and 47 to 59 % on one core.
- **What stays on the CPU on all cores:** 28 to 33 % of the build.
  - **The settle** is most of it: 21 to 26 %.
  - **The coarse grower's passes, the handover, the lattice's box and the coarse leaves' deposits** come to 6 to 7 %.
  - **The sweep** is about 1 %.
- **The stepped executor's own glue:** 15 to 17 % on all cores. That is gathering leaves, writing light back, the re-lay's assembly and dispersal, and the start. A device executor holding the roots resident would not have it.
- **The ceiling (estimated):** a free device leaves about 0.3 of the stepped build, which is the CPU's 28 to 33 %. Against today's `grow`, that is roughly 0.3 to 0.45 of today's oak at 80 on all cores. It rests on the stepped executor's 1.1 to 1.4 times today's (PHASES.md), before TASK5-GPU.md's 113 to 282 ms synchronisation floor and the device's own work.
- **The contract (Part 2)** covers Astra's table: positions and frames, re-lay and sag, deposits, sweep, sampling, wood and balance, allocation and sizes. Each is specified with its widths, rounding and order. Its main choices are for the host and Astra to review:
  - **One fixed-point arithmetic** for every computation a cut can move, on every axis (not only fine ones), so the cut stays a work partition.
  - **Formats:** Q32 in i64 for positions, directions and per-axis reals; W (Q64.64, i128) for powers of radii, masses, moments and tree-wide sums; T (u32 turns) for orientations and A (i64 Q32 turns) for amounts; 256-bit exact intermediates (revised, 2.2).
  - **Transcendentals:** CORDIC sin, cos and atan2; `isqrt`; the existing `fixed` exp and log, extended to W (each now specified to the bit, 2.3).
  - **Exact sums** whose refusal is independent of order: a bound proven before accumulation, or a saturating sum of magnitudes (revised, 2.3).
  - **The sweep stays f64 on the CPU** with its output rounded once to Q32, since no cut moves it (a proposal; a fixed-point sweep is specified if it moves to the device).
  - **Corner weights** that sum to one exactly, and deposits that conserve area by giving the eighth corner the rest (revised, 2.6).
  - **D16, the balance's shedding,** made an integer threshold with a Q32 kept share.
- **No product code changed.** The instrumentation was reverse-applied, and `git diff -- crates/` is empty.
- **Task 8:** the contract revised against Astra's review (Part 2), its arithmetic built in `fixed::feedback` with a WGSL mirror, and the conformance probe and the feedback slice green on the GPU, word for word (2.13). One design question is open for the host: one layout implementation for the fixed-point rough layout and the f64 final lay (2.14).

## Part 1: the ceiling, measured

### What "movable" means here

The task's definition: the fine roots' own work, which A would move to the device, against what stays on the CPU. Each part of the stepped executor's cycle (`grow/stepped.rs`) is timed as wall seconds, summed over the 80 years, with these classes:

| Class | Parts |
|---|---|
| **Movable** | the items' share of phase (i), their totals and their `shed_by`; their share of (ii) and (iii), inputs, split and partial sums, and setting sizes; their growth and waking; their sketch (rough layout); the fine leaves' deposits; the fine buds' reads; the re-lay's passes at the fine roots' share of the re-laid phytomers (99.5 to 99.6 %) |
| **Stays** | the coarse grower's balance, allotment (with the merge of partials and its own sizes), growth, waking and sketch; the handover; the lattice's box and density; the coarse leaves' deposits; the sweep; the coarse buds' reads; the re-lay's coarse share; the assembly and settle |
| **Glue** | the stepped executor's own: gathering leaves and tips, writing light back, the re-lay's assembly and dispersal, the years' time outside the timed parts, and the start before the first year |

- **The re-lay is apportioned, not timed apart.** It runs once on the assembled tree, and its passes (settle, place, levers, sag and write-back) are per-axis and per-phytomer. So its time is split by the fine roots' share of the re-laid phytomers. Every row also gives the share without it.
- **Deposits and reads are timed apart.** The coarse grower's leaves and tips come first in the shade's lists. The probe deposits the two runs of leaves in separate timed loops, which is the same lattice because `fixed::sum` is order-free, and reads the two runs of tips in separate fills.
- **Same tree:** the instrumented build passes `stepcost`'s "same tree: true" (oak 40, seeds 1 and 7).

### Method

- **The patch:** `ceiling-instrumentation.patch` is task 6's `hybrid-instrumentation.patch` extended. It adds sub-phase timers in `balance`, `allot`, `shade` and `relay`, a timed copy of `Field::on` (`probe_on`, `light.rs`), and counts of leaves, tips, lattice nodes and re-laid phytomers. It was reverse-applied after the runs.
- **The probe:** `hybrid-probe.rs` as `examples/hybrid.rs`. The lit oak (extinction 0.5, sky 0.5) at 80, seeds 1 and 7, cut 1,024, forward order, `HYBRID=1`.
- **The runner:** `ceiling-run.sh` waits for a one-minute load under 8 before each run and records the load either side.
- **The tables:** `ceiling-tables.py`. The warm build (the second in the process) is used. Every "build" excludes the per-phase hash, `mark`, as HYBRID.md does.
- **The machine:** Ryzen 9 5950X, 32 threads. "One core" is `taskset -c 0`.
- **Two sets, `a` and `b`,** each run started at load 5.9 to 7.9. The load rose to 10 to 17 during several runs, from other workers. Set `h` (one core, at load 46 to 18) is kept as a third reading.
- **The seconds** are inflated by the desk's load (seed 1 on all cores: 6.0 to 6.1 s here, against task 6's 4.65 s at load 5.5 to 6). The shares are the measurement.

### The shares (percent of the build, no marks)

| Oak 80 | Set, load | Build s | **Movable** | Movable, no re-lay | Stays | of it: settle | Glue |
|---|---|--:|--:|--:|--:|--:|--:|
| s1, all cores | a, 7.5 → 12.3 | 6.09 | **53.6** | 38.2 | 31.2 | 23.3 | 14.6 |
| s1, all cores | b, 5.9 → 10.2 | 6.04 | **56.3** | 36.3 | 28.1 | 20.6 | 15.1 |
| s7, all cores | a, 7.1 → 10.6 | 11.30 | **49.7** | 33.7 | 32.9 | 25.7 | 16.8 |
| s7, all cores | b, 7.9 → 16.6 | 10.00 | **51.2** | 36.8 | 33.3 | 25.9 | 14.9 |
| s1, one core | a, 7.1 → 9.6 | 13.50 | **64.0** | 49.1 | 30.4 | 26.9 | 5.5 |
| s1, one core | b, 7.4 → 13.7 | 17.34 | **56.6** | 46.6 | 38.5 | 35.3 | 4.7 |
| s1, one core | h, 46 → 35 | 25.02 | 70.1 | 54.5 | 23.7 | 20.1 | 5.9 |
| s7, one core | a, 7.9 → 5.9 | 25.05 | **67.8** | 54.4 | 26.8 | 23.0 | 5.3 |
| s7, one core | b, 6.9 → 6.8 | 32.11 | **74.0** | 58.7 | 19.8 | 15.5 | 6.0 |
| s7, one core | h, 35 → 18 | 31.46 | 69.8 | 55.6 | 25.0 | 21.2 | 5.0 |

**How the movable share divides** (percent of the build; all cores, sets a and b; one core, sets a and b):

| Part | All cores | One core |
|---|--:|--:|
| Items' balance (i) | 4.5 to 5.9 | 5.6 to 8.1 |
| Items' allotment (ii, iii) | 5.7 to 6.7 | 7.7 to 10.3 |
| Items' growth and waking | 10.9 to 12.7 | 20.6 to 25.2 |
| Items' sketch (rough layout) | 6.1 to 6.9 | 9.6 to 13.0 |
| Fine leaves' deposits | 6.1 to 7.0 | 2.4 to 2.8 |
| Fine buds' reads | 0.2 to 0.4 | 0.3 to 0.4 |
| Re-lay, fine share | 14.5 to 20.0 | 10.0 to 15.3 |

**What stays** (all cores):
- the settle with its assembly, 21 to 26 %;
- the coarse grower's passes, the handover, the lattice's box and the coarse deposits and reads, 6 to 7 %;
- the sweep, 0.8 to 1.3 %.

What the tables say:
- **On all cores the movable share is 50 to 56 %, above the 30 % bar in every run.** Without the re-lay it is 34 to 38 %, still above.
- **One core shares vary more** (57 to 74 %), because core 0 was shared with other workers during the runs (seed 7's builds: 25.1 s and 32.1 s). Every run is above the bar.
- **On all cores the deposits weigh more than on one core** (6 to 7 % against 2 to 3 %). They are one serial loop (`deposit_all`) while the rest spreads over the cores. A device would deposit in parallel. How it accumulates 128-bit W sums on 32-bit atomics is the device design's question (unknown here).
- **The settle is the largest part that stays** (21 to 26 %). After A, it is the next lever, and it is CPU work (HYBRID.md, PROFILE.md).
- **The glue (15 to 17 % on all cores) is not counted as movable.** A resident device executor would not gather, assemble or disperse. If it is counted, the share is 66 to 71 %.

### The ceiling (estimated)

- **The best case:** a device that does the movable work in no time, and has no glue, leaves the stepped build's "stays", 28 to 33 % on all cores. That is about 1.7 to 2.0 s for seed 1 and 3.3 to 3.7 s for seed 7 at these runs' load.
- **Against today's `grow`:** the stepped executor is 1.1 to 1.4 times today's per year (PHASES.md, measured at another load). So the ceiling is roughly 0.3 to 0.45 of today's oak on all cores. This is an estimate, not a same-run comparison.
- **What the ceiling omits:**
  - TASK5-GPU.md's synchronisation floor of 113 to 282 ms at 80, which is 3 to 8 % of today's seed 1;
  - the device's own kernels;
  - the lattice's traffic.
- **What the ceiling says:** A is worth its sessions only if the device's work and traffic stay well below the 50 % it removes. HYBRID.md's delete verdict came from a 8 to 10 % ceiling; this one is five to six times that.

## Part 2: the contract (revised after Astra's review, task 8)

Astra accepted the contract with changes (ASTRA-REVIEW-CONTRACT-A.md, binding). This part is the revised contract. Each change cites the review's point:
- **R1.1 to R1.6:** section 1's six defects, in its order: intermediates that cannot fit; a wrapped sum that hides its value; angles without semantics; the radian constant; small-vector normalisation; the missing zero cases.
- **R2.1 to R2.7:** section 2's seven choices.
- **R3:** section 3, what changes in today's trees. **R4:** section 4, the smallest first step.

Every operation below is implemented on the CPU in `crates/telperion-space/src/fixed/feedback/` and mirrored on the GPU in `crates/telperion-render/src/generation/fine/feedback*.wgsl`. Where this text and that code differ, the code's tests show which; the conformance probe (2.13) holds the two executors to each other word for word.

### 2.1 What the contract binds, and what it leaves alone

- **Bound: every computation a cut can move between executors** (unchanged). A tree must be the same at every cut (FINE-CONTRACT.md, Astra's change 2). So balance, allotment, growth, the rough layout, deposits, sampling and the re-lay run in this arithmetic on every axis, above the cut as below it.
- **Every parameter path, dormant ones included (R2.1).** The arithmetic covers every supported parameter combination, not only the paths today's four lit oaks exercise. A path dormant in an unlit tree (shade survival, leaf-light girth, the balance) is specified as fully as an active one.
- **No extra rounding at a handover (R2.1).** A value exchanged across the cut (a fine root's pipe to its bearer, a subtree's light and cost, a vigour) is the same integer either grower would hold. It is never converted, re-rounded or re-scaled because it crossed.
- **Not bound: the stages no cut can move.**
  - **The sweep** stays the CPU's f64 code, run once per cycle on the integer lattice and rounded once (2.6).
  - **The settle's final lay**, whose outputs cannot return to growth or decide identity. Its outputs that do decide identity are already integers (D17's fades) or stay above the cut (D19).
- **One layout implementation (R2.1; host decision on 2.14, 2026-10-07: option (a)).** The rough layout and the final lay are one fixed-point layout. The final lay converts to f64 only at its output, so final geometry is integer-identical on both executors too. The settle's cost in fixed point is measured against today's f64 settle, and reported to the host if it exceeds 1.3 times.
- **Unchanged:** the decision contract already built (D1 to D18, ticks, Q62 probabilities, Q32 logs and presences). The feedback arithmetic extends `fixed`, and the GPU mirrors it in 32-bit words, as F1's chain grower mirrors `fixed` today.
- **Phases:** Astra's four phase boundaries, same-cycle waking and the re-lay every 10 cycles stay. No phase is fused before equivalence is shown.

### 2.2 Formats

| Name | Width | Meaning | Used for |
|---|---|---|---|
| **Q32** | i64, 32 fraction bits | `n / 2^32` | positions, lengths, unit components, scales, sizes, presences, light, vigour, wood, Q and C, λ, ψ, logs |
| **W** | i128, 64 fraction bits | `n / 2^64` | pipe sections, leaf-girth terms, masses, moments, lattice deposits, every tree-wide sum |
| **T** | u32, wrapping, a turn is 2^32 | an orientation | azimuths, phyllotactic positions, the elevation read from a direction, sag's fall as read by sin and cos |
| **A** | i64, Q32 turns, never wrapping | a signed angular amount | rotations, rates times ranks, elevation differences, wander, epitony's turn (R1.3) |
| **Q62** | u64 | a probability (as today) | survival, abortion, the balance's hazard |
| **ticks** | u64 | 2^-20 cycle (as today) | every time |

**Wide intermediates (R1.1)** are 256-bit two's complement integers (`wide::I256`). They hold an exact product of two i128, so of four Q32 factors (raw up to 2^252), a Q32 position times a W mass, and a W numerator shifted for a quotient. No intermediate is checked after it could have wrapped: each is exact, then rounded once, then narrowed with a check (2.3).

**One position domain (R2.2).** Every position component lies below 2^12 m (raw 2^44 in Q32), checked where a step's next point or a leaf enters the lattice, refusing by name. Every other bound below is derived from this one. The earlier text had 2^20 m in 2.10 and 32 m in sag's moment bound. 2^12 m is the worker's choice of value: far above any tree, and small enough that the lattice's node indices fit an i32. The host may change it; nothing else in the contract depends on its value except the derived bounds in 2.10.

**Species values and per-request constants** are converted once on the CPU and supplied to both executors as integers (unchanged):
- Reals go to the nearest, half away from zero (`fixed::q32`'s rule at each format's scale).
- Angles in radians go to A as `round(a / 2π · 2^32)`, and to T as that, wrapped.
- Exponents `e` and `1/e` go to Q32 separately.

### 2.3 Operations: one rule each

Every operation has one result, specified by that result. Where the result depends on the algorithm (CORDIC, the logs), the algorithm is the specification.

**Rounding and range (R1.1, R2.3).**
- **A product** of Q32 factors is exact in 256 bits and rounded once: `prod_shr(f0, f1, f2, f3, n) = floor(f0 f1 f2 f3 / 2^n)`, narrowed to i64. A missing factor is the integer 1. A chain the source writes as successive products is successive `prod_shr`s, one floor each, only where 2.4 to 2.8 say so; elsewhere a formula's product is one rounding.
- **A W product** `mul_shr(a, b, n) = floor(a b / 2^n)`, exact in 256 bits, narrowed to i128 (Q32 × W with n = 32, W × W with n = 64).
- **A quotient** `ratio(a, b, n) = trunc(a 2^n / b)`, the numerator exact in 256 bits, truncated toward zero (both executors' integer division), narrowed to i128. A W over W quotient in Q32 is n = 32. A quotient of a product by a divisor (`split`'s `v (1 − l) sub / split`) is one exact 256-bit numerator and one truncating division.
- **Narrowing** a result outside its format is `Fault::Range`; a division by none is `Fault::Zero`. Every site states what it does with each. On the feedback path a fault refuses the tree by name; nothing wraps or saturates silently.
- **Two signed shifts (R2.2).** `floor_shift` is the arithmetic shift: floor, so a tiny negative becomes −1. `zero_shift` shifts the magnitude and keeps the sign: truncation toward zero, symmetric about zero. Each site names which it uses. Scaling a vector or a lever uses `zero_shift`, so no direction is biased.
- **Probability clamps are pointwise (R2.3).** A probability formed as `P_ONE − 2^-x` lies in [0, P_ONE] by construction; where a site clamps a probability or a presence, it clamps the one value, never a running sum.

**Exact sums with an order-free refusal (R1.2, R2.3).** A wrapped sum cannot reveal its true value: eight terms of 2^125 wrap to none. Two rules replace "wrapping plus a range check where read":
- **A bound proven before accumulation**, where each term has a static bound: each term is checked pointwise against its bound, and the count of terms against its bound, so the sum's bound holds before any term is added. The lattice deposits use it (2.6): an area below 2^16 m² and fewer than 2^40 leaves keep a node's sum below 2^123.
- **A magnitude sum** otherwise (signed moments, sections, the PA sums): `total::Total` carries the wrapped value and the sum of its terms' magnitudes, saturating at u128's top. Saturating addition of non-negative terms is `min(true sum, top)` in any grouping, and wrapping addition is exact modulo 2^128 in any grouping. The sum is accepted where the magnitudes stay below 2^127, and then the wrapped value is the true one. Otherwise it refuses. The acceptance and the refusal are therefore functions of the multiset of terms alone: independent of cut, order and chunking. Checking each signed partial would not be, since partials depend on grouping.
- `fixed::sum::Sum` (today's saturating f64-term sum) is retired from the feedback path when the reference adopts this contract; it stays as built until then.

**Angles (R1.3, R1.4).**
- **An orientation** (T) wraps. **An amount** (A) never wraps: a rate times a rank is `prod_shr(rate, rank, 1, 1, 32)`, an A, and keeps its whole turns until `wrap` takes its low 32 bits where an orientation is read.
- **The difference of two orientations** is the A in [−1/2, 1/2) turn: `between(from, to) = (to − from) as i32`. A half turn exactly is −1/2.
- **Interpolation** `toward(from, to, share) = from + floor(between(from, to) · share)` (share Q32), wrapped. It always moves the short way.
- **A half angle** halves the A, floored (`half(a) = a >> 1`), before any wrap. A T is never halved.
- **Radians.** `radians(a) = floor(a · TAU60 / 2^60)`, with `TAU60 = round(2π · 2^60)` = `0x6487ed5110b4611a`; an amount at or past 2^60 (2^28 turns) is `Range`. `turns(r) = floor(r · INV_TAU64 / 2^64)`, with `INV_TAU64 = round(2^64 / 2π)` = `0x28be60db9391054a`. Measured: the error is under one Q32 unit plus 1e-15 relative (`radians_are_precise`); one turn gives `floor(2π · 2^32)` exactly, where the old Q32 constant (raw 6) erred by 4.5 %.

**sin and cos of a T (R2.2; host decision, 2026-10-07: a polynomial in place of CORDIC), specified to the bit.**
1. **The nearest eighth:** `n = (t + 2^28) >> 29` (wrapping, three bits), and the rest `r = (t − n 2^29) as i32`, within 1/16 turn.
2. **Radians and their square, in Q62:** `θ = floor(r · TAU60 / 2^30)`, then `z = floor(θ² / 2^62)`.
3. **The polynomials:** `sin = floor(θ · P_S(z) / 2^62)` and `cos = P_C(z)`.
   - Each is evaluated by Horner's rule from its highest coefficient down: `p = 0`, then for k from 5 down to 0, `p = c_k + floor(z p / 2^62)`.
   - The coefficients are `(−1)^k / (2k+1)!` for sin and `(−1)^k / (2k)!` for cos, k from 0 to 5, each the nearest Q62. They are literal in `angle.rs` (`SIN`, `COS`).
4. **An odd eighth** turns the pair by 1/8 turn: `(floor((c − s) H / 2^62), floor((c + s) H / 2^62))`, with `H = round(√½ · 2^62)` = `0x2d413cccfe779921`.
5. **Then `n >> 1` quarter turns, exactly:** q = 1 gives `(−s, c)`, 2 `(−c, −s)`, 3 `(s, −c)`.
6. **Each result** is floored to Q32 (`>> 30`) and clamped to [−1, 1].

**Error bound:** measured at most 1.00005 Q32 units over 200,000 angles, against the 40-step CORDIC's 1.007, so it is the tighter of the two; the test holds 1.001. Most of the bound is the final floor; the truncated series contributes under 3e-14. Every multiple of a quarter turn is exact. **Why:** the CORDIC took 114 ns and about half the layout's time (TASK9-CPU.md). atan2 stays CORDIC in vectoring mode, with the `ATAN` table below.

**atan2(y, x) of two i64, to a T (R2.2), specified to the bit.**
1. **Axes, exact:** `y = 0` gives 0 (a half turn where `x < 0`; `atan2(0, 0) = 0`); `x = 0` gives a quarter (three quarters where `y < 0`).
2. **Input normalisation:** the larger magnitude's top bit is brought to bit 60 by shifting both magnitudes by the same amount, left, or right with `zero_shift`; the signs are restored. With CORDIC's gain of 1.65, every intermediate stays below 2^62.3, inside an i64, for any i64 input (the review's overflow of an unscaled Q62 shift).
3. **Pre-rotation:** where `x < 0`, both are negated and the base is a half turn.
4. **Vectoring:** `z = 0`; for `i` in 0 to 39, where `y ≥ 0` (a tie goes this way), `x += y >> i`, `y −= x >> i`, `z += ATAN[i]`, else the opposite signs, from the old values together.
5. **Output:** `base + ((z + 2^29) >> 30)`, wrapped: to the nearest T, ties up.

`ATAN[i] = round(atan(2^-i) / 2π · 2^62)` for the 40 steps, literal in `angle.rs` and generated by `feedback-constants.py`.

**Error bound:** measured at most 0.502 T units over 200,000 random pairs; the test holds 0.55.

**The inverse functions (R1.6).**
- `asin(x)` of a scalar Q32 `x`, clamped to [−1, 1], is `atan2(x, isqrt(2^64 − x²))`, the square exact in u128. It covers sag's scalar `asin(min(cap / reach, 1))`. `asin(1)` is a quarter turn exactly.
- `acos(c) = atan2(isqrt(2^64 − c²), c)`, `c` clamped to [−1, 1]. `acos(±1)` and `acos(0)` are exact.
- A direction's elevation is `atan2(z, isqrt(x² + y²))`.
- **No tangent is formed** (R1.6): every `tan(a/2)` in the reference is replaced by the pair `(sin(a/2), cos(a/2))` (2.5).

**Rotations built from vectors (host decision, 2026-10-07: trig only where an angle is a stated quantity).** A rotation is a `Turn`: a unit pivot with a Q32 cosine and sine (`fixed::feedback::vector`).
- **Applied by Rodrigues** (`turned`): `v cos + (p × v) sin + p (p · v)(1 − cos)`. Each term is one floor a component from exact intermediates; the last is a 256-bit product shifted by 96. The three add exactly.
- **An angle that is a stated quantity** (a species' angle, a phyllotactic amount, a wander or sag turn, a curvature times a length) takes one `sincos`. Its `Turn` is reused for every vector it turns: running, across and curve alike.
- **A rotation from one direction to another takes no angle** (`carry`):
  - the pivot is `unit(from × to)`;
  - `(cos, sin)` is the unit, normalised in fixed point, of `(from · to, |from × to|)`;
  - parallel or opposed vectors give none.
  
  This replaces `carried`'s `atan2` followed by `sincos` (2.4).
- **Why:** realtime is the goal, and the layout cost 3.0 to 3.7 times today's f64 layout (TASK9-CPU.md). The GPU mirror of the layout was not yet built (task 10), so revising the contract cost least now.

**Roots and unit vectors (R1.5, R2.2).**
- `isqrt(n)` of a u128: `floor(sqrt n)`, specified by its result.
- `length(v)` of a Q32 vector: `isqrt` of the exact sum of squares (Q64, below 2^128 for any i64 components), giving Q32.
- **Normalisation with guard precision:** the largest magnitude's top bit is brought to bit 61 by one common shift (`zero_shift` when shifting right). The squares then sum exactly below 2^126, the root is in [2^61, 2^63), and each component is `trunc(c · 2^32 / root)`. Raw `(1, 1, 0)` is the unit diagonal (0.70710678 · 2^32 each, within 2 units), not `(1, 1, 0)`. `unit` is none exactly for the zero vector (D20).
- **A W vector (a lever)** normalises from W the same way (`unit_w`), never shortened to Q32 first. It returns the unit and `log2` of its real length in Q32: `log2(root) − (shift + 64) · 2^32`. A raw W lever `(1, 0, −1)` keeps its diagonal and its length of 2^-63.5; an arithmetic shift to Q32 would have made it `(0, 0, −1)`.

**Logs and powers of W (R2.2).** The decision contract's `log2` and `exp2_neg` are table-reduced Q62 polynomials, specified to the bit (host decision, 2026-10-07: the exclusion of re-specifying D1–D18 lifted; FINE-CONTRACT.md has the specification). Their bounds are no looser than the squarings' and roots' they replace, and they cost 8 ns where those cost 62 to 69.
- `log2_w(n)` of a W value: −INF for none; else the exponent `e = 127 − clz(n)`, the mantissa `n`'s top 63 bits as Q62 (`n >> (e − 62)`, truncated, or `n << (62 − e)`), then `fixed::log2_frac`, the decision contract's table-reduced polynomial (host decision, 2026-10-07; FINE-CONTRACT.md). The result is `(e − 64) · 2^32 + frac`. Error: within one Q32 unit (as `fixed::log2`).
- `exp2_w(y)` of a Q32 `y`, in W: with `y = k + f` (`k = y >> 32`), the mantissa is `2^62` where `f = 0`, else `exp2_neg(2^32 − f) = 2^(f − 1)` in Q62. It is shifted once, by `k + 2` or `k + 3`: left where the shift is 0 to 63, right (floored) to −63, none beyond. A shift of 64 or more is `Range`, so every result is below 2^126. Small powers keep W's two extra fraction bits and underflow only past W's last place: `2^-63` is raw 2 and `2^-64` raw 1 (the review's two missing bits). Error: under 1e-15 relative plus one W unit.
- `pow(x, y) = exp2(prod_shr(y, log2 x, 1, 1, 32))` in Q32 or `exp2_w` in W; `x = 0` gives 0 where `y > 0` and one where `y = 0` (unchanged).
- `1 − e^(−k)` is `P_ONE − exp2_neg(floor(k · LOG2E / 2^62))` in Q62 (unchanged).
- `ln(a/b) = floor((log2 a − log2 b) · LN2 / 2^62)` (`fixed::ln_ratio`'s rule) for u64 or W operands.

**Keyed draws** in geometry (unchanged): D1's 53-bit draw `d`; a unit `u` is `d >> 21` in Q32, exact; an azimuth is `d >> 21` read as T.

### 2.4 Positions and frames (the sketch, the re-lay's place)

Sites and source order are as before (`grow/draw.rs`, `geometry.rs`, `geometry/lay.rs`). The changes:
- **Angles (R1.3):** `divergence · rank` is an A from `prod_shr`, wrapped only where it sets an azimuth. `slot / whorl` is `ratio(slot, whorl, 32)` as an A. `roll = form.roll · (2u − 1)` is an A. The azimuth is their sum, wrapped once.
- **Epitony (R1.1):** `epitony · sided · turn · (1 + facing) / 2` is one `prod_shr` of four factors with n = 97 (96 for the factors, one for the half), an A. Every factor stays in its own format; nothing is pre-narrowed.
- **The weakened tropism share and `relay_point`'s `t`** are `ratio`s, each clamped pointwise to [0, 1] after the quotient.
- **`toward_elevation` (R1.3; host decision, 2026-10-07):** `now = atan2(z, lean)` of the direction's height and its lean `isqrt(x² + y²)`, already taken for the cone, read as the A `between(0, now)` (in place of `asin(z)`, which took a second root); `to = now + floor((elevation − now) · share)` on As; then `out cos(to) + UP sin(to)`, rounded once a component.
- **Wander (R1.4):** the angle `|curve| · length` is in radians, converted by `turns(r)` (one product with `INV_TAU64`), never by a Q32 constant. `keep = e^(−length / bend_length)` and `isqrt(1 − keep²)` as 2.3.
- **`carried` (host decision, 2026-10-07):** the across and the curve are carried from the old running direction to the bent one by one `carry`, built from the two vectors, with no angle. The wander's and the sag's turns are one `sincos` each, reused for every vector they turn.
- **Sag's room:** `atan2` of the two dots gives a T; its unwrapping (`< −π/2` adds a turn, `> π` folds) is done on As from `between`, so no turn is lost.
- **`support`:** `isqrt` of the Q32 quotient shifted to Q64 (unchanged), the quotient a `ratio`.
- **Positions:** each next point is checked against the 2^12 m domain (2.2) and refuses by name; the trunk's ground refusal (D19) and the clamp to z = 0 for other wood are unchanged.
- **`draw`'s scale** `base_scale · present · nodes[j] · grown` is one `prod_shr` of four factors (R1.1).

### 2.5 Re-lay and sag

**The re-lay is in A (R2.7).** It writes frames and walks back into growth (`grow/relay.rs`), so its arithmetic is bound whether it runs on the CPU or the device:
- **The intended resident executor runs it on the device.** A CPU re-lay over resident fine geometry would need structural transfers and scatter-back, which revisits FINE-CONTRACT decisions 5 and 6 (no per-node readback). It is not proposed.
- **While the reference is proven, the re-lay runs on the CPU** in this arithmetic. That is a proving placement, not a production one.
- **The realised exchanges are kept:** each re-laid root's section, retained pipe, mass and moment go up, its bearer's frame comes down, then its load goes up and its bent frame comes down. So is the dependency: unbent place, then gather, then bent place.
- **Part 1's 14.5 to 20 % re-lay share is apportioned, not timed,** and is not credited to A until the device schedule is measured.

**Girth:**
- `ripe(years, ripening)` is one where `ripening ≤ 0` (R1.6, today's `girth::ripe`), else `min(trunc(years · 2^32 / ripening), 1)`.
- `own = pipe · scale · ripe · leaf`: one `prod_shr` of four factors, to W.
- `mean = Σpipe / Σpipe·light^χ` is a W over W `ratio` to Q32 (R1.1); none where the weighted sum is 0.
- Sections are W sums with magnitude refusal (`Total`). A relay's split is `mul_shr(section, b, 32)` floored and the rest as `section − first`.
- `pow(section, 1/e)` is `exp2(prod_shr(1/e, log2_w(section), 1, 1, 32))`.

**Sag levers (R2.2):**
- `mass = length · (r² + leaves²)` is one exact expression in 256 bits, rounded once to W.
- `moment = middle · mass` is `mul_shr(middle, mass, 32)`, a Q32 position by W, exact in 256 bits and narrowed with a check. Moments sum in a `Total`: the review's "checked bound on the sum of absolute term magnitudes" (R2.3).
- **The PA's sag enters as `log2 sag`, a Q32 converted once from the species' real (task 9).** As a Q32 value, a sag of 1e-8 kept two digits and bent fine wood by up to 0.017 rad off its real. `sag::turn` is held to the f64 form within 1e-4 rad (`sag/tests.rs`).
- **Carrying the lever into the frame now** uses the rotation built from the two frames (2.3): each frame is its heading, its side made square and unit, and their normal. A W vector's components in the first frame are 256-bit sums floored to W, carried by one floor a term. Where either frame has no side (a phytomer never laid), the lever is carried unchanged, as today's f64 pass does.
- The lever `moment − tip · mass` stays in W. It is **never shortened to Q32**: `turn` reads it through `unit_w`, so its direction is exact to Q32 and `log2 reach` comes from W (R2.2). A small moment over `r⁴` is taken in logs, `log2 c = log2 sag − 4 log2 r`, and `k = c · reach · s` as `exp2(log2 c + log2 reach + log2 s)`; it is a small bend only where its logs say so.
- The load operand before `support`'s product is checked against i128 by the narrowing (2.3), not by a separate bound.

**`turn`, without a tangent (R1.6):**
- `start = acos(−z)` of the lever's unit `z`; `low = asin(min(cap / reach, 1))` with `cap / reach` from logs (`exp2` of their difference, clamped to one); `high = half turn − low`.
- `settle(fall, s) = 2 atan2(sin(fall/2) · e^(−k), cos(fall/2))` with `fall/2` the A halved. An upright lever (fall a half turn) has `cos = 0` and stays a half turn; `fall = 0` gives 0; `e^(−k) = 0` with `fall` a half turn gives `atan2(0, 0) = 0`. Today's f64 forms `tan(π/2) ≈ 1.6e16` and leaves this case to rounding.
- The first branch's `ln(tan(fall/2) / tan(high/2))` is `ln` of the exact products `sin_f cos_h` and `cos_f sin_h` (each factor at least none), through `log2_w`. It is INF where the second is none (an upright fall, so the branch never leaves it), and −INF where the first is.
- The cap's rate step: `s = (fall − low) / rate`, the A difference in radians by `radians` (R1.4), divided as a `ratio`.
- **Zero cases as the source:** `reach ≤ 0` (the zero lever, `unit_w` none) or `cap ≤ 0` turns by nothing; `r = 0` gives no sag.

**What returns to growth** is unchanged: each axis's base, heading and side; each phytomer's tip, heading and side; each pencil's `Layer`; the cycle's leaves as tip and area. All are Q32 and none is recomputed in f64.

### 2.6 Light: deposits, sweep, sampling

**Deposits (R2.5):**
- A leaf is its Q32 tip and its area `scale · size · leaf_area`, one `prod_shr`, checked to lie in [0, 2^16 m²).
- The spacing is 0.5 m: a coordinate's node is `x >> 31` (the floor, negatives included) and its fraction `x & (2^31 − 1)` in Q31.
- **Weights:** corner `c` is upper on axis `a` where bit `a` of `c` is set (`light::corners`' order). The seven corners other than (1, 1, 1) are `floor(wx wy wz / 2^31)` in Q62, each factor `t` or `2^31 − t`; the eighth is `2^62` minus the seven. They sum to one exactly, so sampling a field of ones returns one, inside the box or out.
- **Deposits conserve area exactly (R2.5).** The earlier text claimed the weights conserved area, but each `area × weight` floor loses a remainder; Astra's raw area 1 with thirds lost 3 W units. Now the seven corners' deposits are `floor(area · w / 2^27)` (that is `8 area w` in W) and the eighth is `area · 2^35` minus the seven. The eight sum to the leaf's `8 area` in W to the last unit, for every area and fraction (`weights_and_deposits_conserve`).
- **Accumulation** is wrapping u128 addition, exact by the bound proven before accumulation (2.3): each deposit below 2^83, fewer than 2^40 leaves.
- **The box:** each axis from the least node to the greatest node plus one, over every leaf and read; for no points, one node at the origin (size 1 on each axis). A component past the position domain refuses; so does a box of more than 2^28 nodes, its product checked in u64.
- **Retries:** a retry after a buffer overflow rebuilds the cycle's lattice from zero, so no deposit is counted twice (unchanged; proven in 2.13's slice).

**Sweep (R2.4):**
- The CPU runs today's f64 sweep (`light::swept_w`, the same `Field::sweep`) once per cycle on the lattice. Each W density is converted once to the nearest f64 (`d / 2^64`, as `Sum::value` does). The sweep keeps its direction order, lattice extent, boundary rules and one-thread summation order.
- Each node's light is rounded once to Q32, to the nearest with ties to even, clamped to [0, 1]. With no extinction it is one exactly.
- **The guarantee, stated precisely (R2.4):** CPU/GPU placement identity, given the same host sweep and the same supplied constants. Both executors read the integers that one host computation produced. This does not establish bitwise identity across different hosts' floating-point implementations.

**Sampling:** the same corners and weights as the deposit. `light = floor((Σ_inside w · L + (2^62 − Σ_inside w) · 2^32) / 2^62)` in Q32: the corners inside the box by their light, the rest of the weight by the open sky's one. No intermediate is f32 or f64. A bud's light is that value. It is what shade survival, the balance's Q, the allotment's lit presence and the phytomer's light read.

### 2.7 Wood and balance (D16, made integer)

- Upkeep, Q, the totals and their order are as before; the totals are W sums with magnitude refusal.
- `now = trunc((Q − C) · 2^32 / (Q + C))`, or one where `Q + C ≤ 0`; Q and C are non-negative, else `Range`.
- `memory = floor((memory + now) / 2)`, taken in i128.
- `hazard = mul(balance_hazard, max(tolerance − memory, 0))`. No decision where the hazard is none or less: the memory updates, the kept share is the bearer's.
- **The threshold (R2.6):** `p = P_ONE − exp2_neg(floor(hazard · LOG2E / 2^62))` in Q62 and `t = threshold(p) = ceil(p / 2^9)`, as every D1 threshold. The endpoints: `p = 0` gives `t = 0` (never shed); `p = P_ONE` gives `t = 2^53` (always shed).
- **Equality (R2.6):** shed where `d < t`. A draw equal to the threshold is kept.
- **The kept share (R2.6):** where kept, `kept = times(keep, presence::of(over(t, d), window))`. The window is `Windows::window_q((pa, cycle, None), 1.0)`: the existing conversion, `q32(RATE)`, `q32(FLOOR)`, `q32(SPAN)` and `exp` of `q32(log_wood)`, supplied per PA and cycle as an integer. Inherited multiplication is `times`, floored, parent first. Where shed, the bearer's share is carried (as today).
- **A reference change (R2.6):** today decides on an f64 `p` and leads from it. The integer threshold can change later growth even where the immediate shed decision is the same, so the change is measured with the rest (2.11).
- **Shade survival (R1.6):** dormant where φ ≤ 0 or the light is whole: `l` unchanged. A certain survival (`−log2 viability = 0`) stays certain at any light, none included, as today's `grow/advance.rs` keeps it. Otherwise no light gives INF. Any other light gives `mul(l, exp2(mul(φ, 32 · 2^32 − log2 n)))` for a Q32 light `n`, and a factor that saturates at INF gives INF. A factor's saturation never stands in for infinity inside a product: `mul(l, INF)` would be finite for `l` below one.

### 2.8 Allocation and sizes

As before, with each product-and-division a 256-bit numerator and one truncating `ratio`-style division (R1.1), each W sum a `Total`, and every zero case as written (`split ≤ 0`, `above ≤ 0`, `v ≤ 0`, `p ≤ 0`, ψ = 0).

### 2.9 The inductive argument

Unchanged in shape. Two premises it now rests on are explicit:
- Every sum's acceptance or refusal is a function of its terms alone (2.3).
- Every operation's result, a fault included, is a function of its inputs alone (2.3, 2.13).

So identical starting state gives identical phase inputs, integer results, refusals, decisions and next-cycle state, regardless of placement. The probe and the slice (2.13) check the implementations of this argument. They do not prove whole-tree identity (R4); that is the next integration proof (2.12).

### 2.10 Ranges and refusals

| Value | Bound | Where checked |
|---|---|---|
| Position components | below 2^12 m (raw 2^44) | each step's next point; each leaf and read entering the lattice |
| Lattice nodes | at most 2^28 | the box |
| A leaf's area | [0, 2^16 m²) | before its deposit |
| Leaves a cycle | below 2^40 | before the deposits |
| Every narrowed product or quotient | its format | at the narrowing (`Range`, `Zero`) |
| W sums without a static bound | magnitudes below 2^127 | `Total::get`, order-free |
| An amount converted to radians | below 2^28 turns | `radians` |
| A W power | below 2^126 | `exp2_w` |

A value out of range refuses the tree by name. Nothing saturates silently on the feedback path; the only saturations are the decision contract's logs at INF, whose meaning is "unbounded" (unchanged). How far today's trees sit from these bounds is measured when the reference adopts the contract (unknown here).

### 2.11 What changes in today's trees, and how it is shown (R3)

This contract changes the numerical reference for geometry and feedback across species: positions, frames, radii, sag, light, sizes, balance memories and presences. Threshold-adjacent cases can change topology. Normalisation, the radian conversion and small moments change by more than rounding, which Astra noted.

Two comparisons are kept separate:
- **New CPU against new GPU:** exact feedback state and botanical identity after every phase, across cuts, orderings, chunkings and retries.
- **New CPU against today:**
  - the 28-tree dump, reporting topology changes by lineage separately from displacement and radius changes on matching elements;
  - light, allocation, shedding and ground-contact checks;
  - the four species at 80, both seeds, bare and in leaf, canonical ordering and matched views, with close views of fine twigs and contacts, and motion where it applies;
  - parameter neighbourhoods about zero shade response, zero sag, zero ripening, degenerate frames and decision thresholds.

Costs measured: CPU regression, device runtime, transfer bytes, peak memory and shipped artifact size. Hashes are re-pinned only after this evidence establishes the new reference.

### 2.12 Build order

1. **Done (task 8): the primitives and the probe (R4).** The arithmetic on the CPU with its accuracy tests, the WGSL mirror, the conformance probe, and the feedback slice (2.13).
2. **The integration proof (R4):** a small lit specimen spanning a scheduled re-lay, the complete phase state compared, CPU against GPU (task 9; 2.14 settled as option (a)).
3. **The CPU reference adopts 2.4 to 2.8 for every axis,** measured against today as 2.11 says.
4. **The four lit oaks on the device, then the oak timed,** warm and cold, against today's.

### 2.13 The proof built in task 8 (R4)

- **The conformance probe** (`fine/feedback_tests.rs`, `fine/feedback_probe.wgsl`):
  - 16 selectors cover the wide products, the quotient, both signed shifts, radians and amounts, sin, cos and atan2, asin, acos, isqrt and length, `unit`, `unit_w`, the W logs and powers, the order-free sum and its refusal, the weights and deposits, D16, the endpoint cases, and (task 9) the vector-built rotation: `turned`, and `carry` with `dot` and `cross`.
  - Each selector runs 16,384 random cases at and about its domain's edges, with Astra's counterexamples first: four Q32 ones; a W quotient of 2^100 by 2^99; eight terms of 2^125; one turn in radians; raw `(1, 1, 0)`; the W lever `(1, 0, −1)`; area raw 1 with thirds; zero ripening; zero light at viability one; φ zero; the upright lever.
  - **Every result word and error status is equal on the GPU and the CPU:** 229,400 cases, none differing (task 8). Task 9 added 32,768 cases for the rotation, none differing.
- **The feedback slice** (`fixed::feedback::slice`, `fine/feedback_slice.rs`): a synthetic lit crown of 20,000 leaves and 3,000 buds.
  - The device deposits by 128-bit atomic adds with carries, in forward, reversed and shuffled order, in one, 4,096-leaf and 777-leaf dispatches.
  - The 777-leaf run starts in a buffer half the box's size, overflows, and retries from a zeroed lattice.
  - Every lattice read back equals the CPU's word for word. The one CPU sweep of it shades 4,258 of 4,851 nodes. The device's samples and D16 results equal the CPU's for all 3,000 buds, 1,449 of them shed.
  - The CPU's slice is likewise identical across three orders and chunks of 1, 7 and 1,000 (`the_slice_is_the_same_in_any_order_and_chunking`).
- **No tree changed:** nothing in the generator calls `fixed::feedback`. The only edits to existing code are a shared helper (`log2_frac`, a pure extraction in `fixed.rs` and `contract.wgsl`) and `light::swept_w`, a new entry to the existing sweep.

### 2.14 Design questions the review does not settle (for the host)

**Decided (host, 2026-10-07): option (a).** One fixed-point layout for the rough and the final lay, f64 only at the output; the settle's cost measured against today's, reported above 1.3 times. The options as put:

1. **One layout implementation for the rough layout and the final lay.** Astra (R2.1) accepts f64 terminal geometry *and* requires shared stage logic, with no copied rough and final algorithms. Once the rough layout is fixed point, the two cannot both hold with today's code. The options:
   - **(a)** The final lay runs the same fixed-point layout, converting to f64 only at the output. One algorithm, and final geometry becomes integer-identical too. The cost of the settle (21 to 26 % of the stepped build, CPU) in fixed point is unknown, and today's final geometry moves as much as the rough one.
   - **(b)** The layout is written once, generic over its arithmetic (a numeric trait with f64 and fixed-point instances). One source, two instantiations; the settle keeps f64 cost; the generic layer is new structure to maintain and review.
   - **(c)** The rough layout alone becomes fixed point and the final lay keeps today's code. This is the copy Astra rejects, so it is listed only to rule it out.

   The probe and the slice do not depend on the answer; step 2 of 2.12 does.

## Files

- `CONTRACT-A.md`: this file.
- `ceiling-instrumentation.patch`: task 6's patch extended with the sub-phase timers and counts (balance, allotment, deposits, sweep, reads, re-lay passes), reverse-applied after the runs.
- `ceiling-run.sh` and `ceiling-tables.py`: the runner (gated on load) and the tables; the probe is task 6's `hybrid-probe.rs`.
- `raw/task7/` (gitignored): the runs.
- `feedback-constants.py`: the feedback contract's literal constants (2π, 1/2π, CORDIC's table and gain), in exact integer arithmetic (task 8).
