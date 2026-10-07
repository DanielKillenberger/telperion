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
