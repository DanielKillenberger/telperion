# The growth path, removed

The growth path grew a specimen year by year and showed it at an age: a
retained `Specimen` with an annual timeline, radius keyframes, a chronicle of
births and deaths, change records, retention and a portable snapshot, read
through `SpecimenStore` and `SpecimenView`, the Wasm `specimen_*` exports, the
renderer's `buildSpecimen` and `seekSpecimen`, and the harness's `?growth=1`.
From 2026-09-18 it was a hidden feature, never a default and never a gate. It
was removed on 2026-10-02 (fn-181; owner, 2026-09-27: "The implementation was
never good enough anyway. We'll need some different approach"). Growth over a
lifecycle stays a direction in STRATEGY.md; this page is what a rewrite starts
from. It keeps the lessons and, below them, the design as the README described
it on the day of removal. The code is gone, so nothing below names something
that still exists.

What remains: `Family.age`, the age the species' literature dimensions refer to
(the species runner pins it beside the height as the growth reference), and
the catalogue's `growth` column, which records how the growth path read each
row. The growth traits (`/growth`: `workBudget`, `rate`, `shape`,
`sheddingTolerance`, `apicalControlLoss`, `leafLifetime`, `resizeTolerance`)
are retired wire rows.

## Why it did not hold

- **It did not land on the mature tree.** Growth was meant to reproduce the
  direct build from the same value table at the preset's age. It never did: on
  2026-09-17 the silver birch at seed 1 was 73,337 nodes on the direct build
  and 590,410 on the specimen grown to age 100 (fn-67). Every verdict was
  taken on the direct build, so the growth path drew a tree nobody had judged.
- **Per-slice rebuilds of state a young tree barely uses.** Each yearly slice
  rebuilt envelope-derived state for a crown that asked few questions of it.
  fn-173's prepared crown shell, 19 to 25% faster on the mature build, made
  age-12 growth on the birch 70 times slower, because every slice prepared new
  shells. An optimisation for the product was a regression for growth.
- **It was braided into the direct build.** The growth path's clock, width
  queries and growing envelope ran through the same planner and scaffold
  builder as the mature tree, behind flags the direct build always set off.
  Every change to the product's builder had to keep a second path pinned, and
  that path cost a measured share of every speed spec (fn-91, fn-124).
- **Cost.** The specimen read was seconds per species at the birch's size
  (fn-67), against tens of milliseconds for the direct build.

A rewrite should start from one rule: a grown tree at the preset's age is the
direct build, so growth is a view over the mature tree's structure rather than
a second builder that has to converge on it.

## The design, as recorded at removal

The retained `branching::Specimen` owns scaffold and local frontiers. Nodes
carry a monotone birth order and a generational key. The annual timeline stamps
shed nodes with their death year and keeps their slots and topology unchanged;
`Specimen::node(identity)` finds survivors and rejects dead identities. Packed
reads filter dead wood and order the structural segment before local nodes.
The legacy full-envelope builder retains its existing compaction path.
The core uses pinned pure-Rust `libm` for transcendental functions. Run
`npm run wasm:build && npx vitest run harness/parity.test.ts` to compare the
five preset node buffers byte for byte across native and wasm targets.
The native `Specimen::build(&family)` path starts at a seedling and grows to
`family.age`; `advance(years)` continues its retained frontiers. Age supports
0 through 1,000,000 years, rounded to one twelve-billionth of a year at each API call (the original
billionth-of-a-month resolution). Whole years run in order and the integer
sub-year remainder carries between calls;
zero pauses, and backward inspection filters the retained chronicle at the earlier age.
The same quantized elapsed time produces the same annual history. `growth.rate`
and `growth.shape` are numeric Chapman–Richards traits and blend with age.
Their current defaults are provisional, without species age calibration.

```rust
use telperion_core::{branching::Specimen, presets::Preset};
let mut family = Preset::OregonWhiteOak.parameters();
family.age = 10.0;
let mut tree = Specimen::build(&family)?;
let mut buffers = tree.buffers()?;
let changes = tree.advance(0.25)?;
changes.validate(&buffers, &tree.buffers()?)?; // optional reconciliation check
changes.apply(&mut buffers)?;
let skeleton = tree.tree();
let earlier = tree.read_at_age(5.0)?; // owned skeleton, envelope, placements and shed identities
# Ok::<(), telperion_core::Error>(())
```

`read()` returns an owned view at the frontier; `read_at_age(years)` filters the
chronicle at an earlier age without simulation. The view contains the packed
skeleton, envelope, placements and shed identities. Radii use the last keyframe
at that age, and copied shoot histories omit future observations. A read beyond
the frontier is refused with the frontier's age.

Each native advance returns born, resized and shed runs and born, moved and shed
leaf placements. `buffers()` reads those outputs in identity order. To check a
record, call `changes.validate(&previous_buffers, &tree.buffers()?)` before applying
it; a mismatch names the run's birth identity. Application needs no fresh read.
Run-buffer radii are exact canonical keyframe values; the family's resize
tolerance controls frame creation without a separate consumer rounding grid.
Packed placements reconcile bit for bit - the three words a record carries are
the three a fresh read carries - including movement caused by an adjacent
branch changing a surface-contact polygon. A clock-only advance
returns newly reached cohorts, deriving transforms only for their shoots.
Once those cohorts are full, clock-only advances derive no placements.

Negative/non-finite advances and invalid ages name the field and value. A node
cap rolls back the failed year, retains completed years, sets `node_capped`,
and refuses the next advance. `set_node_ceiling` can raise the resource limit
and resume the same frontier. The curve's final work quantum defines saturation;
advancing beyond it jumps directly to the requested age.

Annual scaffold stations release their lateral buds while the parent axis is
still extending. Boundary pauses leave the growth budget for eligible shoots;
local terminal and lateral buds retain separate allocation state. Local runs
plan against the authored room and wait at the current crown before each birth.
The mature annual populations still differ substantially from the envelope
builds; convergence and calibration remain unfinished.

Annual shoots retain birth/death years, terminal/lateral fate and year-stamped
crown-depth vigour observations. `shoot.vigour()` reads the latest observation;
tolerance counters are retained in the same append-only event sequence.
Attractor consumption likewise retains its first consumption year.
The existing habit `sheddingThreshold` is the annual vigour threshold;
`growth.sheddingTolerance` counts consecutive active years below it, in years.
Equality resets the clock. The shell contributes to vigour, and a lit descendant
supports its ancestors. Each slice snapshots decisions before growth, sheds at
most 32 subtrees in birth order, and protects the main structural leader.
`growth.apicalControlLoss` weakens terminal control with age and releases lateral
allocation. Its default is zero; the tolerance defaults to two years. These are
uncalibrated numeric traits. Oak and spruce still have threshold zero.

Surviving structural and local radii never decrease. Local allocations and taper
are re-derived from current parents without changing twig lengths. Dead shoots
leave the growth frontiers while their records remain. A cut invalidates only
surviving pipe ancestor paths; local widths propagate from changed parents.
Dead records keep canonical final widths independent of advance partitions.
The annual solve records radius keyframes only along changed paths. A frame is
appended when any radius exceeds the last frame by more than
`growth.resizeTolerance` (metres, range 0–1, default `1e-9`); births always get a
frame. Radii never decrease. One ten-year advance retains the same frames as ten
yearly advances. Output radii still materialize once per advance, from the latest
frames, and packing remains lazy.

Integration is incomplete: `branching::generate`, `Specimen::grow`, mesh builds
and the browser still use the existing full-envelope build. The JSON wire now
round-trips and validates `age` and `growth` (`rate`, `shape`, `sheddingTolerance`,
`apicalControlLoss`, `leafLifetime`, `resizeTolerance`); those fields
currently affect only `Specimen::build`, not the full-envelope entry points.
`Specimen::placements()` returns owned leaf transforms, each identified by its
shoot's generational identity and station ordinal, before optional canopy shell
culling. `growth.leafLifetime` is a numeric family trait: one year by default and
for oak, provisionally six for spruce; zero bears no leaves. Stations are spread
across `ceil(leafLifetime)` annual cohort offsets, beginning at birth. A one-year
lifetime fills immediately; a longer lifetime fills over its first years and
then holds the same station identities while the shoot lives. Wood above the
twig anatomy's bearing diameter carries no foliage. Station randomness is keyed
by shoot identity. Unchanged wood reuses its cached transforms; changes to radii
or neighboring contact polygons re-derive only the affected shoots. This timeline
foliage path is not yet used by production. Cohort persistence fixes the earlier
bare mature crowns; the structural convergence and visual judgment remain open.
`Specimen::changes_between(from, to)` filters birth/death years, radius frames
and cohort offsets in either direction. Growing advances use the same filter.
Records carry exact keyframe radii and selected station transforms, including
motion caused by neighboring contact paths; no whole-buffer diff is computed.
`ChangeRecord::apply` updates identity-keyed consumer buffers atomically.
`build_with_history_cap(family, years)` and `set_history_cap(years)` set retention;
the default is 10,000 years. Reads older than the retained window refuse with the
cap and earliest available age. Increasing the cap cannot restore discarded data.
Compaction drops old dead geometry, shoot histories, radius frames and placements,
retaining a compact death index for the cumulative shed set and reserving identity
slots. It preserves frontier bytes, later growth and node-ceiling behavior.
`TreeEngine.buildSpecimen(family, historyCap?)` returns a retained handle.
`read()` defaults to its frontier; `read(age)` and `changes(from, to)` inspect
retained ages. `advance(years)` returns the new frontier and its change record.
Reads, records and snapshots are owned JavaScript copies. Successful specimen
rebuild/import, handle release, engine release and disposal invalidate the old
handle. Failed rebuilds/imports preserve it. `setNodeCeiling(limit)` resumes a
capped specimen, and `setHistoryCap(years)` changes retention. Native consumers
have the same operations through `specimen::SpecimenStore` (snapshot operations
require the `json` feature), or use `branching::Specimen` directly.

```ts
const specimen = engine.buildSpecimen({ ...OREGON_WHITE_OAK, age: 10 }, 100);
const before = specimen.read();
const { changes } = specimen.advance(0.25);
const earlier = specimen.read(5);
const snapshot = specimen.snapshot(); // optional, never a mesh
const restored = engine.importSpecimen(snapshot); // invalidates specimen
restored.advance(1);
```

The schema-7 specimen snapshot carries the chronicle, retained frontiers,
integer clock, cap/floor, identity slots and writer state. It omits packed reads,
foliage contact caches, crown caches and meshes. The owned `Uint8Array` is the
same byte format as native `Specimen::snapshot()` / `from_snapshot(bytes)`:
`TLPS`, a little-endian u32 schema (7), fixed-integer little-endian bincode 1.3.3
state in the declared `Specimen` field order, then an eight-byte FNV-1a checksum
of the preceding bytes. Lengths and native indices encode as u64; unlimited
canopy counts encode as UINT64_MAX, compacted identity indices as UINT32_MAX.
The adapters restore native sentinels on import. Invalid size, schema, checksum
or payload refuses before replacement. The current staging limit is 512 MiB.
`harness/parity.test.ts` exchanges snapshots in both directions between native
and wasm, advances them, and compares earlier and frontier node/placement bytes
for every preset.

The harness's age number and slider inspect the one retained specimen; beyond
its frontier they advance it. Rebuild starts a new specimen at the chosen age.
Play uses the page's years-per-second setting and carries fractional years.
`SpecimenView` applies interval records to its identity buffers, sweeps wood
again when a year changes and submits the updated placement transforms. Camera
framing remains explicit. The ordinary production `mesh::build` and
`branching::generate` routes retain the envelope build until calibration.
Its pipe cache recomputes insertion/deletion ancestor paths. An ordered scale
index visits structural wood only when its historical width can be exceeded;
local width changes propagate to descendants in birth order. Crown exposure uses
an indexed profile and caches samples until that envelope or position changes.
With shedding disabled, only frontier shoots sample vigour; other nodes retain
their last sampled state. With shedding enabled, the slice-start survival pass
refreshes the live crown and propagates descendant support.
Structural births append without moving local storage inside a slice. Internal
frontiers and pipe reductions use node kinds. Consumer reads lazily pack a
structural-first view without moving the retained frontiers' storage.
Chronicle slots are never reused, so retention boundaries cannot change
future handles. Local seeding retains unallocated stations and structural child
counts.
Full-tree validation remains available to callers; annual mutations validate
new or resized nodes. Native cost measurements, including sparse and dense
changes on large trees, run with
`FN11_MEASURE=1 cargo test --release -p telperion-core --lib monthly_cost_report -- --nocapture`.
The command retains its historical name; it now measures annual slices. Widths
finalize once per advance from the annual radius keyframes;
consumer packing is lazy and timed separately. Fixed-geometry shoots sleep until
the crown can reach them. An unchanged queue keeps its identity order and an
empty local frontier makes no width queries. Radius-dependent failures still retry.
R10 selected annual slices after the mature monthly oak measured 5.906 seconds
(native three-build median), above the approximately half-second target. The
change-record timings are included in that command. For the mature oak's
optional snapshot export/import and fresh-build equivalence checks, run
`FN11_SNAPSHOT=1 cargo test --release -p telperion-core --lib monthly_cost_report -- --nocapture`.
The final cost report and calibration remain later work.
The early annual medians were 779 ms oak and 561 ms spruce. The annual oak also
misses the target; the closed-form design remains the owner's reserve. See the
[measurement and convergence figures](scripts/benchmarks/generation.md#annual-slice-choice-fn-11-native-2026-09-13).
