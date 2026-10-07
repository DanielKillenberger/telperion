# fn-212 task 3, first step: D1 against the ring path (worker, 2026-10-07)

D1 has the conversion keep every kept phytomer. A trial of it collides with the GPU ring path's float32 qualification. That is a design question for the host, so the task stops here: nothing is committed but this note.

## The trial

- **The change, uncommitted and reverted:** `pipeline/space.rs` at `884a0bb7` with `FINEST`'s cut and the `SHORTEST`, behind and inside merges switched off. Every phytomer of every axis becomes a node; the relay's joint, the aside and codominance are left as they are.
- **The probe:** an example on the CPU expansion at 40, seed 1, asking what the resident path asks.

| Tree | Nodes | Of them under 0.2 mm | Curve | Compact rings qualified | Unqualified rings | Canonical float32 wood |
|---|--:|--:|---|---|--:|---|
| beech 40 | 113,169 | 9,215 | builds, frames finite | **no** | 1,320 | **refused** (`None`) |
| spruce 40 | 1,183,167 | 34,663 | builds, frames finite | **no** | 7,234 | **refused** |
| oak 40 | 190,728 | 29,666 | builds, frames finite | **no** | 12,724 | **refused** |
| palm 40 | 993 | 0 | builds | no (shaped, as today) | 0 | admitted |

## What it means

- **The curve takes every phytomer.** It builds with finite frames, so F3's drawing of thin wood as ribbons has its input.
- **The ring path refuses every engine tree, because its rings are too thin.**
  - `qualified_ring` (`surface/compact.rs`) requires a ring's radius to be at least the largest |centre| over 131,072, about 0.08 mm at 10 m.
  - The canonical float32 wood's triangle admission also refuses these rings. It returns `None`, so the resident path would fall back.
- **What the spruce would do under D1:** it seats its leaves, so it would leave the GPU path and fall back to the CPU reference mesh for the whole tree. Task 2 bought the spruce at 80 its 6 s; this fallback would undo that.
- **What the beech and oak would not lose:** they seat nothing, so since task 2 they build no rings, and only their curve and stations change.
- **The qualification guards a ring mesh nobody draws** (AUDIT.md §2b). The rings' one reader is the seated leaves' contact. The question is what a contact needs, not what a triangle mesh needs.

## For the host

- **D1 together with the seated spruce needs one of:**
  - (a) a ring qualification for contacts alone: the contact's precision bound in float32, not the mesh's triangle admission. Rings under it are still emitted, or are clamped where a leaf seats.
  - (b) contact rings only on the runs that bear stations, with thin wood above some bound left to the curve.
  - (c) the conversion's thin-wood cut kept for the seated path only. This contradicts D1's "one identity per kept phytomer".
- Each option changes what a seated leaf stands on, so the choice is the host's.
- The probe (`f212_keep.rs`) and the trial patch were not kept. The table above holds their result.

## Host decision D1b (2026-10-07)

- **Option (b):** contact rings are built only on runs that bear leaves.
- Every other run, thin wood included, is surfaced as F3's curves on both executors, the CPU reference included. No ring check then meets thin unseated wood.
- If (b) still trips the float32 ring check on leaf-bearing runs, that is reported before (a) is taken.

## (b) measured: it still trips the ring check (2026-10-07, after merging fn-198's tip)

D1 was built in the conversion: `d1-conversion.patch`. `FINEST`, `SHORTEST` and the relay's behind and inside merges are gone, and every axis is whole. The probe was `task3-d1-probe.rs`, at seed 1. The patch is reverted and nothing of it is committed, because (b) does not clear the check.

| Tree | Nodes | Under 0.2 mm | Internodes under 1 mm | Runs | Leaf-bearing runs | Bearing runs with an unqualified ring | Unqualified rings on them | Stations on them |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| spruce 40 | 1,183,167 | 34,663 | 7,314 | 222,525 | 222,525 | 2,531 | 7,234 | 12,424 of 4,662,320 |
| spruce 80 | 4,780,268 | 137,987 | 57,872 | 700,504 | 700,504 | 20,637 | 77,393 | 120,013 of 17,069,121 |

- **Every run of the spruce bears leaves.** Its needles stand on the thinnest wood, so (b) removes no run from the contact rings.
- **The rings that fail the check are on runs that bear leaves.** At 80 that is 77,393 rings on 20,637 runs, under 0.7 % of the stations. The spruce would leave the GPU path whole.
  - The station probe counts a run as leaf-bearing when a station's contact names the run's first ring.
- **The CPU reference fails too.** It does not fall back; it refuses, and it is not only the seated spruce:
  - `mesh::build` errs on the spruce at 40 (`surface triangles collapsed in float32 = 3435`) and on the unseated oak (`= 296`). That is the swept wood mesh meeting thin and short wood.
  - The beech and palm at 40 build.
  - So D1b's "the CPU reference included" needs the CPU reference's drawn wood to come from the curve, not from the swept mesh. Its consumers (the Wasm binding's buffers, the tests, the headless tools) read that mesh today. That is a design step of its own.
- **The curve takes every tree:** frames finite, 5.48M points at the spruce at 80. Stations prepare (17.1M at 80). The GPU executor's unseated path (beech, oak) builds no rings since task 2, so it draws them.

**For the host:**
- (a) is now the only option of the three for the seated spruce: a contact qualification by the contact's own float32 precision.
- The CPU reference's thin wood needs a decision too: the curve's tessellation as the reference's wood, or the swept mesh kept with its ring check relaxed.
