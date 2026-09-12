# fn-11 Growth over time — status at the implementer handover

This is not the spec's report. The report R8 and R10 ask for states sourced
curves with checksums and measured per-slice and mature-build costs, and none of
those numbers exist yet: the work stopped inside step 2 of the spec's seven-step
order. What follows is what landed, what it proved, and the two things that need
the owner before the spine can be finished. Nothing here is a measurement, and
no acceptance criterion is claimed met.

Written 2026-09-12 by the host session. The code was written by Codex
gpt-6-astra at high effort over three `codex exec` invocations; the host composed
each prompt, verified the five gates after every return and made every commit.

## What landed

**Identities and the retained frontier** (step 1, most of it). A `Specimen`
beside `Tree` retains what growth needs to continue: scaffold axis progress,
pending children, attractor positions with their spent flags, local shoots and
cached runs. Every node takes a birth identity from a per-specimen monotone
counter, and a run takes its origin node's. Structural insertion remaps local
storage and pending shoots while survivors keep node, parent and run identity,
and the frontier is refreshed after shell compaction. The identities are
generational keys — a slot plus a version, through a dense slot map — so a stale
handle fails loudly instead of resolving a reused slot, which is the ABA bug a
hand-rolled index map invites.

**A native-to-wasm parity finding.** The probe the spec's amended Edge Cases
asks for, `harness/parity.test.ts`, hashes the node buffer of each preset built
natively and in wasm and compares. **All five presets differed before this task
and are byte-identical after it.** R5's byte-identity across the two targets did
not hold in the shipped builder. The cause is the standard library's
transcendentals, which differ between native and wasm; the fix is one pinned
pure-Rust implementation, `libm =0.2.16` with default architecture features off,
behind the core's own math wrapper.

Hashes before the fix, native against wasm, over six `f64` node fields followed
by three `u32` topology fields, little endian:

| Family | native | wasm |
|---|---|---|
| ordinary | `8c8ea752…3424` | `ac93ab1a…5584` |
| oregon-white-oak | `f9c636b7…2bce` | `c16e4028…b284` |
| norway-spruce | `a20961f7…d2c6` | `abe0dd62…f2b8` |
| telperion | `2ab7c443…6f5a` | `cf94c715…3f7a` |
| laurelin | `c9ab4c17…1690` | `fd2d0bcd…b137` |

Node counts matched in every case; only the bytes differed. After the fix all
five comparisons pass, and the drift the fixed math introduced against the old
platform math is 1.8e-13 m at most on the oak and the spruce and 1.3e-8 m on
Laurelin, with no change to counts or topology.

**The monthly clock and the curve budget** (step 2's core, native only).
`growth::Age` is an integer month count with an integer sub-month remainder at
one billionth of a month, so no float accumulator can make two partitions of the
same advance disagree. `GrowthTraits` are a Chapman–Richards rate and shape; a
slice's unit budget is the difference of *rounded cumulative* work, so every
quantum of growth belongs to exactly one month however the advance was cut.
Growth saturates as the amended Architecture requires: the mature month is found
by bisection rather than by stepping, a slice whose budget rounds to zero changes
nothing, and a build to an age past maturity jumps the empty span.
`Specimen::build(&Family)` and `advance(years)` grow a specimen through those
slices with a transactional node ceiling that leaves the last complete slice.

**Eleven monthly tests, each confirmed red before it passed**: remainder carry
across irregular advances, a pause and a sub-month advance, the enumerated value
errors, ceiling rollback, identity-order traversal against storage order,
saturation, trait interpolation, budget expansion, a zero ceiling that must not
build a root anyway, a cached local run that appended a node outside the current
crown at age 24.33, and blend endpoint clamping where weighted arithmetic pushed
a valid rate just outside its own range. The last three failed against the real
implementation, not against a mutation — they are defects this pass found and
fixed.

## The two things that need the owner

**1. The oak's identity pins stand in the way of the production path.** Routing
`branching::generate` through the monthly build at the curve-derived mature month
changes the oak's topology: the seed-7 skeleton hash moves from
`14986275773972546726` to `15191549139260502920`, and the audit hash from
`11389017044164293456` to `15153313695119125838`. This was measured with a
temporary probe that replaced only the one call and restored it in a `finally`
block; no pin or assertion was edited. The spec's drift clause allows the pins to
be re-pinned once, with the mature oak and spruce judged again by the owner — and
that single allowance was already spent on the parity fix above. So the
production path is still the old envelope build, and the monthly implementation
lives beside it as `Specimen::build` plus `advance`. Whether the mature tree the
monthly path produces is acceptable is exactly the R11 judgment the owner holds;
it cannot be settled by a test, and it gates step 2's integration and everything
after it.

**2. Age cannot reach the JSON wire without a protocol migration.** Adding `age`
to `params::metadata` fails
`geometry_benchmark::frozen_parameters_resolve_without_default_substitution`,
which reads the frozen `.flow/evidence/fn19/protocol.json` and reports that
frozen parameters omit `/age`. The protocol is immutable by fn-19's own terms and
the implementer was not permitted to write under `.flow/`, so age is a native
family field that blends, and its wire exposure is deferred. R4 wants age walked
by the blend like every other field, which holds natively; R9's wasm handle needs
the wire, so it needs this decision first.

## What remains

The tail of step 1 (routing the whole build through the age path, blocked on the
pins above) and steps 3 through 7 in full: thickening and shedding by vigour;
foliage by leaf lifetime and the change record; the wasm specimen handle, the
native API, the harness age dial and play rate; the curves and the calibration;
the cost measurement, the age strips, the mature stills and the report. The
radius work so far caches fork reductions on the ancestor paths of changed nodes,
which is the beginning of "a slice costs what grew" but not its completion —
trunk-scale writes, frontier scans, identity scans and local reindexing still
touch existing storage, so R10's requirement that a slice scale with the nodes it
changed is not yet met and was not measured.

**No R8 curve was sourced and no R10 number was taken.** Step 6 was never
reached, so there are no sources, no checksums, no three-age deviations, no
per-slice or mature-build costs and no snapshot round trip. No still was
rendered, no GPU capture was taken and no dev server was started. A separate host
search corroborated the spec's research on the curves: no freely accessible
open-grown age-indexed height or diameter curve exists for either species, and
the Urban Tree Database's coverage of them could not be confirmed through
automated fetches, so the amended R8's named-composition path is the expected
route when step 6 is reached.

## Evidence on disk

The implementer's own notes, with every red-then-green log named, are at
`/tmp/flow-handover-fn11/child-notes.md`; the raw logs are beside them in that
directory and are not committed. The prior-art sources read for the growth-trait
vocabulary (EvoEngine's shoot descriptors, PlantArchitect's
`GeneralTreeBehaviour`) were fetched into the gitignored `.refs/fn11/`; no source
was copied. All five gates — `cargo fmt`, `cargo clippy -D warnings`, `cargo test
--release --workspace`, `npm run wasm:build && npm test`, `npm run typecheck` —
were green at the base commit and are green at HEAD, verified by the host after
each of the three invocations.
