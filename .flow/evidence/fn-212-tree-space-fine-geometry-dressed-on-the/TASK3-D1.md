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

## Host decision D1' (2026-10-07): D1 withdrawn

- **The reason** is the measurement above. Keeping every phytomer breaks the float32 ring check on the spruce's leaf-bearing runs, and the CPU reference mesh on the spruce and oak.
- **What replaces D1:** the conversion keeps its thinning rules: `FINEST`, `SHORTEST`, the relay merges, the whole-axis rule, `STRUCTURAL` and codominance. Every quantity those rules read is computed exactly under the integer contract: each phytomer's length and radius from realized totals in fixed point, and every comparison in integers. Both executors then make identical cuts.
- Thin wood stays pruned as today. The ring checks and the CPU reference mesh are unchanged.
- **The drawn output** of the four species at 40 and 80 must be shown unchanged, or changed only where a decision next to its threshold moved: counts by lineage, and stills if any moved.
- **This revises fn-209 host decision 2** (FINE-CONTRACT.md: identities botanical, merges at draw time). Identities are the conversion's again, made by integer decisions.

## D1' measured: how near its thresholds each conversion decision falls

A temporary instrumentation of `pipeline/space.rs`'s conversion (reverted; `raw/task3/margins.patch`) logged every decision's relative distance from its threshold, for the four species at 40 and 80, seeds 1 and 7. The full table is in `task3-margins.txt`. The executors disagree on a radius by up to 6.4e-6 relative (REALIZED.md), so 1e-5 is the band in which a cut can differ today.

| Rule | Reads | Decisions under 1e-5 of the threshold, per tree |
|---|---|---|
| `FINEST` | radius (pipe model, f64 on the CPU, f32 on the device) | 0 to 3 |
| `SHORTEST` | laid internode length (after sag) | 0 to 2 |
| `STRUCTURAL` | radius against the root's | 0 to 2 |
| codominance | radius ratio | 0 to 2, except the beech (below) |
| relay behind / inside | the laid tip against the stem's laid span and radius | 0 |
| joint | the laid tips' distances to the relay's base | 0 |
| aside inside / fork | laid positions and radii | 0 |

- **About one decision in a million lies in the band.** The spruce at 80 decides 4.7M `FINEST` cuts, and 3 of them lie within 1e-5.
- **D1' therefore moves very little of today's drawn output:** a few nodes a tree where a cut flips.
- **The beech's codominance zeros** (18 to 303 decisions) are exact ties of the code's own making, not rounding. `aside` scales a head to exactly `FORK_FROM` of its carrier, and `codominance` then reads 0.4.

## For the host: D1' needs its integer forms decided

D1' asks for each quantity from realized totals in fixed point. Three of the quantities have no such form today, and the forms below change the engine's contract on both executors, so I stopped here:

1. **Radius.**
   - Today: `girth.rs` sums each phytomer's sections `share · r^e` and takes `s · Σ^(1/e) + (1 − s) · established`, with `powf_fixed` in f64; the device does the same in f32 (`settle.wgsl`).
   - Proposed integer form: sections in a fixed-point format, summed as integers, the power through the contract's `log2` and `exp2_neg`, so the radius is the same integer on both executors.
   - Open: the format's range (radii from 1e-5 to 1 m), the exponent's representation, and whether `established` follows. This also changes every radius's bits, so every tree's bytes.
2. **Internode length.**
   - Today: `SHORTEST` reads the laid tip's distance from its parent, after sag.
   - Proposed integer form: the phytomer's grown length (internode × scale × size), Q32 from the presences already integer, independent of sag. This changes what `SHORTEST` measures from the laid distance to the grown one.
3. **The relay and head rules: behind, inside, joint, aside inside.**
   - They read laid, bent geometry: the tips, the stem's span and the base's nearest node, in the tree's frame after phase 4.
   - "From realized totals" has no candidate for these, because they are positions. Options:
     - (i) decide them on the unbent first lay, carried in fixed point;
     - (ii) keep them on the CPU and hand the device each relay's merged count as a record (relays in fine roots exist: `fine/realized.rs:185`);
     - (iii) restate them on integer quantities: the relay's node index and its share of its parent's span.
   - None of them is in AUDIT.md or CONTRACT.md.

D2 (counts from record data) shares item 1's answer; D3 to D5 do not depend on it.

## Host decisions on D1' (2026-10-07)

1. **D1' is delivered by fn-211's contract A.** Its CPU port, in progress on `fn-211-tree-space-f2b-growth-years-cheap`, puts girth, sag and the layout in fixed point on both executors. Once it lands, the conversion's thinning reads exact radii and laid positions and cuts identically. There are no separate integer forms here.
2. **Until then the conversion stays as today.** The margins above (0 to 3 decisions per tree near a threshold) are the known residual, and topology is compared before conversion.
3. **D2 to D5 and the native probe proceed** exactly as CONTRACT.md specifies, against task 2's baseline B: the spruce at 80 about 8.1 s, so the 3x target is about 2.7 s. D2 uses today's radius until contract A.
4. **The beech's exact 0.4 codominance ties** are made by the code itself and are deterministic on both sides. No change.
