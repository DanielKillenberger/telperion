# A tip extends by what its balance leaves

Child of fn-351 (P1's old trees; Phase 3). Host-written, 13 Nov 2026. Builds on `fn-363-a-tip-is-charged-by-its-height`.

## Problem

With the charge read against height (fn-363) the old Oregon white oak keeps its crown, and grows too big: the trunk at 1.3 m at 300 is 2.20, 3.13 and 2.90 m on seeds 1001, 1007, 1012 against White's open-grown 1.56 to 1.92 m, and the tallest tip reaches 36.8, 38.8 and 39.4 m against the species' potential height of 34.8 m (`.flow/evidence/fn-351-020-the-release-candidate/SURPLUS-PROBE.md`, round 3, V4). The cause is in the code as read: the balance (`grow/cycle/passes.rs`) only decides death. A tip whose charge has reached what it earns goes on laying full-length units until the hazard takes it.

Probed (same file, V8 = fn-363's charge with the rule below; seeds 1001 and 1007): tallest tip 26.8 and 26.9 m at any age; trunk at 120 1.06 m (White 0.83 to 1.09), at 300 1.71 and 2.53 m; settled leaf over the unruled tree's at 40, 80, 120: 0.97, 0.93, 1.00 and 0.97, 1.05, 1.32; leaf at 300 over leaf at 120: 0.34 and 0.55; 87 s and 1.5 GiB at 300 on seed 1001. The variants that shrink the whole unit with its leaves (V1, V7) are refuted there: many small leaves, up to five times the phytomers per m² of leaf, one tree past 24 GB.

## The design (host)

A living tip's surplus share is what its balance leaves of what it earns:

  s = max(0, 1 − (h · inverse_height)² / light)

with h, `inverse_height` and `light` exactly the quantities fn-363's charge and `balance_one` read for that tip in that cycle (light 0 gives 0; no potential height gives 1). No new value.

- **s shortens the unit's internodes, and nothing else.** The length each phytomer of the unit is laid at during growth (so the tip's path and position) and its length in the settled tree are multiplied by s. The unit's leaf area, the deposit the light field reads, the sapwood demand from its leaf, its girth, its node count and its presence keep the allotment's size, as today. A tip near its limit bears full leaves on short internodes.
- **It is computed in the allotment** (`allot_one`), after `sizes`, for every living apex, and recorded per growth unit beside the unit's shade size, so a year is still a state: the rule reads only that cycle's pencil.
- **The settled tree says its lengths.** The settled phytomer carries the stretch (the probe's `v3-plumbing.patch` and `v8.patch` in `scratchpad/surplus/` show one way; nothing is copied without a rewrite and a test), and every reader of a phytomer's length reads it through one function, in the space crate and in the pipeline (`telperion-core/src/pipeline/space.rs` computes an internode's piece from `scale` today, line 373 at the probe's commit). No reader multiplies `scale` by an internode on its own after this.
- The whole-tree test engine (`sketch.rs`'s `allot`) takes the same rule if any test holds it to the one cycle; the reading says which.

Expected, from the probe (the builder returns if a measure is off these by more than a quarter): the oak on seeds 1001 and 1007 as quoted above; at 40 within 5% of fn-363's tree in height and leaf.

## Acceptance Criteria

- **R1.** The reading, in the evidence, before the change: every reader of a phytomer's length in both crates (lay, reaches, stand, sag, the pipeline's conversion, the surface stage's joins, leaves' placement, anything in the render crate that reads the grown structure), each marked "reads position" or "reads scale times internode"; what a zero-length internode does in each (the probe grew 842,770 of them and built a wood mesh without error; whether a run of coincident rings is drawn, dropped or degenerate in the surface stage is not known); the bytes added per settled phytomer, per unit and per axis, with the type chosen and why; which compactness or size tests see it.
- **R2.** Tests, red before and green after: a hand-built tip at light L and height h lays a unit of length (allotment length) × max(0, 1 − (h/H)²/L) with its leaf area unchanged; at h = 0 or no potential height the tree is equal to the bit to fn-363's; a tip past its limit lays zero-length internodes and keeps its nodes and leaves; the settled lengths equal the growing lay's.
- **R3.** Year-independence: the five tests of `grow/cycle/paused/tests.rs` pass (`every_year_is_the_horizons_prefix` by name), and the oak grown to 80 at once equals the oak paused at 40 and grown on.
- **R4.** Pins and bounds as in fn-363's R4: every one that moves listed with its tree, repinned by its own procedure; a bound never loosened to pass. A size or compactness budget that the new record exceeds is not raised: return with the bytes.
- **R5.** Measured, fn-363's branch against this one, grower and settle only, with fn-363's tooling: every preset at its preset age, 40 and 80 on seeds 1 and 7; the oak at 120, 200, 300 on seeds 1001, 1007, 1012; fn-363's columns plus leaf per leafed phytomer, phytomers per m² of leaf, the count of living tips with s = 0, and zero-length phytomers. The oak's trunk beside White's ranges and each species' tallest tip beside its potential height.
- **R6.** Stills on one sheet per species for the host's verdict: the oak whole and leafless at 40, 80, 120, 200, 300 on seeds 1001 and 1007, in three columns: the base's `base-2026-11-11`, fn-363's branch, this branch. Every other preset at its preset age on seeds 1 and 7, the same three columns. A close crop of one old oak limb's outer metre at 200, leafless, on both branches, to show what short internodes with full leaves look like.
- **R7.** `docs/pipeline.md` states the rule beside the charge; the generation-limit inventory is updated if the guard asks.

## Stops

- A zero-length internode that the drawing shows as a fault (a pinched or doubled ring, a gap, a leaf cluster at a point that reads as a ball): return with the still.
- Any measure of the oak off the probe's by more than a quarter; any other preset whose height at its preset age falls by more than 15%, or whose leaf moves by more than 25%: return with the numbers before drawing.
- A budget (bytes, compactness, the gate's memory list) that the record exceeds.
- Any question about the form of s, about scaling leaf, the hazard, the settle's fade or a preset row.

## Boundaries

The surplus share, its record, the length readers. No change to the charge, the hazard, the sizes, the settle's shed or fade, any preset value, or the drawing beyond reading length through the one function.

## Rigor

Opus builds; narrow tests while building; heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, one GPU job at a time, under the yield watcher (jobs in the background, checked by the clock; seconds only with no PAUSE inside). The host views the sheets and gives the verdict on the old oak here; the gate and review at the phase's landing.
