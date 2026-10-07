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
