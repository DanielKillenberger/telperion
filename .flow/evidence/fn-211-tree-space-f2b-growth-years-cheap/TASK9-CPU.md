# fn-211 task 9 (9a): the CPU reference on contract A, slice by slice (worker, 2026-10-07)

Feedback-first order (host, 2026-10-07). Each pass is one commit. Each is compared with the base (`56b02531`) on the 28-tree dump (`contract dump`; `raw/task9/*-vs-base.txt`, gitignored).

| Pass | Commit | Crate tests | What moves (28 trees, against the base) |
|---|---|---|---|
| Light lattice and sampling | `dd4f4bef` | One tolerance: the light slab law, 1e-9 relative to 1e-8 absolute. | The lit oak only: topology unchanged, tips at most 1.5e-6 m, radii at most 5.2e-4 relative. |
| D16 | `4f6c2888` | Unchanged. | The lit oak only: as above, tips at most 1.8e-6 m. |
| Allocation | `02979d5a` | Four allocation laws held to Q32 (1e-12 to 1e-8). The coupling pass shares the size rule. | The lit oak only: topology unchanged, tips at most 2.1e-6 m, radii at most 1.4e-3 relative. |
| One fixed-point layout (option (a)) | `7bd941e2` | Four tolerances move from 1e-12 or 1e-9 to 1e-8: `tests/form.rs` (two), `tests/geometry.rs` and the closed form's extent bound. | Every species moves; see below. |

## The layout pass: what it moves

- **Topology:**
  - The beech, spruce and palm are unchanged; no geometry feeds their growth.
  - The lit oak changes by 0.3 to 0.4 % of its phytomers at 80 (seed 1: 3,062 gone and 3,097 new of 1.10 M; seed 7: 9,409 gone and 9,651 new of 2.59 M), and by a few phytomers at 40.
- **Displacement on matching phytomers:**
  - At 40, at most 1 to 3 cm (RMS 1 to 3 mm).
  - At 80, at most 0.13 to 1.55 m (RMS 1.6 to 5.5 cm).
  - The palm moves at most 9e-5 m.
- **Radii:**
  - Beech and spruce change by at most 2.9e-3 relative, except one tree (spruce 80, seed 7, whole), where one radius changes wholly (1.0 relative).
  - The lit oak's matching lineages change radius by up to 5e4 relative, because their subtrees differ.
- **Not yet explained:** displacements of centimetres to a metre are above what Q32 rounding alone gives on a walk. The candidates are not yet separated:
  - sag, still the f64 lever, fed the moved geometry;
  - the landing's square root near the ground;
  - the backstop's piecewise room.

  Sag is the next pass. The stills are viewed after it.

## The settle's cost: over the host's 1.3x report rule

From `examples/passcost.rs`: the warm best of three, age 80, seed 1, at a one-minute load of 10 to 11 in both runs. "Layout" is the sketch, the re-lay's place and sag, and the final place and sag.

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
