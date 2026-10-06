# fn-209 friction

## 2026-10-05, the light lattice is private to its crate (worker, R0)

- **Doing:** measuring, for the proposal, how much bud light changes when each small subtree's leaves are gathered at its centroid (a stand-in's light coupling).
- **Slowed:** `light::Field` is `pub(crate)`, and an example sees only the public API, so the probe rewrote the lattice (deposit, sweep, read; about 150 lines) and checked the rewrite against `bud_light` (most apart 0.0). The alternative, a temporary test module inside `src/`, meant editing the crate for evidence.
- **Cost:** about 10 minutes and one extra build.
- **Would remove it:** a `light::field(light, leaves, reads) -> impl Fn(Vec3) -> f64` in the public API beside `bud_light`, or an evidence feature that exposes it to examples.

## 2026-10-05, one test binary of the crate's suite runs 404 s (worker, step 1)

- **Doing:** running `cargo test --profile ci -p telperion-space` once after the closed-form refactor.
- **Slowed:** every other binary of the crate finishes in under 9 s; one (11 tests, 1 ignored) took 404 s, most of the suite's wall time. The quick red/green loop for the envelope ran on `--lib envelope` instead.
- **Cost:** about 7 minutes of waiting, once.
- **Would remove it:** naming the slow binary's tests and moving the long walks behind the gate's own run, or splitting them, so a crate-level check stays under a minute.

## 2026-10-06, the competition's contenders share one desk (contender 4, option 4)

- **Doing:** timing chain resolution and the closed form's rows for the competition's performance numbers.
- **Slowed:** the 1-minute load reached 150 on 32 cores while other contenders and a walk test ran; the same beech 40 run went from 0.07 s to 0.39 s one-pass and its rows from 3.3 s to 16 s. Timings taken then are not comparable between contenders.
- **Cost:** one perf run thrown away, about 5 minutes of waiting for a quieter window.
- **Would remove it:** the host running every contender's `perf` itself, one after another, on a quiet machine (or a shared lock like the GPU lock for timing runs).

## 2026-10-06, the closed form's rows cost O(m³) per species and age (contender 4, option 4)

- **Doing:** reading every committed boundary a bud can need (each borne PA at every age 0 to the tree's) up front.
- **Slowed:** each kept table of a bud m cycles old works every threshold t ≤ m, and each m is its own memo entry, so the beech's limb rows alone took 3.3 s at 40 years on one core (the seed's table 0.03 s).
- **Cost:** the test bed's trees could not each read their own rows; they now share one read per species and age, spread over the cores.
- **Would remove it:** a kept table that carries every m at once (one recursion yielding the row at each age), or rows kept between runs on disk.

## 2026-10-06, a still at age 80 holds the GPU lock through the closed form's rows (probe P2)

- **Doing:** rendering the P2 stills (beech, spruce, palm, oak at age 80, seed 1) under the shared GPU lock.
- **Slowed:** the stills runner reads the closed form's rows inside the locked command, and the rows take 87 to 103 s at age 80 on this branch (P1's subject), so the GPU lock is held through a minute and a half of CPU work per alone still, and the other probes wait on it.
- **Cost:** about 3 minutes of lock time held without the GPU, once.
- **Would remove it:** rows read outside the lock (a `--rows` file the runner loads, or the stills runner taking the lock only around `render`), or P1's faster tables.
