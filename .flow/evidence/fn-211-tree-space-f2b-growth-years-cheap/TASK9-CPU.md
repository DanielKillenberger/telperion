# fn-211 task 9 (9a): the CPU reference on contract A, slice by slice (worker, 2026-10-07)

Feedback-first order (host, 2026-10-07). Each pass is one commit. Each is compared with the base (`56b02531`) on the 28-tree dump (`contract dump`; `raw/task9/*-vs-base.txt`, gitignored).

| Pass | Commit | Crate tests | What moves (28 trees, against the base) |
|---|---|---|---|
| Light lattice and sampling | `dd4f4bef` | One tolerance: the light slab law, 1e-9 relative to 1e-8 absolute. | The lit oak only: topology unchanged, tips at most 1.5e-6 m, radii at most 5.2e-4 relative. |
| D16 | `4f6c2888` | Unchanged. | The lit oak only: as above, tips at most 1.8e-6 m. |
| Allocation | `02979d5a` | Four allocation laws held to Q32 (1e-12 to 1e-8). The coupling pass shares the size rule. | The lit oak only: topology unchanged, tips at most 2.1e-6 m, radii at most 1.4e-3 relative. |
| One fixed-point layout (option (a)) | `7bd941e2`, fixed in `0c70e64b` | Four tolerances move from 1e-12 or 1e-9 to 1e-8: `tests/form.rs` (two), `tests/geometry.rs` and the closed form's extent bound. | Every species moves; see below. |

## The layout pass: what it moves

At `7bd941e2` the trees moved by centimetres to 1.5 m, and the lit oak changed topology. The cause was a bug: Rodrigues' third term was shifted by 128 where 96 is right. A test now holds the layout's primitives (`rotated`, `unit`, `dot`, `cross`, `turned`, `toward_elevation`) to their f64 forms within 1e-8, and it caught the bug. After the fix (`0c70e64b`):

- **Topology:** unchanged in all 28 trees. That includes the lit oak at 40 and 80.
- **Displacement on matching phytomers:**
  - At 40: at most 0.9 to 18 µm (RMS at most 4.5 µm).
  - At 80: at most 6.7 µm to 0.73 mm (RMS at most 6.8 µm). The largest is spruce 80, seed 7, alone.
  - The palm: at most 81 µm, RMS 31 to 60 µm.
- **Radii:**
  - Beech and spruce change by at most 2.9e-3 relative. The exception is one radius in spruce 80, seed 7, whole, which changes wholly (1.0 relative). Not yet looked into: a pipe that a ground-contact or disuse decision now sets or keeps.
  - The lit oak changes by at most 0.25 relative (oak 80, seed 1).
- **The crate's tests and telperion-render's** are green after the fix. That includes F1's GPU settle against the CPU (`every_root_settles_on_the_gpu_as_on_the_cpu`), which failed at `7bd941e2`.

## The settle's cost: over the host's 1.3x report rule

From `examples/passcost.rs`: the warm best of three, age 80, seed 1, at a one-minute load of 10 to 11 in both runs, measured at `7bd941e2`. The fix changes one shift constant, not the cost. A re-run after the fix at load 16 to 22 is inflated throughout (`raw/task9/passcost-new-fixed-load22.txt`). "Layout" is the sketch, the re-lay's place and sag, and the final place and sag.

| Tree | Base build ms | New build ms | Base layout ms | New layout ms | Layout ratio |
|---|--:|--:|--:|--:|--:|
| beech | 630 | 724 | 80 | 240 | 3.0 |
| spruce | 3,219 | 4,772 | 650 | 2,387 | 3.7 |
| oak | 2,161 | 2,497 | 145 | 468 | 3.2 |
| oak lit | 8,225 | 8,783 | 1,014 | 1,998 | 2.0 (the final place and sag alone: 104 to 335, 3.2) |

Micro-costs on this desk:
- CORDIC `sincos`: 114 ns.
- `atan2`: 70 ns.
- `unit`: 40 ns.
- `isqrt`: 18 ns.
- A 256-bit product, shift and narrow: 4 ns.

A layout step takes about six sin/cos and three atan2.

## After options 1 and 2 (host, 2026-10-07): still over 1.3x

Commit `053ba4ec` (vector-built rotations; one sin and cos per stated angle). The 28 trees move as at `0c70e64b`, to the micrometre (`raw/task9/turn-vs-base.txt`). The base and the new layout were run back to back at load 9 to 10 (`raw/task9/passcost-base-2.txt`, `passcost-turn.txt`).

| Tree 80 s1 | Base build ms | New build ms | Base layout ms | New layout ms | Layout ratio |
|---|--:|--:|--:|--:|--:|
| beech | 586 | 802 | 71 | 220 | 3.1 |
| spruce | 3,389 | 4,659 | 729 | 2,004 | 2.75 |
| oak | 2,153 | 2,738 | 147 | 446 | 3.0 |
| oak lit | 8,217 | 9,173 | 1,044 | 1,860 | 1.8 (final place and sag alone: 108 to 276, 2.6) |

### The profile

`perf` and `samply` cannot run on this desk (`perf_event_paranoid`). Instead, calls were counted once per spruce 80 build, with counters added for one run and then removed, and multiplied by the micro-costs above. These are CPU seconds summed over threads, an estimate:

| Primitive | Calls a build | ns each | CPU s | Its sources in a step |
|---|--:|--:|--:|---|
| `sincos` (CORDIC, 40 steps) | 38.5 M | 114 | 4.4 | the tropism's elevation, the wander's azimuth, the wander's turn, sag's turn: about 4 a step |
| `unit` (includes one `isqrt`) | 52.2 M | 40 | 2.1 | elevation's level, carry (twice), the lifts' blend, the side: about 5 a step |
| other `isqrt` | 30.6 M | 18 | 0.6 | `length`, `asin` |
| Rodrigues (`turned`) | 47.6 M | about 25 | 1.2 | carry (2), wander (2), sag (3) |
| `atan2` | 11.4 M | 70 | 0.8 | `asin` in the elevation |
| `exp2_neg` | 15.2 M | about 40 | 0.6 | the tropism's share, the bend length's keep |

The sin and cos are about half the identified cost, and the unit vectors about a quarter. Neither is redundant any more: each angle left is a stated quantity.

## Steps 1 and 2 of the host's cost decision, then the re-lay, girth and sag pass

Run back to back, each at a one-minute load under 10 (`raw/task9/passcost-steps.txt`, `passcost-sag.txt`). Layout ms at 80, seed 1; the base is `02979d5a`.

| Commit | What | beech | spruce | oak | oak lit |
|---|---|--:|--:|--:|--:|
| `02979d5a` | base (f64 layout) | 71 | 660 to 679 | 145 to 150 | 911 to 918 |
| `053ba4ec` | vector-built rotations | 193 | 1,860 | 385 | 1,745 |
| `550e72a4` | step 1, first form (slower: u128 conversions, libm round) | 230 | 2,165 | 468 | 1,944 |
| `a64b3045` | step 2, polynomial sin and cos | 189 | 1,805 | 429 | 1,789 |
| `02cd51bc` | step 1 fixed (`cb8466bd`), then girth and sag on contract A | 227 | 2,135 | 437 | 1,995 |

- **Ratio after steps 1 and 2:** 2.6 to 3.2 times, against the 1.3x rule. The sag pass adds W levers, logs and the vector-built frames, and its cost is about the polynomial's saving.
- **Micro-costs now:**
  - `sincos` 16 ns (was 114 ns);
  - `atan2` 67 ns;
  - `unit` 34 ns;
  - `isqrt` 19 ns;
  - `unit_w` 97 ns;
  - `turned` 24 ns;
  - `fixed::log2` 67 ns, `exp2_neg` 64 ns and `exp2` 65 ns (the decision contract's squarings and roots).

### The profile now (spruce 80, seed 1, one build)

Calls counted with counters added for one run and then removed, multiplied by the micro-costs. CPU seconds summed over threads:

| Primitive | Calls | CPU s | Where |
|---|--:|--:|---|
| `fixed::log2` | 51.1 M | 3.4 | girth (each section and radius), sag's turn (logs of radius, lengths and settled spans), `unit_w`'s length |
| `exp2_neg` (with `exp2`'s) | 33.8 M | 2.2 | the tropism's share, the bend length's keep, girth's powers, sag's settle |
| `unit` | 54.9 M | 1.9 | the elevation's level, carry (2), the lifts' blend, the side |
| `atan2` | 21.0 M | 1.4 | the elevation's `asin`, sag's room |
| `turned` | 47.6 M | 1.1 | carry (2), wander (2), sag (3) |
| `exp2` | 14.7 M | 1.0 | girth's radius, sag's logs |
| other `isqrt` | 40 M | 0.8 | `length`, `asin`, `unit_w` |
| `unit_w` | 7.6 M | 0.7 | sag's levers and caps |
| `sincos` | 40.7 M | 0.7 | elevation, wander, sag |

The decision contract's log and power procedures (32 squarings, 32 roots) are now the largest cost, ahead of the tropism.

## The re-lay, girth and sag pass (`f4dd8b0e`, fixes `de934601` and `02cd51bc`)

Against the base (`raw/task9/sag-vs-base.txt`):
- **Topology:** unchanged in 27 of 28 trees. The lit oak at 80, seed 7, changes 44 of 342,705 axes (113 of 2.59 M phytomers).
- **Tips on matching phytomers:** at most 1 to 30 µm, except the lit oak at 80 seed 7 (2 cm, next to its changed axes) and the spruce at 80 seed 7 alone (0.72 mm).
- **Radii:**
  - **The spruce 80 seed 7 radius that "flips" (1.0 relative):** its cause is the radius of a phytomer of vanishing presence, 1.8e-11 m, below Q32's last place (2.3e-10 m), which floors to 0. Every 1.0 in the table is such a radius (oak 40 seed 7, 2.7e-10 m; oak 80 seed 1, 1.5e-10 m; spruce 40 seed 7, 2.0e-11 m). The comparison now prints the worst radius before and after.
  - Elsewhere radii move by at most 1e-2 relative, on radii below 1e-7 m.
- **Two bugs found and fixed:**
  - sag as a Q32 value lost its digits (`de934601`);
  - a frame with no side skipped sag where today carries the lever unchanged. The alone spruce had moved metres (`02cd51bc`).

## The host's second cost decision: steps 1 and 2 reach nothing, so the work stops (2026-10-07)

**Step 1, fewer logs in girth and sag: almost nothing to cut, and the one cut tried broke an identity.**
- **Where spruce 80's 51 M `log2` calls come from** (by count, 4.8 M phytomers):
  - Girth runs twice: the tree as grown (`attachments`) and the tree as kept (`thicken`). Each run takes two logs a phytomer, one for its own pipe's power and one for its radius: about 19 M.
  - The children's sections: about 2 M, one per axis.
  - Sag's bent pass: about five a phytomer (log r, the lever's and the cap's lengths, the settled span, the rest): about 24 M.
- **Nothing is computed twice within a pass.** Each log is of a value made just before it. The radius's log is taken in the bent pass, after the girth pass has floored the radius to Q32.
- **The cut tried:** handing each child's base-radius log to its bearer in place of the radius (`raw/task9/step1-girth-carry.patch`). It saves one log and one power per axis, about 2 % of the calls. It also broke the fine executor's identity at every cut (`fine::tests::realized`), because a fine root hands its bearer a radius and the bearer would read a log. Carrying the log through the executor's records would fix that, for a 2 % saving. Reverted.
- **One real saving, not built:** both girth runs compute the same own pipe, and so the same `section(own, e)`, for every phytomer kept. Caching it between the two runs would save about 10 M of the logs and powers (about 20 %). It needs a phytomer map across the shed. Larger work; not in the decision's scope.

**Step 2, faster log2, exp2 and exp2_neg with identical bits: not reached; stopped, as the decision says.**
- **Branchless forms of both procedures** (`raw/task9/branchless-logs.patch`) were proven identical:
  - `exp2_neg` exhaustively, all 2^32 fractions (its whole part only shifts);
  - `log2_frac` on 10^8 random mantissas, every power of two and its neighbours, and the ends.
  
  They were no faster: `log2` 69 ns, `exp2_neg` 62 ns, `exp2` 61 ns, against 67, 64 and 65 ns. Reverted.
- **Why the bits cannot come faster:**
  - Each procedure is a chain of 32 dependent 64-by-64-bit products, each truncated before the next. The time is the chain's latency, not its branches.
  - `log2`'s squarings depend on all 63 bits of the mantissa from the first step, so no table can stand for any prefix of the chain.
  - `exp2_neg` can halve its chain with identical bits: a 2^16-entry table (512 KB) of the chain over the fraction's top 16 bits, then 16 products. That is about twice as fast on 2.2 of the 13 CPU seconds, about 8 % of the layout.
  - Anything faster changes the truncation order, which is a re-specification of D1 to D18. The host excluded that.
- **Step 3 (the tropism rework) is not started:** the decision stops at step 2.

## The host's third cost decision (2026-10-07): the polynomial logs, the girth cache, the tropism

- **Step 1, log2 and exp2_neg as table-reduced Q62 polynomials** (`afa6eec2`). Specified to the bit in FINE-CONTRACT.md and CONTRACT-A.md and mirrored in `contract.wgsl`. The contract probe, over every D1–D18 user, agrees word for word, with the tables' edges added; so does the feedback probe. The bounds are no looser than the procedures': log2 within 1.000 Q32 units, as before; 2^-f within 2.22e-16 relative, as before. They cost 8 ns where the procedures took 62 to 69.
- **Step 2, caching the own-pipe sections between the two girth runs: skipped.** After step 1 a section costs about 20 ns (a log, a power). A lineage-keyed lookup over 4.8 M phytomers costs as much or more, with its cache misses and about 100 MB. The saving would be at most about 0.1 CPU s on spruce 80.
- **Step 3, the tropism's elevation as `atan2(z, lean)`** (`eb3f1416`). It saves the second root, 9.5 M `isqrt` calls on spruce 80. The layout has no device mirror yet (task 10); atan2 is already probed.

### Measured (`raw/task9/passcost-logs.txt`; back to back at load under 10; 80, seed 1)

| Tree | Base build ms | Now build ms | Whole build | Base layout ms | Now layout ms | Layout ratio |
|---|--:|--:|--:|--:|--:|--:|
| beech | 538 | 581 | +8 % | 68 | 154 | 2.3 |
| spruce | 2,983 | 3,709 | +24 % | 613 | 1,637 | 2.7 |
| oak | 1,968 | 2,180 | +11 % | 144 | 310 | 2.2 |
| oak lit | 7,363 | 7,577 | +3 % | 886 | 1,502 | 1.7 |

### Decision flips against `02cd51bc` (`raw/task9/logs-vs-sag.txt`)

- **Topology** changes in one of 28 trees: the lit oak at 80, seed 7, with 3 phytomers new of 2.59 M.
- **Tips** move at most 0.67 mm (spruce 80, seed 7, alone). Every other tree moves at most 0.36 mm (lit oak 80, seed 7), most of them under 0.1 mm.

### The profile now (spruce 80, seed 1; calls counted for one run, times the micro-costs)

| Primitive | Calls | ns | CPU s |
|---|--:|--:|--:|
| `unit` | 54.9 M | 34 | 1.9 |
| `atan2` (CORDIC, 40 steps) | 21.0 M | 67 | 1.4 |
| `turned` | 47.6 M | 24 | 1.1 |
| `sincos` | 40.7 M | 16 | 0.7 |
| other `isqrt` | 30.8 M | 19 | 0.6 |
| `unit_w` | 7.6 M | 63 | 0.5 |
| `log2` | 56.6 M | 8 | 0.5 |
| `exp2_neg` (with `exp2`'s) | 33.8 M | 8 | 0.3 |

The decision contract's logs are no longer the cost. What remains is the vector work (unit vectors and rotations) and the CORDIC `atan2`: one in the elevation and one in sag's room each step.
