# fn-211 task 7: option A's ceiling, then its arithmetic contract (worker, 2026-10-07)

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
  - **Formats:** Q32 in i64 for positions, directions and per-axis reals; W (Q64.64, i128) for powers of radii, masses, moments and tree-wide sums; T (u32 turns) for angles.
  - **Transcendentals:** CORDIC sin, cos and atan2; `isqrt`; the existing `fixed` exp and log, extended to W.
  - **Wrapping sums with a range check** in place of saturation, so every sum is exact in any order.
  - **The sweep stays f64 on the CPU** with its output rounded once to Q32, since no cut moves it (a proposal; a fixed-point sweep is specified if it moves to the device).
  - **Corner weights** that sum to one exactly.
  - **D16, the balance's shedding,** made an integer threshold with a Q32 kept share.
- **No product code changed.** The instrumentation was reverse-applied, and `git diff -- crates/` is empty.

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

## Part 2: the contract

### 2.1 What the contract binds, and what it leaves alone

- **Bound: every computation a cut can move between executors.** A tree must be the same at every cut (FINE-CONTRACT.md, Astra's change 2). Today the cut moves a fine root's balance, allotment, growth, rough layout, deposits, sampling and re-lay between growers. So each of these runs in the contract's arithmetic on every axis, above the cut as well as below it. A coarse axis in f64 and a fine axis in fixed point would make the tree depend on the cut.
- **Not bound: the stages no cut can move.**
  - **The sweep.** It runs once per cycle over the whole lattice, on the CPU, in both executors. Its input is the deposits' integer lattice; its output is rounded once to Q32 light on the CPU (2.6). Both executors therefore read the same integers. This is a proposal: it removes the sweep from the device port. If the sweep later moves to the device (TASK5-GPU.md, section 2, to halve the lattice's traffic), 2.6's fixed-point sweep applies instead.
  - **The settle's final lay.** Nothing it computes returns to growth. Its outputs that decide identity are already integers (D17's fades, FINE-CONTRACT.md) or stay above the cut (D19, host decision 3 of F1). It keeps today's f64 geometry, within fn-125's screen.
- **Unchanged:** the decision contract already built. That covers D1 to D18's integer draws and thresholds, ticks of 2^-20 cycle, Q62 probabilities, Q32 logs and presences, and `fixed::sum`. Everything below extends `crates/telperion-space/src/fixed*` and is mirrored by the device in 32-bit words, as F1's chain grower mirrors those files today.
- **Phases:** Astra's four phase boundaries stay. They are (i) balance, (ii) demand and vigour, (iii) the normalisers, and (iv) growth, waking, sketch, the scheduled re-lay, deposits, sweep and sampling. Same-cycle waking and the re-lay every 10 cycles stay as PHASES.md has them. No phase is fused before equivalence is shown.

### 2.2 Formats

| Name | Width | Meaning | Used for |
|---|---|---|---|
| **Q32** | i64, 32 fraction bits | a real `x` is `n / 2^32` | positions (metres), lengths, unit-vector components, scales, sizes, presences, light, vigour, wood, the balance's Q and C, λ, ψ, logs (as today) |
| **W** | i128, 64 fraction bits (`fixed::sum`'s Q64.64) | `n / 2^64` | pipe sections, leaf-girth terms, masses and moments, lattice deposits, every tree-wide sum |
| **T** | u32 (wrapping), one turn = 2^32 | an angle | every angle: azimuths, divergence × rank, insertion, roll, plane, tropism's elevation, wander, sag's fall |
| **Q62** | u64 | a probability (as today) | survival, abortion, the balance hazard's probability |
| **ticks** | u64 | 2^-20 cycle (as today) | every time |

Why these widths:
- **Q32 for positions:** its resolution is 2.3e-10 m and its range ±2^31 m. A trunk walk of thousands of steps accumulates under a micrometre of rounding, well inside fn-125's 50 µm screen.
- **W wherever Q32 would lose a quantity:** a twig of radius 2e-4 m at exponent 2.5 has a pipe section of 5.7e-10. That is two units of Q32's last place, but 10^10 of W's. Its mass `length · r²` is similar. So everything that is a power of a radius, a mass or a sum over the tree is W.
- **T for angles:** reduction modulo a turn is the wrap of the integer, which is exact and free. A keyed draw's azimuth (`TAU · u`) is the draw's top 32 bits, with no multiplication.

**Species values and per-request constants** are converted once on the CPU and supplied to both executors as integers. That covers internode, tropism, elevation, insertion, divergence, wander, bend length, pipe, exponent, secondary, ripening, sag, leaf area, upkeep, hazard, tolerance, λ, ψ, φ, viability, `relay_at`, the sky's directions and the windows' tables.
- **Reals** go to the nearest integer, half away from zero (`fixed::q32`'s rule, applied at each format's scale).
- **Angles in radians** go to T as `round(a / 2π · 2^32)` mod 2^32.
- **Exponents** `e` and `1/e` go to Q32 separately. `1/e` is the nearest Q32 to the real reciprocal, not a division in Q32.

### 2.3 Operations: one rule each

Every operation below has a single result. Where a result could depend on the algorithm, the algorithm is the specification.

- **Product** `mul(a, b)` (Q32 × Q32): the exact i128 product, shifted right 32 arithmetically. That is floored, as `fixed::mul` is today. A product chain of the reference's source runs left to right, one floor per product, unless the formula below says "one rounding". There it is one i128 product of all factors and one shift or division.
- **Quotient** `div(a, b)`: `(a << 32) / b` in i128, truncated toward zero (Rust's and WGSL's integer division). Each site states its result for `b = 0`; no site divides by zero unguarded.
- **Square root** `isqrt(x)` of a u128: the floor of the real square root. It is specified by its result, so any exact algorithm is the same. The length of a Q32 vector is `isqrt(x² + y² + z²)`, with the squares summed exactly in i128 (Q64), giving Q32.
- **Unit vector** `unit(v)`: none where `x² + y² + z² = 0`, the exact integer test. Otherwise each component is `div(c, length)`. Today's `length > 1e-12` lies below Q32's resolution, so "none" means the zero vector exactly (D20).
- **Dot and cross:** each term an exact i128 product, the terms summed exactly, one arithmetic shift of 32 at the end.
- **Rotation** (Rodrigues, `rotated`): `v cos + (p × v) sin + p (p · v)(1 − cos)`. Each of the three vectors is one rounding per component, and they add exactly.
- **sin and cos of a T angle:**
  - The quadrant is the top two bits. The remainder is centred in [−1/8, 1/8] turn by the octant bit, with sin and cos swapped and negated as the octant requires.
  - Then 34 iterations of CORDIC in rotation mode on i64 with 62 fraction bits. The table is `atan(2^-i)` in T at 2^62 per turn and the gain `K` in Q62. Both are literal constants in the module, generated once and held against f64 by a test, as `fixed.rs` holds `ROOTS`.
  - The result is shifted to Q32, floored.
- **atan2(y, x) to T:**
  - CORDIC in vectoring mode, 34 iterations, on the inputs shifted to 62 fraction bits.
  - Pre-rotation is by the quadrant of (x, y), exact (a negation or swap).
  - `atan2(0, 0) = 0`.
- **The inverse functions:**
  - `asin(z)` of a unit vector's height is defined as `atan2(z, isqrt(x² + y²))`. It is not a series. It agrees with today's `asin` to rounding and needs no clamp.
  - `acos(c) = atan2(isqrt(1 − c²), c)`, with `c` clamped to [−1, 1].
  - `tan(a/2)` is `div(sin, cos)` at half the T angle (a shift).
  - `atan(t)` is `atan2(t, 1)`.
- **exp, ln and pow:** the existing `fixed` procedures (`exp2_neg`, `exp2`, `log2`, `exp`, `ln_ratio`) are used, with two additions:
  - `log2` of a W value (an i128 mantissa through the same 32 squarings);
  - `exp2` to W (`exp2_neg`'s Q62 shifted, or `exp2`'s for exponents above 0).
  - `pow(x, y) = exp2(mul(y, log2 x))`. For `x = 0` it is 0 when `y > 0` and one when `y = 0`, as today's `powf`.
  - `1 − e^(−k)` is taken as `P_ONE − exp2_neg(k log2 e)` in Q62, never as one minus a rounded exponential in Q32.
- **Sums:** every sum is of integer terms. That covers a subtree's light, a pipe section, a lattice node or a moment.
  - The sum is taken with wrapping addition. That is exact modulo 2^64 or 2^128 and so the same in any order, which settles Astra's point that saturating addition is order-dependent on overflow.
  - Each sum's true value is then range-checked where it is read (2.10). Its partials are never multiplied while out of range.
  - `fixed::sum::Sum` changes from saturating to wrapping, with that check. On today's trees this moves nothing, because no sum saturates.
- **Keyed draws** in geometry (wander, roll, dominance) are D1's 53-bit draws. A unit `u` is `d >> 21` in Q32, exact. An azimuth `TAU · u` is `d >> 21` read as T.

### 2.4 Positions and frames (the sketch, the re-lay's place)

Sites: `grow/draw.rs` (`pencil`, `draw`), `geometry.rs` (`frame`, `relay_point`, `turned`, `bend`, `dominance`, `scale`, `reaches`), `geometry/lay.rs` (`Layer::bent`, `step`, `toward_elevation`, `carried`, `framed`, `rotated`, `support`).

**Each axis's state, as the contract holds it:**
- tip, running direction, across, curve: Q32 vectors;
- run, pull, rank, base scale, running presence, wood: Q32;
- inherited lifts: `bend`, `pull`, `total`, `offset`, all Q32.

**A new axis's frame** (`frame`, in the reference's case order):
- **Seed:** the origin, up, +x, exactly.
- **Continuation:** base is the parent's last tip. The side is rotated by `divergence · rank`: the product of a T rate and a Q32 rank, to T, floored.
- **Lateral:**
  - The azimuth is `divergence · rank + slot / whorl + roll`, in T.
    - `slot / whorl` is `(slot << 32) / whorl`, truncated.
    - `roll` is `form.roll · (2u − 1)` in T.
    - The three are added with wrapping.
  - `turned` follows its source order.
    - `level = UP − along (UP · along)`.
    - `facing` is a dot product, and `turn = atan2(along · (toward × upper), facing)`.
    - `sided = min(|level| / UPRIGHT, 1)`, where `UPRIGHT` is a Q32 constant.
    - The epitony rotation is `epitony · sided · turn · (1 + facing) / 2`: one rounding, as an i128 product of the four Q32 factors and the T angle, shifted.
    - Insertion and plane are T.
  - A woken bud's base is `(below + tip) >> 1`, each component floored.
- **Relay:** `relay_point`'s `t = clamp(div(share · total − before, scale), 0, 1)`.
  - `total` and `before` are exact sums of Q32 scales.
  - D18's node is unchanged (already Q32).
  - The base is `start + (tip − start) · t`. `along` and `side` are `unit` of the blended vectors, falling back as today.
  - The blend `b` (Q32) and the final orthogonalisation follow the source order.

**`bend`** is `1 − (1 − straightening)(1 − erected)`, with `erected = 1 − e^(−erection · years)`.
- `years` is ticks, so `erection · years` is one i128 product shifted by 20.
- The exponential is `exp2_neg`'s, in Q62 and then Q32.

**`Layer::step`, in source order:**
1. `length = mul(internode, scale)`. `share = 1 − e^(−tropism · length)`, as 2.3's form.
2. `toward_elevation`:
   - `lean = isqrt(x² + y²)`; the cone is `sin(0.05)` as a constant.
   - The weakened share is `div(mul(share, lean), cone)`.
   - `now = asin(z)` (2.3), and `to = now + (elevation − now) · share`, in T.
   - The result is `out cos(to) + UP sin(to)`, with `out = unit(x, y, 0)`, else `unit(side.x, side.y, 0)`, else unchanged.
3. `carried(v, from, to)`: the pivot is `from × to`, `sin = |pivot|` and the angle is `atan2(sin, from · to)`. With no pivot, `v` is unchanged.
4. **Wander:**
   - `draw = wander · (2u₀ − 1)`; `azimuth = u₁` as T; `pivot = across cos + (running × across) sin`.
   - With a bend length:
     - `keep = e^(−length / bend_length)`, where the quotient is `div` and the exponential is 2.3's.
     - `curve = curve · keep + pivot · (draw · isqrt(1 − keep²))`, and the curve is made square to running.
     - The angle is `|curve| · length`, in radians, converted to T by one product with the Q32 constant `2^32 / 2π`, then rotated about `unit(curve)`.
   - Without a bend length: the angle is `wander · length · (2u₀ − 1)`, converted in the same way, and `curve = pivot · draw`.
5. **Sag** (2.5's lever, on the bent pass only): `framed`, `held` and `torque` follow the source.
   - `room` is `atan2` of the two dots, and its unwrapping (`< −π/2` adds a turn; `> π` folds) is T arithmetic on signed i64 turns.
   - `turn` is 2.5's.
   - `support` is `isqrt(max(z, 0) / zone)`, with the quotient by `div` and the root taken of the Q32 quotient shifted to Q64.
6. **Lifts:**
   - `along = div(offset + run + scale / 2, total)`, or one where `total ≤ 0`.
   - `here = mul(bend, max(1 − along, 0))`.
   - `lift` and `up` are exact sums.
   - `blend = running (1 − lift) + UP up`. The direction is `unit(blend)` where `lift > 0`, else running.
7. **Landing:** for non-trunk wood with `z < 0` and `ease < 1`, the direction becomes `unit(x, y, z · ease)`.
8. **Next point:** `next = tip + direction · length`.
   - The trunk below `−GROUND_TOLERANCE` refuses. The tolerance in Q32 is 4 units (1e-9 m → 4.29, floored). This is D19, above the cut only.
   - Other wood with `z < 0` is clamped to 0, and its direction is re-made as `unit(next − tip)`.
9. **The phytomer's side:** `unit(across − direction (across · direction))`, else across.

**`draw`** (the sketch's unit loop):
- `scale = base_scale · present · nodes[j] · grown` is one rounding. `grown` is the unit's share in ticks, `share << 12` in Q32, exact.
- `rank += nodes[j] · grown`.
- `total = run + Σ scale · size` is an exact sum of products, each floored.
- `wood += mul(internode, scale · size)`.
- `running = present · persist`.

**`scale` and `reaches`** (the re-lay):
- The base scales are products in index order, parents first, one floor each.
- Each phytomer's scale times its base, then its size: two floors, in that order, as the source has two passes.
- Reaches are exact sums. A relay's `(1 − blend) · reach` is floored.

### 2.5 Re-lay and sag

Sites: `grow/relay.rs`, `girth.rs` (`thicken_given`, `leaf_terms`, `own`, `ripe`), `girth/disused.rs`, `sag.rs` (`gather`, `turn`, `held`, `torque`), `shed.rs` (`standing`, `shed`).

**Standing and shedding** are integer already: ticks, Q32 presences and D17's fades. The relay's living set is a set, so order-free.

**Girth** (`thicken`), per axis, from the tip back:
- `ripe(years, ripening) = min(div(years, ripening), 1)`. `years` is whole cycles plus one, exact.
- `own = pipe · scale · ripe · leaf` is one rounding, to W.
- `leaf = light^χ · mean`, where:
  - `light` is the phytomer's Q32 light;
  - `light^χ` is `pow` (2.3);
  - `mean = Σpipe / Σpipe·light^χ` is W over W, a `div` in i128 to Q32. The mean is none where the weighted sum is 0, as today.
- **Children's sections**, in W:
  - each is `pow(radius, e)` (W from Q32);
  - a relay's split is `section · b` and `section · (1 − b)`: two floors, with the second taken as `section − first` so the pair sums exactly;
  - a retained pipe is `share · radius^e`.
- `section += pow(own, e) + at_node[k]` is exact.
- `radius = s · pow(section, 1/e) + (1 − s) · established`, in Q32. `established = pipe · scale · ripe(1)`.
- The pipe a fine root hands its bearer is `pow(section, 1/e)` at its base, in Q32: the same value whichever grower computes it.

**Sag levers** (`gather`), per axis, from the tip back:
- `far` is the farthest tip along the continuations (positions, exact copies).
- `length = |tip − base|` (2.3).
- `leaves = pipe · scale · ripe(years)`, in Q32.
- `mass = length · (r² + leaves²)`, in W, with the squares and their product taken as one i128 expression and rounded once.
- `middle = (base + tip) >> 1`, floored.
- `moment = middle · mass`, in W; per component an i64 × i128 product.
  - The bound: middle below 2^37 in Q32 and mass below 2^76 in W keep it under 2^113.
  - Checked (2.10).
- **Lever** `= moment − tip · mass` (W) and `chord = far − base` (Q32).
- Loads add exactly. A relay splits its load by `b` and by the remainder, as sections do.
- For non-trunk wood, `support(min(z), r, length)` multiplies the running load. That is one floor per component, and the operand is range-checked before the product (2.10).

**`turn`**:
- `reach = |lever|`, where the lever is W shortened to Q32 by an arithmetic shift of 32.
- `c = sag / r⁴` is taken in logs: `log2 c = log2 sag − 4 log2 r`. Every later use of `c` is a product in logs.
  - `k = c · reach · s` is `exp2(log2 c + log2 reach + log2 s)`, saturating.
  - `e^(−k)` is `exp2_neg(k log2 e)`.
- `start = acos(−z / reach)`; `low = asin(min(cap / reach, 1))`; `high = π − low`. All are in T.
- **`settle(fall, s)`** `= 2 atan(tan(fall/2) · e^(−c reach s))`.
- **The first branch's** `s = ln(tan(fall/2) / tan(high/2)) / (c · reach)`. It is `ln_ratio` of the two tangents, shifted to positive integers, times `exp2(−log2(c · reach))`.
- **The rate** is `c · cap`, again in logs. `s = div(fall − low, rate)`, with the T difference converted to radians by the Q32 constant `2π / 2^32`.
- **Zero cases as the source:** `reach ≤ 0` or `cap ≤ 0` turns by nothing; `r = 0` gives no sag (the step's guard `radius > 0`).

**What the bent pass reads:**
- each phytomer's heading and side from the unbent pass (`framed`'s first frame);
- its radius from `thicken`;
- its lever from `gather` on the unbent tree.

The contract keeps that order: unbent place, gather, bent place.

**What returns to growth:**
- each axis's base, heading and side;
- each phytomer's tip, heading and side;
- each pencil's whole `Layer`;
- the cycle's leaves, regathered as tip and `scale · size · area`.

All of these are Q32, and none is recomputed in f64.

### 2.6 Light: deposits, sweep, sampling

Sites: `light.rs` (`Field::spanning`, `deposit`, `sweep`, `swept`, `bilinear`, `at`, `corners`), `grow/stepped.rs` (`shade`).

**Deposits** (bound):
- A leaf is its tip (Q32) and its area `scale · size · leaf_area` (Q32, one rounding).
- The lattice's spacing is 0.5 m, a power of two. So a node index is `tip >> 31`, an arithmetic shift, which is the floor for negative coordinates too. The fraction is `tip & (2^31 − 1)`, in Q31.
- **The eight corner weights:**
  - seven are `w = wx · wy · wz`, each a Q31 factor (`t` or `2^31 − t`), as one exact product shifted to Q62 and floored;
  - the eighth, in `corners`' order the (1, 1, 1) corner, is `2^62` minus the other seven.
  - The weights then sum to one exactly, so deposits conserve leaf area. This is a deliberate change from today, at rounding level.
- **A deposit** is `area · 8 · w` (density over a 1/8 m³ cell), to W by one shift, added with wrapping.
- **The box** is the integer minimum and maximum of every leaf's and read's node, order-free.
- **Retries:** a retry after a buffer overflow rebuilds the cycle's lattice from zero, so no deposit is counted twice (Astra).

**Sweep** (not bound; 2.1):
- It runs on the CPU, today's f64 code, from the W lattice converted once to f64 (`Sum::value`).
- Its light at each node is rounded once to Q32, to the nearest with ties to even, clamped to [0, 1].
- With no extinction there is no sweep, and every node is exactly Q32 one, as today.
- **If the sweep moves to the device** it is bound, as follows.
  - Per direction: `ox`, `oy` and their constant bilinear weights; `k · path / 2`; and the share. All are Q32 constants from the CPU.
  - Per node:
    - `depth = d0 + (k path / 2) · (ρ + ρ0)`, one rounding;
    - `d0` and `ρ0` are bilinear with the direction's constant weights, and are 0 beyond the layer;
    - `light += share · e^(−depth)`, by 2.3's exponential.
  - Directions are summed exactly in any order.

**Sampling** (bound):
- `at(tip)`: the same corner weights as the deposit, in Q62.
- `light = Σ w · L[node]` over the corners inside the box, plus `(2^62 − Σ w inside)` times one (the sky beyond the box), shifted to Q32 and floored.
- No intermediate is in f32 or f64.
- A bud's light is that Q32 value. It is what shade survival, the balance's Q, the allotment's lit presence and the phytomer's light (for girth) all read.

### 2.7 Wood and balance (D16, made integer)

Sites: `grow/balance.rs`, `grow/sketch.rs` (`totals` inputs), `presence.rs` (`Windows::decided`).

- **Upkeep** per axis is `upkeep · wood`, in Q32.
- **Q** per living apex is `base_scale · running · light`, one rounding.
- **Totals:**
  - `light` and `cost` are exact sums child to parent;
  - `living` is an OR.
  - The reverse index order is kept for the reference's reading, but no longer changes a bit.
- `now = div(Q − C, Q + C)`, or one where `Q + C ≤ 0`.
- `memory = (memory + now) >> 1`. That is `MEMORY = 0.5`, exactly, floored.
- `hazard = mul(balance_hazard, max(tolerance − memory, 0))`. Shedding is skipped where the hazard is 0 or less.
- **The probability:** `p = 1 − e^(−hazard)` in Q62 (2.3), with threshold `threshold(p)`.
- **The decision:** shed where `d < threshold(p)`. That is D1's integer test, replacing `d.under(p)` in f64.
- **The kept share:** `presence::of(over(threshold, d), window)`, with the window from `Windows::window_q` at the PA's wood for the cycle. That is the Q32 lead, as decision 8 makes every other presence. Then `kept = times(keep, that)`.
- **Shade survival** (`advance.rs`, D5 under light):
  - The supplied value is `−log2 viability`, Q32 from Q62 per PA.
  - The factor is `light^−φ = exp2(−mul(φ, log2 light))`, with `log2` of a Q32 light. Zero light gives an unbounded factor.
  - `l = mul(−log2 viability, factor)`, saturating at `INF`. Then `survival_q(l, share)`, as built.
  - Zero light therefore gives survival zero, as today's `viability^∞`.

### 2.8 Allocation and sizes

Sites: `allocation.rs` (`split`, `subtotals`, `size_sums`, `sized`, `drawn`), `grow/sketch.rs` (`inputs`, `allot`), `grow/stepped.rs` (`allot`).

- **Inputs:**
  - `presence = base_scale · running · kept`, one rounding;
  - `lit = mul(presence, light)`;
  - `λ` and `ψ` are supplied.
- **`subtotals`:** exact sums child to parent.
- **`split`**, in source order:
  - The node groups are exactly as today, children sorted by (parent, node, index).
  - `lateral` is an exact sum, and `main = max(above − lateral, 0)`.
  - `split = mul(l, main) + mul(1 − l, lateral)`.
  - Each lateral's incoming vigour is `v · (1 − l) · sub / split`: one i128 product of the three Q32 factors, then one truncating division by `split` (Q64 over Q32, then shifted to Q32).
  - The main line carries `v · l · main / split` in the same way.
  - At the tip, the bud's share is `v · own / above` and the continuation's is `v · sub / above`, each one product and one division.
  - Where `split ≤ 0` or `above ≤ 0`, the source's skips apply. Where `v ≤ 0`, nothing is split.
  - The base's vigour is its subtotal, or the given base (a fine root's incoming vigour). That is a Q32 integer either way, so `(v · x) / x` is no longer an issue (PHASES.md's note on `split`).
- **Sizes:**
  - `drawn = (v/p)^ψ = exp2(mul(ψ, log2 v − log2 p))`, in Q32.
  - It is one where `p ≤ 0`, as today. With `v = 0` it is 0 for `ψ > 0` and one for `ψ = 0`.
  - The PA sums `Σp` and `Σp · drawn` are W, wrapping and exact.
  - Each size is `drawn · Σp / Σp·drawn`: one i128 product, one division. It is one where `p ≤ 0` or the weighted sum is 0.
  - The size feeds `draw` (2.4) as the unit's size and the leaf area's factor.

### 2.9 The inductive argument

At the start of a cycle, both executors hold the same integer state for every axis:
- frames, layers, scales, presences, light, wood and memory;
- the same draws, the same living lists and the same sleepers.

Then, phase by phase:
- **(i) and (ii)** read only that state and the supplied constants. Their reductions are exact integer sums and ORs, and their per-axis steps are 2.3's operations. So the balance's sheds and kept shares and every incoming vigour are the same integers.
- **(iii)**'s W sums are exact in any grouping. So every size is the same.
- **(iv)'s growth** reads the same light, size and kept share, and takes D1 to D18's decisions (built). So both executors grow the same axes and phytomers.
- **(iv)'s sketch, re-lay and deposits** run 2.4 to 2.6 on the same inputs, so they give the same positions and the same lattice.
- **The sweep** is one CPU computation of one lattice, and sampling reads it in integers. So next cycle's light is the same.

By induction the botanical tree is the same at every cycle, for any cut, item order, chunking or placement. The settle then runs once on the CPU on the same assembled state.

Tests check the implementations of this argument. They do not replace it:
- probe (a) through the stepped executor at every cut, order and chunking, phase by phase;
- the device against the CPU, phase by phase, on the four lit oaks;
- each primitive against f64 within a stated bound (as `fixed/tests.rs`).

### 2.10 Ranges and refusals

| Value | Bound | Where checked |
|---|---|---|
| Position components | |x| < 2^20 m (far inside Q32's 2^31) | each step's `next`; refuse by name |
| Lattice node count | as today's box | `spanning` |
| W sums (sections, deposits, PA sums) | true value below 2^126 | where read |
| Load operand before `support`'s product, and a moment's i64 × i128 product | below 2^113 | before the product |
| Q32 products feeding a division (`split`, sizes) | the i128 product below 2^126 | before the division |

A value out of range refuses the tree by name. Nothing saturates silently on the feedback path: a saturated value is order-dependent (Astra). How far today's trees sit from these bounds is not measured here (unknown). `fixed::sum`'s note puts the largest lattice term below 2^20. The margins are measured when step 2 is built.

### 2.11 What the contract does not establish

- **That the movement from today's tree is negligible.** Fixed point guarantees that executors agree, not that they agree with today. That is measured separately, under AGENTS.md "Generator evolution":
  - the 28-tree dump against the base;
  - the four species' stills at 80, bare and in leaf, viewed.
- **The device's cost.** TASK5-GPU.md's synchronisation floor (113 to 282 ms at 80) and Part 1's ceiling bound it from both sides. Neither is a measurement of the executor.
- **The converter's merges (D22).** These are F4's, by host decision 2 of F1.

### 2.12 Build order, if the host takes A

1. **The primitives on the CPU,** each with its accuracy test against f64: CORDIC sin, cos and atan2; `isqrt`; W's `log2` and `exp2`; wrapping `Sum`.
2. **The CPU reference adopts 2.4 to 2.8 for every axis,** above and below the cut. Then:
   - the movement is measured on the 28 trees and the stills;
   - probe (a) is re-run;
   - `measures hash` is re-pinned once.
3. **The device mirror,** phase by phase, each held against the CPU on the four lit oaks.
4. **The oak timed,** warm and cold, against today's.

Steps 1 and 2 are CPU-only and useful alone: they make the stepped executor's identity independent of f64 regrouping everywhere, not only for the three sums. Step 3 is TASK5-GPU.md's three to four sessions plus two to three; this contract does not change that estimate.

## Files

- `CONTRACT-A.md`: this file.
- `ceiling-instrumentation.patch`: task 6's patch extended with the sub-phase timers and counts (balance, allotment, deposits, sweep, reads, re-lay passes), reverse-applied after the runs.
- `ceiling-run.sh` and `ceiling-tables.py`: the runner (gated on load) and the tables; the probe is task 6's `hybrid-probe.rs`.
- `raw/task7/` (gitignored): the runs.
