# fn-11 Growth over time — status at the implementer handover

This is not the spec's report. The report R8 and R10 ask for states sourced
curves with checksums and a full measurement protocol, and the curves do not
exist yet: the work reached the end of step 3 of the spec's seven-step order and
stopped inside the cost work that follows it. What follows is what landed, the
numbers that were measured, and the three things that need the owner. Nothing
here is claimed as an acceptance criterion met.

Written 2026-09-12 by the host session. The code was written by Codex
gpt-6-astra at high effort over seven `codex exec` invocations; the host composed
each prompt, verified the five gates after every return, and made every commit.
The timebox expired at the end of the seventh.

## What landed

**Step 1 — identities and the retained frontier.** A `Specimen` beside `Tree`
retains what growth needs to continue: scaffold axis progress, pending children,
attractor positions with their spent flags, local shoots and cached runs. Node
identities come from a per-specimen monotone counter and a run takes its origin
node's; survivors keep node, parent and run identity through structural insertion
and shell compaction. Identities are generational keys — a slot plus a version
through a dense slot map — so a stale handle fails loudly instead of resolving a
reused slot.

**Pinned math, and a parity finding.** `harness/parity.test.ts` hashes each
preset's node buffer built natively and in wasm. **All five presets differed
before this task and are byte-identical after it**: R5's byte-identity across the
two targets did not hold in the shipped builder. The cause was the standard
library's transcendentals, which differ between native and wasm; the fix is
pinned `libm =0.2.16` with default architecture features off, behind the core's
own math wrapper. The re-pin that followed moved the branching audit hashes and
the oak and spruce skeleton and placement pins, with counts, topology, bounds and
element pins untouched and maximum position drift 1.8e-13 m on the oak.

**Step 2 — the monthly clock, the curve budget and the wire.** `growth::Age` is
an integer month count plus an integer sub-month remainder at one billionth of a
month, so no float accumulator can make two partitions of the same advance
disagree. `GrowthTraits` are a Chapman–Richards rate and shape, and a slice's
unit budget is the difference of *rounded cumulative* work, so every quantum
belongs to exactly one month however the advance was cut. Growth saturates: the
mature month is found by bisection rather than by stepping, a zero-budget slice
changes nothing, and a build past maturity jumps the empty span.
`Specimen::build(&Family)` and `advance(years)` grow through those slices with a
transactional node ceiling that leaves the last complete slice. `/age`,
`/growth/rate` and `/growth/shape` are on the JSON wire with validation, in the
generated browser metadata and in the sweep's exact schema inventory; the frozen
fn-19 protocol test drops the timeline fields before comparing, following fn-14's
material-row precedent, with the frozen evidence itself untouched.

**The frontier repairs — the bare-pole defect.** Routed through the monthly path
the spruce first grew 599 nodes: a leader with almost no laterals. Five repairs,
each red first, fixed it. Laterals are released on every monthly visit rather
than when the parent axis completes — an excurrent leader had been holding its
lateral axes until the lifetime budget was nearly spent. A structural pause on an
expanding envelope refunds its unit and a local visit is charged only when wood
grows, so waiting frontiers no longer starve live shoots. A local station is
marked seeded after eligibility rather than before. Cached runs plan against an
internal planning envelope, so a juvenile crown no longer clips a run for life,
while the current-crown guard still gates every birth. A station allocates its
terminal and its laterals under separate bits. The spruce now grows 72,375 nodes.

**Step 3 — thickening, shoot state and shedding.** Local branches and twigs are
re-derived each slice from their parent's current radius by the existing
child-radius rule, so a surviving local branch no longer keeps its birth radius
while its parent thickens, and surviving radii are monotone non-decreasing across
extension, forks and shedding. Shoot state per node carries birth year, bud fate
as terminal or lateral, and a vigour proxy from crown depth. A shoot below its
family's threshold for the family's tolerance period is shed with its subtree and
its identity retired, decided from a slice-start snapshot with a fixed comparison
side and a cap on sheds per slice, with the existing shell rule folded into vigour
rather than left as a separate pass. The threshold, the tolerance period and
apical loss with age are numeric family traits, validated, blended and published
— no species branch anywhere in the growth code.

**Test discipline.** Every new test was confirmed red before it passed, and the
notes name the log for each. Several failed against the real implementation
rather than against a mutation, and those are defects this work found: a zero node
ceiling that built a root anyway, a cached local run appending outside the current
crown at age 24.33, and blend arithmetic pushing a valid growth rate just outside
its own range.

## What was measured

Native, with the core's own timers, no GPU capture. These are step 7's numbers
taken early because the cost work needed them; the full R10 protocol, including
the snapshot round trip, is not done.

| | slice at 20 / 100 / mature | mature build | envelope build today |
|---|---|---|---|
| oak, before the cost work | 89.8 / 211.5 / 178.3 ms | 198.7 s | 76.4 ms |
| oak, after | 4.55 / 18.71 / 18.40 ms | **18.9 s** | 62.8 ms |
| spruce, before | 29.8 / 48.3 / 45.3 ms | 59.0 s | 33.5 ms |
| spruce, after | 3.52 / 8.36 / 5.19 ms | **9.15 s** | 35.6 ms |

**Does a slice cost what it changed?** On the spruce, yes: a slice changing 264
records costs 0.84 ms where one changing 65,989 costs 5.19 ms. On the oak, not
yet: a slice changing 1,698 records costs 13.69 ms against 18.40 ms for one
changing 203,775. Of that sparse-slice cost, 8.14 ms is structural insertion
moving existing local storage and 4.38 ms is frontier retries for waiting shoots.

A mature build is still 301 times the envelope build on the oak and 257 times on
the spruce.

## The three things that need the owner

**1. Is a mature build of 19 seconds acceptable, or does the design change?** The
spec's own Implementation Tradeoffs keep the closed-form alternative — birth times
fixed by seed and identity — "available if the per-slice cost proves too high",
and STRATEGY's Build metric holds dial-to-tree time to what keeps dragging usable.
Nineteen seconds does not, and a game that regenerates in view would pay it per
tree. The measurement above is what that decision needs; the call is not the
implementer's.

**2. The mature population differs from today's trees, and the routing waits on
it.** At seed 7 and the derived maturity of 172.75 years the monthly oak carries
215,964 nodes against the envelope build's 139,040 (+55%) and the spruce 74,667
against 90,439 (−17%), with structural nodes up 50% and 22%; bounds agree within
1.8% on the oak and 6.6% on the spruce, so these are trees of the right size and
shape carrying the wrong amount of wood. Both presets author a zero shedding
threshold, so the excess is what a calibrated threshold in step 6 has to remove.
Production `branching::generate` is therefore still the envelope build, and the
re-pin the coordinator authorized has not been spent: routing it now would pin an
uncalibrated tree. What the monthly path grows at maturity is the R11 judgment.

**3. R11's verdicts remain the owner's**, and no still has been rendered to judge.

## What remains

Step 4 (leaf lifetime and the change record), step 5 (the wasm specimen handle,
the native API, the harness age dial and play rate, the portable snapshot), step 6
(the curves and the calibration, then the routing and the re-pin), step 7 (the
full R10 protocol including the snapshot round trip, the age strips, the stills
and the report). **No R8 curve was sourced and no still was rendered.** A separate
host search corroborated the spec's research: no freely accessible open-grown
age-indexed height or diameter curve exists for either species, and the Urban Tree
Database's coverage of them could not be confirmed through automated fetches, so
the amended R8's named-composition path is the expected route when step 6 is
reached.

## Evidence on disk

The implementer's notes, with every red-then-green log named and every number's
command recorded, are at `/tmp/flow-handover-fn11/child-notes.md`; the raw logs
and the seven session transcripts are beside them and are not committed. Prior-art
sources read for the growth-trait vocabulary (EvoEngine's shoot descriptors,
PlantArchitect's `GeneralTreeBehaviour`) were fetched into the gitignored
`.refs/fn11/`; no source was copied. All five gates — `cargo fmt`, `cargo clippy
-D warnings`, `cargo test --release --workspace`, `npm run wasm:build && npm
test`, `npm run typecheck` — were green at the base commit and are green at HEAD,
verified by the host after each of the seven invocations.
