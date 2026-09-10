---
satisfies: [R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.2 One outline routine: lobes and section roundness as traits

## Description
Dissolve the element anatomy enum into a lobe count, a lobe depth and a section roundness on the numeric outline path, so the oak blade and the spruce needle are rows and every element in between builds with a level ladder (R7 element half).

**Size:** M
**Files:** `crates/telperion-core/src/foliage/element.rs` (split into outline and section modules under 400 lines each), `crates/telperion-core/src/foliage/levels.rs` (validation only), `crates/telperion-core/src/params.rs` (element rows), `crates/telperion-core/src/presets.rs` (element rows), element tests
**Touches:** [crates/telperion-core/src/foliage/element.rs, crates/telperion-core/src/foliage/outline.rs, crates/telperion-core/src/foliage/levels.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/**]

### Approach
- The generic outline at `element.rs:120-228` already reads widest point, base fullness, tip sharpness, cup and curl. Add a lobe term to the half-width profile: lobe count sets the number of crests along the margin and lobe depth how far each sinus cuts toward the midrib, with the oak's five-lobe table at `element.rs:364-380` as the calibration target for count 5 and depth 0.7, then delete it.
- Add a section term: section roundness blends the transverse section from the flat strip the blade uses to the four-sided shaft the needle builds at `element.rs:229-300`, then delete the needle routine and `build_anatomy`.
- Validation: axial section count at least twice the lobe count plus two when lobe depth is positive, error naming both fields; connector length rules stay.
- The level ladder in `levels.rs` reads sections and positions only; confirm it builds for count 0 and 8, depth 1 and roundness 1, and add those cases to its tests.
- Wire rows for the three traits in the `fields!` table; remove the `ElementAnatomy` `enum_wire!` row at `params.rs:192-196`. Presets set the traits from the spec's second table at `presets.rs:87,119`.
- Add the element hash step test: stepping each element trait on each shipped preset changes the element hash.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/foliage/element.rs:29-60,120-228` — parameters and the generic outline path to extend
- `crates/telperion-core/src/foliage/element.rs:229-380` — the lobed and needle routines being replaced and the oak lobe table
- `crates/telperion-core/src/foliage/levels.rs:1-60` — the section-based ladder and its invariants

**Optional** (reference as needed):
- `crates/telperion-core/src/params.rs:181-196` — the enum wire rows to remove

### Key context
- The element's output shape is the mesh contract fn-23 fixed: sections, positions, indices, levels. Only how the sections are computed changes.
- Section count rounds up and lobe count rounds down in a blend so the validity rule survives rounding; note this for task 5.

## Acceptance
- [ ] No `ElementAnatomy` identifier remains; one outline routine builds every element from numeric traits
- [ ] Lobe count, lobe depth and section roundness are validated fields on every family; a section count below the lobe rule is rejected naming both fields; an anatomy tag on input is rejected naming the field
- [ ] Oak and spruce element rows build elements the fn-23 level ladder accepts, with the ladder tests extended to the trait extremes
- [ ] Stepping each element trait on each shipped preset changes the element hash
- [ ] Element modules each under 400 lines; `cargo test --release -p telperion-core` and clippy pass

## Done summary
One outline routine now builds every foliage element from numeric traits: the
element anatomy enum is gone, a lobe count and lobe depth cut sinuses into the
outline profile, and a section roundness rolls the flat strip into the
four-sided shaft the needle used to build by its own routine. The oak blade and
the spruce needle are rows in the same table, and every point between them is an
element the fn-23 level ladder accepts.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after it integrates)
stage: host-review-fix - ran(one finding, the oak margin drawn as a sawtooth; fixed in 8e630d3)
stage: owner-verdict-fix - ran(broaden the lobes, narrow the sinuses; fixed in 543627a)

### The owner's verdict, and the third commit

The margin was a plain cosine, so every lobe was a pointed chevron and every
sinus was as wide as a lobe. Commit `543627a` raises the sinus term
`(1 + cos)/2` to the sixth power in `outline.rs`. That broadens the crest and
narrows the notch without moving either, keeps the margin continuous in every
trait, and adds no field: `MIDRIB`, lobe count 5, depth 0.7 and the forty
stations are all as they were.

### Picking the exponent by measurement

The band the coordinator named — a lobe half-extent of 0.115 to 0.15 of the
length — **cannot be reached by this parameterization, and the reason is worth
recording.** Five lobes spread evenly along the blade put crests 0.2 apart, so
a lobe's half-extent is capped at 0.1, half its own period, whatever the
exponent (at the sixth power it measures 0.079 at the half-depth contour and
0.092 at nine tenths of full cut). The retired table exceeded that band only
because its five ellipses were 0.115 to 0.15 wide on centres 0.157 to 0.183
apart: they overlapped, and its "sinuses" were the crossings between
neighbours rather than gaps cut down toward the midrib. That is also why its
notches were only 26.7% deep where these cut to 41.9% of the local envelope.

So I measured the shape the owner actually named — broad lobes, narrow
sinuses — with two contours defined identically on both margins: the share of
one lobe period lying within a quarter of the crest (the broad top), and the
share lying within a quarter of the sinus floor (the notch).

| margin | broad top | notch | notch depth |
|---|---|---|---|
| retired five-lobe table | 60.2% | 16.9% | 26.7% of the crest |
| plain cosine (the rejected blade) | 30.0% | 34.8% | 56.8% |
| sixth power (shipped) | **60.2%** | **17.2%** | 59.2% |

The sixth power reproduces the retired table's lobe-to-sinus proportions to
within a third of a percentage point on either contour. The sweep to each
side: the second power gives 43.3% / 26.3%, the third 50.2% / 22.4%, the
fourth 54.6% / 20.1%, the fifth 57.8% / 18.4%. Six is the match, not a round
number picked by eye. The notch stays deeper than the retired table's by
construction, because depth 0.7 is the spec's row and the instruction was to
keep it.

Re-rendered to
`/tmp/flow-handover-fn24/stills/fn-24.2-oregon-white-oak-leaf-broad.png` and
viewed once: the chevrons are gone, the lobes are broad and rounded, the
sinuses are narrow. One honest caveat for task 6: at forty stations a notch
occupying 17% of a lobe period spans about one and a half of them, so the
sinus floor comes to a point rather than a curve. Rounding it needs stations
inside the notch — either more of them, or spacing that concentrates where the
margin bends. The leaf is still 129 vertices and 172 triangles against the
retired blade's 268, so there is room if the owner wants it.

**The spruce is byte for byte what it was.** Its element hash,
`7287062639823569932`, is unchanged across all three commits: at lobe count 0
the sinus term is not taken at all, so no exponent can reach it.

The oak element hash and the oak's mesh bounds were re-recorded a third time
(the bounds by under a millimetre, from the broader lobes). Wood counts,
instance counts, skeleton and placement pins have not moved since task 1. The
oak's ladder is six levels now rather than four — a margin with more shape in
it gives the ladder more to say — still inside the four-to-eight the ladder
test holds it to, and its coarsest level still inside the twelve-triangle cap.

### The host review finding, and the second commit

The host rendered the single-leaf view and found the oak margin reading as a
sawtooth: at twenty axial segments each half-lobe carried two stations, so the
cosine margin was drawn as the straight zigzag between crest and sinus rather
than as the curve through them. Commit `8e630d3` raises the oak row's
`axial_segments` to 40. Stations every 0.025 still land exactly on every crest
(0.1, 0.3, 0.5, 0.7, 0.9) and every sinus (0.2, 0.4, 0.6, 0.8), and each
half-lobe now carries four samples, so the silhouette follows the curve. Lobe
count, lobe depth and the whole spruce row are untouched.

Re-rendered to
`/tmp/flow-handover-fn24/stills/fn-24.2-oregon-white-oak-leaf-40.png` and
viewed once: the sawtooth is gone and the lobes and sinuses read as curves. The
crest apexes are still a little angular, since a crest is one station with its
neighbours a curve-width to either side; more stations keep softening it and
there is headroom to do so — the leaf is 129 vertices and **172 triangles
against the retired blade's 268**, and even sixty segments would sit under that
count. Whether it should go further is the owner's call at task 6, not this
task's.

The oak element hash was re-recorded a second time. The mesh bounds, wood
counts, instance counts, skeleton and placement pins did not move, and no
species-metrics number changed: `tests/species_metrics.rs` builds its own
element rows rather than the oak's, and the oak's gating profile metrics
(`foliage_length_m`, `foliage_width_m`) are set by length and width, not by the
station count. The oak's ladder is four levels — 4, 16, 76, 172 triangles — so
the coarsest is still well inside the twelve-triangle cap the ladder test holds
it to.

### What changed

- `crates/telperion-core/src/foliage/outline.rs` (new, 59 lines): the margin —
  the widest point with its base fullness and tip sharpness, times a lobe term
  with `lobe_count` crests at (2k+1)/2n and a sinus between each pair and at
  either end, each cutting `lobe_depth` of the way to a midrib of 0.17 of the
  local envelope, the cut distributed along the period by the sixth power that
  the third commit measured — and the transverse section, a linear blend from
  the flat cupped strip at roundness 0 to the four-sided shaft at 1.
- `crates/telperion-core/src/foliage/element.rs` (296 lines): `ElementAnatomy`,
  `build_anatomy` and the hard-coded five-lobe oak table are deleted. One
  routine builds base point, rows, tip, connector for every element; the card
  stays a two-triangle stand-in.
- `params.rs`: `element.lobeCount`, `element.lobeDepth`,
  `element.sectionRoundness` are flat wire rows; the `element.anatomy`
  `enum_wire!` row is gone and the closed schema refuses an anatomy tag with
  `InvalidInput("unknown element trait")`, the same shape task 1 gave the habit.
- `presets.rs`: oak 5 / 0.7 / 0 with a recalibrated envelope (widest_at 0.55,
  base_fullness 0.6, tip_sharpness 0.6) and 40 axial segments so crests and
  sinuses land on sampled stations with four samples to a half-lobe (20 in the
  first commit, raised by the host review); spruce 0 / 0 / 1 with widest_at 0.2,
  fullness and sharpness at 0.2 and four cross segments. Every other preset
  takes the defaults, 0 / 0 / 0.

### The section, and why it has no seam vertex

A section is `columns + 1` samples across `u` in -1..1. At roundness 0 that is
today's flat strip. At 1 the same samples run clockwise around a four-sided loop
from a seam at the top, so the first and last column land on the same point and
the shaft is closed without a wrap in the index list and without one degenerate
triangle. Nothing branches on the extremes: roundness 0, 1 and everything
between run the identical arithmetic, which is what makes the oak-to-spruce
transition continuous through the leaf.

### Calibration against the retired shapes (measured, not asserted)

Needle section reach, as a fraction of half the authored width, at the sampled
stations, new against the retired taper:

| t | new | retired | delta |
|---|-----|---------|-------|
| 0.00 | 0.000 | 0.350 | the base is a point, not a collar |
| 0.20 | 1.000 | 1.000 | 0.0% |
| 0.40 | 0.984 | 0.970 | +1.5% |
| 0.60 | 0.933 | 0.940 | -0.7% |
| 0.80 | 0.825 | 0.910 | -9.3% |

Within a few percent along the shaft, as the task asked; the distal station
tapers 9% narrower, and the retired blunt base collar becomes a point. The
needle is 32 vertices and 48 triangles against the retired 22 and 72.

Oak half-width against the retired five-lobe table, both as a fraction of the
authored width, at the 21 sampled stations: RMS 0.245. The retired table is
neither periodic nor deep — its lobe centres sit at 0.15, 0.36, 0.59, 0.78,
0.88 and its sinuses only dip to about three quarters of the local envelope. A
periodic term with five crests cannot land on irregular centres: the new crests
at 0.1, 0.3 and 0.9 fall within 0.03 to 0.08 of the retired reach, while the new
sinuses at 0.4 and 0.6 fall where the retired table put its two middle lobes.
Fitting depth freely, the retired table is best matched at depth ≈ 0.2
(RMS 0.10); at the spec's row of 0.7 the sinuses cut to 0.29-0.42 of the local
envelope where the retired ones stopped at 0.51-0.75.

**This is a deliberate, reported difference.** The spec's table names 0.7 and
the task names it as the calibration target; the honest reading of "how far each
sinus cuts toward the midrib" puts 0.7 well past the retired shape, and a
deeply lobed blade is what Quercus garryana actually carries. The retired table
is no longer reachable, so if the owner wants the shallower margin back it is
one number: `lobe_depth` on the oak row. The oak leaf is 129 vertices and 172
triangles against the retired 190 and 268, so the crown got cheaper too.

**This station-by-station table is superseded by the third commit** and is kept
because it is what the depth argument above rests on: it was measured at twenty
stations with the plain cosine margin, before the sixth power broadened the
crests. The shipped margin keeps every crest and sinus where this table put
them and the same 41.9% floor at each sinus, but spends far less of the period
cutting, so its stations between crest and sinus sit nearer the envelope than
the numbers above. The lobe-to-sinus proportions of the shipped margin are the
measurement in "The owner's verdict" section at the top.

### Deliberate deviations from the task's approach note

- **The base of every element is a point.** The retired needle opened with a
  four-vertex collar at 0.35 radius and a cap centre; the retired blade began at
  the origin with no connector. One routine gives both a single base vertex on
  top of a connector, so `FoliageUnit`, sections and the ladder read the same
  shape whatever the traits say.
- **Every element carries a connector.** `connector_length` was validated in
  1e-6..length for the species anatomies and ignored entirely for the generic
  blade; it is now that rule for every element, so the Ordinary and Two Trees
  leaves gained the centimetre of petiole their row already asked for and sit
  that much higher. This is the retired hidden branch, not a new parameter.
- **`Element::anatomy` is always `Some`** for a built element (the card aside),
  where the generic blade used to leave it `None`. Consequence for the wasm
  diagnostics: `foliageAnatomy` and `biologicalUnits` are now populated for
  every preset rather than only the two species, and `species_metrics` reports
  the measured blade note instead of the whole-prototype estimate note for them.
  `src/browser/core.ts` types that field as nullable, so nothing there breaks.
- **`FoliageUnit` is derived from roundness** (`>= 0.5` is a needle) rather than
  from a tag. It is a label on the output for measurement; nothing in the build
  reads it.
- **A card is refused only when it carries lobes or roundness**, naming both,
  where the retired rule refused any card with a species anatomy.

### Files outside the declared Touches, and why

- `crates/telperion-core/examples/geometry_benchmark.rs` — its fn-19 capability
  strings read `element.anatomy`. They now read `lobe_count > 0 && lobe_depth >
  0` and `section_roundness >= 0.5`, the same continuous conversion task 1 made
  for `tiered-secondary`. Deleting the enum could not leave it compiling.
- `crates/telperion-render/tests/conformance.rs` — its `compact()` already moves
  habit traits that sit on their own bound off it so a quarter jitter is still a
  tree; the spruce's new row sits on three element bounds
  (`sectionRoundness` 1.0, `baseFullness` 0.2, `tipSharpness` 0.2) and was being
  refused a quarter of the time. Three lines added there, same convention, same
  comment style. Refusals in that sweep fell from 11 to 8 (the remaining 8 are
  the oak's and spruce's `shellDepth` of 1.0, which predates this task).

### Pins re-recorded

`tests/identity.rs`: both element hashes, and the oak's mesh bounds (sub-
millimetre, from the new blade). Wood counts, instance counts, skeleton and
placement hashes are unchanged — placement does not read the element, so
task 3 inherits them as task 1 left them.

### What the siblings need to know

Written in full to the run-notes directory as `task2-integration-notes.md`.
The short version: `ElementParams` lost `anatomy` and gained three traits;
`presets.rs` element blocks are rewritten in the oak and spruce sections and
will conflict textually with task 3's canopy edits (take both sides); the
TypeScript surface (`src/browser/presets.generated.ts`, `harness/params.ts`,
`harness/GrowerDev.tsx`, `tests/browser/bindings.mjs`) still speaks
`element.anatomy` and is task 4's to regenerate and retire.

### Integration and the host review (conductor, 2026-09-10)

Cherry-picked onto the spec branch as 6d11d09, 5e6d7d8 and 68a3e76; the workspace commits 6c90c2b, 8e630d3 and 543627a are retired with the worktree. Two review rounds ran before completion. The first found the oak margin drawn as a sawtooth at 20 stations and asked for 40, so every crest and sinus keeps a station and each half-lobe has four. The second put the 40-station blade before the owner, who asked for broad rounded lobes and narrow sinuses; the worker shaped the sinus term with a sixth power, chosen by matching the retired table's lobe-to-sinus proportions (60% broad top, 17% notch), and the spruce needle pin stayed byte-identical through all three commits. The host viewed the final single-leaf stills of both species: the oak reads as a lobed oak blade, the needle as a four-sided shaft.

> _owner (2026-09-10), on the 40-station blade:_ broaden the lobes now.

Carried to task 6: the sinus floor is one station wide at 40 stations; more stations or spacing concentrated at the bends would round it, and the leaf sits at 172 triangles against the retired 268.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 6d11d09, 5e6d7d8, 68a3e76
- Tests: worker: cargo test --release --workspace - 131 tests, 29 suites, green at each of the three workspace commits, worker: cargo clippy --release --workspace --all-targets - clean, worker: cargo fmt --all -- --check - clean, conductor, integrated target 68a3e76: cargo test --release --workspace - green, rc=0, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --view leaf --out /tmp/flow-handover-fn24/stills/fn-24.2-oregon-white-oak-leaf-broad.png, cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 7 --view leaf --out /tmp/flow-handover-fn24/stills/fn-24.2-norway-spruce-leaf.png
- PRs: