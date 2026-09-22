---
satisfies: [R1, R2, R3, R4]
---
# fn-110-the-palms-trunk-organs-persistent-leaf.1 Implement The palm's trunk organs: persistent leaf bases and acanthophylls

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Eight canopy rows give the generator the palm's two trunk organs: the retained
bases of shed fronds, hung on every stem as wood of their own after the radius
solve and carrying the crown's own spiral down the trunk, and the basal
leaflets of a frond borne as spines. Both are off at their neutrals, so every
shipped preset keeps every byte; the date palm's table states first values for
each and fn-82 owns the tuning.

R1: acanthophyll and persistent-leaf-base move into the expressed vocabulary
with a derivable clause each, guarded on the crown; the palm's gate expectation
now halts on infructescence alone. The gate rerun at this commit is the host's.
R2: tests/species/digests.json and the catalogue pins are unchanged, and the
positive control asserts the palm's own wood moves the moment leaf_bases rises
off zero. R3: the still is at
.flow/evidence/fn-110-the-palms-trunk-organs-persistent-leaf/date-palm-trunk-seed1.png,
one render of the date palm at seed 1; the owner's eye is unclaimed here.
R4: the eight rows are in crates/telperion-jev/data/dials.json with meaning,
range and steps, four of them on SWITCHES_AT_ZERO, and the workspace gate is
green.

Host decisions honoured: the lattice's axial resolution comes from splitting
the stem's polyline for each base rather than a finer step distance, the
skeleton is untouched; persistent-leaf-base requires the rosette; the
acanthophyll's cross-section stays the leaflet's.

Two departures from the design handoff, both recorded: a base is two nodes
rather than one, because the split of the polyline needs an attach point on the
axis, and the crown's frame is carried onto each local axis by rotation rather
than squared onto it, because an orthogonal projection turns the spiral's phase
with the trunk's wander. LeafBase carries the radial and the girth beside the
four fields the design named, so the spiral is checkable as arithmetic.

Follow-ups, not built: a base's taper is the weathering term rather than a row
of its own (design unknown 4), and the growth path builds its own tree, so a
grown specimen carries no bases (design unknown 2) - the host owns whether
parity is owed.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 07ab8dc0731c37cc5c690b6477648e10da6c7d32, 040f0f6c36cf1184b2386c4e28bdcbbcf878a121
- Tests: env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast (suite_rc=0, 120 test binaries green), cargo run --release -p telperion-render --example headless -- --preset date-palm --seed 1 --out .flow/evidence/fn-110-the-palms-trunk-organs-persistent-leaf/date-palm-trunk-seed1.png
- PRs: